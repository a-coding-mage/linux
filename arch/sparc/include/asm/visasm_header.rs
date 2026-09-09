/* SPDX-License-Identifier: GPL-2.0 */
/*
 * visasm.h: FPU saving macros for VIS routines
 *
 * Copyright (C) 1998 Jakub Jelinek (jj@ultra.linux.cz)
 *
 * C header dependencies: asm/pstate.h and asm/ptrace.h.
 * The following macros contain SPARC assembly and are preserved as assembly
 * source fragments for use by the surrounding low-level implementation.
 */

/* Clobbers %o5, %g1, %g2, %g3, %g7, %icc, %xcc */
#[macro_export]
macro_rules! VISEntry {
    () => {
        "rd %fprs, %o5; andcc %o5, (FPRS_FEF|FPRS_DU), %g0; be,pt %icc, 297f; sethi %hi(297f), %g7; sethi %hi(VISenter), %g1; jmpl %g1 + %lo(VISenter), %g0; or %g7, %lo(297f), %g7; 297: wr %g0, FPRS_FEF, %fprs;"
    };
}

#[macro_export]
macro_rules! VISExit {
    () => {
        "wr %g0, 0, %fprs;"
    };
}

/* Clobbers %o5, %g1, %g2, %g3, %g7, %icc, %xcc.
 * Must preserve %o5 between VISEntryHalf and VISExitHalf.
 */
#[macro_export]
macro_rules! VISEntryHalf {
    () => { $crate::VISEntry!() };
}

#[macro_export]
macro_rules! VISExitHalf {
    () => { $crate::VISExit!() };
}

#[macro_export]
macro_rules! VISEntryHalfFast {
    ($fail_label:expr) => {
        concat!(
            "rd %fprs, %o5; andcc %o5, FPRS_FEF, %g0; be,pt %icc, 297f; nop; ",
            "ba,a,pt %xcc, ", stringify!($fail_label),
            "; 297: wr %o5, FPRS_FEF, %fprs;"
        )
    };
}

#[macro_export]
macro_rules! VISExitHalfFast {
    () => {
        "wr %o5, 0, %fprs;"
    };
}

/* Equivalent of the C inline assembly routine. */
pub unsafe fn save_and_clear_fpu() {
    // The implementation is supplied by the SPARC low-level assembly layer.
    core::arch::asm!(
        "rd %fprs, %o5",
        "andcc %o5, {fprs}, %g0",
        "be,pt %icc, 299f",
        " sethi %hi(298f), %g7",
        "sethi %hi(VISenter), %g1",
        "jmpl %g1 + %lo(VISenter), %g0",
        " or %g7, %lo(298f), %g7",
        "298: wr %g0, 0, %fprs",
        "299:",
        fprs = const FPRS_FEF | FPRS_DU,
        options(nostack)
    );
}

unsafe extern "C" {
    pub fn vis_emul(regs: *mut pt_regs, instruction: core::ffi::c_uint) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
