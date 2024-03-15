use super::*;
use frame_support::{
	pallet_prelude::{DispatchResult, *},
	traits::nonfungible::{Inspect, Mutate, Transfer},
	Hashable,
};
use ismp::router::{DispatchGet, DispatchRequest};
use pallet_broker::RegionId;
use sp_runtime::traits::Zero;

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
	type ItemId = u128;

	fn owner(item: &Self::ItemId) -> Option<T::AccountId> {
		Regions::<T>::get(RegionId::from(*item)).map(|r| r.owner)
	}

	fn attribute(item: &Self::ItemId, key: &[u8]) -> Option<Vec<u8>> {
		let id = RegionId::from(*item);
		let item = Regions::<T>::get(id)?;
		match key {
			b"begin" => Some(id.begin.encode()),
			b"end" => Some(item.end.encode()),
			b"length" => Some(item.end.saturating_sub(id.begin).encode()),
			b"core" => Some(id.core.encode()),
			b"part" => Some(id.mask.encode()),
			b"owner" => Some(item.owner.encode()),
			b"paid" => Some(item.paid.encode()),
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
	fn mint_into(item: &Self::ItemId, who: &T::AccountId) -> DispatchResult {
		let region_id: RegionId = (*item).into();

		let pallet_hash = sp_io::hashing::twox_128("Broker".as_bytes());
		let storage_hash = sp_io::hashing::twox_128("Regions".as_bytes());
		let region_id_hash = sp_io::hashing::blake2_128(&region_id.encode());

		// pallet_hash + storage_hash + blake2_128(region_id) + region_id
		let key = [
			pallet_hash,
			storage_hash,
			// TODO: don't unwrap
			region_id_hash.try_into().unwrap(),
			region_id.encode().try_into().unwrap(),
		]
		.concat();

		println!("{:?}", hex::encode(key.clone()));

		let get = DispatchGet {
			dest: T::CoretimeChain::get(),
			from: PALLET_ID.to_vec(),
			keys: vec![key],
			height: 0,            // TODO: FIXME
			timeout_timestamp: 0, // TODO: FIXME
			gas_limit: 0,
		};

		let dispatcher = T::IsmpDispatcher::default();
		dispatcher.dispatch_request(DispatchRequest::Get(get), who.clone(), Zero::zero());

		Ok(())
	}

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
