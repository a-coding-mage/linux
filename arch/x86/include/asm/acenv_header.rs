/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * X86 specific ACPICA environments and implementation
 *
 * Copyright (C) 2014, Intel Corporation
 *   Author: Lv Zheng <lv.zheng@intel.com>
 */

// C header guard: _ASM_X86_ACENV_H
// Dependency: <asm/special_insns.h>

/* Asm macros */

/*
 * ACPI_FLUSH_CPU_CACHE() flushes caches on entering sleep states.
 * It is required to prevent data loss.
 *
 * While running inside virtual machine, the kernel can bypass cache flushing.
 * Changing sleep state in a virtual machine doesn't affect the host system
 * sleep state and cannot lead to data loss.
 */
#[macro_export]
macro_rules! ACPI_FLUSH_CPU_CACHE {
    () => {{
        if !cpu_feature_enabled(X86_FEATURE_HYPERVISOR) {
            wbinvd();
        }
    }};
}

extern "C" {
    pub fn __acpi_acquire_global_lock(lock: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn __acpi_release_global_lock(lock: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[macro_export]
macro_rules! ACPI_ACQUIRE_GLOBAL_LOCK {
    ($facs:expr, $acq:expr) => {
        $acq = unsafe { __acpi_acquire_global_lock(&mut (*$facs).global_lock) };
    };
}

#[macro_export]
macro_rules! ACPI_RELEASE_GLOBAL_LOCK {
    ($facs:expr, $acq:expr) => {
        $acq = unsafe { __acpi_release_global_lock(&mut (*$facs).global_lock) };
    };
}

/*
 * Math helper asm macros
 */
#[macro_export]
macro_rules! ACPI_DIV_64_BY_32 {
    ($n_hi:expr, $n_lo:expr, $d32:expr, $q32:expr, $r32:expr) => {{
        let mut quotient: u32;
        let mut remainder: u32;
        unsafe {
            ::core::arch::asm!(
                "divl {divisor:e}",
                divisor = in(reg) ($d32),
                inlateout("eax") ($n_lo) => quotient,
                inlateout("edx") ($n_hi) => remainder,
                options(nostack)
            );
        }
        $q32 = quotient;
        $r32 = remainder;
    }};
}

#[macro_export]
macro_rules! ACPI_SHIFT_RIGHT_64 {
    ($n_hi:expr, $n_lo:expr) => {{
        unsafe {
            ::core::arch::asm!(
                "shrl $1, {hi:e}",
                "rcrl $1, {lo:e}",
                hi = inout(reg) ($n_hi),
                lo = inout(reg) ($n_lo),
                options(nostack)
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
