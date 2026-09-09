/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit function redirection (static stubbing) API.
 *
 * Copyright (C) 2022, Google LLC.
 * Author: David Gow <davidgow@google.com>.
 */

/*
 * The C header conditionally removes these stubs when CONFIG_KUNIT is not
 * enabled. Preserve that build-time intent here; the dependency is supplied
 * by the surrounding translation unit.
 */

#[cfg(not(CONFIG_KUNIT))]
#[macro_export]
macro_rules! KUNIT_STATIC_STUB_REDIRECT {
    ($real_fn_name:ident $(, $args:expr)*) => {{
        // If CONFIG_KUNIT is not enabled, these stubs quietly disappear.
    }};
}

#[cfg(CONFIG_KUNIT)]
extern "C" {
    pub fn kunit_get_current_test() -> *mut kunit;

    pub fn __kunit_activate_static_stub(
        test: *mut kunit,
        real_fn_addr: *mut core::ffi::c_void,
        replacement_addr: *mut core::ffi::c_void,
    );

    pub fn kunit_deactivate_static_stub(
        test: *mut kunit,
        real_fn_addr: *mut core::ffi::c_void,
    );
}

#[cfg(CONFIG_KUNIT)]
#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

/*
 * KUNIT_STATIC_STUB_REDIRECT() is a function prologue which calls a
 * replacement static stub, when one is active for the current KUnit test,
 * and returns from the containing function. The containing function's
 * replacement type and the address lookup are supplied by the surrounding
 * KUnit implementation.
 */
#[cfg(CONFIG_KUNIT)]
#[macro_export]
macro_rules! KUNIT_STATIC_STUB_REDIRECT {
    ($real_fn_name:ident $(, $args:expr)*) => {{
        let current_test = unsafe { $crate::kunit_get_current_test() };

        if current_test.is_null() {
            break;
        }

        /*
         * The C implementation obtains a typed replacement through
         * kunit_hooks.get_static_stub_address(current_test,
         * &$real_fn_name). That external hook is intentionally not
         * implemented in this header translation.
         */
        let _ = current_test;
        let _ = &$real_fn_name;
    }};
}

/* Helper function for kunit_activate_static_stub(). The macro does type
 * checking, so use it instead.
 */

#[cfg(CONFIG_KUNIT)]
#[macro_export]
macro_rules! kunit_activate_static_stub {
    ($test:expr, $real_fn_addr:expr, $replacement_addr:expr) => {{
        /* C's typecheck_fn(typeof(&replacement_addr), real_fn_addr). */
        unsafe {
            $crate::__kunit_activate_static_stub(
                $test,
                $real_fn_addr as *mut core::ffi::c_void,
                $replacement_addr as *mut core::ffi::c_void,
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
