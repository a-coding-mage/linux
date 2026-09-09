/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * This is a lookup table for speeding up access to the .orc_unwind table.
 * Given an input address offset, the corresponding lookup table entry
 * specifies a subset of the .orc_unwind table to search.
 *
 * Each block represents the end of the previous range and the start of the
 * next range.  An extra block is added to give the last range an end.
 *
 * The block size should be a power of 2 to avoid a costly 'div' instruction.
 *
 * A block size of 256 was chosen because it roughly doubles unwinder
 * performance while only adding ~5% to the ORC data footprint.
 */
pub const LOOKUP_BLOCK_ORDER: u32 = 8;
pub const LOOKUP_BLOCK_SIZE: u32 = 1u32 << LOOKUP_BLOCK_ORDER;

/* `LINKER_SCRIPT` is a build-time condition from the C header. */
#[cfg(not(LINKER_SCRIPT))]
unsafe extern "C" {
    pub static mut orc_lookup: [core::ffi::c_uint; 0];
    pub static mut orc_lookup_end: [core::ffi::c_uint; 0];
    pub static _stext: core::ffi::c_uchar;
    pub static _etext: core::ffi::c_uchar;
}

#[cfg(not(LINKER_SCRIPT))]
#[inline]
pub unsafe fn lookup_start_ip() -> usize {
    core::ptr::addr_of!(_stext) as usize
}

#[cfg(not(LINKER_SCRIPT))]
#[inline]
pub unsafe fn lookup_stop_ip() -> usize {
    core::ptr::addr_of!(_etext) as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
