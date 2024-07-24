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

//! Benchmarks for pallet-market

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_support::{assert_ok, traits::fungible::Mutate};
use frame_system::RawOrigin;
use pallet_broker::{CoreMask, RegionId, RegionRecord};

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn fulfill_order() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 57600, // Full core.
		};

		assert_ok!(Orders::create_order(
			T::RuntimeOrigin::signed(caller.clone()),
			2000.into(),
			requirements.clone()
		));
		// Create a region which meets the requirements:
		// let region_id = RegionId { begin: 0, core: 0, mask: CoreMask::complete() };
		// assert_ok!(Regions::mint_into(&region_id.into(), &caller));
		// assert_ok!(Regions::set_record(region_id, RegionRecord { end: 10, owner: 1, paid: None
		// }));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), 0, region_id);

		// assert_last_event::<T>(
		// 	Event::Listed {
		// 		region_id,
		// 		timeslice_price,
		// 		seller: caller.clone(),
		// 		sale_recipient: caller,
		// 	}
		// 	.into(),
		// );

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
