// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux soft-fp and math-emu headers are kept as
// external declarations below; the original C implementation uses these
// operations as macros.

use core::ffi::c_void;

#[repr(C)]
pub struct FpDouble {
    _private: [u8; 0],
}

extern "C" {
    fn fp_decl_ex() -> i32;
    fn fp_unpack_dp(value: *mut FpDouble, source: *mut c_void);
    fn fp_add_d(result: *mut FpDouble, left: *mut FpDouble, right: *mut FpDouble);
    fn fp_pack_d(destination: *mut c_void, value: *mut FpDouble);
    static FP_CUR_EXCEPTIONS: i32;
}

#[no_mangle]
pub unsafe extern "C" fn fadd(
    fr_d: *mut c_void,
    fr_a: *mut c_void,
    fr_b: *mut c_void,
) -> i32 {
    let mut a = core::mem::MaybeUninit::<FpDouble>::uninit();
    let mut b = core::mem::MaybeUninit::<FpDouble>::uninit();
    let mut r = core::mem::MaybeUninit::<FpDouble>::uninit();

    // FP_DECL_EX;
    let _ = fp_decl_ex();

    // FP_UNPACK_DP(A, frA);
    fp_unpack_dp(a.as_mut_ptr(), fr_a);
    // FP_UNPACK_DP(B, frB);
    fp_unpack_dp(b.as_mut_ptr(), fr_b);

    // FP_ADD_D(R, A, B);
    fp_add_d(r.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());

    // __FP_PACK_D(frD, R);
    fp_pack_d(fr_d, r.as_mut_ptr());

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
