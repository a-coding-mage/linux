/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * timerfd definitions for NOLIBC
 * Copyright (C) 2025 Thomas Weißschuh <thomas.weissschuh@linutronix.de>
 */

/* make sure to include all global symbols */
/* C include dependency: "../nolibc.h" */

/* C header guard removed: _NOLIBC_SYS_TIMERFD_H */

/* C include dependencies: "../sys.h", "../time.h", <linux/timerfd.h> */

#[allow(dead_code)]
pub unsafe fn _sys_timerfd_create(clockid: i32, flags: i32) -> i32 {
    unsafe { __nolibc_syscall2(__NR_timerfd_create, clockid, flags) as i32 }
}

#[allow(dead_code)]
pub unsafe fn timerfd_create(clockid: i32, flags: i32) -> i32 {
    unsafe { __sysret(_sys_timerfd_create(clockid, flags)) }
}

#[allow(dead_code)]
pub unsafe fn _sys_timerfd_gettime(fd: i32, curr_value: *mut itimerspec) -> i32 {
    /*
     * Original C condition:
     * #if defined(__NR_timerfd_gettime64)
     */
    #[cfg(__NR_timerfd_gettime64)]
    {
        unsafe {
            __nolibc_assert_time64_type((*curr_value).it_value.tv_sec);
            return __nolibc_syscall2(__NR_timerfd_gettime64, fd, curr_value) as i32;
        }
    }

    /*
     * Original C fallback:
     * #else
     */
    #[cfg(not(__NR_timerfd_gettime64))]
    {
        unsafe {
            __nolibc_assert_native_time64();
            return __nolibc_syscall2(__NR_timerfd_gettime, fd, curr_value) as i32;
        }
    }
}

#[allow(dead_code)]
pub unsafe fn timerfd_gettime(fd: i32, curr_value: *mut itimerspec) -> i32 {
    unsafe { __sysret(_sys_timerfd_gettime(fd, curr_value)) }
}

#[allow(dead_code)]
pub unsafe fn _sys_timerfd_settime(
    fd: i32,
    flags: i32,
    new_value: *const itimerspec,
    old_value: *mut itimerspec,
) -> i32 {
    /*
     * Original C condition:
     * #if defined(__NR_timerfd_settime64)
     */
    #[cfg(__NR_timerfd_settime64)]
    {
        unsafe {
            __nolibc_assert_time64_type((*new_value).it_value.tv_sec);
            return __nolibc_syscall4(__NR_timerfd_settime64, fd, flags, new_value, old_value)
                as i32;
        }
    }

    /*
     * Original C fallback:
     * #else
     */
    #[cfg(not(__NR_timerfd_settime64))]
    {
        unsafe {
            __nolibc_assert_native_time64();
            return __nolibc_syscall4(__NR_timerfd_settime, fd, flags, new_value, old_value)
                as i32;
        }
    }
}

#[allow(dead_code)]
pub unsafe fn timerfd_settime(
    fd: i32,
    flags: i32,
    new_value: *const itimerspec,
    old_value: *mut itimerspec,
) -> i32 {
    unsafe { __sysret(_sys_timerfd_settime(fd, flags, new_value, old_value)) }
}
