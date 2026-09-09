/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2021, Microsoft Corporation.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

/* Translated from the C header; C include dependencies are supplied externally. */

/*
 * trans_alloc_page
 *	- Allocator that should return exactly one zeroed page, if this
 *	  allocator fails, trans_pgd_create_copy() and trans_pgd_idmap_page()
 *	  return -ENOMEM error.
 *
 * trans_alloc_arg
 *	- Passed to trans_alloc_page as an argument
 */
#[repr(C)]
pub struct trans_pgd_info {
	pub trans_alloc_page: Option<unsafe extern "C" fn(arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void>,
	pub trans_alloc_arg: *mut core::ffi::c_void,
}

extern "C" {
	pub fn trans_pgd_create_copy(
		info: *mut trans_pgd_info,
		trans_pgd: *mut *mut pgd_t,
		start: usize,
		end: usize,
	) -> i32;

	pub fn trans_pgd_idmap_page(
		info: *mut trans_pgd_info,
		trans_ttbr0: *mut phys_addr_t,
		t0sz: *mut usize,
		page: *mut core::ffi::c_void,
	) -> i32;

	pub fn trans_pgd_copy_el2_vectors(
		info: *mut trans_pgd_info,
		el2_vectors: *mut phys_addr_t,
	) -> i32;

	pub static mut trans_pgd_stub_vectors: [core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
