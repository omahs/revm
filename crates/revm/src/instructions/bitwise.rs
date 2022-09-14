use core::cmp::Ordering;
use std::ops::{BitAnd, BitOr, BitXor};

use crate::{Host, Interpreter, Return};

use super::i256::{i256_cmp, i256_sign, two_compl, Sign};
use primitive_types::U256;

pub fn slt(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if i256_cmp(&mut op1, op2) == Ordering::Less {
        U256::one()
    } else {
        U256::zero()
    };
    Return::Continue
}

pub fn sgt(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if i256_cmp(&mut op1, op2) == Ordering::Greater {
        U256::one()
    } else {
        U256::zero()
    };
    Return::Continue
}

pub fn iszero(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1);
    *op1 = if op1.is_zero() {
        U256::one()
    } else {
        U256::zero()
    };
    Return::Continue
}

pub fn not(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1);
    *op1 = !(*op1);
    Return::Continue
}

pub fn byte(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1, op2);
    let mut ret = U256::zero();
    // TODO optimize, can be better
    for i in 0..256 {
        if i < 8 && op1 < 32.into() {
            let o: usize = op1.as_usize();
            let t = 255 - (7 - i + 8 * o);
            let bit_mask = U256::one() << t;
            let value = (*op2 & bit_mask) >> t;
            ret = ret.overflowing_add(value << i).0;
        }
    }
    *op2 = ret;

    Return::Continue
}

pub fn shl(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, shift, value);
    if value.is_zero() || shift >= U256::from(256) {
        *value = U256::zero()
    } else {
        let shift: u64 = shift.as_u64();
        *value <<= shift as usize
    };
    Return::Continue
}

pub fn shr(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, shift, value);
    if value.is_zero() || shift >= U256::from(256) {
        *value = U256::zero()
    } else {
        let shift: u64 = shift.as_u64();
        *value >>= shift as usize
    }
    Return::Continue
}

pub fn sar(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, shift, value);
    let value_sign = i256_sign::<true>(value);

    *value = if value.is_zero() || shift >= U256::from(256) {
        match value_sign {
            // value is 0 or >=1, pushing 0
            Sign::Plus | Sign::Zero => U256::zero(),
            // value is <0, pushing -1
            Sign::Minus => two_compl(U256::one()),
        }
    } else {
        let shift: u64 = shift.as_u64();

        match value_sign {
            Sign::Plus | Sign::Zero => *value >> shift as usize,
            Sign::Minus => {
                let shifted = ((value.overflowing_sub(U256::one()).0) >> shift as usize)
                    .overflowing_add(U256::one())
                    .0;
                two_compl(shifted)
            }
        }
    };
    Return::Continue
}

pub fn bitand(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = op1.bitand(*op2);
    Return::Continue
}

pub fn bitor(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = op1.bitor(*op2);
    Return::Continue
}

pub fn bitxor(interpreter: &mut Interpreter, _host: &mut dyn Host) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = op1.bitxor(*op2);
    Return::Continue
}
