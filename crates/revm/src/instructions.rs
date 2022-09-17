#[macro_use]
mod macros;
mod arithmetic;
mod bitwise;
mod control;
mod host;
mod host_env;
mod i256;
mod memory;
pub mod opcode;
mod stack;
mod system;

pub use opcode::{OpCode, OPCODE_JUMPMAP};

use crate::{interpreter::Interpreter, CallScheme, Host, Spec, SpecId::*, SPEC_ID_LONDON};
use core::ops::{BitAnd, BitOr, BitXor};
use primitive_types::U256;

#[macro_export]
macro_rules! return_ok {
    () => {
        Return::Continue | Return::Stop | Return::Return | Return::SelfDestruct
    };
}

#[macro_export]
macro_rules! return_revert {
    () => {
        Return::Revert | Return::CallTooDeep | Return::OutOfFund
    };
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Return {
    //success codes
    Continue = 0x00,
    Stop = 0x01,
    Return = 0x02,
    SelfDestruct = 0x03,

    // revert code
    Revert = 0x20, // revert opcode
    CallTooDeep = 0x21,
    OutOfFund = 0x22,

    // error codes
    OutOfGas = 0x50,
    OpcodeNotFound,
    CallNotAllowedInsideStatic,
    InvalidOpcode,
    InvalidJump,
    InvalidMemoryRange,
    NotActivated,
    StackUnderflow,
    StackOverflow,
    OutOfOffset,
    FatalExternalError,
    GasMaxFeeGreaterThanPriorityFee,
    GasPriceLessThenBasefee,
    CallerGasLimitMoreThenBlock,
    /// EIP-3607 Reject transactions from senders with deployed code
    RejectCallerWithCode,
    LackOfFundForGasLimit,
    CreateCollision,
    OverflowPayment,
    PrecompileError,
    NonceOverflow,
    /// Create init code exceeds limit (runtime).
    CreateContractLimit,
    /// Error on created contract that begins with EF
    CreateContractWithEF,
    /// Thrown if state change is attempted (through sstore, log, selfdestruct, or create) inside during static call. 
    StateChangeDuringStaticCall,
}

pub const fn u256_one() -> U256 {
    make_u256(1)
}

pub const fn u256_zero() -> U256 {
    make_u256(0)
}

pub const fn make_u256(x: u64) -> U256 {
    U256([x,0,0,0])
}