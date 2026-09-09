/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Profiling infrastructure declarations.
 *
 *  This file is based on gcc-internal definitions. Data structures are
 *  defined to be compatible with gcc counterparts. For a better
 *  understanding, refer to gcc source: gcc/gcov-io.h.
 *
 *    Copyright IBM Corp. 2009
 *    Author(s): Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
 *
 *    Uses gcc-internal data definitions.
 */

/* C header guard GCOV_H. */

/* Profiling data types used for gcc 3.4 and above - these are defined by
 * gcc and need to be kept as close to the original definition as possible to
 * remain compatible.
 */
pub const GCOV_DATA_MAGIC: u32 = 0x67636461;
pub const GCOV_TAG_FUNCTION: u32 = 0x01000000;
pub const GCOV_TAG_COUNTER_BASE: u32 = 0x01a10000;

#[inline]
pub const fn gcov_tag_for_counter(count: u32) -> u32 {
	GCOV_TAG_COUNTER_BASE.wrapping_add(count.wrapping_shl(17))
}

#[cfg(target_pointer_width = "64")]
pub type gcov_type = libc::c_long;
#[cfg(not(target_pointer_width = "64"))]
pub type gcov_type = i64;

/* Opaque gcov_info. The gcov structures can change as for example in gcc 4.7 so
 * we cannot use full definition here and they need to be placed in gcc specific
 * implementation of gcov. This also means no direct access to the members in
 * generic code and usage of the interface below.*/
#[repr(C)]
pub struct gcov_info {
	_opaque: [u8; 0],
}

/* Interface to access gcov_info data  */
unsafe extern "C" {
	pub fn gcov_info_filename(info: *mut gcov_info) -> *const libc::c_char;
	pub fn gcov_info_version(info: *mut gcov_info) -> u32;
	pub fn gcov_info_next(info: *mut gcov_info) -> *mut gcov_info;
	pub fn gcov_info_link(info: *mut gcov_info);
	pub fn gcov_info_unlink(prev: *mut gcov_info, info: *mut gcov_info);
	pub fn gcov_info_within_module(info: *mut gcov_info, mod_: *mut crate::module) -> bool;
	pub fn convert_to_gcda(buffer: *mut libc::c_char, info: *mut gcov_info) -> usize;

	/* Base interface. */
	pub fn gcov_event(action: gcov_action, info: *mut gcov_info);
	pub fn gcov_enable_events();

	/* writing helpers */
	pub fn store_gcov_u32(buffer: *mut libc::c_void, off: usize, v: u32) -> usize;
	pub fn store_gcov_u64(buffer: *mut libc::c_void, off: usize, v: u64) -> usize;

	/* gcov_info control. */
	pub fn gcov_info_reset(info: *mut gcov_info);
	pub fn gcov_info_is_compatible(info1: *mut gcov_info, info2: *mut gcov_info) -> i32;
	pub fn gcov_info_add(dest: *mut gcov_info, source: *mut gcov_info);
	pub fn gcov_info_dup(info: *mut gcov_info) -> *mut gcov_info;
	pub fn gcov_info_free(info: *mut gcov_info);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gcov_action {
	GCOV_ADD,
	GCOV_REMOVE,
}

#[repr(C)]
pub enum gcov_link_dir {
	OBJ_TREE,
	SRC_TREE,
}

#[repr(C)]
pub struct gcov_link {
	pub dir: gcov_link_dir,
	pub ext: *const libc::c_char,
}

unsafe extern "C" {
	pub static gcov_link: *const gcov_link;
	pub static mut gcov_events_enabled: i32;
	pub static mut gcov_lock: crate::mutex;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
