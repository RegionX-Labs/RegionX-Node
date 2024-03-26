use ismp::{
	error::Error,
	router::{DispatchRequest, IsmpDispatcher, PostResponse},
};
use ismp_testsuite::mocks::Host;
use std::{cell::RefCell, sync::Arc};

#[derive(Default)]
pub struct MockDispatcher(pub Arc<Host>);

/// Mock request
#[derive(Clone)]
pub struct Request<AccountId> {
	pub request: DispatchRequest,
	pub who: AccountId,
}

thread_local! {
	pub static REQUESTS: RefCell<Vec<Request<u64>>> = Default::default();
}

impl IsmpDispatcher for MockDispatcher {
	type Account = u64;
	type Balance = u64;

	fn dispatch_request(
		&self,
		request: DispatchRequest,
		who: Self::Account,
		_fee: Self::Balance,
	) -> Result<(), Error> {
		REQUESTS.with(|requests| {
			let mut requests = requests.borrow_mut();
			requests.push(Request { request, who });
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

pub fn requests() -> Vec<Request<u64>> {
	REQUESTS.with(|requests| requests.borrow().clone())
}
