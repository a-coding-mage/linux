/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header defines kernel_ulong_t and includes linux/types.h only when
 * __KERNEL__ is defined.  In Rust, use the native pointer-sized unsigned
 * integer as the corresponding unsigned-long representation.
 */
pub type kernel_ulong_t = usize;

/* SLIMbus */

pub const SLIMBUS_NAME_SIZE: usize = 32;
pub const SLIMBUS_MODULE_PREFIX: &str = "slim:";

#[repr(C)]
pub struct slim_device_id {
    pub manf_id: u16,
    pub prod_code: u16,
    pub dev_index: u16,
    pub instance: u16,

    /* Data private to the driver */
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
