/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2015, Michael Ellerman, IBM Corp.
 */

// C header dependencies: <stdbool.h>, <asm/tm.h>, "utils.h", "reg.h".

use core::arch::asm;

pub const TM_RETRIES: i32 = 100;

unsafe extern "C" {
    pub fn printf(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub fn have_hwcap2(feature: u64) -> bool;
}

unsafe extern "C" {
    pub fn __builtin_get_texasr() -> u64;
    pub fn __builtin_get_texasru() -> u64;
}

unsafe extern "C" {
    pub static PPC_FEATURE2_HTM: u64;
    pub static PPC_FEATURE2_HTM_NOSC: u64;
    pub static TEXASR_FP: u64;
    pub static TEXASR_IC: u64;
    pub static TM_CAUSE_PERSISTENT: i64;
    pub static TM_CAUSE_SYSCALL: i64;
    pub static TM_CAUSE_FAC_UNAV: i64;
    pub static TM_CAUSE_RESCHED: i64;
    pub static TM_CAUSE_KVM_RESCHED: i64;
    pub static TM_CAUSE_KVM_FAC_UNAV: i64;
}

#[inline]
pub unsafe fn have_htm() -> bool {
    /*
     * Original C conditional:
     * #ifdef PPC_FEATURE2_HTM
     *     return have_hwcap2(PPC_FEATURE2_HTM);
     * #else
     *     printf("PPC_FEATURE2_HTM not defined, can't check AT_HWCAP2\n");
     *     return false;
     * #endif
     */
    unsafe { have_hwcap2(PPC_FEATURE2_HTM) }
}

#[inline]
pub unsafe fn have_htm_nosc() -> bool {
    /*
     * Original C conditional:
     * #ifdef PPC_FEATURE2_HTM_NOSC
     *     return have_hwcap2(PPC_FEATURE2_HTM_NOSC);
     * #else
     *     printf("PPC_FEATURE2_HTM_NOSC not defined, can't check AT_HWCAP2\n");
     *     return false;
     * #endif
     */
    unsafe { have_hwcap2(PPC_FEATURE2_HTM_NOSC) }
}

/*
 * Transactional Memory was removed in ISA 3.1. A synthetic TM implementation
 * is provided on P10 for threads running in P8/P9 compatibility  mode. The
 * synthetic implementation immediately fails after tbegin. This failure sets
 * Bit 7 (Failure Persistent) and Bit 15 (Implementation-specific).
 */
#[inline]
pub unsafe fn htm_is_synthetic() -> bool {
    let mut i: i32;

    /*
     * Per the ISA, the Failure Persistent bit may be incorrect. Try a few
     * times in case we got an Implementation-specific failure on a non ISA
     * v3.1 system. On these systems the Implementation-specific failure
     * should not be persistent.
     */
    i = 0;
    while i < TM_RETRIES {
        unsafe {
            asm!(
                "tbegin.",
                "beq 1f",
                "tend.",
                "1:",
                options(nostack, preserves_flags),
            );
        }

        if unsafe { __builtin_get_texasr() & (TEXASR_FP | TEXASR_IC) }
            != unsafe { TEXASR_FP | TEXASR_IC }
        {
            break;
        }

        i += 1;
    }

    i == TM_RETRIES
}

#[inline]
pub unsafe fn failure_code() -> i64 {
    (unsafe { __builtin_get_texasru() } >> 24) as i64
}

#[inline]
pub unsafe fn failure_is_persistent() -> bool {
    unsafe { (failure_code() & TM_CAUSE_PERSISTENT) == TM_CAUSE_PERSISTENT }
}

#[inline]
pub unsafe fn failure_is_syscall() -> bool {
    unsafe { (failure_code() & TM_CAUSE_SYSCALL) == TM_CAUSE_SYSCALL }
}

#[inline]
pub unsafe fn failure_is_unavailable() -> bool {
    unsafe { (failure_code() & TM_CAUSE_FAC_UNAV) == TM_CAUSE_FAC_UNAV }
}

#[inline]
pub unsafe fn failure_is_reschedule() -> bool {
    if unsafe {
        (failure_code() & TM_CAUSE_RESCHED) == TM_CAUSE_RESCHED
            || (failure_code() & TM_CAUSE_KVM_RESCHED) == TM_CAUSE_KVM_RESCHED
            || (failure_code() & TM_CAUSE_KVM_FAC_UNAV) == TM_CAUSE_KVM_FAC_UNAV
    } {
        return true;
    }

    false
}

#[inline]
pub unsafe fn failure_is_nesting() -> bool {
    (unsafe { __builtin_get_texasru() } & 0x400000) != 0
}

#[inline]
pub unsafe fn tcheck() -> i32 {
    let cr: i64;

    unsafe {
        asm!(
            "tcheck 0",
            out(reg) cr,
            out("cr0") _,
            options(nostack),
        );
    }

    ((cr >> 28) & 4) as i32
}

#[inline]
pub unsafe fn tcheck_doomed() -> bool {
    unsafe { (tcheck() & 8) != 0 }
}

#[inline]
pub unsafe fn tcheck_active() -> bool {
    unsafe { (tcheck() & 4) != 0 }
}

#[inline]
pub unsafe fn tcheck_suspended() -> bool {
    unsafe { (tcheck() & 2) != 0 }
}

#[inline]
pub unsafe fn tcheck_transactional() -> bool {
    unsafe { (tcheck() & 6) != 0 }
}
