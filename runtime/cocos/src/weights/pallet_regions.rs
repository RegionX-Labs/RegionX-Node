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

//! Autogenerated weights for `pallet_regions`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-20, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cocos-instance-1`, CPU: `Intel(R) Xeon(R) CPU @ 2.80GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("cocos")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/regionx-node
// benchmark
// pallet
// --chain
// cocos
// --pallet
// pallet_regions
// --steps
// 20
// --repeat
// 50
// --output
// ./runtime/cocos/src/weights/
// --header
// ./config/HEADER-GPL3
// --template
// ./config/frame-weight-template.hbs
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_regions` using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_regions::WeightInfo for WeightInfo<T> {
	/// Storage: `Regions::Regions` (r:1 w:1)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `3584`
		// Minimum execution time: 17_401_000 picoseconds.
		Weight::from_parts(17_804_000, 3584)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Regions::Regions` (r:1 w:1)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `Ismp::LatestStateMachineHeight` (r:1 w:0)
	/// Proof: `Ismp::LatestStateMachineHeight` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Ismp::Nonce` (r:1 w:1)
	/// Proof: `Ismp::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x52657175657374436f6d6d69746d656e7473a610605b386e00611ea5ab7d28d2` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x52657175657374436f6d6d69746d656e7473a610605b386e00611ea5ab7d28d2` (r:1 w:1)
	fn request_region_record() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `527`
		//  Estimated: `3992`
		// Minimum execution time: 48_680_000 picoseconds.
		Weight::from_parts(49_953_000, 3992)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn on_accept() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 746_000 picoseconds.
		Weight::from_parts(791_000, 0)
	}
	/// Storage: `Regions::Regions` (r:1 w:1)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn on_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `3584`
		// Minimum execution time: 13_807_000 picoseconds.
		Weight::from_parts(14_255_000, 3584)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Regions::Regions` (r:1 w:1)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn on_timeout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `3584`
		// Minimum execution time: 9_038_000 picoseconds.
		Weight::from_parts(9_310_000, 3584)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
