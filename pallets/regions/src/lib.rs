#![cfg_attr(not(feature = "std"), no_std)]

use core::cmp::max;
use frame_support::PalletId;
use ismp::{
	consensus::StateMachineId,
	error::Error as IsmpError,
	host::StateMachine,
	module::IsmpModule,
	router::{DispatchGet, DispatchRequest, IsmpDispatcher, Post, Request, Response, Timeout},
};
use ismp_parachain::PARACHAIN_CONSENSUS_ID;
pub use pallet::*;
use pallet_broker::RegionId;
use pallet_ismp::primitives::ModuleId;
use parity_scale_codec::{alloc::collections::BTreeMap, Decode};
use scale_info::prelude::{format, vec, vec::Vec};
use sp_runtime::traits::Zero;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod ismp_mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod nonfungible_impls;

mod types;
use types::*;

const LOG_TARGET: &'static str = "runtime::regions";

/// Constant Pallet ID
pub const PALLET_ID: ModuleId = ModuleId::Pallet(PalletId(*b"ismp-reg"));

// TODO: move trait outside the pallet.
pub trait StateMachineHeightProvider {
	/// Return the latest height of the state machine
	fn get_latest_state_machine_height(id: StateMachineId) -> u64;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{Inspect, Mutate},
			tokens::Balance,
		},
	};
	use frame_system::pallet_prelude::*;

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Native balance
		type Balance: Balance
			+ Into<<Self::NativeCurrency as Inspect<Self::AccountId>>::Balance>
			+ From<u32>;

		/// Native currency implementation
		type NativeCurrency: Mutate<Self::AccountId>;

		/// The Coretime chain from which we read region state.
		type CoretimeChain: Get<StateMachine>;

		/// The ISMP dispatcher.
		type IsmpDispatcher: IsmpDispatcher<Account = Self::AccountId, Balance = <Self as Config>::Balance>
			+ Default;

		/// Used for getting the height of the Coretime chain.
		type StateMachineHeightProvider: StateMachineHeightProvider;

		/// Number of seconds before a get request times out.
		type Timeout: Get<u64>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Regions that got cross-chain transferred to the RegionX parachain.
	#[pallet::storage]
	#[pallet::getter(fn regions)]
	pub type Regions<T> = StorageMap<_, Blake2_128Concat, RegionId, Region<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Ownership of a Region has been transferred.
		Transferred {
			/// The Region which has been transferred.
			region_id: RegionId,
			/// The old owner of the Region.
			old_owner: T::AccountId,
			/// The new owner of the Region.
			owner: T::AccountId,
		},
	}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// The given region identity is not known.
		UnknownRegion,
		/// The owner of the region is not the origin.
		NotOwner,
		/// The region record of the region is already set.
		RegionRecordAlreadySet,
		/// An error occured when attempting to dispatch an ISMP get request.
		IsmpDispatchError,
		/// Failed reading the height of the Coretime chain.
		FailedReadingCoretimeHeight,
		/// The record must be unavailable to be able to re-request it.
		NotUnavailable,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO: correct weight
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn transfer(
			origin: OriginFor<T>,
			region_id: RegionId,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(region_id, Some(who), new_owner)?;

			Ok(())
		}

		// TODO: correct weight
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn request_region_record(origin: OriginFor<T>, region_id: RegionId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let region =
				Regions::<T>::get(region_id).map_or(Err(Error::<T>::UnknownRegion).into(), Ok)?;

			ensure!(region.record.is_unavailable(), Error::<T>::NotUnavailable);

			Self::do_request_region_record(region_id, who)?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_transfer(
			region_id: RegionId,
			maybe_check_owner: Option<T::AccountId>,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let mut region = Regions::<T>::get(&region_id).ok_or(Error::<T>::UnknownRegion)?;

			if let Some(check_owner) = maybe_check_owner {
				ensure!(check_owner == region.owner, Error::<T>::NotOwner);
			}

			let old_owner = region.owner;
			region.owner = new_owner;
			Regions::<T>::insert(&region_id, &region);

			Self::deposit_event(Event::Transferred { region_id, old_owner, owner: region.owner });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn set_record(region_id: RegionId, record: RegionRecordOf<T>) -> DispatchResult {
			let Some(mut region) = Regions::<T>::get(&region_id) else {
				return Err(Error::<T>::UnknownRegion.into());
			};
			ensure!(!region.record.is_available(), Error::<T>::RegionRecordAlreadySet);

			region.record = Record::Available(record);
			Regions::<T>::insert(region_id, region);

			Ok(())
		}

		pub(crate) fn do_request_region_record(
			region_id: RegionId,
			who: <T as frame_system::Config>::AccountId,
		) -> DispatchResult {
			let pallet_hash = sp_io::hashing::twox_128("Broker".as_bytes());
			let storage_hash = sp_io::hashing::twox_128("Regions".as_bytes());
			let region_id_hash = sp_io::hashing::blake2_128(&region_id.encode());

			// TODO: be defensive here
			let region_id_encoded: [u8; 16] =
				region_id.encode().try_into().expect("RegionId is exactly 128 bits");

			// pallet_hash + storage_hash + blake2_128(region_id) + scale encoded region_id
			let key = [pallet_hash, storage_hash, region_id_hash, region_id_encoded].concat();

			let coretime_chain_height =
				T::StateMachineHeightProvider::get_latest_state_machine_height(StateMachineId {
					state_id: T::CoretimeChain::get(),
					consensus_state_id: PARACHAIN_CONSENSUS_ID,
				});

			// TODO: should requests be coupled in the future?
			let get = DispatchGet {
				dest: T::CoretimeChain::get(),
				from: PALLET_ID.to_bytes(),
				keys: vec![key],
				height: coretime_chain_height,
				timeout_timestamp: T::Timeout::get(),
				gas_limit: 0,
			};

			let dispatcher = T::IsmpDispatcher::default();

			dispatcher
				.dispatch_request(DispatchRequest::Get(get), who.clone(), Zero::zero())
				.map_err(|_| Error::<T>::IsmpDispatchError)?;

			// TODO: Emit event

			Ok(())
		}
	}
}

/// Module callback for the pallet
pub struct IsmpModuleCallback<T: Config>(core::marker::PhantomData<T>);

impl<T: Config> Default for IsmpModuleCallback<T> {
	fn default() -> Self {
		Self(core::marker::PhantomData)
	}
}

impl<T: Config> IsmpModule for IsmpModuleCallback<T> {
	fn on_accept(&self, _request: Post) -> Result<(), IsmpError> {
		Err(IsmpError::ImplementationSpecific("Not supported".to_string()))
	}

	fn on_response(&self, response: Response) -> Result<(), IsmpError> {
		match response {
			Response::Post(_) => Err(IsmpError::ImplementationSpecific(
				"Post responses are not accepted".to_string(),
			))?,
			Response::Get(res) => {
				res.get.keys.iter().try_for_each(|key| {
					let value = Self::read_value(&res.values, &key)?;

					// The last 16 bytes represent the region id.
					let mut region_id_encoded = &key[max(0, key.len() as isize - 16) as usize..];

					let region_id = RegionId::decode(&mut region_id_encoded).map_err(|_| {
						IsmpError::ImplementationSpecific("Failed to decode region_id".to_string())
					})?;

					let record =
						RegionRecordOf::<T>::decode(&mut value.as_slice()).map_err(|_| {
							IsmpError::ImplementationSpecific("Failed to decode record".to_string())
						})?;

					crate::Pallet::<T>::set_record(region_id, record)
						.map_err(|e| IsmpError::ImplementationSpecific(format!("{:?}", e)))?;

					Ok(())
				})?;
			},
		}

		Ok(())
	}

	fn on_timeout(&self, timeout: Timeout) -> Result<(), IsmpError> {
		match timeout {
			Timeout::Request(Request::Get(get)) => get.keys.iter().try_for_each(|key| {
				let region_id = RegionId::decode(&mut key.as_slice()).map_err(|_| {
					IsmpError::ImplementationSpecific("Failed to decode region_id".to_string())
				})?;

				let Some(mut region) = Regions::<T>::get(&region_id) else {
					return Err(IsmpError::ImplementationSpecific("Unknown region".to_string()));
				};

				region.record = Record::Unavailable;
				Regions::<T>::insert(region_id, region);

				Ok(())
			}),
			Timeout::Request(Request::Post(_)) => Ok(()),
			Timeout::Response(_) => Ok(()),
		}
	}
}

impl<T: Config> IsmpModuleCallback<T> {
	fn read_value(
		values: &BTreeMap<Vec<u8>, Option<Vec<u8>>>,
		key: &Vec<u8>,
	) -> Result<Vec<u8>, IsmpError> {
		let result = values
			.get(key)
			.ok_or(IsmpError::ImplementationSpecific(
				"The key doesn't have a corresponding value".to_string(),
			))?
			.clone()
			.ok_or(IsmpError::ImplementationSpecific("Region record not found".to_string()))?;

		Ok(result)
	}
}