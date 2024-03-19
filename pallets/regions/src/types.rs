use frame_support::traits::fungible::Inspect;
use pallet_broker::RegionRecord;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

pub type BalanceOf<T> = <<T as crate::Config>::NativeCurrency as Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;
pub type RegionRecordOf<T> = RegionRecord<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

/// The request status for getting the region record.
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum RequestStatus {
	/// A request was made and we are still anticipating a response
	Pending,
	/// A request was made, but it timed out.
	Timedout,
	/// Successfully retreived the region record.
	Successful,
}

/// Region that got cross-chain transferred from the Coretime chain.
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Region<T: crate::Config> {
	/// Owner of the region.
	pub owner: T::AccountId,
	/// The associated record of the region. If `None`, we still didn't receive a response
	/// to the ISMP get request.
	///
	/// NOTE: The owner inside the record is the sovereign account of the parachain, so there
	/// isn't really a point to using it.
	pub record: Option<RegionRecordOf<T>>,
	/// The status of the ISMP get request for getting the region record.
	pub request_status: RequestStatus,
}
