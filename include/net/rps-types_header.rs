/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Define a rps_tag_ptr:
 * Low order 5 bits are used to store the ilog2(size) of an RPS table.
 */
pub type rps_tag_ptr = usize;

pub fn rps_tag_to_log(tag_ptr: rps_tag_ptr) -> u8 {
    (tag_ptr & 31usize) as u8
}

pub fn rps_tag_to_mask(tag_ptr: rps_tag_ptr) -> u32 {
    (1u32 << rps_tag_to_log(tag_ptr)) - 1
}

pub fn rps_tag_to_table(tag_ptr: rps_tag_ptr) -> *mut core::ffi::c_void {
    (tag_ptr & !31usize) as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
