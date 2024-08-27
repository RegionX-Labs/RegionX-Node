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

use frame_support::{
	traits::{nonfungible::Transfer, Currency, ExistenceRequirement},
	weights::WeightToFee,
};
use frame_system::WeightInfo;
use order_primitives::{OrderFactory, OrderId, OrderInspect, ParaId, Requirements};
pub use pallet::*;
use sp_runtime::traits::Convert;

const LOG_TARGET: &str = "runtime::rewards";

pub type BalanceOf<T> =
	<<T as crate::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{Inspect, Mutate},
			nonfungible::Mutate as NftMutate,
			tokens::Balance,
			ReservableCurrency,
		},
	};
	use frame_system::pallet_prelude::*;

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Currency used for purchasing coretime.
		type Currency: Mutate<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Relay chain balance type
		type Balance: Balance
			+ Into<<Self::Currency as Inspect<Self::AccountId>>::Balance>
			+ Into<u128>;

		/// Type over which we can access order data.
		type Orders: OrderInspect<Self::AccountId> + OrderFactory<Self::AccountId>;

		/// Weight Info
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn configure_rewards(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO

			Ok(())
		}
	}
}
