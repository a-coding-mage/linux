/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <linux/types.h>

// Corresponds to CONFIG_SYSCALL_USER_DISPATCH.
#[cfg(feature = "CONFIG_SYSCALL_USER_DISPATCH")]
#[repr(C)]
pub struct syscall_user_dispatch {
    pub selector: *mut core::ffi::c_char,
    pub offset: core::ffi::c_ulong,
    pub len: core::ffi::c_ulong,
    pub on_dispatch: bool,
}

#[cfg(not(feature = "CONFIG_SYSCALL_USER_DISPATCH"))]
#[repr(C)]
pub struct syscall_user_dispatch {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
