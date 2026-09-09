/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency: declarations from <linux/types.h> are supplied externally. */

#[repr(C)]
pub struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t, /* seconds */
    pub tv_nsec: i64,              /* nanoseconds */
}

#[repr(C)]
pub struct __kernel_itimerspec {
    pub it_interval: __kernel_timespec, /* timer period */
    pub it_value: __kernel_timespec,    /* timer expiration */
}

/*
 * legacy timeval structure, only embedded in structures that
 * traditionally used 'timeval' to pass time intervals (not absolute
 * times). Do not add new users. If user space fails to compile
 * here, this is probably because it is not y2038 safe and needs to
 * be changed to use another interface.
 */
/* C conditional: define only when __kernel_old_timeval is not already supplied. */
#[repr(C)]
pub struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_long_t,
}

#[repr(C)]
pub struct __kernel_old_timespec {
    pub tv_sec: __kernel_old_time_t, /* seconds */
    pub tv_nsec: __kernel_long_t,    /* nanoseconds */
}

#[repr(C)]
pub struct __kernel_old_itimerval {
    pub it_interval: __kernel_old_timeval, /* timer interval */
    pub it_value: __kernel_old_timeval,    /* current value */
}

#[repr(C)]
pub struct __kernel_sock_timeval {
    pub tv_sec: __s64,
    pub tv_usec: __s64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
