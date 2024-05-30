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

use crate::{mock::*, Error, Event, Order, ParaId, Requirements};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::{DispatchError, TokenError};

#[test]
fn create_order_works() {
	new_test_ext(vec![(BOB, 1000), (CHARLIE, 1000)]).execute_with(|| {
		let creator = ALICE;
		let para_id: ParaId = 2000.into();
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		// Creating an order requires the caller to pay the creation fee.
		// The call fails with insufficient balance:
		assert_noop!(
			Orders::create_order(
				RuntimeOrigin::signed(creator.clone()),
				para_id,
				requirements.clone()
			),
			DispatchError::Token(TokenError::FundsUnavailable)
		);

		endow(ALICE, 1000);

		assert_ok!(Orders::create_order(
			RuntimeOrigin::signed(creator.clone()),
			para_id,
			requirements.clone()
		));

		// Check storage items
		assert_eq!(Orders::next_order_id(), 1);
		assert_eq!(Orders::orders(0), Some(Order { para_id, creator, requirements }));
		assert!(Orders::orders(1).is_none());

		// Balance should be reduced due to fee payment:
		assert_eq!(Balances::free_balance(ALICE), 900);
		// Fee goes to the 'treasury':
		assert_eq!(Balances::free_balance(TREASURY), 100);

		// Check events
		System::assert_last_event(Event::OrderCreated { order_id: 0 }.into());
	});
}

#[test]
fn cancel_order_works() {
	new_test_ext(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)]).execute_with(|| {
		let creator = ALICE;
		let para_id: ParaId = 2000.into();
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		// Unknown order id
		assert_noop!(
			Orders::cancel_order(RuntimeOrigin::signed(creator.clone()), 0),
			Error::<Test>::InvalidOrderId
		);

		// Create an order

		assert_ok!(Orders::create_order(
			RuntimeOrigin::signed(creator.clone()),
			para_id,
			requirements.clone()
		));

		// Caller is not the creator of the order
		assert_noop!(
			Orders::cancel_order(RuntimeOrigin::signed(BOB), 0),
			Error::<Test>::NotAllowed
		);

		// Should be working fine
		assert_ok!(Orders::cancel_order(RuntimeOrigin::signed(creator.clone()), 0));

		// Check storage items
		assert!(Orders::orders(0).is_none());

		// Check events
		System::assert_last_event(Event::OrderRemoved { order_id: 0 }.into());
	});
}

#[test]
fn contribute_works() {
	new_test_ext(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)]).execute_with(|| {
		// Create two orders
		assert_ok!(Orders::create_order(
			RuntimeOrigin::signed(ALICE),
			2000.into(),
			Requirements { begin: 0, end: 8, core_occupancy: 28800 }
		));

		assert_ok!(Orders::create_order(
			RuntimeOrigin::signed(BOB),
			2001.into(),
			Requirements { begin: 0, end: 8, core_occupancy: 28800 }
		));

		// Invalid order id
		assert_noop!(
			Orders::contribute(RuntimeOrigin::signed(ALICE), 2, 1_000),
			Error::<Test>::InvalidOrderId
		);

		// Contribution amount is too small
		assert_noop!(
			Orders::contribute(RuntimeOrigin::signed(CHARLIE), 0, 0),
			Error::<Test>::InvalidAmount,
		);

		assert_eq!(Orders::contributions(0, CHARLIE), 0);

		// Should be working fine
		assert_ok!(Orders::contribute(RuntimeOrigin::signed(CHARLIE), 0, 500));

		// Check storage items
		assert_eq!(Orders::contributions(0, CHARLIE), 500);
		assert_eq!(Orders::total_contributions(0), 500);

		// Check events
		System::assert_last_event(
			Event::Contributed { order_id: 0, who: CHARLIE, amount: 500 }.into(),
		);
	});
}
