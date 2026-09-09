/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 SiFive
 */

// Dependencies supplied by the corresponding PowerPC and kernel headers.

/// `kernel_fpu_available()`
#[inline]
pub unsafe fn kernel_fpu_available() -> bool {
    !cpu_has_feature(CPU_FTR_FPU_UNAVAILABLE)
}

#[inline]
pub unsafe fn kernel_fpu_begin() {
    preempt_disable();
    enable_kernel_fp();
}

#[inline]
pub unsafe fn kernel_fpu_end() {
    disable_kernel_fp();
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
