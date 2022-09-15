use crate::{
    gas, Host, Interpreter, Return, Spec,
    SpecId::{self, *},
};
use primitive_types::U256;

pub fn jump(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::MID);
    pop!(interpreter, dest);
    let dest = as_usize_or_fail!(dest, Return::InvalidJump);
    if interpreter.contract.is_valid_jump(dest) {
        // Safety: In analysis we are checking create our jump table and we do check above to be
        // sure that jump is safe to execute.
        interpreter.instruction_pointer =
            unsafe { interpreter.contract.bytecode.as_ptr().add(dest) };
        Return::Continue
    } else {
        Return::InvalidJump
    }
}

pub fn jumpi(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::HIGH);
    pop!(interpreter, dest, value);
    if !value.is_zero() {
        let dest = as_usize_or_fail!(dest, Return::InvalidJump);
        if interpreter.contract.is_valid_jump(dest) {
            // Safety: In analysis we are checking if jump is valid destination and
            // this `if` makes this unsafe block safe.
            interpreter.instruction_pointer =
                unsafe { interpreter.contract.bytecode.as_ptr().add(dest) };
            Return::Continue
        } else {
            Return::InvalidJump
        }
    } else {
        // if we are not doing jump, add next gas block.
        interpreter.add_next_gas_block(interpreter.program_counter() - 1)
    }
}

pub fn jumpdest(interpreter: &mut Interpreter) -> Return {
    //gas!(interpreter, gas::JUMPDEST);
    interpreter.add_next_gas_block(interpreter.program_counter() - 1)
}

pub fn pc(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.program_counter() - 1));
    Return::Continue
}

pub fn ret(interpreter: &mut Interpreter) -> Return {
    // zero gas cost gas!(interpreter,gas::ZERO);
    pop!(interpreter, start, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);
    if len == 0 {
        interpreter.return_range = usize::MAX..usize::MAX;
    } else {
        let offset = as_usize_or_fail!(start, Return::OutOfGas);
        memory_resize!(interpreter, offset, len);
        interpreter.return_range = offset..(offset + len);
    }
    Return::Return
}

/// Opcode enabled in BYZANTIUM: EIP-140: REVERT instruction
pub fn revert(interpreter: &mut Interpreter) -> Return {
    // zero gas cost gas!(interpreter,gas::ZERO);
    pop!(interpreter, start, len);
    let len = as_usize_or_fail!(len, Return::OutOfGas);
    if len == 0 {
        interpreter.return_range = usize::MAX..usize::MAX;
    } else {
        let offset = as_usize_or_fail!(start, Return::OutOfGas);
        memory_resize!(interpreter, offset, len);
        interpreter.return_range = offset..(offset + len);
    }
    Return::Revert
}
