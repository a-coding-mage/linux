// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025 Red Hat, Inc.*/
// Dependencies from the original C file:
// <test_progs.h>
// "string_kfuncs_success.skel.h"
// "string_kfuncs_failure1.skel.h"
// "string_kfuncs_failure2.skel.h"
// <sys/mman.h>

use core::ffi::{c_char, c_int, c_void};

const E2BIG: c_int = 7;

#[repr(C)]
pub struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
	pub sz: usize,
	pub retval: u32,
}

#[repr(C)]
pub struct string_kfuncs_failure2_bss {
	pub long_str: [c_char; 0],
}

#[repr(C)]
pub struct string_kfuncs_failure2 {
	pub obj: *mut bpf_object,
	pub bss: *mut string_kfuncs_failure2_bss,
}

unsafe extern "C" {
	fn string_kfuncs_failure2__open_and_load() -> *mut string_kfuncs_failure2;
	fn string_kfuncs_failure2__destroy(skel: *mut string_kfuncs_failure2);
	fn bpf_object__find_program_by_name(
		obj: *mut bpf_object,
		name: *const c_char,
	) -> *mut bpf_program;
	fn bpf_program__fd(prog: *const bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: u32, expected: c_int, name: *const c_char) -> bool;
	fn test__start_subtest(name: *const c_char) -> bool;
	fn RUN_TESTS(name: *const c_char);
}

static TEST_CASES: [&[u8]; 17] = [
	b"strcmp\0",
	b"strcasecmp\0",
	b"strncasecmp\0",
	b"strchr\0",
	b"strchrnul\0",
	b"strnchr\0",
	b"strrchr\0",
	b"strlen\0",
	b"strnlen\0",
	b"strspn_str\0",
	b"strspn_accept\0",
	b"strcspn_str\0",
	b"strcspn_reject\0",
	b"strstr\0",
	b"strcasestr\0",
	b"strnstr\0",
	b"strncasestr\0",
];

pub unsafe fn run_too_long_tests() {
	let mut skel: *mut string_kfuncs_failure2;
	let mut prog: *mut bpf_program;
	let mut test_name = [0 as c_char; 256];
	let mut err: c_int;

	skel = unsafe { string_kfuncs_failure2__open_and_load() };
	if !unsafe {
		ASSERT_OK_PTR(
			skel as *const c_void,
			b"string_kfuncs_failure2__open_and_load\0".as_ptr() as *const c_char,
		)
	} {
		return;
	}

	unsafe {
		memset(
			(*(*skel).bss).long_str.as_mut_ptr() as *mut c_void,
			b'a' as c_int,
			core::mem::size_of_val(&(*(*skel).bss).long_str),
		);
	}

	for i in 0..TEST_CASES.len() {
		unsafe {
			snprintf(
				test_name.as_mut_ptr(),
				test_name.len(),
				b"test_%s_too_long\0".as_ptr() as *const c_char,
				TEST_CASES[i].as_ptr() as *const c_char,
			);
		}
		if !unsafe { test__start_subtest(test_name.as_ptr()) } {
			continue;
		}

		prog = unsafe { bpf_object__find_program_by_name((*skel).obj, test_name.as_ptr()) };
		if !unsafe {
			ASSERT_OK_PTR(
				prog as *const c_void,
				b"bpf_object__find_program_by_name\0".as_ptr() as *const c_char,
			)
		} {
			goto_cleanup(skel);
			return;
		}

		let mut topts = bpf_test_run_opts {
			sz: core::mem::size_of::<bpf_test_run_opts>(),
			retval: 0,
		};
		err = unsafe { bpf_prog_test_run_opts(bpf_program__fd(prog), &mut topts) };
		if !unsafe { ASSERT_OK(err, b"bpf_prog_test_run\0".as_ptr() as *const c_char) } {
			goto_cleanup(skel);
			return;
		}

		unsafe {
			ASSERT_EQ(
				topts.retval,
				-E2BIG,
				b"reading too long string fails with -E2BIG\0".as_ptr() as *const c_char,
			);
		}
	}

	goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut string_kfuncs_failure2) {
	unsafe {
		string_kfuncs_failure2__destroy(skel);
	}
}

pub unsafe fn test_string_kfuncs() {
	unsafe {
		RUN_TESTS(b"string_kfuncs_success\0".as_ptr() as *const c_char);
		RUN_TESTS(b"string_kfuncs_failure1\0".as_ptr() as *const c_char);

		run_too_long_tests();
	}
}
