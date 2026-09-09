/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding stack-trace declarations.
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct unwinder {
    pub name: *const c_char,
    pub list: list_head,
    pub rating: c_int,
    pub dump: Option<unsafe extern "C" fn(
        *mut task_struct,
        *mut pt_regs,
        *mut c_ulong,
        *const stacktrace_ops,
        *mut c_void,
    )>,
}

unsafe extern "C" {
    pub fn unwinder_init() -> c_int;
    pub fn unwinder_register(unwinder: *mut unwinder) -> c_int;

    pub fn unwind_stack(
        task: *mut task_struct,
        regs: *mut pt_regs,
        stack: *mut c_ulong,
        ops: *const stacktrace_ops,
        data: *mut c_void,
    );

    pub fn stack_reader_dump(
        task: *mut task_struct,
        regs: *mut pt_regs,
        stack: *mut c_ulong,
        ops: *const stacktrace_ops,
        data: *mut c_void,
    );
}

/*
 * Used by fault handling code to signal to the unwinder code that it
 * should switch to a different unwinder.
 */
pub static mut unwinder_faulted: c_int = 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
