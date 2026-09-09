/* SPDX-License-Identifier: GPL-2.0 */

pub unsafe fn __iter_div_u64_rem(
    mut dividend: u64,
    divisor: u32,
    remainder: *mut u64,
) -> u32 {
    let mut ret: u32 = 0;

    while dividend >= divisor as u64 {
        /* The following compiler barrier prevents the compiler from
           optimising this loop into a modulo operation. */
        core::hint::black_box(&mut dividend);

        dividend = dividend.wrapping_sub(divisor as u64);
        ret = ret.wrapping_add(1);
    }

    *remainder = dividend;

    ret
}

/* The C condition was:
 * defined(CONFIG_ARCH_SUPPORTS_INT128) && defined(__SIZEOF_INT128__).
 * The feature names below preserve that build-time intent for Rust builds.
 */
#[cfg(all(feature = "arch_supports_int128", feature = "sizeof_int128"))]
pub fn mul_u64_u32_add_u64_shr(a: u64, mul: u32, b: u64, shift: u32) -> u64 {
    ((((a as u128).wrapping_mul(mul as u128)).wrapping_add(b as u128)) >> shift) as u64
}

#[cfg(not(all(feature = "arch_supports_int128", feature = "sizeof_int128")))]
pub fn mul_u32_u32(a: u32, b: u32) -> u64 {
    (a as u64).wrapping_mul(b as u64)
}

#[cfg(not(all(feature = "arch_supports_int128", feature = "sizeof_int128")))]
pub fn mul_u64_u32_add_u64_shr(a: u64, mul: u32, b: u64, shift: u32) -> u64 {
    let ah: u32 = (a >> 32) as u32;
    let al: u32 = a as u32;
    let (mut ret, ovf) = mul_u32_u32(al, mul).overflowing_add(b);

    ret >>= shift;
    if ovf && shift != 0 {
        ret = ret.wrapping_add(1u64 << (64 - shift));
    }
    if ah != 0 {
        ret = ret.wrapping_add(mul_u32_u32(ah, mul) << (32 - shift));
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
