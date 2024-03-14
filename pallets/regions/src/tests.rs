use crate::{mock::*, IsmpModuleCallback};
use frame_support::{assert_ok, traits::nonfungible::Mutate};
use ismp::{
	host::StateMachine,
	module::IsmpModule,
	router::{Get, GetResponse, Response},
};
use pallet_broker::{CoreMask, RegionId};
use std::collections::BTreeMap;

#[test]
fn mint_into_works() {
	new_test_ext().execute_with(|| {
		let region_id: u128 = (RegionId { begin: 1, core: 0, mask: CoreMask::complete() }).into();

		assert_ok!(Regions::mint_into(&region_id, &2));
	});
}

#[test]
fn dummy_test() {
	new_test_ext().execute_with(|| {
		let region_id: u128 = (RegionId { begin: 1, core: 0, mask: CoreMask::complete() }).into();

		assert_ok!(Regions::mint_into(&region_id, &2));

		let dummy_value = hex::decode(
			"8ebb010006db383d9e9ec6ff8862b3dc848f8c6a612cb92cb6ab23424b4330cc9a9e447500",
		)
		.unwrap();

		// Fails because the AccountId used in mock is not the same as the one used in production.
		let dummy_response = Response::Get(GetResponse {
			get: Get {
				source: StateMachine::Polkadot(2000),
				dest: StateMachine::Polkadot(1005),
				nonce: Default::default(),
				from: Default::default(),
				keys: vec![vec![]],
				height: Default::default(),
				timeout_timestamp: Default::default(),
				gas_limit: Default::default(),
			},
			values: BTreeMap::from([(vec![], Some(dummy_value))]),
		});

		let module: IsmpModuleCallback<Test> = IsmpModuleCallback::default();
		module.on_response(dummy_response);
	});
}
