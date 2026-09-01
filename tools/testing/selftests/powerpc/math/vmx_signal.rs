// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 *
 * This test attempts to see if the VMX registers are correctly reported in a
 * signal context. Each worker just spins checking its VMX registers, at some
 * point a signal will interrupt it and C code will check the signal context
 * ensuring it is also the same.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Number of times each thread should receive the signal */
const ITERATIONS: c_int = 10;
/*
 * Factor by which to multiply number of online CPUs for total number of
 * worker threads
 */
const THREAD_FACTOR: c_int = 8;

const SA_SIGINFO: c_int = 4;
const SIGUSR1: c_int = 10;
const _SC_NPROCESSORS_ONLN: c_int = 84;
const PPC_FEATURE2_ARCH_2_07: c_ulong = 0x8000_0000;

type c_ulong = u64;
type pthread_t = c_ulong;
type size_t = usize;
type sighandler_t = Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>;

#[repr(C)]
pub struct siginfo_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
	_private: [u8; 128],
}

#[repr(C)]
pub struct sigaction {
	pub sa_sigaction: sighandler_t,
	pub sa_mask: sigset_t,
	pub sa_flags: c_int,
	pub sa_restorer: *mut c_void,
}

#[repr(C)]
pub struct vmx_regs {
	pub vrregs: [[c_uint; 4]; 34],
}

#[repr(C)]
pub struct mcontext_t {
	pub v_regs: *mut vmx_regs,
}

#[repr(C)]
pub struct ucontext_t {
	pub uc_mcontext: mcontext_t,
}

#[thread_local]
static mut varray: [[c_int; 4]; 12] = [
	[1, 2, 3, 4],
	[5, 6, 7, 8],
	[9, 10, 11, 12],
	[13, 14, 15, 16],
	[17, 18, 19, 20],
	[21, 22, 23, 24],
	[25, 26, 27, 28],
	[29, 30, 31, 32],
	[33, 34, 35, 36],
	[37, 38, 39, 40],
	[41, 42, 43, 44],
	[45, 46, 47, 48],
];

static mut bad_context: bool = false;
static mut running: c_int = 0;
static mut threads_starting: c_int = 0;

unsafe extern "C" {
	fn preempt_vmx(
		varray: *mut [c_int; 4],
		threads_starting: *mut c_int,
		sentinal: *mut c_int,
	) -> c_int;

	fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;
	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
	fn srand(seed: c_uint);
	fn rand() -> c_int;
	fn pthread_self() -> pthread_t;
	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const c_void,
		start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
	fn sysconf(name: c_int) -> c_long;
	fn malloc(size: size_t) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn setbuf(stream: *mut FILE, buf: *mut c_char);
	fn usleep(usec: c_uint) -> c_int;
	fn sleep(seconds: c_uint) -> c_uint;
	fn have_hwcap2(feature: c_ulong) -> bool;
	fn test_harness_set_timeout(seconds: c_int);
	fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

	static mut stdout: *mut FILE;
	static mut stderr: *mut FILE;
}

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

macro_rules! SKIP_IF {
	($cond:expr) => {
		if $cond {
			return 0;
		}
	};
}

macro_rules! FAIL_IF {
	($cond:expr) => {
		if $cond {
			return 1;
		}
	};
}

unsafe extern "C" fn signal_vmx_sig(sig: c_int, info: *mut siginfo_t, context: *mut c_void) {
	let mut i: c_int;
	let uc: *mut ucontext_t = context as *mut ucontext_t;
	let mc: *mut mcontext_t = &mut (*uc).uc_mcontext;

	let _ = sig;
	let _ = info;

	/* Only the non volatiles were loaded up */
	i = 20;
	while i < 32 {
		if memcmp(
			(*(*mc).v_regs).vrregs[i as usize].as_ptr() as *const c_void,
			varray[(i - 20) as usize].as_ptr() as *const c_void,
			16,
		) != 0
		{
			let mut j: c_int;
			/*
			 * Shouldn't printf() in a signal handler, however, this is a
			 * test and we've detected failure. Understanding what failed
			 * is paramount. All that happens after this is tests exit with
			 * failure.
			 */
			printf(b"VMX mismatch at reg %d!\n\0".as_ptr() as *const c_char, i);
			printf(b"Reg | Actual                  | Expected\n\0".as_ptr() as *const c_char);
			j = 20;
			while j < 32 {
				printf(
					b"%d  | 0x%04x%04x%04x%04x      | 0x%04x%04x%04x%04x\n\0".as_ptr()
						as *const c_char,
					j,
					(*(*mc).v_regs).vrregs[j as usize][0],
					(*(*mc).v_regs).vrregs[j as usize][1],
					(*(*mc).v_regs).vrregs[j as usize][2],
					(*(*mc).v_regs).vrregs[j as usize][3],
					varray[(j - 20) as usize][0],
					varray[(j - 20) as usize][1],
					varray[(j - 20) as usize][2],
					varray[(j - 20) as usize][3],
				);
				j += 1;
			}
			bad_context = true;
			break;
		}
		i += 1;
	}
}

unsafe extern "C" fn signal_vmx_c(p: *mut c_void) -> *mut c_void {
	let mut i: c_int;
	let mut j: c_int;
	let mut rc: c_long;
	let mut act: sigaction = core::mem::zeroed();
	act.sa_sigaction = Some(signal_vmx_sig);
	act.sa_flags = SA_SIGINFO;
	rc = sigaction(SIGUSR1, &act, ptr::null_mut()) as c_long;
	if rc != 0 {
		return p;
	}

	srand(pthread_self() as c_uint);
	i = 0;
	while i < 12 {
		j = 0;
		while j < 4 {
			varray[i as usize][j as usize] = rand();
			j += 1;
		}
		i += 1;
	}

	rc = preempt_vmx(varray.as_mut_ptr(), &mut threads_starting, &mut running) as c_long;

	rc as *mut c_void
}

unsafe extern "C" fn test_signal_vmx() -> c_int {
	let mut i: c_int;
	let mut j: c_int;
	let mut rc: c_int;
	let threads: c_int;
	let mut rc_p: *mut c_void = ptr::null_mut();
	let tids: *mut pthread_t;

	// vcmpequd used in vmx_asm.S is v2.07
	SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));

	threads = (sysconf(_SC_NPROCESSORS_ONLN) as c_int) * THREAD_FACTOR;
	tids = malloc((threads as usize) * size_of::<pthread_t>()) as *mut pthread_t;
	FAIL_IF!(tids.is_null());

	running = true as c_int;
	threads_starting = threads;
	i = 0;
	while i < threads {
		rc = pthread_create(
			tids.add(i as usize),
			ptr::null(),
			signal_vmx_c,
			ptr::null_mut(),
		);
		FAIL_IF!(rc != 0);
		i += 1;
	}

	setbuf(stdout, ptr::null_mut());
	printf(
		b"\tWaiting for %d workers to start... %d\0".as_ptr() as *const c_char,
		threads,
		threads_starting,
	);
	while threads_starting != 0 {
		asm!("", options(nostack, preserves_flags));
		usleep(1000);
		printf(b", %d\0".as_ptr() as *const c_char, threads_starting);
	}
	printf(b" ...done\n\0".as_ptr() as *const c_char);

	printf(
		b"\tSending signals to all threads %d times...\0".as_ptr() as *const c_char,
		ITERATIONS,
	);
	i = 0;
	while i < ITERATIONS {
		j = 0;
		while j < threads {
			pthread_kill(*tids.add(j as usize), SIGUSR1);
			j += 1;
		}
		sleep(1);
		i += 1;
	}
	printf(b"done\n\0".as_ptr() as *const c_char);

	printf(b"\tKilling workers...\0".as_ptr() as *const c_char);
	running = 0;
	i = 0;
	while i < threads {
		pthread_join(*tids.add(i as usize), &mut rc_p);

		/*
		 * Harness will say the fail was here, look at why signal_vmx
		 * returned
		 */
		if (rc_p as c_long) != 0 || bad_context {
			printf(b"oops\n\0".as_ptr() as *const c_char);
		}
		if bad_context {
			fprintf(stderr, b"\t!! bad_context is true\n\0".as_ptr() as *const c_char);
		}
		FAIL_IF!((rc_p as c_long) != 0 || bad_context);
		i += 1;
	}
	printf(b"done\n\0".as_ptr() as *const c_char);

	free(tids as *mut c_void);
	0
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let _ = argc;
	let _ = argv;
	test_harness_set_timeout(360);
	test_harness(test_signal_vmx, b"vmx_signal\0".as_ptr() as *const c_char)
}

fn main() {
	unsafe {
		let code = main_impl(0, ptr::null_mut());
		if code != 0 {
			std::process::exit(code);
		}
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
