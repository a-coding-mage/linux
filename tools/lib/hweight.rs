// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bitops.h>, <asm/types.h>

use std::os::raw::c_ulong;

pub type __u64 = u64;

/**
 * hweightN - returns the hamming weight of a N-bit word
 * @x: the word to weigh
 *
 * The Hamming Weight of a number is the total number of bits set in it.
 */

#[no_mangle]
pub extern "C" fn __sw_hweight32(mut w: u32) -> u32 {
    // Original C condition: CONFIG_ARCH_HAS_FAST_MULTIPLIER.
    #[cfg(feature = "CONFIG_ARCH_HAS_FAST_MULTIPLIER")]
    {
        w = w.wrapping_sub((w >> 1) & 0x55555555);
        w = (w & 0x33333333).wrapping_add((w >> 2) & 0x33333333);
        w = w.wrapping_add(w >> 4) & 0x0f0f0f0f;
        return w.wrapping_mul(0x01010101) >> 24;
    }

    // Original C fallback when CONFIG_ARCH_HAS_FAST_MULTIPLIER is not set.
    #[cfg(not(feature = "CONFIG_ARCH_HAS_FAST_MULTIPLIER"))]
    {
        let mut res: u32 = w.wrapping_sub((w >> 1) & 0x55555555);
        res = (res & 0x33333333).wrapping_add((res >> 2) & 0x33333333);
        res = res.wrapping_add(res >> 4) & 0x0F0F0F0F;
        res = res.wrapping_add(res >> 8);
        return res.wrapping_add(res >> 16) & 0x000000FF;
    }
}

#[no_mangle]
pub extern "C" fn __sw_hweight16(w: u32) -> u32 {
    let mut res: u32 = w.wrapping_sub((w >> 1) & 0x5555);
    res = (res & 0x3333).wrapping_add((res >> 2) & 0x3333);
    res = res.wrapping_add(res >> 4) & 0x0F0F;
    res.wrapping_add(res >> 8) & 0x00FF
}

#[no_mangle]
pub extern "C" fn __sw_hweight8(w: u32) -> u32 {
    let mut res: u32 = w.wrapping_sub((w >> 1) & 0x55);
    res = (res & 0x33).wrapping_add((res >> 2) & 0x33);
    res.wrapping_add(res >> 4) & 0x0F
}

#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn __sw_hweight64(w: __u64) -> c_ulong {
    (__sw_hweight32((w >> 32) as u32).wrapping_add(__sw_hweight32(w as u32))) as c_ulong
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn __sw_hweight64(mut w: __u64) -> c_ulong {
    // Original C condition: CONFIG_ARCH_HAS_FAST_MULTIPLIER.
    #[cfg(feature = "CONFIG_ARCH_HAS_FAST_MULTIPLIER")]
    {
        w = w.wrapping_sub((w >> 1) & 0x5555555555555555);
        w = (w & 0x3333333333333333).wrapping_add((w >> 2) & 0x3333333333333333);
        w = w.wrapping_add(w >> 4) & 0x0f0f0f0f0f0f0f0f;
        return (w.wrapping_mul(0x0101010101010101) >> 56) as c_ulong;
    }

    // Original C fallback when CONFIG_ARCH_HAS_FAST_MULTIPLIER is not set.
    #[cfg(not(feature = "CONFIG_ARCH_HAS_FAST_MULTIPLIER"))]
    {
        let mut res: __u64 = w.wrapping_sub((w >> 1) & 0x5555555555555555);
        res = (res & 0x3333333333333333).wrapping_add((res >> 2) & 0x3333333333333333);
        res = res.wrapping_add(res >> 4) & 0x0F0F0F0F0F0F0F0F;
        res = res.wrapping_add(res >> 8);
        res = res.wrapping_add(res >> 16);
        return (res.wrapping_add(res >> 32) & 0x00000000000000FF) as c_ulong;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
