// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

// Declarations supplied by <asm/unwind.h> and related kernel headers.
#[repr(C)]
pub struct unwind_state {
    pub pc: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn __unwind_get_return_address(state: *mut unwind_state) -> ::core::ffi::c_ulong;
    fn __unwind_start(
        state: *mut unwind_state,
        task: *mut task_struct,
        regs: *mut pt_regs,
    );
    fn unwind_done(state: *mut unwind_state) -> bool;
    fn __kernel_text_address(addr: ::core::ffi::c_ulong) -> bool;
    fn default_next_frame(state: *mut unwind_state) -> bool;
}

pub unsafe fn unwind_get_return_address(state: *mut unwind_state) -> ::core::ffi::c_ulong {
    unsafe { __unwind_get_return_address(state) }
}

// EXPORT_SYMBOL_GPL(unwind_get_return_address);

pub unsafe fn unwind_start(
    state: *mut unwind_state,
    task: *mut task_struct,
    regs: *mut pt_regs,
) {
    unsafe { __unwind_start(state, task, regs) };
    if unsafe { !unwind_done(state) } && unsafe { !__kernel_text_address((*state).pc) } {
        unsafe { unwind_next_frame(state) };
    }
}

// EXPORT_SYMBOL_GPL(unwind_start);

pub unsafe fn unwind_next_frame(state: *mut unwind_state) -> bool {
    unsafe { default_next_frame(state) }
}

// EXPORT_SYMBOL_GPL(unwind_next_frame);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
