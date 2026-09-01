// SPDX-License-Identifier: GPL-2.0

/* kselftest for acct() system call
 *  The acct() system call enables or disables process accounting.
 */

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

type pid_t = c_int;

const SEEK_END: c_int = 2;

unsafe extern "C" {
	fn ksft_print_header();
	fn ksft_set_plan(plan: c_uint);
	fn ksft_exit_skip(msg: *const c_char, ...);
	fn ksft_test_result_error(msg: *const c_char, ...);
	fn ksft_finished();
	fn ksft_test_result_fail(msg: *const c_char, ...);
	fn ksft_exit_fail();
	fn ksft_test_result_pass(msg: *const c_char, ...);
	fn ksft_exit_pass();

	fn geteuid() -> c_uint;
	fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
	fn strerror(errnum: c_int) -> *mut c_char;
	fn fclose(stream: *mut FILE) -> c_int;
	fn acct(filename: *const c_char) -> c_int;
	fn fork() -> pid_t;
	fn wait(wstatus: *mut c_int) -> pid_t;
	fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
	fn ftell(stream: *mut FILE) -> c_long;
	fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
	*__errno_location()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
	let filename: [c_char; 12] = [
		b'p' as c_char,
		b'r' as c_char,
		b'o' as c_char,
		b'c' as c_char,
		b'e' as c_char,
		b's' as c_char,
		b's' as c_char,
		b'_' as c_char,
		b'l' as c_char,
		b'o' as c_char,
		b'g' as c_char,
		0,
	];
	let fp: *mut FILE;
	let child_pid: pid_t;
	let sz: c_int;

	// Setting up kselftest framework
	ksft_print_header();
	ksft_set_plan(1);

	// Check if test is run a root
	if geteuid() != 0 {
		ksft_exit_skip(c"This test needs root to run!\n".as_ptr());
		return 1;
	}

	// Create file to log closed processes
	fp = fopen(filename.as_ptr(), c"w".as_ptr());

	if fp.is_null() {
		ksft_test_result_error(c"%s.\n".as_ptr(), strerror(errno()));
		ksft_finished();
		return 1;
	}

	acct(filename.as_ptr());

	// Handle error conditions
	if errno() != 0 {
		ksft_test_result_error(c"%s.\n".as_ptr(), strerror(errno()));
		fclose(fp);
		ksft_finished();
		return 1;
	}

	// Create child process and wait for it to terminate.

	child_pid = fork();

	if child_pid < 0 {
		ksft_test_result_error(c"Creating a child process to log failed\n".as_ptr());
		acct(ptr::null());
		return 1;
	} else if child_pid > 0 {
		wait(ptr::null_mut());
		fseek(fp, 0 as c_long, SEEK_END);
		sz = ftell(fp) as c_int;

		acct(ptr::null());

		if sz <= 0 {
			ksft_test_result_fail(c"Terminated child process not logged\n".as_ptr());
			ksft_exit_fail();
			return 1;
		}

		ksft_test_result_pass(c"Successfully logged terminated process.\n".as_ptr());
		fclose(fp);
		ksft_exit_pass();
		return 0;
	}

	1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
