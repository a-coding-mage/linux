// SPDX-License-Identifier: GPL-2.0
/*
 * Test backward bit in event attribute, read ring buffer from end to
 * beginning
 */

// C dependencies translated as external Rust dependencies:
// <evlist.h>, <sys/prctl.h>, "record.h", "tests.h", "debug.h",
// "parse-events.h", "util/mmap.h", <errno.h>, <linux/string.h>,
// <perf/mmap.h>

use core::ffi::{c_char, c_int, c_void};

const NR_ITERS: c_int = 111;

unsafe extern "C" {
    static mut errno: c_int;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn getpid() -> c_int;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;

    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__overwrite_mmap(evlist: *mut evlist) -> *mut mmap;
    fn perf_mmap__read_init(map: *mut perf_mmap);
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__read_done(map: *mut perf_mmap);
    fn evlist__do_mmap(evlist: *mut evlist, mmap_pages: c_int) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evlist__do_munmap(evlist: *mut evlist);
    fn evlist__new() -> *mut evlist;
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events(
        evlist: *mut evlist,
        str_: *const c_char,
        err: *mut parse_events_error,
    ) -> c_int;
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__close(evlist: *mut evlist);
    fn evlist__put(evlist: *mut evlist);
    fn pr_debug(format: *const c_char, ...);
    fn pr_err(format: *const c_char, ...);
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    pub nr_mmaps: c_int,
}

#[repr(C)]
pub struct mmap {
    pub core: perf_mmap,
}

#[repr(C)]
pub struct perf_mmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    pub uses_mmap: bool,
    pub tid: *mut c_char,
    pub pid: *mut c_char,
}

#[repr(C)]
pub struct record_opts {
    pub target: target,
    pub freq: c_int,
    pub mmap_pages: c_int,
    pub default_interval: c_int,
}

// External constants from the original C headers.
const PR_SET_NAME: c_int = 15;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_COMM: u32 = 3;
const STRERR_BUFSIZE: usize = 128;
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;

unsafe fn testcase() {
    let mut i: c_int;

    i = 0;
    while i < NR_ITERS {
        let mut proc_name: [c_char; 15] = [0; 15];

        snprintf(
            proc_name.as_mut_ptr(),
            proc_name.len(),
            c"p:%d\n".as_ptr(),
            i,
        );
        prctl(PR_SET_NAME, proc_name.as_mut_ptr());
        i += 1;
    }
}

unsafe fn count_samples(
    evlist: *mut evlist,
    sample_count: *mut c_int,
    comm_count: *mut c_int,
) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < (*evlist__core(evlist)).nr_mmaps {
        let map: *mut mmap = evlist__overwrite_mmap(evlist).offset(i as isize);
        let mut event: *mut perf_event;

        perf_mmap__read_init(&mut (*map).core);
        loop {
            event = perf_mmap__read_event(&mut (*map).core);
            if event.is_null() {
                break;
            }

            let type_: u32 = (*event).header.type_;

            match type_ {
                PERF_RECORD_SAMPLE => {
                    *sample_count += 1;
                }
                PERF_RECORD_COMM => {
                    *comm_count += 1;
                }
                _ => {
                    pr_err(c"Unexpected record of type %d\n".as_ptr(), type_);
                    return TEST_FAIL;
                }
            }
        }
        perf_mmap__read_done(&mut (*map).core);
        i += 1;
    }
    TEST_OK
}

unsafe fn do_test(
    evlist: *mut evlist,
    mmap_pages: c_int,
    sample_count: *mut c_int,
    comm_count: *mut c_int,
) -> c_int {
    let mut err: c_int;
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];

    err = evlist__do_mmap(evlist, mmap_pages);
    if err < 0 {
        pr_debug(
            c"evlist__mmap: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        return TEST_FAIL;
    }

    evlist__enable(evlist);
    testcase();
    evlist__disable(evlist);

    err = count_samples(evlist, sample_count, comm_count);
    evlist__do_munmap(evlist);
    err
}

unsafe fn test__backward_ring_buffer(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let mut ret: c_int = TEST_SKIP;
    let mut err: c_int;
    let mut sample_count: c_int = 0;
    let mut comm_count: c_int = 0;
    let mut pid: [c_char; 16] = [0; 16];
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];
    let mut evlist: *mut evlist;
    let mut evsel: *mut evsel;
    let mut parse_error: parse_events_error = core::mem::zeroed();
    let mut opts: record_opts = record_opts {
        target: target {
            uses_mmap: true,
            tid: core::ptr::null_mut(),
            pid: core::ptr::null_mut(),
        },
        freq: 0,
        mmap_pages: 256,
        default_interval: 1,
    };

    let _ = test;
    let _ = subtest;
    let _ = &mut evsel;

    snprintf(pid.as_mut_ptr(), pid.len(), c"%d".as_ptr(), getpid());
    pid[pid.len() - 1] = 0;
    opts.target.pid = pid.as_mut_ptr();
    opts.target.tid = opts.target.pid;

    evlist = evlist__new();
    if evlist.is_null() {
        pr_debug(c"Not enough memory to create evlist\n".as_ptr());
        return TEST_FAIL;
    }

    err = evlist__create_maps(evlist, &mut opts.target);
    if err < 0 {
        pr_debug(c"Not enough memory to create thread/cpu maps\n".as_ptr());
        goto_out_put_evlist(evlist);
        return ret;
    }

    parse_events_error__init(&mut parse_error);
    /*
     * Set backward bit, ring buffer should be writing from end. Record
     * it in aux evlist
     */
    err = parse_events(
        evlist,
        c"syscalls:sys_enter_prctl/overwrite/".as_ptr(),
        &mut parse_error,
    );
    parse_events_error__exit(&mut parse_error);
    if err != 0 {
        pr_debug(c"Failed to parse tracepoint event, try use root\n".as_ptr());
        ret = TEST_SKIP;
        goto_out_put_evlist(evlist);
        return ret;
    }

    evlist__config(evlist, &mut opts, core::ptr::null_mut());

    err = evlist__open(evlist);
    if err < 0 {
        pr_debug(
            c"perf_evlist__open: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        goto_out_put_evlist(evlist);
        return ret;
    }

    ret = TEST_FAIL;
    err = do_test(
        evlist,
        opts.mmap_pages,
        &mut sample_count,
        &mut comm_count,
    );
    if err != TEST_OK {
        goto_out_put_evlist(evlist);
        return ret;
    }

    if sample_count != NR_ITERS || comm_count != NR_ITERS {
        pr_err(
            c"Unexpected counter: sample_count=%d, comm_count=%d\n".as_ptr(),
            sample_count,
            comm_count,
        );
        goto_out_put_evlist(evlist);
        return ret;
    }

    evlist__close(evlist);

    err = evlist__open(evlist);
    if err < 0 {
        pr_debug(
            c"perf_evlist__open: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        goto_out_put_evlist(evlist);
        return ret;
    }

    err = do_test(evlist, 1, &mut sample_count, &mut comm_count);
    if err != TEST_OK {
        goto_out_put_evlist(evlist);
        return ret;
    }

    ret = TEST_OK;
    goto_out_put_evlist(evlist);
    ret
}

unsafe fn goto_out_put_evlist(evlist: *mut evlist) {
    evlist__put(evlist);
}

// DEFINE_SUITE("Read backward ring buffer", backward_ring_buffer);
define_suite!("Read backward ring buffer", backward_ring_buffer);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
