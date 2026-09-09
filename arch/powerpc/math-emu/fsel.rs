// SPDX-License-Identifier: GPL-2.0

// The declarations supplied by the Linux soft-fp headers are represented here
// by the corresponding primitive Rust types and operations.

const FP_CLS_ZERO: i32 = 0;
const FP_CLS_NAN: i32 = 5;

#[inline]
unsafe fn read_u32(p: *const u32) -> u32 {
    core::ptr::read(p)
}

pub unsafe fn fsel(
    fr_d: *mut u32,
    fr_a: *mut core::ffi::c_void,
    fr_b: *const u32,
    fr_c: *const u32,
) -> i32 {
    // FP_DECL_D(A), FP_DECL_EX, and FP_UNPACK_DP(A) from soft-fp/double.h.
    // The unpacked fields needed by this implementation are the class and sign.
    let a_words = fr_a as *const u32;
    let a_hi = read_u32(a_words);
    let a_lo = read_u32(a_words.add(1));
    let a_exp = (a_hi >> 20) & 0x7ff;
    let a_frac = (a_hi & 0x000f_ffff) as u64 | ((a_lo as u64) << 20);
    let a_s = (a_hi >> 31) as i32;
    let a_c = if a_exp == 0x7ff {
        if a_frac == 0 { 4 } else { FP_CLS_NAN }
    } else if a_exp == 0 && a_frac == 0 {
        FP_CLS_ZERO
    } else {
        1
    };

    if a_c == FP_CLS_NAN || (a_c != FP_CLS_ZERO && a_s != 0) {
        core::ptr::write(fr_d, read_u32(fr_b));
        core::ptr::write(fr_d.add(1), read_u32(fr_b.add(1)));
    } else {
        core::ptr::write(fr_d, read_u32(fr_c));
        core::ptr::write(fr_d.add(1), read_u32(fr_c.add(1)));
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
