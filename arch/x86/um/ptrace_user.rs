/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// Dependency declarations from <errno.h> and <ptrace_user.h> are supplied by
// the surrounding translation unit/build environment.
use core::ffi::{c_int, c_long, c_ulong, c_void};

unsafe extern "C" {
    fn ptrace(
        request: c_ulong,
        pid: c_long,
        addr: *mut c_void,
        data: *mut c_void,
    ) -> c_long;
    static mut errno: c_int;
}

pub unsafe fn ptrace_getregs(pid: c_long, regs_out: *mut c_ulong) -> c_int {
    if ptrace(
        PTRACE_GETREGS,
        pid,
        core::ptr::null_mut(),
        regs_out.cast::<c_void>(),
    ) < 0
    {
        return -errno;
    }
    0
}

pub unsafe fn ptrace_setregs(pid: c_long, regs: *mut c_ulong) -> c_int {
    if ptrace(
        PTRACE_SETREGS,
        pid,
        core::ptr::null_mut(),
        regs.cast::<c_void>(),
    ) < 0
    {
        return -errno;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
