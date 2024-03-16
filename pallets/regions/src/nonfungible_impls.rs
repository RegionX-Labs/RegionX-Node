use super::*;
use frame_support::{
	pallet_prelude::{DispatchResult, *},
	traits::nonfungible::{Inspect, Mutate, Transfer},
};
use ismp::router::{DispatchGet, DispatchRequest};
use pallet_broker::RegionId;
use sp_runtime::{
	generic::Header,
	traits::{BlakeTwo256, Zero},
};

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

		// TODO: be defensive here
		let region_id_encoded: [u8; 16] =
			region_id.encode().try_into().expect("RegionId is exactly 128 bits");

		// TODO: remove:
		// DEMO: How to read latest block number:
		let scale_encoded_header = hex::decode("6ecca4d404698fd01e26423b17ee39d193ae71c8860814515c712f62aa10ef76fe0c6601b9a50f5eeaa02c5378c31cf7b6629188f474113861af9b81bc7df5b184e99945cb7c94f60f4820d09e5ee959f322244120deaa1b0cf7f1551775232ddbfcf0720c066175726120c50d7f0800000000045250535290e91a6fa0957fc4b2084adcbdb70c9b7efc51305f423b0789e453fd8c1f23177b0278bf0405617572610101df7e956a8fa912b217bf799e151368f96601e3e0bffcbfd1debaaa0f176e741225f84eb1c21933bba27734ee2da3d80d77fc5b74f9f24da051c1d09122a49f07").unwrap();
		let header: Header<u32, BlakeTwo256> =
			Header::decode(&mut scale_encoded_header.as_slice()).unwrap();
		println!("{:?}", header);

		// pallet_hash + storage_hash + blake2_128(region_id) + scale encoded region_id
		let key = [pallet_hash, storage_hash, region_id_hash, region_id_encoded].concat();

		let coretime_chain_height =
			T::StateMachineHeightProvider::get_latest_state_machine_height(StateMachineId {
				state_id: T::CoretimeChain::get(),
				consensus_state_id: Default::default(), // TODO: FIXME
			})
			.map_or(Err(Error::<T>::FailedReadingCoretimeHeight), Ok)?;

		let get = DispatchGet {
			dest: T::CoretimeChain::get(),
			from: PALLET_ID.to_vec(),
			keys: vec![key],
			height: coretime_chain_height,
			timeout_timestamp: T::TimeoutTimestamp::get(),
			gas_limit: 0,
		};

		let dispatcher = T::IsmpDispatcher::default();

		dispatcher
			.dispatch_request(DispatchRequest::Get(get), who.clone(), Zero::zero())
			.map_err(|_| Error::<T>::IsmpDispatchError)?;

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
