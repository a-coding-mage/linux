/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C header guard: _ASM_MICROBLAZE_SWITCH_TO_H

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_info {
    _private: [u8; 0],
}

extern "C" {
    pub fn _switch_to(
        prev: *mut thread_info,
        next: *mut thread_info,
    ) -> *mut task_struct;
}

// External dependency supplied by another translation unit/header.
extern "C" {
    pub fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        $last = unsafe {
            $crate::_switch_to(
                $crate::task_thread_info($prev),
                $crate::task_thread_info($next),
            )
        };
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
