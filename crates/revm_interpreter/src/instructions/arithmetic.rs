use crate::{gas, Host, Interpreter, Return, Spec, SpecId, instructions::{u256_zero, u256_one}};

use super::i256::{i256_div, i256_mod};
use core::{convert::TryInto, ops::Rem};
use primitive_types::{U256, U512};

pub fn overflowing_add(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    let (ret, ..) = op1.overflowing_add(*op2);
    *op2 = ret;
    Return::Continue
}

pub fn overflowing_mul(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    let (ret, ..) = op1.overflowing_mul(*op2);
    *op2 = ret;
    Return::Continue
}

pub fn overflowing_sub(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    let (ret, ..) = op1.overflowing_sub(*op2);
    *op2 = ret;
    Return::Continue
}

pub fn lt(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if op1.lt(op2) {
        u256_one()
    } else {
        u256_zero()
    };
    Return::Continue
}

pub fn gt(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if op1.gt(op2) {
        u256_one()
    } else {
        u256_zero()
    };
    Return::Continue
}

pub fn eq(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if op1.eq(op2) {
        u256_one()
    } else {
        u256_zero()
    };
    Return::Continue
}

/*
opcode::ADD => op2_u256_tuple!(interp, overflowing_add),
opcode::MUL => op2_u256_tuple!(interp, overflowing_mul),
opcode::SUB => op2_u256_tuple!(interp, overflowing_sub),
opcode::DIV => op2_u256_fn!(interp, arithmetic::div),
opcode::SDIV => op2_u256_fn!(interp, arithmetic::sdiv),
opcode::MOD => op2_u256_fn!(interp, arithmetic::rem),
opcode::SMOD => op2_u256_fn!(interp, arithmetic::smod),
opcode::ADDMOD => op3_u256_fn!(interp, arithmetic::addmod),
opcode::MULMOD => op3_u256_fn!(interp, arithmetic::mulmod),
opcode::EXP => arithmetic::eval_exp::<S>(interp),
opcode::SIGNEXTEND => op2_u256_fn!(interp, arithmetic::signextend),
opcode::LT => op2_u256_bool_ref!(interp, lt),
opcode::GT => op2_u256_bool_ref!(interp, gt),
opcode::SLT => op2_u256_fn!(interp, bitwise::slt),
opcode::SGT => op2_u256_fn!(interp, bitwise::sgt),
opcode::EQ => op2_u256_bool_ref!(interp, eq),
opcode::ISZERO => op1_u256_fn!(interp, bitwise::iszero),
opcode::AND => op2_u256!(interp, bitand),
opcode::OR => op2_u256!(interp, bitor),
opcode::XOR => op2_u256!(interp, bitxor),
opcode::NOT => op1_u256_fn!(interp, bitwise::not),
opcode::BYTE => op2_u256_fn!(interp, bitwise::byte),
 */
pub fn div(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if op2.is_zero() {
        u256_zero()
    } else {
        //op1 / op2
        //super::i256::div_u256::div_mod(op1, op2).0
        let t = unsafe { crate::fast_div_rem(op1.as_ref().as_ptr(), op2.as_ref().as_ptr()) };
        U256([t.n1, t.n2, t.n3, t.n4])
    };
    Return::Continue
}

pub fn sdiv(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    i256_div(op1, op2);
    Return::Continue
}

pub fn rem(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if op2.is_zero() {
        u256_zero()
    } else {
        op1.rem(*op2)
    };
    Return::Continue
}

pub fn smod(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    *op2 = if op2.is_zero() {
        u256_zero()
    } else {
        i256_mod(op1, *op2)
    };
    Return::Continue
}

pub fn addmod(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2, op3);
    *op3 = if op3.is_zero() {
        u256_zero()
    } else {
        let op1: U512 = op1.into();
        let op2: U512 = op2.into();
        let op3: U512 = (*op3).into();
        let v = (op1 + op2) % op3;
        v.try_into()
            .expect("op3 is less than U256::MAX, thus it never overflows; qed")
    };
    Return::Continue
}

pub fn mulmod(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2, op3);
    *op3 = if op3.is_zero() {
        u256_zero()
    } else {
        let op1: U512 = op1.into();
        let op2: U512 = op2.into();
        let op3: U512 = (*op3).into();
        let v = (op1 * op2) % op3;
        v.try_into()
            .expect("op3 is less than U256::MAX, thus it never overflows; qed")
    };
    Return::Continue
}

pub fn eval_exp<const SPEC_ID: u8>(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    gas_or_fail!(interpreter, gas::exp_cost::<SPEC_ID>(*op2));
    let mut op1 = op1;
    let r = op2;
    let mut op2 = r.clone();
    *r = 1.into();
    // TODO check logic to be sure

    while op2 != 0.into() {
        if op2 & 1.into() != 0.into() {
            *r = r.overflowing_mul(op1).0;
        }
        op2 >>= 1;
        op1 = op1.overflowing_mul(op1).0;
    }
    Return::Continue
}

/// In the yellow paper `SIGNEXTEND` is defined to take two inputs, we will call them
/// `x` and `y`, and produce one output. The first `t` bits of the output (numbering from the
/// left, starting from 0) are equal to the `t`-th bit of `y`, where `t` is equal to
/// `256 - 8(x + 1)`. The remaining bits of the output are equal to the corresponding bits of `y`.
/// Note: if `x >= 32` then the output is equal to `y` since `t <= 0`. To efficiently implement
/// this algorithm in the case `x < 32` we do the following. Let `b` be equal to the `t`-th bit
/// of `y` and let `s = 255 - t = 8x + 7` (this is effectively the same index as `t`, but
/// numbering the bits from the right instead of the left). We can create a bit mask which is all
/// zeros up to and including the `t`-th bit, and all ones afterwards by computing the quantity
/// `2^s - 1`. We can use this mask to compute the output depending on the value of `b`.
/// If `b == 1` then the yellow paper says the output should be all ones up to
/// and including the `t`-th bit, followed by the remaining bits of `y`; this is equal to
/// `y | !mask` where `|` is the bitwise `OR` and `!` is bitwise negation. Similarly, if
/// `b == 0` then the yellow paper says the output should start with all zeros, then end with
/// bits from `b`; this is equal to `y & mask` where `&` is bitwise `AND`.

pub fn signextend(interpreter: &mut Interpreter) -> Return {
    pop_top!(interpreter, op1, op2);
    if op1 < U256::from(32) {
        // `low_u32` works since op1 < 32
        let bit_index = (8 * op1.low_u32() + 7) as usize;
        let bit = op2.bit(bit_index);
        let mask = (u256_one() << bit_index) - u256_one();
        *op2 = if bit { *op2 | !mask } else { *op2 & mask };
    }
    Return::Continue
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    // use super::{signextend, U256};

    // /// Test to ensure new (optimized) `signextend` implementation is equivalent to the previous
    // /// implementation.
    // #[test]
    // fn test_signextend() {
    //     let test_values = vec![
    //         u256_zero(),
    //         u256_one(),
    //         U256::from(8),
    //         U256::from(10),
    //         U256::from(65),
    //         U256::from(100),
    //         U256::from(128),
    //         U256::from(11) * (u256_one() << 65),
    //         U256::from(7) * (u256_one() << 123),
    //         U256::MAX / 167,
    //         U256::MAX,
    //     ];
    //     for x in 0..64 {
    //         for y in test_values.iter() {
    //             compare_old_signextend(x.into(), *y);
    //         }
    //     }
    // }

    // fn compare_old_signextend(x: U256, y: U256) {
    //     let old = old_signextend(x, y);
    //     let new = signextend(x, y);

    //     assert_eq!(old, new);
    // }

    // fn old_signextend(op1: U256, op2: U256) -> U256 {
    //     if op1 > U256::from(32) {
    //         op2
    //     } else {
    //         let mut ret = u256_zero();
    //         let len: usize = op1.as_usize();
    //         let t: usize = 8 * (len + 1) - 1;
    //         let t_bit_mask = u256_one() << t;
    //         let t_value = (op2 & t_bit_mask) >> t;
    //         for i in 0..256 {
    //             let bit_mask = u256_one() << i;
    //             let i_value = (op2 & bit_mask) >> i;
    //             if i <= t {
    //                 ret = ret.overflowing_add(i_value << i).0;
    //             } else {
    //                 ret = ret.overflowing_add(t_value << i).0;
    //             }
    //         }
    //         ret
    //     }
    // }
}
