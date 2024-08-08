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

//! Autogenerated weights for `pallet_orders`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-08, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_orders
// --steps
// 20
// --repeat
// 50
// --output
// ./runtime/cocos/src/weights/
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

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_orders` using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_orders::WeightInfo for WeightInfo<T> {
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Metadata` (r:1 w:0)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(737), added: 3212, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Orders::NextOrderId` (r:1 w:1)
	/// Proof: `Orders::NextOrderId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Orders::Orders` (r:0 w:1)
	/// Proof: `Orders::Orders` (`max_values`: None, `max_size`: Some(66), added: 2541, mode: `MaxEncodedLen`)
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `521`
		//  Estimated: `6156`
		// Minimum execution time: 50_880_000 picoseconds.
		Weight::from_parts(52_427_000, 6156)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Orders::Orders` (r:1 w:1)
	/// Proof: `Orders::Orders` (`max_values`: None, `max_size`: Some(66), added: 2541, mode: `MaxEncodedLen`)
	fn cancel_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3531`
		// Minimum execution time: 18_507_000 picoseconds.
		Weight::from_parts(19_132_000, 3531)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Orders::Orders` (r:1 w:0)
	/// Proof: `Orders::Orders` (`max_values`: None, `max_size`: Some(66), added: 2541, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Metadata` (r:1 w:0)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(737), added: 3212, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Orders::Contributions` (r:1 w:1)
	/// Proof: `Orders::Contributions` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `648`
		//  Estimated: `6156`
		// Minimum execution time: 65_524_000 picoseconds.
		Weight::from_parts(66_826_000, 6156)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Orders::Orders` (r:1 w:0)
	/// Proof: `Orders::Orders` (`max_values`: None, `max_size`: Some(66), added: 2541, mode: `MaxEncodedLen`)
	/// Storage: `Orders::Contributions` (r:1 w:1)
	/// Proof: `Orders::Contributions` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Metadata` (r:1 w:0)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(737), added: 3212, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn remove_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `811`
		//  Estimated: `6156`
		// Minimum execution time: 65_813_000 picoseconds.
		Weight::from_parts(67_517_000, 6156)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
