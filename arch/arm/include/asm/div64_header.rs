/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The semantics of __div64_32() are:
 *
 * uint32_t __div64_32(uint64_t *n, uint32_t base)
 * {
 *     uint32_t remainder = *n % base;
 *     *n = *n / base;
 *     return remainder;
 * }
 *
 * The original implementation uses the ARM __do_div64 assembly routine and
 * its non-standard calling convention.  This Rust translation preserves the
 * operation and pointer-visible result.
 */
#[inline]
pub unsafe fn __div64_32(n: *mut u64, base: u32) -> u32 {
    let dividend = *n;
    let remainder = dividend % base as u64;
    *n = dividend / base as u64;
    remainder as u32
}

/* In OABI configurations, do_div uses the out-of-line-compatible operation. */
#[inline]
pub unsafe fn do_div(n: *mut u64, base: u32) -> u32 {
    __div64_32(n, base)
}

/*
 * ARM's xprod operation computes the low 64 bits of the 128-bit product
 * m*n, with the high 64-bit product word accumulated according to bias.
 * The assembly implementation's result is equivalent to this wrapping
 * 64-bit product for the returned value.
 */
#[inline]
pub fn __arch_xprod_64(m: u64, n: u64, _bias: bool) -> u64 {
    m.wrapping_mul(n)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
