// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
/* Dependencies from the original C file:
 * #include <test_progs.h>
 * #include "tracing_failure.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of_val;

#[repr(C)]
pub struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct tracing_failure__progs {
	pub test_spin_lock: *mut bpf_program,
	pub test_spin_unlock: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_failure {
	pub obj: *mut bpf_object,
	pub progs: tracing_failure__progs,
}

const BPF_TRACE_FENTRY: c_int = 38;

unsafe extern "C" {
	fn tracing_failure__open() -> *mut tracing_failure;
	fn tracing_failure__load(skel: *mut tracing_failure) -> c_int;
	fn tracing_failure__attach(skel: *mut tracing_failure) -> c_int;
	fn tracing_failure__destroy(skel: *mut tracing_failure);

	fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
	fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_size: usize);
	fn bpf_object__find_program_by_name(
		obj: *mut bpf_object,
		name: *const c_char,
	) -> *mut bpf_program;
	fn libbpf_find_vmlinux_btf_id(name: *const c_char, attach_type: c_int) -> c_int;

	fn test__skip();
	fn test__start_subtest(name: *const c_char) -> bool;

	fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_HAS_SUBSTR(buf: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
}

unsafe fn test_bpf_spin_lock(is_spin_lock: bool) {
	let skel: *mut tracing_failure;
	let mut err: c_int;

	skel = tracing_failure__open();
	if !ASSERT_OK_PTR(skel as *mut c_void, c"tracing_failure__open".as_ptr()) {
		return;
	}

	if is_spin_lock {
		bpf_program__set_autoload((*skel).progs.test_spin_lock, true);
	} else {
		bpf_program__set_autoload((*skel).progs.test_spin_unlock, true);
	}

	err = tracing_failure__load(skel);
	if !ASSERT_OK(err, c"tracing_failure__load".as_ptr()) {
		tracing_failure__destroy(skel);
		return;
	}

	err = tracing_failure__attach(skel);
	ASSERT_ERR(err, c"tracing_failure__attach".as_ptr());

	tracing_failure__destroy(skel);
}

unsafe fn test_tracing_fail_prog(prog_name: *const c_char, exp_msg: *const c_char) {
	let skel: *mut tracing_failure;
	let prog: *mut bpf_program;
	let mut log_buf: [c_char; 256] = [0; 256];
	let mut err: c_int;

	skel = tracing_failure__open();
	if !ASSERT_OK_PTR(skel as *mut c_void, c"tracing_failure__open".as_ptr()) {
		return;
	}

	prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
	if !ASSERT_OK_PTR(prog as *mut c_void, c"bpf_object__find_program_by_name".as_ptr()) {
		tracing_failure__destroy(skel);
		return;
	}

	bpf_program__set_autoload(prog, true);
	bpf_program__set_log_buf(prog, log_buf.as_mut_ptr(), size_of_val(&log_buf));

	err = tracing_failure__load(skel);
	if !ASSERT_ERR(err, c"tracing_failure__load".as_ptr()) {
		tracing_failure__destroy(skel);
		return;
	}

	ASSERT_HAS_SUBSTR(log_buf.as_ptr(), exp_msg, c"log_buf".as_ptr());
	tracing_failure__destroy(skel);
}

unsafe fn test_tracing_deny() {
	let btf_id: c_int;

	/* __rcu_read_lock depends on CONFIG_PREEMPT_RCU */
	btf_id = libbpf_find_vmlinux_btf_id(c"__rcu_read_lock".as_ptr(), BPF_TRACE_FENTRY);
	if btf_id <= 0 {
		test__skip();
		return;
	}

	test_tracing_fail_prog(
		c"tracing_deny".as_ptr(),
		c"Attaching tracing programs to function '__rcu_read_lock' is rejected.".as_ptr(),
	);
}

unsafe fn test_fexit_noreturns() {
	test_tracing_fail_prog(
		c"fexit_noreturns".as_ptr(),
		c"Attaching fexit/fsession/fmod_ret to __noreturn function 'do_exit' is rejected.".as_ptr(),
	);
}

unsafe fn test_fexit_int128_ret() {
	/*
	 * __int128 is returned in a register pair on x86_64 and arm64, so
	 * bpf_testmod_test_int128_ret() is BTF-encoded and attachable and the
	 * verifier can reject its >8 byte return value. Other architectures
	 * return a __int128 differently (e.g. s390x returns larger values by
	 * reference, which makes pahole skip BTF encoding of the function), so
	 * only exercise this on x86_64 and arm64.
	 */
	#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
	{
		test_tracing_fail_prog(
			c"fexit_int128_ret".as_ptr(),
			c"with a >8 byte return value is not supported for this attach type".as_ptr(),
		);
	}
	#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
	{
		test__skip();
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_tracing_failure() {
	if test__start_subtest(c"bpf_spin_lock".as_ptr()) {
		test_bpf_spin_lock(true);
	}
	if test__start_subtest(c"bpf_spin_unlock".as_ptr()) {
		test_bpf_spin_lock(false);
	}
	if test__start_subtest(c"tracing_deny".as_ptr()) {
		test_tracing_deny();
	}
	if test__start_subtest(c"fexit_noreturns".as_ptr()) {
		test_fexit_noreturns();
	}
	if test__start_subtest(c"fexit_int128_ret".as_ptr()) {
		test_fexit_int128_ret();
	}
}
