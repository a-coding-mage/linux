/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *	include/linux/kmod.h
 *
 * C header dependencies are supplied by other translated files.
 */

use core::ffi::{c_char, c_int};

/* CONFIG_MODULES is a build-time condition from the original header. */
#[cfg(CONFIG_MODULES)]
extern "C" {
    /* modprobe exit status on success, -ve on error.  Return value
     * usually useless though. */
    pub fn __request_module(wait: bool, name: *const c_char, ...) -> c_int;
}

#[cfg(CONFIG_MODULES)]
#[macro_export]
macro_rules! request_module {
    ($($mod:expr),* $(,)?) => {
        unsafe { $crate::__request_module(true, $($mod),*) }
    };
}

#[cfg(CONFIG_MODULES)]
#[macro_export]
macro_rules! request_module_nowait {
    ($($mod:expr),* $(,)?) => {
        unsafe { $crate::__request_module(false, $($mod),*) }
    };
}

#[cfg(CONFIG_MODULES)]
#[macro_export]
macro_rules! try_then_request_module {
    ($x:expr, $($mod:expr),* $(,)?) => {{
        let value = $x;
        if value != 0 {
            value
        } else {
            unsafe { $crate::__request_module(true, $($mod),*) };
            value
        }
    }};
}

#[cfg(not(CONFIG_MODULES))]
#[inline]
pub unsafe fn request_module(_name: *const c_char, ...) -> c_int {
    -ENOSYS
}

#[cfg(not(CONFIG_MODULES))]
#[inline]
pub unsafe fn request_module_nowait(_name: *const c_char, ...) -> c_int {
    -ENOSYS
}

#[cfg(not(CONFIG_MODULES))]
#[macro_export]
macro_rules! try_then_request_module {
    ($x:expr, $($mod:expr),* $(,)?) => {
        $x
    };
}

/* Supplied by the translated errno dependency. */
#[cfg(not(CONFIG_MODULES))]
const ENOSYS: c_int = 38;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
