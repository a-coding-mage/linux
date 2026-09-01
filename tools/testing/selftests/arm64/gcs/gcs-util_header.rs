/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 ARM Limited.
 */

/* C header guard and #include <stdbool.h> omitted in Rust translation. */

/* Fallback syscall/note constants from the C preprocessor guards. */
pub const __NR_map_shadow_stack: u64 = 453;
pub const __NR_prctl: u64 = 167;
pub const NT_ARM_GCS: u64 = 0x410;

/* Shadow Stack/Guarded Control Stack interface */
pub const PR_GET_SHADOW_STACK_STATUS: u64 = 74;
pub const PR_SET_SHADOW_STACK_STATUS: u64 = 75;
pub const PR_LOCK_SHADOW_STACK_STATUS: u64 = 76;

pub const PR_SHADOW_STACK_ENABLE: u64 = 1u64 << 0;
pub const PR_SHADOW_STACK_WRITE: u64 = 1u64 << 1;
pub const PR_SHADOW_STACK_PUSH: u64 = 1u64 << 2;

pub const PR_SHADOW_STACK_ALL_MODES: u64 =
    PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE | PR_SHADOW_STACK_PUSH;

pub const SHADOW_STACK_SET_TOKEN: u64 = 1u64 << 0; /* Set up a restore token in the shadow stack */
pub const SHADOW_STACK_SET_MARKER: u64 = 1u64 << 1; /* Set up a top of stack merker in the shadow stack */

pub const GCS_CAP_ADDR_MASK: u64 = 0xfffffffffffff000u64;
pub const GCS_CAP_TOKEN_MASK: u64 = 0x0000000000000fffu64;
pub const GCS_CAP_VALID_TOKEN: u64 = 1;
pub const GCS_CAP_IN_PROGRESS_TOKEN: u64 = 5;

#[inline]
pub const fn GCS_CAP(x: u64) -> u64 {
    (x & GCS_CAP_ADDR_MASK) | GCS_CAP_VALID_TOKEN
}

#[inline]
pub unsafe fn get_gcspr() -> *mut u64 {
    let gcspr: *mut u64;

    unsafe {
        core::arch::asm!(
            "mrs {0}, S3_3_C2_C5_1",
            out(reg) gcspr,
            options(nostack, preserves_flags)
        );
    }

    gcspr
}

#[inline(always)]
pub unsafe fn gcsss1(Xt: *mut u64) {
    unsafe {
        core::arch::asm!(
            "sys #3, C7, C7, #2, {0}",
            in(reg) Xt,
            options(nostack)
        );
    }
}

#[inline(always)]
pub unsafe fn gcsss2() -> *mut u64 {
    let Xt: *mut u64;

    unsafe {
        core::arch::asm!(
            "SYSL {0}, #3, C7, C7, #3",
            out(reg) Xt,
            options(nostack)
        );
    }

    Xt
}

#[inline]
pub unsafe fn chkfeat_gcs() -> bool {
    let mut val: i64 = 1;

    /* CHKFEAT x16 */
    unsafe {
        core::arch::asm!(
            "hint #0x28",
            inout("x16") val,
            options(nostack)
        );
    }

    val != 1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
