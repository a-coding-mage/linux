// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2015, Laurent Dufour, IBM Corp.
 *
 * Test the kernel's signal returning code to check reclaim is done if the
 * sigreturn() is called while in a transaction (suspended since active is
 * already dropped trough the system call path).
 *
 * The kernel must discard the transaction when entering sigreturn, since
 * restoring the potential TM SPRS from the signal frame is requiring to not be
 * in a transaction.
 */

// C dependencies:
// <signal.h>, <stdio.h>, <stdlib.h>, <string.h>, <sys/types.h>,
// <sys/wait.h>, <unistd.h>, "tm.h", "utils.h"

use core::arch::asm;
use core::ffi::{c_int, c_void};

type uint64_t = u64;

const SIGSEGV: c_int = 11;

#[repr(C)]
pub struct sigset_t {
	__val: [u64; 16],
}

#[repr(C)]
pub struct sigaction {
	sa_handler: extern "C" fn(c_int),
	sa_mask: sigset_t,
	sa_flags: c_int,
	sa_restorer: Option<extern "C" fn()>,
}

extern "C" {
	fn exit(status: c_int) -> !;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn sigemptyset(set: *mut sigset_t) -> c_int;
	fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

	fn have_htm() -> c_int;
	fn htm_is_synthetic() -> c_int;
	fn is_ppc64le() -> c_int;
	fn test_harness(test_function: extern "C" fn() -> c_int, name: *const u8) -> c_int;
}

// External macro from utils.h; expected to be provided by the Rust test harness
// translation environment.
macro_rules! SKIP_IF {
	($cond:expr) => {
		if $cond {
			return 0;
		}
	};
}

extern "C" fn handler(sig: c_int) {
	let mut ret: uint64_t;

	unsafe {
		asm!(
			"li             3,1             ;",
			"tbegin.                        ;",
			"beq            1f              ;",
			"li             3,0             ;",
			"tsuspend.                      ;",
			"1:                             ;",
			"std            3, 0({ret})     ;",
			ret = in(reg) &mut ret,
			out("r3") _,
			out("cr0") _,
			options(nostack, preserves_flags),
		);
	}

	if ret != 0 {
		unsafe {
			exit(1);
		}
	}

	/*
	 * We return from the signal handle while in a suspended transaction
	 */
}

extern "C" fn tm_sigreturn() -> c_int {
	let mut sa: sigaction;
	let mut ret: uint64_t = 0;

	unsafe {
		SKIP_IF!(have_htm() == 0);
		SKIP_IF!(htm_is_synthetic() != 0);
		SKIP_IF!(is_ppc64le() == 0);

		sa = core::mem::zeroed();
		memset(
			&mut sa as *mut sigaction as *mut c_void,
			0,
			core::mem::size_of::<sigaction>(),
		);
		sa.sa_handler = handler;
		sigemptyset(&mut sa.sa_mask);

		if sigaction(SIGSEGV, &sa, core::ptr::null_mut()) != 0 {
			exit(1);
		}

		asm!(
			"tbegin.                        ;",
			"beq            1f              ;",
			"li             3,0             ;",
			"std            3,0(3)          ;", /* trigger SEGV */
			"li             3,1             ;",
			"std            3, 0({ret_ptr}) ;",
			"tend.                          ;",
			"b              2f              ;",
			"1:                             ;",
			"li             3,2             ;",
			"std            3, 0({ret_ptr}) ;",
			"2:                             ;",
			ret_ptr = in(reg) &mut ret,
			out("r3") _,
			out("cr0") _,
			options(nostack, preserves_flags),
		);

		if ret != 2 {
			exit(1);
		}

		exit(0);
	}
}

fn main() -> c_int {
	unsafe { test_harness(tm_sigreturn, b"tm_sigreturn\0".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
