// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// <linux/compiler.h>, <linux/bitmap.h>, <perf/cpumap.h>,
// <internal/cpumap.h>, "tests.h", "debug.h"

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

const NBITS: c_int = 100;

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn perf_cpu_map__new(str_: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;

    fn bitmap_zalloc(nbits: c_uint) -> *mut c_ulong;
    fn __set_bit(nr: c_int, addr: *mut c_ulong);
    fn bitmap_scnprintf(
        bitmap: *const c_ulong,
        nbits: c_uint,
        buf: *mut c_char,
        size: usize,
    ) -> c_int;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
}

unsafe fn get_bitmap(str_: *const c_char, nbits: c_int) -> *mut c_ulong {
    let map: *mut perf_cpu_map = perf_cpu_map__new(str_);
    let bm: *mut c_ulong = bitmap_zalloc(nbits as c_uint);

    if !map.is_null() && !bm.is_null() {
        let mut i: c_uint = 0;

        while (i as c_int) < perf_cpu_map__nr(map) {
            let cpu: perf_cpu = perf_cpu_map__cpu(map, i as c_int);
            __set_bit(cpu.cpu, bm);
            i = i.wrapping_add(1);
        }
    }

    perf_cpu_map__put(map);
    bm
}

unsafe fn test_bitmap(str_: *const c_char) -> c_int {
    let bm: *mut c_ulong = get_bitmap(str_, NBITS);
    let mut buf: [c_char; 100] = [0; 100];
    let ret: c_int;

    bitmap_scnprintf(bm, NBITS as c_uint, buf.as_mut_ptr(), buf.len());
    pr_debug!("bitmap: %s\n", buf.as_ptr());

    ret = (strcmp(buf.as_ptr(), str_) == 0) as c_int;
    free(bm as *mut c_void);
    ret
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if $cond == 0 {
            return -1;
        }
    };
}

macro_rules! pr_debug {
    ($($arg:tt)*) => {
        // External debug-printing macro from "debug.h".
    };
}

unsafe extern "C" fn test__bitmap_print(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    TEST_ASSERT_VAL!(
        "failed to convert map",
        test_bitmap(c"1".as_ptr())
    );
    TEST_ASSERT_VAL!(
        "failed to convert map",
        test_bitmap(c"1,5".as_ptr())
    );
    TEST_ASSERT_VAL!(
        "failed to convert map",
        test_bitmap(c"1,3,5,7,9,11,13,15,17,19,21-40".as_ptr())
    );
    TEST_ASSERT_VAL!(
        "failed to convert map",
        test_bitmap(c"2-5".as_ptr())
    );
    TEST_ASSERT_VAL!(
        "failed to convert map",
        test_bitmap(c"1,3-6,8-10,24,35-37".as_ptr())
    );
    TEST_ASSERT_VAL!(
        "failed to convert map",
        test_bitmap(c"1,3-6,8-10,24,35-37".as_ptr())
    );
    TEST_ASSERT_VAL!(
        "failed to convert map",
        test_bitmap(c"1-10,12-20,22-30,32-40".as_ptr())
    );
    0
}

// DEFINE_SUITE("Print bitmap", bitmap_print);
// The original C macro registers test__bitmap_print as the bitmap_print suite.
