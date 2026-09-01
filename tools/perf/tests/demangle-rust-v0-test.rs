// SPDX-License-Identifier: Apache-2.0 OR MIT
// Translated from C. Original dependencies:
// "tests.h", "debug.h", "symbol.h", <linux/kernel.h>, <stdlib.h>, <string.h>

use core::ffi::{c_char, c_int, c_void};

// External declarations supplied by the surrounding perf test harness.
#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn dso__demangle_sym(dso: *mut c_void, kmodule: c_int, sym: *const c_char) -> *mut c_char;
    fn pr_debug(fmt: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
}

const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;

#[repr(C)]
struct TestCase {
    mangled: *const c_char,
    demangled: *const c_char,
}

unsafe extern "C" fn test__demangle_rust(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut ret: c_int = TEST_OK;
    let mut buf: *mut c_char;

    let test_cases = [
        TestCase {
            mangled: c"_RNvMsr_NtCs3ssYzQotkvD_3std4pathNtB5_7PathBuf3newCs15kBYyAo9fc_7mycrate".as_ptr(),
            demangled: c"<std::path::PathBuf>::new".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvCs15kBYyAo9fc_7mycrate7example".as_ptr(),
            demangled: c"mycrate::example".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvMs_Cs4Cv8Wi1oAIB_7mycrateNtB4_7Example3foo".as_ptr(),
            demangled: c"<mycrate::Example>::foo".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvXCs15kBYyAo9fc_7mycrateNtB2_7ExampleNtB2_5Trait3foo".as_ptr(),
            demangled: c"<mycrate::Example as mycrate::Trait>::foo".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvMCs7qp2U7fqm6G_7mycrateNtB2_7Example3foo".as_ptr(),
            demangled: c"<mycrate::Example>::foo".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvMs_Cs7qp2U7fqm6G_7mycrateNtB4_7Example3bar".as_ptr(),
            demangled: c"<mycrate::Example>::bar".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvYNtCs15kBYyAo9fc_7mycrate7ExampleNtB4_5Trait7exampleB4_".as_ptr(),
            demangled: c"<mycrate::Example as mycrate::Trait>::example".as_ptr(),
        },
        TestCase {
            mangled: c"_RNCNvCsgStHSCytQ6I_7mycrate4main0B3_".as_ptr(),
            demangled: c"mycrate::main::{closure#0}".as_ptr(),
        },
        TestCase {
            mangled: c"_RNCNvCsgStHSCytQ6I_7mycrate4mains_0B3_".as_ptr(),
            demangled: c"mycrate::main::{closure#1}".as_ptr(),
        },
        TestCase {
            mangled: c"_RINvCsgStHSCytQ6I_7mycrate7examplelKj1_EB2_".as_ptr(),
            demangled: c"mycrate::example::<i32, 1>".as_ptr(),
        },
        TestCase {
            mangled: c"_RINvCs7qp2U7fqm6G_7mycrate7exampleFG0_RL1_hRL0_tEuEB2_".as_ptr(),
            demangled: c"mycrate::example::<for<'a, 'b> fn(&'a u8, &'b u16)>".as_ptr(),
        },
        TestCase {
            mangled: c"_RINvCs7qp2U7fqm6G_7mycrate7exampleKy12345678_EB2_".as_ptr(),
            demangled: c"mycrate::example::<305419896>".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvNvMCsd9PVOYlP1UU_7mycrateINtB4_7ExamplepKpE3foo14EXAMPLE_STATIC".as_ptr(),
            demangled: c"<mycrate::Example<_, _>>::foo::EXAMPLE_STATIC".as_ptr(),
        },
        TestCase {
            mangled: c"_RINvCs7qp2U7fqm6G_7mycrate7exampleAtj8_EB2_".as_ptr(),
            demangled: c"mycrate::example::<[u16; 8]>".as_ptr(),
        },
        TestCase {
            mangled: c"_RINvCs7qp2U7fqm6G_7mycrate7exampleNtB2_7ExampleBw_EB2_".as_ptr(),
            demangled: c"mycrate::example::<mycrate::Example, mycrate::Example>".as_ptr(),
        },
        TestCase {
            mangled: c"_RINvMsY_NtCseXNvpPnDBDp_3std4pathNtB6_4Path3neweECs7qp2U7fqm6G_7mycrate".as_ptr(),
            demangled: c"<std::path::Path>::new::<str>".as_ptr(),
        },
        TestCase {
            mangled: c"_RNvNvNvCs7qp2U7fqm6G_7mycrate7EXAMPLE7___getit5___KEY".as_ptr(),
            demangled: c"mycrate::EXAMPLE::__getit::__KEY".as_ptr(),
        },
    ];

    for i in 0..test_cases.len() {
        buf = dso__demangle_sym(
            core::ptr::null_mut(),
            0,
            test_cases[i].mangled,
        );
        if buf.is_null() {
            pr_debug(
                c"FAILED to demangle: \"%s\"\n \"%s\"\n".as_ptr(),
                test_cases[i].mangled,
                test_cases[i].demangled,
            );
            continue;
        }
        if strcmp(buf, test_cases[i].demangled) != 0 {
            pr_debug(
                c"FAILED: %s: %s != %s\n".as_ptr(),
                test_cases[i].mangled,
                buf,
                test_cases[i].demangled,
            );
            ret = TEST_FAIL;
        }
        free(buf.cast::<c_void>());
    }

    ret
}

// C source registers this with:
// DEFINE_SUITE("Demangle Rust", demangle_rust);
// The Rust-side equivalent depends on the surrounding test harness macro support.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
