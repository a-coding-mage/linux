/* SPDX-License-Identifier: GPL-2.0 */

// The C header defines this alias only when __KERNEL__ is set. Rust has no
// direct preprocessor equivalent here; `usize` preserves unsigned long's
// platform-sized intent.
pub type kernel_ulong_t = usize;

pub const AP_DEVICE_ID_MATCH_CARD_TYPE: u32 = 0x01;
pub const AP_DEVICE_ID_MATCH_QUEUE_TYPE: u32 = 0x02;

/* s390 AP bus devices */
#[repr(C)]
pub struct ap_device_id {
    pub match_flags: u16, /* which fields to match against */
    pub dev_type: u8,     /* device type */
    pub driver_info: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
