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

//! Benchmarking setup for pallet-regions

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use crate::mock::*;
use pallet_broker::{RegionId, CoreMask};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks  {
	use super::*;

	#[benchmark]
	fn transfer() -> Result<(), BenchmarkError> {
		let old_owner = 1;
		let new_owner: T::AccountId = 2;
		let region_id = RegionId { begin: 112830, core: 72, mask: CoreMask::complete() };

		#[extrinsic_call]
		_(RuntimeOrigin::signed(old_owner), region_id, new_owner);

		Ok(())
	}

	#[benchmark]
	fn request_region_record() -> Result<(), BenchmarkError> {
		let owner = 1;
		let region_id = RegionId { begin: 112830, core: 72, mask: CoreMask::complete() };

		#[extrinsic_call]
		_(RuntimeOrigin::signed(owner), region_id);

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}