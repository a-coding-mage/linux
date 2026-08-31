// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2019 Facebook */
/* Dependencies from the original C source:
 * <test_progs.h>
 * <linux/bpf.h>
 * "bpf/libbpf_internal.h"
 * "test_raw_tp_test_run.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u64 = u64;

const O_WRONLY: c_int = 1;
const O_TRUNC: c_int = 0o1000;
const BPF_F_TEST_RUN_ON_CPU: c_uint = 1 << 0;
const ENXIO: c_int = 6;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct bpf_test_run_opts {
	pub sz: usize,
	pub ctx_in: *mut c_void,
	pub ctx_size_in: u32,
	pub flags: u32,
	pub cpu: u32,
	pub retval: u32,
}

#[repr(C)]
pub struct test_raw_tp_test_run {
	pub bss: *mut test_raw_tp_test_run__bss,
	pub data: *mut test_raw_tp_test_run__data,
	pub progs: test_raw_tp_test_run__progs,
}

#[repr(C)]
pub struct test_raw_tp_test_run__bss {
	pub count: c_int,
}

#[repr(C)]
pub struct test_raw_tp_test_run__data {
	pub on_cpu: c_uint,
}

#[repr(C)]
pub struct test_raw_tp_test_run__progs {
	pub rename: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut errno: c_int;

	fn parse_cpu_mask_file(
		path: *const c_char,
		mask: *mut *mut bool,
		nr_cpus: *mut c_int,
	) -> c_int;
	fn test_raw_tp_test_run__open_and_load() -> *mut test_raw_tp_test_run;
	fn test_raw_tp_test_run__attach(skel: *mut test_raw_tp_test_run) -> c_int;
	fn test_raw_tp_test_run__destroy(skel: *mut test_raw_tp_test_run);
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

	fn open(path: *const c_char, flags: c_int, ...) -> c_int;
	fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
	fn close(fd: c_int) -> c_int;
	fn free(ptr: *mut c_void);

	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_raw_tp_test_run() {
	let mut comm_fd: c_int = -1;
	let mut err: c_int;
	let mut nr_online: c_int = 0;
	let mut i: c_int;
	let prog_fd: c_int;
	let mut args: [__u64; 2] = [0x1234u64, 0x5678u64];
	let expected_retval: c_int = 0x1234 + 0x5678;
	let mut skel: *mut test_raw_tp_test_run;
	let buf: [c_char; 9] = *b"new_name\0".as_ptr().cast::<[c_char; 9]>();
	let mut online: *mut bool = core::ptr::null_mut();
	let mut opts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		ctx_in: args.as_mut_ptr().cast::<c_void>(),
		ctx_size_in: core::mem::size_of_val(&args) as u32,
		flags: BPF_F_TEST_RUN_ON_CPU,
		cpu: 0,
		retval: 0,
	};

	err = parse_cpu_mask_file(
		c"/sys/devices/system/cpu/online".as_ptr(),
		&mut online,
		&mut nr_online,
	);
	if !ASSERT_OK(err, c"parse_cpu_mask_file".as_ptr()) {
		return;
	}

	skel = test_raw_tp_test_run__open_and_load();
	if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"skel_open".as_ptr()) {
		goto_cleanup(comm_fd, skel, online);
		return;
	}

	err = test_raw_tp_test_run__attach(skel);
	if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
		goto_cleanup(comm_fd, skel, online);
		return;
	}

	comm_fd = open(c"/proc/self/comm".as_ptr(), O_WRONLY | O_TRUNC);
	if !ASSERT_GE(comm_fd, 0, c"open /proc/self/comm".as_ptr()) {
		goto_cleanup(comm_fd, skel, online);
		return;
	}

	err = write(comm_fd, buf.as_ptr().cast::<c_void>(), core::mem::size_of_val(&buf)) as c_int;
	ASSERT_GE(err, 0, c"task rename".as_ptr());

	ASSERT_NEQ((*(*skel).bss).count, 0, c"check_count".as_ptr());
	ASSERT_EQ((*(*skel).data).on_cpu as c_int, 0xffffffffu32 as c_int, c"check_on_cpu".as_ptr());

	prog_fd = bpf_program__fd((*skel).progs.rename);
	opts.ctx_in = args.as_mut_ptr().cast::<c_void>();
	opts.ctx_size_in = core::mem::size_of::<__u64>() as u32;

	err = bpf_prog_test_run_opts(prog_fd, &mut opts);
	ASSERT_NEQ(err, 0, c"test_run should fail for too small ctx".as_ptr());

	opts.ctx_size_in = core::mem::size_of_val(&args) as u32;
	err = bpf_prog_test_run_opts(prog_fd, &mut opts);
	ASSERT_OK(err, c"test_run".as_ptr());
	ASSERT_EQ(opts.retval as c_int, expected_retval, c"check_retval".as_ptr());

	i = 0;
	while i < nr_online {
		if !*online.offset(i as isize) {
			i += 1;
			continue;
		}

		opts.cpu = i as u32;
		opts.retval = 0;
		err = bpf_prog_test_run_opts(prog_fd, &mut opts);
		ASSERT_OK(err, c"test_run_opts".as_ptr());
		ASSERT_EQ((*(*skel).data).on_cpu as c_int, i, c"check_on_cpu".as_ptr());
		ASSERT_EQ(opts.retval as c_int, expected_retval, c"check_retval".as_ptr());

		i += 1;
	}

	/* invalid cpu ID should fail with ENXIO */
	opts.cpu = 0xffffffff;
	err = bpf_prog_test_run_opts(prog_fd, &mut opts);
	ASSERT_EQ(errno, ENXIO, c"test_run_opts should fail with ENXIO".as_ptr());
	ASSERT_ERR(err, c"test_run_opts_fail".as_ptr());

	/* non-zero cpu w/o BPF_F_TEST_RUN_ON_CPU should fail with EINVAL */
	opts.cpu = 1;
	opts.flags = 0;
	err = bpf_prog_test_run_opts(prog_fd, &mut opts);
	ASSERT_EQ(errno, EINVAL, c"test_run_opts should fail with EINVAL".as_ptr());
	ASSERT_ERR(err, c"test_run_opts_fail".as_ptr());

	goto_cleanup(comm_fd, skel, online);
}

unsafe fn goto_cleanup(
	comm_fd: c_int,
	skel: *mut test_raw_tp_test_run,
	online: *mut bool,
) {
	close(comm_fd);
	test_raw_tp_test_run__destroy(skel);
	free(online.cast::<c_void>());
}
