use evmc_declare::evmc_declare_vm;
use evmc_vm::*;

use eth_pairings::{PrecompileAPI, API};

// FIXME: use 'precompile'
#[evmc_declare_vm("EIP1962", "evm", "0.1.0")]
pub struct EIP1962;

impl EvmcVm for EIP1962 {
    fn init() -> Self {
        EIP1962 {}
    }

    fn execute(&self, _code: &[u8], context: &ExecutionContext) -> ExecutionResult {
        let msg = context.get_message();

        let is_call = msg.kind() == evmc_sys::evmc_call_kind::EVMC_CALL;

        // FIXME: check that destination address matches EIP1962
        if !is_call || msg.input().is_none() || msg.input().unwrap().len() < 1 {
            return ExecutionResult::failure();
        }

        let input = msg.input().unwrap();

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

        let result = result.unwrap();

        // FIXME: calculate this properly
        let gas_used = 65536;

        ExecutionResult::success(gas_used, Some(&result))
    }
}
