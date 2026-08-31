// SPDX-License-Identifier: GPL-2.0

// C dependencies from:
// <subcmd/parse-options.h>, <linux/hw_breakpoint.h>, <linux/perf_event.h>,
// <linux/time64.h>, <sys/syscall.h>, <sys/ioctl.h>, <sys/time.h>,
// <pthread.h>, <stddef.h>, <stdlib.h>, <unistd.h>, <stdio.h>, <errno.h>,
// "bench.h", and "futex.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
struct option {
    _private: [u8; 0],
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct perf_event_attr {
    type_: c_uint,
    size: c_uint,
    config: u64,
    sample_period_or_freq: u64,
    sample_type: u64,
    read_format: u64,
    flags: u64,
    wakeup_events_or_watermark: u32,
    bp_type: u32,
    bp_addr: u64,
    bp_len: u64,
}

type pthread_t = c_ulong;

const EXIT_FAILURE: c_int = 1;
const ENODEV: c_int = 19;
const SYS_perf_event_open: c_long = 298;
const PERF_TYPE_BREAKPOINT: c_uint = 5;
const HW_BREAKPOINT_RW: u32 = 3;
const HW_BREAKPOINT_LEN_1: u64 = 1;
const PERF_EVENT_IOC_ENABLE: c_ulong = 9216;
const PERF_EVENT_IOC_DISABLE: c_ulong = 9217;
const BENCH_FORMAT_DEFAULT: c_int = 0;
const BENCH_FORMAT_SIMPLE: c_int = 1;
const USEC_PER_MSEC: c_long = 1000;
const USEC_PER_SEC: c_long = 1_000_000;

const ATTR_INHERIT: u64 = 1 << 1;
const ATTR_EXCLUDE_KERNEL: u64 = 1 << 5;
const ATTR_EXCLUDE_HV: u64 = 1 << 6;

unsafe extern "C" {
    static mut errno: c_int;
    static mut bench_repeat: c_int;
    static mut bench_format: c_int;
    static mut stderr: *mut c_void;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn futex_wait(uaddr: *mut c_uint, val: c_uint, timeout: *mut c_void, futex_flags: c_uint) -> c_int;
    fn futex_wake(uaddr: *mut c_uint, nr_wake: c_uint, futex_flags: c_uint) -> c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
}

#[repr(C)]
struct ThreadParams {
    nbreakpoints: c_uint,
    nparallel: c_uint,
    nthreads: c_uint,
}

static mut thread_params: ThreadParams = ThreadParams {
    nbreakpoints: 1,
    nparallel: 1,
    nthreads: 1,
};

// Original C initializes thread_options with OPT_UINTEGER/OPT_END macros.
// Those macro expansions depend on <subcmd/parse-options.h>.
static thread_options: [option; 0] = [];

static THREAD_USAGE_0: &[u8] = b"perf bench breakpoint thread <options>\0";
static thread_usage: [*const c_char; 2] = [THREAD_USAGE_0.as_ptr() as *const c_char, ptr::null()];

#[repr(C)]
struct breakpoint {
    fd: c_int,
    watched: c_char,
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1_000_000;
    }
}

unsafe fn atomic_load_relaxed_u32(ptr: *const c_uint) -> c_uint {
    core::ptr::read_volatile(ptr)
}

unsafe fn atomic_store_relaxed_u32(ptr: *mut c_uint, val: c_uint) {
    core::ptr::write_volatile(ptr, val);
}

unsafe fn atomic_fetch_sub_relaxed_i32(ptr: *mut c_int, val: c_int) -> c_int {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_sub(val));
    old
}

unsafe fn breakpoint_setup(addr: *mut c_void) -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut fd: c_int;

    attr.type_ = PERF_TYPE_BREAKPOINT;
    attr.size = size_of::<perf_event_attr>() as c_uint;
    attr.flags |= ATTR_INHERIT;
    attr.flags |= ATTR_EXCLUDE_KERNEL;
    attr.flags |= ATTR_EXCLUDE_HV;
    attr.bp_addr = addr as c_ulong as u64;
    attr.bp_type = HW_BREAKPOINT_RW;
    attr.bp_len = HW_BREAKPOINT_LEN_1;
    fd = syscall(
        SYS_perf_event_open,
        &mut attr as *mut perf_event_attr,
        0,
        -1,
        -1,
        0,
    ) as c_int;

    if fd < 0 {
        fd = -errno;
    }

    fd
}

unsafe extern "C" fn passive_thread(arg: *mut c_void) -> *mut c_void {
    let done = arg as *mut c_uint;

    while atomic_load_relaxed_u32(done) == 0 {
        futex_wait(done, 0, ptr::null_mut(), 0);
    }
    ptr::null_mut()
}

unsafe extern "C" fn active_thread(arg: *mut c_void) -> *mut c_void {
    let done = arg as *mut c_uint;

    while atomic_load_relaxed_u32(done) == 0 {}
    ptr::null_mut()
}

unsafe extern "C" fn breakpoint_thread(arg: *mut c_void) -> *mut c_void {
    let mut i: c_uint;
    let mut done: c_uint;
    let repeat = arg as *mut c_int;
    let threads: *mut pthread_t;

    threads = calloc(thread_params.nthreads as usize, size_of::<pthread_t>()) as *mut pthread_t;
    if threads.is_null() {
        perror(c"calloc".as_ptr());
        exit(EXIT_FAILURE);
    }

    while atomic_fetch_sub_relaxed_i32(repeat, 1) > 0 {
        done = 0;
        i = 0;
        while i < thread_params.nthreads {
            if pthread_create(
                threads.add(i as usize),
                ptr::null(),
                passive_thread,
                &mut done as *mut c_uint as *mut c_void,
            ) != 0
            {
                perror(c"pthread_create".as_ptr());
                exit(EXIT_FAILURE);
            }
            i += 1;
        }
        atomic_store_relaxed_u32(&mut done, 1);
        futex_wake(&mut done, thread_params.nthreads, 0);
        i = 0;
        while i < thread_params.nthreads {
            pthread_join(*threads.add(i as usize), ptr::null_mut());
            i += 1;
        }
    }
    free(threads as *mut c_void);
    ptr::null_mut()
}

// The benchmark creates nbreakpoints inheritable breakpoints,
// then starts nparallel threads which create and join bench_repeat batches of nthreads threads.
#[no_mangle]
pub unsafe extern "C" fn bench_breakpoint_thread(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut i: c_uint;
    let mut result_usec: c_uint;
    let mut repeat: c_int = bench_repeat;
    let breakpoints: *mut breakpoint;
    let parallel: *mut pthread_t;
    let mut start: timeval = core::mem::zeroed();
    let mut stop: timeval = core::mem::zeroed();
    let mut diff: timeval = core::mem::zeroed();

    if parse_options(argc, argv, thread_options.as_ptr(), thread_usage.as_ptr(), 0) != 0 {
        usage_with_options(thread_usage.as_ptr(), thread_options.as_ptr());
        exit(EXIT_FAILURE);
    }
    breakpoints = calloc(thread_params.nbreakpoints as usize, size_of::<breakpoint>()) as *mut breakpoint;
    parallel = calloc(thread_params.nparallel as usize, size_of::<pthread_t>()) as *mut pthread_t;
    if breakpoints.is_null() || parallel.is_null() {
        perror(c"calloc".as_ptr());
        exit(EXIT_FAILURE);
    }

    i = 0;
    while i < thread_params.nbreakpoints {
        (*breakpoints.add(i as usize)).fd =
            breakpoint_setup(&mut (*breakpoints.add(i as usize)).watched as *mut c_char as *mut c_void);

        if (*breakpoints.add(i as usize)).fd < 0 {
            if (*breakpoints.add(i as usize)).fd == -ENODEV {
                printf(c"Skipping perf bench breakpoint thread: No hardware support\n".as_ptr());
                return 0;
            }
            perror(c"perf_event_open".as_ptr());
            exit(EXIT_FAILURE);
        }
        i += 1;
    }
    gettimeofday(&mut start, ptr::null_mut());
    i = 0;
    while i < thread_params.nparallel {
        if pthread_create(
            parallel.add(i as usize),
            ptr::null(),
            breakpoint_thread,
            &mut repeat as *mut c_int as *mut c_void,
        ) != 0
        {
            perror(c"pthread_create".as_ptr());
            exit(EXIT_FAILURE);
        }
        i += 1;
    }
    i = 0;
    while i < thread_params.nparallel {
        pthread_join(*parallel.add(i as usize), ptr::null_mut());
        i += 1;
    }
    gettimeofday(&mut stop, ptr::null_mut());
    timersub(&stop, &start, &mut diff);
    i = 0;
    while i < thread_params.nbreakpoints {
        close((*breakpoints.add(i as usize)).fd);
        i += 1;
    }
    free(parallel as *mut c_void);
    free(breakpoints as *mut c_void);
    match bench_format {
        BENCH_FORMAT_DEFAULT => {
            printf(
                c"# Created/joined %d threads with %d breakpoints and %d parallelism\n".as_ptr(),
                bench_repeat,
                thread_params.nbreakpoints,
                thread_params.nparallel,
            );
            printf(
                c" %14s: %lu.%03lu [sec]\n\n".as_ptr(),
                c"Total time".as_ptr(),
                diff.tv_sec as c_long,
                (diff.tv_usec / USEC_PER_MSEC) as c_long,
            );
            result_usec = (diff.tv_sec * USEC_PER_SEC + diff.tv_usec) as c_uint;
            printf(
                c" %14lf usecs/op\n".as_ptr(),
                result_usec as f64 / bench_repeat as f64 / thread_params.nthreads as f64,
            );
            printf(
                c" %14lf usecs/op/cpu\n".as_ptr(),
                result_usec as f64 / bench_repeat as f64 / thread_params.nthreads as f64
                    * thread_params.nparallel as f64,
            );
        }
        BENCH_FORMAT_SIMPLE => {
            printf(
                c"%lu.%03lu\n".as_ptr(),
                diff.tv_sec as c_long,
                (diff.tv_usec / USEC_PER_MSEC) as c_long,
            );
        }
        _ => {
            fprintf(stderr, c"Unknown format: %d\n".as_ptr(), bench_format);
            exit(EXIT_FAILURE);
        }
    }
    0
}

#[repr(C)]
struct EnableParams {
    npassive: c_uint,
    nactive: c_uint,
}

static mut enable_params: EnableParams = EnableParams {
    nactive: 0,
    npassive: 0,
};

// Original C initializes enable_options with OPT_UINTEGER/OPT_END macros.
// Those macro expansions depend on <subcmd/parse-options.h>.
static enable_options: [option; 0] = [];

static ENABLE_USAGE_0: &[u8] = b"perf bench breakpoint enable <options>\0";
static enable_usage: [*const c_char; 2] = [ENABLE_USAGE_0.as_ptr() as *const c_char, ptr::null()];

// The benchmark creates an inheritable breakpoint,
// then starts npassive threads that block and nactive threads that actively spin
// and then disables and enables the breakpoint bench_repeat times.
#[no_mangle]
pub unsafe extern "C" fn bench_breakpoint_enable(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut i: c_uint;
    let nthreads: c_uint;
    let mut result_usec: c_uint;
    let mut done: c_uint = 0;
    let mut watched: c_char = 0;
    let fd: c_int;
    let threads: *mut pthread_t;
    let mut start: timeval = core::mem::zeroed();
    let mut stop: timeval = core::mem::zeroed();
    let mut diff: timeval = core::mem::zeroed();

    if parse_options(argc, argv, enable_options.as_ptr(), enable_usage.as_ptr(), 0) != 0 {
        usage_with_options(enable_usage.as_ptr(), enable_options.as_ptr());
        exit(EXIT_FAILURE);
    }
    fd = breakpoint_setup(&mut watched as *mut c_char as *mut c_void);

    if fd < 0 {
        if fd == -ENODEV {
            printf(c"Skipping perf bench breakpoint enable: No hardware support\n".as_ptr());
            return 0;
        }
        perror(c"perf_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }
    nthreads = enable_params.npassive + enable_params.nactive;
    threads = calloc(nthreads as usize, size_of::<pthread_t>()) as *mut pthread_t;
    if threads.is_null() {
        perror(c"calloc".as_ptr());
        exit(EXIT_FAILURE);
    }

    i = 0;
    while i < nthreads {
        if pthread_create(
            threads.add(i as usize),
            ptr::null(),
            if i < enable_params.npassive {
                passive_thread
            } else {
                active_thread
            },
            &mut done as *mut c_uint as *mut c_void,
        ) != 0
        {
            perror(c"pthread_create".as_ptr());
            exit(EXIT_FAILURE);
        }
        i += 1;
    }
    usleep(10000); // let the threads block
    gettimeofday(&mut start, ptr::null_mut());
    i = 0;
    while i < bench_repeat as c_uint {
        if ioctl(fd, PERF_EVENT_IOC_DISABLE, 0) != 0 {
            perror(c"ioctl(PERF_EVENT_IOC_DISABLE)".as_ptr());
            exit(EXIT_FAILURE);
        }
        if ioctl(fd, PERF_EVENT_IOC_ENABLE, 0) != 0 {
            perror(c"ioctl(PERF_EVENT_IOC_ENABLE)".as_ptr());
            exit(EXIT_FAILURE);
        }
        i += 1;
    }
    gettimeofday(&mut stop, ptr::null_mut());
    timersub(&stop, &start, &mut diff);
    atomic_store_relaxed_u32(&mut done, 1);
    futex_wake(&mut done, enable_params.npassive, 0);
    i = 0;
    while i < nthreads {
        pthread_join(*threads.add(i as usize), ptr::null_mut());
        i += 1;
    }
    free(threads as *mut c_void);
    close(fd);
    match bench_format {
        BENCH_FORMAT_DEFAULT => {
            printf(
                c"# Enabled/disabled breakpoint %d time with %d passive and %d active threads\n".as_ptr(),
                bench_repeat,
                enable_params.npassive,
                enable_params.nactive,
            );
            printf(
                c" %14s: %lu.%03lu [sec]\n\n".as_ptr(),
                c"Total time".as_ptr(),
                diff.tv_sec as c_long,
                (diff.tv_usec / USEC_PER_MSEC) as c_long,
            );
            result_usec = (diff.tv_sec * USEC_PER_SEC + diff.tv_usec) as c_uint;
            printf(c" %14lf usecs/op\n".as_ptr(), result_usec as f64 / bench_repeat as f64);
        }
        BENCH_FORMAT_SIMPLE => {
            printf(
                c"%lu.%03lu\n".as_ptr(),
                diff.tv_sec as c_long,
                (diff.tv_usec / USEC_PER_MSEC) as c_long,
            );
        }
        _ => {
            fprintf(stderr, c"Unknown format: %d\n".as_ptr(), bench_format);
            exit(EXIT_FAILURE);
        }
    }
    0
}
