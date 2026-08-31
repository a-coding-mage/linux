// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Red Hat, Inc. */

// C dependencies translated as external declarations:
// <test_progs.h>
// "fentry_recursive.skel.h"
// "fentry_recursive_target.skel.h"
// <bpf/btf.h>
// "bpf/libbpf_internal.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct fentry_recursive__progs {
	pub recursive_attach: *mut bpf_program,
}

#[repr(C)]
pub struct fentry_recursive {
	pub progs: fentry_recursive__progs,
}

#[repr(C)]
pub struct fentry_recursive_target__progs {
	pub test1: *mut bpf_program,
	pub fentry_target: *mut bpf_program,
}

#[repr(C)]
pub struct fentry_recursive_target {
	pub progs: fentry_recursive_target__progs,
}

#[repr(C)]
pub struct bpf_test_run_opts {
	pub sz: usize,
}

pub type pthread_t = usize;

pub const BPF_TRACE_FENTRY: c_int = 0;
pub const __ATOMIC_SEQ_CST: c_int = 5;

unsafe extern "C" {
	fn fentry_recursive_target__open_and_load() -> *mut fentry_recursive_target;
	fn fentry_recursive_target__destroy(obj: *mut fentry_recursive_target);
	fn fentry_recursive__open() -> *mut fentry_recursive;
	fn fentry_recursive__load(obj: *mut fentry_recursive) -> c_int;
	fn fentry_recursive__attach(obj: *mut fentry_recursive) -> c_int;
	fn fentry_recursive__detach(obj: *mut fentry_recursive);
	fn fentry_recursive__destroy(obj: *mut fentry_recursive);

	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_program__set_attach_target(
		prog: *mut bpf_program,
		attach_prog_fd: c_int,
		attach_func_name: *const c_char,
	) -> c_int;
	fn bpf_link_create(
		prog_fd: c_int,
		target_fd: c_int,
		attach_type: c_int,
		opts: *const c_void,
	) -> c_int;
	fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

	fn test__start_subtest(name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool;

	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const c_void,
		start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
}

unsafe fn atomic_load_n_int(ptr: *mut c_int, _memorder: c_int) -> c_int {
	core::ptr::read_volatile(ptr)
}

unsafe fn atomic_store_n_int(ptr: *mut c_int, val: c_int, _memorder: c_int) {
	core::ptr::write_volatile(ptr, val);
}

/* Test recursive attachment of tracing progs with more than one nesting level
 * is not possible. Create a chain of attachment, verify that the last prog
 * will fail. Depending on the arguments, following cases are tested:
 *
 * - Recursive loading of tracing progs, without attaching (attach = false,
 *   detach = false). The chain looks like this:
 *       load target
 *       load fentry1 -> target
 *       load fentry2 -> fentry1 (fail)
 *
 * - Recursive attach of tracing progs (attach = true, detach = false). The
 *   chain looks like this:
 *       load target
 *       load fentry1 -> target
 *       attach fentry1 -> target
 *       load fentry2 -> fentry1 (fail)
 *
 * - Recursive attach and detach of tracing progs (attach = true, detach =
 *   true). This validates that attach_tracing_prog flag will be set throughout
 *   the whole lifecycle of an fentry prog, independently from whether it's
 *   detached. The chain looks like this:
 *       load target
 *       load fentry1 -> target
 *       attach fentry1 -> target
 *       detach fentry1
 *       load fentry2 -> fentry1 (fail)
 */
unsafe fn test_recursive_fentry_chain(attach: bool, detach: bool) {
	let mut target_skel: *mut fentry_recursive_target = ptr::null_mut();
	let mut tracing_chain: [*mut fentry_recursive; 2] = [ptr::null_mut(); 2];
	let mut prog: *mut bpf_program;
	let mut prev_fd: c_int;
	let mut err: c_int;

	target_skel = fentry_recursive_target__open_and_load();
	if !ASSERT_OK_PTR(
		target_skel as *const c_void,
		c"fentry_recursive_target__open_and_load".as_ptr(),
	) {
		return;
	}

	/* Create an attachment chain with two fentry progs */
	for i in 0..2 {
		tracing_chain[i] = fentry_recursive__open();
		if !ASSERT_OK_PTR(
			tracing_chain[i] as *const c_void,
			c"fentry_recursive__open".as_ptr(),
		) {
			break;
		}

		/* The first prog in the chain is going to be attached to the target
		 * fentry program, the second one to the previous in the chain.
		 */
		prog = (*tracing_chain[i]).progs.recursive_attach;
		if i == 0 {
			prev_fd = bpf_program__fd((*target_skel).progs.test1);
			err = bpf_program__set_attach_target(prog, prev_fd, c"test1".as_ptr());
		} else {
			prev_fd = bpf_program__fd((*tracing_chain[i - 1]).progs.recursive_attach);
			err = bpf_program__set_attach_target(prog, prev_fd, c"recursive_attach".as_ptr());
		}

		if !ASSERT_OK(err, c"bpf_program__set_attach_target".as_ptr()) {
			break;
		}

		err = fentry_recursive__load(tracing_chain[i]);
		/* The first attach should succeed, the second fail */
		if i == 0 {
			if !ASSERT_OK(err, c"fentry_recursive__load".as_ptr()) {
				break;
			}

			if attach {
				err = fentry_recursive__attach(tracing_chain[i]);
				if !ASSERT_OK(err, c"fentry_recursive__attach".as_ptr()) {
					break;
				}
			}

			if detach {
				/* Flag attach_tracing_prog should still be set, preventing
				 * attachment of the following prog.
				 */
				fentry_recursive__detach(tracing_chain[i]);
			}
		} else if !ASSERT_ERR(err, c"fentry_recursive__load".as_ptr()) {
			break;
		}
	}

	fentry_recursive_target__destroy(target_skel);
	for i in 0..2 {
		fentry_recursive__destroy(tracing_chain[i]);
	}
}

#[no_mangle]
pub unsafe extern "C" fn test_recursive_fentry() {
	if test__start_subtest(c"attach".as_ptr()) {
		test_recursive_fentry_chain(true, false);
	}
	if test__start_subtest(c"load".as_ptr()) {
		test_recursive_fentry_chain(false, false);
	}
	if test__start_subtest(c"detach".as_ptr()) {
		test_recursive_fentry_chain(true, true);
	}
}

/* Test that a tracing prog reattachment (when we land in
 * "prog->aux->dst_trampoline and tgt_prog is NULL" branch in
 * bpf_tracing_prog_attach) does not lead to a crash due to missing attach_btf
 */
#[no_mangle]
pub unsafe extern "C" fn test_fentry_attach_btf_presence() {
	let mut target_skel: *mut fentry_recursive_target = ptr::null_mut();
	let mut tracing_skel: *mut fentry_recursive = ptr::null_mut();
	let prog: *mut bpf_program;
	let mut err: c_int;
	let link_fd: c_int;
	let mut tgt_prog_fd: c_int;

	target_skel = fentry_recursive_target__open_and_load();
	if !ASSERT_OK_PTR(
		target_skel as *const c_void,
		c"fentry_recursive_target__open_and_load".as_ptr(),
	) {
		goto_close_prog_attach_btf_presence(target_skel, tracing_skel);
		return;
	}

	tracing_skel = fentry_recursive__open();
	if !ASSERT_OK_PTR(
		tracing_skel as *const c_void,
		c"fentry_recursive__open".as_ptr(),
	) {
		goto_close_prog_attach_btf_presence(target_skel, tracing_skel);
		return;
	}

	prog = (*tracing_skel).progs.recursive_attach;
	tgt_prog_fd = bpf_program__fd((*target_skel).progs.fentry_target);
	err = bpf_program__set_attach_target(prog, tgt_prog_fd, c"fentry_target".as_ptr());
	if !ASSERT_OK(err, c"bpf_program__set_attach_target".as_ptr()) {
		goto_close_prog_attach_btf_presence(target_skel, tracing_skel);
		return;
	}

	err = fentry_recursive__load(tracing_skel);
	if !ASSERT_OK(err, c"fentry_recursive__load".as_ptr()) {
		goto_close_prog_attach_btf_presence(target_skel, tracing_skel);
		return;
	}

	tgt_prog_fd = bpf_program__fd((*tracing_skel).progs.recursive_attach);
	link_fd = bpf_link_create(tgt_prog_fd, 0, BPF_TRACE_FENTRY, ptr::null());
	if !ASSERT_GE(link_fd, 0, c"link_fd".as_ptr()) {
		goto_close_prog_attach_btf_presence(target_skel, tracing_skel);
		return;
	}

	fentry_recursive__detach(tracing_skel);

	err = fentry_recursive__attach(tracing_skel);
	ASSERT_ERR(err, c"fentry_recursive__attach".as_ptr());

	goto_close_prog_attach_btf_presence(target_skel, tracing_skel);
}

unsafe fn goto_close_prog_attach_btf_presence(
	target_skel: *mut fentry_recursive_target,
	tracing_skel: *mut fentry_recursive,
) {
	fentry_recursive_target__destroy(target_skel);
	fentry_recursive__destroy(tracing_skel);
}

unsafe extern "C" fn fentry_target_test_run(arg: *mut c_void) -> *mut c_void {
	loop {
		let prog_fd: c_int = atomic_load_n_int(arg as *mut c_int, __ATOMIC_SEQ_CST);
		let mut topts: bpf_test_run_opts = core::mem::zeroed();
		topts.sz = core::mem::size_of::<bpf_test_run_opts>();
		let err: c_int;

		if prog_fd == -1 {
			break;
		}
		err = bpf_prog_test_run_opts(prog_fd, &mut topts);
		if !ASSERT_OK(err, c"fentry_target test_run".as_ptr()) {
			break;
		}
	}

	ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn test_fentry_attach_stress() {
	let mut target_skel: *mut fentry_recursive_target = ptr::null_mut();
	let mut tracing_skel: *mut fentry_recursive = ptr::null_mut();
	let mut prog: *mut bpf_program;
	let mut err: c_int;
	let mut i: c_int;
	let mut tgt_prog_fd: c_int = 0;
	let mut thread: pthread_t = core::mem::zeroed();

	target_skel = fentry_recursive_target__open_and_load();
	if !ASSERT_OK_PTR(
		target_skel as *const c_void,
		c"fentry_recursive_target__open_and_load".as_ptr(),
	) {
		goto_close_prog_stress(tracing_skel, target_skel);
		return;
	}
	tgt_prog_fd = bpf_program__fd((*target_skel).progs.fentry_target);
	err = pthread_create(
		&mut thread,
		ptr::null(),
		fentry_target_test_run,
		&mut tgt_prog_fd as *mut c_int as *mut c_void,
	);
	if !ASSERT_OK(err, c"bpf_program__set_attach_target".as_ptr()) {
		goto_close_prog_stress(tracing_skel, target_skel);
		return;
	}

	i = 0;
	while i < 1000 {
		tracing_skel = fentry_recursive__open();
		if !ASSERT_OK_PTR(
			tracing_skel as *const c_void,
			c"fentry_recursive__open".as_ptr(),
		) {
			goto_stop_thread(thread, &mut tgt_prog_fd, tracing_skel, target_skel);
			return;
		}

		prog = (*tracing_skel).progs.recursive_attach;
		err = bpf_program__set_attach_target(prog, tgt_prog_fd, c"fentry_target".as_ptr());
		if !ASSERT_OK(err, c"bpf_program__set_attach_target".as_ptr()) {
			goto_stop_thread(thread, &mut tgt_prog_fd, tracing_skel, target_skel);
			return;
		}

		err = fentry_recursive__load(tracing_skel);
		if !ASSERT_OK(err, c"fentry_recursive__load".as_ptr()) {
			goto_stop_thread(thread, &mut tgt_prog_fd, tracing_skel, target_skel);
			return;
		}

		err = fentry_recursive__attach(tracing_skel);
		if !ASSERT_OK(err, c"fentry_recursive__attach".as_ptr()) {
			goto_stop_thread(thread, &mut tgt_prog_fd, tracing_skel, target_skel);
			return;
		}

		fentry_recursive__destroy(tracing_skel);
		tracing_skel = ptr::null_mut();
		i += 1;
	}

	goto_stop_thread(thread, &mut tgt_prog_fd, tracing_skel, target_skel);
}

unsafe fn goto_stop_thread(
	thread: pthread_t,
	tgt_prog_fd: *mut c_int,
	tracing_skel: *mut fentry_recursive,
	target_skel: *mut fentry_recursive_target,
) {
	let err: c_int;

	atomic_store_n_int(tgt_prog_fd, -1, __ATOMIC_SEQ_CST);
	err = pthread_join(thread, ptr::null_mut());
	ASSERT_OK(err, c"pthread_join".as_ptr());
	goto_close_prog_stress(tracing_skel, target_skel);
}

unsafe fn goto_close_prog_stress(
	tracing_skel: *mut fentry_recursive,
	target_skel: *mut fentry_recursive_target,
) {
	fentry_recursive__destroy(tracing_skel);
	fentry_recursive_target__destroy(target_skel);
}
