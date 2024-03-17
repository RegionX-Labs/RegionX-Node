use super::*;
use frame_support::{
	pallet_prelude::{DispatchResult, *},
	traits::nonfungible::{Inspect, Mutate, Transfer},
};
use pallet_broker::RegionId;

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
	type ItemId = u128;

	fn owner(item: &Self::ItemId) -> Option<T::AccountId> {
		Regions::<T>::get(RegionId::from(*item)).map(|r| r.owner)
	}

	fn attribute(item: &Self::ItemId, key: &[u8]) -> Option<Vec<u8>> {
		let id = RegionId::from(*item);
		let record = Regions::<T>::get(id)?.record?;
		match key {
			b"begin" => Some(id.begin.encode()),
			b"end" => Some(record.end.encode()),
			b"length" => Some(record.end.saturating_sub(id.begin).encode()),
			b"core" => Some(id.core.encode()),
			b"part" => Some(id.mask.encode()),
			b"owner" => Some(record.owner.encode()),
			b"paid" => Some(record.paid.encode()),
			_ => None,
		}
	}
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
	fn transfer(item: &Self::ItemId, dest: &T::AccountId) -> DispatchResult {
		Self::do_transfer((*item).into(), None, dest.clone()).map_err(Into::into)
	}
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
	/// Minting is used for depositing a region from the holding registar.
	fn mint_into(item: &Self::ItemId, who: &T::AccountId) -> DispatchResult {
		let region_id: RegionId = (*item).into();
		Self::do_request_region_record(region_id, who.clone())?;

		Ok(())
	}

	/// Burning is used for withdrawing a region into the holding registar.
	fn burn(item: &Self::ItemId, maybe_check_owner: Option<&T::AccountId>) -> DispatchResult {
		let region_id: RegionId = (*item).into();

		let record = Regions::<T>::get(&region_id).ok_or(Error::<T>::UnknownRegion)?;
		if let Some(owner) = maybe_check_owner {
			ensure!(owner.clone() == record.owner, Error::<T>::NotOwner);
		}

		Regions::<T>::remove(region_id);

		Ok(())
	}
}
