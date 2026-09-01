// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * futex_waitv() test by Andre Almeida <andrealmeid@collabora.com>
 *
 * Copyright 2021 Collabora Ltd.
 */

// C dependencies in the original source:
// errno.h, error.h, getopt.h, stdio.h, stdlib.h, string.h, time.h,
// pthread.h, stdint.h, sys/shm.h, futextest.h, futex2test.h,
// kselftest_harness.h

use core::ffi::{c_char, c_int, c_uint, c_void};

const WAKE_WAIT_US: c_uint = 10000;
const NR_FUTEXES: usize = 30;

#[repr(C)]
pub struct futex_waitv {
    pub val: u64,
    pub uaddr: u64,
    pub flags: u32,
    pub __reserved: u32,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

type pthread_t = usize;

const CLOCK_MONOTONIC: c_int = 1;
const CLOCK_TAI: c_int = 11;
const EINVAL: c_int = 22;
const ENOSYS: c_int = 38;
const IPC_PRIVATE: c_int = 0;
const IPC_CREAT: c_int = 0o1000;
const FUTEX_PRIVATE_FLAG: c_int = 128;
const FUTEX_32: u32 = 2;

static mut waitv: [futex_waitv; NR_FUTEXES] = [futex_waitv {
    val: 0,
    uaddr: 0,
    flags: 0,
    __reserved: 0,
}; NR_FUTEXES];
static mut futexes: [u32; NR_FUTEXES] = [0; NR_FUTEXES];

unsafe extern "C" {
    static mut errno: c_int;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn shmget(key: c_int, size: usize, shmflg: c_int) -> c_int;
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_uint;
    fn shmdt(shmaddr: *const c_void) -> c_int;

    fn futex_waitv(
        waiters: *mut futex_waitv,
        nr_futexes: c_uint,
        flags: c_uint,
        timeout: *mut timespec,
        clockid: c_int,
    ) -> c_int;
    fn futex_wake(uaddr: *mut c_void, nr_wake: c_int, opflags: c_int) -> c_int;
    fn is_futex_waitv_supported() -> bool;
    fn u64_to_ptr(addr: u64) -> *mut c_void;
}

unsafe extern "C" fn waiterfn(arg: *mut c_void) -> *mut c_void {
    let _metadata: *mut __test_metadata = arg as *mut __test_metadata;
    let mut to: timespec = core::mem::zeroed();
    let mut res: c_int;

    /* setting absolute timeout for futex2 */
    ASSERT_EQ!(clock_gettime(CLOCK_MONOTONIC, &mut to), 0);
    TH_LOG!("gettime64 failed");

    to.tv_sec += 1;

    res = futex_waitv(
        waitv.as_mut_ptr(),
        NR_FUTEXES as c_uint,
        0,
        &mut to,
        CLOCK_MONOTONIC,
    );
    if res < 0 {
        EXPECT_EQ!(res, (NR_FUTEXES - 1) as c_int);
        TH_LOG!("futex_waitv failed: %s", strerror(errno));
    } else {
        EXPECT_EQ!(res, (NR_FUTEXES - 1) as c_int);
        TH_LOG!(
            "futex_waitv returned %d, expected %d",
            res,
            NR_FUTEXES - 1
        );
    }

    core::ptr::null_mut()
}

TEST!(private_waitv, {
    let mut waiter: pthread_t = 0;
    let mut res: c_int;
    let mut i: c_int;

    unsafe {
        if !is_futex_waitv_supported() {
            SKIP!(return, "futex_waitv syscall not supported");
        }

        i = 0;
        while i < NR_FUTEXES as c_int {
            waitv[i as usize].uaddr = (&mut futexes[i as usize] as *mut u32) as usize as u64;
            waitv[i as usize].flags = FUTEX_32 | FUTEX_PRIVATE_FLAG as u32;
            waitv[i as usize].val = 0;
            waitv[i as usize].__reserved = 0;
            i += 1;
        }

        /* Private waitv */
        ASSERT_EQ!(
            pthread_create(
                &mut waiter,
                core::ptr::null(),
                waiterfn,
                _metadata as *mut c_void
            ),
            0
        );
        TH_LOG!("pthread_create failed");

        usleep(WAKE_WAIT_US);

        res = futex_wake(
            u64_to_ptr(waitv[NR_FUTEXES - 1].uaddr),
            1,
            FUTEX_PRIVATE_FLAG,
        );
        EXPECT_EQ!(res, 1);
        TH_LOG!(
            "futex_wake private returned: %d %s",
            res,
            if res < 0 {
                strerror(errno)
            } else {
                b"\0".as_ptr() as *mut c_char
            }
        );
    }
});

TEST!(shared_waitv, {
    let mut waiter: pthread_t = 0;
    let mut res: c_int;
    let mut i: c_int;

    unsafe {
        if !is_futex_waitv_supported() {
            SKIP!(return, "futex_waitv syscall not supported");
        }

        /* Shared waitv */
        i = 0;
        while i < NR_FUTEXES as c_int {
            let shm_id: c_int = shmget(IPC_PRIVATE, 4096, IPC_CREAT | 0o666);

            if shm_id < 0 {
                if errno == ENOSYS {
                    SKIP!(return, "shmget syscall not supported");
                }
                ASSERT_GE!(shm_id, 0);
                TH_LOG!("shmget failed");
            }

            let shared_data: *mut c_uint = shmat(shm_id, core::ptr::null(), 0);

            *shared_data = 0;
            waitv[i as usize].uaddr = shared_data as usize as u64;
            waitv[i as usize].flags = FUTEX_32;
            waitv[i as usize].val = 0;
            waitv[i as usize].__reserved = 0;
            i += 1;
        }

        ASSERT_EQ!(
            pthread_create(
                &mut waiter,
                core::ptr::null(),
                waiterfn,
                _metadata as *mut c_void
            ),
            0
        );
        TH_LOG!("pthread_create failed");

        usleep(WAKE_WAIT_US);

        res = futex_wake(u64_to_ptr(waitv[NR_FUTEXES - 1].uaddr), 1, 0);
        EXPECT_EQ!(res, 1);
        TH_LOG!(
            "futex_wake shared returned: %d %s",
            res,
            if res < 0 {
                strerror(errno)
            } else {
                b"\0".as_ptr() as *mut c_char
            }
        );

        i = 0;
        while i < NR_FUTEXES as c_int {
            shmdt(u64_to_ptr(waitv[i as usize].uaddr));
            i += 1;
        }
    }
});

TEST!(invalid_flag, {
    let mut to: timespec = unsafe { core::mem::zeroed() };
    let mut res: c_int;

    unsafe {
        if !is_futex_waitv_supported() {
            SKIP!(return, "futex_waitv syscall not supported");
        }

        /* Testing a waiter without FUTEX_32 flag */
        waitv[0].flags = FUTEX_PRIVATE_FLAG as u32;

        ASSERT_EQ!(clock_gettime(CLOCK_MONOTONIC, &mut to), 0);
        TH_LOG!("gettime64 failed");

        to.tv_sec += 1;

        res = futex_waitv(
            waitv.as_mut_ptr(),
            NR_FUTEXES as c_uint,
            0,
            &mut to,
            CLOCK_MONOTONIC,
        );

        EXPECT_EQ!(res, -1);
        TH_LOG!("futex_waitv returned unexpected result: %d", res);
        if res == -1 {
            EXPECT_EQ!(errno, EINVAL);
            TH_LOG!("futex_waitv returned unexpected errno: %d", errno);
        }
    }
});

TEST!(unaligned_address, {
    let mut to: timespec = unsafe { core::mem::zeroed() };
    let mut res: c_int;

    unsafe {
        if !is_futex_waitv_supported() {
            SKIP!(return, "futex_waitv syscall not supported");
        }

        /* Testing a waiter with an unaligned address */
        waitv[0].flags = FUTEX_PRIVATE_FLAG as u32 | FUTEX_32;
        waitv[0].uaddr = 1;

        ASSERT_EQ!(clock_gettime(CLOCK_MONOTONIC, &mut to), 0);
        TH_LOG!("gettime64 failed");

        to.tv_sec += 1;

        res = futex_waitv(
            waitv.as_mut_ptr(),
            NR_FUTEXES as c_uint,
            0,
            &mut to,
            CLOCK_MONOTONIC,
        );

        EXPECT_EQ!(res, -1);
        TH_LOG!("futex_waitv returned unexpected result: %d", res);
        if res == -1 {
            EXPECT_EQ!(errno, EINVAL);
            TH_LOG!("futex_waitv returned unexpected errno: %d", errno);
        }
    }
});

TEST!(null_address, {
    let mut to: timespec = unsafe { core::mem::zeroed() };
    let mut res: c_int;

    unsafe {
        if !is_futex_waitv_supported() {
            SKIP!(return, "futex_waitv syscall not supported");
        }

        /* Testing a NULL address for waiters.uaddr */
        waitv[0].uaddr = 0x00000000;

        ASSERT_EQ!(clock_gettime(CLOCK_MONOTONIC, &mut to), 0);
        TH_LOG!("gettime64 failed");

        to.tv_sec += 1;

        res = futex_waitv(
            waitv.as_mut_ptr(),
            NR_FUTEXES as c_uint,
            0,
            &mut to,
            CLOCK_MONOTONIC,
        );

        EXPECT_EQ!(res, -1);
        TH_LOG!("futex_waitv returned unexpected result: %d", res);
        if res == -1 {
            EXPECT_EQ!(errno, EINVAL);
            TH_LOG!("futex_waitv returned unexpected errno: %d", errno);
        }

        /* Testing a NULL address for *waiters */
        ASSERT_EQ!(clock_gettime(CLOCK_MONOTONIC, &mut to), 0);
        TH_LOG!("gettime64 failed");

        to.tv_sec += 1;

        res = futex_waitv(
            core::ptr::null_mut(),
            NR_FUTEXES as c_uint,
            0,
            &mut to,
            CLOCK_MONOTONIC,
        );

        EXPECT_EQ!(res, -1);
        TH_LOG!("futex_waitv returned unexpected result: %d", res);
        if res == -1 {
            EXPECT_EQ!(errno, EINVAL);
            TH_LOG!("futex_waitv returned unexpected errno: %d", errno);
        }
    }
});

TEST!(invalid_clockid, {
    let mut to: timespec = unsafe { core::mem::zeroed() };
    let mut res: c_int;

    unsafe {
        if !is_futex_waitv_supported() {
            SKIP!(return, "futex_waitv syscall not supported");
        }

        /* Testing an invalid clockid */
        ASSERT_EQ!(clock_gettime(CLOCK_MONOTONIC, &mut to), 0);
        TH_LOG!("gettime64 failed");

        to.tv_sec += 1;

        res = futex_waitv(
            core::ptr::null_mut(),
            NR_FUTEXES as c_uint,
            0,
            &mut to,
            CLOCK_TAI,
        );

        EXPECT_EQ!(res, -1);
        TH_LOG!("futex_waitv returned unexpected result: %d", res);
        if res == -1 {
            EXPECT_EQ!(errno, EINVAL);
            TH_LOG!("futex_waitv returned unexpected errno: %d", errno);
        }
    }
});

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
