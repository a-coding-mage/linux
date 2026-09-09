/* SPDX-License-Identifier: GPL-2.0 */

// C conditional: this type is declared when __KERNEL__ is defined.
pub type kernel_ulong_t = ::core::ffi::c_ulong;

/* rpmsg */

pub const RPMSG_NAME_SIZE: usize = 32;
pub const RPMSG_DEVICE_MODALIAS_FMT: &str = "rpmsg:%s";

#[repr(C)]
pub struct rpmsg_device_id {
    pub name: [::core::ffi::c_char; RPMSG_NAME_SIZE],
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
