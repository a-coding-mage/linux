// SPDX-License-Identifier: GPL-2.0
// Dependencies from:
// "arch-tests.h", "linux/perf_event.h", "tests/tests.h", "pmu.h",
// "pmus.h", "../perf-sys.h", "debug.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;

const NR_SUB_TESTS: usize = 5;

extern "C" {
    static PERF_TYPE_HARDWARE: c_int;
    static PERF_COUNT_HW_CPU_CYCLES: c_ulong;
    static PERF_COUNT_HW_INSTRUCTIONS: c_ulong;
    static PERF_TYPE_RAW: c_int;
    static PERF_SAMPLE_IP: u64;
    static PERF_SAMPLE_TID: u64;
    static TEST_OK: c_int;
    static TEST_SKIP: c_int;
    static TEST_FAIL: c_int;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn pr_debug(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub disabled: u64,
    pub precise_ip: u64,
}

#[repr(C)]
struct sub_tests {
    type_: c_int,
    config: c_ulong,
    valid: bool,
}

static mut SUB_TESTS: [sub_tests; NR_SUB_TESTS] = [
    sub_tests {
        type_: unsafe { PERF_TYPE_HARDWARE },
        config: unsafe { PERF_COUNT_HW_CPU_CYCLES },
        valid: true,
    },
    sub_tests {
        type_: unsafe { PERF_TYPE_HARDWARE },
        config: unsafe { PERF_COUNT_HW_INSTRUCTIONS },
        valid: false,
    },
    sub_tests {
        type_: unsafe { PERF_TYPE_RAW },
        config: 0x076,
        valid: true,
    },
    sub_tests {
        type_: unsafe { PERF_TYPE_RAW },
        config: 0x0C1,
        valid: true,
    },
    sub_tests {
        type_: unsafe { PERF_TYPE_RAW },
        config: 0x012,
        valid: false,
    },
];

unsafe fn event_open(type_: c_int, config: c_ulong) -> c_int {
    let mut attr: perf_event_attr = mem::zeroed();

    memset(
        &mut attr as *mut perf_event_attr as *mut c_void,
        0,
        mem::size_of::<perf_event_attr>(),
    );
    attr.type_ = type_ as u32;
    attr.size = mem::size_of::<perf_event_attr>() as u32;
    attr.config = config as u64;
    attr.disabled = 1;
    attr.precise_ip = 1;
    attr.sample_type = PERF_SAMPLE_IP | PERF_SAMPLE_TID;
    attr.sample_period = 100000;

    sys_perf_event_open(&mut attr, -1, 0, -1, 0)
}

#[no_mangle]
pub unsafe extern "C" fn test__amd_ibs_via_core_pmu(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut ret: c_int = TEST_OK;
    let mut fd: c_int;
    let mut i: c_int;

    let ibs_pmu = perf_pmus__find(c"ibs_op".as_ptr());
    if ibs_pmu.is_null() {
        return TEST_SKIP;
    }

    i = 0;
    while i < NR_SUB_TESTS as c_int {
        fd = event_open(
            SUB_TESTS[i as usize].type_,
            SUB_TESTS[i as usize].config,
        );
        pr_debug(
            c"type: 0x%x, config: 0x%lx, fd: %d  -  ".as_ptr(),
            SUB_TESTS[i as usize].type_,
            SUB_TESTS[i as usize].config,
            fd,
        );
        if (SUB_TESTS[i as usize].valid && fd == -1)
            || (!SUB_TESTS[i as usize].valid && fd > 0)
        {
            pr_debug(c"Fail\n".as_ptr());
            ret = TEST_FAIL;
        } else {
            pr_debug(c"Pass\n".as_ptr());
        }

        if fd > 0 {
            close(fd);
        }
        i += 1;
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
