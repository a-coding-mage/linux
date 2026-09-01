/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * sched function definitions for NOLIBC
 * Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
 */

/* C header dependencies removed: "nolibc.h", "sys.h", and <linux/sched.h>. */

/*
 * int setns(int fd, int nstype);
 */

#[allow(dead_code)]
pub unsafe fn _sys_setns(fd: i32, nstype: i32) -> i32 {
    __nolibc_syscall2(__NR_setns, fd, nstype) as i32
}

#[allow(dead_code)]
pub unsafe fn setns(fd: i32, nstype: i32) -> i32 {
    __sysret(_sys_setns(fd, nstype))
}

/*
 * int unshare(int flags);
 */

#[allow(dead_code)]
pub unsafe fn _sys_unshare(flags: i32) -> i32 {
    __nolibc_syscall1(__NR_unshare, flags) as i32
}

#[allow(dead_code)]
pub unsafe fn unshare(flags: i32) -> i32 {
    __sysret(_sys_unshare(flags))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
