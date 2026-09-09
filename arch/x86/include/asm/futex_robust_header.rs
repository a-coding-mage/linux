/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the architecture ptrace definitions.
use crate::asm::ptrace::{pt_regs, X86_EFLAGS_ZF};

/// If ZF is set then the cmpxchg succeeded and the pending op pointer
/// needs to be cleared.
#[inline(always)]
pub unsafe fn x86_futex_robust_unlock_get_pop(
    regs: *const pt_regs,
) -> *mut core::ffi::c_void {
    if (*regs).flags & X86_EFLAGS_ZF != 0 {
        (*regs).dx as *mut core::ffi::c_void
    } else {
        core::ptr::null_mut()
    }
}

#[macro_export]
macro_rules! arch_futex_robust_unlock_get_pop {
    ($regs:expr) => {
        $crate::x86_futex_robust_unlock_get_pop($regs)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
