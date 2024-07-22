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
use crate::LOG_TARGET;
use core::marker::PhantomData;
use order_primitives::ParaId;
use pallet_broker::RegionId;
use sp_runtime::{traits::Get, DispatchResult};
use xcm::latest::prelude::*;

/// Type assigning the region to the specified task.
pub trait RegionAssigner {
	// Assigns the region to the specified task.
	fn assign(region_id: RegionId, para_id: ParaId) -> DispatchResult;
}

/// A type that implements the RegionAssigner trait and assigns a region to a task by sending the
/// appropriate XCM message to the Coretime chain.
pub struct XcmRegionAssigner<T: crate::Config>(PhantomData<T>);
impl<T: crate::Config + pallet_xcm::Config> RegionAssigner for XcmRegionAssigner<T> {
	fn assign(region_id: RegionId, para_id: ParaId) -> DispatchResult {
		let message = Xcm(vec![]);

		match pallet_xcm::Pallet::<T>::send_xcm(
			Here,
			<T as crate::Config>::CoretimeChain::get(),
			message,
		) {
			Ok(_) => log::info!(
				target: LOG_TARGET,
				"Region assignment sent successfully"
			),
			Err(e) => log::error!(
				target: LOG_TARGET,
				"Failed to send region assignment: {:?}",
				e
			),
		}

		Ok(())
	}
}
