// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 *   Copyright © International Business Machines  Corp., 2009
 *
 * DESCRIPTION
 *      1. Block a thread using FUTEX_WAIT
 *      2. Attempt to use FUTEX_CMP_REQUEUE_PI on the futex from 1.
 *      3. The kernel must detect the mismatch and return -EINVAL.
 *
 * AUTHOR
 *      Darren Hart <dvhart@linux.intel.com>
 *
 * HISTORY
 *      2009-Nov-9: Initial version by Darren Hart <dvhart@linux.intel.com>
 *
 *****************************************************************************/

// C dependencies: errno.h, getopt.h, pthread.h, stdio.h, stdlib.h, string.h,
// time.h, futextest.h, kselftest_harness.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type futex_t = c_int;
type pthread_t = usize;
type pthread_attr_t = c_void;

#[repr(C)]
pub struct __test_metadata {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut errno: c_int;

	fn strerror(errnum: c_int) -> *mut c_char;
	fn sleep(seconds: c_uint) -> c_uint;
	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const pthread_attr_t,
		start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
	fn TH_LOG(fmt: *const c_char, ...);

	fn futex_wait(
		uaddr: *mut futex_t,
		val: futex_t,
		timeout: *mut c_void,
		opflags: c_int,
	) -> c_int;
	fn futex_cmp_requeue_pi(
		uaddr: *mut futex_t,
		val: futex_t,
		uaddr2: *mut futex_t,
		nr_wake: c_int,
		nr_requeue: c_int,
		opflags: c_int,
	) -> c_int;
	fn futex_wake(uaddr: *mut futex_t, nr_wake: c_int, opflags: c_int) -> c_int;
}

const FUTEX_INITIALIZER: futex_t = 0;
const FUTEX_PRIVATE_FLAG: c_int = 128;
const EINVAL: c_int = 22;

static mut f1: futex_t = FUTEX_INITIALIZER;
static mut f2: futex_t = FUTEX_INITIALIZER;
static mut child_ret: c_int = 0;

unsafe extern "C" fn blocking_child(arg: *mut c_void) -> *mut c_void {
	let _metadata: *mut __test_metadata = arg as *mut __test_metadata;

	child_ret = futex_wait(&raw mut f1, f1, ptr::null_mut(), FUTEX_PRIVATE_FLAG);
	if child_ret < 0 {
		child_ret = -errno;
		ASSERT_EQ!(child_ret, 0);
		TH_LOG(c"futex_wait failed: %s".as_ptr(), strerror(errno));
	}
	&raw mut child_ret as *mut c_void
}

// TEST(requeue_pi_mismatched_ops)
unsafe fn requeue_pi_mismatched_ops(_metadata: *mut __test_metadata) {
	let mut child: pthread_t = 0;
	let mut ret: c_int;

	ASSERT_EQ!(
		pthread_create(
			&mut child,
			ptr::null(),
			blocking_child,
			_metadata as *mut c_void
		),
		0
	);
	TH_LOG(c"pthread_create failed".as_ptr());

	/* Allow the child to block in the kernel. */
	sleep(1);

	/*
	 * The kernel should detect the waiter did not setup the
	 * q->requeue_pi_key and return -EINVAL. If it does not,
	 * it likely gave the lock to the child, which is now hung
	 * in the kernel.
	 */
	ret = futex_cmp_requeue_pi(&raw mut f1, f1, &raw mut f2, 1, 0, FUTEX_PRIVATE_FLAG);
	if ret < 0 {
		if errno == EINVAL {
			/*
			 * The kernel correctly detected the mismatched
			 * requeue_pi target and aborted. Wake the child with
			 * FUTEX_WAKE.
			 */
			ret = futex_wake(&raw mut f1, 1, FUTEX_PRIVATE_FLAG);
			if ret == 1 {
				ret = 0;
			} else if ret < 0 {
				ASSERT_GE!(ret, 0);
				TH_LOG(c"futex_wake failed: %s".as_ptr(), strerror(errno));
			} else {
				ASSERT_TRUE!(0);
				TH_LOG(c"futex_wake did not wake the child".as_ptr());
			}
		} else {
			ASSERT_TRUE!(0);
			TH_LOG(
				c"futex_cmp_requeue_pi failed with unexpected errno: %s".as_ptr(),
				strerror(errno),
			);
		}
	} else if ret > 0 {
		EXPECT_EQ!(ret, 0);
		TH_LOG(c"futex_cmp_requeue_pi failed to detect the mismatch".as_ptr());
	} else {
		ASSERT_TRUE!(0);
		TH_LOG(c"futex_cmp_requeue_pi found no waiters".as_ptr());
	}

	pthread_join(child, ptr::null_mut());

	EXPECT_EQ!(ret, 0);
	TH_LOG(c"Test failed: ret=%d".as_ptr(), ret);
	EXPECT_EQ!(child_ret, 0);
	TH_LOG(c"Child failed: child_ret=%d".as_ptr(), child_ret);
}

// TEST_HARNESS_MAIN
