use ismp::consensus::StateMachineId;

pub trait StateMachineHeightProvider {
	/// Return the latest height of the state machine
	fn get_latest_state_machine_height(id: StateMachineId) -> u64;
}
