#![allow(dead_code)]
//#![no_std]

pub mod db;
mod evm;
mod evm_impl;
pub(crate) mod gas;
mod inspector;
mod instructions;
mod interpreter;
mod journaled_state;
mod models;
mod specification;

pub use evm_impl::{create2_address, create_address, EVMData, Host};

pub type DummyStateDB = InMemoryDB;

pub use db::{Database, DatabaseCommit, InMemoryDB};
pub use evm::{evm_inner, new, EVM};
pub use gas::Gas;
pub use inspector::{Inspector, NoOpInspector};
pub use instructions::{
    opcode::{self, opcode_info_table, opcode_jump_table, OpCode, OPCODE_JUMPMAP},
    Return,
};
pub use interpreter::{
    Bytecode, BytecodeLocked, BytecodeState, Contract, Interpreter, Memory, Stack,
};
pub use journaled_state::{Account, JournalEntry, JournaledState};
pub use models::*;
pub use specification::*;

extern crate alloc;

pub(crate) const USE_GAS: bool = !cfg!(feature = "no_gas_measuring");

<<<<<<< HEAD
// reexport `revm_precompiles`
pub mod precompiles {
    pub use revm_precompiles::*;
}
=======

#[repr(C)]
#[derive(Debug)]
pub struct RetUint {
    pub n1: u64,
    pub n2: u64,
    pub n3: u64,
    pub n4: u64,
}

#[link(name = "intx")]
extern "C" {
    pub fn fast_div_rem(f: *const u64, s: *const u64) -> RetUint;
}


use primitive_types::{U256,H256};

pub fn test_it() {
    let f = U256::from_big_endian(H256::from_low_u64_be(20).as_ref());
    let s = U256::from_big_endian(H256::from_low_u64_be(10).as_ref());

    let t = unsafe { fast_div_rem(f.as_ref().as_ptr(),s.as_ref().as_ptr())};
    println!("TEST_IT:{:?}",t);

} 
>>>>>>> origin/intx
