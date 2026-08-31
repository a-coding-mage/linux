// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 *   Copyright © International Business Machines  Corp., 2009
 *
 * DESCRIPTION
 *      Test if FUTEX_WAIT op returns -EWOULDBLOCK if the futex value differs
 *      from the expected one.
 *
 * AUTHOR
 *      Gowrishankar <gowrishankar.m@in.ibm.com>
 *
 * HISTORY
 *      2009-Nov-14: Initial version by Gowrishankar <gowrishankar.m@in.ibm.com>
 *
 *****************************************************************************/

// C dependencies removed from executable Rust:
// errno.h, getopt.h, stdio.h, stdlib.h, string.h, time.h
// futextest.h, futex2test.h, kselftest_harness.h

const timeout_ns: i64 = 100000;

unsafe extern "C" {
    static mut errno: ::std::os::raw::c_int;

    static FUTEX_INITIALIZER: futex_t;
    static FUTEX_PRIVATE_FLAG: ::std::os::raw::c_int;
    static FUTEX_32: u32;
    static CLOCK_MONOTONIC: ::std::os::raw::c_int;
    static EWOULDBLOCK: ::std::os::raw::c_int;

    fn futex_wait(
        uaddr: *mut futex_t,
        val: futex_t,
        timeout: *const timespec,
        opflags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn futex_waitv(
        waiters: *mut futex_waitv,
        nr_futexes: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
        timeout: *const timespec,
        clockid: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn is_futex_waitv_supported() -> bool;
    fn clock_gettime(clk_id: ::std::os::raw::c_int, tp: *mut timespec) -> ::std::os::raw::c_int;
}

type futex_t = u32;

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct futex_waitv {
    uaddr: usize,
    val: futex_t,
    flags: u32,
    __reserved: u32,
}

// TEST(futex_wait_wouldblock)
unsafe fn futex_wait_wouldblock() {
    let mut to = timespec {
        tv_sec: 0,
        tv_nsec: timeout_ns,
    };
    let mut f1: futex_t = FUTEX_INITIALIZER;
    let mut res: ::std::os::raw::c_int;

    TH_LOG!(
        "Calling futex_wait on f1: %u @ %p with val=%u",
        f1,
        &mut f1,
        f1.wrapping_add(1)
    );
    res = futex_wait(
        &mut f1,
        f1.wrapping_add(1),
        &to,
        FUTEX_PRIVATE_FLAG,
    );
    EXPECT_EQ!(res, -1);
    TH_LOG!("futex_wait returned unexpected result: %d", res);
    if res == -1 {
        EXPECT_EQ!(errno, EWOULDBLOCK);
        TH_LOG!("futex_wait returned unexpected errno: %d", errno);
    }
}

// TEST(futex_waitv_wouldblock)
unsafe fn futex_waitv_wouldblock() {
    let mut to = timespec {
        tv_sec: 0,
        tv_nsec: timeout_ns,
    };
    let mut f1: futex_t = FUTEX_INITIALIZER;
    let mut waitv = futex_waitv {
        uaddr: (&mut f1 as *mut futex_t) as usize,
        val: f1.wrapping_add(1),
        flags: FUTEX_32,
        __reserved: 0,
    };
    let mut res: ::std::os::raw::c_int;

    if !is_futex_waitv_supported() {
        SKIP!(return, "futex_waitv syscall not supported");
    }

    ASSERT_EQ!(clock_gettime(CLOCK_MONOTONIC, &mut to), 0);
    TH_LOG!("clock_gettime failed");

    to.tv_nsec += timeout_ns;

    if to.tv_nsec >= 1000000000 {
        to.tv_sec += 1;
        to.tv_nsec -= 1000000000;
    }

    TH_LOG!(
        "Calling futex_waitv on f1: %u @ %p with val=%u",
        f1,
        &mut f1,
        f1.wrapping_add(1)
    );
    res = futex_waitv(&mut waitv, 1, 0, &to, CLOCK_MONOTONIC);
    EXPECT_EQ!(res, -1);
    TH_LOG!("futex_waitv returned unexpected result: %d", res);
    if res == -1 {
        EXPECT_EQ!(errno, EWOULDBLOCK);
        TH_LOG!("futex_waitv returned unexpected errno: %d", errno);
    }
}

// TEST_HARNESS_MAIN
