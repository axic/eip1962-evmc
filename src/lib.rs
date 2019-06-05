use evmc_declare::evmc_declare_vm;
use evmc_vm::*;

use eth_pairings::{PrecompileAPI, API};

// FIXME: use 'precompile'
#[evmc_declare_vm("EIP1962", "evm")]
pub struct EIP1962;

impl EvmcVm for EIP1962 {
    fn init() -> Self {
        EIP1962 {}
    }

    fn execute(&self, code: &[u8], context: &ExecutionContext) -> ExecutionResult {
        let msg = context.get_message();

        let is_call = (msg.kind == evmc_sys::evmc_call_kind::EVMC_CALL);

        if !is_call || msg.input_data.is_null() || msg.input_size < 1 {
            return ExecutionResult::failure();
        }

        let input = unsafe { std::slice::from_raw_parts(msg.input_data, msg.input_size) };

        let result = match input[0] {
            0 => API::add_points(&input[1..]),
            1 => API::mul_point(&input[1..]),
            2 => API::multiexp(&input[1..]),
            // FIXME: support pairing
            _ => Err(()),
        };

        if result.is_err() {
            return ExecutionResult::failure();
        }

        // FIXME: calculate this properly
        let gas_used = 65536;

        ExecutionResult::success(gas_used, Some(result.unwrap()))
    }
}
