/*
 * Stack trace support for Microblaze.
 *
 * Copyright (C) 2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2009 PetaLogix
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the Linux kernel and Microblaze unwind code.
use core::ffi::c_char;

#[repr(C)]
pub struct stack_trace {
    pub skip: u32,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn microblaze_unwind(
        task: *mut task_struct,
        trace: *mut stack_trace,
        name: *const c_char,
    );
}

pub unsafe extern "C" fn save_stack_trace(trace: *mut stack_trace) {
    /* Exclude our helper functions from the trace*/
    (*trace).skip = (*trace).skip.wrapping_add(2);
    microblaze_unwind(core::ptr::null_mut(), trace, b"\0".as_ptr() as *const c_char);
}

pub unsafe extern "C" fn save_stack_trace_tsk(
    tsk: *mut task_struct,
    trace: *mut stack_trace,
) {
    microblaze_unwind(tsk, trace, b"\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
