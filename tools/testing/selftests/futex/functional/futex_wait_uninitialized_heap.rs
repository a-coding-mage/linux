// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 * Copyright FUJITSU LIMITED 2010
 * Copyright KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
 *
 * DESCRIPTION
 *      Wait on uninitialized heap. It shold be zero and FUTEX_WAIT should
 *      return immediately. This test is intent to test zero page handling in
 *      futex.
 *
 * AUTHOR
 *      KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
 *
 * HISTORY
 *      2010-Jan-6: Initial version by KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
 *
 *****************************************************************************/

// C includes translated as external dependencies:
// errno.h, libgen.h, pthread.h, stdio.h, stdlib.h, string.h, unistd.h,
// linux/futex.h, sys/mman.h, syscall.h, sys/types.h, sys/stat.h,
// "futextest.h", and "kselftest_harness.h".

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

const WAIT_US: c_int = 5000000;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const _SC_PAGESIZE: c_int = 30;
const EWOULDBLOCK: c_int = 11;

#[repr(C)]
pub struct __test_metadata {
    _unused: [u8; 0],
}

type pthread_t = usize;

unsafe extern "C" {
    static mut errno: c_int;

    fn futex_wait(uaddr: *mut c_void, val: c_int, timeout: *mut c_void, flags: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn pthread_exit(value_ptr: *mut c_void) -> !;
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn usleep(usec: c_int) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;

    fn TH_LOG(fmt: *const c_char, ...);
    fn EXPECT_EQ_int(left: c_int, right: c_int);
    fn EXPECT_TRUE_bool(value: bool);
    fn ASSERT_NE_ptr(left: *mut c_void, right: *mut c_void);
    fn ASSERT_EQ_int(left: c_int, right: c_int);
}

static mut child_blocked: c_int = 1;
static mut child_ret: bool = false;
static mut buf: *mut c_void = ptr::null_mut();

unsafe extern "C" fn wait_thread(arg: *mut c_void) -> *mut c_void {
    let _metadata: *mut __test_metadata = arg as *mut __test_metadata;
    let res: c_int;

    child_ret = true;
    res = futex_wait(buf, 1, ptr::null_mut(), 0);
    child_blocked = 0;

    if res != 0 && errno != EWOULDBLOCK {
        EXPECT_EQ_int(res, 0);
        TH_LOG(
            b"futex failure: %s\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        child_ret = false;
    }
    pthread_exit(ptr::null_mut());
}

unsafe fn futex_wait_uninitialized_heap(_metadata: *mut __test_metadata) {
    let page_size: c_long;
    let mut thr: pthread_t = 0;
    let ret: c_int;

    page_size = sysconf(_SC_PAGESIZE);

    buf = mmap(
        ptr::null_mut(),
        page_size as usize,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        0,
        0,
    );
    ASSERT_NE_ptr(buf, MAP_FAILED);
    TH_LOG(
        b"mmap failed: %s\0".as_ptr() as *const c_char,
        strerror(errno),
    );

    ret = pthread_create(
        &mut thr,
        ptr::null(),
        wait_thread,
        _metadata as *mut c_void,
    );
    ASSERT_EQ_int(ret, 0);
    TH_LOG(b"pthread_create failed\0".as_ptr() as *const c_char);

    TH_LOG(
        b"waiting %dus for child to return\0".as_ptr() as *const c_char,
        WAIT_US,
    );
    usleep(WAIT_US);

    EXPECT_EQ_int(child_blocked, 0);
    TH_LOG(b"child blocked in kernel\0".as_ptr() as *const c_char);
    EXPECT_TRUE_bool(child_ret);
    TH_LOG(b"child error\0".as_ptr() as *const c_char);

    pthread_join(thr, ptr::null_mut());
    munmap(buf, page_size as usize);
}

// TEST(futex_wait_uninitialized_heap)
// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
