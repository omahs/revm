use std::cmp::min;

use crate::{gas, Interpreter, Host, Return, Spec, SpecId::*, KECCAK_EMPTY};
use primitive_types::{H256, U256};

use sha3::{Digest, Keccak256};

pub fn sha3(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop!(interpreter, from, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);
    gas_or_fail!(interpreter, gas::sha3_cost(len as u64));
    let h256 = if len == 0 {
        KECCAK_EMPTY
    } else {
        let from = as_usize_or_fail!(from, Return::OutOfGas);
        memory_resize!(interpreter, from, len);
        H256::from_slice(Keccak256::digest(interpreter.memory.get_slice(from, len)).as_slice())
    };

    push_h256!(interpreter, h256);
    Return::Continue
}

pub fn address(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    let ret = H256::from(interpreter.contract.address);
    push_h256!(interpreter, ret);
    Return::Continue
}

pub fn caller(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    let ret = H256::from(interpreter.contract.caller);
    push_h256!(interpreter, ret);
    Return::Continue
}

pub fn codesize(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    let size = U256::from(interpreter.contract.bytecode.len());
    push!(interpreter, size);
    Return::Continue
}

pub fn codecopy(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop!(interpreter, memory_offset, code_offset, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);
    gas_or_fail!(interpreter, gas::verylowcopy_cost(len as u64));
    if len == 0 {
        return Return::Continue;
    }
    let memory_offset = as_usize_or_fail!(memory_offset, Return::OutOfGas);
    let code_offset = as_usize_saturated!(code_offset);
    memory_resize!(interpreter, memory_offset, len);

    // Safety: set_data is unsafe function and memory_resize ensures us that it is safe to call it
    interpreter.memory.set_data(
        memory_offset,
        code_offset,
        len,
        interpreter.contract.bytecode.original_bytecode_slice(),
    );
    Return::Continue
}

pub fn calldataload(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index);
    let index = as_usize_saturated!(index);

    let load = if index < interpreter.contract.input.len() {
        let mut load = H256::zero();
        let have_bytes = min(interpreter.contract.input.len() - index, 32);
        load.0[..have_bytes].copy_from_slice(&interpreter.contract.input[index..index + have_bytes]);
        load
    } else {
        H256::zero()
    };

    push_h256!(interpreter, load);
    Return::Continue
}

pub fn calldatasize(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    let len = U256::from(interpreter.contract.input.len());
    push!(interpreter, len);
    Return::Continue
}

pub fn callvalue(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    let mut ret = H256::default();
    interpreter.contract.value.to_big_endian(&mut ret[..]);
    push_h256!(interpreter, ret);
    Return::Continue
}

pub fn calldatacopy(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop!(interpreter, memory_offset, data_offset, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);
    gas_or_fail!(interpreter, gas::verylowcopy_cost(len as u64));
    if len == 0 {
        return Return::Continue;
    }
    let memory_offset = as_usize_or_fail!(memory_offset, Return::OutOfGas);
    let data_offset = as_usize_saturated!(data_offset);
    memory_resize!(interpreter, memory_offset, len);

    // Safety: set_data is unsafe function and memory_resize ensures us that it is safe to call it
    interpreter
        .memory
        .set_data(memory_offset, data_offset, len, &interpreter.contract.input);
    Return::Continue
}

pub fn returndatasize(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    // EIP-211: New opcodes: RETURNDATASIZE and RETURNDATACOPY
    //check!(SpecId::BYZANTIUM.enabled_in(SPEC_ID));
    let size = U256::from(interpreter.return_data_buffer.len());
    push!(interpreter, size);
    Return::Continue
}

pub fn returndatacopy(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // EIP-211: New opcodes: RETURNDATASIZE and RETURNDATACOPY
    //check!(SpecId::BYZANTIUM.enabled_in(SPEC_ID));
    pop!(interpreter, memory_offset, offset, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);
    gas_or_fail!(interpreter, gas::verylowcopy_cost(len as u64));
    let memory_offset = as_usize_or_fail!(memory_offset, Return::OutOfGas);
    let data_offset = as_usize_saturated!(offset);
    memory_resize!(interpreter, memory_offset, len);
    let (data_end, overflow) = data_offset.overflowing_add(len);
    if overflow || data_end > interpreter.return_data_buffer.len() {
        return Return::OutOfOffset;
    }
    interpreter.memory.set(
        memory_offset,
        &interpreter.return_data_buffer[data_offset..data_end],
    );
    Return::Continue
}

pub fn gas(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.gas.remaining()));
    interpreter.add_next_gas_block(interpreter.program_counter() - 1)
}
