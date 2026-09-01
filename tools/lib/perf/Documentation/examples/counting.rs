// Translated from lib/perf/Documentation/examples/counting.c.
// Original C includes:
// <linux/perf_event.h>, <perf/evlist.h>, <perf/evsel.h>,
// <perf/cpumap.h>, <perf/threadmap.h>, <perf/mmap.h>, <perf/core.h>,
// <perf/event.h>, <stdio.h>, <unistd.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

type va_list = *mut c_void;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub enum libperf_print_level {
    LIBPERF_ERR = 0,
    LIBPERF_WARN = 1,
    LIBPERF_INFO = 2,
    LIBPERF_DEBUG = 3,
    LIBPERF_DEBUG2 = 4,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: c_uint,
    pub size: c_uint,
    pub config: u64,
    pub sample_period_or_freq: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
}

const PERF_EVENT_ATTR_DISABLED: u64 = 1 << 0;

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;

    static PERF_TYPE_SOFTWARE: c_uint;
    static PERF_COUNT_SW_CPU_CLOCK: u64;
    static PERF_COUNT_SW_TASK_CLOCK: u64;
    static PERF_FORMAT_TOTAL_TIME_ENABLED: u64;
    static PERF_FORMAT_TOTAL_TIME_RUNNING: u64;

    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: va_list) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    fn libperf_init(print_fn: Option<unsafe extern "C" fn(libperf_print_level, *const c_char, va_list) -> c_int>);
    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    fn perf_thread_map__set_pid(threads: *mut perf_thread_map, thread: c_int, pid: c_int);
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_evlist__new() -> *mut perf_evlist;
    fn perf_evlist__delete(evlist: *mut perf_evlist);
    fn perf_evlist__add(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
    fn perf_evlist__set_maps(evlist: *mut perf_evlist, cpus: *mut c_void, threads: *mut perf_thread_map);
    fn perf_evlist__open(evlist: *mut perf_evlist) -> c_int;
    fn perf_evlist__enable(evlist: *mut perf_evlist);
    fn perf_evlist__disable(evlist: *mut perf_evlist);
    fn perf_evlist__close(evlist: *mut perf_evlist);
    fn perf_evsel__new(attr: *const perf_event_attr) -> *mut perf_evsel;
    fn perf_evsel__read(evsel: *mut perf_evsel, cpu: c_int, thread: c_int, counts: *mut perf_counts_values) -> c_int;

    // Rust has no preprocessor equivalent for the C macro
    // perf_evlist__for_each_evsel(evlist, evsel). These declarations preserve
    // the source-level iteration intent supplied by libperf.
    fn perf_evlist__first(evlist: *mut perf_evlist) -> *mut perf_evsel;
    fn perf_evsel__next(evsel: *mut perf_evsel) -> *mut perf_evsel;
}

unsafe extern "C" fn libperf_print(
    _level: libperf_print_level,
    fmt: *const c_char,
    ap: va_list,
) -> c_int {
    unsafe { vfprintf(stderr, fmt, ap) }
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut count: c_int = 100000;
    let mut err: c_int = 0;
    let mut evsel: *mut perf_evsel;
    let threads: *mut perf_thread_map;
    let evlist: *mut perf_evlist;
    let mut counts: perf_counts_values = core::mem::zeroed();

    let attr1 = perf_event_attr {
        type_: PERF_TYPE_SOFTWARE,
        size: 0,
        config: PERF_COUNT_SW_CPU_CLOCK,
        sample_period_or_freq: 0,
        sample_type: 0,
        read_format: PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING,
        flags: PERF_EVENT_ATTR_DISABLED,
    };
    let attr2 = perf_event_attr {
        type_: PERF_TYPE_SOFTWARE,
        size: 0,
        config: PERF_COUNT_SW_TASK_CLOCK,
        sample_period_or_freq: 0,
        sample_type: 0,
        read_format: PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING,
        flags: PERF_EVENT_ATTR_DISABLED,
    };

    libperf_init(Some(libperf_print));
    threads = perf_thread_map__new_dummy();
    if threads.is_null() {
        fprintf(stderr, c"failed to create threads\n".as_ptr());
        return -1;
    }
    perf_thread_map__set_pid(threads, 0, 0);
    evlist = perf_evlist__new();
    if evlist.is_null() {
        fprintf(stderr, c"failed to create evlist\n".as_ptr());
        perf_thread_map__put(threads);
        return err;
    }
    evsel = perf_evsel__new(&attr1);
    if evsel.is_null() {
        fprintf(stderr, c"failed to create evsel1\n".as_ptr());
        perf_evlist__delete(evlist);
        perf_thread_map__put(threads);
        return err;
    }
    perf_evlist__add(evlist, evsel);
    evsel = perf_evsel__new(&attr2);
    if evsel.is_null() {
        fprintf(stderr, c"failed to create evsel2\n".as_ptr());
        perf_evlist__delete(evlist);
        perf_thread_map__put(threads);
        return err;
    }
    perf_evlist__add(evlist, evsel);
    perf_evlist__set_maps(evlist, core::ptr::null_mut(), threads);
    err = perf_evlist__open(evlist);
    if err != 0 {
        fprintf(stderr, c"failed to open evsel\n".as_ptr());
        perf_evlist__delete(evlist);
        perf_thread_map__put(threads);
        return err;
    }
    perf_evlist__enable(evlist);
    while {
        let old_count = count;
        count = count.wrapping_sub(1);
        old_count != 0
    } {}
    perf_evlist__disable(evlist);

    evsel = perf_evlist__first(evlist);
    while !evsel.is_null() {
        perf_evsel__read(evsel, 0, 0, &mut counts);
        fprintf(
            stdout,
            c"count %llu, enabled %llu, run %llu\n".as_ptr(),
            counts.val as c_ulonglong,
            counts.ena as c_ulonglong,
            counts.run as c_ulonglong,
        );
        evsel = perf_evsel__next(evsel);
    }

    perf_evlist__close(evlist);
    perf_evlist__delete(evlist);
    perf_thread_map__put(threads);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe { main_impl(argc, argv) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
