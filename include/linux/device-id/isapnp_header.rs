/* SPDX-License-Identifier: GPL-2.0 */

// C source condition: `kernel_ulong_t` is declared under `__KERNEL__`.
// Rust's `usize` preserves the platform-sized unsigned integer intent.
pub type kernel_ulong_t = usize;

pub const ISAPNP_ANY_ID: u16 = 0xffff;

#[repr(C)]
pub struct isapnp_device_id {
    pub card_vendor: u16,
    pub card_device: u16,
    pub vendor: u16,
    pub function: u16,
    pub driver_data: kernel_ulong_t, // data private to the driver
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
