// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Translated from C. External declarations correspond to dependencies from:
 * <test_progs.h>, <network_helpers.h>, "stack_arg.skel.h",
 * and "stack_arg_kfunc.skel.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
	pub sz: usize,
	pub data_in: *const c_void,
	pub data_size_in: c_uint,
	pub repeat: c_uint,
	pub retval: c_uint,
}

#[repr(C)]
pub struct stack_arg_rodata {
	pub has_stack_arg: bool,
}

#[repr(C)]
pub struct stack_arg_bss {
	pub timer_result: c_int,
}

#[repr(C)]
pub struct stack_arg_progs {
	pub test_global_many_args: *mut bpf_program,
	pub test_async_cb_many_args: *mut bpf_program,
	pub test_bpf2bpf_ptr_stack_arg: *mut bpf_program,
	pub test_bpf2bpf_mix_stack_args: *mut bpf_program,
	pub test_bpf2bpf_nesting_stack_arg: *mut bpf_program,
	pub test_bpf2bpf_dynptr_stack_arg: *mut bpf_program,
	pub test_two_callees: *mut bpf_program,
}

#[repr(C)]
pub struct stack_arg {
	pub rodata: *mut stack_arg_rodata,
	pub bss: *mut stack_arg_bss,
	pub progs: stack_arg_progs,
}

#[repr(C)]
pub struct stack_arg_kfunc_progs {
	pub test_stack_arg_scalar: *mut bpf_program,
	pub test_stack_arg_ptr: *mut bpf_program,
	pub test_stack_arg_mix: *mut bpf_program,
	pub test_stack_arg_dynptr: *mut bpf_program,
	pub test_stack_arg_mem: *mut bpf_program,
	pub test_stack_arg_iter: *mut bpf_program,
	pub test_stack_arg_const_str: *mut bpf_program,
	pub test_stack_arg_timer: *mut bpf_program,
}

#[repr(C)]
pub struct stack_arg_kfunc {
	pub rodata: *mut stack_arg_rodata,
	pub progs: stack_arg_kfunc_progs,
}

unsafe extern "C" {
	static pkt_v4: [u8; 0];

	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

	fn stack_arg__open() -> *mut stack_arg;
	fn stack_arg__load(skel: *mut stack_arg) -> c_int;
	fn stack_arg__destroy(skel: *mut stack_arg);

	fn stack_arg_kfunc__open() -> *mut stack_arg_kfunc;
	fn stack_arg_kfunc__load(skel: *mut stack_arg_kfunc) -> c_int;
	fn stack_arg_kfunc__destroy(skel: *mut stack_arg_kfunc);

	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn test__skip();
	fn test__start_subtest(name: *const c_char) -> bool;
	fn usleep(usec: c_uint) -> c_int;
}

const TEST_RUN: *const c_char = b"test_run\0".as_ptr() as *const c_char;
const RETVAL: *const c_char = b"retval\0".as_ptr() as *const c_char;
const OPEN: *const c_char = b"open\0".as_ptr() as *const c_char;
const LOAD: *const c_char = b"load\0".as_ptr() as *const c_char;
const TIMER_RESULT: *const c_char = b"timer_result\0".as_ptr() as *const c_char;

unsafe fn run_subtest(prog: *mut bpf_program, expected: c_int) {
	let mut topts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		data_in: pkt_v4.as_ptr() as *const c_void,
		data_size_in: core::mem::size_of_val(&pkt_v4) as c_uint,
		repeat: 1,
		retval: 0,
	};

	let prog_fd = bpf_program__fd(prog);
	let err = bpf_prog_test_run_opts(prog_fd, &mut topts);
	ASSERT_OK(err, TEST_RUN);
	ASSERT_EQ(topts.retval as c_int, expected, RETVAL);
}

unsafe fn test_global_many() {
	let skel = stack_arg__open();
	if !ASSERT_OK_PTR(skel as *const c_void, OPEN) {
		return;
	}

	if !(*(*skel).rodata).has_stack_arg {
		test__skip();
		stack_arg__destroy(skel);
		return;
	}

	if !ASSERT_OK(stack_arg__load(skel), LOAD) {
		stack_arg__destroy(skel);
		return;
	}

	run_subtest((*skel).progs.test_global_many_args, 55);

	stack_arg__destroy(skel);
}

unsafe fn test_async_cb_many() {
	let skel = stack_arg__open();
	if !ASSERT_OK_PTR(skel as *const c_void, OPEN) {
		return;
	}

	if !(*(*skel).rodata).has_stack_arg {
		test__skip();
		stack_arg__destroy(skel);
		return;
	}

	if !ASSERT_OK(stack_arg__load(skel), LOAD) {
		stack_arg__destroy(skel);
		return;
	}

	run_subtest((*skel).progs.test_async_cb_many_args, 0);

	/* Wait for the timer callback to fire and verify the result.
	 * 10+20+30+40+50+60+70+80+90+100 = 550
	 */
	usleep(50);
	ASSERT_EQ((*(*skel).bss).timer_result, 550, TIMER_RESULT);

	stack_arg__destroy(skel);
}

unsafe fn test_bpf2bpf() {
	let skel = stack_arg__open();
	if !ASSERT_OK_PTR(skel as *const c_void, OPEN) {
		return;
	}

	if !(*(*skel).rodata).has_stack_arg {
		test__skip();
		stack_arg__destroy(skel);
		return;
	}

	if !ASSERT_OK(stack_arg__load(skel), LOAD) {
		stack_arg__destroy(skel);
		return;
	}

	run_subtest((*skel).progs.test_bpf2bpf_ptr_stack_arg, 75);
	run_subtest((*skel).progs.test_bpf2bpf_mix_stack_args, 66);
	run_subtest((*skel).progs.test_bpf2bpf_nesting_stack_arg, 84);
	run_subtest((*skel).progs.test_bpf2bpf_dynptr_stack_arg, 99);
	run_subtest((*skel).progs.test_two_callees, 133);

	stack_arg__destroy(skel);
}

unsafe fn test_kfunc() {
	let skel = stack_arg_kfunc__open();
	if !ASSERT_OK_PTR(skel as *const c_void, OPEN) {
		return;
	}

	if !(*(*skel).rodata).has_stack_arg {
		test__skip();
		stack_arg_kfunc__destroy(skel);
		return;
	}

	if !ASSERT_OK(stack_arg_kfunc__load(skel), LOAD) {
		stack_arg_kfunc__destroy(skel);
		return;
	}

	run_subtest((*skel).progs.test_stack_arg_scalar, 55);
	run_subtest((*skel).progs.test_stack_arg_ptr, 75);
	run_subtest((*skel).progs.test_stack_arg_mix, 66);
	run_subtest((*skel).progs.test_stack_arg_dynptr, 99);
	run_subtest((*skel).progs.test_stack_arg_mem, 151);
	run_subtest((*skel).progs.test_stack_arg_iter, 145);
	run_subtest((*skel).progs.test_stack_arg_const_str, 45);
	run_subtest((*skel).progs.test_stack_arg_timer, 45);

	stack_arg_kfunc__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stack_arg() {
	if test__start_subtest(b"global_many_args\0".as_ptr() as *const c_char) {
		test_global_many();
	}
	if test__start_subtest(b"async_cb_many_args\0".as_ptr() as *const c_char) {
		test_async_cb_many();
	}
	if test__start_subtest(b"bpf2bpf\0".as_ptr() as *const c_char) {
		test_bpf2bpf();
	}
	if test__start_subtest(b"kfunc\0".as_ptr() as *const c_char) {
		test_kfunc();
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
