// This file is part of RegionX.
//
// RegionX is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RegionX is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RegionX.  If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::fungibles;
use frame_system::WeightInfo;
use order_primitives::{OrderFactory, OrderId, OrderInspect};
pub use pallet::*;
use sp_runtime::traits::Zero;

mod types;

const LOG_TARGET: &str = "runtime::rewards";

pub type BalanceOf<T> = <<T as crate::Config>::Assets as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type AssetIdOf<T> = <<T as crate::Config>::Assets as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::AssetId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{fungibles::Inspect, tokens::Balance},
	};
	use frame_system::pallet_prelude::*;
	use types::PrizePoolDetails;

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Relay chain balance type
		type Balance: Balance
			+ Into<<Self::Assets as Inspect<Self::AccountId>>::Balance>
			+ Into<u128>;

		/// The type should provide access to assets which can be used as reward.
		type Assets: Inspect<Self::AccountId>;

		/// Type over which we can access order data.
		type Orders: OrderInspect<Self::AccountId> + OrderFactory<Self::AccountId>;

		/// Weight Info
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Defines the rewards that will be distributed among order contributors if the order is
	/// fulfilled on time.
	#[pallet::storage]
	#[pallet::getter(fn order_rewards)]
	pub type PrizePools<T: Config> =
		StorageMap<_, Blake2_128Concat, OrderId, PrizePoolDetails<AssetIdOf<T>, BalanceOf<T>>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// The caller submitted an extrinsic which wasn't allowed with their origin.
		Unallowed,
		/// The order expired and there is no point in configuring rewards.
		OrderExpired,
		/// Order not found.
		UnknownOrder,
		/// Rewards can only be configured if there is no existing configuration or if the rewards
		/// are not yet set and are currently zero.
		CantConfigure,
		/// The reward pool of an order was not found.
		PrizePoolNotFound,
		/// The asset the user wanted to use for rewards does not exist.
		AssetNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// The order creator can configure the asset which will be used for rewarding contributors.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn configure_rewards(
			origin: OriginFor<T>,
			order_id: OrderId,
			asset_id: AssetIdOf<T>,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			let order = T::Orders::order(&order_id).ok_or(Error::<T>::UnknownOrder)?;

			ensure!(!T::Orders::order_expired(&order), Error::<T>::OrderExpired);
			ensure!(order.creator == caller, Error::<T>::Unallowed);

			let maybe_pool = PrizePools::<T>::get(order_id);
			// Rewards can be reconfigured if the amount is still zero.
			if let Some(pool) = maybe_pool {
				ensure!(pool.amount == Zero::zero(), Error::<T>::CantConfigure);
			}

			ensure!(T::Assets::asset_exists(asset_id.clone()) == true, Error::<T>::AssetNotFound);

			PrizePools::<T>::insert(order_id, PrizePoolDetails { asset_id, amount: Zero::zero() });

			// TODO: deposit event
			Ok(())
		}

		/// Add rewards to a reward pool of an order.
		///
		/// The added rewards will be of the configured asset kind.
		///
		/// ## Parameters
		///
		/// - `origin`: Signed origin. Can be called by any account.
		/// - `order_id`: The order to which the user wants to contribute rewards. There must exist
		///   a reward pool for the specified order.
		///  -`amount`: number of tokens the user wants to add to the reward pool.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn add_rewards(
			origin: OriginFor<T>,
			order_id: OrderId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let _caller = ensure_signed(origin)?;

			let Some(pool) = PrizePools::<T>::get(order_id) else {
				return Err(Error::<T>::PrizePoolNotFound.into())
			};

			// TODO: check if user has enough tokens
			// TODO: contribute to the pool.

			Ok(())
		}
	}
}
