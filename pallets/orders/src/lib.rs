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

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
	pub type Orders<T: Config> = StorageMap<_, Blake2_128Concat, OrderId, Order<T::AccountId>>;

	/// Crowdfunding contributions.
	#[pallet::storage]
	#[pallet::getter(fn contributions)]
	pub type Contributions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		OrderId, // the order.
		Blake2_128Concat,
		T::AccountId, // the account which contributed to the order.
		BalanceOf<T>, // the amount they contributed.
		ValueQuery,
	>;

	/// The total amount that was contributed to an order.
	///
	/// The sum of contributions for a specific order from the Contributions map should be equal to
	/// the total contribution stored here.
	#[pallet::storage]
	#[pallet::getter(fn total_contributions)]
	pub type TotalContributions<T: Config> =
		StorageMap<_, Blake2_128Concat, OrderId, BalanceOf<T>, ValueQuery>;

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
		/// Callable by signed origin or by the parachain iteslf.
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

			// TODO: charge order creation cost

			// TODO: add the order to the `Orders` storage map. (The order id is incremental)

			// TODO: Emit event

			Ok(())
		}

		/// Extrinsic for cancelling an order.
		///
		/// Callable by signed origin or by the parachain iteslf.
		///
		/// ## Arguments:
		/// - `para_id`: The para id to which Coretime will be allocated.
		/// - `requirements`: Region requirements of the order.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)] // TODO
		pub fn cancel_order(origin: OriginFor<T>, order: OrderId) -> DispatchResult {
			let who = Self::ensure_signed_or_para(origin)?;

			// TODO: ensure order creator

			// TODO: remove the order from the `Orders` storage map.

			// TODO: emit event

			Ok(())
		}

		/// Extrinsic for contributing to an order.
		///
		/// Callable by signed origin.
		///
		/// ## Arguments:
		/// - `order`: The order to which the caller wants to contribute.
		/// - `amount`: The amount of tokens the user wants to contribute.
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)] // TODO
		pub fn contribute(
			origin: OriginFor<T>,
			order: OrderId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// TODO: ensure order exists

			// TODO: Update `Contributions`

			// TODO: Update `TotalContributions`

			// TODO: emit event

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
						MultiLocation { parents: 1, interior: X1(Parachain(para.into())) };
					T::SovereignAccountOf::convert_location(&location).ok_or(BadOrigin)
				},
			)?;
			Ok(account)
		}
	}
}
