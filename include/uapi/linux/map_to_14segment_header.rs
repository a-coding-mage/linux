/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of map_to_14segment.h. */

const EINVAL: i32 = 22;
pub const BIT_SEG14_A: u32 = 0;
pub const BIT_SEG14_B: u32 = 1;
pub const BIT_SEG14_C: u32 = 2;
pub const BIT_SEG14_D: u32 = 3;
pub const BIT_SEG14_E: u32 = 4;
pub const BIT_SEG14_F: u32 = 5;
pub const BIT_SEG14_G1: u32 = 6;
pub const BIT_SEG14_G2: u32 = 7;
pub const BIT_SEG14_H: u32 = 8;
pub const BIT_SEG14_I: u32 = 9;
pub const BIT_SEG14_J: u32 = 10;
pub const BIT_SEG14_K: u32 = 11;
pub const BIT_SEG14_L: u32 = 12;
pub const BIT_SEG14_M: u32 = 13;
pub const BIT_SEG14_RESERVED1: u32 = 14;
pub const BIT_SEG14_RESERVED2: u32 = 15;

#[repr(C)]
pub struct seg14_conversion_map { pub table: [u16; 128] }

#[inline]
pub unsafe fn map_to_seg14(map: *mut seg14_conversion_map, c: i32) -> i32 {
    if c < 0 || c >= 128 { return -EINVAL; }
    u16::from_be((*map).table[c as usize]) as i32
}

pub const MAP_TO_SEG14_SYSFS_FILE: &str = "map_seg14";

/* _SEG14 and the ASCII range macros are expanded here into the equivalent table. */
pub const MAP_ASCII14SEG_ALPHANUM: [u16; 128] = [
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0x0060,0x0006,0x0486,0x0b29,0x0b2d,0x0c24,0x1529,0x0200,
    0x1400,0x0900,0x3f00,0x1c0,0x0800,0x00c0,0x2000,0x1800,
    0x0c3f,0x0406,0x00db,0x004f,0x0066,0x00ad,0x00fd,0x0407,
    0x00ff,0x006f,0x1200,0x1000,0x0940,0x00c0,0x0280,0x0443,
    0x02ff,0x006f,0x00f9,0x0039,0x044f,0x0079,0x0071,0x013d,
    0x0076,0x1209,0x000f,0x1500,0x0038,0x0176,0x1076,0x003f,
    0x0079,0x103f,0x1079,0x00ed,0x1201,0x003e,0x1806,0x1c06,
    0x1b00,0x1206,0x1809,0x0039,0x1006,0x003f,0x0040,0x0008,
    0x0100,0x1c08,0x10f8,0x00d8,0x048e,0x00d8,0x0140,0x0040,
    0x0c08,0x1000,0x0800,0x1500,0x0030,0x0148,0x0148,0x00d8,
    0x0148,0x0048,0x0098,0x00f0,0x00e0,0x0038,0x0800,0x0800,
    0x0c00,0x0c00,0x0040,0x0249,0x0148,0x0000,0x0000,0x0000,
];

#[macro_export]
macro_rules! SEG14_DEFAULT_MAP { ($name:ident) => {
    let $name = seg14_conversion_map { table: MAP_ASCII14SEG_ALPHANUM };
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
