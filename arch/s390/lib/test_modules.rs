// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the kernel KUnit and module headers are intentionally
// left external to this translation.

use core::ffi::c_int;

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

extern "C" {
    // The test_modules.h dependency supplies test_modules_return_0 through
    // test_modules_return_9999 and the REPEAT_10000 macro used below.
}

/*
 * Test that modules with many relocations are loaded properly.
 */
unsafe fn test_modules_many_vmlinux_relocs(test: *mut kunit) {
    let mut result: c_int = 0;

    // C: #define CALL_RETURN(i) result += test_modules_return_ ## i()
    // REPEAT_10000 and the generated test_modules_return_* symbols are supplied
    // by the external test_modules.h dependency.
    REPEAT_10000!(CALL_RETURN);
    KUNIT_ASSERT_EQ!(test, result, 49995000);
}

// static struct kunit_case modules_testcases[] = {
//     KUNIT_CASE(test_modules_many_vmlinux_relocs),
//     {}
// };
//
// static struct kunit_suite modules_test_suite = {
//     .name = "modules_test_s390",
//     .test_cases = modules_testcases,
// };
//
// kunit_test_suites(&modules_test_suite);
//
// MODULE_DESCRIPTION("KUnit test that modules with many relocations are loaded properly");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
