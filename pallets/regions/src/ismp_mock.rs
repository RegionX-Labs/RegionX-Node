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

use anyhow;
use ismp::{
	dispatcher::{DispatchRequest, FeeMetadata, IsmpDispatcher},
	router::{GetRequest as IsmpGet, PostResponse, Request},
};
use ismp_testsuite::mocks::Host;
use polkadot_sdk::*;
use sp_core::{Get, H256};
use std::{cell::RefCell, marker::PhantomData, sync::Arc};

pub struct MockDispatcher<T: crate::Config>(pub Arc<Host>, PhantomData<T>);

impl<T: crate::Config> Default for MockDispatcher<T> {
	fn default() -> Self {
		MockDispatcher(Default::default(), PhantomData::<T>::default())
	}
}

/// Mock request
#[derive(Clone)]
pub struct MockRequest<AccountId> {
	pub request: Request,
	pub who: AccountId,
}

thread_local! {
	pub static REQUESTS: RefCell<Vec<MockRequest<u64>>> = Default::default();
}

impl<T: crate::Config> IsmpDispatcher for MockDispatcher<T> {
	type Account = u64;
	type Balance = u64;

	fn dispatch_request(
		&self,
		request: DispatchRequest,
		fee: FeeMetadata<Self::Account, Self::Balance>,
	) -> Result<H256, anyhow::Error> {
		let request = match request {
			DispatchRequest::Get(get) => Request::Get(IsmpGet {
				source: T::CoretimeChain::get(),
				dest: get.dest,
				nonce: 0,
				from: get.from,
				keys: get.keys.clone(),
				height: get.height,
				timeout_timestamp: T::Timeout::get(),
				context: Default::default(),
			}),
			_ => unimplemented!(),
		};

		REQUESTS.with(|requests| {
			let mut requests = requests.borrow_mut();
			requests.push(MockRequest { request, who: fee.payer });
		});

		Ok(Default::default())
	}

	fn dispatch_response(
		&self,
		_response: PostResponse,
		_fee: FeeMetadata<Self::Account, Self::Balance>,
	) -> Result<H256, anyhow::Error> {
		Ok(Default::default())
	}
}

pub fn requests() -> Vec<MockRequest<u64>> {
	REQUESTS.with(|requests| requests.borrow().clone())
}
