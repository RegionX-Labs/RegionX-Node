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

use frame_support::{
	pallet_prelude::*,
	traits::{fungible::Inspect, tokens::Preservation},
};
use frame_system::pallet_prelude::OriginFor;
use nonfungible_primitives::LockableNonFungible;
pub use pallet::*;
use pallet_broker::{RegionId, Timeslice};
use polkadot_sdk::*;
use region_primitives::{RegionFactory, RegionInspect};
use sp_runtime::{traits::BlockNumberProvider, SaturatedConversion, Saturating};

mod types;
pub use crate::types::*;

pub mod dynamic_pricing;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type BalanceOf<T> =
	<<T as crate::Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

/// Relay chain block number.
pub type RCBlockNumberOf<T> =
	<<T as crate::Config>::RCBlockNumberProvider as BlockNumberProvider>::BlockNumber;

pub trait MarketT<T: crate::Config> {
	type PriceData: Parameter;

	fn list_region(
		who: T::AccountId,
		region_id: RegionId,
		price_data: Self::PriceData,
		sale_recipient: Option<T::AccountId>,
	) -> DispatchResult;

	fn unlist_region(who: T::AccountId, region_id: RegionId) -> DispatchResult;

	fn update_region_price(
		who: T::AccountId,
		region_id: RegionId,
		new_timeslice_price: Self::PriceData,
	) -> DispatchResult;

	fn purchase_region(
		who: T::AccountId,
		region_id: RegionId,
		max_price: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError>;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::traits::{fungible::Mutate, nonfungible::Transfer};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: polkadot_sdk::frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>>
			+ IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

		/// Currency used for purchasing coretime.
		type Currency: Mutate<Self::AccountId>;

		/// Implementation of all the marketplace extrinsics.
		///
		/// This is because we want to support two different pricing models:
		/// - Fixed pricing
		/// - Dynamic pricing: price of an 'active' region decreases over time.
		type MarketImpl: MarketT<Self>;

		/// Type providing a way of reading, transferring and locking regions.
		//
		// The item id is `u128` encoded RegionId.
		type Regions: Transfer<Self::AccountId, ItemId = u128>
			+ LockableNonFungible<Self::AccountId, ItemId = u128>
			+ RegionInspect<Self::AccountId, BalanceOf<Self>, ItemId = u128>
			+ RegionFactory<Self::AccountId, RegionRecordOf<Self>>;

		/// Type for getting the current relay chain block.
		///
		/// This is used for determining the current timeslice.
		type RCBlockNumberProvider: BlockNumberProvider;

		/// Number of Relay-chain blocks per timeslice.
		#[pallet::constant]
		type TimeslicePeriod: Get<RCBlockNumberOf<Self>>;

		/// Weight Info
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Regions that got listed on sale.
	#[pallet::storage]
	#[pallet::getter(fn listings)]
	pub type Listings<T: Config> =
		StorageMap<_, Blake2_128Concat, RegionId, Listing<T::AccountId, BalanceOf<T>>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A region got listed on sale.
		Listed {
			/// The region that got listed on sale.
			region_id: RegionId,
			/// The price per timeslice of the listed region.
			price_data: <T::MarketImpl as MarketT<T>>::PriceData,
			/// The seller of the region.
			seller: T::AccountId,
			/// The sale revenue recipient.
			sale_recipient: T::AccountId,
		},
		Unlisted {
			/// The region that got unlisted.
			region_id: RegionId,
		},
		Purchased {
			/// The region that got purchased.
			region_id: RegionId,
			/// The buyer of the region.
			buyer: T::AccountId,
			/// The total price paid for the listed region.
			total_price: BalanceOf<T>,
		},
		PriceUpdated {
			/// The region for which the sale price was updated.
			region_id: RegionId,
			/// New timeslice price
			price_data: <T::MarketImpl as MarketT<T>>::PriceData,
		},
	}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// Caller tried to list a region which is already listed.
		AlreadyListed,
		/// Caller tried to unlist a region which is not listed.
		NotListed,
		/// Region not found.
		UnknownRegion,
		/// The specified region is expired.
		RegionExpired,
		/// The caller is not allowed to perform a certain action.
		NotAllowed,
		/// The price of the region is higher than what the buyer is willing to pay.
		PriceTooHigh,
		/// The region record is not available.
		RecordUnavailable,
		/// Locked regions cannot be listed on sale.
		RegionLocked,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Extrinsic for listing a region on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The region that the caller intends to list for sale.
		/// - `timeslice_price`: The price per a single timeslice.
		/// - `sale_recipient`: The `AccountId` receiving the payment from the sale. If not
		///   specified this will be the caller.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::list_region())]
		pub fn list_region(
			origin: OriginFor<T>,
			region_id: RegionId,
			price_data: <T::MarketImpl as MarketT<T>>::PriceData,
			sale_recipient: Option<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<T::MarketImpl as MarketT<T>>::list_region(
				who.clone(),
				region_id,
				price_data.clone(),
				sale_recipient.clone(),
			)?;

			Self::deposit_event(Event::Listed {
				region_id,
				price_data,
				seller: who.clone(),
				sale_recipient: sale_recipient.unwrap_or(who.clone()),
			});
			Ok(())
		}

		/// Extrinsic for unlisting a region on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The region that the caller intends to unlist from sale.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::unlist_region())]
		pub fn unlist_region(origin: OriginFor<T>, region_id: RegionId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<T::MarketImpl as MarketT<T>>::unlist_region(who.clone(), region_id)?;

			Self::deposit_event(Event::Unlisted { region_id });
			Ok(())
		}

		/// Extrinsic for updating the price of a region listed on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The region that is listed on sale.
		/// - `new_timeslice_price`: The new timeslice price.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::update_region_price())]
		pub fn update_region_price(
			origin: OriginFor<T>,
			region_id: RegionId,
			price_data: <T::MarketImpl as MarketT<T>>::PriceData,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<T::MarketImpl as MarketT<T>>::update_region_price(
				who.clone(),
				region_id,
				price_data.clone(),
			)?;

			Self::deposit_event(Event::PriceUpdated { region_id, price_data });
			Ok(())
		}

		/// Extrinsic for purchasing a region listed on sale.
		///
		/// ## Arguments:
		/// - `region_id`: The region that is listed on sale.
		/// - `max_price`: The maximum price the buyer is willing to pay for the region. If the
		///   actual price exceeds this amount, the purchase will not be executed. The region price
		///   is linearly decreasing for currently active(i.e. usable) regions.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::purchase_region())]
		pub fn purchase_region(
			origin: OriginFor<T>,
			region_id: RegionId,
			max_price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let price =
				<T::MarketImpl as MarketT<T>>::purchase_region(who.clone(), region_id, max_price)?;

			Self::deposit_event(Event::Purchased { region_id, buyer: who, total_price: price });
			Ok(())
		}
	}
}
