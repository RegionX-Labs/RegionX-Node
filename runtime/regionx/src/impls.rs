use crate::{AccountId, AssetId, Assets, Authorship, Runtime};
use frame_support::traits::{fungibles, Defensive};
use orml_asset_registry::DefaultAssetMetadata;
use orml_traits::asset_registry::AssetProcessor;
use pallet_asset_tx_payment::HandleCredit;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

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
