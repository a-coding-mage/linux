// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Data Stream Control Register (DSCR) explicit test
 *
 * This test modifies the DSCR value using mtspr instruction and
 * verifies the change with mfspr instruction. It uses both the
 * privilege state SPR and the problem state SPR for this purpose.
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

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

#[allow(non_camel_case_types)]
type pthread_t = c_ulong;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct sem_t {
	_private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct pthread_barrier_t {
	_private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct pthread_attr_t {
	_private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct pthread_barrierattr_t {
	_private: [u8; 0],
}

// Constants and helper routines are supplied by dscr.h, utils.h, pthread.h,
// sched.h, and semaphore.h in the original C source.
unsafe extern "C" {
	static COUNT: c_int;
	static THREADS: c_int;
	static DSCR_MAX: c_ulong;
	static PPC_FEATURE2_DSCR: c_ulong;
	static BIND_CPU_ANY: c_int;
	static PTHREAD_BARRIER_SERIAL_THREAD: c_int;

	fn set_dscr(val: c_ulong);
	fn get_dscr() -> c_ulong;
	fn get_dscr_usr() -> c_ulong;
	fn set_dscr_usr(val: c_ulong);
	fn get_default_dscr() -> c_ulong;
	fn set_default_dscr(val: c_ulong);
	fn have_hwcap2(feature: c_ulong) -> c_int;
	fn bind_to_cpu(cpu: c_int) -> c_int;
	fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

	fn srand(seed: c_uint);
	fn rand() -> c_int;
	fn gettid() -> c_int;
	fn sched_yield() -> c_int;

	fn sem_wait(sem: *mut sem_t) -> c_int;
	fn sem_post(sem: *mut sem_t) -> c_int;
	fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
	fn sem_destroy(sem: *mut sem_t) -> c_int;

	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const pthread_attr_t,
		start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
	fn pthread_barrier_init(
		barrier: *mut pthread_barrier_t,
		attr: *const pthread_barrierattr_t,
		count: c_uint,
	) -> c_int;
	fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
	fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
}

macro_rules! FAIL_IF {
	($cond:expr) => {
		if $cond {
			return 1;
		}
	};
}

macro_rules! FAIL_IF_EXIT {
	($cond:expr) => {
		if $cond {
			return ptr::null_mut();
		}
	};
}

macro_rules! SKIP_IF {
	($cond:expr) => {
		if $cond {
			return 0;
		}
	};
}

unsafe extern "C" fn dscr_explicit_lockstep_thread(args: *mut c_void) -> *mut c_void {
	let prev = args as *mut sem_t;
	let next = (args as *mut sem_t).add(1);
	let mut expected_dscr: c_ulong = 0;
	let mut i: c_int;

	set_dscr(expected_dscr);
	srand(gettid() as c_uint);

	i = 0;
	while i < COUNT {
		FAIL_IF_EXIT!(sem_wait(prev) != 0);

		FAIL_IF_EXIT!(expected_dscr != get_dscr());
		FAIL_IF_EXIT!(expected_dscr != get_dscr_usr());

		expected_dscr = expected_dscr.wrapping_add(1) % DSCR_MAX;
		set_dscr(expected_dscr);

		FAIL_IF_EXIT!(sem_post(next) != 0);
		i += 1;
	}

	ptr::null_mut()
}

unsafe extern "C" fn dscr_explicit_lockstep_test() -> c_int {
	let mut thread: pthread_t = 0;
	let mut semaphores: [MaybeUninit<sem_t>; 2] = MaybeUninit::uninit().assume_init();
	let prev = semaphores.as_mut_ptr().add(1) as *mut sem_t; /* reversed prev/next than for the other thread */
	let next = semaphores.as_mut_ptr().add(0) as *mut sem_t;
	let mut expected_dscr: c_ulong = 0;
	let mut i: c_int;

	SKIP_IF!(have_hwcap2(PPC_FEATURE2_DSCR) == 0);

	srand(gettid() as c_uint);
	set_dscr(expected_dscr);

	FAIL_IF!(sem_init(prev, 0, 0) != 0);
	FAIL_IF!(sem_init(next, 0, 1) != 0); /* other thread starts first */
	FAIL_IF!(bind_to_cpu(BIND_CPU_ANY) < 0);
	FAIL_IF!(pthread_create(
		&mut thread,
		ptr::null(),
		dscr_explicit_lockstep_thread,
		semaphores.as_mut_ptr() as *mut c_void,
	) != 0);

	i = 0;
	while i < COUNT {
		FAIL_IF!(sem_wait(prev) != 0);

		FAIL_IF!(expected_dscr != get_dscr());
		FAIL_IF!(expected_dscr != get_dscr_usr());

		expected_dscr = expected_dscr.wrapping_sub(1) % DSCR_MAX;
		set_dscr(expected_dscr);

		FAIL_IF!(sem_post(next) != 0);
		i += 1;
	}

	FAIL_IF!(pthread_join(thread, ptr::null_mut()) != 0);
	FAIL_IF!(sem_destroy(prev) != 0);
	FAIL_IF!(sem_destroy(next) != 0);

	0
}

#[repr(C)]
struct random_thread_args {
	thread_id: pthread_t,
	do_yields: bool,
	barrier: *mut pthread_barrier_t,
}

unsafe extern "C" fn dscr_explicit_random_thread(in_: *mut c_void) -> *mut c_void {
	let args = in_ as *mut random_thread_args;
	let mut expected_dscr: c_ulong;
	let mut err: c_int;
	let mut i: c_int;

	srand(gettid() as c_uint);

	err = pthread_barrier_wait((*args).barrier);
	FAIL_IF_EXIT!(err != 0 && err != PTHREAD_BARRIER_SERIAL_THREAD);

	i = 0;
	while i < COUNT {
		expected_dscr = (rand() as c_ulong) % DSCR_MAX;
		set_dscr(expected_dscr);

		let mut j: c_int = rand() % 5;
		while j > 0 {
			FAIL_IF_EXIT!(get_dscr() != expected_dscr);
			FAIL_IF_EXIT!(get_dscr_usr() != expected_dscr);

			if (*args).do_yields && rand() % 2 != 0 {
				sched_yield();
			}
			j -= 1;
		}

		expected_dscr = (rand() as c_ulong) % DSCR_MAX;
		set_dscr_usr(expected_dscr);

		let mut j: c_int = rand() % 5;
		while j > 0 {
			FAIL_IF_EXIT!(get_dscr() != expected_dscr);
			FAIL_IF_EXIT!(get_dscr_usr() != expected_dscr);

			if (*args).do_yields && rand() % 2 != 0 {
				sched_yield();
			}
			j -= 1;
		}

		i += 1;
	}

	ptr::null_mut()
}

unsafe extern "C" fn dscr_explicit_random_test() -> c_int {
	let mut threads: Vec<random_thread_args> = Vec::with_capacity(THREADS as usize);
	let mut barrier = MaybeUninit::<pthread_barrier_t>::uninit();
	let mut i: c_int;

	SKIP_IF!(have_hwcap2(PPC_FEATURE2_DSCR) == 0);

	FAIL_IF!(pthread_barrier_init(barrier.as_mut_ptr(), ptr::null(), THREADS as c_uint) != 0);
	let barrier = barrier.as_mut_ptr();

	i = 0;
	while i < THREADS {
		threads.push(random_thread_args {
			thread_id: 0,
			do_yields: i % 2 == 0,
			barrier,
		});

		let thread_args = threads.as_mut_ptr().add(i as usize);
		FAIL_IF!(pthread_create(
			&mut (*thread_args).thread_id,
			ptr::null(),
			dscr_explicit_random_thread,
			thread_args as *mut c_void,
		) != 0);

		i += 1;
	}

	i = 0;
	while i < THREADS {
		FAIL_IF!(pthread_join((*threads.as_ptr().add(i as usize)).thread_id, ptr::null_mut()) != 0);
		i += 1;
	}

	FAIL_IF!(pthread_barrier_destroy(barrier) != 0);

	0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
	let mut orig_dscr_default: c_ulong = 0;
	let mut err: c_int = 0;

	if have_hwcap2(PPC_FEATURE2_DSCR) != 0 {
		orig_dscr_default = get_default_dscr();
	}

	err |= test_harness(dscr_explicit_lockstep_test, c"dscr_explicit_lockstep_test".as_ptr());
	err |= test_harness(dscr_explicit_random_test, c"dscr_explicit_random_test".as_ptr());

	if have_hwcap2(PPC_FEATURE2_DSCR) != 0 {
		set_default_dscr(orig_dscr_default);
	}

	err
}
