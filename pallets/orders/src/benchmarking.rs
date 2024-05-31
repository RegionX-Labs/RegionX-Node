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

//! Benchmarks for pallet-orders

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_order() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();

		let para_id: ParaId = 2000.into();
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), para_id, requirements);

		assert_last_event::<T>(Event::OrderCreated { order_id: 0 }.into());

		Ok(())
	}

	#[benchmark]
	fn cancel_order() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();

		let para_id: ParaId = 2000.into();
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		crate::Pallet::<T>::create_order(RawOrigin::signed(caller.clone()), para_id, requirements);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), 0);

		assert_last_event::<T>(Event::OrderRemoved { order_id: 0 }.into());

		Ok(())
	}

	#[benchmark]
	fn contribute() -> Result<(), BenchmarkError> {
		let creator: T::AccountId = whitelisted_caller();
		let contributor: T::AccountId = account("contri", 0, SEED);

		let para_id: ParaId = 2000.into();
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		crate::Pallet::<T>::create_order(RawOrigin::signed(creator.clone()), para_id, requirements);

		#[extrinsic_call]
		_(RawOrigin::Signed(contributor.clone()), 0, 1_000);

		assert_last_event::<T>(
			Event::Contributed { order_id: 0, who: contributor, amount: 1_000 }.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn remove_contribution() -> Result<(), BenchmarkError> {
		let creator: T::AccountId = whitelisted_caller();
		let contributor: T::AccountId = account("contri", 0, SEED);

		let para_id: ParaId = 2000.into();
		let requirements = Requirements {
			begin: 0,
			end: 8,
			core_occupancy: 28800, // Half of a core.
		};

		crate::Pallet::<T>::create_order(RawOrigin::signed(creator.clone()), para_id, requirements);
		crate::Pallet::<T>::cancel_order(RawOrigin::signed(creator.clone()), 0);

		#[extrinsic_call]
		_(RawOrigin::Signed(contributor.clone()), 0);

		assert_last_event::<T>(
			Event::ContributionRemoved { order_id: 0, who: contributor, amount: 1_000 }.into(),
		);

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
