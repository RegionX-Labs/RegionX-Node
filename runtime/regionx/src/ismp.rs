use crate::{AccountId, Balances, IsmpParachain, ParachainInfo, Runtime, RuntimeEvent, Timestamp};

use frame_support::pallet_prelude::Get;
use frame_system::EnsureRoot;
use ismp::{
	error::Error,
	host::StateMachine,
	module::IsmpModule,
	prelude::Vec,
	router::{IsmpRouter, Post, Request, Response, Timeout},
};
use ismp_parachain::ParachainConsensusClient;
use scale_info::prelude::boxed::Box;

pub struct HostStateMachine;
impl Get<StateMachine> for HostStateMachine {
	fn get() -> StateMachine {
		StateMachine::Kusama(ParachainInfo::get().into())
	}
}

pub struct Coprocessor;

impl Get<Option<StateMachine>> for Coprocessor {
	fn get() -> Option<StateMachine> {
		Some(HostStateMachine::get())
	}
}

impl ismp_parachain::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_ismp::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	const INDEXING_PREFIX: &'static [u8] = b"ISMP";
	type AdminOrigin = EnsureRoot<AccountId>;
	type HostStateMachine = HostStateMachine;
	type Coprocessor = Coprocessor;
	type TimeProvider = Timestamp;
	type Router = Router;
	type ConsensusClients = (ParachainConsensusClient<Runtime, IsmpParachain>,);
	type WeightInfo = ();
	type WeightProvider = ();
}

#[derive(Default)]
pub struct ProxyModule;

impl IsmpModule for ProxyModule {
	fn on_accept(&self, request: Post) -> Result<(), Error> {
		todo!()
	}

	fn on_response(&self, response: Response) -> Result<(), Error> {
		todo!()
	}

	fn on_timeout(&self, timeout: Timeout) -> Result<(), Error> {
		todo!()
	}
}

#[derive(Default)]
pub struct Router;

impl IsmpRouter for Router {
	fn module_for_id(&self, _bytes: Vec<u8>) -> Result<Box<dyn IsmpModule>, Error> {
		Ok(Box::new(ProxyModule::default()))
	}
}
