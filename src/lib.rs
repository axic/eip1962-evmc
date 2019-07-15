use evmc_declare::evmc_declare_vm;
use evmc_vm::*;

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

        let gas_cost = eth_pairings::gas_meter::GasMeter::meter(&input);
        let gas_cost = if gas_cost.is_err() {
            return ExecutionResult::failure();
        } else {
            gas_cost.unwrap()
        };

        // TODO: change upstream to return i64
        let gas_cost = if gas_cost > 0x7fffffffffffffff {
            return ExecutionResult::failure();
        } else {
            gas_cost as i64
        };

        if gas_cost > msg.gas() {
            return ExecutionResult::failure();
        }

        let result = eth_pairings::public_interface::API::run(&input);
        let result = if result.is_err() {
            return ExecutionResult::failure();
        } else {
            result.unwrap()
        };

        ExecutionResult::success(gas_cost, Some(&result))
    }
}
