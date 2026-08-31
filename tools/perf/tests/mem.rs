// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/mem.c.
// Original C dependencies:
// "util/map_symbol.h", "util/mem-events.h", "util/mem-info.h",
// "util/symbol.h", "linux/perf_event.h", "util/debug.h", "tests.h",
// and <string.h>.

use core::ffi::{c_char, c_int, c_ulong};

const PERF_MEM_LVL_HIT: u64 = 0x02;
const PERF_MEM_LVL_MISS: u64 = 0x04;
const PERF_MEM_LVLNUM_RAM: u64 = 0x08;
const PERF_MEM_LVLNUM_PMEM: u64 = 0x0c;
const PERF_MEM_SNOOPX_FWD: u64 = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_mem_data_src {
    pub mem_lvl: u64,
    pub mem_remote: u64,
    pub mem_lvl_num: u64,
    pub mem_snoopx: u64,
}

impl perf_mem_data_src {
    const fn zeroed() -> Self {
        Self {
            mem_lvl: 0,
            mem_remote: 0,
            mem_lvl_num: 0,
            mem_snoopx: 0,
        }
    }
}

#[repr(C)]
pub struct mem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn mem_info__new() -> *mut mem_info;
    fn mem_info__data_src(mi: *mut mem_info) -> *mut perf_mem_data_src;
    fn perf_mem__snp_scnprintf(
        bf: *mut c_char,
        size: c_ulong,
        mi: *mut mem_info,
    ) -> c_int;
    fn perf_mem__lvl_scnprintf(
        bf: *mut c_char,
        size: c_ulong,
        mi: *mut mem_info,
    ) -> c_int;
    fn mem_info__put(mi: *mut mem_info);
    fn scnprintf(bf: *mut c_char, size: c_ulong, fmt: *const c_char, ...) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn TEST_ASSERT_VAL(desc: *const c_char, expr: bool);
}

unsafe fn check(data_src: perf_mem_data_src, string: *const c_char) -> c_int {
    let mut out = [0 as c_char; 100];
    let mut failure = [0 as c_char; 100];
    let mi = unsafe { mem_info__new() };
    let mut n: c_int;

    unsafe {
        TEST_ASSERT_VAL(c"Memory allocation failed".as_ptr(), !mi.is_null());
        *mem_info__data_src(mi) = data_src;
        n = perf_mem__snp_scnprintf(out.as_mut_ptr(), out.len() as c_ulong, mi);
        n += perf_mem__lvl_scnprintf(
            out.as_mut_ptr().add(n as usize),
            (out.len() as c_int - n) as c_ulong,
            mi,
        );
        mem_info__put(mi);
        scnprintf(
            failure.as_mut_ptr(),
            failure.len() as c_ulong,
            c"unexpected %s".as_ptr(),
            out.as_ptr(),
        );
        TEST_ASSERT_VAL(failure.as_ptr(), strcmp(string, out.as_ptr()) == 0);
    }
    0
}

unsafe fn test__mem(_text: *mut test_suite, _subtest: c_int) -> c_int {
    let mut ret: c_int = 0;
    let mut src: perf_mem_data_src;

    src = perf_mem_data_src::zeroed();

    src.mem_lvl = PERF_MEM_LVL_HIT;
    src.mem_lvl_num = 4;

    ret |= unsafe { check(src, c"N/AL4 hit".as_ptr()) };

    src.mem_remote = 1;

    ret |= unsafe { check(src, c"N/ARemote L4 hit".as_ptr()) };

    src.mem_lvl = PERF_MEM_LVL_MISS;
    src.mem_lvl_num = PERF_MEM_LVLNUM_PMEM;
    src.mem_remote = 0;

    ret |= unsafe { check(src, c"N/APMEM miss".as_ptr()) };

    src.mem_remote = 1;

    ret |= unsafe { check(src, c"N/ARemote PMEM miss".as_ptr()) };

    src.mem_snoopx = PERF_MEM_SNOOPX_FWD;
    src.mem_lvl_num = PERF_MEM_LVLNUM_RAM;

    ret |= unsafe { check(src, c"FwdRemote RAM miss".as_ptr()) };

    ret
}

// DEFINE_SUITE("Test data source output", mem);
// Preserved as an externally visible suite descriptor hook for the translated test.
#[unsafe(no_mangle)]
pub static mut mem: *const c_char = c"Test data source output".as_ptr();
