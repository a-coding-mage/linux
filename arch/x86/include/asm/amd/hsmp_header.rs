/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency corresponding to <uapi/asm/amd_hsmp.h>.

use core::ffi::c_int;

// Declaration supplied by the UAPI dependency.
pub struct hsmp_message;

// Equivalent to IS_ENABLED(CONFIG_AMD_HSMP).
#[cfg(feature = "CONFIG_AMD_HSMP")]
unsafe extern "C" {
    pub fn hsmp_send_message(msg: *mut hsmp_message) -> c_int;
}

#[cfg(not(feature = "CONFIG_AMD_HSMP"))]
#[inline]
pub unsafe fn hsmp_send_message(_msg: *mut hsmp_message) -> c_int {
    // -ENODEV
    -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
