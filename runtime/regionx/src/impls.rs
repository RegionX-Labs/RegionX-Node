use crate::{AssetId, Runtime, AccountId, Assets};
use frame_support::traits::fungibles::{Credit, Balanced};
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

pub struct CreditToBlockAuthor;
impl HandleCredit<AccountId, Assets> for CreditToBlockAuthor {
	fn handle_credit(credit: Credit<AccountId, Assets>) {
		if let Some(author) = pallet_authorship::Pallet::<Runtime>::author() {
			// What to do in case paying the author fails (e.g. because `fee < min_balance`)
			// default: drop the result which will trigger the `OnDrop` of the imbalance.
			let _ = <Assets as Balanced<AccountId>>::resolve(&author, credit);
		}
	}
}
