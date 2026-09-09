// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux soft-fp and math-emu headers are kept as
// external Rust macros/items.

extern "C" {
    fn printk(fmt: *const u8, ...);
}

pub unsafe fn fdivs(frD: *mut core::ffi::c_void,
                    frA: *mut core::ffi::c_void,
                    frB: *mut core::ffi::c_void) -> core::ffi::c_int {
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_D!(R);
    FP_DECL_EX!();

    #[cfg(debug_assertions)]
    {
        printk(b"%s: %p %p %p\n\0".as_ptr(), b"fdivs\0".as_ptr(), frD, frA, frB);
    }

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);

    #[cfg(debug_assertions)]
    {
        printk(b"A: %ld %lu %lu %ld (%ld)\n\0".as_ptr(),
               FP_S!(A), FP_F1!(A), FP_F0!(A), FP_E!(A), FP_C!(A));
        printk(b"B: %ld %lu %lu %ld (%ld)\n\0".as_ptr(),
               FP_S!(B), FP_F1!(B), FP_F0!(B), FP_E!(B), FP_C!(B));
    }

    if FP_C!(A) == FP_CLS_ZERO && FP_C!(B) == FP_CLS_ZERO {
        FP_SET_EXCEPTION!(EFLAG_VXZDZ);
        #[cfg(debug_assertions)]
        {
            printk(b"%s: FPSCR_VXZDZ raised\n\0".as_ptr(), b"fdivs\0".as_ptr());
        }
    }
    if FP_C!(A) == FP_CLS_INF && FP_C!(B) == FP_CLS_INF {
        FP_SET_EXCEPTION!(EFLAG_VXIDI);
        #[cfg(debug_assertions)]
        {
            printk(b"%s: FPSCR_VXIDI raised\n\0".as_ptr(), b"fdivs\0".as_ptr());
        }
    }

    if FP_C!(B) == FP_CLS_ZERO && FP_C!(A) != FP_CLS_ZERO {
        FP_SET_EXCEPTION!(EFLAG_DIVZERO);
        if __FPU_TRAP_P!(EFLAG_DIVZERO) {
            return FP_CUR_EXCEPTIONS!();
        }
    }

    FP_DIV_D!(R, A, B);

    #[cfg(debug_assertions)]
    {
        printk(b"D: %ld %lu %lu %ld (%ld)\n\0".as_ptr(),
               FP_S!(R), FP_F1!(R), FP_F0!(R), FP_E!(R), FP_C!(R));
    }

    FP_PACK_DS!(frD, R);

    FP_CUR_EXCEPTIONS!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
