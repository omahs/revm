use crate::{
    alloc::vec::Vec,
    gas::{self, COLD_ACCOUNT_ACCESS_COST, WARM_STORAGE_READ_COST},
    return_ok, return_revert, CallContext, CallInputs, CallScheme, CreateInputs, CreateScheme,
    Host, Interpreter, Return, Spec,
    SpecId::{self, *},
    Transfer,
};
use bytes::Bytes;
use core::cmp::min;
use primitive_types::{H160, H256, U256};

pub fn balance<const SPEC_ID: u8>(interpreter: &mut Interpreter, host: &mut dyn Host) -> Return {
    pop_address!(interpreter, address);
    let ret = host.balance(address);
    if ret.is_none() {
        return Return::FatalExternalError;
    }
    let (balance, is_cold) = ret.unwrap();
    gas!(
        interpreter,
        if SpecId::ISTANBUL.enabled_in(SPEC_ID) {
            // EIP-1884: Repricing for trie-size-dependent opcodes
            gas::account_access_gas::<SPEC_ID>(is_cold)
        } else if SpecId::TANGERINE.enabled_in(SPEC_ID) {
            400
        } else {
            20
        }
    );
    push!(interpreter, balance);

    Return::Continue
}

/// Opcode is introduced in ISTANBUL: EIP-1884: Repricing for trie-size-dependent opcodes
pub fn selfbalance<const SPEC_ID: u8>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    // gas!(interpreter, gas::LOW);
    let ret = host.balance(interpreter.contract.address);
    if ret.is_none() {
        return Return::FatalExternalError;
    }
    let (balance, _) = ret.unwrap();
    push!(interpreter, balance);

    Return::Continue
}

pub fn extcodesize<const SPEC_ID: u8>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    pop_address!(interpreter, address);
    let ret = host.code(address);
    if ret.is_none() {
        return Return::FatalExternalError;
    }
    let (code, is_cold) = ret.unwrap();
    if SpecId::BERLIN.enabled_in(SPEC_ID) && is_cold {
        // WARM_STORAGE_READ_COST is already calculated in gas block
        gas!(
            interpreter,
            COLD_ACCOUNT_ACCESS_COST - WARM_STORAGE_READ_COST
        );
    }

    push!(interpreter, U256::from(code.len()));

    Return::Continue
}

/// Opcode enabled in CONSTANTINOPLE: EIP-1052: EXTCODEHASH opcode
pub fn extcodehash<const SPEC_ID: u8>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    pop_address!(interpreter, address);
    let ret = host.code_hash(address);
    if ret.is_none() {
        return Return::FatalExternalError;
    }
    let (code_hash, is_cold) = ret.unwrap();
    if SpecId::BERLIN.enabled_in(SPEC_ID) && is_cold {
        // WARM_STORAGE_READ_COST is already calculated in gas block
        gas!(
            interpreter,
            COLD_ACCOUNT_ACCESS_COST - WARM_STORAGE_READ_COST
        );
    }
    push_h256!(interpreter, code_hash);

    Return::Continue
}

pub fn extcodecopy<const SPEC_ID: u8>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    pop_address!(interpreter, address);
    pop!(interpreter, memory_offset, code_offset, len_u256);

    let ret = host.code(address);
    if ret.is_none() {
        return Return::FatalExternalError;
    }
    let (code, is_cold) = ret.unwrap();

    let len = as_usize_or_fail!(len_u256, Return::OutOfGas);
    gas_or_fail!(
        interpreter,
        gas::extcodecopy_cost::<SPEC_ID>(len as u64, is_cold)
    );
    if len == 0 {
        return Return::Continue;
    }
    let memory_offset = as_usize_or_fail!(memory_offset, Return::OutOfGas);
    let code_offset = min(as_usize_saturated!(code_offset), code.len());
    memory_resize!(interpreter, memory_offset, len);

    // Safety: set_data is unsafe function and memory_resize ensures us that it is safe to call it
    interpreter
        .memory
        .set_data(memory_offset, code_offset, len, code.bytes());
    Return::Continue
}

pub fn blockhash(interpreter: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BLOCKHASH);
    pop_top!(interpreter, number);

    if let Some(diff) = host.env().block.number.checked_sub(*number) {
        let diff = as_usize_saturated!(diff);
        // blockhash should push zero if number is same as current block number.
        if diff <= 256 && diff != 0 {
            let ret = host.block_hash(*number);
            if ret.is_none() {
                return Return::FatalExternalError;
            }
            *number = U256::from_big_endian(ret.unwrap().as_ref());
            return Return::Continue;
        }
    }
    *number = U256::zero();
    Return::Continue
}

pub fn sload<const SPEC_ID: u8>(interpreter: &mut Interpreter, host: &mut dyn Host) -> Return {
    pop!(interpreter, index);

    let ret = host.sload(interpreter.contract.address, index);
    if ret.is_none() {
        return Return::FatalExternalError;
    }
    let (value, is_cold) = ret.unwrap();
    gas!(interpreter, gas::sload_cost::<SPEC_ID>(is_cold));
    push!(interpreter, value);
    Return::Continue
}

pub fn sstore<const SPEC_ID: u8>(interpreter: &mut Interpreter, host: &mut dyn Host) -> Return {
    //check!(!SPEC::IS_STATIC_CALL);

    pop!(interpreter, index, value);
    let ret = host.sstore(interpreter.contract.address, index, value);
    if ret.is_none() {
        return Return::FatalExternalError;
    }
    let (original, old, new, is_cold) = ret.unwrap();
    gas_or_fail!(interpreter, {
        let remaining_gas = interpreter.gas.remaining();
        gas::sstore_cost::<SPEC_ID>(original, old, new, remaining_gas, is_cold)
    });
    refund!(
        interpreter,
        gas::sstore_refund::<SPEC_ID>(original, old, new)
    );
    interpreter.add_next_gas_block(interpreter.program_counter() - 1)
}

// Is not available in static call
pub fn log<const SPEC_ID: u8, const N: u8>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    pop!(interpreter, offset, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);
    gas_or_fail!(interpreter, gas::log_cost(N, len as u64));
    let data = if len == 0 {
        Bytes::new()
    } else {
        let offset = as_usize_or_fail!(offset, Return::OutOfGas);
        memory_resize!(interpreter, offset, len);
        Bytes::copy_from_slice(interpreter.memory.get_slice(offset, len))
    };

    if interpreter.stack.len() < N as usize {
        return Return::StackUnderflow;
    }

    let mut topics = Vec::with_capacity(N as usize);
    for _ in 0..(N) {
        let mut t = H256::zero();
        // Safety: stack bounds already checked few lines above
        unsafe {
            interpreter
                .stack
                .pop_unsafe()
                .to_big_endian(t.as_bytes_mut())
        };
        topics.push(t);
    }

    host.log(interpreter.contract.address, topics, data);
    Return::Continue
}

/// In static call this opcode is not available
pub fn selfdestruct<const SPEC_ID: u8>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    pop_address!(interpreter, target);

    let res = host.selfdestruct(interpreter.contract.address, target);
    if res.is_none() {
        return Return::FatalExternalError;
    }
    let res = res.unwrap();

    // EIP-3529: Reduction in refunds
    if !SpecId::LONDON.enabled_in(SPEC_ID) && !res.previously_destroyed {
        refund!(interpreter, gas::SELFDESTRUCT)
    }
    gas!(interpreter, gas::selfdestruct_cost::<SPEC_ID>(res));

    Return::SelfDestruct
}

pub fn create<const SPEC_ID: u8, const IS_CREATE2: bool>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    //check!(!SPEC::IS_STATIC_CALL);
    if IS_CREATE2 {
        // EIP-1014: Skinny CREATE2
        check!(SpecId::PETERSBURG.enabled_in(SPEC_ID));
    }

    interpreter.return_data_buffer = Bytes::new();

    pop!(interpreter, value, code_offset, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);

    let code = if len == 0 {
        Bytes::new()
    } else {
        let code_offset = as_usize_or_fail!(code_offset, Return::OutOfGas);
        memory_resize!(interpreter, code_offset, len);
        Bytes::copy_from_slice(interpreter.memory.get_slice(code_offset, len))
    };

    let scheme = if IS_CREATE2 {
        pop!(interpreter, salt);
        gas_or_fail!(interpreter, gas::create2_cost(len));
        CreateScheme::Create2 { salt }
    } else {
        gas!(interpreter, gas::CREATE);
        CreateScheme::Create
    };

    let mut gas_limit = interpreter.gas().remaining();

    // EIP-150: Gas cost changes for IO-heavy operations
    if SpecId::TANGERINE.enabled_in(SPEC_ID) {
        // take remaining gas and deduce l64 part of it.
        gas_limit -= gas_limit / 64
    }
    gas!(interpreter, gas_limit);

    let mut create_input = CreateInputs {
        caller: interpreter.contract.address,
        scheme,
        value,
        init_code: code,
        gas_limit,
    };

    let (return_reason, address, gas, return_data) = host.create(&mut create_input);
    interpreter.return_data_buffer = return_data;

    match return_reason {
        return_ok!() => {
            push_h256!(interpreter, address.map(|a| a.into()).unwrap_or_default());
            interpreter.gas.erase_cost(gas.remaining());
            interpreter.gas.record_refund(gas.refunded());
        }
        return_revert!() => {
            push_h256!(interpreter, H256::default());
            interpreter.gas.erase_cost(gas.remaining());
        }
        Return::FatalExternalError => return Return::FatalExternalError,
        _ => {
            push_h256!(interpreter, H256::default());
        }
    }
    interpreter.add_next_gas_block(interpreter.program_counter() - 1)
}

// TODO make these calls nicer

pub fn call<const SPEC_ID: u8, const IS_STATIC: bool>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    call_template::<SPEC_ID, IS_STATIC>(interpreter, CallScheme::Call, host)
}

pub fn callcode<const SPEC_ID: u8, const IS_STATIC: bool>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    call_template::<SPEC_ID, IS_STATIC>(interpreter, CallScheme::CallCode, host)
}

pub fn delegatecall<const SPEC_ID: u8, const IS_STATIC: bool>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    call_template::<SPEC_ID, IS_STATIC>(interpreter, CallScheme::DelegateCall, host)
}

pub fn staticcall<const SPEC_ID: u8, const IS_STATIC: bool>(
    interpreter: &mut Interpreter,
    host: &mut dyn Host,
) -> Return {
    call_template::<SPEC_ID, IS_STATIC>(interpreter, CallScheme::StaticCall, host)
}

pub fn call_template<const SPEC_ID: u8, const IS_STATIC: bool>(
    interpreter: &mut Interpreter,
    scheme: CallScheme,
    host: &mut dyn Host,
) -> Return {
    match scheme {
        CallScheme::DelegateCall => check!(SpecId::HOMESTEAD.enabled_in(SPEC_ID)), // EIP-7: DELEGATECALL
        CallScheme::StaticCall => check!(SpecId::BYZANTIUM.enabled_in(SPEC_ID)), // EIP-214: New opcode STATICCALL
        _ => (),
    }
    interpreter.return_data_buffer = Bytes::new();

    pop!(interpreter, local_gas_limit);
    pop_address!(interpreter, to);
    let local_gas_limit = if local_gas_limit > U256::from(u64::MAX) {
        u64::MAX
    } else {
        local_gas_limit.as_u64()
    };

    let value = match scheme {
        CallScheme::CallCode => {
            pop!(interpreter, value);
            value
        }
        CallScheme::Call => {
            pop!(interpreter, value);
            // TODO
            if IS_STATIC && !value.is_zero() {
                return Return::CallNotAllowedInsideStatic;
            }
            value
        }
        CallScheme::DelegateCall | CallScheme::StaticCall => U256::zero(),
    };

    pop!(interpreter, in_offset, in_len, out_offset, out_len);

    let in_len = as_usize_or_fail!(in_len, Return::OutOfGas);
    let input = if in_len != 0 {
        let in_offset = as_usize_or_fail!(in_offset, Return::OutOfGas);
        memory_resize!(interpreter, in_offset, in_len);
        Bytes::copy_from_slice(interpreter.memory.get_slice(in_offset, in_len))
    } else {
        Bytes::new()
    };

    let out_len = as_usize_or_fail!(out_len, Return::OutOfGas);
    let out_offset = if out_len != 0 {
        let out_offset = as_usize_or_fail!(out_offset, Return::OutOfGas);
        memory_resize!(interpreter, out_offset, out_len);
        out_offset
    } else {
        usize::MAX //unrealistic value so we are sure it is not used
    };

    let context = match scheme {
        CallScheme::Call | CallScheme::StaticCall => CallContext {
            address: to,
            caller: interpreter.contract.address,
            code_address: to,
            apparent_value: value,
            scheme,
        },
        CallScheme::CallCode => CallContext {
            address: interpreter.contract.address,
            caller: interpreter.contract.address,
            code_address: to,
            apparent_value: value,
            scheme,
        },
        CallScheme::DelegateCall => CallContext {
            address: interpreter.contract.address,
            caller: interpreter.contract.caller,
            code_address: to,
            apparent_value: interpreter.contract.value,
            scheme,
        },
    };

    let transfer = if scheme == CallScheme::Call {
        Transfer {
            source: interpreter.contract.address,
            target: to,
            value,
        }
    } else if scheme == CallScheme::CallCode {
        Transfer {
            source: interpreter.contract.address,
            target: interpreter.contract.address,
            value,
        }
    } else {
        //this is dummy send for StaticCall and DelegateCall, it should do nothing and dont touch anything.
        Transfer {
            source: interpreter.contract.address,
            target: interpreter.contract.address,
            value: U256::zero(),
        }
    };

    // load account and calculate gas cost.
    let res = host.load_account(to);
    if res.is_none() {
        return Return::FatalExternalError;
    }
    let (is_cold, exist) = res.unwrap();
    let is_new = !exist;

    gas!(
        interpreter,
        gas::call_cost::<SPEC_ID>(
            value,
            is_new,
            is_cold,
            matches!(scheme, CallScheme::Call | CallScheme::CallCode),
            matches!(scheme, CallScheme::Call | CallScheme::StaticCall),
        )
    );

    // take l64 part of gas_limit
    let mut gas_limit = if SpecId::TANGERINE.enabled_in(SPEC_ID) {
        //EIP-150: Gas cost changes for IO-heavy operations
        let gas = interpreter.gas().remaining();
        min(gas - gas / 64, local_gas_limit)
    } else {
        local_gas_limit
    };

    gas!(interpreter, gas_limit);

    // add call stipend if there is value to be transferred.
    if matches!(scheme, CallScheme::Call | CallScheme::CallCode) && !transfer.value.is_zero() {
        gas_limit = gas_limit.saturating_add(gas::CALL_STIPEND);
    }
    let is_static = matches!(scheme, CallScheme::StaticCall);

    let mut call_input = CallInputs {
        contract: to,
        transfer,
        input,
        gas_limit,
        context,
        is_static,
    };
    // CALL CONTRACT, with static or ordinary spec.
    let (reason, gas, return_data) = host.call(&mut call_input);
    interpreter.return_data_buffer = return_data;

    let target_len = min(out_len, interpreter.return_data_buffer.len());

    match reason {
        return_ok!() => {
            // return unspend gas.
            interpreter.gas.erase_cost(gas.remaining());
            interpreter.gas.record_refund(gas.refunded());
            interpreter
                .memory
                .set(out_offset, &interpreter.return_data_buffer[..target_len]);
            push!(interpreter, U256::one());
        }
        return_revert!() => {
            interpreter.gas.erase_cost(gas.remaining());
            interpreter
                .memory
                .set(out_offset, &interpreter.return_data_buffer[..target_len]);
            push!(interpreter, U256::zero());
        }
        Return::FatalExternalError => return Return::FatalExternalError,
        _ => {
            push!(interpreter, U256::zero());
        }
    }
    interpreter.add_next_gas_block(interpreter.program_counter() - 1)
}
