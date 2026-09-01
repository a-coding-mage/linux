/* threadtest.c
 *		by: john stultz (johnstul@us.ibm.com)
 *		(C) Copyright IBM 2004, 2005, 2006, 2012
 *		Licensed under the GPLv2
 *
 *  To build:
 *	$ gcc threadtest.c -o threadtest -lrt
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 2 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

/* Dependencies originally included from:
 * stdio.h, unistd.h, stdlib.h, sys/time.h, pthread.h, and "kselftest.h".
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
	pub tv_sec: time_t,
	pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
	pub tm_sec: c_int,
	pub tm_min: c_int,
	pub tm_hour: c_int,
	pub tm_mday: c_int,
	pub tm_mon: c_int,
	pub tm_year: c_int,
	pub tm_wday: c_int,
	pub tm_yday: c_int,
	pub tm_isdst: c_int,
	pub tm_gmtoff: c_long,
	pub tm_zone: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pthread_mutex_t {
	pub __data: [u8; 40],
}

pub type pthread_t = c_long;
pub type time_t = c_long;
pub type size_t = usize;
pub type clockid_t = c_int;

const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { __data: [0; 40] };
const CLOCK_MONOTONIC: clockid_t = 1;

unsafe extern "C" {
	fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
	fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const c_void,
		start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
	fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
	fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
	fn atoi(nptr: *const c_char) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;
	fn fflush(stream: *mut c_void) -> c_int;
	fn setbuf(stream: *mut c_void, buf: *mut c_char);
	fn time(tloc: *mut time_t) -> time_t;
	fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;
	fn localtime(timep: *const time_t) -> *mut tm;
	fn sleep(seconds: c_int) -> c_int;
	static mut optarg: *mut c_char;
	static mut stdout: *mut c_void;
	fn ksft_exit_fail() -> !;
	fn ksft_exit_pass() -> !;
}

/* serializes shared list access */
static mut list_lock: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
/* serializes console output */
static mut print_lock: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

const MAX_THREADS: usize = 128;
const LISTSIZE: usize = 128;

static mut done: c_int = 0;

static mut global_list: [timespec; LISTSIZE] = [timespec { tv_sec: 0, tv_nsec: 0 }; LISTSIZE];
static mut listcount: c_int = 0;

unsafe extern "C" fn checklist(list: *const timespec, size: c_int) {
	let mut i: c_int;
	let mut j: c_int;
	let mut a: *const timespec;
	let mut b: *const timespec;

	/* scan the list */
	i = 0;
	while i < size - 1 {
		a = list.add(i as usize);
		b = list.add((i + 1) as usize);

		/* look for any time inconsistencies */
		if ((*b).tv_sec <= (*a).tv_sec) && ((*b).tv_nsec < (*a).tv_nsec) {
			/* flag other threads */
			done = 1;

			/*serialize printing to avoid junky output*/
			pthread_mutex_lock(&raw mut print_lock);

			/* dump the list */
			printf(c"\n".as_ptr());
			j = 0;
			while j < size {
				if j == i {
					printf(c"---------------\n".as_ptr());
				}
				printf(
					c"%lu:%lu\n".as_ptr(),
					(*list.add(j as usize)).tv_sec as c_long,
					(*list.add(j as usize)).tv_nsec as c_long,
				);
				if j == i + 1 {
					printf(c"---------------\n".as_ptr());
				}
				j += 1;
			}
			printf(c"[FAILED]\n".as_ptr());

			pthread_mutex_unlock(&raw mut print_lock);
		}
		i += 1;
	}
}

/* The shared thread shares a global list
 * that each thread fills while holding the lock.
 * This stresses clock synchronization across cpus.
 */
unsafe extern "C" fn shared_thread(_arg: *mut c_void) -> *mut c_void {
	while done == 0 {
		/* protect the list */
		pthread_mutex_lock(&raw mut list_lock);

		/* see if we're ready to check the list */
		if listcount >= LISTSIZE as c_int {
			checklist((&raw const global_list).cast::<timespec>(), LISTSIZE as c_int);
			listcount = 0;
		}
		let idx = listcount as usize;
		listcount += 1;
		clock_gettime(CLOCK_MONOTONIC, (&raw mut global_list[idx]).cast::<timespec>());

		pthread_mutex_unlock(&raw mut list_lock);
	}
	core::ptr::null_mut()
}

/* Each independent thread fills in its own
 * list. This stresses clock_gettime() lock contention.
 */
unsafe extern "C" fn independent_thread(_arg: *mut c_void) -> *mut c_void {
	let mut my_list: [timespec; LISTSIZE] = [timespec { tv_sec: 0, tv_nsec: 0 }; LISTSIZE];
	let mut count: c_int;

	while done == 0 {
		/* fill the list */
		count = 0;
		while count < LISTSIZE as c_int {
			clock_gettime(CLOCK_MONOTONIC, &mut my_list[count as usize]);
			count += 1;
		}
		checklist(my_list.as_ptr(), LISTSIZE as c_int);
	}
	core::ptr::null_mut()
}

const DEFAULT_THREAD_COUNT: c_int = 8;
const DEFAULT_RUNTIME: time_t = 30;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let mut thread_count: c_int;
	let mut i: c_int;
	let mut start: time_t;
	let mut now: time_t = 0;
	let mut runtime: time_t;
	let mut buf: [c_char; 255] = [0; 255];
	let mut pth: [pthread_t; MAX_THREADS] = [0; MAX_THREADS];
	let mut opt: c_int;
	let mut tret: *mut c_void = core::ptr::null_mut();
	let mut ret: c_int = 0;
	let mut thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void> = Some(shared_thread);

	thread_count = DEFAULT_THREAD_COUNT;
	runtime = DEFAULT_RUNTIME;

	/* Process arguments */
	loop {
		opt = getopt(argc, argv, c"t:n:i".as_ptr());
		if opt == -1 {
			break;
		}
		match opt {
			116 => {
				runtime = atoi(optarg) as time_t;
			}
			110 => {
				thread_count = atoi(optarg);
			}
			105 => {
				thread = Some(independent_thread);
				printf(c"using independent threads\n".as_ptr());
			}
			_ => {
				printf(c"Usage: %s [-t <secs>] [-n <numthreads>] [-i]\n".as_ptr(), *argv);
				printf(c"\t-t: time to run\n".as_ptr());
				printf(c"\t-n: number of threads\n".as_ptr());
				printf(c"\t-i: use independent threads\n".as_ptr());
				return -1;
			}
		}
	}

	if thread_count > MAX_THREADS as c_int {
		thread_count = MAX_THREADS as c_int;
	}

	setbuf(stdout, core::ptr::null_mut());

	start = time(core::ptr::null_mut());
	strftime(
		buf.as_mut_ptr(),
		255,
		c"%a, %d %b %Y %T %z".as_ptr(),
		localtime(&start),
	);
	printf(c"%s\n".as_ptr(), buf.as_ptr());
	printf(
		c"Testing consistency with %i threads for %ld seconds: ".as_ptr(),
		thread_count,
		runtime,
	);
	fflush(stdout);

	/* spawn */
	i = 0;
	while i < thread_count {
		pthread_create(
			&mut pth[i as usize],
			core::ptr::null(),
			thread,
			core::ptr::null_mut(),
		);
		i += 1;
	}

	while time(&mut now) < start + runtime {
		sleep(1);
		if done != 0 {
			ret = 1;
			strftime(
				buf.as_mut_ptr(),
				255,
				c"%a, %d %b %Y %T %z".as_ptr(),
				localtime(&now),
			);
			printf(c"%s\n".as_ptr(), buf.as_ptr());
			break;
		}
	}
	if ret == 0 {
		printf(c"[OK]\n".as_ptr());
		done = 1;
	}

	/* wait */
	i = 0;
	while i < thread_count {
		pthread_join(pth[i as usize], &mut tret);
		i += 1;
	}

	/* die */
	if ret != 0 {
		ksft_exit_fail();
	}
	ksft_exit_pass();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
