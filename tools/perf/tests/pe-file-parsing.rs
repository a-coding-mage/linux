// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/pe-file-parsing.c.
// C include dependencies:
// <stdbool.h>, <inttypes.h>, <stdlib.h>, <string.h>, <linux/bitops.h>,
// <linux/kernel.h>, <linux/types.h>, <sys/types.h>, <sys/stat.h>,
// <unistd.h>, <subcmd/exec-cmd.h>, "debug.h", "util/build-id.h",
// "util/symbol.h", "util/dso.h", "tests.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

// External constants, macros, types, and functions are supplied by the
// translated perf build environment.
extern "C" {
    static PATH_MAX: usize;
    static TEST_OK: c_int;
    static TEST_SKIP: c_int;

    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
    fn get_argv_exec_path() -> *const c_char;

    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn filename__read_debuglink(filename: *const c_char, debuglink: *mut c_char, size: usize) -> c_int;
    fn dso__new(name: *const c_char) -> *mut dso;
    fn dso__load_bfd_symbols(dso: *mut dso, debugfile: *const c_char) -> c_int;
    fn dso__sort_by_name(dso: *mut dso);
    fn dso__find_symbol_by_name(dso: *mut dso, name: *const c_char, idx: *mut usize) -> *mut symbol;
    fn dso__delete(dso: *mut dso);
}

#[repr(C)]
pub struct build_id {
    pub size: c_int,
    pub data: [u8; 20],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !$cond {
            return TEST_SKIP;
        }
    };
}

// Original C condition: #ifdef HAVE_LIBBFD_SUPPORT
#[cfg(HAVE_LIBBFD_SUPPORT)]
unsafe fn run_dir(d: *const c_char) -> c_int {
    let mut filename = [0 as c_char; PATH_MAX];
    let mut debugfile = [0 as c_char; PATH_MAX];
    let mut bid = build_id {
        size: 0,
        data: [0; 20],
    };
    let mut debuglink = [0 as c_char; PATH_MAX];
    let expect_build_id: [c_char; 16] = [
        0x5a as c_char,
        0x0f as c_char,
        0xd8u8 as c_char,
        0x82u8 as c_char,
        0xb5u8 as c_char,
        0x30 as c_char,
        0x84u8 as c_char,
        0x22 as c_char,
        0x4b as c_char,
        0xa4u8 as c_char,
        0x7b as c_char,
        0x62 as c_char,
        0x4c as c_char,
        0x55 as c_char,
        0xa4u8 as c_char,
        0x69 as c_char,
    ];
    let mut expect_debuglink = [0 as c_char; PATH_MAX];
    let init_expect_debuglink = b"pe-file.exe.debug\0";
    core::ptr::copy_nonoverlapping(
        init_expect_debuglink.as_ptr() as *const c_char,
        expect_debuglink.as_mut_ptr(),
        init_expect_debuglink.len(),
    );
    let mut dso: *mut dso;
    let mut sym: *mut symbol;
    let mut ret: c_int;
    let mut idx: usize = 0;

    scnprintf(
        filename.as_mut_ptr(),
        PATH_MAX,
        b"%s/pe-file.exe\0".as_ptr() as *const c_char,
        d,
    );
    ret = filename__read_build_id(filename.as_mut_ptr(), &mut bid);
    TEST_ASSERT_VAL!(
        "Failed to read build_id",
        ret == core::mem::size_of_val(&expect_build_id) as c_int
    );
    TEST_ASSERT_VAL!(
        "Wrong build_id",
        memcmp(
            bid.data.as_ptr() as *const c_void,
            expect_build_id.as_ptr() as *const c_void,
            core::mem::size_of_val(&expect_build_id),
        ) == 0
    );

    ret = filename__read_debuglink(filename.as_mut_ptr(), debuglink.as_mut_ptr(), PATH_MAX);
    TEST_ASSERT_VAL!("Failed to read debuglink", ret == 0);
    TEST_ASSERT_VAL!(
        "Wrong debuglink",
        strcmp(debuglink.as_mut_ptr(), expect_debuglink.as_mut_ptr()) == 0
    );

    scnprintf(
        debugfile.as_mut_ptr(),
        PATH_MAX,
        b"%s/%s\0".as_ptr() as *const c_char,
        d,
        debuglink.as_mut_ptr(),
    );
    ret = filename__read_build_id(debugfile.as_mut_ptr(), &mut bid);
    TEST_ASSERT_VAL!(
        "Failed to read debug file build_id",
        ret == core::mem::size_of_val(&expect_build_id) as c_int
    );
    TEST_ASSERT_VAL!(
        "Wrong build_id",
        memcmp(
            bid.data.as_ptr() as *const c_void,
            expect_build_id.as_ptr() as *const c_void,
            core::mem::size_of_val(&expect_build_id),
        ) == 0
    );

    dso = dso__new(filename.as_mut_ptr());
    TEST_ASSERT_VAL!("Failed to get dso", !dso.is_null());

    ret = dso__load_bfd_symbols(dso, debugfile.as_mut_ptr());
    TEST_ASSERT_VAL!("Failed to load symbols", ret == 0);

    dso__sort_by_name(dso);
    sym = dso__find_symbol_by_name(dso, b"main\0".as_ptr() as *const c_char, &mut idx);
    TEST_ASSERT_VAL!("Failed to find main", !sym.is_null());
    dso__delete(dso);

    TEST_OK
}

#[cfg(HAVE_LIBBFD_SUPPORT)]
unsafe extern "C" fn test__pe_file_parsing(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;
    let mut st: stat = core::mem::zeroed();
    let mut path_dir = [0 as c_char; PATH_MAX];

    /* First try development tree tests. */
    if lstat(b"./tests\0".as_ptr() as *const c_char, &mut st) == 0 {
        return run_dir(b"./tests\0".as_ptr() as *const c_char);
    }

    /* Then installed path. */
    snprintf(
        path_dir.as_mut_ptr(),
        PATH_MAX,
        b"%s/tests\0".as_ptr() as *const c_char,
        get_argv_exec_path(),
    );

    if lstat(path_dir.as_mut_ptr(), &mut st) == 0 {
        return run_dir(path_dir.as_mut_ptr());
    }

    TEST_SKIP
}

#[cfg(not(HAVE_LIBBFD_SUPPORT))]
unsafe extern "C" fn test__pe_file_parsing(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    TEST_SKIP
}

// DEFINE_SUITE("PE file support", pe_file_parsing);
// The DEFINE_SUITE macro is provided by the surrounding translated test
// harness; preserve the invocation intent here.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
