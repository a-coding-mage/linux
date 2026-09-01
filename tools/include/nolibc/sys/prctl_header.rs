/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Prctl definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C include dependency: "../nolibc.h" */

/* Header guard omitted: _NOLIBC_SYS_PRCTL_H */

/* C include dependencies: "../sys.h", <linux/prctl.h> */

/*
 * int prctl(int option, unsigned long arg2, unsigned long arg3,
 *                       unsigned long arg4, unsigned long arg5);
 */

pub unsafe fn _sys_prctl(
    option: core::ffi::c_int,
    arg2: core::ffi::c_ulong,
    arg3: core::ffi::c_ulong,
    arg4: core::ffi::c_ulong,
    arg5: core::ffi::c_ulong,
) -> core::ffi::c_int {
    unsafe { __nolibc_syscall5(__NR_prctl, option, arg2, arg3, arg4, arg5) as core::ffi::c_int }
}

pub unsafe fn prctl(
    option: core::ffi::c_int,
    arg2: core::ffi::c_ulong,
    arg3: core::ffi::c_ulong,
    arg4: core::ffi::c_ulong,
    arg5: core::ffi::c_ulong,
) -> core::ffi::c_int {
    unsafe { __sysret(_sys_prctl(option, arg2, arg3, arg4, arg5)) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
