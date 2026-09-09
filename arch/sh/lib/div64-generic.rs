// SPDX-License-Identifier: GPL-2.0
/*
 * Generic __div64_32 wrapper for __xdiv64_32.
 */

unsafe extern "C" {
    fn __xdiv64_32(n: u64, d: u32) -> u64;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __div64_32(xp: *mut u64, y: u32) -> u32 {
    let q: u64 = __xdiv64_32(*xp, y);

    let rem: u32 = (*xp).wrapping_sub(q.wrapping_mul(y as u64)) as u32;
    *xp = q;

    rem
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
