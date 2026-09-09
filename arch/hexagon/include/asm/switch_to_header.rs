/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Task switching definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C header guard: _ASM_SWITCH_TO_H

pub struct thread_struct;
pub struct task_struct;

extern "C" {
    pub fn __switch_to(
        prev: *mut task_struct,
        next: *mut task_struct,
        last: *mut task_struct,
    ) -> *mut task_struct;
}

#[macro_export]
macro_rules! switch_to {
    ($p:expr, $n:expr, $r:ident) => {{
        $r = unsafe { $crate::__switch_to(($p), ($n), ($r)) };
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
