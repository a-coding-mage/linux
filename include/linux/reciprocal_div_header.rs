/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This algorithm is based on the paper "Division by Invariant
 * Integers Using Multiplication" by Torbjörn Granlund and Peter
 * L. Montgomery.
 *
 * The assembler implementation from Agner Fog, which this code is
 * based on, can be found here:
 * http://www.agner.org/optimize/asmlib.zip
 *
 * This optimization for A/B is helpful if the divisor B is mostly
 * runtime invariant. The reciprocal of B is calculated in the
 * slow-path with reciprocal_value(). The fast-path can then just use
 * a much faster multiplication operation with a variable dividend A
 * to calculate the division A/B.
 */

#[repr(C)]
pub struct reciprocal_value {
    pub m: u32,
    pub sh1: u8,
    pub sh2: u8,
}

/* "reciprocal_value" and "reciprocal_divide" together implement the basic
 * version of the algorithm described in Figure 4.1 of the paper.
 */
unsafe extern "C" {
    pub fn reciprocal_value(d: u32) -> reciprocal_value;
}

#[inline]
pub fn reciprocal_divide(a: u32, r: reciprocal_value) -> u32 {
    let t = ((a as u64 * r.m as u64) >> 32) as u32;
    t.wrapping_add(a.wrapping_sub(t) >> r.sh1) >> r.sh2
}

#[repr(C)]
pub struct reciprocal_value_adv {
    pub m: u32,
    pub sh: u8,
    pub exp: u8,
    pub is_wide_m: bool,
}

/* "reciprocal_value_adv" implements the advanced version of the algorithm
 * described in Figure 4.2 of the paper except when "divisor > (1U << 31)" whose
 * ceil(log2(d)) result will be 32 which then requires u128 divide on host. The
 * exception case could be easily handled before calling "reciprocal_value_adv".
 *
 * The advanced version requires more complex calculation to get the reciprocal
 * multiplier and other control variables, but then could reduce the required
 * emulation operations.
 *
 * It makes no sense to use this advanced version for host divide emulation,
 * those extra complexities for calculating multiplier etc could completely
 * waive our saving on emulation operations.
 *
 * However, it makes sense to use it for JIT divide code generation for which
 * we are willing to trade performance of JITed code with that of host. As shown
 * by the following pseudo code, the required emulation operations could go down
 * from 6 (the basic version) to 3 or 4.
 *
 * To use the result of "reciprocal_value_adv", suppose we want to calculate
 * n/d, the pseudo C code will be:
 *
 *   struct reciprocal_value_adv rvalue;
 *   u8 pre_shift, exp;
 *
 *   // handle exception case.
 *   if (d >= (1U << 31)) {
 *     result = n >= d;
 *     return;
 *   }
 *
 *   rvalue = reciprocal_value_adv(d, 32)
 *   exp = rvalue.exp;
 *   if (rvalue.is_wide_m && !(d & 1)) {
 *     // floor(log2(d & (2^32 -d)))
 *     pre_shift = fls(d & -d) - 1;
 *     rvalue = reciprocal_value_adv(d >> pre_shift, 32 - pre_shift);
 *   } else {
 *     pre_shift = 0;
 *   }
 *
 *   // code generation starts.
 *   if (imm == 1U << exp) {
 *     result = n >> exp;
 *   } else if (rvalue.is_wide_m) {
 *     // pre_shift must be zero when reached here.
 *     t = (n * rvalue.m) >> 32;
 *     result = n - t;
 *     result >>= 1;
 *     result += t;
 *     result >>= rvalue.sh - 1;
 *   } else {
 *     if (pre_shift)
 *       result = n >> pre_shift;
 *     result = ((u64)result * rvalue.m) >> 32;
 *     result >>= rvalue.sh;
 *   }
 */
unsafe extern "C" {
    pub fn reciprocal_value_adv(d: u32, prec: u8) -> reciprocal_value_adv;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
