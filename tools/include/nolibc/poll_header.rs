/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * poll definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C included "nolibc.h", "arch.h", "sys.h", <linux/poll.h>, and <linux/time.h>. */

/*
 * int poll(struct pollfd *fds, int nfds, int timeout);
 */

#[allow(dead_code)]
pub unsafe fn _sys_poll(fds: *mut pollfd, nfds: i32, timeout: i32) -> i32 {
    /*
     * C preprocessor condition preserved:
     * #if defined(__NR_ppoll_time64)
     */
    #[cfg(__NR_ppoll_time64)]
    {
        let mut t: __kernel_timespec = core::mem::zeroed();

        if timeout >= 0 {
            t.tv_sec = timeout / 1000;
            t.tv_nsec = (timeout % 1000) * 1000000;
        }
        return __nolibc_syscall5(
            __NR_ppoll_time64,
            fds,
            nfds,
            if timeout >= 0 {
                &mut t as *mut __kernel_timespec
            } else {
                core::ptr::null_mut()
            },
            core::ptr::null_mut::<core::ffi::c_void>(),
            0,
        );
    }

    /*
     * C preprocessor fallback preserved:
     * #else
     */
    #[cfg(not(__NR_ppoll_time64))]
    {
        let mut t: __kernel_old_timespec = core::mem::zeroed();

        if timeout >= 0 {
            t.tv_sec = timeout / 1000;
            t.tv_nsec = (timeout % 1000) * 1000000;
        }
        return __nolibc_syscall5(
            __NR_ppoll,
            fds,
            nfds,
            if timeout >= 0 {
                &mut t as *mut __kernel_old_timespec
            } else {
                core::ptr::null_mut()
            },
            core::ptr::null_mut::<core::ffi::c_void>(),
            0,
        );
    }
}

#[allow(dead_code)]
pub unsafe fn poll(fds: *mut pollfd, nfds: i32, timeout: i32) -> i32 {
    return __sysret(_sys_poll(fds, nfds, timeout));
}
