// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2022 Linutronix GmbH */

/* Translated dependencies from:
 * <test_progs.h>
 * <network_helpers.h>
 * "test_time_tai.skel.h"
 * <time.h>
 * <stdint.h>
 */

const TAI_THRESHOLD: u64 = 1000000000u64; /* 1s */
const NSEC_PER_SEC: u64 = 1000000000u64;

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct __sk_buff {
    pub cb: [u32; 5],
    pub tstamp: u64,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const core::ffi::c_void,
    pub data_size_in: u32,
    pub ctx_in: *mut core::ffi::c_void,
    pub ctx_size_in: u32,
    pub ctx_out: *mut core::ffi::c_void,
    pub ctx_size_out: u32,
}

#[repr(C)]
pub struct test_time_tai {
    pub progs: test_time_tai_progs,
}

#[repr(C)]
pub struct test_time_tai_progs {
    pub time_tai: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    static pkt_v4: core::ffi::c_void;
    static CLOCK_TAI: core::ffi::c_int;

    fn test_time_tai__open_and_load() -> *mut test_time_tai;
    fn test_time_tai__destroy(skel: *mut test_time_tai);
    fn bpf_program__fd(prog: *mut bpf_program) -> core::ffi::c_int;
    fn bpf_prog_test_run_opts(prog_fd: core::ffi::c_int, opts: *mut bpf_test_run_opts) -> core::ffi::c_int;
    fn clock_gettime(clk_id: core::ffi::c_int, tp: *mut timespec) -> core::ffi::c_int;

    fn ASSERT_OK_PTR(ptr: *mut core::ffi::c_void, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_OK(ret: core::ffi::c_int, name: *const core::ffi::c_char);
    fn ASSERT_NEQ(actual: u64, expected: u64, name: *const core::ffi::c_char);
    fn ASSERT_GE(actual: u64, expected: u64, name: *const core::ffi::c_char);
    fn ASSERT_EQ(actual: core::ffi::c_int, expected: core::ffi::c_int, name: *const core::ffi::c_char);
    fn ASSERT_TRUE(condition: bool, name: *const core::ffi::c_char);
}

unsafe fn ts_to_ns(ts: *const timespec) -> u64 {
    ((*ts).tv_sec as u64)
        .wrapping_mul(NSEC_PER_SEC)
        .wrapping_add((*ts).tv_nsec as u64)
}

pub unsafe fn test_time_tai() {
    let mut skb = __sk_buff {
        cb: [0; 5],
        tstamp: 0,
    };

    let mut topts = bpf_test_run_opts {
        data_in: &pkt_v4 as *const _ as *const core::ffi::c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        ctx_in: &mut skb as *mut _ as *mut core::ffi::c_void,
        ctx_size_in: core::mem::size_of_val(&skb) as u32,
        ctx_out: &mut skb as *mut _ as *mut core::ffi::c_void,
        ctx_size_out: core::mem::size_of_val(&skb) as u32,
    };
    let skel: *mut test_time_tai;
    let mut now_tai = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let ts1: u64;
    let ts2: u64;
    let now: u64;
    let mut ret: core::ffi::c_int;
    let prog_fd: core::ffi::c_int;

    /* Open and load */
    skel = test_time_tai__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut core::ffi::c_void, c"tai_open".as_ptr()) {
        return;
    }

    /* Run test program */
    prog_fd = bpf_program__fd((*skel).progs.time_tai);
    ret = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(ret, c"test_run".as_ptr());

    /* Retrieve generated TAI timestamps */
    ts1 = skb.tstamp;
    ts2 = (skb.cb[0] as u64) | ((skb.cb[1] as u64) << 32);

    /* TAI != 0 */
    ASSERT_NEQ(ts1, 0, c"tai_ts1".as_ptr());
    ASSERT_NEQ(ts2, 0, c"tai_ts2".as_ptr());

    /* TAI is moving forward only */
    ASSERT_GE(ts2, ts1, c"tai_forward".as_ptr());

    /* Check for future */
    ret = clock_gettime(CLOCK_TAI, &mut now_tai);
    ASSERT_EQ(ret, 0, c"tai_gettime".as_ptr());
    now = ts_to_ns(&now_tai);

    ASSERT_TRUE(now > ts1, c"tai_future_ts1".as_ptr());
    ASSERT_TRUE(now > ts2, c"tai_future_ts2".as_ptr());

    /* Check for reasonable range */
    ASSERT_TRUE(now.wrapping_sub(ts1) < TAI_THRESHOLD, c"tai_range_ts1".as_ptr());
    ASSERT_TRUE(now.wrapping_sub(ts2) < TAI_THRESHOLD, c"tai_range_ts2".as_ptr());

    test_time_tai__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
