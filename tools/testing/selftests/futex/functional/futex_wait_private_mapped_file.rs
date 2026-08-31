// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 * Copyright FUJITSU LIMITED 2010
 * Copyright KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
 *
 * DESCRIPTION
 *      Internally, Futex has two handling mode, anon and file. The private file
 *      mapping is special. At first it behave as file, but after write anything
 *      it behave as anon. This test is intent to test such case.
 *
 * AUTHOR
 *      KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
 *
 * HISTORY
 *      2010-Jan-6: Initial version by KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
 *
 *****************************************************************************/

// C dependencies: stdio.h, stdlib.h, syscall.h, unistd.h, errno.h,
// linux/futex.h, pthread.h, libgen.h, signal.h, string.h, futextest.h,
// kselftest_harness.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const PAGE_SZ: usize = 4096;

type futex_t = c_int;
type pthread_t = c_ulong;
type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

static mut pad: [c_char; PAGE_SZ] = {
    let mut a = [0; PAGE_SZ];
    a[0] = 1;
    a
};
static mut val: futex_t = 1;
static mut pad2: [c_char; PAGE_SZ] = {
    let mut a = [0; PAGE_SZ];
    a[0] = 1;
    a
};

const WAKE_WAIT_US: c_uint = 3000000;
static mut wait_timeout: timespec = timespec {
    tv_sec: 5,
    tv_nsec: 0,
};

const EWOULDBLOCK: c_int = 11;
const ETIMEDOUT: c_int = 110;

unsafe extern "C" {
    static mut errno: c_int;

    fn futex_wait(
        uaddr: *mut futex_t,
        val: futex_t,
        timeout: *mut timespec,
        opflags: c_int,
    ) -> c_int;
    fn futex_wake(uaddr: *mut futex_t, nr_wake: c_int, opflags: c_int) -> c_int;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
}

unsafe extern "C" fn thr_futex_wait(arg: *mut c_void) -> *mut c_void {
    let _metadata: *mut __test_metadata = arg as *mut __test_metadata;
    let mut ret: c_int;

    TH_LOG!("futex wait");
    ret = futex_wait(&raw mut val, 1, &raw mut wait_timeout, 0);
    if ret != 0 && errno != EWOULDBLOCK && errno != ETIMEDOUT {
        ASSERT_TRUE!(0);
        TH_LOG!("futex error: %s", strerror(errno));
    }

    if ret != 0 && errno == ETIMEDOUT {
        ASSERT_TRUE!(0);
        TH_LOG!("waiter timedout");
    }

    TH_LOG!("futex_wait: ret = %d, errno = %d", ret, errno);

    core::ptr::null_mut()
}

TEST!(wait_private_mapped_file, {
    let mut thr: pthread_t = 0;
    let mut res: c_int;

    res = pthread_create(
        &mut thr as *mut pthread_t,
        core::ptr::null(),
        thr_futex_wait,
        _metadata as *mut c_void,
    );
    ASSERT_EQ!(res, 0);
    TH_LOG!("pthread_create error");

    TH_LOG!("wait a while");
    usleep(WAKE_WAIT_US);
    val = 2;
    res = futex_wake(&raw mut val, 1, 0);
    TH_LOG!("futex_wake %d", res);
    EXPECT_EQ!(res, 1);
    TH_LOG!("FUTEX_WAKE didn't find the waiting thread");

    TH_LOG!("join");
    pthread_join(thr, core::ptr::null_mut());
});

TEST_HARNESS_MAIN!();
