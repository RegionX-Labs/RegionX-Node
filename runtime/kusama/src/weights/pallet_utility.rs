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

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.2.0
//! DATE: 2025-07-01, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sergej-B650-AORUS-ELITE-AX`, CPU: `AMD Ryzen 9 7900X3D 12-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("regionx-kusama-dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/regionx-node
// benchmark
// pallet
// --chain
// regionx-kusama-dev
// --pallet
// pallet_utility
// --steps
// 20
// --repeat
// 50
// --output
// ./runtime/kusama/src/weights/
// --header
// ./config/HEADER-GPL3
// --template
// ./config/runtime-weight-template.hbs
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;
use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_utility` using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_377_000 picoseconds.
		Weight::from_parts(7_871_344, 0)
			// Standard Error: 1_567
			.saturating_add(Weight::from_parts(2_084_676, 0).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_106_000 picoseconds.
		Weight::from_parts(3_306_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_456_000 picoseconds.
		Weight::from_parts(21_110_576, 0)
			// Standard Error: 2_638
			.saturating_add(Weight::from_parts(2_310_304, 0).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_709_000 picoseconds.
		Weight::from_parts(4_939_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_436_000 picoseconds.
		Weight::from_parts(2_307_763, 0)
			// Standard Error: 4_709
			.saturating_add(Weight::from_parts(2_105_849, 0).saturating_mul(c.into()))
	}
}
