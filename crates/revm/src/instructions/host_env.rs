use crate::{Interpreter, Host, Return};
use primitive_types::H256;

pub fn chainid(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    // EIP-1344: ChainID opcode
    //check!(SpecId::ISTANBUL.enabled_in(SPEC_ID));
    push!(interpteret, host.env().cfg.chain_id);
    Return::Continue
}

pub fn coinbase(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    push_h256!(interpteret, host.env().block.coinbase.into());
    Return::Continue
}

pub fn timestamp(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    push!(interpteret, host.env().block.timestamp);
    Return::Continue
}

pub fn number(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    push!(interpteret, host.env().block.number);
    Return::Continue
}

pub fn difficulty(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    push!(interpteret, host.env().block.difficulty);
    Return::Continue
}

pub fn gaslimit(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    push!(interpteret, host.env().block.gas_limit);
    Return::Continue
}

pub fn gasprice(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    push!(interpteret, host.env().effective_gas_price());
    Return::Continue
}

pub fn basefee(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    // EIP-3198: BASEFEE opcode
    //check!(SpecId::LONDON.enabled_in(SPEC_ID));
    push!(interpteret, host.env().block.basefee);
    Return::Continue
}

pub fn origin(interpteret: &mut Interpreter, host: &mut dyn Host) -> Return {
    // gas!(interpteret, gas::BASE);
    let ret = H256::from(host.env().tx.caller);
    push_h256!(interpteret, ret);
    Return::Continue
}
