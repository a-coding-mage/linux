// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/tests/keep-tracking.c.
// C include dependencies are represented below as FFI declarations.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type pid_t = c_int;

const UINT_MAX: u32 = u32::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const PERF_RECORD_COMM: u32 = 3;
const PR_SET_NAME: c_int = 15;
const TEST_SKIP: c_int = 2;

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_record_comm {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub comm: perf_record_comm,
}

#[repr(C)]
pub struct perf_mmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmap {
    pub core: perf_mmap,
}

#[repr(C)]
pub struct evlist_core {
    pub nr_mmaps: c_int,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub comm: u64,
    pub disabled: u64,
    pub enable_on_exec: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct target {
    pub uses_mmap: bool,
}

#[repr(C)]
pub struct record_opts {
    pub mmap_pages: u32,
    pub user_freq: u32,
    pub user_interval: u64,
    pub target: target,
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub func: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
}

unsafe extern "C" {
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);

    fn getpid() -> pid_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);

    fn thread_map__new_by_tid(tid: pid_t) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn evlist__new() -> *mut evlist;
    fn perf_evlist__set_maps(
        evlist: *mut evlist_core,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    );
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: u32) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evlist__put(evlist: *mut evlist);
    fn evsel__disable(evsel: *mut evsel) -> c_int;
}

macro_rules! CHECK__ {
    ($x:expr, $err_label:lifetime) => {{
        while $x < 0 {
            pr_debug(concat!(stringify!($x), " failed!\n\0").as_ptr() as *const c_char);
            break $err_label;
        }
    }};
}

macro_rules! CHECK_NOT_NULL__ {
    ($x:expr, $err_label:lifetime) => {{
        while ($x).is_null() {
            pr_debug(concat!(stringify!($x), " failed!\n\0").as_ptr() as *const c_char);
            break $err_label;
        }
    }};
}

unsafe extern "C" fn find_comm(evlist: *mut evlist, comm: *const c_char) -> c_int {
    let mut event: *mut perf_event;
    let mut md: *mut mmap;
    let mut i: c_int;
    let mut found: c_int;

    found = 0;
    i = 0;
    while i < (*evlist__core(evlist)).nr_mmaps {
        md = evlist__mmap(evlist).offset(i as isize);
        if perf_mmap__read_init(&mut (*md).core) < 0 {
            i += 1;
            continue;
        }
        loop {
            event = perf_mmap__read_event(&mut (*md).core);
            if event.is_null() {
                break;
            }
            if (*event).header.type_ == PERF_RECORD_COMM
                && (*event).comm.pid as pid_t == getpid()
                && (*event).comm.tid as pid_t == getpid()
                && strcmp((*event).comm.comm.as_ptr(), comm) == 0
            {
                found += 1;
            }
            perf_mmap__consume(&mut (*md).core);
        }
        perf_mmap__read_done(&mut (*md).core);
        i += 1;
    }
    found
}

/**
 * test__keep_tracking - test using a dummy software event to keep tracking.
 *
 * This function implements a test that checks that tracking events continue
 * when an event is disabled but a dummy software event is not disabled.  If the
 * test passes %0 is returned, otherwise %-1 is returned.
 */
unsafe extern "C" fn test__keep_tracking(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut opts = record_opts {
        mmap_pages: UINT_MAX,
        user_freq: UINT_MAX,
        user_interval: ULLONG_MAX,
        target: target { uses_mmap: true },
    };
    let mut threads: *mut perf_thread_map = core::ptr::null_mut();
    let mut cpus: *mut perf_cpu_map = core::ptr::null_mut();
    let mut evlist: *mut evlist = core::ptr::null_mut();
    let mut evsel: *mut evsel;
    let mut found: c_int;
    let mut err: c_int = -1;
    let mut comm: *const c_char;

    'out_err: loop {
        threads = thread_map__new_by_tid(getpid());
        CHECK_NOT_NULL__!(threads, 'out_err);

        cpus = perf_cpu_map__new_online_cpus();
        CHECK_NOT_NULL__!(cpus, 'out_err);

        evlist = evlist__new();
        CHECK_NOT_NULL__!(evlist, 'out_err);

        perf_evlist__set_maps(evlist__core(evlist), cpus, threads);

        CHECK__!(parse_event(evlist, c"dummy:u".as_ptr()), 'out_err);
        CHECK__!(parse_event(evlist, c"cpu-cycles:u".as_ptr()), 'out_err);

        evlist__config(evlist, &mut opts, core::ptr::null_mut());

        evsel = evlist__first(evlist);

        (*evsel).core.attr.comm = 1;
        (*evsel).core.attr.disabled = 1;
        (*evsel).core.attr.enable_on_exec = 0;

        if evlist__open(evlist) < 0 {
            pr_debug(c"Unable to open dummy and cycles event\n".as_ptr());
            err = TEST_SKIP;
            break 'out_err;
        }

        CHECK__!(evlist__do_mmap(evlist, UINT_MAX), 'out_err);

        /*
         * First, test that a 'comm' event can be found when the event is
         * enabled.
         */

        evlist__enable(evlist);

        comm = c"Test COMM 1".as_ptr();
        CHECK__!(prctl(PR_SET_NAME, comm as c_ulong, 0, 0, 0), 'out_err);

        evlist__disable(evlist);

        found = find_comm(evlist, comm);
        if found != 1 {
            pr_debug(c"First time, failed to find tracking event.\n".as_ptr());
            break 'out_err;
        }

        /*
         * Secondly, test that a 'comm' event can be found when the event is
         * disabled with the dummy event still enabled.
         */

        evlist__enable(evlist);

        evsel = evlist__last(evlist);

        CHECK__!(evsel__disable(evsel), 'out_err);

        comm = c"Test COMM 2".as_ptr();
        CHECK__!(prctl(PR_SET_NAME, comm as c_ulong, 0, 0, 0), 'out_err);

        evlist__disable(evlist);

        found = find_comm(evlist, comm);
        if found != 1 {
            pr_debug(c"Second time, failed to find tracking event.\n".as_ptr());
            break 'out_err;
        }

        err = 0;
        break 'out_err;
    }

    if !evlist.is_null() {
        evlist__disable(evlist);
        evlist__put(evlist);
    }
    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);

    return err;
}

#[unsafe(no_mangle)]
pub static mut keep_tracking: test_suite = test_suite {
    desc: c"Use a dummy software event to keep tracking".as_ptr(),
    func: Some(test__keep_tracking),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
