// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/tests/demangle-java-test.c.
// Original C dependencies: string.h, stdlib.h, stdio.h, linux/kernel.h,
// debug.h, symbol.h, tests.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    static TEST_OK: c_int;
    static TEST_FAIL: c_int;

    fn dso__demangle_sym(
        dso: *mut c_void,
        kmodule: c_int,
        sym: *const c_char,
    ) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn pr_debug(fmt: *const c_char, ...);
}

#[repr(C)]
struct TestCase {
    mangled: *const c_char,
    demangled: *const c_char,
}

unsafe extern "C" fn test__demangle_java(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    let mut ret: c_int = unsafe { TEST_OK };
    let mut buf: *mut c_char = core::ptr::null_mut();
    let mut i: usize;

    let test_cases = [
        TestCase {
            mangled: c"Ljava/lang/StringLatin1;equals([B[B)Z".as_ptr(),
            demangled: c"java.lang.StringLatin1.equals(byte[], byte[])".as_ptr(),
        },
        TestCase {
            mangled: c"Ljava/util/zip/ZipUtils;CENSIZ([BI)J".as_ptr(),
            demangled: c"java.util.zip.ZipUtils.CENSIZ(byte[], int)".as_ptr(),
        },
        TestCase {
            mangled: c"Ljava/util/regex/Pattern$BmpCharProperty;match(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z".as_ptr(),
            demangled: c"java.util.regex.Pattern$BmpCharProperty.match(java.util.regex.Matcher, int, java.lang.CharSequence)".as_ptr(),
        },
        TestCase {
            mangled: c"Ljava/lang/AbstractStringBuilder;appendChars(Ljava/lang/String;II)V".as_ptr(),
            demangled: c"java.lang.AbstractStringBuilder.appendChars(java.lang.String, int, int)".as_ptr(),
        },
        TestCase {
            mangled: c"Ljava/lang/Object;<init>()V".as_ptr(),
            demangled: c"java.lang.Object<init>()".as_ptr(),
        },
    ];

    i = 0;
    while i < test_cases.len() {
        buf = unsafe {
            dso__demangle_sym(
                core::ptr::null_mut(), /*dso=*/
                0,                     /*kmodule=*/
                test_cases[i].mangled,
            )
        };
        if buf.is_null() {
            unsafe {
                pr_debug(
                    c"FAILED to demangle: \"%s\"\n \"%s\"\n".as_ptr(),
                    test_cases[i].mangled,
                    test_cases[i].demangled,
                );
            }
            i += 1;
            continue;
        }
        if unsafe { strcmp(buf, test_cases[i].demangled) } != 0 {
            unsafe {
                pr_debug(
                    c"FAILED: %s: %s != %s\n".as_ptr(),
                    test_cases[i].mangled,
                    buf,
                    test_cases[i].demangled,
                );
            }
            ret = unsafe { TEST_FAIL };
        }
        unsafe {
            free(buf.cast::<c_void>());
        }

        i += 1;
    }

    ret
}

DEFINE_SUITE!(c"Demangle Java", demangle_java);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
