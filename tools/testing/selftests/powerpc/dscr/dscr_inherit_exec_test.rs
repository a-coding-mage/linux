// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Data Stream Control Register (DSCR) fork exec test
 *
 * This testcase modifies the DSCR using mtspr, forks & execs and
 * verifies that the child is using the changed DSCR using mfspr.
 *
 * When using the privilege state SPR, the instructions such as
 * mfspr or mtspr are privileged and the kernel emulates them
 * for us. Instructions using problem state SPR can be executed
 * directly without any emulation if the HW supports them. Else
 * they also get emulated by the kernel.
 *
 * Copyright 2012, Anton Blanchard, IBM Corporation.
 * Copyright 2015, Anshuman Khandual, IBM Corporation.
 */
/* Translated from C. External declarations are supplied by dscr.h/libc. */

use core::ffi::c_void;
use core::ptr;
use std::os::raw::{c_char, c_int, c_ulong};

type pid_t = c_int;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut stderr: *mut FILE;

	static COUNT: c_ulong;
	static DSCR_MAX: c_ulong;
	static PPC_FEATURE2_DSCR: c_ulong;

	fn get_dscr() -> c_ulong;
	fn get_dscr_usr() -> c_ulong;
	fn get_default_dscr() -> c_ulong;
	fn set_dscr(dscr: c_ulong);
	fn set_dscr_usr(dscr: c_ulong);
	fn have_hwcap2(feature: c_ulong) -> c_int;
	fn SKIP_IF(condition: c_int);
	fn test_harness(
		test_function: unsafe extern "C" fn() -> c_int,
		name: *const c_char,
	) -> c_int;

	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn perror(s: *const c_char);
	fn exit(status: c_int) -> !;
	fn fork() -> pid_t;
	fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
	fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
	fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn atoi(nptr: *const c_char) -> c_int;
}

static mut prog: *mut c_char = ptr::null_mut();

fn WIFEXITED(status: c_int) -> bool {
	(status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
	(status & 0xff00) >> 8
}

unsafe fn do_exec(parent_dscr: c_ulong) {
	let cur_dscr: c_ulong;
	let cur_dscr_usr: c_ulong;

	cur_dscr = unsafe { get_dscr() };
	cur_dscr_usr = unsafe { get_dscr_usr() };

	if cur_dscr != parent_dscr {
		unsafe {
			fprintf(
				stderr,
				c"Parent DSCR %ld was not inherited over exec (kernel value)\n".as_ptr(),
				parent_dscr,
			);
			exit(1);
		}
	}

	if cur_dscr_usr != parent_dscr {
		unsafe {
			fprintf(
				stderr,
				c"Parent DSCR %ld was not inherited over exec (user value)\n".as_ptr(),
				parent_dscr,
			);
			exit(1);
		}
	}
	unsafe { exit(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dscr_inherit_exec() -> c_int {
	let mut i: c_ulong;
	let mut dscr: c_ulong = 0;
	let mut pid: pid_t;

	unsafe { SKIP_IF((have_hwcap2(PPC_FEATURE2_DSCR) == 0) as c_int) };

	i = 0;
	while i < unsafe { COUNT } {
		dscr = dscr.wrapping_add(1);
		if dscr > unsafe { DSCR_MAX } {
			dscr = 0;
		}

		if dscr == unsafe { get_default_dscr() } {
			i = i.wrapping_add(1);
			continue;
		}

		if i % 2 == 0 {
			unsafe { set_dscr_usr(dscr) };
		} else {
			unsafe { set_dscr(dscr) };
		}

		pid = unsafe { fork() };
		if pid == -1 {
			unsafe {
				perror(c"fork() failed".as_ptr());
				exit(1);
			}
		} else if pid != 0 {
			let mut status: c_int = 0;

			if unsafe { waitpid(pid, &mut status, 0) } == -1 {
				unsafe {
					perror(c"waitpid() failed".as_ptr());
					exit(1);
				}
			}

			if !WIFEXITED(status) {
				unsafe {
					fprintf(stderr, c"Child didn't exit cleanly\n".as_ptr());
					exit(1);
				}
			}

			if WEXITSTATUS(status) != 0 {
				unsafe {
					fprintf(stderr, c"Child didn't exit cleanly\n".as_ptr());
				}
				return 1;
			}
		} else {
			let mut dscr_str: [c_char; 16] = [0; 16];

			unsafe {
				sprintf(dscr_str.as_mut_ptr(), c"%ld".as_ptr(), dscr);
				execlp(
					prog,
					prog,
					c"exec".as_ptr(),
					dscr_str.as_mut_ptr(),
					ptr::null_mut::<c_void>(),
				);
				exit(1);
			}
		}
		i = i.wrapping_add(1);
	}
	return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	if argc == 3 && unsafe { strcmp(*argv.add(1), c"exec".as_ptr()) } == 0 {
		let parent_dscr: c_ulong;

		parent_dscr = unsafe { atoi(*argv.add(2)) as c_ulong };
		unsafe { do_exec(parent_dscr) };
	} else if argc != 1 {
		unsafe {
			fprintf(stderr, c"Usage: %s\n".as_ptr(), *argv.add(0));
			exit(1);
		}
	}

	unsafe {
		prog = *argv.add(0);
		test_harness(dscr_inherit_exec, c"dscr_inherit_exec_test".as_ptr())
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
