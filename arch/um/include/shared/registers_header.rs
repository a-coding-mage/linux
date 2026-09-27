/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2004 PathScale, Inc
 */

// Dependency intent: declarations from <sysdep/ptrace.h> are supplied externally.

unsafe extern "C" {
    pub fn init_pid_registers(pid: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn get_safe_registers(
        regs: *mut kernel::ffi::c_ulong,
        fp_regs: *mut kernel::ffi::c_ulong,
    );
    pub fn get_fp_registers(
        pid: kernel::ffi::c_int,
        regs: *mut kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;
    pub fn put_fp_registers(
        pid: kernel::ffi::c_int,
        regs: *mut kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
