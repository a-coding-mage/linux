/*
 * arch/xtensa/include/asm/stacktrace.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2013 Tensilica Inc.
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

// Supplied by the scheduler and register definitions.
extern "C" {
    pub static mut current: *mut task_struct;
    pub static mut current_stack_pointer: c_ulong;
}

pub enum task_struct {}
pub enum pt_regs {}

#[repr(C)]
pub struct stackframe {
    pub pc: c_ulong,
    pub sp: c_ulong,
}

#[inline(always)]
pub unsafe fn stack_pointer(task: *mut task_struct) -> *mut c_ulong {
    let sp: c_ulong;

    if task.is_null() || task == current {
        sp = current_stack_pointer;
    } else {
        // The `task_struct::thread.sp` field is supplied by the scheduler
        // dependency represented by `task_struct` above.
        sp = (*task).thread.sp;
    }

    sp as *mut c_ulong
}

extern "C" {
    pub fn walk_stackframe(
        sp: *mut c_ulong,
        fn_: Option<unsafe extern "C" fn(frame: *mut stackframe, data: *mut c_void) -> c_int>,
        data: *mut c_void,
    );

    pub fn xtensa_backtrace_kernel(
        regs: *mut pt_regs,
        depth: c_uint,
        kfn: Option<unsafe extern "C" fn(frame: *mut stackframe, data: *mut c_void) -> c_int>,
        ufn: Option<unsafe extern "C" fn(frame: *mut stackframe, data: *mut c_void) -> c_int>,
        data: *mut c_void,
    );

    pub fn xtensa_backtrace_user(
        regs: *mut pt_regs,
        depth: c_uint,
        ufn: Option<unsafe extern "C" fn(frame: *mut stackframe, data: *mut c_void) -> c_int>,
        data: *mut c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
