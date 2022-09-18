use crate::instructions::{arithmetic, bitwise, control, host, host_env, memory, stack, system};

use crate::Interpreter;
use crate::Return;
use crate::Spec;
use crate::SpecId;
use crate::{SPEC_ID_FRONTIER, SPEC_ID_LATEST};
use once_cell::sync::OnceCell;

type InstructionFn = fn(&mut Interpreter) -> Return;

pub fn return_stop(_: &mut Interpreter) -> Return {
    Return::Stop
}

pub fn invalid_opcode(_: &mut Interpreter) -> Return {
    Return::InvalidOpcode
}

pub fn not_activated(_: &mut Interpreter) -> Return {
    Return::NotActivated
}

pub fn state_change_during_static(_: &mut Interpreter) -> Return {
    Return::StateChangeDuringStaticCall
}

pub fn jump_table<const SPEC_ID: u8, const IS_STATIC: bool>() -> [InstructionFn; 256] {
    [
        /* 0x00  STOP */ return_stop,
        /* 0x01  ADD */ arithmetic::overflowing_add,
        /* 0x02  MUL */ arithmetic::overflowing_mul,
        /* 0x03  SUB */ arithmetic::overflowing_sub,
        /* 0x04  DIV */ arithmetic::div,
        /* 0x05  SDIV */ arithmetic::sdiv,
        /* 0x06  MOD */ arithmetic::rem,
        /* 0x07  SMOD */ arithmetic::smod,
        /* 0x08  ADDMOD */ arithmetic::addmod,
        /* 0x09  MULMOD */ arithmetic::mulmod,
        /* 0x0a  EXP */ arithmetic::eval_exp::<SPEC_ID>,
        /* 0x0b  SIGNEXTEND */ arithmetic::signextend,
        /* 0x0c */ invalid_opcode,
        /* 0x0d */ invalid_opcode,
        /* 0x0e */ invalid_opcode,
        /* 0x0f */ invalid_opcode,
        /* 0x10  LT */ arithmetic::lt,
        /* 0x11  GT */ arithmetic::gt,
        /* 0x12  SLT */ bitwise::slt,
        /* 0x13  SGT */ bitwise::sgt,
        /* 0x14  EQ */ arithmetic::eq,
        /* 0x15  ISZERO */ bitwise::iszero,
        /* 0x16  AND */ bitwise::bitand,
        /* 0x17  OR */ bitwise::bitor,
        /* 0x18  XOR */ bitwise::bitxor,
        /* 0x19  NOT */ bitwise::not,
        /* 0x1a  BYTE */ bitwise::byte,
        /* 0x1b  SHL */
        if SpecId::CONSTANTINOPLE.enabled_in(SPEC_ID) {
            bitwise::shl
        } else {
            not_activated
        },
        /* 0x1c  SHR */
        if SpecId::CONSTANTINOPLE.enabled_in(SPEC_ID) {
            bitwise::shr
        } else {
            not_activated
        },
        /* 0x1d  SAR */
        if SpecId::CONSTANTINOPLE.enabled_in(SPEC_ID) {
            bitwise::sar
        } else {
            not_activated
        },
        /* 0x1e */ invalid_opcode,
        /* 0x1f */ invalid_opcode,
        /* 0x20  SHA3 */ system::sha3,
        /* 0x21 */ invalid_opcode,
        /* 0x22 */ invalid_opcode,
        /* 0x23 */ invalid_opcode,
        /* 0x24 */ invalid_opcode,
        /* 0x25 */ invalid_opcode,
        /* 0x26 */ invalid_opcode,
        /* 0x27 */ invalid_opcode,
        /* 0x28 */ invalid_opcode,
        /* 0x29 */ invalid_opcode,
        /* 0x2a */ invalid_opcode,
        /* 0x2b */ invalid_opcode,
        /* 0x2c */ invalid_opcode,
        /* 0x2d */ invalid_opcode,
        /* 0x2e */ invalid_opcode,
        /* 0x2f */ invalid_opcode,
        /* 0x30  ADDRESS */ system::address,
        /* 0x31  BALANCE */ host::balance::<SPEC_ID>,
        /* 0x32  ORIGIN */ host_env::origin,
        /* 0x33  CALLER */ system::caller,
        /* 0x34  CALLVALUE */ system::callvalue,
        /* 0x35  CALLDATALOAD */ system::calldataload,
        /* 0x36  CALLDATASIZE */ system::calldatasize, // TODO
        /* 0x37  CALLDATACOPY */ system::calldatacopy,
        /* 0x38  CODESIZE */ system::codesize,
        /* 0x39  CODECOPY */ system::codecopy,
        /* 0x3a  GASPRICE */ host_env::gasprice,
        /* 0x3b  EXTCODESIZE */
        host::extcodesize::<SPEC_ID>,
        /* 0x3c  EXTCODECOPY */
        if SpecId::CONSTANTINOPLE.enabled_in(SPEC_ID) {
            // EIP-1052: EXTCODEHASH opcode
            host::extcodecopy::<SPEC_ID>
        } else {
            not_activated
        },
        /* 0x3d  RETURNDATASIZE */
        if SpecId::BYZANTIUM.enabled_in(SPEC_ID) {
            // EIP-211: New opcodes: RETURNDATASIZE and RETURNDATACOPY
            system::returndatasize
        } else {
            not_activated
        },
        /* 0x3e  RETURNDATACOPY */
        if SpecId::BYZANTIUM.enabled_in(SPEC_ID) {
            // EIP-211: New opcodes: RETURNDATASIZE and RETURNDATACOPY
            system::returndatacopy
        } else {
            not_activated
        },
        /* 0x3f  EXTCODEHASH */
        if SpecId::CONSTANTINOPLE.enabled_in(SPEC_ID) {
            // EIP-1052: EXTCODEHASH opcode
            host::extcodehash::<SPEC_ID>
        } else {
            not_activated
        },
        /* 0x40  BLOCKHASH */ host::blockhash,
        /* 0x41  COINBASE */ host_env::coinbase,
        /* 0x42  TIMESTAMP */ host_env::timestamp,
        /* 0x43  NUMBER */ host_env::number,
        /* 0x44  DIFFICULTY */ host_env::difficulty,
        /* 0x45  GASLIMIT */ host_env::gaslimit,
        /* 0x46  CHAINID */
        if SpecId::ISTANBUL.enabled_in(SPEC_ID) {
            // EIP-1344: ChainID opcode
            host_env::chainid
        } else {
            not_activated
        },
        /* 0x47  SELFBALANCE */
        if SpecId::ISTANBUL.enabled_in(SPEC_ID) {
            // EIP-1884: Repricing for trie-size-dependent opcodes
            host::selfbalance::<SPEC_ID>
        } else {
            not_activated
        },
        /* 0x48  BASEFEE */
        if SpecId::LONDON.enabled_in(SPEC_ID) {
            // EIP-1884: Repricing for trie-size-dependent opcodes
            host_env::basefee
        } else {
            not_activated
        },
        /* 0x49 */ invalid_opcode,
        /* 0x4a */ invalid_opcode,
        /* 0x4b */ invalid_opcode,
        /* 0x4c */ invalid_opcode,
        /* 0x4d */ invalid_opcode,
        /* 0x4e */ invalid_opcode,
        /* 0x4f */ invalid_opcode,
        /* 0x50  POP */ stack::pop,
        /* 0x51  MLOAD */ memory::mload,
        /* 0x52  MSTORE */ memory::mstore,
        /* 0x53  MSTORE8 */ memory::mstore8,
        /* 0x54  SLOAD */ host::sload::<SPEC_ID>,
        /* 0x55  SSTORE */ host::sstore::<SPEC_ID>,
        /* 0x56  JUMP */ control::jump,
        /* 0x57  JUMPI */ control::jumpi,
        /* 0x58  PC */ control::pc,
        /* 0x59  MSIZE */ memory::msize,
        /* 0x5a  GAS */ system::gas,
        /* 0x5b  JUMPDEST */ control::jumpdest,
        /* 0x5c */ invalid_opcode,
        /* 0x5d */ invalid_opcode,
        /* 0x5e */ invalid_opcode,
        /* 0x5f */ invalid_opcode,
        /* 0x60  PUSH1 */ stack::push::<1>,
        /* 0x61  PUSH2 */ stack::push::<2>,
        /* 0x62  PUSH3 */ stack::push::<3>,
        /* 0x63  PUSH4 */ stack::push::<4>,
        /* 0x64  PUSH5 */ stack::push::<5>,
        /* 0x65  PUSH6 */ stack::push::<6>,
        /* 0x66  PUSH7 */ stack::push::<7>,
        /* 0x67  PUSH8 */ stack::push::<8>,
        /* 0x68  PUSH9 */ stack::push::<9>,
        /* 0x69  PUSH10 */ stack::push::<10>,
        /* 0x6a  PUSH11 */ stack::push::<11>,
        /* 0x6b  PUSH12 */ stack::push::<12>,
        /* 0x6c  PUSH13 */ stack::push::<13>,
        /* 0x6d  PUSH14 */ stack::push::<14>,
        /* 0x6e  PUSH15 */ stack::push::<15>,
        /* 0x6f  PUSH16 */ stack::push::<16>,
        /* 0x70  PUSH17 */ stack::push::<17>,
        /* 0x71  PUSH18 */ stack::push::<18>,
        /* 0x72  PUSH19 */ stack::push::<19>,
        /* 0x73  PUSH20 */ stack::push::<20>,
        /* 0x74  PUSH21 */ stack::push::<21>,
        /* 0x75  PUSH22 */ stack::push::<22>,
        /* 0x76  PUSH23 */ stack::push::<23>,
        /* 0x77  PUSH24 */ stack::push::<24>,
        /* 0x78  PUSH25 */ stack::push::<25>,
        /* 0x79  PUSH26 */ stack::push::<26>,
        /* 0x7a  PUSH27 */ stack::push::<27>,
        /* 0x7b  PUSH28 */ stack::push::<28>,
        /* 0x7c  PUSH29 */ stack::push::<29>,
        /* 0x7d  PUSH30 */ stack::push::<30>,
        /* 0x7e  PUSH31 */ stack::push::<31>,
        /* 0x7f  PUSH32 */ stack::push::<32>,
        /* 0x80  DUP1 */ stack::dup::<1>,
        /* 0x81  DUP2 */ stack::dup::<2>,
        /* 0x82  DUP3 */ stack::dup::<3>,
        /* 0x83  DUP4 */ stack::dup::<4>,
        /* 0x84  DUP5 */ stack::dup::<5>,
        /* 0x85  DUP6 */ stack::dup::<6>,
        /* 0x86  DUP7 */ stack::dup::<7>,
        /* 0x87  DUP8 */ stack::dup::<8>,
        /* 0x88  DUP9 */ stack::dup::<9>,
        /* 0x89  DUP10 */ stack::dup::<10>,
        /* 0x8a  DUP11 */ stack::dup::<11>,
        /* 0x8b  DUP12 */ stack::dup::<12>,
        /* 0x8c  DUP13 */ stack::dup::<13>,
        /* 0x8d  DUP14 */ stack::dup::<14>,
        /* 0x8e  DUP15 */ stack::dup::<15>,
        /* 0x8f  DUP16 */ stack::dup::<16>,
        /* 0x90  SWAP1 */ stack::swap::<1>,
        /* 0x91  SWAP2 */ stack::swap::<2>,
        /* 0x92  SWAP3 */ stack::swap::<3>,
        /* 0x93  SWAP4 */ stack::swap::<4>,
        /* 0x94  SWAP5 */ stack::swap::<5>,
        /* 0x95  SWAP6 */ stack::swap::<6>,
        /* 0x96  SWAP7 */ stack::swap::<7>,
        /* 0x97  SWAP8 */ stack::swap::<8>,
        /* 0x98  SWAP9 */ stack::swap::<9>,
        /* 0x99  SWAP10 */ stack::swap::<10>,
        /* 0x9a  SWAP11 */ stack::swap::<11>,
        /* 0x9b  SWAP12 */ stack::swap::<12>,
        /* 0x9c  SWAP13 */ stack::swap::<13>,
        /* 0x9d  SWAP14 */ stack::swap::<14>,
        /* 0x9e  SWAP15 */ stack::swap::<15>,
        /* 0x9f  SWAP16 */ stack::swap::<16>,
        /* 0xa0  LOG0 */
        if !IS_STATIC {
            host::log::<SPEC_ID, 0>
        } else {
            state_change_during_static
        },
        /* 0xa1  LOG1 */
        if !IS_STATIC {
            host::log::<SPEC_ID, 1>
        } else {
            state_change_during_static
        },
        /* 0xa2  LOG2 */
        if !IS_STATIC {
            host::log::<SPEC_ID, 2>
        } else {
            state_change_during_static
        },
        /* 0xa3  LOG3 */
        if !IS_STATIC {
            host::log::<SPEC_ID, 3>
        } else {
            state_change_during_static
        },
        /* 0xa4  LOG4 */
        if !IS_STATIC {
            host::log::<SPEC_ID, 4>
        } else {
            state_change_during_static
        },
        /* 0xa5 */ invalid_opcode,
        /* 0xa6 */ invalid_opcode,
        /* 0xa7 */ invalid_opcode,
        /* 0xa8 */ invalid_opcode,
        /* 0xa9 */ invalid_opcode,
        /* 0xaa */ invalid_opcode,
        /* 0xab */ invalid_opcode,
        /* 0xac */ invalid_opcode,
        /* 0xad */ invalid_opcode,
        /* 0xae */ invalid_opcode,
        /* 0xaf */ invalid_opcode,
        /* 0xb0 */ invalid_opcode,
        /* 0xb1 */ invalid_opcode,
        /* 0xb2 */ invalid_opcode,
        /* 0xb3 */ invalid_opcode,
        /* 0xb4 */ invalid_opcode,
        /* 0xb5 */ invalid_opcode,
        /* 0xb6 */ invalid_opcode,
        /* 0xb7 */ invalid_opcode,
        /* 0xb8 */ invalid_opcode,
        /* 0xb9 */ invalid_opcode,
        /* 0xba */ invalid_opcode,
        /* 0xbb */ invalid_opcode,
        /* 0xbc */ invalid_opcode,
        /* 0xbd */ invalid_opcode,
        /* 0xbe */ invalid_opcode,
        /* 0xbf */ invalid_opcode,
        /* 0xc0 */ invalid_opcode,
        /* 0xc1 */ invalid_opcode,
        /* 0xc2 */ invalid_opcode,
        /* 0xc3 */ invalid_opcode,
        /* 0xc4 */ invalid_opcode,
        /* 0xc5 */ invalid_opcode,
        /* 0xc6 */ invalid_opcode,
        /* 0xc7 */ invalid_opcode,
        /* 0xc8 */ invalid_opcode,
        /* 0xc9 */ invalid_opcode,
        /* 0xca */ invalid_opcode,
        /* 0xcb */ invalid_opcode,
        /* 0xcc */ invalid_opcode,
        /* 0xcd */ invalid_opcode,
        /* 0xce */ invalid_opcode,
        /* 0xcf */ invalid_opcode,
        /* 0xd0 */ invalid_opcode,
        /* 0xd1 */ invalid_opcode,
        /* 0xd2 */ invalid_opcode,
        /* 0xd3 */ invalid_opcode,
        /* 0xd4 */ invalid_opcode,
        /* 0xd5 */ invalid_opcode,
        /* 0xd6 */ invalid_opcode,
        /* 0xd7 */ invalid_opcode,
        /* 0xd8 */ invalid_opcode,
        /* 0xd9 */ invalid_opcode,
        /* 0xda */ invalid_opcode,
        /* 0xdb */ invalid_opcode,
        /* 0xdc */ invalid_opcode,
        /* 0xdd */ invalid_opcode,
        /* 0xde */ invalid_opcode,
        /* 0xdf */ invalid_opcode,
        /* 0xe0 */ invalid_opcode,
        /* 0xe1 */ invalid_opcode,
        /* 0xe2 */ invalid_opcode,
        /* 0xe3 */ invalid_opcode,
        /* 0xe4 */ invalid_opcode,
        /* 0xe5 */ invalid_opcode,
        /* 0xe6 */ invalid_opcode,
        /* 0xe7 */ invalid_opcode,
        /* 0xe8 */ invalid_opcode,
        /* 0xe9 */ invalid_opcode,
        /* 0xea */ invalid_opcode,
        /* 0xeb */ invalid_opcode,
        /* 0xec */ invalid_opcode,
        /* 0xed */ invalid_opcode,
        /* 0xee */ invalid_opcode,
        /* 0xef */ invalid_opcode,
        /* 0xf0  CREATE */
        if !IS_STATIC {
            host::create::<SPEC_ID, false>
        } else {
            state_change_during_static
        },
        /* 0xf1  CALL */ host::call::<SPEC_ID, IS_STATIC>,
        /* 0xf2  CALLCODE */ host::callcode::<SPEC_ID, IS_STATIC>,
        /* 0xf3  RETURN */ control::ret,
        /* 0xf4  DELEGATECALL */ host::delegatecall::<SPEC_ID, IS_STATIC>,
        /* 0xf5  CREATE2 */
        if !IS_STATIC {
            host::create::<SPEC_ID, true>
        } else {
            state_change_during_static
        },
        /* 0xf6 */ invalid_opcode,
        /* 0xf7 */ invalid_opcode,
        /* 0xf8 */ invalid_opcode,
        /* 0xf9 */ invalid_opcode,
        /* 0xfa  STATICCALL */ host::staticcall::<SPEC_ID, IS_STATIC>,
        /* 0xfb */ invalid_opcode,
        /* 0xfc */ invalid_opcode,
        /* 0xfd  REVERT */
        if SpecId::BYZANTIUM.enabled_in(SPEC_ID) {
            //EIP-140: REVERT instruction,
            control::revert
        } else {
            not_activated
        },
        /* 0xfe  INVALID */ invalid_opcode,
        /* 0xff  SELFDESTRUCT */
        if !IS_STATIC {
            host::selfdestruct::<SPEC_ID>
        } else {
            state_change_during_static
        },
    ]
}

pub fn opcode_jump_table<const IS_STATIC: bool, SPEC: Spec>() -> &'static [InstructionFn; 256] {
    match SPEC::SPEC_ID {
        SpecId::FRONTIER => {
            static INSTANCE: OnceCell<[InstructionFn; 256]> = OnceCell::new();
            INSTANCE.get_or_init(|| jump_table::<SPEC_ID_FRONTIER, IS_STATIC>())
        }
        _ => {
            static INSTANCE: OnceCell<[InstructionFn; 256]> = OnceCell::new();
            INSTANCE.get_or_init(|| jump_table::<SPEC_ID_LATEST, IS_STATIC>())
        }
    }
}
