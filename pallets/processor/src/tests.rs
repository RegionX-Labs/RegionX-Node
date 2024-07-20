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

use crate::mock::{new_test_ext, Balances, Orders, Processor, Regions, RuntimeOrigin, Test};
use frame_support::{
	assert_ok,
	traits::{nonfungible::Mutate, Currency},
};
use order_primitives::{Order, Requirements};
use pallet_broker::{CoreMask, RegionId, RegionRecord};
use region_primitives::RegionInspect;

#[test]
fn fulfill_order_works() {
	new_test_ext(vec![(2000, 1000), (10, 1000), (11, 1000), (12, 1000)]).execute_with(|| {
		let region_owner = 1;
		let order_creator = 2000;
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		// 2. create an order
		<Test as crate::Config>::Currency::make_free_balance_be(&order_creator, 1000u32.into());
		assert_ok!(Orders::create_order(
			RuntimeOrigin::signed(order_creator.clone()),
			2000.into(),
			requirements.clone()
		));
		assert_eq!(
			Orders::orders(0),
			Some(Order { para_id: 2000.into(), creator: order_creator, requirements })
		);

		// 3. make contributions to an order
		assert_ok!(Orders::contribute(RuntimeOrigin::signed(10), 0, 500));
		assert_ok!(Orders::contribute(RuntimeOrigin::signed(11), 0, 800));
		assert_ok!(Orders::contribute(RuntimeOrigin::signed(12), 0, 200));

		// Fulfill order fails with a region that doesn't meet the requirements:

		// Create a region which doesn't the requirements:
		let region_id = RegionId { begin: 0, core: 0, mask: CoreMask::from_chunk(0, 10) };
		assert_ok!(Regions::mint_into(&region_id.into(), &region_owner));
		assert_ok!(Regions::set_record(
			region_id,
			RegionRecord { end: 123600, owner: 1, paid: None }
		));

		// Create a region which meets the requirements:
		let region_id = RegionId { begin: 0, core: 0, mask: CoreMask::complete() };
		assert_ok!(Regions::mint_into(&region_id.into(), &region_owner));
		assert_ok!(Regions::set_record(
			region_id,
			RegionRecord { end: 123600, owner: 1, paid: None }
		));

		// Works with a region that meets the requirements:
		assert_ok!(Processor::fulfill_order(RuntimeOrigin::signed(region_owner), 0, region_id));
		// Ensure order is removed:
		assert!(Orders::orders(0).is_none());
		// Region owner receives as the contributions for fulfilling the order:
		assert_eq!(Balances::free_balance(region_owner), 1500);
		assert_eq!(Regions::regions(region_id).unwrap().owner, 2000);
	});
}
