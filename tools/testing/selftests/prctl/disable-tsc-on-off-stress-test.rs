// SPDX-License-Identifier: GPL-2.0
/*
 * Tests for prctl(PR_GET_TSC, ...) / prctl(PR_SET_TSC, ...)
 *
 * Tests if the control register is updated correctly
 * when set with prctl()
 *
 * Warning: this test will cause a very high load for a few seconds
 *
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut stderr: *mut FILE;

	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn perror(s: *const c_char);
	fn exit(status: c_int) -> !;
	fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
	fn alarm(seconds: c_uint) -> c_uint;
	fn fork() -> c_int;
	fn wait(wstatus: *mut c_int) -> c_int;
	fn prctl(option: c_int, ...) -> c_int;
}

/* Get/set the process' ability to use the timestamp counter instruction */
const PR_SET_TSC: c_int = 26;
const PR_TSC_ENABLE: c_ulong = 1; /* allow the use of the timestamp counter */
const PR_TSC_SIGSEGV: c_ulong = 2; /* throw a SIGSEGV instead of reading the TSC */

const SIGSEGV: c_int = 11;

/* snippet from wikipedia :-) */

unsafe fn rdtsc() -> u64
{
	let lo: u32;
	let hi: u32;
	/* We cannot use "=A", since this would use %rax on x86_64 */
	unsafe {
		asm!("rdtsc", out("eax") lo, out("edx") hi, options(nomem, nostack, preserves_flags));
	}
	((hi as u64) << 32) | lo as u64
}

static mut should_segv: c_int = 0;

extern "C" fn sigsegv_cb(_sig: c_int)
{
	unsafe {
		if should_segv == 0
		{
			fprintf(stderr, c"FATAL ERROR, rdtsc() failed while enabled\n".as_ptr());
			exit(0);
		}
		if prctl(PR_SET_TSC, PR_TSC_ENABLE) < 0
		{
			perror(c"prctl".as_ptr());
			exit(0);
		}
		should_segv = 0;

		rdtsc();
	}
}

unsafe fn task()
{
	unsafe {
		signal(SIGSEGV, sigsegv_cb);
		alarm(10);
		loop
		{
			rdtsc();
			if should_segv != 0
			{
				fprintf(stderr, c"FATAL ERROR, rdtsc() succeeded while disabled\n".as_ptr());
				exit(0);
			}
			if prctl(PR_SET_TSC, PR_TSC_SIGSEGV) < 0
			{
				perror(c"prctl".as_ptr());
				exit(0);
			}
			should_segv = 1;
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> c_int
{
	unsafe {
		let n_tasks: c_int = 100;
		let mut i: c_int;

		fprintf(stderr, c"[No further output means we're all right]\n".as_ptr());

		i = 0;
		while i < n_tasks {
			if fork() == 0 {
				task();
			}
			i += 1;
		}

		i = 0;
		while i < n_tasks {
			wait(core::ptr::null_mut::<c_int>());
			i += 1;
		}

		exit(0);
	}
}
