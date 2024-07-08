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

use frame_support::traits::{nonfungible::Transfer, Currency};
use frame_system::WeightInfo;
use nonfungible_primitives::LockableNonFungible;
use order_primitives::{OrderId, OrderInspect, Requirements};
pub use pallet::*;
use pallet_broker::{RegionId, RegionRecord};
use region_primitives::RegionInspect;

pub type BalanceOf<T> =
	<<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type RegionRecordOf<T> = RegionRecord<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{fungible::Mutate, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Currency used for purchasing coretime.
		type Currency: Mutate<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Type over which we can access order data.
		type Orders: OrderInspect<Self::AccountId>;

		/// Type providing a way of reading, transferring and locking regions.
		//
		// The item id is `u128` encoded RegionId.
		type Regions: Transfer<Self::AccountId, ItemId = u128>
			+ LockableNonFungible<Self::AccountId, ItemId = u128>
			+ RegionInspect<Self::AccountId, BalanceOf<Self>, ItemId = u128>;

		/// Weight Info
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// Region not found.
		UnknownRegion,
		/// Order not found.
		UnknownOrder,
		/// The region doesn't start when it should based on the requirements.
		RegionStartsTooLate,
		/// The region doesn't end when it should based on the requirements.
		RegionEndsTooSoon,
		/// Regions core mask doesn't match the requirements.
		RegionCoreOccupancyInsufficient,
		/// The region record is not available.
		RecordUnavailable,
		/// Locked regions cannot be listed on sale.
		RegionLocked,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn fulfill_order(
			origin: OriginFor<T>,
			order_id: OrderId,
			region_id: RegionId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let region = T::Regions::region(&region_id.into()).ok_or(Error::<T>::UnknownRegion)?;
			ensure!(!region.locked, Error::<T>::RegionLocked);

			let record = region.record.get().ok_or(Error::<T>::RecordUnavailable)?;
			let order = T::Orders::order(&order_id).ok_or(Error::<T>::UnknownOrder)?;

			Self::ensure_matching_requirements(region_id, record, order.requirements)?;

			// TODO: process fulfilling.

			// TODO: event

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn ensure_matching_requirements(
			region_id: RegionId,
			record: RegionRecordOf<T>,
			requirements: Requirements,
		) -> DispatchResult {
			ensure!(region_id.begin <= requirements.begin, Error::<T>::RegionStartsTooLate);
			ensure!(record.end >= requirements.end, Error::<T>::RegionEndsTooSoon);

			let mask_as_nominator = region_id.mask.count_ones() * (57600 / 80);
			ensure!(
				mask_as_nominator >= requirements.core_occupancy.into(),
				Error::<T>::RegionCoreOccupancyInsufficient
			);

			Ok(())
		}
	}
}
