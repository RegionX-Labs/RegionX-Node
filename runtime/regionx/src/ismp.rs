use crate::{AccountId, Balance};
use ismp::{
	error::Error,
	router::{DispatchRequest, IsmpDispatcher, PostResponse},
};
use ismp_testsuite::mocks::Host;
use std::sync::Arc;

#[derive(Default)]
pub struct MockDispatcher(pub Arc<Host>);

impl IsmpDispatcher for MockDispatcher {
	type Account = AccountId;
	type Balance = Balance;

	fn dispatch_request(
		&self,
		_request: DispatchRequest,
		_who: Self::Account,
		_fee: Self::Balance,
	) -> Result<(), Error> {
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
