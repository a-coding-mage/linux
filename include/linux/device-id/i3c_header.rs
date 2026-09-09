/* SPDX-License-Identifier: GPL-2.0 */

// The C header conditionally includes <linux/types.h> under __KERNEL__.

/* i3c */

pub const I3C_MATCH_DCR: u32 = 0x1;
pub const I3C_MATCH_MANUF: u32 = 0x2;
pub const I3C_MATCH_PART: u32 = 0x4;
pub const I3C_MATCH_EXTRA_INFO: u32 = 0x8;

#[repr(C)]
pub struct i3c_device_id {
	pub match_flags: u8,
	pub dcr: u8,
	pub manuf_id: u16,
	pub part_id: u16,
	pub extra_info: u16,

	pub data: *const core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
