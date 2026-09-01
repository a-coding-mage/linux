/*
 *  sync test runner
 *  Copyright 2015-2016 Collabora Ltd.
 *
 *  Based on the implementation from the Android Open Source Project,
 *
 *  Copyright 2012 Google, Inc
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a
 *  copy of this software and associated documentation files (the "Software"),
 *  to deal in the Software without restriction, including without limitation
 *  the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
 *  Software is furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in
 *  all copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 *  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 *  OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 *  ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 *  OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::{c_char, c_int, c_void};

type pid_t = c_int;

const ENOENT: c_int = 2;
const EACCES: c_int = 13;

#[repr(C)]
struct stat {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut stdout: *mut c_void;
	static mut errno: c_int;

	fn fflush(stream: *mut c_void) -> c_int;
	fn fork() -> pid_t;
	fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
	fn exit(status: c_int) -> !;
	fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
	fn strerror(errnum: c_int) -> *mut c_char;

	fn ksft_print_header();
	fn ksft_set_plan(cnt: c_int);
	fn ksft_print_msg(fmt: *const c_char, ...);
	fn ksft_test_result_pass(fmt: *const c_char, ...);
	fn ksft_test_result_fail(fmt: *const c_char, ...);
	fn ksft_get_fail_cnt() -> c_int;
	fn ksft_test_num() -> c_int;
	fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
	fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
	fn ksft_exit_pass() -> !;

	fn test_alloc_timeline() -> c_int;
	fn test_alloc_fence() -> c_int;
	fn test_alloc_fence_negative() -> c_int;
	fn test_fence_one_timeline_wait() -> c_int;
	fn test_fence_one_timeline_merge() -> c_int;
	fn test_fence_merge_same_fence() -> c_int;
	fn test_fence_multi_timeline_wait() -> c_int;
	fn test_stress_two_threads_shared_timeline() -> c_int;
	fn test_consumer_stress_multi_producer_single_consumer() -> c_int;
	fn test_merge_stress_random_merge() -> c_int;
}

fn wifexited(status: c_int) -> bool {
	(status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
	(status & 0xff00) >> 8
}

unsafe fn run_test(test: unsafe extern "C" fn() -> c_int, name: *mut c_char) -> c_int {
	let mut result: c_int = 0;
	let childpid: pid_t;
	let ret: c_int;

	fflush(stdout);
	childpid = fork();

	if childpid != 0 {
		waitpid(childpid, &mut result, 0);
		if wifexited(result) {
			ret = wexitstatus(result);
			if ret == 0 {
				ksft_test_result_pass(c"[RUN]\t%s\n".as_ptr(), name);
			} else {
				ksft_test_result_fail(c"[RUN]\t%s\n".as_ptr(), name);
			}
			return ret;
		}
		return 1;
	}

	exit(test());
}

unsafe fn sync_api_supported() {
	let mut sbuf = core::mem::MaybeUninit::<stat>::uninit();
	let ret: c_int;

	ret = stat(
		c"/sys/kernel/debug/sync/sw_sync".as_ptr(),
		sbuf.as_mut_ptr(),
	);
	if ret == 0 {
		return;
	}

	if errno == ENOENT {
		ksft_exit_skip(c"Sync framework not supported by kernel\n".as_ptr());
	}

	if errno == EACCES {
		ksft_exit_skip(c"Run Sync test as root.\n".as_ptr());
	}

	ksft_exit_fail_msg(
		c"stat failed on /sys/kernel/debug/sync/sw_sync: %s".as_ptr(),
		strerror(errno),
	);
}

macro_rules! RUN_TEST {
	($test:ident) => {
		run_test($test, concat!(stringify!($test), "\0").as_ptr() as *mut c_char);
	};
}

fn main() {
	let err: c_int;

	unsafe {
		ksft_print_header();

		sync_api_supported();
		ksft_set_plan(3 + 7);

		ksft_print_msg(c"[RUN]\tTesting sync framework\n".as_ptr());

		RUN_TEST!(test_alloc_timeline);
		RUN_TEST!(test_alloc_fence);
		RUN_TEST!(test_alloc_fence_negative);

		RUN_TEST!(test_fence_one_timeline_wait);
		RUN_TEST!(test_fence_one_timeline_merge);
		RUN_TEST!(test_fence_merge_same_fence);
		RUN_TEST!(test_fence_multi_timeline_wait);
		RUN_TEST!(test_stress_two_threads_shared_timeline);
		RUN_TEST!(test_consumer_stress_multi_producer_single_consumer);
		RUN_TEST!(test_merge_stress_random_merge);

		err = ksft_get_fail_cnt();
		if err != 0 {
			ksft_exit_fail_msg(
				c"%d out of %d sync tests failed\n".as_ptr(),
				err,
				ksft_test_num(),
			);
		}

		ksft_exit_pass();
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
