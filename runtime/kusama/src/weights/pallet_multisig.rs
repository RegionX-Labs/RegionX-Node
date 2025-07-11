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
// pallet_multisig
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

/// Weights for `pallet_multisig` using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_957_000 picoseconds.
		Weight::from_parts(9_135_307, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(213, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 29_736_000 picoseconds.
		Weight::from_parts(23_653_283, 6811)
			// Standard Error: 632
			.saturating_add(Weight::from_parts(68_023, 0).saturating_mul(s.into()))
			// Standard Error: 6
			.saturating_add(Weight::from_parts(1_102, 0).saturating_mul(z.into()))
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
		// Minimum execution time: 19_097_000 picoseconds.
		Weight::from_parts(11_632_187, 6811)
			// Standard Error: 494
			.saturating_add(Weight::from_parts(80_370, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_130, 0).saturating_mul(z.into()))
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
		//  Measured:  `388 + s * (33 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 33_724_000 picoseconds.
		Weight::from_parts(25_786_026, 6811)
			// Standard Error: 373
			.saturating_add(Weight::from_parts(90_167, 0).saturating_mul(s.into()))
			// Standard Error: 3
			.saturating_add(Weight::from_parts(1_176, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 22_503_000 picoseconds.
		Weight::from_parts(23_503_889, 6811)
			// Standard Error: 463
			.saturating_add(Weight::from_parts(74_827, 0).saturating_mul(s.into()))
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
		// Minimum execution time: 11_572_000 picoseconds.
		Weight::from_parts(12_138_764, 6811)
			// Standard Error: 273
			.saturating_add(Weight::from_parts(66_294, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `453 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 22_452_000 picoseconds.
		Weight::from_parts(23_462_202, 6811)
			// Standard Error: 371
			.saturating_add(Weight::from_parts(71_864, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
