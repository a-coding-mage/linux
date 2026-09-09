/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Translated from the C header. The original include dependencies are supplied
// by other translated units.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn __switch_to(
        p: *mut task_struct,
        n: *mut task_struct,
    ) -> *mut task_struct;
}

// Corresponds to the C switch_to(prev, next, last) macro.
#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        unsafe {
            $crate::dsp_save_restore($prev, $next);
            $crate::fpu_save_restore($prev, $next);
            $last = $crate::__switch_to($prev, $next);
            $crate::mb();
        }
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
