// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2026 Valve Corporation.
 * Author: Changwoo Min <changwoo@igalia.com>
 */

// C dependencies:
// #include <test_progs.h>
// #include <sys/syscall.h>
// #include "test_ctx.skel.h"

use core::ffi::{c_char, c_int, c_uint};
use core::mem::{size_of, MaybeUninit};

#[repr(C)]
pub struct bpf_test_run_opts {
	_private: [u8; 0],
}

#[repr(C)]
pub struct cpu_set_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_ctx {
	pub progs: test_ctx__progs,
	pub bss: *mut test_ctx__bss,
}

#[repr(C)]
pub struct test_ctx__progs {
	pub trigger_all_contexts: *mut bpf_program,
}

#[repr(C)]
pub struct test_ctx__bss {
	pub count_task: c_int,
	pub count_hardirq: c_int,
	pub count_softirq: c_int,
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn sched_getaffinity(pid: c_int, cpusetsize: usize, mask: *mut cpu_set_t) -> c_int;
	fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
	fn usleep(usec: c_uint) -> c_int;

	fn CPU_ZERO(cpuset: *mut cpu_set_t);
	fn CPU_SET(cpu: c_int, cpuset: *mut cpu_set_t);

	fn test_ctx__open_and_load() -> *mut test_ctx;
	fn test_ctx__attach(skel: *mut test_ctx) -> c_int;
	fn test_ctx__destroy(skel: *mut test_ctx);

	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

	fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const test_ctx, name: *const c_char) -> bool;
	fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_exe_ctx() {
	let mut opts = MaybeUninit::<bpf_test_run_opts>::zeroed().assume_init();
	let mut old_cpuset = MaybeUninit::<cpu_set_t>::uninit();
	let mut target_cpuset = MaybeUninit::<cpu_set_t>::uninit();
	let mut skel: *mut test_ctx;
	let mut err: c_int;
	let prog_fd: c_int;

	/* 1. Pin the current process to CPU 0. */
	if sched_getaffinity(0, size_of::<cpu_set_t>(), old_cpuset.as_mut_ptr()) == 0 {
		CPU_ZERO(target_cpuset.as_mut_ptr());
		CPU_SET(0, target_cpuset.as_mut_ptr());
		ASSERT_OK(
			sched_setaffinity(0, size_of::<cpu_set_t>(), target_cpuset.as_ptr()),
			b"setaffinity\0".as_ptr() as *const c_char,
		);
	}

	skel = test_ctx__open_and_load();
	if !ASSERT_OK_PTR(skel, b"skel_load\0".as_ptr() as *const c_char) {
		goto_restore_affinity(old_cpuset.as_ptr());
		return;
	}

	err = test_ctx__attach(skel);
	if !ASSERT_OK(err, b"skel_attach\0".as_ptr() as *const c_char) {
		goto_cleanup(skel, old_cpuset.as_ptr());
		return;
	}

	/* 2. When we run this, the kernel will execute the BPF prog on CPU 0. */
	prog_fd = bpf_program__fd((*skel).progs.trigger_all_contexts);
	err = bpf_prog_test_run_opts(prog_fd, &mut opts);
	ASSERT_OK(err, b"test_run_trigger\0".as_ptr() as *const c_char);

	/* 3. Wait for the local CPU's softirq/tasklet to finish. */
	for _i in 0..1000 {
		if (*(*skel).bss).count_task > 0
			&& (*(*skel).bss).count_hardirq > 0
			&& (*(*skel).bss).count_softirq > 0
		{
			break;
		}
		usleep(1000); /* Wait 1ms per iteration, up to 1 sec total */
	}

	/* On CPU 0, these should now all be non-zero. */
	ASSERT_GT((*(*skel).bss).count_task, 0, b"task_ok\0".as_ptr() as *const c_char);
	ASSERT_GT(
		(*(*skel).bss).count_hardirq,
		0,
		b"hardirq_ok\0".as_ptr() as *const c_char,
	);
	ASSERT_GT(
		(*(*skel).bss).count_softirq,
		0,
		b"softirq_ok\0".as_ptr() as *const c_char,
	);

	goto_cleanup(skel, old_cpuset.as_ptr());
}

unsafe fn goto_cleanup(skel: *mut test_ctx, old_cpuset: *const cpu_set_t) {
	test_ctx__destroy(skel);
	goto_restore_affinity(old_cpuset);
}

unsafe fn goto_restore_affinity(old_cpuset: *const cpu_set_t) {
	ASSERT_OK(
		sched_setaffinity(0, size_of::<cpu_set_t>(), old_cpuset),
		b"restore_affinity\0".as_ptr() as *const c_char,
	);
}
