use crate::{ismp_mock::requests, mock::*, Config, IsmpModuleCallback, Region, RequestStatus};
use frame_support::{assert_ok, pallet_prelude::*, traits::nonfungible::Mutate};
use ismp::{
	module::IsmpModule,
	router::{DispatchRequest, Get, GetResponse, Response},
};
use pallet_broker::{CoreMask, RegionId};
use std::collections::BTreeMap;

#[test]
fn nonfungibles_implementation_works() {
	new_test_ext().execute_with(|| {
		let region_id = RegionId { begin: 112830, core: 72, mask: CoreMask::complete() };

		assert!(Regions::regions(&region_id).is_none());
		assert_ok!(Regions::mint_into(&region_id.into(), &2));
		assert_eq!(
			Regions::regions(&region_id).unwrap(),
			Region { owner: 2, record: None, request_status: RequestStatus::Pending }
		);

		// The user is not required to set the region record to withdraw the asset back to the coretime
		// chain.
		//
		// NOTE: Burning occurs when placing the region into the XCM holding registrar at the time of
		// reserve transfer.

		assert_ok!(Regions::burn(&region_id.into(), Some(&2)));
		assert!(Regions::regions(&region_id).is_none());
	});
}

#[test]
fn setting_record_works() {
	new_test_ext().execute_with(|| {
		let region_id = RegionId { begin: 112830, core: 72, mask: CoreMask::complete() };

		assert_ok!(Regions::mint_into(&region_id.into(), &2));
		assert_eq!(
			Regions::regions(&region_id).unwrap(),
			Region { owner: 2, record: None, request_status: RequestStatus::Pending }
		);

		let request = &requests()[0];
		let DispatchRequest::Get(get) = request.request.clone() else {
			panic!("Expected get request")
		};

		assert_eq!(request.who, 2);

		let mock_response = Response::Get(GetResponse {
			get: Get {
				source: <Test as Config>::CoretimeChain::get(),
				dest: get.dest,
				nonce: 0,
				from: get.from,
				keys: get.keys,
				height: get.height,
				timeout_timestamp: <Test as Config>::Timeout::get(),
				gas_limit: 0,
			},
			// TODO: value
			values: BTreeMap::from([(vec![], None)]),
		});

		let module: IsmpModuleCallback<Test> = IsmpModuleCallback::default();
		let _ = module.on_response(mock_response);
	});
}
