/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header defines kernel_ulong_t when compiled in the kernel.  Rust's
 * usize has the corresponding platform-dependent unsigned-word layout.
 */
pub type kernel_ulong_t = usize;

/* USB Type-C Alternate Modes */

pub const TYPEC_ANY_MODE: u32 = 0x7;

/**
 * struct typec_device_id - USB Type-C alternate mode identifiers
 * @svid: Standard or Vendor ID
 * @mode: Mode index
 * @driver_data: Driver specific data
 */
#[repr(C)]
pub struct typec_device_id {
    pub svid: u16,
    pub mode: u8,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
