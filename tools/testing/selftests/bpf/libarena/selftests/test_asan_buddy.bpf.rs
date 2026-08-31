// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include <libarena/common.h>
 * #include <libarena/asan.h>
 * #include <libarena/buddy.h>
 *
 * Required for parsing the ASAN call stacks:
 * #include "test_progs_compat.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr::{addr_of, write_volatile};

type size_t = usize;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;

const ENOMEM: c_int = 12;

#[repr(C)]
pub struct buddy {
	_private: [u8; 0],
}

#[repr(C)]
pub struct buddy_header {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut buddy: buddy;
	static zero: c_int;
	static can_loop: bool;

	static BUDDY_HEADER_OFF: c_int;

	fn buddy_init(buddy: *mut buddy) -> c_int;
	fn buddy_destroy(buddy: *mut buddy);
	fn buddy_alloc(buddy: *mut buddy, size: size_t) -> *mut u8;
	fn buddy_free(buddy: *mut buddy, mem: *mut c_void);

	fn asan_validate() -> c_int;
	fn asan_validate_addr(poisoned: bool, addr: *const c_void) -> c_int;

	fn arena_stdout(fmt: *const c_char, ...);
	fn barrier_var(i: u32);
}

/* Original C code below was conditional on #ifdef BPF_ARENA_ASAN and included
 * "test_asan_common.h" inside that condition.
 */

#[inline(always)]
unsafe fn asan_test_buddy_oob_single(alloc_size: size_t) -> c_int {
	let mut mem: *mut u8;
	let mut ret: c_int;
	let mut i: c_int;

	ret = unsafe { asan_validate() };
	if ret < 0 {
		return ret;
	}

	mem = unsafe { buddy_alloc(addr_of!(buddy).cast_mut(), alloc_size) };
	if mem.is_null() {
		unsafe {
			arena_stdout(
				c"buddy_alloc failed for size %lu".as_ptr(),
				alloc_size,
			);
		}
		return -ENOMEM;
	}

	ret = unsafe { asan_validate() };
	if ret < 0 {
		return ret;
	}

	i = unsafe { zero };
	while (i as size_t) < alloc_size && unsafe { can_loop } {
		unsafe {
			*mem.add(i as usize) = 0xba;
		}
		ret = unsafe { asan_validate_addr(false, mem.add(i as usize).cast_const().cast()) };
		if ret < 0 {
			return ret;
		}
		i += 1;
	}

	unsafe {
		*mem.add(alloc_size) = 0xba;
	}
	ret = unsafe { asan_validate_addr(true, mem.add(alloc_size).cast_const().cast()) };
	if ret < 0 {
		return ret;
	}

	unsafe {
		buddy_free(addr_of!(buddy).cast_mut(), mem.cast());
	}

	0
}

/*
 * Factored out because asan_validate_addr is complex enough to cause
 * verification failures if verified with the rest of asan_test_buddy_uaf_single.
 */
#[no_mangle]
pub unsafe extern "C" fn asan_test_buddy_byte(mem: *mut u8, i: c_int, freed: bool) -> c_int {
	let mut ret: c_int;

	/* The header in freed blocks doesn't get poisoned. */
	if freed
		&& unsafe { BUDDY_HEADER_OFF } <= i
		&& i < unsafe { BUDDY_HEADER_OFF } + size_of::<buddy_header>() as c_int
	{
		return 0;
	}

	unsafe {
		*mem.add(i as usize) = 0xba;
	}
	ret = unsafe { asan_validate_addr(freed, mem.add(i as usize).cast_const().cast()) };
	if ret < 0 {
		return ret;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn asan_test_buddy_uaf_single(alloc_size: size_t) -> c_int {
	let mut mem: *mut u8;
	let mut ret: c_int;
	let mut i: c_int;

	mem = unsafe { buddy_alloc(addr_of!(buddy).cast_mut(), alloc_size) };
	if mem.is_null() {
		unsafe {
			arena_stdout(
				c"buddy_alloc failed for size %lu".as_ptr(),
				alloc_size,
			);
		}
		return -ENOMEM;
	}

	ret = unsafe { asan_validate() };
	if ret < 0 {
		return ret;
	}

	i = unsafe { zero };
	while (i as size_t) < alloc_size && unsafe { can_loop } {
		ret = unsafe { asan_test_buddy_byte(mem, i, false) };
		if ret != 0 {
			return ret;
		}
		i += 1;
	}

	ret = unsafe { asan_validate() };
	if ret < 0 {
		return ret;
	}

	unsafe {
		buddy_free(addr_of!(buddy).cast_mut(), mem.cast());
	}

	i = unsafe { zero };
	while (i as size_t) < alloc_size && unsafe { can_loop } {
		ret = unsafe { asan_test_buddy_byte(mem, i, true) };
		if ret != 0 {
			return ret;
		}
		i += 1;
	}

	0
}

#[repr(C)]
pub struct buddy_blob {
	pub mem: [u8; 48],
	pub oob: u8,
}

#[inline(always)]
unsafe fn asan_test_buddy_blob_single() -> c_int {
	let mut blob: *mut buddy_blob;
	let alloc_size: size_t = size_of::<buddy_blob>() - 1;
	let mut ret: c_int;

	blob = unsafe { buddy_alloc(addr_of!(buddy).cast_mut(), alloc_size).cast() };
	if blob.is_null() {
		return -ENOMEM;
	}

	unsafe {
		write_volatile(addr_of!((*blob).mem[0]).cast_mut(), 0xba);
	}
	ret = unsafe { asan_validate_addr(false, addr_of!((*blob).mem[0]).cast()) };
	if ret < 0 {
		return ret;
	}

	unsafe {
		write_volatile(addr_of!((*blob).mem[47]).cast_mut(), 0xba);
	}
	ret = unsafe { asan_validate_addr(false, addr_of!((*blob).mem[47]).cast()) };
	if ret < 0 {
		return ret;
	}

	unsafe {
		(*blob).oob = 0;
	}
	ret = unsafe { asan_validate_addr(true, addr_of!((*blob).oob).cast()) };
	if ret < 0 {
		return ret;
	}

	unsafe {
		buddy_free(addr_of!(buddy).cast_mut(), blob.cast());
	}

	0
}

/* SEC("syscall")
 * __stderr("Memory violation for address {{.*}} for write of size 1")
 * __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
 * __stderr("Call trace:\n"
 * "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
 * "|[ \t]+[^\n]+\n)*}}")
 */
#[no_mangle]
pub unsafe extern "C" fn asan_test_buddy_oob() -> c_int {
	let sizes: [size_t; 9] = [7, 8, 17, 18, 64, 256, 317, 512, 1024];
	let mut ret: c_int;
	let mut i: u32;

	ret = unsafe { buddy_init(addr_of!(buddy).cast_mut()) };
	if ret != 0 {
		unsafe {
			arena_stdout(c"buddy_init failed with %d".as_ptr(), ret);
		}
		return ret;
	}

	i = unsafe { zero } as u32;
	while (i as usize) < sizes.len() && unsafe { can_loop } {
		unsafe {
			barrier_var(i);
		}
		ret = unsafe { asan_test_buddy_oob_single(sizes[i as usize]) };
		if ret != 0 {
			unsafe {
				arena_stdout(
					c"%s:%d Failed for size %lu".as_ptr(),
					c"asan_test_buddy_oob".as_ptr(),
					line!() as c_int,
					sizes[i as usize],
				);
				buddy_destroy(addr_of!(buddy).cast_mut());
			}
			return ret;
		}
		i += 1;
	}

	unsafe {
		buddy_destroy(addr_of!(buddy).cast_mut());
	}

	ret = unsafe { asan_validate() };
	if ret < 0 {
		return ret;
	}

	0
}

/* SEC("syscall")
 * __stderr("Memory violation for address {{.*}} for write of size 1")
 * __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
 * __stderr("Call trace:\n"
 * "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
 * "|[ \t]+[^\n]+\n)*}}")
 */
#[no_mangle]
pub unsafe extern "C" fn asan_test_buddy_uaf() -> c_int {
	let sizes: [size_t; 8] = [16, 32, 64, 128, 256, 512, 1024, 16384];
	let mut ret: c_int;
	let mut i: u32;

	ret = unsafe { buddy_init(addr_of!(buddy).cast_mut()) };
	if ret != 0 {
		unsafe {
			arena_stdout(c"buddy_init failed with %d".as_ptr(), ret);
		}
		return ret;
	}

	i = unsafe { zero } as u32;
	while (i as usize) < sizes.len() && unsafe { can_loop } {
		unsafe {
			barrier_var(i);
		}
		ret = unsafe { asan_test_buddy_uaf_single(sizes[i as usize]) };
		if ret != 0 {
			unsafe {
				arena_stdout(
					c"%s:%d Failed for size %lu".as_ptr(),
					c"asan_test_buddy_uaf".as_ptr(),
					line!() as c_int,
					sizes[i as usize],
				);
				buddy_destroy(addr_of!(buddy).cast_mut());
			}
			return ret;
		}
		i += 1;
	}

	unsafe {
		buddy_destroy(addr_of!(buddy).cast_mut());
	}

	ret = unsafe { asan_validate() };
	if ret < 0 {
		return ret;
	}

	0
}

/* SEC("syscall")
 * __stderr("Memory violation for address {{.*}} for write of size 1")
 * __stderr("CPU: {{[0-9]+}} UID: 0 PID: {{[0-9]+}} Comm: {{.*}}")
 * __stderr("Call trace:\n"
 * "{{([a-zA-Z_][a-zA-Z0-9_]*\\+0x[0-9a-fA-F]+/0x[0-9a-fA-F]+\n"
 * "|[ \t]+[^\n]+\n)*}}")
 */
#[no_mangle]
pub unsafe extern "C" fn asan_test_buddy_blob() -> c_int {
	let iters: c_int = 10;
	let mut ret: c_int;
	let mut i: c_int;

	ret = unsafe { buddy_init(addr_of!(buddy).cast_mut()) };
	if ret != 0 {
		unsafe {
			arena_stdout(c"buddy_init failed with %d".as_ptr(), ret);
		}
		return ret;
	}

	i = unsafe { zero };
	while i < iters && unsafe { can_loop } {
		ret = unsafe { asan_test_buddy_blob_single() };
		if ret != 0 {
			unsafe {
				arena_stdout(
					c"%s:%d Failed on iteration %d".as_ptr(),
					c"asan_test_buddy_blob".as_ptr(),
					line!() as c_int,
					i,
				);
				buddy_destroy(addr_of!(buddy).cast_mut());
			}
			return ret;
		}
		i += 1;
	}

	unsafe {
		buddy_destroy(addr_of!(buddy).cast_mut());
	}

	ret = unsafe { asan_validate() };
	if ret < 0 {
		return ret;
	}

	0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];
