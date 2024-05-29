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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{error::BadOrigin, traits::fungible::Inspect};
use frame_system::WeightInfo;
pub use pallet::*;
use parachain_primitives::{ensure_parachain, Origin, ParaId};
use xcm::latest::prelude::*;
use xcm_executor::traits::ConvertLocation;

mod types;
pub use crate::types::*;

pub type BalanceOf<T> =
	<<T as crate::Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{Inspect, Mutate},
			tokens::Balance,
		},
	};
	use frame_system::pallet_prelude::*;

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config + parachain_primitives::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The balance type
		type Balance: Balance
			+ Into<<Self::Currency as Inspect<Self::AccountId>>::Balance>
			+ From<u32>;

		/// The aggregated origin type must support the `parachains` origin. We require that we can
		/// infallibly convert between this origin and the system origin, but in reality, they're
		/// the same type, we just can't express that to the Rust type system without writing a
		/// `where` clause everywhere.
		type RuntimeOrigin: From<<Self as frame_system::Config>::RuntimeOrigin>
			+ Into<Result<Origin, <Self as Config>::RuntimeOrigin>>;

		/// Currency used for purchasing coretime.
		type Currency: Mutate<Self::AccountId>;

		/// How to get an `AccountId` value from a `Location`.
		type SovereignAccountOf: ConvertLocation<Self::AccountId>;

		/// The cost of order creation.
		///
		/// Used to prevent spamming.
		#[pallet::constant]
		type OrderCreationCost: Get<BalanceOf<Self>>;

		/// The minimum contribution to an order.
		#[pallet::constant]
		type MinimumContribution: Get<BalanceOf<Self>>;

		/// Weight Info
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Created orders.
	#[pallet::storage]
	#[pallet::getter(fn orders)]
	pub type Orders<T: Config> = StorageMap<_, Blake2_128Concat, (), ()>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Extrinsic for creating an order.
		///
		/// ## Arguments:
		/// - `para_id`: The para id to which Coretime will be allocated.
		/// - `requirements`: Region requirements of the order.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)] // TODO
		pub fn create_order(
			origin: OriginFor<T>,
			para_id: ParaId,
			requirements: Requirements,
		) -> DispatchResult {
			let who = Self::ensure_signed_or_para(origin)?;

			//ensure!();

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Ensure the origin is signed or the `para` itself.
		fn ensure_signed_or_para(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
			let account = ensure_signed(origin.clone()).map_err(|e| e.into()).or_else(
				|_: DispatchError| -> Result<_, _> {
					let para =
						ensure_parachain(<T as Config>::RuntimeOrigin::from(origin.clone()))?;
					let location: MultiLocation =
						MultiLocation { parents: 1, interior: X1(Parachain(para)) };
					T::SovereignAccountOf::convert_location(&location).ok_or(BadOrigin)
				},
			)?;
			Ok(account)
		}
	}
}
