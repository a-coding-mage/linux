/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency: <linux/types.h>
// Rust primitive integer types are used for u64/u32 equivalents.

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn mul_u64_u64_div64(a: u64, b: u64, c: u64) -> u64 {
    let q: u64;

    core::arch::asm!(
        "mulq {b}; divq {c}",
        b = in(reg) b,
        c = in(reg) c,
        inlateout("rax") a => q,
        lateout("rdx") _,
    );

    q
}

// C conditional: #ifdef __SIZEOF_INT128__
// Rust has a native u128 type on supported targets.
#[inline]
pub fn mul_u64_u32_shr(a: u64, b: u32, shift: u32) -> u64 {
    (((a as u128) * (b as u128)) >> shift) as u64
}

// C fallback for targets without __SIZEOF_INT128__:
//
// #ifdef __i386__
// static inline u64 mul_u32_u32(u32 a, u32 b)
// {
//     u32 high, low;
//
//     asm ("mull %[b]" : "=a" (low), "=d" (high)
//                      : [a] "a" (a), [b] "rm" (b) );
//
//     return low | ((u64)high) << 32;
// }
// #else
// static inline u64 mul_u32_u32(u32 a, u32 b)
// {
//     return (u64)a * b;
// }
// #endif
//
// static inline u64 mul_u64_u32_shr(u64 a, u32 b, unsigned int shift)
// {
//     u32 ah, al;
//     u64 ret;
//
//     al = a;
//     ah = a >> 32;
//
//     ret = mul_u32_u32(al, b) >> shift;
//     if (ah)
//         ret += mul_u32_u32(ah, b) << (32 - shift);
//
//     return ret;
// }

#[cfg(not(target_arch = "x86_64"))]
#[inline]
pub fn mul_u64_u64_div64(a: u64, b: u64, c: u64) -> u64 {
    let quot: u64;
    let rem: u64;

    quot = a / c;
    rem = a % c;

    quot.wrapping_mul(b).wrapping_add(rem.wrapping_mul(b) / c)
}

#[inline]
pub fn div_u64(dividend: u64, divisor: u32) -> u64 {
    dividend / (divisor as u64)
}
