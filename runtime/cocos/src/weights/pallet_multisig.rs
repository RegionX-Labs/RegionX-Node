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

//! Autogenerated weights for `pallet_multisig`
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
// pallet_multisig
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

/// Weights for `pallet_multisig` using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_131_000 picoseconds.
		Weight::from_parts(16_978_422, 0)
			// Standard Error: 4
			.saturating_add(Weight::from_parts(591, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 50_456_000 picoseconds.
		Weight::from_parts(39_816_139, 6811)
			// Standard Error: 929
			.saturating_add(Weight::from_parts(127_793, 0).saturating_mul(s.into()))
			// Standard Error: 9
			.saturating_add(Weight::from_parts(1_443, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `6811`
		// Minimum execution time: 32_312_000 picoseconds.
		Weight::from_parts(21_261_678, 6811)
			// Standard Error: 539
			.saturating_add(Weight::from_parts(119_644, 0).saturating_mul(s.into()))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_544, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421 + s * (33 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 56_810_000 picoseconds.
		Weight::from_parts(43_101_432, 6811)
			// Standard Error: 907
			.saturating_add(Weight::from_parts(160_132, 0).saturating_mul(s.into()))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_642, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 35_843_000 picoseconds.
		Weight::from_parts(37_448_993, 6811)
			// Standard Error: 823
			.saturating_add(Weight::from_parts(135_329, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `6811`
		// Minimum execution time: 19_544_000 picoseconds.
		Weight::from_parts(19_994_838, 6811)
			// Standard Error: 498
			.saturating_add(Weight::from_parts(118_911, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `486 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 37_733_000 picoseconds.
		Weight::from_parts(39_211_183, 6811)
			// Standard Error: 776
			.saturating_add(Weight::from_parts(129_956, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
