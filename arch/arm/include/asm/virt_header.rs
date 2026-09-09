/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2012 Linaro Limited.
 */

/* Translated from virt.h. */

/*
 * Flag indicating that the kernel was not entered in the same mode on every
 * CPU.  The zImage loader stashes this value in an SPSR, so we need an
 * architecturally defined flag bit here.
 */
pub const BOOT_CPU_MODE_MISMATCH: _ = PSR_N_BIT;

/* CONFIG_ARM_VIRT_EXT controls which boot-mode representation is used. */
#[cfg(CONFIG_ARM_VIRT_EXT)]
unsafe extern "C" {
    pub static mut __boot_cpu_mode: core::ffi::c_int;
}

#[cfg(CONFIG_ARM_VIRT_EXT)]
#[inline]
pub unsafe fn sync_boot_mode() {
    /*
     * As secondaries write to __boot_cpu_mode with caches disabled, we
     * must flush the corresponding cache entries to ensure the visibility
     * of their writes.
     */
    sync_cache_r(core::ptr::addr_of!(__boot_cpu_mode));
}

#[cfg(not(CONFIG_ARM_VIRT_EXT))]
pub const __boot_cpu_mode: _ = SVC_MODE;

#[cfg(not(CONFIG_ARM_VIRT_EXT))]
#[inline]
pub fn sync_boot_mode() {}

/* ZIMAGE excludes the following kernel-side declarations. */
#[cfg(not(ZIMAGE))]
unsafe extern "C" {
    pub fn hyp_mode_check();
}

#[cfg(not(ZIMAGE))]
#[inline]
pub fn is_hyp_mode_available() -> bool {
    unsafe {
        ((__boot_cpu_mode & MODE_MASK) == HYP_MODE)
            && ((__boot_cpu_mode & BOOT_CPU_MODE_MISMATCH) == 0)
    }
}

#[cfg(not(ZIMAGE))]
#[inline]
pub fn is_hyp_mode_mismatched() -> bool {
    unsafe { (__boot_cpu_mode & BOOT_CPU_MODE_MISMATCH) != 0 }
}

#[cfg(not(ZIMAGE))]
#[inline]
pub fn is_kernel_in_hyp_mode() -> bool {
    false
}

/* Only assembly code should need those. */
pub const HVC_SET_VECTORS: i32 = 0;
pub const HVC_SOFT_RESTART: i32 = 1;

pub const HVC_STUB_ERR: i32 = 0xbadca11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
