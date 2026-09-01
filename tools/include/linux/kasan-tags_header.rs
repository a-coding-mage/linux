/* SPDX-License-Identifier: GPL-2.0 */

pub const KASAN_TAG_KERNEL: u32 = 0xFF; /* native kernel pointers tag */
pub const KASAN_TAG_INVALID: u32 = 0xFE; /* inaccessible memory tag */
pub const KASAN_TAG_MAX: u32 = 0xFD; /* maximum value for random tags */

/*
 * C conditional:
 * #ifdef CONFIG_KASAN_HW_TAGS
 * #define KASAN_TAG_MIN 0xF0
 * #else
 * #define KASAN_TAG_MIN 0x00
 * #endif
 */
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const KASAN_TAG_MIN: u32 = 0xF0; /* minimum value for random tags */

#[cfg(not(CONFIG_KASAN_HW_TAGS))]
pub const KASAN_TAG_MIN: u32 = 0x00; /* minimum value for random tags */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
