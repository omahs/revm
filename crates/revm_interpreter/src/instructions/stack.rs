use crate::{interpreter::Interpreter, Return};

pub fn pop(interpreter: &mut Interpreter) -> Return {
    // gas!(interp, gas::BASE);
    interpreter.stack.reduce_one()
}

pub fn push<const N: usize>(interpreter: &mut Interpreter) -> Return {
    // gas!(interp, gas::VERYLOW);
    let start = interpreter.instruction_pointer;
    // Safety: In Analysis we appended needed bytes for bytecode so that we are safe to just add without
    // checking if it is out of bound. This makes both of our unsafes block safe to do.
    let ret = interpreter
        .stack
        .push_slice::<N>(unsafe { core::slice::from_raw_parts(start, N) });
    interpreter.instruction_pointer = unsafe { interpreter.instruction_pointer.add(N) };
    ret
}

pub fn dup<const N: usize>(interpreter: &mut Interpreter) -> Return {
    // gas!(interp, gas::VERYLOW);
    interpreter.stack.dup::<N>()
}

pub fn swap<const N: usize>(interpreter: &mut Interpreter) -> Return {
    // gas!(interp, gas::VERYLOW);
    interpreter.stack.swap::<N>()
}
