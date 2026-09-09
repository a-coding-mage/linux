/* SPDX-License-Identifier: GPL-2.0 */

// The C definition is enabled when compiling with __KERNEL__.
#[cfg(feature = "__KERNEL__")]
pub type kernel_ulong_t = libc::c_ulong;

pub const PLATFORM_NAME_SIZE: usize = 24;
pub const PLATFORM_MODULE_PREFIX: &[u8] = b"platform:\0";

#[repr(C)]
pub struct platform_device_id {
    pub name: [libc::c_char; PLATFORM_NAME_SIZE],
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
