// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013  Davidlohr Bueso <davidlohr@hp.com>
 *
 * futex-hash: Stress the hell out of the Linux kernel futex uaddr hashing.
 *
 * This program is particularly useful for measuring the kernel's futex hash
 * table/function implementation. In order for it to make sense, use with as
 * many threads and futexes as possible.
 */

/* For the CLR_() macros */
/* C dependencies:
 * string.h, pthread.h, errno.h, signal.h, stdlib.h, linux/compiler.h,
 * linux/kernel.h, linux/zalloc.h, sys/time.h, sys/mman.h, sys/prctl.h,
 * perf/cpumap.h, ../util/mutex.h, ../util/stat.h, subcmd/parse-options.h,
 * bench.h, futex.h, err.h
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type pthread_t = c_ulong;
type u_int32_t = u32;

#[repr(C)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct sigset_t {
    _private: [u64; 16],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct pthread_attr_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cond {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bench_futex_parameters {
    pub nfutexes: c_uint,
    pub runtime: c_uint,
    pub nbuckets: c_int,
    pub nthreads: c_uint,
    pub silent: bool,
    pub fshared: bool,
    pub mlockall: bool,
}

#[repr(C)]
pub struct worker {
    pub tid: c_int,
    pub futex: *mut u_int32_t,
    pub thread: pthread_t,
    pub ops: c_ulong,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn warn(fmt: *const c_char, ...);
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn exit(status: c_int) -> !;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mlockall(flags: c_int) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(
        attr: *mut pthread_attr_t,
        cpusetsize: size_t,
        cpuset: *const cpu_set_t,
    ) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn cond_init(cond: *mut cond);
    fn cond_signal(cond: *mut cond);
    fn cond_wait(cond: *mut cond, lock: *mut mutex);
    fn cond_broadcast(cond: *mut cond);
    fn cond_destroy(cond: *mut cond);

    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: c_ulong);
    fn avg_stats(stats: *mut stats) -> c_ulong;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn rel_stddev_stats(stddev: f64, avg: c_ulong) -> f64;

    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);

    fn futex_wait(
        uaddr: *mut u_int32_t,
        val: u_int32_t,
        timeout: *mut c_void,
        futex_flag: c_int,
    ) -> c_int;
    fn futex_set_nbuckets_param(params: *mut bench_futex_parameters);
    fn futex_print_nbuckets(params: *mut bench_futex_parameters);

    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpu: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpu: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn cpu__max_cpu() -> perf_cpu;

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_FREE(cpuset: *mut cpu_set_t);
}

unsafe extern "C" {
    static options: [option; 0];
}

const EAGAIN: c_int = 11;
const EWOULDBLOCK: c_int = EAGAIN;
const EXIT_FAILURE: c_int = 1;
const FUTEX_PRIVATE_FLAG: c_int = 128;
const MCL_CURRENT: c_int = 1;
const MCL_FUTURE: c_int = 2;
const SIGINT: c_int = 2;

static mut done: bool = false;
static mut futex_flag: c_int = 0;

#[unsafe(no_mangle)]
pub static mut bench__start: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
#[unsafe(no_mangle)]
pub static mut bench__end: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
#[unsafe(no_mangle)]
pub static mut bench__runtime: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
static mut thread_lock: mutex = mutex { _private: [] };
static mut threads_starting: c_uint = 0;
static mut throughput_stats: stats = stats { _private: [] };
static mut thread_parent: cond = cond { _private: [] };
static mut thread_worker: cond = cond { _private: [] };

static mut params: bench_futex_parameters = bench_futex_parameters {
    nfutexes: 1024,
    runtime: 10,
    nbuckets: -1,
    nthreads: 0,
    silent: false,
    fshared: false,
    mlockall: false,
};

/* Original C used OPT_INTEGER/OPT_UINTEGER/OPT_BOOLEAN/OPT_END initializers here. */
static bench_futex_hash_usage_0: &[u8] = b"perf bench futex hash <options>\0";
static bench_futex_hash_usage_1: *const c_char = core::ptr::null();
static bench_futex_hash_usage: [*const c_char; 2] = [
    bench_futex_hash_usage_0.as_ptr() as *const c_char,
    bench_futex_hash_usage_1,
];

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1000000;
    }
}

unsafe fn BUG_ON(cond: bool) {
    if cond {
        panic!("BUG_ON");
    }
}

unsafe fn zfree<T>(ptr: *mut *mut T) {
    if !(*ptr).is_null() {
        free(*ptr as *mut c_void);
        *ptr = core::ptr::null_mut();
    }
}

unsafe extern "C" fn workerfn(arg: *mut c_void) -> *mut c_void {
    let mut ret: c_int;
    let w: *mut worker = arg as *mut worker;
    let mut i: c_uint;
    let mut ops: c_ulong = (*w).ops; /* avoid cacheline bouncing */

    mutex_lock(&raw mut thread_lock);
    threads_starting = threads_starting.wrapping_sub(1);
    if threads_starting == 0 {
        cond_signal(&raw mut thread_parent);
    }
    cond_wait(&raw mut thread_worker, &raw mut thread_lock);
    mutex_unlock(&raw mut thread_lock);

    loop {
        i = 0;
        while i < params.nfutexes {
            /*
             * We want the futex calls to fail in order to stress
             * the hashing of uaddr and not measure other steps,
             * such as internal waitqueue handling, thus enlarging
             * the critical region protected by hb->lock.
             */
            ret = futex_wait((*w).futex.add(i as usize), 1234, core::ptr::null_mut(), futex_flag);
            if !params.silent && (ret == 0 || errno != EAGAIN || errno != EWOULDBLOCK) {
                warn(c"Non-expected futex return call".as_ptr());
            }
            i = i.wrapping_add(1);
            ops = ops.wrapping_add(1);
        }
        if done {
            break;
        }
    }

    (*w).ops = ops;
    core::ptr::null_mut()
}

unsafe extern "C" fn toggle_done(
    _sig: c_int,
    _info: *mut siginfo_t,
    _uc: *mut c_void,
) {
    /* inform all threads that we're done for the day */
    done = true;
    gettimeofday(&raw mut bench__end, core::ptr::null_mut());
    timersub(&raw const bench__end, &raw const bench__start, &raw mut bench__runtime);
}

unsafe fn print_summary() {
    let avg: c_ulong = avg_stats(&raw mut throughput_stats);
    let stddev: f64 = stddev_stats(&raw mut throughput_stats);

    printf(
        c"%sAveraged %ld operations/sec (+- %.2f%%), total secs = %d\n".as_ptr(),
        if !params.silent {
            c"\n".as_ptr()
        } else {
            c"".as_ptr()
        },
        avg,
        rel_stddev_stats(stddev, avg),
        bench__runtime.tv_sec as c_int,
    );
    futex_print_nbuckets(&raw mut params);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_futex_hash(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut ret: c_int = 0;
    let cpuset: *mut cpu_set_t;
    let mut act: sigaction = core::mem::zeroed();
    let mut i: c_uint;
    let mut thread_attr: pthread_attr_t = core::mem::zeroed();
    let mut worker: *mut worker = core::ptr::null_mut();
    let cpu: *mut perf_cpu_map;
    let nrcpus: c_int;
    let size: size_t;

    argc = parse_options(
        argc,
        argv,
        options.as_ptr(),
        bench_futex_hash_usage.as_ptr(),
        0,
    );
    if argc != 0 {
        usage_with_options(bench_futex_hash_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    cpu = perf_cpu_map__new_online_cpus();
    if cpu.is_null() {
        errmem();
    }

    memset(
        &mut act as *mut sigaction as *mut c_void,
        0,
        core::mem::size_of_val(&act),
    );
    sigfillset(&mut act.sa_mask);
    act.sa_sigaction = Some(toggle_done);
    sigaction(SIGINT, &act, core::ptr::null_mut());

    if params.mlockall {
        if mlockall(MCL_CURRENT | MCL_FUTURE) != 0 {
            err(EXIT_FAILURE, c"mlockall".as_ptr());
        }
    }

    if params.nthreads == 0 {
        /* default to the number of CPUs */
        params.nthreads = perf_cpu_map__nr(cpu) as c_uint;
    }

    worker = calloc(
        params.nthreads as size_t,
        core::mem::size_of::<worker>(),
    ) as *mut worker;
    if worker.is_null() {
        errmem();
    }

    if !params.fshared {
        futex_flag = FUTEX_PRIVATE_FLAG;
    }
    futex_set_nbuckets_param(&raw mut params);

    printf(
        c"Run summary [PID %d]: %d threads, each operating on %d [%s] futexes for %d secs.\n\n"
            .as_ptr(),
        getpid(),
        params.nthreads,
        params.nfutexes,
        if params.fshared {
            c"shared".as_ptr()
        } else {
            c"private".as_ptr()
        },
        params.runtime,
    );

    init_stats(&raw mut throughput_stats);
    mutex_init(&raw mut thread_lock);
    cond_init(&raw mut thread_parent);
    cond_init(&raw mut thread_worker);

    threads_starting = params.nthreads;
    pthread_attr_init(&mut thread_attr);
    gettimeofday(&raw mut bench__start, core::ptr::null_mut());

    nrcpus = cpu__max_cpu().cpu;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(cpuset.is_null());
    size = CPU_ALLOC_SIZE(nrcpus);

    i = 0;
    while i < params.nthreads {
        (*worker.add(i as usize)).tid = i as c_int;
        (*worker.add(i as usize)).futex = calloc(
            params.nfutexes as size_t,
            core::mem::size_of::<u_int32_t>(),
        ) as *mut u_int32_t;
        if (*worker.add(i as usize)).futex.is_null() {
            errmem();
        }

        CPU_ZERO_S(size, cpuset);

        CPU_SET_S(
            perf_cpu_map__cpu(cpu, (i as c_int) % perf_cpu_map__nr(cpu)).cpu,
            size,
            cpuset,
        );
        ret = pthread_attr_setaffinity_np(&mut thread_attr, size, cpuset);
        if ret != 0 {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, c"pthread_attr_setaffinity_np".as_ptr());
        }
        ret = pthread_create(
            &mut (*worker.add(i as usize)).thread,
            &thread_attr,
            Some(workerfn),
            worker.add(i as usize) as *mut c_void,
        );
        if ret != 0 {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, c"pthread_create".as_ptr());
        }

        i = i.wrapping_add(1);
    }
    CPU_FREE(cpuset);
    pthread_attr_destroy(&mut thread_attr);

    mutex_lock(&raw mut thread_lock);
    while threads_starting != 0 {
        cond_wait(&raw mut thread_parent, &raw mut thread_lock);
    }
    cond_broadcast(&raw mut thread_worker);
    mutex_unlock(&raw mut thread_lock);

    sleep(params.runtime);
    toggle_done(0, core::ptr::null_mut(), core::ptr::null_mut());

    i = 0;
    while i < params.nthreads {
        ret = pthread_join((*worker.add(i as usize)).thread, core::ptr::null_mut());
        if ret != 0 {
            err(EXIT_FAILURE, c"pthread_join".as_ptr());
        }
        i = i.wrapping_add(1);
    }

    /* cleanup & report results */
    cond_destroy(&raw mut thread_parent);
    cond_destroy(&raw mut thread_worker);
    mutex_destroy(&raw mut thread_lock);

    i = 0;
    while i < params.nthreads {
        let t: c_ulong = if bench__runtime.tv_sec > 0 {
            (*worker.add(i as usize)).ops / bench__runtime.tv_sec as c_ulong
        } else {
            0
        };
        update_stats(&raw mut throughput_stats, t);
        if !params.silent {
            if params.nfutexes == 1 {
                printf(
                    c"[thread %2d] futex: %p [ %ld ops/sec ]\n".as_ptr(),
                    (*worker.add(i as usize)).tid,
                    (*worker.add(i as usize)).futex.add(0),
                    t,
                );
            } else {
                printf(
                    c"[thread %2d] futexes: %p ... %p [ %ld ops/sec ]\n".as_ptr(),
                    (*worker.add(i as usize)).tid,
                    (*worker.add(i as usize)).futex.add(0),
                    (*worker.add(i as usize)).futex.add(params.nfutexes as usize - 1),
                    t,
                );
            }
        }

        zfree(&mut (*worker.add(i as usize)).futex);
        i = i.wrapping_add(1);
    }

    print_summary();

    free(worker as *mut c_void);
    free(cpu as *mut c_void);
    ret
}

unsafe fn errmem() -> ! {
    err(EXIT_FAILURE, c"calloc".as_ptr());
}
