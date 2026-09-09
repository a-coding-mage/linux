// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel and the soft-fp implementation
// are intentionally left external, as in the original source.

use core::ffi::c_void;

extern "C" {
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
}

pub unsafe fn lfs(fr_d: *mut c_void, ea: *mut c_void) -> i32 {
    // FP_DECL_D(R);
    FP_DECL_D!(R);
    // FP_DECL_S(A);
    FP_DECL_S!(A);
    // FP_DECL_EX;
    FP_DECL_EX!();
    let mut f: f32;

    // #ifdef DEBUG
    // printk("%s: D %p, ea %p\n", __func__, frD, ea);
    // #endif

    f = core::mem::MaybeUninit::<f32>::uninit().assume_init();
    if copy_from_user(
        (&mut f as *mut f32).cast::<c_void>(),
        ea.cast_const(),
        core::mem::size_of::<f32>(),
    ) != 0
    {
        return -(EFAULT as i32);
    }

    FP_UNPACK_S!(A, f);

    // #ifdef DEBUG
    // printk("A: %ld %lu %ld (%ld) [%08lx]\n", A_s, A_f, A_e, A_c,
    //        *(unsigned long *)&f);
    // #endif

    FP_CONV!(D, S, 2, 1, R, A);

    // #ifdef DEBUG
    // printk("R: %ld %lu %lu %ld (%ld)\n", R_s, R_f1, R_f0, R_e, R_c);
    // #endif

    if R_c == FP_CLS_NAN {
        R_e = _FP_EXPMAX_D;
        _FP_PACK_RAW_2_P!(D, fr_d, R);
    } else {
        __FP_PACK_D!(fr_d, R);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
