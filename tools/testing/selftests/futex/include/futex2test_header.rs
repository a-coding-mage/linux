/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Futex2 library addons for futex tests
 *
 * Copyright 2021 Collabora Ltd.
 */

use core::ffi::{c_int, c_long, c_uint, c_ulong, c_void};

// C dependencies: <linux/time_types.h>, <errno.h>, <stdint.h>, <stdbool.h>.

pub const fn u64_to_ptr(x: u64) -> *mut c_void {
    x as usize as *mut c_void
}

// If __NR_futex_waitv is not provided by the target headers, C defines it here.
pub const __NR_futex_waitv: c_long = 449;

#[repr(C)]
pub struct futex_waitv {
    pub val: u64,
    pub uaddr: u64,
    pub flags: u32,
    pub __reserved: u32,
}

// If __NR_futex_wake is not provided by the target headers, C defines it here.
pub const __NR_futex_wake: c_long = 454;

// If __NR_futex_wait is not provided by the target headers, C defines it here.
pub const __NR_futex_wait: c_long = 455;

// If FUTEX2_* constants are not provided by the target headers, C defines them here.
pub const FUTEX2_SIZE_U32: c_uint = 0x02;
pub const FUTEX2_NUMA: c_uint = 0x04;
pub const FUTEX2_MPOL: c_uint = 0x08;

// C fallback: #define FUTEX2_PRIVATE FUTEX_PRIVATE_FLAG
pub const FUTEX2_PRIVATE: c_uint = FUTEX_PRIVATE_FLAG;

// C fallback: #define FUTEX_NO_NODE (-1)
pub const FUTEX_NO_NODE: c_int = -1;

// C fallback: #define FUTEX_32 FUTEX2_SIZE_U32
pub const FUTEX_32: c_uint = FUTEX2_SIZE_U32;

#[repr(C)]
pub struct futex32_numa {
    pub futex: futex_t,
    pub numa: futex_t,
}

unsafe extern "C" {
    pub fn syscall(num: c_long, ...) -> c_long;
    pub static mut errno: c_int;
}

/**
 * futex_waitv - Wait at multiple futexes, wake on any
 * @waiters:    Array of waiters
 * @nr_waiters: Length of waiters array
 * @flags: Operation flags
 * @timo:  Optional timeout for operation
 */
pub unsafe fn futex_waitv(
    waiters: *mut futex_waitv,
    nr_waiters: c_ulong,
    flags: c_ulong,
    timo: *mut timespec,
    clockid: clockid_t,
) -> c_int {
    let mut ts = __kernel_timespec {
        tv_sec: unsafe { (*timo).tv_sec },
        tv_nsec: unsafe { (*timo).tv_nsec },
    };

    unsafe {
        syscall(
            __NR_futex_waitv,
            waiters,
            nr_waiters,
            flags,
            &mut ts as *mut __kernel_timespec,
            clockid,
        ) as c_int
    }
}

/*
 * futex_wait() - block on uaddr with optional timeout
 * @val:	Expected value
 * @flags:	FUTEX2 flags
 * @timeout:	Relative timeout
 * @clockid:	Clock id for the timeout
 */
pub unsafe fn futex2_wait(
    uaddr: *mut c_void,
    val: c_long,
    flags: c_uint,
    timeout: *mut timespec,
    clockid: clockid_t,
) -> c_int {
    unsafe {
        syscall(
            __NR_futex_wait,
            uaddr,
            val,
            !0u32,
            flags,
            timeout,
            clockid,
        ) as c_int
    }
}

/*
 * futex2_wake() - Wake a number of futexes
 * @nr:		Number of threads to wake at most
 * @flags:	FUTEX2 flags
 */
pub unsafe fn futex2_wake(uaddr: *mut c_void, nr: c_int, flags: c_uint) -> c_int {
    unsafe { syscall(__NR_futex_wake, uaddr, !0u32, nr, flags) as c_int }
}

pub unsafe fn is_futex_waitv_supported() -> bool {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let res = unsafe {
        futex_waitv(
            core::ptr::null_mut(),
            0,
            0,
            &mut ts as *mut timespec,
            CLOCK_MONOTONIC,
        )
    };

    !(res < 0 && unsafe { errno } == ENOSYS)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
