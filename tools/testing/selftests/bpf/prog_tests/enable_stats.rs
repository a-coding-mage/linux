// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "test_enable_stats.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_prog_info {
    pub run_time_ns: u64,
    pub run_cnt: u64,
}

#[repr(C)]
pub struct test_enable_stats_bss {
    pub count: u64,
}

#[repr(C)]
pub struct test_enable_stats_progs {
    pub test_enable_stats: *mut c_void,
}

#[repr(C)]
pub struct test_enable_stats {
    pub progs: test_enable_stats_progs,
    pub bss: *mut test_enable_stats_bss,
}

unsafe extern "C" {
    fn test_enable_stats__open_and_load() -> *mut test_enable_stats;
    fn test_enable_stats__destroy(skel: *mut test_enable_stats);
    fn test_enable_stats__attach(skel: *mut test_enable_stats) -> c_int;
    fn test_enable_stats__detach(skel: *mut test_enable_stats);

    fn bpf_enable_stats(stats_type: c_int) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_prog_get_info_by_fd(
        fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut u32,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    static mut errno: c_int;
}

unsafe extern "C" {
    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
}

const BPF_STATS_RUN_TIME: c_int = 0;

pub unsafe fn test_enable_stats() {
    let mut skel: *mut test_enable_stats;
    let stats_fd: c_int;
    let mut err: c_int;
    let prog_fd: c_int;
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_len: u32 = core::mem::size_of::<bpf_prog_info>() as u32;
    let _duration: c_int = 0;

    skel = test_enable_stats__open_and_load();
    if CHECK(
        skel.is_null(),
        c"skel_open_and_load".as_ptr(),
        c"skeleton open/load failed\n".as_ptr(),
    ) {
        return;
    }

    stats_fd = bpf_enable_stats(BPF_STATS_RUN_TIME);
    if CHECK(
        stats_fd < 0,
        c"get_stats_fd".as_ptr(),
        c"failed %d\n".as_ptr(),
        errno,
    ) {
        test_enable_stats__destroy(skel);
        return;
    }

    err = test_enable_stats__attach(skel);
    if CHECK(
        err != 0,
        c"attach_raw_tp".as_ptr(),
        c"err %d\n".as_ptr(),
        err,
    ) {
        test_enable_stats__destroy(skel);
        close(stats_fd);
        return;
    }

    test_enable_stats__detach(skel);

    prog_fd = bpf_program__fd((*skel).progs.test_enable_stats);
    memset(
        &mut info as *mut bpf_prog_info as *mut c_void,
        0,
        info_len as usize,
    );
    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    if CHECK(
        err != 0,
        c"get_prog_info".as_ptr(),
        c"failed to get bpf_prog_info for fd %d\n".as_ptr(),
        prog_fd,
    ) {
        test_enable_stats__destroy(skel);
        close(stats_fd);
        return;
    }
    if CHECK(
        info.run_time_ns == 0,
        c"check_stats_enabled".as_ptr(),
        c"failed to enable run_time_ns stats\n".as_ptr(),
    ) {
        test_enable_stats__destroy(skel);
        close(stats_fd);
        return;
    }

    CHECK(
        info.run_cnt != (*(*skel).bss).count,
        c"check_run_cnt_valid".as_ptr(),
        c"invalid run_cnt stats\n".as_ptr(),
    );

    test_enable_stats__destroy(skel);
    close(stats_fd);
}
