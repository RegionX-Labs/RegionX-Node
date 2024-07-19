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

use crate::mock::{new_test_ext, Orders, Regions, RuntimeOrigin, Test};
use frame_support::{
	assert_ok,
	traits::{nonfungible::Mutate, Currency},
};
use order_primitives::{Order, Requirements};
use pallet_broker::{CoreMask, RegionId};

#[test]
fn fulfill_order_works() {
	new_test_ext(vec![(1, 1000)]).execute_with(|| {
		let region_id = RegionId { begin: 0, core: 0, mask: CoreMask::complete() };
		let region_owner = 1;

		let order_creator = 2000;
		<Test as crate::Config>::Currency::make_free_balance_be(&order_creator, 1000u32.into());
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		// 1. create a region

		assert!(Regions::regions(&region_id).is_none());
		assert_ok!(Regions::mint_into(&region_id.into(), &region_owner));

		// 2. create an order.

		assert_ok!(Orders::create_order(
			RuntimeOrigin::signed(order_creator.clone()),
			2000.into(),
			requirements.clone()
		));
		// Check storage items
		assert_eq!(
			Orders::orders(0),
			Some(Order { para_id: 2000.into(), creator: order_creator, requirements })
		);

		// 3. make contributions to an order
		// 4. call the fulfill extrinsic
		// 5. ensure correct storage items.
	});
}
