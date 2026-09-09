/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the x86 kernel FPU legacy interface.  The C inline-assembly
// exception-table annotations are retained in comments because their symbols
// are supplied by the surrounding kernel assembly environment.

use core::arch::asm;

extern "C" {
    pub static mut mxcsr_feature_mask: u32;
}

#[inline]
pub unsafe fn ldmxcsr(mxcsr: u32) {
    asm!("ldmxcsr [{0}]", in(reg) &mxcsr, options(readonly, nostack, preserves_flags));
}

// Returns 0 on success or the trap number when the operation raises an exception.
// user_insn and kernel_insn are represented by the corresponding inline
// assembly at each call site; might_fault(), STAC/CLAC, and exception-table
// directives are kernel-provided operations.

#[inline]
pub unsafe fn fnsave_to_user_sigframe(fx: *mut crate::fregs_state) -> i32 {
    let mut err: i32;
    asm!(
        "stac\n1: fnsave [{fx}]\n2: clac",
        fx = in(reg) fx,
        lateout("eax") err,
        options(nostack)
    );
    err
}

#[inline]
pub unsafe fn fxsave_to_user_sigframe(fx: *mut crate::fxregs_state) -> i32 {
    let mut err: i32;
    if cfg!(target_pointer_width = "32") {
        asm!("stac\n1: fxsave [{fx}]\n2: clac", fx = in(reg) fx, lateout("eax") err, options(nostack));
    } else {
        asm!("stac\n1: fxsaveq [{fx}]\n2: clac", fx = in(reg) fx, lateout("eax") err, options(nostack));
    }
    err
}

#[inline]
pub unsafe fn fxrstor(fx: *mut crate::fxregs_state) {
    if cfg!(target_pointer_width = "32") {
        asm!("fxrstor [{fx}]", fx = in(reg) fx, options(nostack));
    } else {
        asm!("fxrstorq [{fx}]", fx = in(reg) fx, options(nostack));
    }
}

#[inline]
pub unsafe fn fxrstor_safe(fx: *mut crate::fxregs_state) -> i32 {
    let mut err: i32;
    if cfg!(target_pointer_width = "32") {
        asm!("fxrstor [{fx}]", fx = in(reg) fx, lateout("eax") err, options(nostack));
    } else {
        asm!("fxrstorq [{fx}]", fx = in(reg) fx, lateout("eax") err, options(nostack));
    }
    err
}

#[inline]
pub unsafe fn fxrstor_from_user_sigframe(fx: *mut crate::fxregs_state) -> i32 {
    let mut err: i32;
    if cfg!(target_pointer_width = "32") {
        asm!("stac\n1: fxrstor [{fx}]\n2: clac", fx = in(reg) fx, lateout("eax") err, options(nostack));
    } else {
        asm!("stac\n1: fxrstorq [{fx}]\n2: clac", fx = in(reg) fx, lateout("eax") err, options(nostack));
    }
    err
}

#[inline]
pub unsafe fn frstor(fx: *mut crate::fregs_state) {
    asm!("frstor [{fx}]", fx = in(reg) fx, options(nostack));
}

#[inline]
pub unsafe fn frstor_safe(fx: *mut crate::fregs_state) -> i32 {
    let mut err: i32;
    asm!("frstor [{fx}]", fx = in(reg) fx, lateout("eax") err, options(nostack));
    err
}

#[inline]
pub unsafe fn frstor_from_user_sigframe(fx: *mut crate::fregs_state) -> i32 {
    let mut err: i32;
    asm!("stac\n1: frstor [{fx}]\n2: clac", fx = in(reg) fx, lateout("eax") err, options(nostack));
    err
}

#[inline]
pub unsafe fn fxsave(fx: *mut crate::fxregs_state) {
    if cfg!(target_pointer_width = "32") {
        asm!("fxsave [{fx}]", fx = in(reg) fx, options(nostack));
    } else {
        asm!("fxsaveq [{fx}]", fx = in(reg) fx, options(nostack));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
