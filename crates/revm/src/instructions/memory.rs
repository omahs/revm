use crate::{interpreter::Interpreter, Return, Host};
use primitive_types::U256;

pub fn mload(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index);
    let index = as_usize_or_fail!(index, Return::OutOfGas);
    memory_resize!(interpreter, index, 32);
    push!(
        interpreter,
        U256::from_big_endian(interpreter.memory.get_slice(index, 32))
    );
    Return::Continue
}

pub fn mstore(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index, value);
    let index = as_usize_or_fail!(index, Return::OutOfGas);
    memory_resize!(interpreter, index, 32);
    interpreter.memory.set_u256(index, value);
    Return::Continue
}

pub fn mstore8(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index, value);
    let index = as_usize_or_fail!(index, Return::OutOfGas);
    memory_resize!(interpreter, index, 1);
    let value = (value.low_u32() & 0xff) as u8;
    // Safety: we resized our memory two lines above.
    unsafe { interpreter.memory.set_byte(index, value) }
    Return::Continue
}

pub fn msize(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.memory.effective_len()));
    Return::Continue
}
