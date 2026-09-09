/* SPDX-License-Identifier: GPL-2.0 */

pub const KASAN_TAG_KERNEL: u8 = 0xFF; // native kernel pointers tag
pub const KASAN_TAG_INVALID: u8 = 0xFE; // inaccessible memory tag
pub const KASAN_TAG_MAX: u8 = 0xFD; // maximum value for random tags

#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const KASAN_TAG_MIN: u8 = 0xF0; // minimum value for random tags

#[cfg(not(CONFIG_KASAN_HW_TAGS))]
pub const KASAN_TAG_MIN: u8 = 0x00; // minimum value for random tags

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
