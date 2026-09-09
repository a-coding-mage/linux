/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies: <sys/ptrace.h> and <sysdep/ptrace_user.h>.

use std::os::raw::{c_int, c_long, c_ulong};

extern "C" {
    pub fn ptrace_getregs(pid: c_long, regs_out: *mut c_ulong) -> c_int;
    pub fn ptrace_setregs(pid: c_long, regs_in: *mut c_ulong) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
