// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>, "test_pkt_access.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type __u64 = u64;

const BPF_STATS_RUN_TIME: c_uint = 0;
const ENOSPC: c_int = 28;

static duration: __u32 = 0;

#[repr(C)]
pub struct bpf_prog_info {
    pub run_cnt: __u64,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub repeat: __u32,
    pub data_in: *const c_void,
    pub data_size_in: __u32,
    pub data_out: *mut c_void,
    pub data_size_out: __u32,
    pub retval: __u32,
}

#[repr(C)]
pub struct test_pkt_access {
    pub progs: test_pkt_access_progs,
}

#[repr(C)]
pub struct test_pkt_access_progs {
    pub test_pkt_access: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

extern "C" {
    static pkt_v4: [u8; 0];
    static mut errno: c_int;

    fn bpf_prog_get_info_by_fd(
        prog_fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_enable_stats(stats_type: c_uint) -> c_int;
    fn test_pkt_access__open_and_load() -> *mut test_pkt_access;
    fn test_pkt_access__destroy(obj: *mut test_pkt_access);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ_U64(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn ASSERT_EQ_I32(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_I32(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(actual: c_int, name: *const c_char) -> bool;
}

unsafe fn check_run_cnt(prog_fd: c_int, run_cnt: __u64) {
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let err: c_int;

    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    if CHECK(
        err != 0,
        b"get_prog_info\0".as_ptr() as *const c_char,
        b"failed to get bpf_prog_info for fd %d\n\0".as_ptr() as *const c_char,
        prog_fd,
    ) {
        return;
    }

    CHECK(
        run_cnt != info.run_cnt,
        b"run_cnt\0".as_ptr() as *const c_char,
        b"incorrect number of repetitions, want %llu have %llu\n\0".as_ptr() as *const c_char,
        run_cnt,
        info.run_cnt,
    );
}

pub unsafe fn test_prog_run_opts() {
    let mut skel: *mut test_pkt_access;
    let mut err: c_int;
    let mut stats_fd: c_int = -1;
    let prog_fd: c_int;
    let mut buf: [c_char; 10] = [0; 10];
    let mut run_cnt: __u64 = 0;

    let mut topts = bpf_test_run_opts {
        repeat: 1,
        data_in: ptr::addr_of!(pkt_v4) as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as __u32,
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: 5,
        retval: 0,
    };

    stats_fd = bpf_enable_stats(BPF_STATS_RUN_TIME);
    if !ASSERT_GE(stats_fd, 0, b"enable_stats good fd\0".as_ptr() as *const c_char) {
        return;
    }

    skel = test_pkt_access__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"open_and_load\0".as_ptr() as *const c_char) {
        goto_cleanup(skel, stats_fd);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.test_pkt_access);

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_EQ_I32(errno, ENOSPC, b"test_run errno\0".as_ptr() as *const c_char);
    ASSERT_ERR(err, b"test_run\0".as_ptr() as *const c_char);
    ASSERT_OK_I32(topts.retval as c_int, b"test_run retval\0".as_ptr() as *const c_char);

    ASSERT_EQ_U64(
        topts.data_size_out as __u64,
        core::mem::size_of_val(&pkt_v4) as __u64,
        b"test_run data_size_out\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ_I32(
        buf[5] as c_int,
        0,
        b"overflow, BPF_PROG_TEST_RUN ignored size hint\0".as_ptr() as *const c_char,
    );

    run_cnt += topts.repeat as __u64;
    check_run_cnt(prog_fd, run_cnt);

    topts.data_out = ptr::null_mut();
    topts.data_size_out = 0;
    topts.repeat = 2;
    errno = 0;

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK_I32(errno, b"run_no_output errno\0".as_ptr() as *const c_char);
    ASSERT_OK_I32(err, b"run_no_output err\0".as_ptr() as *const c_char);
    ASSERT_OK_I32(topts.retval as c_int, b"run_no_output retval\0".as_ptr() as *const c_char);

    run_cnt += topts.repeat as __u64;
    check_run_cnt(prog_fd, run_cnt);

    goto_cleanup(skel, stats_fd);
}

unsafe fn goto_cleanup(skel: *mut test_pkt_access, stats_fd: c_int) {
    if !skel.is_null() {
        test_pkt_access__destroy(skel);
    }
    if stats_fd >= 0 {
        close(stats_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
