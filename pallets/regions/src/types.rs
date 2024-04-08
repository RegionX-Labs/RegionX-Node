use frame_support::traits::fungible::Inspect;
use pallet_broker::RegionRecord;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

pub type BalanceOf<T> = <<T as crate::Config>::NativeCurrency as Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;
pub type RegionRecordOf<T> = RegionRecord<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

/// The request status for getting the region record.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub enum Record<T: crate::Config> {
	/// An ISMP request was made to query the region record and we are now anticipating a response.
	Pending,
	/// An ISMP request was made, but we failed at getting a response.
	Unavailable,
	/// Successfully retrieved the region record.
	Available(RegionRecordOf<T>),
}

impl<T: crate::Config> Record<T> {
	pub fn is_pending(&self) -> bool {
		matches!(self, Record::Pending)
	}

	pub fn is_unavailable(&self) -> bool {
		matches!(self, Record::Unavailable)
	}

	pub fn is_available(&self) -> bool {
		matches!(self, Record::Available(_))
	}

	pub fn get(&self) -> Option<RegionRecordOf<T>> {
		match self {
			Self::Available(r) => Some(r.clone()),
			_ => None,
		}
	}
}

/// Region that got cross-chain transferred from the Coretime chain.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Region<T: crate::Config> {
	/// Owner of the region.
	pub owner: T::AccountId,
	/// The associated record of the region. If `None`, we still didn't receive a response
	/// to the ISMP get request.
	///
	/// NOTE: The owner inside the record is the sovereign account of the parachain, so there
	/// isn't really a point to using it.
	pub record: Record<T>,
}
