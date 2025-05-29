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

use crate::{AccountId, Balance, Balances, PalletCurrency, PotId, RuntimeCall};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{Imbalance, InstanceFilter, OnUnbalanced};
use order_primitives::ParaId;
use pallet_broker::{Finality, RegionId};
use pallet_processor::assigner::AssignmentCallEncoder as AssignmentCallEncoderT;
use sp_runtime::{traits::AccountIdConversion, DispatchResult, RuntimeDebug};

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	/// Fully permissioned proxy. Can execute any call on behalf of _proxied_.
	Any,
	/// Can execute any call that does not transfer funds or assets.
	NonTransfer,
	/// Proxy with the ability to reject time-delay proxy announcements.
	CancelProxy,
	// TODO: add more proxies in future related to coretime trading.
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(c, RuntimeCall::Balances { .. }),
			ProxyType::CancelProxy =>
				matches!(c, RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })),
		}
	}
}

type NegativeImbalance = <Balances as PalletCurrency<AccountId>>::NegativeImbalance;

pub struct ToStakingPot;
impl OnUnbalanced<NegativeImbalance> for ToStakingPot {
	fn on_nonzero_unbalanced(amount: NegativeImbalance) {
		let staking_pot = PotId::get().into_account_truncating();
		Balances::resolve_creating(&staking_pot, amount);
	}
}

pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
		if let Some(fees) = fees_then_tips.next() {
			// 60% of the fees go to the treasury, and the rest goes to the collators along with the
			// tips.
			let (_treasury, mut collators) = fees.ration(60, 40);

			if let Some(tips) = fees_then_tips.next() {
				tips.merge_into(&mut collators);
			}

			<ToStakingPot as OnUnbalanced<_>>::on_unbalanced(collators);
			// TODO: to treasury
		}
	}
}

pub struct OrderCreationFeeHandler;
impl pallet_orders::FeeHandler<AccountId, Balance> for OrderCreationFeeHandler {
	fn handle(_who: &AccountId, _fee: Balance) -> DispatchResult {
		// We send the order creation fee to the treasury:
		// TODO:
		// <Runtime as pallet_orders::Config>::Currency::transfer(
		// 	who,
		// 	&RegionXTreasuryAccount::get(),
		// 	fee,
		// 	ExistenceRequirement::KeepAlive,
		// )?;
		Ok(())
	}
}

#[derive(Encode, Decode)]
enum CoretimeRuntimeCalls {
	#[codec(index = 50)]
	Broker(BrokerPalletCalls),
}

/// Broker pallet calls. We don't define all of them, only the ones we use.
#[derive(Encode, Decode)]
enum BrokerPalletCalls {
	#[codec(index = 10)]
	Assign(RegionId, ParaId, Finality),
}

pub struct AssignmentCallEncoder;
impl AssignmentCallEncoderT for AssignmentCallEncoder {
	fn encode_assignment_call(region_id: RegionId, para_id: ParaId) -> sp_std::vec::Vec<u8> {
		CoretimeRuntimeCalls::Broker(BrokerPalletCalls::Assign(region_id, para_id, Finality::Final))
			.encode()
	}
}
