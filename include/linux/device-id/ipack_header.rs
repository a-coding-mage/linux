/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: LINUX_DEVICE_ID_IPACK_H */

pub const IPACK_ANY_FORMAT: u8 = 0xff;
pub const IPACK_ANY_ID: u32 = !0;

#[repr(C)]
pub struct ipack_device_id {
	pub format: u8,  /* Format version or IPACK_ANY_ID */
	pub vendor: u32, /* Vendor ID or IPACK_ANY_ID */
	pub device: u32, /* Device ID or IPACK_ANY_ID */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
