use ismp::{
	error::Error,
	router::{DispatchRequest, Get as IsmpGet, IsmpDispatcher, PostResponse, Request},
};
use ismp_testsuite::mocks::Host;
use sp_core::Get;
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
		who: Self::Account,
		_fee: Self::Balance,
	) -> Result<(), Error> {
		let request = match request {
			DispatchRequest::Get(get) => Request::Get(IsmpGet {
				source: T::CoretimeChain::get(),
				dest: get.dest,
				nonce: 0,
				from: get.from,
				keys: get.keys.clone(),
				height: get.height,
				timeout_timestamp: T::Timeout::get(),
				gas_limit: 0,
			}),
			_ => unimplemented!(),
		};

		REQUESTS.with(|requests| {
			let mut requests = requests.borrow_mut();
			requests.push(MockRequest { request, who });
		});

		Ok(())
	}

	fn dispatch_response(
		&self,
		_response: PostResponse,
		_who: Self::Account,
		_fee: Self::Balance,
	) -> Result<(), Error> {
		Ok(())
	}
}

pub fn requests() -> Vec<MockRequest<u64>> {
	REQUESTS.with(|requests| requests.borrow().clone())
}
