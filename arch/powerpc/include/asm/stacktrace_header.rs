/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Stack trace functions.
 *
 * Copyright 2018, Murilo Opsfelder Araujo, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// `pt_regs`, `task_struct`, `current`, `current_stack_pointer`, and
// `THREAD_SIZE`.

extern "C" {
    pub fn show_user_instructions(regs: *mut pt_regs);
}

/// Corresponds to `struct pt_regs` from the C source.
#[allow(non_camel_case_types)]
pub type pt_regs = core::ffi::c_void;

/// Corresponds to `struct task_struct` from the C source.
#[allow(non_camel_case_types)]
pub type task_struct = core::ffi::c_void;

extern "C" {
    pub static mut current: *mut task_struct;
    pub static mut current_stack_pointer: usize;
}

#[inline(always)]
pub unsafe fn on_thread_stack() -> bool {
    let current_stack = *(current as *mut *mut core::ffi::c_void);
    !((((current_stack as usize) ^ current_stack_pointer)
        & !(THREAD_SIZE - 1)) != 0)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
