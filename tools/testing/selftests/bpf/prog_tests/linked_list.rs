// SPDX-License-Identifier: GPL-2.0
// Translated from linked_list.c. External symbols come from the original
// libbpf, selftest, network helper, and generated skeleton dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

const LOG_BUF_SIZE: usize = 1024 * 1024;
static mut log_buf: [c_char; LOG_BUF_SIZE] = [0; LOG_BUF_SIZE];

#[repr(C)]
struct btf {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_object_open_opts {
	kernel_log_buf: *mut c_char,
	kernel_log_size: usize,
	kernel_log_level: c_uint,
}

#[repr(C)]
struct bpf_test_run_opts {
	data_in: *const c_void,
	data_size_in: u32,
	repeat: u32,
	retval: u32,
}

#[repr(C)]
struct linked_list_fail {
	obj: *mut bpf_object,
}

#[repr(C)]
struct linked_list_progs {
	map_list_push_pop: *mut bpf_program,
	clear_map_list: *mut bpf_program,
	inner_map_list_push_pop: *mut bpf_program,
	clear_inner_map_list: *mut bpf_program,
	global_list_push_pop: *mut bpf_program,
	clear_global_list: *mut bpf_program,
	global_list_push_pop_nested: *mut bpf_program,
	clear_global_nested_list: *mut bpf_program,
	global_list_array_push_pop: *mut bpf_program,
	clear_global_array_list: *mut bpf_program,
	map_list_push_pop_multiple: *mut bpf_program,
	inner_map_list_push_pop_multiple: *mut bpf_program,
	global_list_push_pop_multiple: *mut bpf_program,
	map_list_in_list: *mut bpf_program,
	inner_map_list_in_list: *mut bpf_program,
	global_list_in_list: *mut bpf_program,
}

#[repr(C)]
struct linked_list {
	progs: linked_list_progs,
}

#[repr(C)]
struct linked_list_fail_test {
	prog_name: *const c_char,
	err_msg: *const c_char,
}

const BTF_INT_SIGNED: c_uint = 1;
const E2BIG: c_int = 7;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EEXIST: c_int = 17;
const ELOOP: c_int = 40;

const SPIN_LOCK: c_int = 2;
const LIST_HEAD: c_int = 3;
const LIST_NODE: c_int = 4;

const TEST_ALL: c_int = 0;
const PUSH_POP: c_int = 1;
const PUSH_POP_MULT: c_int = 2;
const LIST_IN_LIST: c_int = 3;

unsafe extern "C" {
	static pkt_v4: [u8; 0];

	fn linked_list_fail__open_opts(opts: *const bpf_object_open_opts) -> *mut linked_list_fail;
	fn linked_list_fail__load(skel: *mut linked_list_fail) -> c_int;
	fn linked_list_fail__destroy(skel: *mut linked_list_fail);
	fn linked_list__open_and_load() -> *mut linked_list;
	fn linked_list__destroy(skel: *mut linked_list);

	fn bpf_object__find_program_by_name(
		obj: *mut bpf_object,
		name: *const c_char,
	) -> *mut bpf_program;
	fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

	fn btf__new_empty() -> *mut btf;
	fn btf__free(btf: *mut btf);
	fn btf__add_int(btf: *mut btf, name: *const c_char, sz: c_uint, encoding: c_uint) -> c_int;
	fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: c_uint) -> c_int;
	fn btf__add_field(
		btf: *mut btf,
		name: *const c_char,
		type_id: c_int,
		bit_offset: c_uint,
		bit_size: c_uint,
	) -> c_int;
	fn btf__add_decl_tag(
		btf: *mut btf,
		value: *const c_char,
		type_id: c_int,
		component_idx: c_int,
	) -> c_int;
	fn btf__load_into_kernel(btf: *mut btf) -> c_int;

	fn test__start_subtest(name: *const c_char) -> bool;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
	static mut stderr: *mut c_void;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(ret: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn RUN_TESTS_linked_list_peek();
}

macro_rules! cstr {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

macro_rules! fail_test {
	($prog:literal, $err:literal) => {
		linked_list_fail_test {
			prog_name: cstr!($prog),
			err_msg: cstr!($err),
		}
	};
}

static mut linked_list_fail_tests: [linked_list_fail_test; 91] = [
	fail_test!("kptr_missing_lock_push_front", "bpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("kptr_missing_lock_push_back", "bpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("kptr_missing_lock_pop_front", "bpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("kptr_missing_lock_pop_back", "bpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("global_missing_lock_push_front", "bpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("global_missing_lock_push_back", "bpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("global_missing_lock_pop_front", "bpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("global_missing_lock_pop_back", "bpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("map_missing_lock_push_front", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_missing_lock_push_back", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_missing_lock_pop_front", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_missing_lock_pop_back", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_missing_lock_push_front", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_missing_lock_push_back", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_missing_lock_pop_front", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_missing_lock_pop_back", "bpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_kptr_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("kptr_global_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("kptr_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_inner_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_kptr_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("kptr_global_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("kptr_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_inner_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_kptr_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("kptr_global_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("kptr_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_inner_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_kptr_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("kptr_global_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("kptr_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("kptr_inner_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_kptr_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("global_global_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("global_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_inner_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_kptr_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("global_global_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("global_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_inner_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_kptr_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("global_global_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("global_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_inner_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_kptr_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("global_global_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("global_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("global_inner_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_kptr_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("map_global_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("map_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_inner_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_kptr_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("map_global_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("map_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_inner_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_kptr_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("map_global_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("map_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_inner_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_kptr_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("map_global_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("map_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_inner_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_kptr_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("inner_map_global_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("inner_map_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_inner_map_incorrect_lock_push_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_kptr_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("inner_map_global_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("inner_map_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_inner_map_incorrect_lock_push_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_kptr_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("inner_map_global_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("inner_map_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_inner_map_incorrect_lock_pop_front", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_kptr_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=40 must be held for bpf_list_head"),
	fail_test!("inner_map_global_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=16 must be held for bpf_list_head"),
	fail_test!("inner_map_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("inner_map_inner_map_incorrect_lock_pop_back", "held lock and object are not in the same allocation\nbpf_spin_lock at off=0 must be held for bpf_list_head"),
	fail_test!("map_compat_kprobe", "tracing progs cannot use bpf_{list_head,rb_root} yet"),
	fail_test!("map_compat_kretprobe", "tracing progs cannot use bpf_{list_head,rb_root} yet"),
	fail_test!("map_compat_tp", "tracing progs cannot use bpf_{list_head,rb_root} yet"),
	fail_test!("map_compat_perf", "tracing progs cannot use bpf_{list_head,rb_root} yet"),
	fail_test!("map_compat_raw_tp", "tracing progs cannot use bpf_{list_head,rb_root} yet"),
	fail_test!("map_compat_raw_tp_w", "tracing progs cannot use bpf_{list_head,rb_root} yet"),
	fail_test!("obj_type_id_oor", "local type ID argument must be in range [0, U32_MAX]"),
	fail_test!("obj_new_no_composite", "bpf_obj_new/bpf_percpu_obj_new type ID argument must be of a struct"),
	fail_test!("obj_new_no_struct", "bpf_obj_new/bpf_percpu_obj_new type ID argument must be of a struct"),
	fail_test!("obj_new_flex_array", "access beyond struct obj_new_flex"),
	fail_test!("obj_drop_non_zero_off", "R1 must have zero offset when passed to release func"),
	fail_test!("new_null_ret", "R0 invalid mem access 'ptr_or_null_'"),
	fail_test!("obj_new_acq", "Unreleased reference id="),
	fail_test!("use_after_drop", "invalid mem access 'scalar'"),
	fail_test!("ptr_walk_scalar", "type=rdonly_untrusted_mem expected=percpu_ptr_"),
	fail_test!("direct_read_lock", "direct access to bpf_spin_lock is disallowed"),
	fail_test!("direct_write_lock", "direct access to bpf_spin_lock is disallowed"),
	fail_test!("direct_read_head", "direct access to bpf_list_head is disallowed"),
	fail_test!("direct_write_head", "direct access to bpf_list_head is disallowed"),
	fail_test!("direct_read_node", "direct access to bpf_list_node is disallowed"),
	fail_test!("direct_write_node", "direct access to bpf_list_node is disallowed"),
	fail_test!("use_after_unlock_push_front", "invalid mem access 'scalar'"),
	fail_test!("use_after_unlock_push_back", "invalid mem access 'scalar'"),
	fail_test!("double_push_front", "R2 expected pointer to allocated object"),
	fail_test!("double_push_back", "R2 expected pointer to allocated object"),
	fail_test!("no_node_value_type", "bpf_list_node not found at offset=0"),
	fail_test!("incorrect_value_type", "operation on bpf_list_head expects arg#1 bpf_list_node at offset=48 in struct foo, but arg is at offset=0 in struct bar"),
	fail_test!("incorrect_node_var_off", "variable ptr_ access var_off=(0x0; 0x1ffffffff) disallowed"),
	fail_test!("incorrect_node_off1", "bpf_list_node not found at offset=49"),
	fail_test!("incorrect_node_off2", "arg#1 offset=0, but expected bpf_list_node at offset=48 in struct foo"),
	fail_test!("no_head_type", "bpf_list_head not found at offset=0"),
	fail_test!("incorrect_head_var_off1", "R1 doesn't have constant offset"),
	fail_test!("incorrect_head_var_off2", "variable ptr_ access var_off=(0x0; 0x1ffffffff) disallowed"),
	fail_test!("incorrect_head_off1", "bpf_list_head not found at offset=25"),
	fail_test!("incorrect_head_off2", "bpf_list_head not found at offset=1"),
	fail_test!("pop_front_off", "off 48 doesn't point to 'struct bpf_spin_lock' that is at 40"),
	fail_test!("pop_back_off", "off 48 doesn't point to 'struct bpf_spin_lock' that is at 40"),
];

unsafe fn test_linked_list_fail_prog(prog_name: *const c_char, err_msg: *const c_char) {
	let opts = bpf_object_open_opts {
		kernel_log_buf: log_buf.as_mut_ptr(),
		kernel_log_size: core::mem::size_of_val(&log_buf),
		kernel_log_level: 1,
	};
	let skel: *mut linked_list_fail;
	let prog: *mut bpf_program;
	let ret: c_int;

	skel = linked_list_fail__open_opts(&opts);
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("linked_list_fail__open_opts")) {
		return;
	}

	prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
	if !ASSERT_OK_PTR(prog as *const c_void, cstr!("bpf_object__find_program_by_name")) {
		linked_list_fail__destroy(skel);
		return;
	}

	bpf_program__set_autoload(prog, true);

	ret = linked_list_fail__load(skel);
	if !ASSERT_ERR(ret, cstr!("linked_list_fail__load must fail")) {
		linked_list_fail__destroy(skel);
		return;
	}

	if !ASSERT_OK_PTR(strstr(log_buf.as_ptr(), err_msg) as *const c_void, cstr!("expected error message")) {
		fprintf(stderr, cstr!("Expected: %s\n"), err_msg);
		fprintf(stderr, cstr!("Verifier: %s\n"), log_buf.as_ptr());
	}

	linked_list_fail__destroy(skel);
}

unsafe fn clear_fields(prog: *mut bpf_program) {
	let mut opts = bpf_test_run_opts {
		data_in: core::ptr::null(),
		data_size_in: 0,
		repeat: 0,
		retval: 0,
	};
	let ret: c_int;

	ret = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts);
	ASSERT_OK(ret, cstr!("clear_fields"));
	ASSERT_OK(opts.retval as c_int, cstr!("clear_fields retval"));
}

unsafe fn run_prog(prog: *mut bpf_program, opts: *mut bpf_test_run_opts, name: *const c_char, retval_name: *const c_char) {
	let ret = bpf_prog_test_run_opts(bpf_program__fd(prog), opts);
	ASSERT_OK(ret, name);
	ASSERT_OK((*opts).retval as c_int, retval_name);
}

unsafe fn test_linked_list_success(mode: c_int, leave_in_map: bool) {
	let mut opts = bpf_test_run_opts {
		data_in: pkt_v4.as_ptr() as *const c_void,
		data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
		repeat: 1,
		retval: 0,
	};
	let skel: *mut linked_list;

	skel = linked_list__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("linked_list__open_and_load")) {
		return;
	}

	if mode != LIST_IN_LIST {
		if mode != PUSH_POP_MULT {
			run_prog((*skel).progs.map_list_push_pop, &mut opts, cstr!("map_list_push_pop"), cstr!("map_list_push_pop retval"));
			if !leave_in_map {
				clear_fields((*skel).progs.clear_map_list);
			}

			run_prog((*skel).progs.inner_map_list_push_pop, &mut opts, cstr!("inner_map_list_push_pop"), cstr!("inner_map_list_push_pop retval"));
			if !leave_in_map {
				clear_fields((*skel).progs.clear_inner_map_list);
			}

			run_prog((*skel).progs.global_list_push_pop, &mut opts, cstr!("global_list_push_pop"), cstr!("global_list_push_pop retval"));
			if !leave_in_map {
				clear_fields((*skel).progs.clear_global_list);
			}

			run_prog((*skel).progs.global_list_push_pop_nested, &mut opts, cstr!("global_list_push_pop_nested"), cstr!("global_list_push_pop_nested retval"));
			if !leave_in_map {
				clear_fields((*skel).progs.clear_global_nested_list);
			}

			run_prog((*skel).progs.global_list_array_push_pop, &mut opts, cstr!("global_list_array_push_pop"), cstr!("global_list_array_push_pop retval"));
			if !leave_in_map {
				clear_fields((*skel).progs.clear_global_array_list);
			}

			if mode == PUSH_POP {
				linked_list__destroy(skel);
				return;
			}
		}

		run_prog((*skel).progs.map_list_push_pop_multiple, &mut opts, cstr!("map_list_push_pop_multiple"), cstr!("map_list_push_pop_multiple retval"));
		if !leave_in_map {
			clear_fields((*skel).progs.clear_map_list);
		}

		run_prog((*skel).progs.inner_map_list_push_pop_multiple, &mut opts, cstr!("inner_map_list_push_pop_multiple"), cstr!("inner_map_list_push_pop_multiple retval"));
		if !leave_in_map {
			clear_fields((*skel).progs.clear_inner_map_list);
		}

		run_prog((*skel).progs.global_list_push_pop_multiple, &mut opts, cstr!("global_list_push_pop_multiple"), cstr!("global_list_push_pop_multiple retval"));
		if !leave_in_map {
			clear_fields((*skel).progs.clear_global_list);
		}

		if mode == PUSH_POP_MULT {
			linked_list__destroy(skel);
			return;
		}
	}

	run_prog((*skel).progs.map_list_in_list, &mut opts, cstr!("map_list_in_list"), cstr!("map_list_in_list retval"));
	if !leave_in_map {
		clear_fields((*skel).progs.clear_map_list);
	}

	run_prog((*skel).progs.inner_map_list_in_list, &mut opts, cstr!("inner_map_list_in_list"), cstr!("inner_map_list_in_list retval"));
	if !leave_in_map {
		clear_fields((*skel).progs.clear_inner_map_list);
	}

	run_prog((*skel).progs.global_list_in_list, &mut opts, cstr!("global_list_in_list"), cstr!("global_list_in_list retval"));
	ASSERT_OK(opts.retval as c_int, cstr!("global_list_in_list retval"));
	if !leave_in_map {
		clear_fields((*skel).progs.clear_global_list);
	}

	linked_list__destroy(skel);
}

unsafe fn init_btf() -> *mut btf {
	let id: c_int;
	let lid: c_int;
	let hid: c_int;
	let nid: c_int;
	let btf: *mut btf;

	btf = btf__new_empty();
	if !ASSERT_OK_PTR(btf as *const c_void, cstr!("btf__new_empty")) {
		return core::ptr::null_mut();
	}
	id = btf__add_int(btf, cstr!("int"), 4, BTF_INT_SIGNED);
	if !ASSERT_EQ(id, 1, cstr!("btf__add_int")) {
		btf__free(btf);
		return core::ptr::null_mut();
	}
	lid = btf__add_struct(btf, cstr!("bpf_spin_lock"), 4);
	if !ASSERT_EQ(lid, SPIN_LOCK, cstr!("btf__add_struct bpf_spin_lock")) {
		btf__free(btf);
		return core::ptr::null_mut();
	}
	hid = btf__add_struct(btf, cstr!("bpf_list_head"), 16);
	if !ASSERT_EQ(hid, LIST_HEAD, cstr!("btf__add_struct bpf_list_head")) {
		btf__free(btf);
		return core::ptr::null_mut();
	}
	nid = btf__add_struct(btf, cstr!("bpf_list_node"), 24);
	if !ASSERT_EQ(nid, LIST_NODE, cstr!("btf__add_struct bpf_list_node")) {
		btf__free(btf);
		return core::ptr::null_mut();
	}
	btf
}

unsafe fn list_and_rb_node_same_struct(refcount_field: bool) {
	let bpf_rb_node_btf_id: c_int;
	let mut bpf_refcount_btf_id: c_int = 0;
	let foo_btf_id: c_int;
	let btf: *mut btf;
	let mut id: c_int;
	let mut err: c_int;

	btf = init_btf();
	if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) {
		return;
	}

	bpf_rb_node_btf_id = btf__add_struct(btf, cstr!("bpf_rb_node"), 32);
	if !ASSERT_GT(bpf_rb_node_btf_id, 0, cstr!("btf__add_struct bpf_rb_node")) {
		return;
	}

	if refcount_field {
		bpf_refcount_btf_id = btf__add_struct(btf, cstr!("bpf_refcount"), 4);
		if !ASSERT_GT(bpf_refcount_btf_id, 0, cstr!("btf__add_struct bpf_refcount")) {
			return;
		}
	}

	id = btf__add_struct(btf, cstr!("bar"), if refcount_field { 60 } else { 56 });
	if !ASSERT_GT(id, 0, cstr!("btf__add_struct bar")) {
		return;
	}
	err = btf__add_field(btf, cstr!("a"), LIST_NODE, 0, 0);
	if !ASSERT_OK(err, cstr!("btf__add_field bar::a")) {
		return;
	}
	err = btf__add_field(btf, cstr!("c"), bpf_rb_node_btf_id, 192, 0);
	if !ASSERT_OK(err, cstr!("btf__add_field bar::c")) {
		return;
	}
	if refcount_field {
		err = btf__add_field(btf, cstr!("ref"), bpf_refcount_btf_id, 448, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::ref")) {
			return;
		}
	}

	foo_btf_id = btf__add_struct(btf, cstr!("foo"), 20);
	if !ASSERT_GT(foo_btf_id, 0, cstr!("btf__add_struct foo")) {
		return;
	}
	err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
	if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) {
		return;
	}
	err = btf__add_field(btf, cstr!("b"), SPIN_LOCK, 128, 0);
	if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) {
		return;
	}
	id = btf__add_decl_tag(btf, cstr!("contains:bar:a"), foo_btf_id, 0);
	if !ASSERT_GT(id, 0, cstr!("btf__add_decl_tag contains:bar:a")) {
		return;
	}

	err = btf__load_into_kernel(btf);
	ASSERT_EQ(err, if refcount_field { 0 } else { -EINVAL }, cstr!("check btf"));
	btf__free(btf);
}

macro_rules! btf_subtest {
	($name:literal, $body:block) => {
		while test__start_subtest(cstr!($name)) {
			$body
			break;
		}
	};
}

unsafe fn test_btf() {
	let mut btf: *mut btf = core::ptr::null_mut();
	let mut id: c_int;
	let mut err: c_int;

	btf_subtest!("btf: too many locks", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 24);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), SPIN_LOCK, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_struct foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), SPIN_LOCK, 32, 0);
		if !ASSERT_OK(err, cstr!("btf__add_struct foo::a")) { break; }
		err = btf__add_field(btf, cstr!("c"), LIST_HEAD, 64, 0);
		if !ASSERT_OK(err, cstr!("btf__add_struct foo::a")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -E2BIG, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: missing lock", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 16);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_struct foo::a")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:baz:a"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:baz:a")) { break; }
		id = btf__add_struct(btf, cstr!("baz"), 16);
		if !ASSERT_EQ(id, 7, cstr!("btf__add_struct baz")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_NODE, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field baz::a")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -EINVAL, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: bad offset", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 36);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:foo:b"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:foo:b")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -EEXIST, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: missing contains:", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 24);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), SPIN_LOCK, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_HEAD, 64, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -EINVAL, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: missing struct", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 24);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), SPIN_LOCK, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_HEAD, 64, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bar:bar"), 5, 1);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:bar:bar")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -ENOENT, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: missing node", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 24);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), SPIN_LOCK, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_HEAD, 64, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:foo:c"), 5, 1);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:foo:c")) { break; }
		err = btf__load_into_kernel(btf);
		btf__free(btf);
		ASSERT_EQ(err, -ENOENT, cstr!("check btf"));
	});

	btf_subtest!("btf: node incorrect type", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 20);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), SPIN_LOCK, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bar:a"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:bar:a")) { break; }
		id = btf__add_struct(btf, cstr!("bar"), 4);
		if !ASSERT_EQ(id, 7, cstr!("btf__add_struct bar")) { break; }
		err = btf__add_field(btf, cstr!("a"), SPIN_LOCK, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::a")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -EINVAL, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: multiple bpf_list_node with name b", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 52);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 256, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::c")) { break; }
		err = btf__add_field(btf, cstr!("d"), SPIN_LOCK, 384, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::d")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:foo:b"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:foo:b")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -EINVAL, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: owning | owned AA cycle", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 44);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:foo:b"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:foo:b")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -ELOOP, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: owning | owned ABA cycle", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 44);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bar:b"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:bar:b")) { break; }
		id = btf__add_struct(btf, cstr!("bar"), 44);
		if !ASSERT_EQ(id, 7, cstr!("btf__add_struct bar")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:foo:b"), 7, 0);
		if !ASSERT_EQ(id, 8, cstr!("btf__add_decl_tag contains:foo:b")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -ELOOP, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: owning -> owned", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 28);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), SPIN_LOCK, 192, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bar:a"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:bar:a")) { break; }
		id = btf__add_struct(btf, cstr!("bar"), 24);
		if !ASSERT_EQ(id, 7, cstr!("btf__add_struct bar")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_NODE, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::a")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, 0, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: owning -> owning | owned -> owned", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 28);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), SPIN_LOCK, 192, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bar:b"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:bar:b")) { break; }
		id = btf__add_struct(btf, cstr!("bar"), 44);
		if !ASSERT_EQ(id, 7, cstr!("btf__add_struct bar")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:baz:a"), 7, 0);
		if !ASSERT_EQ(id, 8, cstr!("btf__add_decl_tag contains:baz:a")) { break; }
		id = btf__add_struct(btf, cstr!("baz"), 24);
		if !ASSERT_EQ(id, 9, cstr!("btf__add_struct baz")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_NODE, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field baz:a")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, 0, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: owning | owned -> owning | owned -> owned", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 44);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bar:b"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:bar:b")) { break; }
		id = btf__add_struct(btf, cstr!("bar"), 44);
		if !ASSERT_EQ(id, 7, cstr!("btf__add_struct bar")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar:a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar:b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar:c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:baz:a"), 7, 0);
		if !ASSERT_EQ(id, 8, cstr!("btf__add_decl_tag contains:baz:a")) { break; }
		id = btf__add_struct(btf, cstr!("baz"), 24);
		if !ASSERT_EQ(id, 9, cstr!("btf__add_struct baz")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_NODE, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field baz:a")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -ELOOP, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: owning -> owning | owned -> owning | owned -> owned", {
		btf = init_btf();
		if !ASSERT_OK_PTR(btf as *const c_void, cstr!("init_btf")) { break; }
		id = btf__add_struct(btf, cstr!("foo"), 20);
		if !ASSERT_EQ(id, 5, cstr!("btf__add_struct foo")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), SPIN_LOCK, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field foo::b")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bar:b"), 5, 0);
		if !ASSERT_EQ(id, 6, cstr!("btf__add_decl_tag contains:bar:b")) { break; }
		id = btf__add_struct(btf, cstr!("bar"), 44);
		if !ASSERT_EQ(id, 7, cstr!("btf__add_struct bar")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:baz:b"), 7, 0);
		if !ASSERT_EQ(id, 8, cstr!("btf__add_decl_tag")) { break; }
		id = btf__add_struct(btf, cstr!("baz"), 44);
		if !ASSERT_EQ(id, 9, cstr!("btf__add_struct baz")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_HEAD, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::a")) { break; }
		err = btf__add_field(btf, cstr!("b"), LIST_NODE, 128, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::b")) { break; }
		err = btf__add_field(btf, cstr!("c"), SPIN_LOCK, 320, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bar::c")) { break; }
		id = btf__add_decl_tag(btf, cstr!("contains:bam:a"), 9, 0);
		if !ASSERT_EQ(id, 10, cstr!("btf__add_decl_tag contains:bam:a")) { break; }
		id = btf__add_struct(btf, cstr!("bam"), 24);
		if !ASSERT_EQ(id, 11, cstr!("btf__add_struct bam")) { break; }
		err = btf__add_field(btf, cstr!("a"), LIST_NODE, 0, 0);
		if !ASSERT_OK(err, cstr!("btf__add_field bam::a")) { break; }
		err = btf__load_into_kernel(btf);
		ASSERT_EQ(err, -ELOOP, cstr!("check btf"));
		btf__free(btf);
	});

	btf_subtest!("btf: list_node and rb_node in same struct", {
		list_and_rb_node_same_struct(true);
	});

	btf_subtest!("btf: list_node and rb_node in same struct, no bpf_refcount", {
		list_and_rb_node_same_struct(false);
	});
}

#[no_mangle]
pub unsafe extern "C" fn test_linked_list() {
	let mut i: usize = 0;

	while i < linked_list_fail_tests.len() {
		if !test__start_subtest(linked_list_fail_tests[i].prog_name) {
			i += 1;
			continue;
		}
		test_linked_list_fail_prog(
			linked_list_fail_tests[i].prog_name,
			linked_list_fail_tests[i].err_msg,
		);
		i += 1;
	}
	test_btf();
	test_linked_list_success(PUSH_POP, false);
	test_linked_list_success(PUSH_POP, true);
	test_linked_list_success(PUSH_POP_MULT, false);
	test_linked_list_success(PUSH_POP_MULT, true);
	test_linked_list_success(LIST_IN_LIST, false);
	test_linked_list_success(LIST_IN_LIST, true);
	test_linked_list_success(TEST_ALL, false);
}

#[no_mangle]
pub unsafe extern "C" fn test_linked_list_peek() {
	RUN_TESTS_linked_list_peek();
}
