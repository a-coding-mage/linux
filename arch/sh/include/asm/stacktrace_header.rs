/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2009  Matt Fleming
 *
 * Based on:
 *	The x86 implementation - arch/x86/include/asm/stacktrace.h
 */

/* Generic stack tracer with callbacks */

#[repr(C)]
pub struct stacktrace_ops {
    pub address: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void,
                                               address: libc::c_ulong,
                                               reliable: libc::c_int)>,
}

extern "C" {
    pub fn dump_trace(
        tsk: *mut task_struct,
        regs: *mut pt_regs,
        stack: *mut libc::c_ulong,
        ops: *const stacktrace_ops,
        data: *mut core::ffi::c_void,
    );
}

/* External types supplied by other translation units. */
pub enum task_struct {}
pub enum pt_regs {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
