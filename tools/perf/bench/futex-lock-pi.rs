// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Davidlohr Bueso.
 */

/* Original C dependencies:
 * string.h, pthread.h, signal.h, ../util/mutex.h, ../util/stat.h,
 * subcmd/parse-options.h, linux/compiler.h, linux/kernel.h, linux/zalloc.h,
 * errno.h, perf/cpumap.h, bench.h, futex.h, err.h, stdlib.h, sys/time.h,
 * sys/mman.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u_int32_t = u32;
type size_t = usize;
type pthread_t = c_ulong;
type bool_ = bool;

#[repr(C)]
struct worker {
    tid: c_int,
    futex: *mut u_int32_t,
    thread: pthread_t,
    ops: c_ulong,
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct cond {
    _private: [u8; 0],
}

#[repr(C)]
struct stats {
    _private: [u8; 0],
}

#[repr(C)]
struct bench_futex_parameters {
    nbuckets: c_int,
    nthreads: c_uint,
    runtime: c_uint,
    multi: bool_,
    silent: bool_,
    fshared: bool_,
    mlockall: bool_,
}

#[repr(C)]
struct option {
    _private: [u8; 0],
}

#[repr(C)]
struct timeval {
    tv_sec: isize,
    tv_usec: isize,
}

#[repr(C)]
struct sigset_t {
    _private: [u8; 0],
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

type sighandler_t = Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>;

#[repr(C)]
struct sigaction {
    sa_sigaction: sighandler_t,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct perf_cpu {
    cpu: c_int,
}

#[repr(C)]
struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
struct pthread_attr_t {
    _private: [u8; 0],
}

const SIGINT: c_int = 2;
const EXIT_FAILURE: c_int = 1;
const MCL_CURRENT: c_int = 1;
const MCL_FUTURE: c_int = 2;
const FUTEX_PRIVATE_FLAG: c_int = 128;

static mut global_futex: u_int32_t = 0;
static mut worker: *mut worker = ptr::null_mut();
static mut done: bool_ = false;
static mut futex_flag: c_int = 0;
static mut thread_lock: mutex = mutex { _private: [] };
static mut threads_starting: c_uint = 0;
static mut throughput_stats: stats = stats { _private: [] };
static mut thread_parent: cond = cond { _private: [] };
static mut thread_worker: cond = cond { _private: [] };

static mut params: bench_futex_parameters = bench_futex_parameters {
    nbuckets: -1,
    nthreads: 0,
    runtime: 10,
    multi: false,
    silent: false,
    fshared: false,
    mlockall: false,
};

/* The original C file initializes this with OPT_* macros. */
static options: [option; 1] = [option { _private: [] }];

static bench_futex_lock_pi_usage: [*const c_char; 2] = [
    b"perf bench futex lock-pi <options>\0".as_ptr() as *const c_char,
    ptr::null(),
];

unsafe extern "C" {
    static mut bench__start: timeval;
    static mut bench__end: timeval;
    static mut bench__runtime: timeval;

    fn avg_stats(stats: *mut stats) -> c_ulong;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn rel_stddev_stats(stddev: f64, avg: c_ulong) -> f64;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: c_ulong);

    fn futex_print_nbuckets(params: *mut bench_futex_parameters);
    fn futex_set_nbuckets_param(params: *mut bench_futex_parameters);
    fn futex_lock_pi(uaddr: *mut u_int32_t, timeout: *mut c_void, opflags: c_int) -> c_int;
    fn futex_unlock_pi(uaddr: *mut u_int32_t, opflags: c_int) -> c_int;

    fn mutex_init(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn mutex_destroy(mutex: *mut mutex);
    fn cond_init(cond: *mut cond);
    fn cond_wait(cond: *mut cond, mutex: *mut mutex);
    fn cond_signal(cond: *mut cond);
    fn cond_broadcast(cond: *mut cond);
    fn cond_destroy(cond: *mut cond);

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;

    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(map: *mut perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(map: *mut perf_cpu_map, idx: c_uint) -> perf_cpu;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn cpu__max_cpu() -> perf_cpu;

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_FREE(cpusetp: *mut cpu_set_t);
    fn BUG_ON(condition: bool_);

    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(
        attr: *mut pthread_attr_t,
        cpusetsize: size_t,
        cpuset: *const cpu_set_t,
    ) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn warn(format: *const c_char, ...);
    fn err(eval: c_int, format: *const c_char, ...) -> !;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zfree(ptr: *mut *mut u_int32_t);
    fn exit(status: c_int) -> !;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn getpid() -> c_int;
    fn mlockall(flags: c_int) -> c_int;
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1_000_000;
    }
}

unsafe fn print_summary() {
    let avg: c_ulong = avg_stats(ptr::addr_of_mut!(throughput_stats));
    let stddev: f64 = stddev_stats(ptr::addr_of_mut!(throughput_stats));

    printf(
        b"%sAveraged %ld operations/sec (+- %.2f%%), total secs = %d\n\0".as_ptr()
            as *const c_char,
        if !params.silent {
            b"\n\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        avg,
        rel_stddev_stats(stddev, avg),
        bench__runtime.tv_sec as c_int,
    );
    futex_print_nbuckets(ptr::addr_of_mut!(params));
}

unsafe extern "C" fn toggle_done(
    _sig: c_int,
    _info: *mut siginfo_t,
    _uc: *mut c_void,
) {
    /* inform all threads that we're done for the day */
    done = true;
    gettimeofday(ptr::addr_of_mut!(bench__end), ptr::null_mut());
    timersub(
        ptr::addr_of!(bench__end),
        ptr::addr_of!(bench__start),
        ptr::addr_of_mut!(bench__runtime),
    );
}

unsafe extern "C" fn workerfn(arg: *mut c_void) -> *mut c_void {
    let w: *mut worker = arg as *mut worker;
    let mut ops: c_ulong = (*w).ops;

    mutex_lock(ptr::addr_of_mut!(thread_lock));
    threads_starting -= 1;
    if threads_starting == 0 {
        cond_signal(ptr::addr_of_mut!(thread_parent));
    }
    cond_wait(
        ptr::addr_of_mut!(thread_worker),
        ptr::addr_of_mut!(thread_lock),
    );
    mutex_unlock(ptr::addr_of_mut!(thread_lock));

    loop {
        let mut ret: c_int;
        loop {
            ret = futex_lock_pi((*w).futex, ptr::null_mut(), futex_flag);

            if ret != 0 {
                /* handle lock acquisition */
                if !params.silent {
                    warn(
                        b"thread %d: Could not lock pi-lock for %p (%d)\0".as_ptr()
                            as *const c_char,
                        (*w).tid,
                        (*w).futex,
                        ret,
                    );
                }
                if done {
                    break;
                }

                continue;
            }
            break;
        }

        if ret != 0 && done {
            break;
        }

        usleep(1);
        ret = futex_unlock_pi((*w).futex, futex_flag);
        if ret != 0 && !params.silent {
            warn(
                b"thread %d: Could not unlock pi-lock for %p (%d)\0".as_ptr() as *const c_char,
                (*w).tid,
                (*w).futex,
                ret,
            );
        }
        ops += 1; /* account for thread's share of work */

        if done {
            break;
        }
    }

    (*w).ops = ops;
    ptr::null_mut()
}

unsafe fn create_threads(w: *mut worker, cpu: *mut perf_cpu_map) {
    let cpuset: *mut cpu_set_t;
    let mut i: c_uint;
    let nrcpus: c_int = cpu__max_cpu().cpu;
    let size: size_t;

    threads_starting = params.nthreads;

    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(cpuset.is_null());
    size = CPU_ALLOC_SIZE(nrcpus);

    i = 0;
    while i < params.nthreads {
        let mut thread_attr: pthread_attr_t = mem::zeroed();

        pthread_attr_init(&mut thread_attr);
        (*worker.add(i as usize)).tid = i as c_int;

        if params.multi {
            (*worker.add(i as usize)).futex = calloc(1, mem::size_of::<u_int32_t>()) as *mut u_int32_t;
            if (*worker.add(i as usize)).futex.is_null() {
                err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
            }
        } else {
            (*worker.add(i as usize)).futex = ptr::addr_of_mut!(global_futex);
        }

        CPU_ZERO_S(size, cpuset);
        CPU_SET_S(
            perf_cpu_map__cpu(cpu, i % perf_cpu_map__nr(cpu)).cpu,
            size,
            cpuset,
        );

        if pthread_attr_setaffinity_np(&mut thread_attr, size, cpuset) != 0 {
            CPU_FREE(cpuset);
            err(
                EXIT_FAILURE,
                b"pthread_attr_setaffinity_np\0".as_ptr() as *const c_char,
            );
        }

        if pthread_create(
            ptr::addr_of_mut!((*w.add(i as usize)).thread),
            &thread_attr,
            workerfn,
            ptr::addr_of_mut!(*worker.add(i as usize)) as *mut c_void,
        ) != 0
        {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, b"pthread_create\0".as_ptr() as *const c_char);
        }
        pthread_attr_destroy(&mut thread_attr);
        i += 1;
    }
    CPU_FREE(cpuset);
}

#[no_mangle]
pub unsafe extern "C" fn bench_futex_lock_pi(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut argc = argc;
    let mut ret: c_int = 0;
    let mut i: c_uint;
    let mut act: sigaction = mem::zeroed();
    let cpu: *mut perf_cpu_map;

    argc = parse_options(
        argc,
        argv,
        options.as_ptr(),
        bench_futex_lock_pi_usage.as_ptr(),
        0,
    );
    if argc != 0 {
        usage_with_options(bench_futex_lock_pi_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    cpu = perf_cpu_map__new_online_cpus();
    if cpu.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
    }

    memset(
        ptr::addr_of_mut!(act) as *mut c_void,
        0,
        mem::size_of::<sigaction>(),
    );
    sigfillset(ptr::addr_of_mut!(act.sa_mask));
    act.sa_sigaction = Some(toggle_done);
    sigaction(SIGINT, ptr::addr_of!(act), ptr::null_mut());

    if params.mlockall {
        if mlockall(MCL_CURRENT | MCL_FUTURE) != 0 {
            err(EXIT_FAILURE, b"mlockall\0".as_ptr() as *const c_char);
        }
    }

    if params.nthreads == 0 {
        params.nthreads = perf_cpu_map__nr(cpu);
    }

    worker = calloc(params.nthreads as size_t, mem::size_of::<worker>()) as *mut worker;
    if worker.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
    }

    if !params.fshared {
        futex_flag = FUTEX_PRIVATE_FLAG;
    }

    printf(
        b"Run summary [PID %d]: %d threads doing pi lock/unlock pairing for %d secs.\n\n\0"
            .as_ptr() as *const c_char,
        getpid(),
        params.nthreads,
        params.runtime,
    );

    init_stats(ptr::addr_of_mut!(throughput_stats));
    mutex_init(ptr::addr_of_mut!(thread_lock));
    cond_init(ptr::addr_of_mut!(thread_parent));
    cond_init(ptr::addr_of_mut!(thread_worker));
    futex_set_nbuckets_param(ptr::addr_of_mut!(params));

    threads_starting = params.nthreads;
    gettimeofday(ptr::addr_of_mut!(bench__start), ptr::null_mut());

    create_threads(worker, cpu);

    mutex_lock(ptr::addr_of_mut!(thread_lock));
    while threads_starting != 0 {
        cond_wait(
            ptr::addr_of_mut!(thread_parent),
            ptr::addr_of_mut!(thread_lock),
        );
    }
    cond_broadcast(ptr::addr_of_mut!(thread_worker));
    mutex_unlock(ptr::addr_of_mut!(thread_lock));

    sleep(params.runtime);
    toggle_done(0, ptr::null_mut(), ptr::null_mut());

    i = 0;
    while i < params.nthreads {
        ret = pthread_join((*worker.add(i as usize)).thread, ptr::null_mut());
        if ret != 0 {
            err(EXIT_FAILURE, b"pthread_join\0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    /* cleanup & report results */
    cond_destroy(ptr::addr_of_mut!(thread_parent));
    cond_destroy(ptr::addr_of_mut!(thread_worker));
    mutex_destroy(ptr::addr_of_mut!(thread_lock));

    i = 0;
    while i < params.nthreads {
        let t: c_ulong = if bench__runtime.tv_sec > 0 {
            (*worker.add(i as usize)).ops / bench__runtime.tv_sec as c_ulong
        } else {
            0
        };

        update_stats(ptr::addr_of_mut!(throughput_stats), t);
        if !params.silent {
            printf(
                b"[thread %3d] futex: %p [ %ld ops/sec ]\n\0".as_ptr() as *const c_char,
                (*worker.add(i as usize)).tid,
                (*worker.add(i as usize)).futex,
                t,
            );
        }

        if params.multi {
            zfree(ptr::addr_of_mut!((*worker.add(i as usize)).futex));
        }
        i += 1;
    }

    print_summary();

    free(worker as *mut c_void);
    perf_cpu_map__put(cpu);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
