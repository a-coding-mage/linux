// SPDX-License-Identifier: GPL-2.0
// C dependencies: string.h, stdlib.h, stdio.h, debug.h, symbol.h, tests.h

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
        mangled: *const c_char,
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

unsafe fn test__demangle_ocaml(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut ret: c_int = TEST_OK;
    let mut buf: *mut c_char = core::ptr::null_mut();
    let mut i: usize;

    let test_cases = [
        TestCase {
            mangled: c"main".as_ptr(),
            demangled: core::ptr::null(),
        },
        TestCase {
            mangled: c"camlStdlib__array__map_154".as_ptr(),
            demangled: c"Stdlib.array.map_154".as_ptr(),
        },
        TestCase {
            mangled: c"camlStdlib__anon_fn$5bstdlib$2eml$3a334$2c0$2d$2d54$5d_1453".as_ptr(),
            demangled: c"Stdlib.anon_fn[stdlib.ml:334,0--54]_1453".as_ptr(),
        },
        TestCase {
            mangled: c"camlStdlib__bytes__$2b$2b_2205".as_ptr(),
            demangled: c"Stdlib.bytes.++_2205".as_ptr(),
        },
    ];

    i = 0;
    while i < test_cases.len() {
        buf = dso__demangle_sym(
            /* dso = */ core::ptr::null_mut(),
            /* kmodule = */ 0,
            test_cases[i].mangled,
        );
        if (buf.is_null() && !test_cases[i].demangled.is_null())
            || (!buf.is_null() && test_cases[i].demangled.is_null())
            || (!buf.is_null() && strcmp(buf, test_cases[i].demangled) != 0)
        {
            pr_debug(
                c"FAILED: %s: %s != %s\n".as_ptr(),
                test_cases[i].mangled,
                if buf.is_null() {
                    c"(null)".as_ptr()
                } else {
                    buf as *const c_char
                },
                if test_cases[i].demangled.is_null() {
                    c"(null)".as_ptr()
                } else {
                    test_cases[i].demangled
                },
            );
            ret = TEST_FAIL;
        }
        free(buf as *mut c_void);

        i += 1;
    }

    ret
}

// C macro registration:
// DEFINE_SUITE("Demangle OCaml", demangle_ocaml);
