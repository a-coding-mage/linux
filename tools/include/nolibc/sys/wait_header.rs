/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * wait definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C header included "../nolibc.h" here. */

/* C header included "../arch.h", "../std.h", and "../types.h" here. */

/*
 * pid_t wait(int *status);
 * pid_t waitpid(pid_t pid, int *status, int options);
 * int waitid(idtype_t idtype, id_t id, siginfo_t *infop, int options);
 */

pub unsafe fn _sys_waitid(
    which: i32,
    pid: pid_t,
    infop: *mut siginfo_t,
    options: i32,
    rusage: *mut rusage,
) -> i32 {
    unsafe { __nolibc_syscall5(__NR_waitid, which, pid, infop, options, rusage) as i32 }
}

pub unsafe fn waitid(which: i32, pid: pid_t, infop: *mut siginfo_t, options: i32) -> i32 {
    unsafe { __sysret(_sys_waitid(which, pid, infop, options, core::ptr::null_mut())) }
}

pub unsafe fn waitpid(pid: pid_t, status: *mut i32, mut options: i32) -> pid_t {
    let idtype: i32;
    let ret: i32;
    let mut info: siginfo_t = unsafe { core::mem::zeroed() };
    let id: pid_t;

    if pid == INT_MIN as pid_t {
        unsafe {
            SET_ERRNO(ESRCH);
        }
        return -1;
    } else if pid < -1 {
        idtype = P_PGID;
        id = -pid;
    } else if pid == -1 {
        idtype = P_ALL;
        id = 0;
    } else if pid == 0 {
        idtype = P_PGID;
        id = 0;
    } else {
        idtype = P_PID;
        id = pid;
    }

    options |= WEXITED;

    ret = unsafe { waitid(idtype, id, &mut info, options) };
    if ret != 0 {
        return -1;
    }

    match info.si_code {
        0 => {
            if !status.is_null() {
                unsafe {
                    *status = 0;
                }
            }
        }
        CLD_EXITED => {
            if !status.is_null() {
                unsafe {
                    *status = (info.si_status & 0xff) << 8;
                }
            }
        }
        CLD_KILLED => {
            if !status.is_null() {
                unsafe {
                    *status = info.si_status & 0x7f;
                }
            }
        }
        CLD_DUMPED => {
            if !status.is_null() {
                unsafe {
                    *status = (info.si_status & 0x7f) | 0x80;
                }
            }
        }
        CLD_STOPPED | CLD_TRAPPED => {
            if !status.is_null() {
                unsafe {
                    *status = (info.si_status << 8) + 0x7f;
                }
            }
        }
        CLD_CONTINUED => {
            if !status.is_null() {
                unsafe {
                    *status = 0xffff;
                }
            }
        }
        _ => {
            return -1;
        }
    }

    info.si_pid
}

pub unsafe fn wait(status: *mut i32) -> pid_t {
    unsafe { waitpid(-1, status, 0) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
