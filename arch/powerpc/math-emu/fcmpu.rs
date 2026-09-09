// SPDX-License-Identifier: GPL-2.0
//
// The declarations and operations below correspond to the Linux soft-fp
// headers included by the original C source.

use core::ffi::c_void;

extern "C" {
    static mut __FPU_FPSCR: u32;

    // These declarations represent the soft-fp operations supplied by the
    // included headers in the original translation unit.
    fn FP_UNPACK_DP(value: *mut FpDouble, fr: *mut c_void);
    fn FP_CMP_D(result: *mut i64, a: *mut FpDouble, b: *mut FpDouble, signaling: i32);
}

#[repr(C)]
pub struct FpDouble {
    _opaque: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn fcmpu(
    ccr: *mut u32,
    crfD: i32,
    frA: *mut c_void,
    frB: *mut c_void,
) -> i32 {
    // FP_DECL_D(A);
    let mut a = FpDouble { _opaque: [] };
    // FP_DECL_D(B);
    let mut b = FpDouble { _opaque: [] };
    // FP_DECL_EX;
    let code: [i32; 4] = [1 << 3, 1 << 1, 1 << 2, 1 << 0];
    let mut cmp: i64;

    // #ifdef DEBUG
    // printk("%s: %p (%08x) %d %p %p\\n", __func__, ccr, *ccr, crfD, frA, frB);
    // #endif

    FP_UNPACK_DP(&mut a, frA);
    FP_UNPACK_DP(&mut b, frB);

    // #ifdef DEBUG
    // printk("A: %ld %lu %lu %ld (%ld)\\n", A_s, A_f1, A_f0, A_e, A_c);
    // printk("B: %ld %lu %lu %ld (%ld)\\n", B_s, B_f1, B_f0, B_e, B_c);
    // #endif

    cmp = 0;
    FP_CMP_D(&mut cmp, &mut a, &mut b, 2);
    cmp = code[((cmp + 1) & 3) as usize] as i64;

    __FPU_FPSCR &= !0x1f000;
    __FPU_FPSCR |= (cmp as u32) << 12;

    let shift = ((7 - crfD) << 2) as u32;
    *ccr &= !(15u32 << shift);
    *ccr |= (cmp as u32) << shift;

    // #ifdef DEBUG
    // printk("CR: %08x\\n", *ccr);
    // #endif

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
