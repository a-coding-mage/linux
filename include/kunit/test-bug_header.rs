/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit API providing hooks for non-test code to interact with tests.
 *
 * Copyright (C) 2020, Google LLC.
 * Author: Uriel Guajardo <urielguajardo@google.com>
 */

/* The C header guard and include directives are not emitted as Rust code. */

use core::ffi::{c_char, c_int, c_void};

/* Equivalent build-time condition for IS_ENABLED(CONFIG_KUNIT). */
#[cfg(feature = "CONFIG_KUNIT")]
mod kunit_enabled {
    use super::*;

    /* Symbols and types supplied by the surrounding kernel translation. */
    #[repr(C)]
    pub struct kunit;
    #[repr(C)]
    pub struct static_key_false;
    #[repr(C)]
    pub struct task_struct {
        pub kunit_test: *mut kunit,
    }

    extern "C" {
        pub static kunit_running: static_key_false;
        pub static mut current: *mut task_struct;
        pub fn static_branch_unlikely(key: *const static_key_false) -> bool;
    }

    /* Hooks table: a table of function pointers filled in when kunit loads. */
    #[repr(C)]
    pub struct kunit_hooks_table {
        pub fail_current_test:
            Option<unsafe extern "C" fn(*const c_char, c_int, *const c_char, ...)> ,
        pub get_static_stub_address:
            Option<unsafe extern "C" fn(*mut kunit, *mut c_void) -> *mut c_void>,
        pub is_suppressed_warning: Option<unsafe extern "C" fn(bool) -> bool>,
    }

    extern "C" {
        pub static mut kunit_hooks: kunit_hooks_table;
    }

    /**
     * Return a pointer to the currently running KUnit test.
     *
     * If a KUnit test is running in the current task, returns a pointer to its
     * associated struct kunit. If no test is running, returns NULL.
     */
    #[inline]
    pub unsafe fn kunit_get_current_test() -> *mut kunit {
        if !static_branch_unlikely(&kunit_running) {
            core::ptr::null_mut()
        } else {
            (*current).kunit_test
        }
    }

    /**
     * If a KUnit test is running, fail it.
     */
    #[macro_export]
    macro_rules! kunit_fail_current_test {
        ($fmt:expr $(, $arg:expr)*) => {{
            if unsafe { $crate::kunit_enabled::static_branch_unlikely(
                &$crate::kunit_enabled::kunit_running
            ) } {
                unsafe {
                    if let Some(fail) = $crate::kunit_enabled::kunit_hooks.fail_current_test {
                        fail(file!().as_ptr() as *const core::ffi::c_char,
                             line!() as core::ffi::c_int,
                             $fmt $(, $arg)*);
                    }
                }
            }
        }};
    }

    /**
     * Check if warnings are being suppressed by the current KUnit test.
     */
    #[inline]
    pub unsafe fn kunit_is_suppressed_warning(count: bool) -> bool {
        if !static_branch_unlikely(&kunit_running) {
            return false;
        }

        match kunit_hooks.is_suppressed_warning {
            Some(is_suppressed) => is_suppressed(count),
            None => false,
        }
    }
}

/* When CONFIG_KUNIT is disabled, these inline functions return immediately. */
#[cfg(not(feature = "CONFIG_KUNIT"))]
pub unsafe fn kunit_get_current_test() -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_KUNIT"))]
pub unsafe fn kunit_is_suppressed_warning(_count: bool) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_KUNIT"))]
#[macro_export]
macro_rules! kunit_fail_current_test {
    ($fmt:expr $(, $arg:expr)*) => {{}};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
