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

use crate::{frame_system::ensure_signed, *};
use polkadot_sdk::frame_support::traits::{fungible::Mutate, nonfungible::Transfer};

pub struct FixedPricing<T: Config>(PhantomData<T>);

impl<T: Config> MarketT<T> for FixedPricing<T> {
	type PriceData = BalanceOf<T>;

	fn list_region(
		who: T::AccountId,
		region_id: RegionId,
		price: Self::PriceData,
		sale_recipient: Option<T::AccountId>,
	) -> DispatchResult {
		ensure!(Listings::<T>::get(region_id).is_none(), Error::<T>::AlreadyListed);

		let region = T::Regions::region(&region_id.into()).ok_or(Error::<T>::UnknownRegion)?;

		T::Regions::lock(&region_id.into(), Some(who.clone()))?;

		let sale_recipient = sale_recipient.unwrap_or(who.clone());
		Listings::<T>::insert(
			region_id,
			Listing {
				seller: who.clone(),
				price_data: price,
				sale_recipient: sale_recipient.clone(),
			},
		);

		Ok(())
	}

	fn unlist_region(who: T::AccountId, region_id: RegionId) -> DispatchResult {
		let listing = Listings::<T>::get(region_id).ok_or(Error::<T>::NotListed)?;

		ensure!(who == listing.seller, Error::<T>::NotAllowed);

		Listings::<T>::remove(region_id);
		T::Regions::unlock(&region_id.into(), None)?;

		Ok(())
	}

	fn update_region_price(
		who: T::AccountId,
		region_id: RegionId,
		new_price: Self::PriceData,
	) -> DispatchResult {
		let mut listing = Listings::<T>::get(region_id).ok_or(Error::<T>::NotListed)?;

		// Only the seller can update the price
		ensure!(who == listing.seller, Error::<T>::NotAllowed);

		listing.price_data = new_price;
		Listings::<T>::insert(region_id, listing);

		Ok(())
	}

	fn purchase_region(
		who: T::AccountId,
		region_id: RegionId,
		max_price: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let listing = Listings::<T>::get(region_id).ok_or(Error::<T>::NotListed)?;

		ensure!(who != listing.seller && who != listing.sale_recipient, Error::<T>::NotAllowed);

		ensure!(listing.price_data <= max_price, Error::<T>::PriceTooHigh);
		T::Currency::transfer(
			&who,
			&listing.sale_recipient,
			listing.price_data,
			Preservation::Preserve,
		)?;

		// Remove the region from sale:
		Listings::<T>::remove(region_id);
		T::Regions::unlock(&region_id.into(), None)?;

		T::Regions::transfer(&region_id.into(), &who)?;

		Ok(listing.price_data)
	}
}
