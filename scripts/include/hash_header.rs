/* SPDX-License-Identifier: GPL-2.0-only */

pub const GOLDEN_RATIO_32: u32 = 0x61C88647;

pub unsafe fn hash_str(mut s: *const ::core::ffi::c_char) -> u32 {
    /* fnv32 hash */
    let mut hash: u32 = 2166136261u32;

    while *s != 0 {
        hash = (hash ^ (*s as u32)).wrapping_mul(0x01000193);
        s = s.add(1);
    }
    hash
}

/* simplified version of functions from include/linux/hash.h */
pub fn hash_32(val: u32) -> u32 {
    GOLDEN_RATIO_32.wrapping_mul(val)
}

pub fn hash_ptr(ptr: *const ::core::ffi::c_void) -> u32 {
    hash_32(ptr as usize as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
