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

use crate::*;
use polkadot_sdk::frame_support::traits::{fungible::Mutate, nonfungible::Transfer};

pub struct DynamicPricing<T: Config>(PhantomData<T>);

impl<T: Config> MarketT<T> for DynamicPricing<T> {
	type PriceData = BalanceOf<T>;

	fn list_region(
		who: T::AccountId,
		region_id: RegionId,
		timeslice_price: Self::PriceData,
		sale_recipient: Option<T::AccountId>,
	) -> DispatchResult {
		ensure!(Listings::<T>::get(region_id).is_none(), Error::<T>::AlreadyListed);

		let region = T::Regions::region(&region_id.into()).ok_or(Error::<T>::UnknownRegion)?;
		ensure!(!region.locked, Error::<T>::RegionLocked);
		let record = region.record.get().ok_or(Error::<T>::RecordUnavailable)?;

		// It doesn't make sense to list a region that expired.
		let current_timeslice = Self::current_timeslice();
		ensure!(record.end > current_timeslice, Error::<T>::RegionExpired);

		T::Regions::lock(&region_id.into(), Some(who.clone()))?;

		let sale_recipient = sale_recipient.unwrap_or(who.clone());
		Listings::<T>::insert(
			region_id,
			Listing {
				seller: who.clone(),
				price_data: timeslice_price,
				sale_recipient: sale_recipient.clone(),
			},
		);

		Ok(())
	}

	fn unlist_region(who: T::AccountId, region_id: RegionId) -> DispatchResult {
		let listing = Listings::<T>::get(region_id).ok_or(Error::<T>::NotListed)?;
		let record = T::Regions::record(&region_id.into()).ok_or(Error::<T>::UnknownRegion)?;

		// If the region expired anyone can remove it from the market.
		let current_timeslice = Self::current_timeslice();
		if current_timeslice <= record.end {
			ensure!(who == listing.seller, Error::<T>::NotAllowed);
		};

		Listings::<T>::remove(region_id);
		T::Regions::unlock(&region_id.into(), None)?;

		Ok(())
	}

	fn update_region_price(
		who: T::AccountId,
		region_id: RegionId,
		new_timeslice_price: Self::PriceData,
	) -> DispatchResult {
		let mut listing = Listings::<T>::get(region_id).ok_or(Error::<T>::NotListed)?;
		let record = T::Regions::record(&region_id.into()).ok_or(Error::<T>::UnknownRegion)?;

		// Only the seller can update the price
		ensure!(who == listing.seller, Error::<T>::NotAllowed);

		let current_timeslice = Self::current_timeslice();
		ensure!(current_timeslice < record.end, Error::<T>::RegionExpired);

		listing.price_data = new_timeslice_price;
		Listings::<T>::insert(region_id, listing);

		Ok(())
	}

	fn purchase_region(
		who: T::AccountId,
		region_id: RegionId,
		max_price: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let listing = Listings::<T>::get(region_id).ok_or(Error::<T>::NotListed)?;
		let record = T::Regions::record(&region_id.into()).ok_or(Error::<T>::UnknownRegion)?;

		ensure!(who != listing.seller && who != listing.sale_recipient, Error::<T>::NotAllowed);

		let price = Self::calculate_region_price(region_id, record, listing.price_data);
		ensure!(price <= max_price, Error::<T>::PriceTooHigh);
		T::Currency::transfer(&who, &listing.sale_recipient, price, Preservation::Preserve)?;

		// Remove the region from sale:
		Listings::<T>::remove(region_id);
		T::Regions::unlock(&region_id.into(), None)?;

		T::Regions::transfer(&region_id.into(), &who)?;

		Ok(price)
	}
}

impl<T: Config> DynamicPricing<T> {
	pub(crate) fn calculate_region_price(
		region_id: RegionId,
		record: RegionRecordOf<T>,
		timeslice_price: BalanceOf<T>,
	) -> BalanceOf<T> {
		let current_timeslice = Self::current_timeslice();
		let duration = record.end.saturating_sub(region_id.begin);

		if current_timeslice < region_id.begin {
			// The region didn't start yet, so there is no value lost.
			let price = timeslice_price.saturating_mul(duration.into());

			return price;
		}

		let remaining_timeslices = record.end.saturating_sub(current_timeslice);
		timeslice_price.saturating_mul(remaining_timeslices.into())
	}

	pub(crate) fn current_timeslice() -> Timeslice {
		let latest_rc_block = T::RCBlockNumberProvider::current_block_number();
		let timeslice_period = T::TimeslicePeriod::get();
		(latest_rc_block / timeslice_period).saturated_into()
	}
}
