// SPDX-License-Identifier: GPL-2.0
// Translated from perf/bench/evlist-open-close.c.
// C include dependencies are represented below as external declarations.

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use std::mem::MaybeUninit;
use std::ptr;

const MMAP_FLUSH_DEFAULT: c_int = 1;
const USEC_PER_SEC: u64 = 1_000_000;
const UINT_MAX: c_uint = c_uint::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EXIT_FAILURE: c_int = 1;
const STRERR_BUFSIZE: usize = 128;
const BUFSIZ: usize = 8192;

static mut iterations: c_int = 100;
static mut nr_events: c_int = 1;
static mut event_string: *const c_char = c"dummy".as_ptr();

#[repr(C)]
pub struct timeval {
    pub tv_sec: u64,
    pub tv_usec: u64,
}

#[repr(C)]
pub struct target {
    pub uses_mmap: bool,
    pub default_per_cpu: bool,
    pub system_wide: bool,
    pub cpu_list: *const c_char,
    pub pid: *const c_char,
    pub tid: *const c_char,
    pub per_thread: bool,
}

#[repr(C)]
pub struct record_opts {
    pub sample_time: bool,
    pub mmap_pages: c_uint,
    pub user_freq: c_uint,
    pub user_interval: u64,
    pub freq: c_int,
    pub target: target,
    pub mmap_flush: c_int,
    pub nr_threads_synthesize: c_int,
    pub ctl_fd: c_int,
    pub ctl_fd_ack: c_int,
    pub ignore_missing_thread: *const c_char,
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    pub nr: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub threads: *mut perf_thread_map,
    pub cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

static mut opts: record_opts = record_opts {
    sample_time: true,
    mmap_pages: UINT_MAX,
    user_freq: UINT_MAX,
    user_interval: ULLONG_MAX,
    freq: 4000,
    target: target {
        uses_mmap: true,
        default_per_cpu: true,
        system_wide: false,
        cpu_list: ptr::null(),
        pid: ptr::null(),
        tid: ptr::null(),
        per_thread: false,
    },
    mmap_flush: MMAP_FLUSH_DEFAULT,
    nr_threads_synthesize: 1,
    ctl_fd: -1,
    ctl_fd_ack: -1,
    ignore_missing_thread: ptr::null(),
};

unsafe extern "C" {
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evlist__do_munmap(evlist: *mut evlist);
    fn evlist__close(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;

    fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__print(err: *mut parse_events_error, evstr: *mut c_char);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events(evlist: *mut evlist, evstr: *mut c_char, err: *mut parse_events_error) -> c_int;
    fn parse_uid(uid: *const c_char) -> c_uint;
    fn parse_uid_filter(evlist: *mut evlist, uid: c_uint) -> c_int;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn strbuf_init(buf: *mut strbuf, hint: c_int) -> c_int;
    fn strbuf_add(buf: *mut strbuf, data: *const c_char, len: c_int) -> c_int;
    fn strbuf_addch(buf: *mut strbuf, c: c_int) -> c_int;
    fn strbuf_detach(buf: *mut strbuf, sz: *mut c_ulong) -> *mut c_char;
    fn strbuf_release(buf: *mut strbuf);
    fn strlen(s: *const c_char) -> c_ulong;
    fn free(ptr: *mut c_void);
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn timersub(a: *const timeval, b: *const timeval, res: *mut timeval);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: c_ulong) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn pr_debug(fmt: *const c_char, ...) -> c_int;
    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn exit(status: c_int) -> !;
    fn target__validate(target: *mut target) -> c_int;
    fn target__strerror(target: *mut target, errnum: c_int, buf: *mut c_char, buflen: c_ulong);
}

#[inline]
unsafe fn errno() -> c_int {
    *__errno_location()
}

#[inline]
unsafe fn timeval2usec(tv: *mut timeval) -> u64 {
    (*tv).tv_sec * USEC_PER_SEC + (*tv).tv_usec
}

unsafe fn evlist__count_evsel_fds(evlist: *mut evlist) -> c_int {
    let mut evsel = evlist__first(evlist);
    let mut cnt: c_int = 0;

    while !evsel.is_null() {
        cnt += (*(*evsel).core.threads).nr * perf_cpu_map__nr((*evsel).core.cpus);
        evsel = evlist__next(evlist, evsel);
    }

    cnt
}

unsafe fn bench__create_evlist(evstr: *mut c_char, uid_str: *const c_char) -> *mut evlist {
    let mut err = MaybeUninit::<parse_events_error>::uninit();
    let evlist = evlist__new();
    let mut ret: c_int;

    if evlist.is_null() {
        pr_err(c"Not enough memory to create evlist\n".as_ptr());
        return ptr::null_mut();
    }

    parse_events_error__init(err.as_mut_ptr());
    ret = parse_events(evlist, evstr, err.as_mut_ptr());
    if ret != 0 {
        parse_events_error__print(err.as_mut_ptr(), evstr);
        parse_events_error__exit(err.as_mut_ptr());
        pr_err(c"Run 'perf list' for a list of valid events\n".as_ptr());
        ret = 1;
        let _ = ret;
        evlist__put(evlist);
        return ptr::null_mut();
    }
    parse_events_error__exit(err.as_mut_ptr());
    if !uid_str.is_null() {
        let uid = parse_uid(uid_str);

        if uid == UINT_MAX {
            pr_err(c"Invalid User: %s".as_ptr(), uid_str);
            ret = -EINVAL;
            let _ = ret;
            evlist__put(evlist);
            return ptr::null_mut();
        }
        ret = parse_uid_filter(evlist, uid);
        if ret != 0 {
            evlist__put(evlist);
            return ptr::null_mut();
        }
    }
    ret = evlist__create_maps(evlist, &raw mut opts.target);
    if ret < 0 {
        pr_err(c"Not enough memory to create thread/cpu maps\n".as_ptr());
        evlist__put(evlist);
        return ptr::null_mut();
    }

    evlist__config(evlist, &raw mut opts, ptr::null_mut());

    evlist
}

unsafe fn bench__do_evlist_open_close(evlist: *mut evlist) -> c_int {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];
    let mut err = evlist__open(evlist);

    if err < 0 {
        pr_err(
            c"evlist__open: %s\n".as_ptr(),
            str_error_r(errno(), sbuf.as_mut_ptr(), sbuf.len() as c_ulong),
        );
        return err;
    }

    err = evlist__do_mmap(evlist, opts.mmap_pages);
    if err < 0 {
        pr_err(
            c"evlist__mmap: %s\n".as_ptr(),
            str_error_r(errno(), sbuf.as_mut_ptr(), sbuf.len() as c_ulong),
        );
        return err;
    }

    evlist__enable(evlist);
    evlist__disable(evlist);
    evlist__do_munmap(evlist);
    evlist__close(evlist);

    0
}

unsafe fn bench_evlist_open_close__run(evstr: *mut c_char, uid_str: *const c_char) -> c_int {
    // used to print statistics only
    let mut evlist = bench__create_evlist(evstr, uid_str);
    let time_average: f64;
    let time_stddev: f64;
    let mut start = MaybeUninit::<timeval>::uninit();
    let mut end = MaybeUninit::<timeval>::uninit();
    let mut diff = MaybeUninit::<timeval>::uninit();
    let mut time_stats = MaybeUninit::<stats>::uninit();
    let mut runtime_us: u64;
    let mut i: c_int;
    let mut err: c_int;

    if evlist.is_null() {
        return -ENOMEM;
    }

    init_stats(time_stats.as_mut_ptr());

    printf(
        c"  Number of cpus:\t%d\n".as_ptr(),
        perf_cpu_map__nr((*evlist__core(evlist)).user_requested_cpus),
    );
    printf(
        c"  Number of threads:\t%d\n".as_ptr(),
        (*(*evlist__core(evlist)).threads).nr,
    );
    printf(
        c"  Number of events:\t%d (%d fds)\n".as_ptr(),
        evlist__nr_entries(evlist),
        evlist__count_evsel_fds(evlist),
    );
    printf(c"  Number of iterations:\t%d\n".as_ptr(), iterations);

    evlist__put(evlist);

    i = 0;
    while i < iterations {
        pr_debug(c"Started iteration %d\n".as_ptr(), i);
        evlist = bench__create_evlist(evstr, uid_str);
        if evlist.is_null() {
            return -ENOMEM;
        }

        gettimeofday(start.as_mut_ptr(), ptr::null_mut());
        err = bench__do_evlist_open_close(evlist);
        if err != 0 {
            evlist__put(evlist);
            return err;
        }

        gettimeofday(end.as_mut_ptr(), ptr::null_mut());
        timersub(end.as_ptr(), start.as_ptr(), diff.as_mut_ptr());
        runtime_us = timeval2usec(diff.as_mut_ptr());
        update_stats(time_stats.as_mut_ptr(), runtime_us);

        evlist__put(evlist);
        pr_debug(c"Iteration %d took:\t%lluus\n".as_ptr(), i, runtime_us);
        i += 1;
    }

    time_average = avg_stats(time_stats.as_mut_ptr());
    time_stddev = stddev_stats(time_stats.as_mut_ptr());
    printf(
        c"  Average open-close took: %.3f usec (+- %.3f usec)\n".as_ptr(),
        time_average,
        time_stddev,
    );

    0
}

unsafe fn bench__repeat_event_string(evstr: *const c_char, n: c_int) -> *mut c_char {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];
    let mut buf = MaybeUninit::<strbuf>::uninit();
    let mut i: c_int;
    let str_size = strlen(evstr) as c_int;
    let final_size = str_size * n + n;
    let mut err = strbuf_init(buf.as_mut_ptr(), final_size);

    if err != 0 {
        pr_err(
            c"strbuf_init: %s\n".as_ptr(),
            str_error_r(err, sbuf.as_mut_ptr(), sbuf.len() as c_ulong),
        );
        strbuf_release(buf.as_mut_ptr());
        return ptr::null_mut();
    }

    i = 0;
    while i < n {
        err = strbuf_add(buf.as_mut_ptr(), evstr, str_size);
        if err != 0 {
            pr_err(
                c"strbuf_add: %s\n".as_ptr(),
                str_error_r(err, sbuf.as_mut_ptr(), sbuf.len() as c_ulong),
            );
            strbuf_release(buf.as_mut_ptr());
            return ptr::null_mut();
        }

        err = strbuf_addch(buf.as_mut_ptr(), if i == n - 1 { '\0' as c_int } else { ',' as c_int });
        if err != 0 {
            pr_err(
                c"strbuf_addch: %s\n".as_ptr(),
                str_error_r(err, sbuf.as_mut_ptr(), sbuf.len() as c_ulong),
            );
            strbuf_release(buf.as_mut_ptr());
            return ptr::null_mut();
        }

        i += 1;
    }

    strbuf_detach(buf.as_mut_ptr(), ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_evlist_open_close(
    mut argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let mut uid_str: *const c_char = ptr::null();
    // The C source initializes `options` with OPT_STRING/OPT_INTEGER/OPT_BOOLEAN/OPT_END
    // parse-options macros supplied by <subcmd/parse-options.h>. Keep the local array
    // as an external dependency placeholder while preserving option parsing flow.
    let options: [option; 0] = [];
    let bench_usage: [*const c_char; 2] = [
        c"perf bench internals evlist-open-close <options>".as_ptr(),
        ptr::null(),
    ];
    let mut evstr: *mut c_char;
    let mut errbuf = [0 as c_char; BUFSIZ];
    let mut err: c_int;

    argc = parse_options(argc, argv, options.as_ptr(), bench_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(bench_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    err = target__validate(&raw mut opts.target);
    if err != 0 {
        target__strerror(
            &raw mut opts.target,
            err,
            errbuf.as_mut_ptr(),
            errbuf.len() as c_ulong,
        );
        pr_err(c"%s\n".as_ptr(), errbuf.as_mut_ptr());
        return err;
    }

    /* Enable ignoring missing threads when -p option is defined. */
    opts.ignore_missing_thread = opts.target.pid;

    evstr = bench__repeat_event_string(event_string, nr_events);
    if evstr.is_null() {
        err = -ENOMEM;
        return err;
    }

    err = bench_evlist_open_close__run(evstr, uid_str);

    free(evstr as *mut c_void);
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
