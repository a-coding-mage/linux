// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2025 Sebastian Andrzej Siewior <bigeasy@linutronix.de>
 */

// C source used _GNU_SOURCE and included:
// errno.h, pthread.h, stdio.h, stdlib.h, string.h, unistd.h,
// linux/futex.h, sys/mman.h, futextest.h, futex2test.h,
// kselftest_harness.h, and conditionally numa.h/numaif.h when
// LIBNUMA_VER_SUFFICIENT is available.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const MAX_THREADS: usize = 64;

static mut barrier_main: pthread_barrier_t = pthread_barrier_t { __private: [] };
static mut threads: [pthread_t; MAX_THREADS] = [0; MAX_THREADS];

#[derive(Copy, Clone)]
#[repr(C)]
struct thread_args {
	futex_ptr: *mut c_void,
	flags: c_uint,
	result: c_int,
}

static mut thread_args: [thread_args; MAX_THREADS] = [thread_args {
	futex_ptr: ptr::null_mut(),
	flags: 0,
	result: 0,
}; MAX_THREADS];

#[cfg(not(defined_FUTEX_NO_NODE))]
const FUTEX_NO_NODE: c_int = -1;

#[cfg(defined_FUTEX_NO_NODE)]
extern "C" {
	static FUTEX_NO_NODE: c_int;
}

#[cfg(not(defined_FUTEX2_MPOL))]
const FUTEX2_MPOL: c_uint = 0x08;

#[cfg(defined_FUTEX2_MPOL)]
extern "C" {
	static FUTEX2_MPOL: c_uint;
}

extern "C" {
	static mut errno: c_int;

	static FUTEX2_SIZE_U32: c_uint;
	static FUTEX_PRIVATE_FLAG: c_uint;
	static FUTEX2_NUMA: c_uint;

	static _SC_PAGE_SIZE: c_int;
	static PROT_READ: c_int;
	static PROT_WRITE: c_int;
	static PROT_NONE: c_int;
	static MAP_PRIVATE: c_int;
	static MAP_ANONYMOUS: c_int;
	static MAP_FAILED: *mut c_void;

	static EINVAL: c_int;
	static EFAULT: c_int;
	static ENOSYS: c_int;

	fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
	fn pthread_barrier_init(
		barrier: *mut pthread_barrier_t,
		attr: *const c_void,
		count: c_uint,
	) -> c_int;
	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const c_void,
		start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

	fn futex2_wait(
		uaddr: *mut c_void,
		val: c_uint,
		flags: c_uint,
		timeout: *const c_void,
		clockid: c_uint,
	) -> c_int;
	fn futex2_wake(uaddr: *mut c_void, nr_wake: c_int, flags: c_uint) -> c_int;

	fn sysconf(name: c_int) -> isize;
	fn mmap(
		addr: *mut c_void,
		length: usize,
		prot: c_int,
		flags: c_int,
		fd: c_int,
		offset: isize,
	) -> *mut c_void;
	fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
	fn munmap(addr: *mut c_void, length: usize) -> c_int;
	fn usleep(usec: c_uint) -> c_int;
	fn strerror(errnum: c_int) -> *mut c_char;

	#[cfg(LIBNUMA_VER_SUFFICIENT)]
	static MPOL_BIND: c_int;
	#[cfg(LIBNUMA_VER_SUFFICIENT)]
	fn mbind(
		start: *mut c_void,
		len: usize,
		mode: c_int,
		nodemask: *const c_ulong,
		maxnode: c_ulong,
		flags: c_uint,
	) -> c_int;
	#[cfg(LIBNUMA_VER_SUFFICIENT)]
	fn numa_set_mempolicy_home_node(
		start: *mut c_void,
		len: usize,
		home_node: c_int,
		flags: c_uint,
	) -> c_int;
}

#[repr(C)]
struct pthread_barrier_t {
	__private: [u8; 0],
}

type pthread_t = c_ulong;

#[repr(C)]
struct __test_metadata {
	_private: [u8; 0],
}

#[repr(C)]
struct futex32_numa {
	futex: u32,
	numa: c_int,
}

extern "C" fn thread_lock_fn(arg: *mut c_void) -> *mut c_void {
	unsafe {
		let args = arg as *mut thread_args;
		let ret: c_int;

		pthread_barrier_wait(&raw mut barrier_main);
		ret = futex2_wait((*args).futex_ptr, 0, (*args).flags, ptr::null(), 0);
		(*args).result = ret;
		ptr::null_mut()
	}
}

unsafe fn create_max_threads(_metadata: *mut __test_metadata, futex_ptr: *mut c_void) {
	let mut i: c_int;
	let mut ret: c_int;

	i = 0;
	while i < MAX_THREADS as c_int {
		thread_args[i as usize].futex_ptr = futex_ptr;
		thread_args[i as usize].flags = FUTEX2_SIZE_U32 | FUTEX_PRIVATE_FLAG | FUTEX2_NUMA;
		thread_args[i as usize].result = 0;
		ret = pthread_create(
			&raw mut threads[i as usize],
			ptr::null(),
			thread_lock_fn,
			&raw mut thread_args[i as usize] as *mut c_void,
		);
		ASSERT_EQ!(ret, 0, {
			TH_LOG!("pthread_create failed");
		});
		i += 1;
	}
}

unsafe fn join_max_threads(_metadata: *mut __test_metadata) {
	let mut i: c_int;
	let mut ret: c_int;

	i = 0;
	while i < MAX_THREADS as c_int {
		ret = pthread_join(threads[i as usize], ptr::null_mut());
		ASSERT_EQ!(ret, 0, {
			TH_LOG!("pthread_join failed for thread %d", i);
		});
		i += 1;
	}
}

unsafe fn __test_futex(
	_metadata: *mut __test_metadata,
	futex_ptr: *mut c_void,
	err_value: c_int,
	futex_flags: c_uint,
) {
	let mut to_wake: c_int;
	let mut ret: c_int;
	let mut i: c_int;

	pthread_barrier_init(&raw mut barrier_main, ptr::null(), (MAX_THREADS + 1) as c_uint);
	create_max_threads(_metadata, futex_ptr);
	pthread_barrier_wait(&raw mut barrier_main);
	to_wake = MAX_THREADS as c_int;

	loop {
		ret = futex2_wake(futex_ptr, to_wake, futex_flags);

		if err_value != 0 {
			EXPECT_LT!(ret, 0, {
				TH_LOG!(
					"futex2_wake(%d, 0x%x) should fail, but didn't",
					to_wake,
					futex_flags
				);
			});

			EXPECT_EQ!(errno, err_value, {
				TH_LOG!(
					"futex2_wake(%d, 0x%x) expected error was %d, but returned %d (%s)",
					to_wake,
					futex_flags,
					err_value,
					errno,
					strerror(errno)
				);
			});

			break;
		}
		if ret < 0 {
			if errno == ENOSYS || (errno == EINVAL && (futex_flags & FUTEX2_NUMA) != 0) {
				SKIP!(return, "futex2 or FUTEX2_NUMA not supported by kernel");
			}

			ASSERT_GE!(ret, 0, {
				TH_LOG!(
					"Failed futex2_wake(%d, 0x%x): %s",
					to_wake,
					futex_flags,
					strerror(errno)
				);
			});
		}
		if ret == 0 {
			usleep(50);
		}
		to_wake -= ret;

		if to_wake == 0 {
			break;
		}
	}
	join_max_threads(_metadata);

	i = 0;
	while i < MAX_THREADS as c_int {
		if err_value != 0 {
			EXPECT_EQ!(thread_args[i as usize].result, -1, {
				TH_LOG!(
					"Thread %d should fail but succeeded (%d)",
					i,
					thread_args[i as usize].result
				);
			});
		} else {
			EXPECT_EQ!(thread_args[i as usize].result, 0, {
				TH_LOG!("Thread %d failed (%d)", i, thread_args[i as usize].result);
			});
		}
		i += 1;
	}
}

unsafe fn test_futex(
	_metadata: *mut __test_metadata,
	futex_ptr: *mut c_void,
	err_value: c_int,
) {
	__test_futex(
		_metadata,
		futex_ptr,
		err_value,
		FUTEX2_SIZE_U32 | FUTEX_PRIVATE_FLAG | FUTEX2_NUMA,
	);
}

TEST!(futex_numa_mpol, {
	unsafe {
		let mut futex_numa: *mut futex32_numa;
		let mut futex_ptr: *mut c_void;
		let mut mem_size: c_int;

		mem_size = sysconf(_SC_PAGE_SIZE) as c_int;
		futex_ptr = mmap(
			ptr::null_mut(),
			(mem_size * 2) as usize,
			PROT_READ | PROT_WRITE,
			MAP_PRIVATE | MAP_ANONYMOUS,
			0,
			0,
		);
		ASSERT_NE!(futex_ptr, MAP_FAILED, {
			TH_LOG!(
				"mmap() for %d bytes failed: %s",
				mem_size,
				strerror(errno)
			);
		});

		/* Create an invalid memory region for the "Memory out of range" test */
		mprotect(
			(futex_ptr as *mut u8).add(mem_size as usize) as *mut c_void,
			mem_size as usize,
			PROT_NONE,
		);

		futex_numa = futex_ptr as *mut futex32_numa;

		TH_LOG!("Regular test");
		(*futex_numa).futex = 0;
		(*futex_numa).numa = FUTEX_NO_NODE;
		test_futex(_metadata, futex_ptr, 0);

		EXPECT_NE!((*futex_numa).numa, FUTEX_NO_NODE, {
			TH_LOG!("NUMA node is left uninitialized");
		});

		/* FUTEX2_NUMA futex must be 8-byte aligned */
		TH_LOG!("Mis-aligned futex");
		test_futex(
			_metadata,
			(futex_ptr as *mut u8).add(mem_size as usize - 4) as *mut c_void,
			EINVAL,
		);

		TH_LOG!("Memory out of range");
		test_futex(
			_metadata,
			(futex_ptr as *mut u8).add(mem_size as usize) as *mut c_void,
			EFAULT,
		);

		(*futex_numa).numa = FUTEX_NO_NODE;
		mprotect(futex_ptr, mem_size as usize, PROT_READ);
		TH_LOG!("Memory, RO");
		test_futex(_metadata, futex_ptr, EFAULT);

		mprotect(futex_ptr, mem_size as usize, PROT_NONE);
		TH_LOG!("Memory, no access");
		test_futex(_metadata, futex_ptr, EFAULT);

		mprotect(futex_ptr, mem_size as usize, PROT_READ | PROT_WRITE);
		TH_LOG!("Memory back to RW");
		test_futex(_metadata, futex_ptr, 0);

		/* MPOL test. Does not work as expected */
		#[cfg(LIBNUMA_VER_SUFFICIENT)]
		{
			let mut i: c_int = 0;
			while i < 4 {
				let mut nodemask: c_ulong;
				let mut ret: c_int;

				nodemask = (1 as c_ulong) << i;
				ret = mbind(
					futex_ptr,
					mem_size as usize,
					MPOL_BIND,
					&nodemask,
					(core::mem::size_of_val(&nodemask) * 8) as c_ulong,
					0,
				);
				if ret == 0 {
					ret = numa_set_mempolicy_home_node(futex_ptr, mem_size as usize, i, 0);
					ASSERT_EQ!(ret, 0, {
						TH_LOG!("Failed to set home node: %s, %d", strerror(errno), errno);
					});

					TH_LOG!("Node %d test", i);
					(*futex_numa).futex = 0;
					(*futex_numa).numa = FUTEX_NO_NODE;

					ret = futex2_wake(
						futex_ptr,
						0,
						FUTEX2_SIZE_U32 | FUTEX_PRIVATE_FLAG | FUTEX2_NUMA | FUTEX2_MPOL,
					);
					EXPECT_GE!(ret, 0, {
						TH_LOG!("Failed to wake 0 with MPOL: %s", strerror(errno));
					});
					EXPECT_EQ!((*futex_numa).numa, i, {
						TH_LOG!(
							"Returned NUMA node is %d expected %d",
							(*futex_numa).numa,
							i
						);
					});
				}
				i += 1;
			}
		}
		#[cfg(not(LIBNUMA_VER_SUFFICIENT))]
		{
			SKIP!(return, "futex2 MPOL hints test requires libnuma 2.0.18+");
		}
		munmap(futex_ptr, (mem_size * 2) as usize);
	}
});

TEST_HARNESS_MAIN!();
