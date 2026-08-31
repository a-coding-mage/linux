// SPDX-License-Identifier: GPL-2.0-only
/*
 * Corrupt the XSTATE header in a signal frame
 *
 * Based on analysis and a test case from Thomas Gleixner.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::arch::x86_64::__cpuid_count;
use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type pid_t = c_int;

const SIGUSR1: c_int = 10;
const SIGSEGV: c_int = 11;

#[repr(C)]
struct siginfo_t {
	_private: [u8; 0],
}

#[repr(C)]
struct fpregset_t {
	_private: [u8; 0],
}

#[repr(C)]
struct mcontext_t {
	_opaque_prefix: [u64; 22],
	fpregs: *mut fpregset_t,
}

#[repr(C)]
struct ucontext_t {
	_opaque_prefix: [u64; 5],
	uc_mcontext: mcontext_t,
}

#[repr(C)]
struct cpu_set_t {
	__bits: [usize; 1024 / (8 * size_of::<usize>())],
}

unsafe extern "C" {
	fn printf(format: *const c_char, ...) -> c_int;
	fn raise(sig: c_int) -> c_int;
	fn fork() -> pid_t;
	fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
	fn getpid() -> pid_t;
	fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
	fn err(eval: c_int, fmt: *const c_char, ...) -> !;

	/* helpers.h */
	fn sethandler(
		sig: c_int,
		handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
		flags: c_int,
	);
}

fn CPU_ZERO(set: *mut cpu_set_t) {
	unsafe {
		ptr::write_bytes(set as *mut u8, 0, size_of::<cpu_set_t>());
	}
}

fn CPU_SET(cpu: usize, set: *mut cpu_set_t) {
	let bits_per_word = 8 * size_of::<usize>();

	unsafe {
		(*set).__bits[cpu / bits_per_word] |= 1usize << (cpu % bits_per_word);
	}
}

#[inline]
fn xsave_enabled() -> c_int {
	let cpuid = unsafe { __cpuid_count(0x1, 0x0) };

	/* Is CR4.OSXSAVE enabled ? */
	(cpuid.ecx & (1u32 << 27)) as c_int
}

unsafe extern "C" fn sigusr1(_sig: c_int, _info: *mut siginfo_t, uc_void: *mut c_void) {
	let uc = uc_void as *mut ucontext_t;
	let fpstate = unsafe { (*uc).uc_mcontext.fpregs as *mut u8 };
	let xfeatures = unsafe { fpstate.add(512) as *mut u64 };

	unsafe {
		printf(c"\tWreck XSTATE header\n".as_ptr());
	}
	/* Wreck the first reserved bytes in the header */
	unsafe {
		*xfeatures.add(2) = 0xfffffff;
	}
}

unsafe extern "C" fn sigsegv(_sig: c_int, _info: *mut siginfo_t, _uc_void: *mut c_void) {
	unsafe {
		printf(c"\tGot SIGSEGV\n".as_ptr());
	}
}

fn main() -> c_int {
	let mut set: cpu_set_t = unsafe { zeroed() };

	unsafe {
		sethandler(SIGUSR1, sigusr1, 0);
		sethandler(SIGSEGV, sigsegv, 0);
	}

	if xsave_enabled() == 0 {
		unsafe {
			printf(c"[SKIP] CR4.OSXSAVE disabled.\n".as_ptr());
		}
		return 0;
	}

	CPU_ZERO(&mut set);
	CPU_SET(0, &mut set);

	/*
	 * Enforce that the child runs on the same CPU
	 * which in turn forces a schedule.
	 */
	unsafe {
		sched_setaffinity(getpid(), size_of::<cpu_set_t>(), &set);
	}

	unsafe {
		printf(c"[RUN]\tSend ourselves a signal\n".as_ptr());
		raise(SIGUSR1);

		printf(c"[OK]\tBack from the signal.  Now schedule.\n".as_ptr());
	}
	let child = unsafe { fork() };
	if child < 0 {
		unsafe {
			err(1, c"fork".as_ptr());
		}
	}
	if child == 0 {
		return 0;
	}
	if child != 0 {
		unsafe {
			waitpid(child, ptr::null_mut(), 0);
		}
	}
	unsafe {
		printf(c"[OK]\tBack in the main thread.\n".as_ptr());
	}

	/*
	 * We could try to confirm that extended state is still preserved
	 * when we schedule.  For now, the only indication of failure is
	 * a warning in the kernel logs.
	 */

	0
}
