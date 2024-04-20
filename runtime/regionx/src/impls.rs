use crate::{AssetId, Balance, OrmlAssetRegistry, Authorship, Runtime, RuntimeCall};
use frame_support::traits::{fungibles, Defensive, InstanceFilter};
use pallet_asset_tx_payment::HandleCredit;
use codec::{Decode, Encode, MaxEncodedLen};
use orml_asset_registry::DefaultAssetMetadata;
use orml_traits::{asset_registry::AssetProcessor, GetByKey};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};

#[derive(
	Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen,
)]
pub struct CustomAssetProcessor;

impl AssetProcessor<AssetId, DefaultAssetMetadata<Runtime>> for CustomAssetProcessor {
	fn pre_register(
		id: Option<AssetId>,
		metadata: DefaultAssetMetadata<Runtime>,
	) -> Result<(AssetId, DefaultAssetMetadata<Runtime>), DispatchError> {
		match id {
			Some(id) => Ok((id, metadata)),
			None => Err(DispatchError::Other("asset-registry: AssetId is required")),
		}
	}

	fn post_register(
		_id: AssetId,
		_metadata: DefaultAssetMetadata<Runtime>,
	) -> Result<(), DispatchError> {
		Ok(())
	}
}

/// A `HandleCredit` implementation that naively transfers the fees to the block author.
/// Will drop and burn the assets in case the transfer fails.
pub struct AssetsToBlockAuthor;
impl HandleCredit<AccountId, Assets> for AssetsToBlockAuthor {
	fn handle_credit(credit: fungibles::Credit<AccountId, Assets>) {
		use frame_support::traits::fungibles::Balanced;
		if let Some(author) = Authorship::author() {
			// In case of error: Will drop the result triggering the `OnDrop` of the imbalance.
			let _ = Assets::resolve(&author, credit).defensive();
		}
	}
}

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
			ProxyType::NonTransfer => !matches!(
				c,
				RuntimeCall::Balances { .. } |
					RuntimeCall::Tokens { .. } |
					RuntimeCall::Currencies { .. }
			),
			ProxyType::CancelProxy =>
				matches!(c, RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })),
		}
	}
}

pub struct ExistentialDeposits;
impl GetByKey<AssetId, Balance> for ExistentialDeposits {
	fn get(asset: &AssetId) -> Balance {
		if let Some(metadata) = OrmlAssetRegistry::metadata(asset) {
			metadata.existential_deposit
		} else {
			// As restrictive as we can be. The asset must have associated metadata.
			Balance::MAX
		}
	}
}
