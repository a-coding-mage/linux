/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2021 Facebook */

/* Translated from C. Dependencies originally came from:
 * #include <test_progs.h>
 * #include <bpf/btf.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct btf {
	_private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
	_private: [u8; 0],
}

#[repr(C)]
pub struct btf_enum {
	pub name_off: c_uint,
	pub val: c_int,
}

pub type bpf_prog_type = c_uint;
pub type bpf_map_type = c_uint;
pub type bpf_func_id = c_uint;

#[repr(C)]
struct case_def {
	prog_type_name: *const c_char,
	helper_name: *const c_char,
	prog_type: bpf_prog_type,
	helper_id: bpf_func_id,
	supported: bool,
}

unsafe extern "C" {
	fn btf__parse(path: *const c_char, opts: *mut c_void) -> *mut btf;
	fn btf__find_by_name_kind(btf: *const btf, name: *const c_char, kind: c_uint) -> c_int;
	fn btf__type_by_id(btf: *const btf, id: c_uint) -> *const btf_type;
	fn btf_enum(t: *const btf_type) -> *const btf_enum;
	fn btf_vlen(t: *const btf_type) -> c_int;
	fn btf__str_by_offset(btf: *const btf, offset: c_uint) -> *const c_char;
	fn btf__free(btf: *mut btf);

	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;

	fn test__start_subtest(name: *const c_char) -> bool;

	fn libbpf_probe_bpf_prog_type(prog_type: bpf_prog_type, opts: *mut c_void) -> c_int;
	fn libbpf_probe_bpf_map_type(map_type: bpf_map_type, opts: *mut c_void) -> c_int;
	fn libbpf_probe_bpf_helper(
		prog_type: bpf_prog_type,
		helper_id: bpf_func_id,
		opts: *mut c_void,
	) -> c_int;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;

	static BTF_KIND_ENUM: c_uint;

	static BPF_PROG_TYPE_UNSPEC: bpf_prog_type;
	static BPF_PROG_TYPE_KPROBE: bpf_prog_type;
	static BPF_PROG_TYPE_SOCKET_FILTER: bpf_prog_type;
	static BPF_PROG_TYPE_SYSCALL: bpf_prog_type;

	static BPF_MAP_TYPE_UNSPEC: bpf_map_type;

	static BPF_FUNC_unspec: bpf_func_id;
	static BPF_FUNC_map_lookup_elem: bpf_func_id;
	static BPF_FUNC_loop: bpf_func_id;
	static BPF_FUNC_ktime_get_coarse_ns: bpf_func_id;
	static BPF_FUNC_sys_bpf: bpf_func_id;
}

pub unsafe fn test_libbpf_probe_prog_types() {
	let btf: *mut btf;
	let mut t: *const btf_type;
	let mut e: *const btf_enum;
	let mut i: c_int;
	let n: c_int;
	let id: c_int;

	btf = btf__parse(c"/sys/kernel/btf/vmlinux".as_ptr(), ptr::null_mut());
	if !ASSERT_OK_PTR(btf as *const c_void, c"btf_parse".as_ptr()) {
		return;
	}

	/* find enum bpf_prog_type and enumerate each value */
	id = btf__find_by_name_kind(btf, c"bpf_prog_type".as_ptr(), BTF_KIND_ENUM);
	if !ASSERT_GT(id, 0, c"bpf_prog_type_id".as_ptr()) {
		btf__free(btf);
		return;
	}
	t = btf__type_by_id(btf, id as c_uint);
	if !ASSERT_OK_PTR(t as *const c_void, c"bpf_prog_type_enum".as_ptr()) {
		btf__free(btf);
		return;
	}

	e = btf_enum(t);
	i = 0;
	n = btf_vlen(t);
	while i < n {
		let prog_type_name: *const c_char = btf__str_by_offset(btf, (*e).name_off);
		let prog_type: bpf_prog_type = (*e).val as bpf_prog_type;
		let res: c_int;

		if prog_type == BPF_PROG_TYPE_UNSPEC {
			e = e.add(1);
			i += 1;
			continue;
		}
		if strcmp(prog_type_name, c"__MAX_BPF_PROG_TYPE".as_ptr()) == 0 {
			e = e.add(1);
			i += 1;
			continue;
		}

		if !test__start_subtest(prog_type_name) {
			e = e.add(1);
			i += 1;
			continue;
		}

		res = libbpf_probe_bpf_prog_type(prog_type, ptr::null_mut());
		ASSERT_EQ(res, 1, prog_type_name);

		e = e.add(1);
		i += 1;
	}

	btf__free(btf);
}

pub unsafe fn test_libbpf_probe_map_types() {
	let btf: *mut btf;
	let mut t: *const btf_type;
	let mut e: *const btf_enum;
	let mut i: c_int;
	let n: c_int;
	let id: c_int;

	btf = btf__parse(c"/sys/kernel/btf/vmlinux".as_ptr(), ptr::null_mut());
	if !ASSERT_OK_PTR(btf as *const c_void, c"btf_parse".as_ptr()) {
		return;
	}

	/* find enum bpf_map_type and enumerate each value */
	id = btf__find_by_name_kind(btf, c"bpf_map_type".as_ptr(), BTF_KIND_ENUM);
	if !ASSERT_GT(id, 0, c"bpf_map_type_id".as_ptr()) {
		btf__free(btf);
		return;
	}
	t = btf__type_by_id(btf, id as c_uint);
	if !ASSERT_OK_PTR(t as *const c_void, c"bpf_map_type_enum".as_ptr()) {
		btf__free(btf);
		return;
	}

	e = btf_enum(t);
	i = 0;
	n = btf_vlen(t);
	while i < n {
		let map_type_name: *const c_char = btf__str_by_offset(btf, (*e).name_off);
		let map_type: bpf_map_type = (*e).val as bpf_map_type;
		let res: c_int;

		if map_type == BPF_MAP_TYPE_UNSPEC {
			e = e.add(1);
			i += 1;
			continue;
		}
		if strcmp(map_type_name, c"__MAX_BPF_MAP_TYPE".as_ptr()) == 0 {
			e = e.add(1);
			i += 1;
			continue;
		}

		if !test__start_subtest(map_type_name) {
			e = e.add(1);
			i += 1;
			continue;
		}

		res = libbpf_probe_bpf_map_type(map_type, ptr::null_mut());
		ASSERT_EQ(res, 1, map_type_name);

		e = e.add(1);
		i += 1;
	}

	btf__free(btf);
}

pub unsafe fn test_libbpf_probe_helpers() {
	/*
	#define CASE(prog, helper, supp) {			\
		.prog_type_name = "BPF_PROG_TYPE_" # prog,	\
		.helper_name = "bpf_" # helper,			\
		.prog_type = BPF_PROG_TYPE_ ## prog,		\
		.helper_id = BPF_FUNC_ ## helper,		\
		.supported = supp,				\
	}
	*/
	let cases: [case_def; 7] = [
		case_def {
			prog_type_name: c"BPF_PROG_TYPE_KPROBE".as_ptr(),
			helper_name: c"bpf_unspec".as_ptr(),
			prog_type: BPF_PROG_TYPE_KPROBE,
			helper_id: BPF_FUNC_unspec,
			supported: false,
		},
		case_def {
			prog_type_name: c"BPF_PROG_TYPE_KPROBE".as_ptr(),
			helper_name: c"bpf_map_lookup_elem".as_ptr(),
			prog_type: BPF_PROG_TYPE_KPROBE,
			helper_id: BPF_FUNC_map_lookup_elem,
			supported: true,
		},
		case_def {
			prog_type_name: c"BPF_PROG_TYPE_KPROBE".as_ptr(),
			helper_name: c"bpf_loop".as_ptr(),
			prog_type: BPF_PROG_TYPE_KPROBE,
			helper_id: BPF_FUNC_loop,
			supported: true,
		},

		case_def {
			prog_type_name: c"BPF_PROG_TYPE_KPROBE".as_ptr(),
			helper_name: c"bpf_ktime_get_coarse_ns".as_ptr(),
			prog_type: BPF_PROG_TYPE_KPROBE,
			helper_id: BPF_FUNC_ktime_get_coarse_ns,
			supported: false,
		},
		case_def {
			prog_type_name: c"BPF_PROG_TYPE_SOCKET_FILTER".as_ptr(),
			helper_name: c"bpf_ktime_get_coarse_ns".as_ptr(),
			prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
			helper_id: BPF_FUNC_ktime_get_coarse_ns,
			supported: true,
		},

		case_def {
			prog_type_name: c"BPF_PROG_TYPE_KPROBE".as_ptr(),
			helper_name: c"bpf_sys_bpf".as_ptr(),
			prog_type: BPF_PROG_TYPE_KPROBE,
			helper_id: BPF_FUNC_sys_bpf,
			supported: false,
		},
		case_def {
			prog_type_name: c"BPF_PROG_TYPE_SYSCALL".as_ptr(),
			helper_name: c"bpf_sys_bpf".as_ptr(),
			prog_type: BPF_PROG_TYPE_SYSCALL,
			helper_id: BPF_FUNC_sys_bpf,
			supported: true,
		},
	];
	let case_cnt: usize = cases.len();
	let mut i: usize;
	let mut buf: [c_char; 128] = [0; 128];

	i = 0;
	while i < case_cnt {
		let d: *const case_def = &cases[i];
		let res: c_int;

		snprintf(
			buf.as_mut_ptr(),
			buf.len(),
			c"%s+%s".as_ptr(),
			(*d).prog_type_name,
			(*d).helper_name,
		);

		if !test__start_subtest(buf.as_ptr()) {
			i += 1;
			continue;
		}

		res = libbpf_probe_bpf_helper((*d).prog_type, (*d).helper_id, ptr::null_mut());
		ASSERT_EQ(res, (*d).supported as c_int, buf.as_ptr());

		i += 1;
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
