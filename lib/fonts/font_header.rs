/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency: declarations supplied by <linux/font.h>. */

/*
 * Font data
 */

pub const FONT_EXTRA_WORDS: usize = 4;

#[repr(C, packed)]
pub struct font_data {
	pub extra: [core::ffi::c_uint; FONT_EXTRA_WORDS],
	pub data: [u8; 0],
}

/*
 * Built-in fonts
 */

pub const VGA8x8_IDX: i32 = 0;
pub const VGA8x16_IDX: i32 = 1;
pub const PEARL8x8_IDX: i32 = 2;
pub const VGA6x11_IDX: i32 = 3;
pub const FONT7x14_IDX: i32 = 4;
pub const FONT10x18_IDX: i32 = 5;
pub const SUN8x16_IDX: i32 = 6;
pub const SUN12x22_IDX: i32 = 7;
pub const ACORN8x8_IDX: i32 = 8;
pub const MINI4x6_IDX: i32 = 9;
pub const FONT6x10_IDX: i32 = 10;
pub const TER16x32_IDX: i32 = 11;
pub const FONT6x8_IDX: i32 = 12;
pub const TER10x18_IDX: i32 = 13;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
