/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the corresponding architecture headers:
// asm/cpu-features.h, asm/fpu.h, and asm/lbt.h.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/**
 * __switch_to - switch execution of a task
 * @prev:    The task previously executed.
 * @next:    The task to begin executing.
 * @sched_ra:    __schedule return address.
 * @sched_cfa:   __schedule call frame address.
 *
 * This function is used whilst scheduling to save the context of prev & load
 * the context of next. Returns prev.
 */
extern "C" {
    pub fn __switch_to(
        prev: *mut task_struct,
        next: *mut task_struct,
        sched_ra: *mut core::ffi::c_void,
        sched_cfa: *mut core::ffi::c_void,
    ) -> *mut task_struct;

    pub fn lose_fpu_inatomic(n: i32, task: *mut task_struct);
    pub fn lose_lbt_inatomic(n: i32, task: *mut task_struct);
    pub fn hw_breakpoint_thread_switch(task: *mut task_struct);
    pub fn set_current(task: *mut task_struct);

    // Compiler builtins used by the C macro; declarations preserve their
    // source-level intent for the translated interface.
    pub fn __builtin_return_address(level: i32) -> *mut core::ffi::c_void;
    pub fn __builtin_frame_address(level: i32) -> *mut core::ffi::c_void;
}

/*
 * For newly created kernel threads switch_to() will return to
 * ret_from_kernel_thread, newly created user threads to ret_from_fork.
 * That is, everything following __switch_to() will be skipped for new threads.
 * So everything that matters to new threads should be placed before __switch_to().
 */
#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        unsafe {
            $crate::lose_fpu_inatomic(1, $prev);
            $crate::lose_lbt_inatomic(1, $prev);
            $crate::hw_breakpoint_thread_switch($next);
            $crate::set_current($next);
            $last = $crate::__switch_to(
                $prev,
                $next,
                $crate::__builtin_return_address(0),
                $crate::__builtin_frame_address(0),
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
