// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/bench/inject-buildid.c.
// C header dependencies intentionally remain external to this isolated file:
// stdlib.h, stddef.h, ftw.h, fcntl.h, errno.h, unistd.h, pthread.h,
// sys/mman.h, sys/wait.h, linux/kernel.h, linux/time64.h, linux/list.h,
// linux/err.h, linux/zalloc.h, internal/lib.h, subcmd/parse-options.h,
// bench.h, util/data.h, util/stat.h, util/debug.h, util/symbol.h,
// util/session.h, util/build-id.h, util/sample.h, util/synthetic-events.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u64 = u64;
type u16 = u16;
type size_t = usize;
type ssize_t = isize;
type pthread_t = usize;
type time_t = isize;
type suseconds_t = isize;

const MMAP_DEV_MAJOR: c_int = 8;
const DSO_MMAP_RATIO: c_uint = 4;

static mut iterations: c_uint = 100;
static mut nr_mmaps: c_uint = 100;
static mut nr_samples: c_uint = 100; /* samples per mmap */

static mut bench_sample_type: u64 = 0;
static mut bench_id_hdr_size: u16 = 0;

#[repr(C)]
struct bench_data {
    pid: c_int,
    input_pipe: [c_int; 2],
    output_pipe: [c_int; 2],
    th: pthread_t,
}

#[repr(C)]
struct bench_dso {
    list: list_head,
    name: *mut c_char,
    ino: c_int,
}

static mut nr_dsos: c_int = 0;
static mut dsos: *mut bench_dso = null_mut();

extern "C" {
    fn main(argc: c_int, argv: *mut *const c_char) -> c_int;
}

extern "C" {
    static mut verbose: c_int;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn rand() -> c_int;
    fn srand(seed: c_uint);
    fn time(tloc: *mut time_t) -> time_t;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn usleep(usec: c_uint) -> c_int;
    fn wait4(pid: c_int, wstatus: *mut c_int, options: c_int, rusage: *mut rusage) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn nftw(
        dirpath: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *const stat, c_int, *mut FTW) -> c_int,
        nopenfd: c_int,
        flags: c_int,
    ) -> c_int;

    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
    fn perf_event__sample_event_size(
        sample: *const perf_sample,
        sample_type: u64,
        read_format: u64,
        branch_sample_type: u64,
    ) -> u16;
    fn perf_event__synthesize_sample(
        event: *mut perf_event,
        sample_type: u64,
        read_format: u64,
        branch_sample_type: u64,
        sample: *const perf_sample,
    );
    fn perf_header__write_pipe(fd: c_int) -> c_int;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn symbol__init(arg: *mut c_void) -> c_int;
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn zfree(ptr: *mut *mut c_char);
    fn __errno_location() -> *mut c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct stat {
    _private: [u8; 0],
}

#[repr(C)]
struct FTW {
    _private: [u8; 0],
}

#[repr(C)]
struct build_id {
    size: c_uint,
}

#[repr(C)]
struct perf_event_header {
    type_: u32,
    misc: u16,
    size: u16,
}

#[repr(C)]
struct perf_event_attr {
    type_: u32,
    config: u64,
    exclude_kernel: u64,
    sample_id_all: u64,
    sample_type: u64,
}

#[repr(C)]
struct perf_record_attr {
    header: perf_event_header,
    attr: perf_event_attr,
    id: [u64; 1],
}

#[repr(C)]
struct perf_record_fork {
    header: perf_event_header,
    pid: u32,
    ppid: u32,
    tid: u32,
    ptid: u32,
    time: u64,
}

#[repr(C)]
struct perf_record_mmap2 {
    header: perf_event_header,
    pid: u32,
    tid: u32,
    start: u64,
    len: u64,
    pgoff: u64,
    maj: u32,
    min: u32,
    ino: u64,
    ino_generation: u64,
    prot: u32,
    flags: u32,
    filename: [c_char; 4096],
}

#[repr(C)]
union perf_event {
    header: core::mem::ManuallyDrop<perf_event_header>,
    attr: core::mem::ManuallyDrop<perf_record_attr>,
    fork: core::mem::ManuallyDrop<perf_record_fork>,
    mmap2: core::mem::ManuallyDrop<perf_record_mmap2>,
}

#[repr(C)]
struct perf_sample {
    tid: c_int,
    pid: c_int,
    ip: u64,
    time: u64,
}

#[repr(C)]
struct rusage {
    ru_maxrss: isize,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
struct stats {
    _private: [u8; 0],
}

#[repr(C)]
struct option {
    _private: [u8; 0],
}

const FTW_D: c_int = 1;
const FTW_SL: c_int = 4;
const FTW_PHYS: c_int = 1;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const O_NONBLOCK: c_int = 0o4000;
const O_WRONLY: c_int = 1;
const STDIN_FILENO: c_int = 0;
const STDOUT_FILENO: c_int = 1;
const STDERR_FILENO: c_int = 2;
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;
const EXIT_FAILURE: c_int = 1;
const PROT_EXEC: u32 = 0x4;
const USEC_PER_SEC: u64 = 1_000_000;
const USEC_PER_MSEC: f64 = 1000.0;

const PERF_RECORD_HEADER_ATTR: u32 = 66;
const PERF_RECORD_FORK: u32 = 7;
const PERF_RECORD_MMAP2: u32 = 10;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_FINISHED_ROUND: u32 = 68;
const PERF_RECORD_MISC_FORK_EXEC: u16 = 0x2000;
const PERF_RECORD_MISC_USER: u16 = 1 << 13;
const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_COUNT_SW_TASK_CLOCK: u64 = 1;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;

const fn roundup(x: size_t, y: size_t) -> size_t {
    (((x) + ((y) - 1)) / (y)) * (y)
}

unsafe fn min_size(a: size_t, b: size_t) -> size_t {
    if a < b { a } else { b }
}

/* The original C file initializes this with OPT_* macros from parse-options.h. */
static options: [option; 1] = [option { _private: [] }];

static bench_usage_0: &[u8] = b"perf bench internals inject-build-id <options>\0";
static bench_usage: [*const c_char; 2] = [bench_usage_0.as_ptr() as *const c_char, null()];

/*
 * Helper for collect_dso that adds the given file as a dso to dso_list
 * if it contains a build-id.  Stops after collecting 4 times more than
 * we need (for MMAP2 events).
 */
unsafe extern "C" fn add_dso(
    fpath: *const c_char,
    _sb: *const stat,
    typeflag: c_int,
    _ftwbuf: *mut FTW,
) -> c_int {
    let dso = dsos.add(nr_dsos as usize);
    let mut bid = build_id { size: 0 };

    if typeflag == FTW_D || typeflag == FTW_SL {
        return 0;
    }

    if filename__read_build_id(fpath, &mut bid) < 0 {
        return 0;
    }

    (*dso).name = realpath(fpath, null_mut());
    if (*dso).name.is_null() {
        return -1;
    }

    (*dso).ino = nr_dsos;
    nr_dsos += 1;
    pr_debug2(b"  Adding DSO: %s\n\0".as_ptr() as *const c_char, fpath);

    /* stop if we collected enough DSOs */
    if nr_dsos as c_uint == DSO_MMAP_RATIO * nr_mmaps {
        return 1;
    }

    0
}

unsafe fn collect_dso() {
    dsos = calloc(
        (nr_mmaps * DSO_MMAP_RATIO) as size_t,
        size_of::<bench_dso>(),
    ) as *mut bench_dso;
    if dsos.is_null() {
        printf(b"  Memory allocation failed\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if nftw(
        b"/usr/lib/\0".as_ptr() as *const c_char,
        add_dso,
        10,
        FTW_PHYS,
    ) < 0
    {
        return;
    }

    pr_debug(b"  Collected %d DSOs\n\0".as_ptr() as *const c_char, nr_dsos);
}

unsafe fn release_dso() {
    let mut i: c_int = 0;

    while i < nr_dsos {
        let dso = dsos.add(i as usize);

        zfree(&mut (*dso).name);
        i += 1;
    }
    free(dsos as *mut c_void);
}

/* Fake address used by mmap and sample events */
unsafe fn dso_map_addr(dso: *mut bench_dso) -> u64 {
    0x400000u64 + ((*dso).ino as u64) * 8192u64
}

unsafe fn synthesize_attr(data: *mut bench_data) -> ssize_t {
    let mut event: perf_event = zeroed();

    memset(
        &mut event as *mut _ as *mut c_void,
        0,
        size_of::<perf_record_attr>() + size_of::<u64>(),
    );

    (*event.header).type_ = PERF_RECORD_HEADER_ATTR;
    (*event.header).size = (size_of::<perf_record_attr>() + size_of::<u64>()) as u16;

    (*event.attr).attr.type_ = PERF_TYPE_SOFTWARE;
    (*event.attr).attr.config = PERF_COUNT_SW_TASK_CLOCK;
    (*event.attr).attr.exclude_kernel = 1;
    (*event.attr).attr.sample_id_all = 1;
    (*event.attr).attr.sample_type = bench_sample_type;

    writen(
        (*data).input_pipe[1],
        &event as *const _ as *const c_void,
        (*event.header).size as size_t,
    )
}

unsafe fn synthesize_fork(data: *mut bench_data) -> ssize_t {
    let mut event: perf_event = zeroed();

    memset(
        &mut event as *mut _ as *mut c_void,
        0,
        size_of::<perf_record_fork>() + bench_id_hdr_size as size_t,
    );

    (*event.header).type_ = PERF_RECORD_FORK;
    (*event.header).misc = PERF_RECORD_MISC_FORK_EXEC;
    (*event.header).size = (size_of::<perf_record_fork>() + bench_id_hdr_size as size_t) as u16;

    (*event.fork).ppid = 1;
    (*event.fork).ptid = 1;
    (*event.fork).pid = (*data).pid as u32;
    (*event.fork).tid = (*data).pid as u32;

    writen(
        (*data).input_pipe[1],
        &event as *const _ as *const c_void,
        (*event.header).size as size_t,
    )
}

unsafe fn synthesize_mmap(data: *mut bench_data, dso: *mut bench_dso, timestamp: u64) -> ssize_t {
    let mut event: perf_event = zeroed();
    let mut len: size_t = core::mem::offset_of!(perf_record_mmap2, filename);
    let id_hdr_ptr = &mut event as *mut _ as *mut u64;
    let ts_idx: c_int;

    len += roundup(strlen((*dso).name) + 1, 8) + bench_id_hdr_size as size_t;

    memset(
        &mut event as *mut _ as *mut c_void,
        0,
        min_size(len, size_of::<perf_record_mmap2>()),
    );

    (*event.header).type_ = PERF_RECORD_MMAP2;
    (*event.header).misc = PERF_RECORD_MISC_USER;
    (*event.header).size = len as u16;

    (*event.mmap2).pid = (*data).pid as u32;
    (*event.mmap2).tid = (*data).pid as u32;
    (*event.mmap2).maj = MMAP_DEV_MAJOR as u32;
    (*event.mmap2).ino = (*dso).ino as u64;

    strcpy((*event.mmap2).filename.as_mut_ptr(), (*dso).name);

    (*event.mmap2).start = dso_map_addr(dso);
    (*event.mmap2).len = 4096;
    (*event.mmap2).prot = PROT_EXEC;

    if len > size_of::<perf_record_mmap2>() {
        /* write mmap2 event first */
        if writen(
            (*data).input_pipe[1],
            &event as *const _ as *const c_void,
            len - bench_id_hdr_size as size_t,
        ) < 0
        {
            return -1;
        }
        /* zero-fill sample id header */
        memset(id_hdr_ptr as *mut c_void, 0, bench_id_hdr_size as size_t);
        /* put timestamp in the right position */
        ts_idx = (bench_id_hdr_size as size_t / size_of::<u64>()) as c_int - 2;
        *id_hdr_ptr.add(ts_idx as usize) = timestamp;
        if writen(
            (*data).input_pipe[1],
            id_hdr_ptr as *const c_void,
            bench_id_hdr_size as size_t,
        ) < 0
        {
            return -1;
        }

        return len as ssize_t;
    }

    ts_idx = (len / size_of::<u64>()) as c_int - 2;
    *id_hdr_ptr.add(ts_idx as usize) = timestamp;
    writen(
        (*data).input_pipe[1],
        &event as *const _ as *const c_void,
        len,
    )
}

unsafe fn synthesize_sample(data: *mut bench_data, dso: *mut bench_dso, timestamp: u64) -> ssize_t {
    let mut event: perf_event = zeroed();
    let sample = perf_sample {
        tid: (*data).pid,
        pid: (*data).pid,
        ip: dso_map_addr(dso),
        time: timestamp,
    };

    (*event.header).type_ = PERF_RECORD_SAMPLE;
    (*event.header).misc = PERF_RECORD_MISC_USER;
    (*event.header).size =
        perf_event__sample_event_size(&sample, bench_sample_type, 0, 0);
    perf_event__synthesize_sample(&mut event, bench_sample_type, 0, 0, &sample);

    writen(
        (*data).input_pipe[1],
        &event as *const _ as *const c_void,
        (*event.header).size as size_t,
    )
}

unsafe fn synthesize_flush(data: *mut bench_data) -> ssize_t {
    let header = perf_event_header {
        size: size_of::<perf_event_header>() as u16,
        type_: PERF_RECORD_FINISHED_ROUND,
        misc: 0,
    };

    writen(
        (*data).input_pipe[1],
        &header as *const _ as *const c_void,
        header.size as size_t,
    )
}

unsafe extern "C" fn data_reader(arg: *mut c_void) -> *mut c_void {
    let data = arg as *mut bench_data;
    let mut buf = [0i8; 8192];
    let flag: c_int;
    let mut n: c_int;

    flag = fcntl((*data).output_pipe[0], F_GETFL);
    fcntl((*data).output_pipe[0], F_SETFL, flag | O_NONBLOCK);

    /* read out data from child */
    loop {
        n = read(
            (*data).output_pipe[0],
            buf.as_mut_ptr() as *mut c_void,
            size_of::<[i8; 8192]>(),
        ) as c_int;
        if n > 0 {
            continue;
        }
        if n == 0 {
            break;
        }

        if *__errno_location() != EINTR && *__errno_location() != EAGAIN {
            break;
        }

        usleep(100);
    }

    close((*data).output_pipe[0]);
    null_mut()
}

unsafe fn setup_injection(data: *mut bench_data, build_id_all: bool) -> c_int {
    let mut ready_pipe = [0 as c_int; 2];
    let dev_null_fd: c_int;
    let mut buf: c_char = 0;

    if pipe(ready_pipe.as_mut_ptr()) < 0 {
        return -1;
    }

    if pipe((*data).input_pipe.as_mut_ptr()) < 0 {
        return -1;
    }

    if pipe((*data).output_pipe.as_mut_ptr()) < 0 {
        return -1;
    }

    (*data).pid = fork();
    if (*data).pid < 0 {
        return -1;
    }

    if (*data).pid == 0 {
        let mut inject_argv: *mut *const c_char;
        let mut inject_argc: c_int = 3;

        close((*data).input_pipe[1]);
        close((*data).output_pipe[0]);
        close(ready_pipe[0]);

        dup2((*data).input_pipe[0], STDIN_FILENO);
        close((*data).input_pipe[0]);
        dup2((*data).output_pipe[1], STDOUT_FILENO);
        close((*data).output_pipe[1]);

        dev_null_fd = open(b"/dev/null\0".as_ptr() as *const c_char, O_WRONLY);
        if dev_null_fd < 0 {
            exit(1);
        }

        dup2(dev_null_fd, STDERR_FILENO);

        if build_id_all {
            inject_argc += 1;
        }

        inject_argv = calloc((inject_argc + 1) as size_t, size_of::<*const c_char>())
            as *mut *const c_char;
        if inject_argv.is_null() {
            exit(1);
        }

        *inject_argv.add(0) = strdup(b"perf\0".as_ptr() as *const c_char);
        *inject_argv.add(1) = strdup(b"inject\0".as_ptr() as *const c_char);
        *inject_argv.add(2) = strdup(b"-b\0".as_ptr() as *const c_char);
        if build_id_all {
            *inject_argv.add(3) = strdup(b"--buildid-all\0".as_ptr() as *const c_char);
        }

        /* signal that we're ready to go */
        close(ready_pipe[1]);

        main(inject_argc, inject_argv);

        exit(0);
    }

    pthread_create(&mut (*data).th, null(), data_reader, data as *mut c_void);

    close(ready_pipe[1]);
    close((*data).input_pipe[0]);
    close((*data).output_pipe[1]);

    /* wait for child ready */
    if read(ready_pipe[0], &mut buf as *mut _ as *mut c_void, 1) < 0 {
        return -1;
    }
    close(ready_pipe[0]);

    0
}

unsafe fn inject_build_id(data: *mut bench_data, max_rss: *mut u64) -> c_int {
    let mut status: c_int = 0;
    let mut i: c_uint;
    let mut k: c_uint;
    let mut rusage: rusage = zeroed();

    /* this makes the child to run */
    if perf_header__write_pipe((*data).input_pipe[1]) < 0 {
        return -1;
    }

    if synthesize_attr(data) < 0 {
        return -1;
    }

    if synthesize_fork(data) < 0 {
        return -1;
    }

    i = 0;
    while i < nr_mmaps {
        let idx = rand() % nr_dsos;
        let dso = dsos.add(idx as usize);
        let timestamp = (rand() % 1000000) as u64;

        pr_debug2(
            b"   [%d] injecting: %s\n\0".as_ptr() as *const c_char,
            i + 1,
            (*dso).name,
        );
        if synthesize_mmap(data, dso, timestamp) < 0 {
            return -1;
        }

        k = 0;
        while k < nr_samples {
            if synthesize_sample(data, dso, timestamp + k as u64 * 1000) < 0 {
                return -1;
            }
            k += 1;
        }

        if (i + 1) % 10 == 0 {
            if synthesize_flush(data) < 0 {
                return -1;
            }
        }
        i += 1;
    }

    /* this makes the child to finish */
    close((*data).input_pipe[1]);

    wait4((*data).pid, &mut status, 0, &mut rusage);
    *max_rss = rusage.ru_maxrss as u64;

    pr_debug(
        b"   Child %d exited with %d\n\0".as_ptr() as *const c_char,
        (*data).pid,
        status,
    );

    0
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += USEC_PER_SEC as suseconds_t;
    }
}

unsafe fn do_inject_loop(data: *mut bench_data, build_id_all: bool) {
    let mut i: c_uint;
    let mut time_stats: stats = zeroed();
    let mut mem_stats: stats = zeroed();
    let mut time_average: f64;
    let mut time_stddev: f64;
    let mem_average: f64;
    let mem_stddev: f64;

    init_stats(&mut time_stats);
    init_stats(&mut mem_stats);

    pr_debug(
        b"  Build-id%s injection benchmark\n\0".as_ptr() as *const c_char,
        if build_id_all {
            b"-all\0".as_ptr()
        } else {
            b"\0".as_ptr()
        } as *const c_char,
    );

    i = 0;
    while i < iterations {
        let mut start: timeval = zeroed();
        let mut end: timeval = zeroed();
        let mut diff: timeval = zeroed();
        let runtime_us: u64;
        let mut max_rss: u64 = 0;

        pr_debug(b"  Iteration #%d\n\0".as_ptr() as *const c_char, i + 1);

        if setup_injection(data, build_id_all) < 0 {
            printf(b"  Build-id injection setup failed\n\0".as_ptr() as *const c_char);
            break;
        }

        gettimeofday(&mut start, null_mut());
        if inject_build_id(data, &mut max_rss) < 0 {
            printf(b"  Build-id injection failed\n\0".as_ptr() as *const c_char);
            break;
        }

        gettimeofday(&mut end, null_mut());
        timersub(&end, &start, &mut diff);
        runtime_us = diff.tv_sec as u64 * USEC_PER_SEC + diff.tv_usec as u64;
        update_stats(&mut time_stats, runtime_us);
        update_stats(&mut mem_stats, max_rss);

        pthread_join((*data).th, null_mut());
        i += 1;
    }

    time_average = avg_stats(&mut time_stats) / USEC_PER_MSEC;
    time_stddev = stddev_stats(&mut time_stats) / USEC_PER_MSEC;
    printf(
        b"  Average build-id%s injection took: %.3f msec (+- %.3f msec)\n\0".as_ptr()
            as *const c_char,
        if build_id_all {
            b"-all\0".as_ptr()
        } else {
            b"\0".as_ptr()
        } as *const c_char,
        time_average,
        time_stddev,
    );

    /* each iteration, it processes MMAP2 + BUILD_ID + nr_samples * SAMPLE */
    time_average = avg_stats(&mut time_stats) / (nr_mmaps * (nr_samples + 2)) as f64;
    time_stddev = stddev_stats(&mut time_stats) / (nr_mmaps * (nr_samples + 2)) as f64;
    printf(
        b"  Average time per event: %.3f usec (+- %.3f usec)\n\0".as_ptr() as *const c_char,
        time_average,
        time_stddev,
    );

    mem_average = avg_stats(&mut mem_stats);
    mem_stddev = stddev_stats(&mut mem_stats);
    printf(
        b"  Average memory usage: %.0f KB (+- %.0f KB)\n\0".as_ptr() as *const c_char,
        mem_average,
        mem_stddev,
    );
}

unsafe fn do_inject_loops(data: *mut bench_data) -> c_int {
    srand(time(null_mut()) as c_uint);
    symbol__init(null_mut());

    bench_sample_type = PERF_SAMPLE_IDENTIFIER | PERF_SAMPLE_IP;
    bench_sample_type |= PERF_SAMPLE_TID | PERF_SAMPLE_TIME;
    bench_id_hdr_size = 32;

    collect_dso();
    if nr_dsos == 0 {
        printf(b"  Cannot collect DSOs for injection\n\0".as_ptr() as *const c_char);
        return -1;
    }

    do_inject_loop(data, false);
    do_inject_loop(data, true);

    release_dso();
    0
}

#[no_mangle]
pub unsafe extern "C" fn bench_inject_build_id(argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut data: bench_data = zeroed();
    let mut argc = argc;

    argc = parse_options(argc, argv, options.as_ptr(), bench_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(bench_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    do_inject_loops(&mut data)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
