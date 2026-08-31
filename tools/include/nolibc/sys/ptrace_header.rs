/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * ptrace for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 * Copyright (C) 2025 Intel Corporation
 */

/* make sure to include all global symbols */
/* C dependency intent: #include "../nolibc.h" */

/* C header guard omitted: _NOLIBC_SYS_PTRACE_H */

/* C dependency intent: #include "../sys.h" */

/* C dependency intent: #include <linux/ptrace.h> */

/*
 * long ptrace(int op, pid_t pid, void *addr, void *data);
 */
#[allow(dead_code)]
pub unsafe fn _sys_ptrace(
    op: core::ffi::c_int,
    pid: pid_t,
    addr: *mut core::ffi::c_void,
    data: *mut core::ffi::c_void,
) -> core::ffi::c_long {
    unsafe { __nolibc_syscall4(__NR_ptrace, op, pid, addr, data) }
}

#[allow(dead_code)]
pub unsafe fn ptrace(
    op: core::ffi::c_int,
    pid: pid_t,
    addr: *mut core::ffi::c_void,
    data: *mut core::ffi::c_void,
) -> ssize_t {
    unsafe { __sysret(_sys_ptrace(op, pid, addr, data)) }
}
