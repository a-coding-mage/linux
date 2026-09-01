// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies: <test_progs.h>, <sys/mman.h>, "mmap_inner_array.skel.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type __u64 = u64;
type pid_t = c_int;
type size_t = usize;
type useconds_t = c_uint;

const _SC_PAGE_SIZE: c_int = 30;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mmap_inner_array_maps {
	pub inner_array: *mut bpf_map,
	pub outer_map: *mut bpf_map,
}

#[repr(C)]
pub struct mmap_inner_array_bss {
	pub pid: pid_t,
	pub pid_match: bool,
	pub outer_map_match: bool,
	pub done: bool,
}

#[repr(C)]
pub struct mmap_inner_array_data {
	pub match_value: __u64,
}

#[repr(C)]
pub struct mmap_inner_array {
	pub maps: mmap_inner_array_maps,
	pub bss: *mut mmap_inner_array_bss,
	pub data: *mut mmap_inner_array_data,
}

unsafe extern "C" {
	fn sysconf(name: c_int) -> c_long;
	fn mmap(
		addr: *mut c_void,
		len: size_t,
		prot: c_int,
		flags: c_int,
		fd: c_int,
		offset: c_long,
	) -> *mut c_void;
	fn munmap(addr: *mut c_void, len: size_t) -> c_int;
	fn getpid() -> pid_t;
	fn usleep(usec: useconds_t) -> c_int;

	fn mmap_inner_array__open_and_load() -> *mut mmap_inner_array;
	fn mmap_inner_array__attach(skel: *mut mmap_inner_array) -> c_int;
	fn mmap_inner_array__destroy(skel: *mut mmap_inner_array);

	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_map__update_elem(
		map: *mut bpf_map,
		key: *const c_void,
		key_sz: size_t,
		value: *const c_void,
		value_sz: size_t,
		flags: u64,
	) -> c_int;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
	fn ASSERT_FALSE(actual: bool, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;
}

pub unsafe fn test_mmap_inner_array() {
	let page_size: c_long = sysconf(_SC_PAGE_SIZE);
	let mut skel: *mut mmap_inner_array;
	let inner_array_fd: c_int;
	let mut err: c_int;
	let tmp: *mut c_void;
	let val: *mut __u64;

	skel = mmap_inner_array__open_and_load();

	if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
		return;
	}

	inner_array_fd = bpf_map__fd((*skel).maps.inner_array);
	tmp = mmap(
		core::ptr::null_mut(),
		page_size as size_t,
		PROT_READ | PROT_WRITE,
		MAP_SHARED,
		inner_array_fd,
		0,
	);
	if !ASSERT_OK_PTR(tmp as *const c_void, c"inner array mmap".as_ptr()) {
		mmap_inner_array__destroy(skel);
		return;
	}
	val = tmp as *mut __u64;

	err = mmap_inner_array__attach(skel);
	if !ASSERT_OK(err, c"attach".as_ptr()) {
		munmap(tmp, page_size as size_t);
		mmap_inner_array__destroy(skel);
		return;
	}

	(*(*skel).bss).pid = getpid();
	usleep(1);

	/* pid is set, pid_match == true and outer_map_match == false */
	ASSERT_TRUE((*(*skel).bss).pid_match, c"pid match 1".as_ptr());
	ASSERT_FALSE(
		(*(*skel).bss).outer_map_match,
		c"outer map match 1".as_ptr(),
	);
	ASSERT_FALSE((*(*skel).bss).done, c"done 1".as_ptr());
	ASSERT_EQ(*val, 0, c"value match 1".as_ptr());

	err = bpf_map__update_elem(
		(*skel).maps.outer_map,
		&(*(*skel).bss).pid as *const pid_t as *const c_void,
		core::mem::size_of_val(&(*(*skel).bss).pid),
		&inner_array_fd as *const c_int as *const c_void,
		core::mem::size_of_val(&inner_array_fd),
		BPF_ANY,
	);
	if !ASSERT_OK(err, c"update elem".as_ptr()) {
		munmap(tmp, page_size as size_t);
		mmap_inner_array__destroy(skel);
		return;
	}
	usleep(1);

	/* outer map key is set, outer_map_match == true */
	ASSERT_TRUE((*(*skel).bss).pid_match, c"pid match 2".as_ptr());
	ASSERT_TRUE(
		(*(*skel).bss).outer_map_match,
		c"outer map match 2".as_ptr(),
	);
	ASSERT_TRUE((*(*skel).bss).done, c"done 2".as_ptr());
	ASSERT_EQ(
		*val,
		(*(*skel).data).match_value,
		c"value match 2".as_ptr(),
	);

	munmap(tmp, page_size as size_t);
	mmap_inner_array__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
