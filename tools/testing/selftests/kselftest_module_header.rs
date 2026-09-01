// SPDX-License-Identifier: GPL-2.0+

// Dependencies from the original header:
// #include <linux/module.h>
// #include <linux/panic.h>

/*
 * Test framework for writing test modules to be loaded by kselftest.
 * See Documentation/dev-tools/kselftest.rst for an example test module.
 */

unsafe extern "C" {
    fn pr_info(fmt: *const ::core::ffi::c_char, ...);
    fn pr_warn(fmt: *const ::core::ffi::c_char, ...);
    fn add_taint(flag: ::core::ffi::c_int, lockdep_ok: ::core::ffi::c_int);
}

pub const EINVAL: ::core::ffi::c_int = 22;

// External constants supplied by the Linux kernel headers.
unsafe extern "C" {
    static TAINT_TEST: ::core::ffi::c_int;
    static LOCKDEP_STILL_OK: ::core::ffi::c_int;
}

#[macro_export]
macro_rules! KSTM_MODULE_GLOBALS {
    () => {
        static mut total_tests: ::core::ffi::c_uint = 0;
        static mut failed_tests: ::core::ffi::c_uint = 0;
        static mut skipped_tests: ::core::ffi::c_uint = 0;
    };
}

#[macro_export]
macro_rules! KSTM_CHECK_ZERO {
    ($x:expr) => {{
        unsafe {
            total_tests = total_tests.wrapping_add(1);
            if $x != 0 {
                pr_warn(
                    c"TC failed at %s:%d\n".as_ptr(),
                    c"unknown".as_ptr(),
                    line!() as ::core::ffi::c_int,
                );
                failed_tests = failed_tests.wrapping_add(1);
            }
        }
    }};
}

#[inline]
pub unsafe fn kstm_report(
    total_tests: ::core::ffi::c_uint,
    failed_tests: ::core::ffi::c_uint,
    skipped_tests: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if failed_tests == 0 {
        if skipped_tests != 0 {
            unsafe {
                pr_info(c"skipped %u tests\n".as_ptr(), skipped_tests);
                pr_info(c"remaining %u tests passed\n".as_ptr(), total_tests);
            }
        } else {
            unsafe {
                pr_info(c"all %u tests passed\n".as_ptr(), total_tests);
            }
        }
    } else {
        unsafe {
            pr_warn(
                c"failed %u out of %u tests\n".as_ptr(),
                failed_tests,
                total_tests,
            );
        }
    }

    if failed_tests != 0 {
        -EINVAL
    } else {
        0
    }
}

#[macro_export]
macro_rules! KSTM_MODULE_LOADERS {
    ($module:ident) => {
        paste::paste! {
            unsafe extern "C" fn [<$module _init>]() -> ::core::ffi::c_int {
                unsafe {
                    pr_info(c"loaded.\n".as_ptr());
                    add_taint(TAINT_TEST, LOCKDEP_STILL_OK);
                    selftest();
                    kstm_report(total_tests, failed_tests, skipped_tests)
                }
            }

            unsafe extern "C" fn [<$module _exit>]() {
                unsafe {
                    pr_info(c"unloaded.\n".as_ptr());
                }
            }

            module_init!([<$module _init>]);
            module_exit!([<$module _exit>]);
        }
    };
}

module_info!(test, "Y");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
