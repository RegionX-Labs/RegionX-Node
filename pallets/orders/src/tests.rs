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

use crate::{mock::*, Requirements};
use frame_support::assert_ok;

#[test]
fn create_order_works() {
	new_test_ext().execute_with(|| {
		// TODO
		assert_ok!(Orders::create_order(
			RuntimeOrigin::signed(ALICE),
			2000.into(),
			Requirements {
				begin: 0,
				end: 8,
				core_occupancy: 28800 // Half of a core.
			}
		));
	});
}
