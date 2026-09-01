// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2020 Google LLC.
 */

// C dependencies translated as external declarations:
// <test_progs.h>, <sys/wait.h>, <unistd.h>, "lsm.skel.h",
// and "lsm_tailcall.skel.h".

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const BPF_ANY: u64 = 0;
const __NR_setdomainname: c_long = 171;

#[repr(C)]
pub struct bpf_link {
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

#[repr(C)]
pub struct lsm_bss {
	pub monitored_pid: c_int,
	pub bprm_count: c_int,
	pub mprotect_count: c_int,
	pub copy_test: c_int,
}

#[repr(C)]
pub struct lsm_progs {
	pub test_int_hook: *mut bpf_program,
}

#[repr(C)]
pub struct lsm {
	pub progs: lsm_progs,
	pub bss: *mut lsm_bss,
}

#[repr(C)]
pub struct lsm_tailcall_maps {
	pub jmp_table: *mut bpf_map,
}

#[repr(C)]
pub struct lsm_tailcall_progs {
	pub lsm_file_permission_prog: *mut bpf_program,
	pub lsm_kernfs_init_security_prog: *mut bpf_program,
}

#[repr(C)]
pub struct lsm_tailcall {
	pub maps: lsm_tailcall_maps,
	pub progs: lsm_tailcall_progs,
}

unsafe extern "C" {
	fn fork() -> c_int;
	fn getpid() -> c_int;
	fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;
	fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
	fn syscall(num: c_long, ...) -> c_long;
	fn __errno_location() -> *mut c_int;

	fn stack_mprotect() -> c_int;

	fn lsm__open_and_load() -> *mut lsm;
	fn lsm__attach(skel: *mut lsm) -> c_int;
	fn lsm__detach(skel: *mut lsm);
	fn lsm__destroy(skel: *mut lsm);

	fn lsm_tailcall__open_and_load() -> *mut lsm_tailcall;
	fn lsm_tailcall__destroy(skel: *mut lsm_tailcall);

	fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_map_update_elem(
		fd: c_int,
		key: *const c_void,
		value: *const c_void,
		flags: u64,
	) -> c_int;

	fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn CHECK_FAIL(condition: bool) -> bool;
	fn test__start_subtest(name: *const c_char) -> bool;
}

static mut CMD_ARGS: [*mut c_char; 2] = [
	b"true\0".as_ptr() as *mut c_char,
	ptr::null_mut(),
];

unsafe fn exec_cmd(monitored_pid: *mut c_int) -> c_int {
	let child_pid: c_int;
	let mut child_status: c_int = 0;

	child_pid = fork();
	if child_pid == 0 {
		*monitored_pid = getpid();
		execvp(CMD_ARGS[0], CMD_ARGS.as_ptr());
		return -EINVAL;
	} else if child_pid > 0 {
		waitpid(child_pid, &mut child_status, 0);
		return child_status;
	}

	-EINVAL
}

unsafe fn test_lsm(skel: *mut lsm) -> c_int {
	let mut link: *mut bpf_link;
	let buf: c_int = 1234;
	let mut err: c_int;

	err = lsm__attach(skel);
	if !ASSERT_OK(err, b"attach\0".as_ptr() as *const c_char) {
		return err;
	}

	/* Check that already linked program can't be attached again. */
	link = bpf_program__attach((*skel).progs.test_int_hook);
	if !ASSERT_ERR_PTR(link as *const c_void, b"attach_link\0".as_ptr() as *const c_char) {
		return -1;
	}

	err = exec_cmd(&mut (*(*skel).bss).monitored_pid);
	if !ASSERT_OK(err, b"exec_cmd\0".as_ptr() as *const c_char) {
		return err;
	}

	ASSERT_EQ((*(*skel).bss).bprm_count, 1, b"bprm_count\0".as_ptr() as *const c_char);

	(*(*skel).bss).monitored_pid = getpid();

	err = stack_mprotect();
	if !ASSERT_EQ(err, -1, b"stack_mprotect\0".as_ptr() as *const c_char)
		|| !ASSERT_EQ(*__errno_location(), EPERM, b"stack_mprotect\0".as_ptr() as *const c_char)
	{
		return err;
	}

	ASSERT_EQ(
		(*(*skel).bss).mprotect_count,
		1,
		b"mprotect_count\0".as_ptr() as *const c_char,
	);

	syscall(__NR_setdomainname, &buf as *const c_int, -2_i64 as c_long);
	syscall(__NR_setdomainname, 0 as c_long, -3_i64 as c_long);
	syscall(__NR_setdomainname, !0_i64 as c_long, -4_i64 as c_long);

	ASSERT_EQ((*(*skel).bss).copy_test, 3, b"copy_test\0".as_ptr() as *const c_char);

	lsm__detach(skel);

	(*(*skel).bss).copy_test = 0;
	(*(*skel).bss).bprm_count = 0;
	(*(*skel).bss).mprotect_count = 0;
	0
}

unsafe fn test_lsm_basic() {
	let mut skel: *mut lsm = ptr::null_mut();
	let mut err: c_int;

	skel = lsm__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, b"lsm_skel_load\0".as_ptr() as *const c_char) {
		goto_close_prog_lsm_basic(skel);
		return;
	}

	err = test_lsm(skel);
	if !ASSERT_OK(err, b"test_lsm_first_attach\0".as_ptr() as *const c_char) {
		goto_close_prog_lsm_basic(skel);
		return;
	}

	err = test_lsm(skel);
	ASSERT_OK(err, b"test_lsm_second_attach\0".as_ptr() as *const c_char);

	goto_close_prog_lsm_basic(skel);
}

unsafe fn goto_close_prog_lsm_basic(skel: *mut lsm) {
	lsm__destroy(skel);
}

unsafe fn test_lsm_tailcall() {
	let mut skel: *mut lsm_tailcall = ptr::null_mut();
	let mut map_fd: c_int;
	let mut prog_fd: c_int;
	let mut err: c_int;
	let mut key: c_int;

	skel = lsm_tailcall__open_and_load();
	if !ASSERT_OK_PTR(
		skel as *const c_void,
		b"lsm_tailcall__skel_load\0".as_ptr() as *const c_char,
	) {
		goto_close_prog_lsm_tailcall(skel);
		return;
	}

	map_fd = bpf_map__fd((*skel).maps.jmp_table);
	if CHECK_FAIL(map_fd < 0) {
		goto_close_prog_lsm_tailcall(skel);
		return;
	}

	prog_fd = bpf_program__fd((*skel).progs.lsm_file_permission_prog);
	if CHECK_FAIL(prog_fd < 0) {
		goto_close_prog_lsm_tailcall(skel);
		return;
	}

	key = 0;
	err = bpf_map_update_elem(
		map_fd,
		&key as *const c_int as *const c_void,
		&prog_fd as *const c_int as *const c_void,
		BPF_ANY,
	);
	if CHECK_FAIL(err == 0) {
		goto_close_prog_lsm_tailcall(skel);
		return;
	}

	prog_fd = bpf_program__fd((*skel).progs.lsm_kernfs_init_security_prog);
	if CHECK_FAIL(prog_fd < 0) {
		goto_close_prog_lsm_tailcall(skel);
		return;
	}

	err = bpf_map_update_elem(
		map_fd,
		&key as *const c_int as *const c_void,
		&prog_fd as *const c_int as *const c_void,
		BPF_ANY,
	);
	if CHECK_FAIL(err != 0) {
		goto_close_prog_lsm_tailcall(skel);
		return;
	}

	goto_close_prog_lsm_tailcall(skel);
}

unsafe fn goto_close_prog_lsm_tailcall(skel: *mut lsm_tailcall) {
	lsm_tailcall__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_test_lsm() {
	if test__start_subtest(b"lsm_basic\0".as_ptr() as *const c_char) {
		test_lsm_basic();
	}
	if test__start_subtest(b"lsm_tailcall\0".as_ptr() as *const c_char) {
		test_lsm_tailcall();
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
