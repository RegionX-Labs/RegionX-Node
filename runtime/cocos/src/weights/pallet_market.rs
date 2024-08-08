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

//! Autogenerated weights for `pallet_market`
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
// pallet_market
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

/// Weights for `pallet_market` using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_market::WeightInfo for WeightInfo<T> {
	/// Storage: `Market::Listings` (r:1 w:1)
	/// Proof: `Market::Listings` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Regions::Regions` (r:1 w:1)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn list_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `3584`
		// Minimum execution time: 30_288_000 picoseconds.
		Weight::from_parts(31_055_000, 3584)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Market::Listings` (r:1 w:1)
	/// Proof: `Market::Listings` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Regions::Regions` (r:1 w:1)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn unlist_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465`
		//  Estimated: `3584`
		// Minimum execution time: 31_776_000 picoseconds.
		Weight::from_parts(32_554_000, 3584)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Market::Listings` (r:1 w:1)
	/// Proof: `Market::Listings` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Regions::Regions` (r:1 w:0)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_region_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465`
		//  Estimated: `3584`
		// Minimum execution time: 25_120_000 picoseconds.
		Weight::from_parts(25_685_000, 3584)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Market::Listings` (r:1 w:1)
	/// Proof: `Market::Listings` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Regions::Regions` (r:1 w:1)
	/// Proof: `Regions::Regions` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Metadata` (r:1 w:0)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(737), added: 3212, mode: `MaxEncodedLen`)
	fn purchase_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `924`
		//  Estimated: `6156`
		// Minimum execution time: 74_563_000 picoseconds.
		Weight::from_parts(77_441_000, 6156)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
