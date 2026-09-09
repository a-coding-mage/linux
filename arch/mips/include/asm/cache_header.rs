/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997, 98, 99, 2000, 2003 Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

// Dependency supplied externally by the original C environment: <kmalloc.h>.

pub const L1_CACHE_SHIFT: usize = CONFIG_MIPS_L1_CACHE_SHIFT;
pub const L1_CACHE_BYTES: usize = 1usize << L1_CACHE_SHIFT;

// C attribute macro: __section(".data..read_mostly").

unsafe extern "C" {
    pub fn cache_noop();
    pub fn r3k_cache_init();
    pub fn r3k_cache_size(value: usize) -> usize;
    pub fn r3k_cache_lsize(value: usize) -> usize;
    pub fn r4k_cache_init();
    pub fn octeon_cache_init();
    pub fn au1x00_fixup_config_od();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
