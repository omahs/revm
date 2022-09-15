use crate::{Host, Interpreter, Return};
use primitive_types::H256;

/// Opcode enabled in ISTANBUL: EIP-1344: ChainID opcode
pub fn chainid(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.host.env().cfg.chain_id);
    Return::Continue
}

pub fn coinbase(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push_h256!(interpreter, interpreter.host.env().block.coinbase.into());
    Return::Continue
}

pub fn timestamp(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.host.env().block.timestamp);
    Return::Continue
}

pub fn number(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.host.env().block.number);
    Return::Continue
}

pub fn difficulty(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.host.env().block.difficulty);
    Return::Continue
}

pub fn gaslimit(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.host.env().block.gas_limit);
    Return::Continue
}

pub fn gasprice(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.host.env().effective_gas_price());
    Return::Continue
}

/// Opcode enabled in LONDON: EIP-3198: BASEFEE opcode
pub fn basefee(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.host.env().block.basefee);
    Return::Continue
}

pub fn origin(interpreter: &mut Interpreter) -> Return {
    // gas!(interpreter, gas::BASE);
    let ret = H256::from(interpreter.host.env().tx.caller);
    push_h256!(interpreter, ret);
    Return::Continue
}
