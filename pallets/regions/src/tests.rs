use crate::{
	ismp_mock::requests, mock::*, utils, IsmpCustomError, IsmpModuleCallback, Record, Region,
};
use frame_support::{assert_err, assert_ok, pallet_prelude::*, traits::nonfungible::Mutate};
use ismp::{
	module::IsmpModule,
	router::{GetResponse, Post, PostResponse, Request, Response, Timeout},
};
use pallet_broker::{CoreMask, RegionId, RegionRecord};
use std::collections::BTreeMap;

#[test]
fn nonfungibles_implementation_works() {
	new_test_ext().execute_with(|| {
		let region_id = RegionId { begin: 112830, core: 72, mask: CoreMask::complete() };

		assert!(Regions::regions(&region_id).is_none());
		assert_ok!(Regions::mint_into(&region_id.into(), &2));
		assert_eq!(
			Regions::regions(&region_id).unwrap(),
			Region { owner: 2, record: Record::Pending }
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
fn set_record_works() {
	new_test_ext().execute_with(|| {
		// TODO
	});
}

#[test]
fn request_region_record_works() {
	new_test_ext().execute_with(|| {
		// TODO
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		// TODO
	});
}

#[test]
fn on_response_works() {
	new_test_ext().execute_with(|| {
		let region_id = RegionId { begin: 112830, core: 72, mask: CoreMask::complete() };

		assert_ok!(Regions::mint_into(&region_id.into(), &2));
		assert_eq!(
			Regions::regions(&region_id).unwrap(),
			Region { owner: 2, record: Record::Pending }
		);

		let request = &requests()[0];
		let Request::Get(get) = request.request.clone() else { panic!("Expected get request") };

		assert_eq!(request.who, 2);

		let mock_record: RegionRecord<u64, u64> = RegionRecord { end: 42, owner: 1, paid: None };

		let mock_response = Response::Get(GetResponse {
			get: get.clone(),
			values: BTreeMap::from([(get.keys[0].clone(), Some(mock_record.encode()))]),
		});

		let module: IsmpModuleCallback<Test> = IsmpModuleCallback::default();
		assert_ok!(module.on_response(mock_response));

		assert_eq!(
			Regions::regions(&region_id).unwrap(),
			Region { owner: 2, record: Record::Available(mock_record) }
		);
	});
}

#[test]
fn on_response_only_handles_get() {
	new_test_ext().execute_with(|| {
		let module: IsmpModuleCallback<Test> = IsmpModuleCallback::default();

		let mock_response = Response::Post(PostResponse {
			post: Post {
				source: <Test as crate::Config>::CoretimeChain::get(),
				dest: <Test as crate::Config>::CoretimeChain::get(),
				nonce: Default::default(),
				from: Default::default(),
				to: Default::default(),
				timeout_timestamp: Default::default(),
				data: Default::default(),
			},
			response: Default::default(),
			timeout_timestamp: Default::default(),
		});

		assert_err!(module.on_response(mock_response), IsmpCustomError::NotSupported);
	});
}

#[test]
fn on_timeout_only_handles_get() {
	new_test_ext().execute_with(|| {
		// TODO
	});
}

#[test]
fn on_timeout_works() {
	new_test_ext().execute_with(|| {
		let region_id = RegionId { begin: 0, core: 72, mask: CoreMask::complete() };

		assert_ok!(Regions::mint_into(&region_id.into(), &2));
		assert_eq!(
			Regions::regions(&region_id).unwrap(),
			Region { owner: 2, record: Record::Pending }
		);

		let request = &requests()[0];

		let Request::Get(get) = request.request.clone() else { panic!("Expected get request") };

		let module: IsmpModuleCallback<Test> = IsmpModuleCallback::default();
		let timeout = Timeout::Request(Request::Get(get));
		assert_ok!(module.on_timeout(timeout));

		assert_eq!(
			Regions::regions(&region_id).unwrap(),
			Region { owner: 2, record: Record::Unavailable }
		);
	});
}

#[test]
fn on_accept_works() {
	new_test_ext().execute_with(|| {
		let post = Post {
			source: <Test as crate::Config>::CoretimeChain::get(),
			dest: <Test as crate::Config>::CoretimeChain::get(),
			nonce: 0,
			from: Default::default(),
			to: Default::default(),
			timeout_timestamp: 0,
			data: Default::default(),
		};
		let module: IsmpModuleCallback<Test> = IsmpModuleCallback::default();
		assert_err!(module.on_accept(post), IsmpCustomError::NotSupported);
	});
}

#[test]
fn utils_read_value_works() {
	new_test_ext().execute_with(|| {
		let mut values: BTreeMap<Vec<u8>, Option<Vec<u8>>> = BTreeMap::new();
		values.insert("key1".as_bytes().to_vec(), Some("value1".as_bytes().to_vec()));
		values.insert("key2".as_bytes().to_vec(), None);

		assert_eq!(
			utils::read_value(&values, &"key1".as_bytes().to_vec()),
			Ok("value1".as_bytes().to_vec())
		);
		assert_eq!(
			utils::read_value(&values, &"key42".as_bytes().to_vec()),
			Err(IsmpCustomError::ValueNotFound.into())
		);
		assert_eq!(
			utils::read_value(&values, &"key2".as_bytes().to_vec()),
			Err(IsmpCustomError::EmptyValue.into())
		);
	});
}
