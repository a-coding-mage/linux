/* SPDX-License-Identifier: GPL-2.0 */

// __PARISC_SWITCH_TO_H

#[repr(C)]
pub struct task_struct;

extern "C" {
    pub fn _switch_to(
        prev: *mut task_struct,
        next: *mut task_struct,
    ) -> *mut task_struct;
}

macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        $last = unsafe { _switch_to($prev, $next) };
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
