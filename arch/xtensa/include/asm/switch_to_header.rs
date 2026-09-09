/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

/*
 * switch_to(n) should switch tasks to task nr n, first
 * checking that n isn't the current task, in which case it does nothing.
 */
extern "C" {
    pub fn _switch_to(
        last: *mut core::ffi::c_void,
        next: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {
        $last = unsafe { $crate::_switch_to($prev, $next) };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
