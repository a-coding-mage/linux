/* SPDX-License-Identifier: GPL-2.0-only */

// Conditional on CONFIG_COMPAT being enabled (the C header uses IS_ENABLED).
#[cfg(feature = "CONFIG_COMPAT")]
unsafe extern "C" {
    pub fn compat_setup_rt_frame(
        ksig: *mut ksignal,
        set: *mut sigset_t,
        regs: *mut pt_regs,
    ) -> i32;
}

// Fallback when CONFIG_COMPAT is disabled.
#[cfg(not(feature = "CONFIG_COMPAT"))]
#[inline]
pub unsafe fn compat_setup_rt_frame(
    _ksig: *mut ksignal,
    _set: *mut sigset_t,
    _regs: *mut pt_regs,
) -> i32 {
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
