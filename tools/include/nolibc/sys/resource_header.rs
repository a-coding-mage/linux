/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Resource definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/*
 * Original C header dependencies:
 * - "../nolibc.h" to make sure to include all global symbols
 * - "../sys.h"
 * - <linux/resource.h>
 *
 * Header guard _NOLIBC_SYS_RESOURCE_H omitted in Rust.
 */

/*
 * int getrlimit(int resource, struct rlimit *rlim);
 * int setrlimit(int resource, const struct rlimit *rlim);
 */

unsafe extern "C" {
    static __NR_prlimit64: core::ffi::c_long;

    fn __nolibc_syscall4(
        nr: core::ffi::c_long,
        arg1: pid_t,
        arg2: core::ffi::c_int,
        arg3: *const rlimit64,
        arg4: *mut rlimit64,
    ) -> core::ffi::c_long;

    fn __sysret(ret: core::ffi::c_long) -> core::ffi::c_int;
}

#[allow(dead_code)]
pub unsafe fn _sys_prlimit64(
    pid: pid_t,
    resource: core::ffi::c_int,
    new_limit: *const rlimit64,
    old_limit: *mut rlimit64,
) -> core::ffi::c_int {
    unsafe {
        __nolibc_syscall4(__NR_prlimit64, pid, resource, new_limit, old_limit)
            as core::ffi::c_int
    }
}

#[allow(dead_code)]
pub unsafe fn getrlimit(resource: core::ffi::c_int, rlim: *mut rlimit) -> core::ffi::c_int {
    let mut rlim64: rlimit64;
    let ret: core::ffi::c_int;

    unsafe {
        rlim64 = core::mem::zeroed();
        ret = __sysret(_sys_prlimit64(
            core::mem::zeroed(),
            resource,
            core::ptr::null(),
            &mut rlim64,
        ) as core::ffi::c_long);
        (*rlim).rlim_cur = rlim64.rlim_cur;
        (*rlim).rlim_max = rlim64.rlim_max;
    }

    ret
}

#[allow(dead_code)]
pub unsafe fn setrlimit(resource: core::ffi::c_int, rlim: *const rlimit) -> core::ffi::c_int {
    let rlim64 = unsafe {
        rlimit64 {
            rlim_cur: (*rlim).rlim_cur,
            rlim_max: (*rlim).rlim_max,
        }
    };

    unsafe {
        __sysret(_sys_prlimit64(
            core::mem::zeroed(),
            resource,
            &rlim64,
            core::ptr::null_mut(),
        ) as core::ffi::c_long)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
