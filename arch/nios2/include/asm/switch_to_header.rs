/*
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/*
 * switch_to(n) should switch tasks to task ptr, first checking that
 * ptr isn't the current task, in which case it does nothing.  This
 * also clears the TS-flag if the task we switched to has used the
 * math co-processor latest.
 */

use core::ffi::c_void;

extern "C" {
    pub fn resume(prev: *mut c_void, next: *mut c_void) -> *mut c_void;
}

/// Switch tasks using the Nios II `resume` routine.
///
/// This preserves the C macro's output assignment through `last`.
#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        let _last: *mut core::ffi::c_void = unsafe {
            // C source:
            //   mov r4, prev
            //   mov r5, next
            //   call resume
            //   mov _last, r4
            // The register-level implementation is provided by the external
            // Nios II `resume` routine and follows the platform ABI.
            $crate::resume(
                ($prev) as *mut core::ffi::c_void,
                ($next) as *mut core::ffi::c_void,
            )
        };
        ($last) = _last;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
