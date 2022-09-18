use crate::{interpreter::Interpreter, Return};
use primitive_types::U256;

pub fn mload(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index);
    let index = as_usize_or_fail!(index, Return::OutOfGas);
    memory_resize!(interpreter, index, 32);

    use byteorder::{BigEndian, ByteOrder};
    let slice = interpreter.memory.get_slice(index, 32);
    push!(
        interpreter,
        U256([
            BigEndian::read_u64(&slice[24..32]),
            BigEndian::read_u64(&slice[16..24]),
            BigEndian::read_u64(&slice[8..16]),
            BigEndian::read_u64(&slice[0..8])
        ]) //]BigEndian::read_u64interpreter.memory.get_slice(index, 32))
    );
    Return::Continue
}

pub fn mstore(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index, value);
    let index = as_usize_or_fail!(index, Return::OutOfGas);
    memory_resize!(interpreter, index, 32);
    interpreter.memory.set_u256(index, value);
    Return::Continue
}

pub fn mstore8(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index, value);
    let index = as_usize_or_fail!(index, Return::OutOfGas);
    memory_resize!(interpreter, index, 1);
    let value = (value.low_u32() & 0xff) as u8;
    // Safety: we resized our memory two lines above.
    unsafe { interpreter.memory.set_byte(index, value) }
    Return::Continue
}

pub fn msize(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.memory.len()));
    Return::Continue
}
