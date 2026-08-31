// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies: <test_progs.h>, <network_helpers.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

type __u32 = u32;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const BPF_PROG_TYPE_CGROUP_SKB: c_int = 8;
const BPF_CGROUP_INET_EGRESS: c_int = 1;
const BPF_F_ALLOW_MULTI: u32 = 1;

unsafe extern "C" {
	fn test__join_cgroup(path: *const c_char) -> c_int;
	fn start_server(
		family: c_int,
		type_: c_int,
		addr: *const c_void,
		port: c_int,
		timeout_ms: c_int,
	) -> c_int;
	fn bpf_prog_test_load(
		file: *const c_char,
		prog_type: c_int,
		pobj: *mut *mut bpf_object,
		prog_fd: *mut c_int,
	) -> c_int;
	fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_object__find_program_by_name(
		obj: *mut bpf_object,
		name: *const c_char,
	) -> *mut bpf_program;
	fn bpf_prog_attach(
		prog_fd: c_int,
		target_fd: c_int,
		attach_type: c_int,
		attach_flags: u32,
	) -> c_int;
	fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
	fn bpf_object__close(obj: *mut bpf_object);

	fn CHECK_FAIL(condition: bool) -> bool;
	fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_load_bytes_relative() {
	let server_fd: c_int;
	let cgroup_fd: c_int;
	let mut prog_fd: c_int = 0;
	let map_fd: c_int;
	let client_fd: c_int;
	let mut err: c_int;
	let mut obj: *mut bpf_object = core::ptr::null_mut();
	let prog: *mut bpf_program;
	let test_result: *mut bpf_map;
	let mut _duration: __u32 = 0;

	let map_key: __u32 = 0;
	let mut map_value: __u32 = 0;

	cgroup_fd = test__join_cgroup(b"/load_bytes_relative\0".as_ptr() as *const c_char);
	if CHECK_FAIL(cgroup_fd < 0) {
		return;
	}

	server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null(), 0, 0);
	if CHECK_FAIL(server_fd < 0) {
		close(cgroup_fd);
		return;
	}

	err = bpf_prog_test_load(
		b"./load_bytes_relative.bpf.o\0".as_ptr() as *const c_char,
		BPF_PROG_TYPE_CGROUP_SKB,
		&mut obj,
		&mut prog_fd,
	);
	if CHECK_FAIL(err != 0) {
		close(server_fd);
		close(cgroup_fd);
		return;
	}

	test_result =
		bpf_object__find_map_by_name(obj, b"test_result\0".as_ptr() as *const c_char);
	if CHECK_FAIL(test_result.is_null()) {
		bpf_object__close(obj);
		close(server_fd);
		close(cgroup_fd);
		return;
	}

	map_fd = bpf_map__fd(test_result);
	if map_fd < 0 {
		bpf_object__close(obj);
		close(server_fd);
		close(cgroup_fd);
		return;
	}

	prog =
		bpf_object__find_program_by_name(obj, b"load_bytes_relative\0".as_ptr() as *const c_char);
	if CHECK_FAIL(prog.is_null()) {
		bpf_object__close(obj);
		close(server_fd);
		close(cgroup_fd);
		return;
	}

	err = bpf_prog_attach(
		prog_fd,
		cgroup_fd,
		BPF_CGROUP_INET_EGRESS,
		BPF_F_ALLOW_MULTI,
	);
	if CHECK_FAIL(err != 0) {
		bpf_object__close(obj);
		close(server_fd);
		close(cgroup_fd);
		return;
	}

	client_fd = connect_to_fd(server_fd, 0);
	if CHECK_FAIL(client_fd < 0) {
		bpf_object__close(obj);
		close(server_fd);
		close(cgroup_fd);
		return;
	}
	close(client_fd);

	err = bpf_map_lookup_elem(
		map_fd,
		&map_key as *const __u32 as *const c_void,
		&mut map_value as *mut __u32 as *mut c_void,
	);
	if CHECK_FAIL(err != 0) {
		bpf_object__close(obj);
		close(server_fd);
		close(cgroup_fd);
		return;
	}

	CHECK(
		map_value != 1,
		b"bpf\0".as_ptr() as *const c_char,
		b"bpf program returned failure\0".as_ptr() as *const c_char,
	);

	bpf_object__close(obj);
	close(server_fd);
	close(cgroup_fd);
}
