/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and kernel-only include are omitted from executable Rust.
// `kernel_ulong_t` is supplied by the surrounding kernel bindings.

pub const APR_NAME_SIZE: usize = 32;
pub const APR_MODULE_PREFIX: &str = "apr:";

#[repr(C)]
pub struct apr_device_id {
    pub name: [core::ffi::c_char; APR_NAME_SIZE],
    pub domain_id: u32,
    pub svc_id: u32,
    pub svc_version: u32,
    pub driver_data: kernel_ulong_t, /* Data private to the driver */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
