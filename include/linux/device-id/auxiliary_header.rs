/* SPDX-License-Identifier: GPL-2.0 */

// The C definition is conditional on __KERNEL__. This translation assumes the
// kernel-side declaration is being consumed.
pub type kernel_ulong_t = ::core::ffi::c_ulong;

pub const AUXILIARY_NAME_SIZE: usize = 40;
pub const AUXILIARY_MODULE_PREFIX: &str = "auxiliary:";

#[repr(C)]
pub struct auxiliary_device_id {
    pub name: [::core::ffi::c_char; AUXILIARY_NAME_SIZE],
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
