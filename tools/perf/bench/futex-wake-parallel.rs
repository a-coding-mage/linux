// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Davidlohr Bueso.
 *
 * Block a bunch of threads and let parallel waker threads wakeup an
 * equal amount of them. The program output reflects the avg latency
 * for each individual thread to service its share of work. Ultimately
 * it can be used to measure futex_wake() changes.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type pthread_t = usize;
type pthread_attr_t = c_void;
type pthread_barrier_t = c_void;
type size_t = usize;
type u_int32_t = u32;
type cpu_set_t = c_void;
type bool_ = bool;

const EXIT_FAILURE: c_int = 1;
const EINTR: c_int = 4;
const SIGINT: c_int = 2;
const MCL_CURRENT: c_int = 1;
const MCL_FUTURE: c_int = 2;
const PTHREAD_CREATE_JOINABLE: c_int = 0;
const FUTEX_PRIVATE_FLAG: c_int = 128;
const USEC_PER_MSEC: f64 = 1000.0;

#[repr(C)]
struct timeval {
    tv_sec: isize,
    tv_usec: isize,
}

#[repr(C)]
struct sigset_t {
    __val: [usize; 16],
}

#[repr(C)]
struct sigaction {
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
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
struct option {
    _private: [u8; 0],
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
struct bench_futex_parameters {
    nbuckets: c_int,
    nthreads: c_uint,
    nwakes: c_uint,
    silent: bool_,
    fshared: bool_,
    mlockall: bool_,
}

#[repr(C)]
struct thread_data {
    worker: pthread_t,
    nwoken: c_uint,
    runtime: timeval,
}

unsafe extern "C" {
    static mut bench_repeat: c_uint;

    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn warnx(fmt: *const c_char, ...);
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn exit(status: c_int) -> !;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn mlockall(flags: c_int) -> c_int;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: c_int) -> c_int;
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
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_FREE(cpuset: *mut cpu_set_t);
    fn BUG_ON(cond: bool);

    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn cond_init(cond: *mut cond);
    fn cond_destroy(cond: *mut cond);
    fn cond_wait(cond: *mut cond, lock: *mut mutex);
    fn cond_signal(cond: *mut cond);
    fn cond_broadcast(cond: *mut cond);

    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn rel_stddev_stats(stddev: f64, avg: f64) -> f64;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);

    fn futex_wait(uaddr: *mut u_int32_t, val: u_int32_t, timeout: *mut c_void, opflags: c_int) -> c_int;
    fn futex_wake(uaddr: *mut u_int32_t, nr_wake: c_uint, opflags: c_int) -> c_uint;
    fn futex_set_nbuckets_param(params: *mut bench_futex_parameters);
    fn futex_print_nbuckets(params: *mut bench_futex_parameters);

    fn cpu__max_cpu() -> perf_cpu;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_uint) -> perf_cpu;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
}

static mut nwakes: c_uint = 1;

/* all threads will block on the same futex -- hash bucket chaos ;) */
static mut futex: u_int32_t = 0;

static mut blocked_worker: *mut pthread_t = core::ptr::null_mut();
static mut done: bool = false;
static mut thread_lock: mutex = mutex { _private: [] };
static mut thread_parent: cond = cond { _private: [] };
static mut thread_worker: cond = cond { _private: [] };
static mut barrier: pthread_barrier_t = unsafe { core::mem::zeroed() };
static mut waketime_stats: stats = stats { _private: [] };
static mut wakeup_stats: stats = stats { _private: [] };
static mut threads_starting: c_uint = 0;
static mut futex_flag: c_int = 0;

static mut params: bench_futex_parameters = bench_futex_parameters {
    nbuckets: -1,
    nthreads: 0,
    nwakes: 0,
    silent: false,
    fshared: false,
    mlockall: false,
};

/* Original C used OPT_INTEGER/OPT_UINTEGER/OPT_BOOLEAN/OPT_END macros here. */
static options: [option; 1] = [option { _private: [] }];

static bench_futex_wake_parallel_usage_0: &[u8] = b"perf bench futex wake-parallel <options>\0";
static bench_futex_wake_parallel_usage: [*const c_char; 2] = [
    bench_futex_wake_parallel_usage_0.as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1000000;
    }
}

unsafe extern "C" fn waking_workerfn(arg: *mut c_void) -> *mut c_void {
    let waker: *mut thread_data = arg as *mut thread_data;
    let mut start: timeval = core::mem::zeroed();
    let mut end: timeval = core::mem::zeroed();

    pthread_barrier_wait(&raw mut barrier);

    gettimeofday(&mut start, core::ptr::null_mut());

    (*waker).nwoken = futex_wake(&raw mut futex, nwakes, futex_flag);
    if (*waker).nwoken != nwakes {
        warnx(
            b"couldn't wakeup all tasks (%d/%d)\0".as_ptr() as *const c_char,
            (*waker).nwoken,
            nwakes,
        );
    }

    gettimeofday(&mut end, core::ptr::null_mut());
    timersub(&end, &start, &mut (*waker).runtime);

    pthread_exit(core::ptr::null_mut());
}

unsafe fn wakeup_threads(td: *mut thread_data) {
    let mut i: c_uint;
    let mut thread_attr: pthread_attr_t = core::mem::zeroed();

    pthread_attr_init(&mut thread_attr);
    pthread_attr_setdetachstate(&mut thread_attr, PTHREAD_CREATE_JOINABLE);

    pthread_barrier_init(&raw mut barrier, core::ptr::null(), params.nwakes + 1);

    /* create and block all threads */
    i = 0;
    while i < params.nwakes {
        /*
         * Thread creation order will impact per-thread latency
         * as it will affect the order to acquire the hb spinlock.
         * For now let the scheduler decide.
         */
        if pthread_create(
            &mut (*td.add(i as usize)).worker,
            &thread_attr,
            waking_workerfn,
            td.add(i as usize) as *mut c_void,
        ) != 0
        {
            err(EXIT_FAILURE, b"pthread_create\0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    pthread_barrier_wait(&raw mut barrier);

    i = 0;
    while i < params.nwakes {
        if pthread_join((*td.add(i as usize)).worker, core::ptr::null_mut()) != 0 {
            err(EXIT_FAILURE, b"pthread_join\0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    pthread_barrier_destroy(&raw mut barrier);
    pthread_attr_destroy(&mut thread_attr);
}

unsafe extern "C" fn blocked_workerfn(_arg: *mut c_void) -> *mut c_void {
    mutex_lock(&raw mut thread_lock);
    threads_starting -= 1;
    if threads_starting == 0 {
        cond_signal(&raw mut thread_parent);
    }
    cond_wait(&raw mut thread_worker, &raw mut thread_lock);
    mutex_unlock(&raw mut thread_lock);

    while 1 != 0 {
        /* handle spurious wakeups */
        if futex_wait(&raw mut futex, 0, core::ptr::null_mut(), futex_flag) != EINTR {
            break;
        }
    }

    pthread_exit(core::ptr::null_mut());
}

unsafe fn block_threads(w: *mut pthread_t, cpu: *mut perf_cpu_map) {
    let mut cpuset: *mut cpu_set_t;
    let mut i: c_uint;
    let nrcpus: c_int = cpu__max_cpu().cpu;
    let size: size_t;

    threads_starting = params.nthreads;

    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(cpuset.is_null());
    size = CPU_ALLOC_SIZE(nrcpus);

    /* create and block all threads */
    i = 0;
    while i < params.nthreads {
        let mut thread_attr: pthread_attr_t = core::mem::zeroed();

        pthread_attr_init(&mut thread_attr);
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
            w.add(i as usize),
            &thread_attr,
            blocked_workerfn,
            core::ptr::null_mut(),
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

unsafe fn print_run(waking_worker: *mut thread_data, run_num: c_uint) {
    let mut i: c_uint;
    let wakeup_avg: c_uint;
    let waketime_avg: f64;
    let waketime_stddev: f64;
    let mut __waketime_stats: stats = stats { _private: [] };
    let mut __wakeup_stats: stats = stats { _private: [] };

    init_stats(&mut __wakeup_stats);
    init_stats(&mut __waketime_stats);

    i = 0;
    while i < params.nwakes {
        update_stats(
            &mut __waketime_stats,
            (*waking_worker.add(i as usize)).runtime.tv_usec as u64,
        );
        update_stats(
            &mut __wakeup_stats,
            (*waking_worker.add(i as usize)).nwoken as u64,
        );
        i += 1;
    }

    waketime_avg = avg_stats(&mut __waketime_stats);
    waketime_stddev = stddev_stats(&mut __waketime_stats);
    wakeup_avg = avg_stats(&mut __wakeup_stats) as c_uint;

    printf(
        b"[Run %d]: Avg per-thread latency (waking %d/%d threads) in %.4f ms (+-%.2f%%)\n\0"
            .as_ptr() as *const c_char,
        run_num + 1,
        wakeup_avg,
        params.nthreads,
        waketime_avg / USEC_PER_MSEC,
        rel_stddev_stats(waketime_stddev, waketime_avg),
    );
}

unsafe fn print_summary() {
    let wakeup_avg: c_uint;
    let waketime_avg: f64;
    let waketime_stddev: f64;

    waketime_avg = avg_stats(&raw mut waketime_stats);
    waketime_stddev = stddev_stats(&raw mut waketime_stats);
    wakeup_avg = avg_stats(&raw mut wakeup_stats) as c_uint;

    printf(
        b"Avg per-thread latency (waking %d/%d threads) in %.4f ms (+-%.2f%%)\n\0".as_ptr()
            as *const c_char,
        wakeup_avg,
        params.nthreads,
        waketime_avg / USEC_PER_MSEC,
        rel_stddev_stats(waketime_stddev, waketime_avg),
    );
    futex_print_nbuckets(&raw mut params);
}

unsafe fn do_run_stats(waking_worker: *mut thread_data) {
    let mut i: c_uint;

    i = 0;
    while i < params.nwakes {
        update_stats(
            &raw mut waketime_stats,
            (*waking_worker.add(i as usize)).runtime.tv_usec as u64,
        );
        update_stats(
            &raw mut wakeup_stats,
            (*waking_worker.add(i as usize)).nwoken as u64,
        );
        i += 1;
    }
}

unsafe extern "C" fn toggle_done(
    _sig: c_int,
    _info: *mut siginfo_t,
    _uc: *mut c_void,
) {
    done = true;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_futex_wake_parallel(
    mut argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut act: sigaction = core::mem::zeroed();
    let mut waking_worker: *mut thread_data;
    let cpu: *mut perf_cpu_map;

    argc = parse_options(
        argc,
        argv,
        options.as_ptr(),
        bench_futex_wake_parallel_usage.as_ptr(),
        0,
    );
    if argc != 0 {
        usage_with_options(
            bench_futex_wake_parallel_usage.as_ptr(),
            options.as_ptr(),
        );
        exit(EXIT_FAILURE);
    }

    memset(
        &mut act as *mut sigaction as *mut c_void,
        0,
        core::mem::size_of::<sigaction>(),
    );
    sigfillset(&mut act.sa_mask);
    act.sa_sigaction = Some(toggle_done);
    sigaction(SIGINT, &act, core::ptr::null_mut());

    if params.mlockall {
        if mlockall(MCL_CURRENT | MCL_FUTURE) != 0 {
            err(EXIT_FAILURE, b"mlockall\0".as_ptr() as *const c_char);
        }
    }

    cpu = perf_cpu_map__new_online_cpus();
    if cpu.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
    }

    if params.nthreads == 0 {
        params.nthreads = perf_cpu_map__nr(cpu);
    }

    /* some sanity checks */
    if params.nwakes > params.nthreads || params.nwakes == 0 {
        params.nwakes = params.nthreads;
    }

    if params.nthreads % params.nwakes != 0 {
        errx(
            EXIT_FAILURE,
            b"Must be perfectly divisible\0".as_ptr() as *const c_char,
        );
    }
    /*
     * Each thread will wakeup nwakes tasks in
     * a single futex_wait call.
     */
    nwakes = params.nthreads / params.nwakes;

    blocked_worker = calloc(
        params.nthreads as size_t,
        core::mem::size_of::<pthread_t>(),
    ) as *mut pthread_t;
    if blocked_worker.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
    }

    if !params.fshared {
        futex_flag = FUTEX_PRIVATE_FLAG;
    }

    futex_set_nbuckets_param(&raw mut params);

    printf(
        b"Run summary [PID %d]: blocking on %d threads (at [%s] futex %p), %d threads waking up %d at a time.\n\n\0"
            .as_ptr() as *const c_char,
        getpid(),
        params.nthreads,
        if params.fshared {
            b"shared\0".as_ptr() as *const c_char
        } else {
            b"private\0".as_ptr() as *const c_char
        },
        &raw mut futex,
        params.nwakes,
        nwakes,
    );

    init_stats(&raw mut wakeup_stats);
    init_stats(&raw mut waketime_stats);

    mutex_init(&raw mut thread_lock);
    cond_init(&raw mut thread_parent);
    cond_init(&raw mut thread_worker);

    j = 0;
    while j < bench_repeat && !done {
        waking_worker = calloc(
            params.nwakes as size_t,
            core::mem::size_of::<thread_data>(),
        ) as *mut thread_data;
        if waking_worker.is_null() {
            err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
        }

        /* create, launch & block all threads */
        block_threads(blocked_worker, cpu);

        /* make sure all threads are already blocked */
        mutex_lock(&raw mut thread_lock);
        while threads_starting != 0 {
            cond_wait(&raw mut thread_parent, &raw mut thread_lock);
        }
        cond_broadcast(&raw mut thread_worker);
        mutex_unlock(&raw mut thread_lock);

        usleep(200000);

        /* Ok, all threads are patiently blocked, start waking folks up */
        wakeup_threads(waking_worker);

        i = 0;
        while i < params.nthreads {
            ret = pthread_join(*blocked_worker.add(i as usize), core::ptr::null_mut());
            if ret != 0 {
                err(EXIT_FAILURE, b"pthread_join\0".as_ptr() as *const c_char);
            }
            i += 1;
        }

        do_run_stats(waking_worker);
        if !params.silent {
            print_run(waking_worker, j);
        }

        free(waking_worker as *mut c_void);
        j += 1;
    }

    /* cleanup & report results */
    cond_destroy(&raw mut thread_parent);
    cond_destroy(&raw mut thread_worker);
    mutex_destroy(&raw mut thread_lock);

    print_summary();

    free(blocked_worker as *mut c_void);
    perf_cpu_map__put(cpu);
    ret
}

/*
 * Original C fallback for !HAVE_PTHREAD_BARRIER:
 *
 * int bench_futex_wake_parallel(int argc __maybe_unused, const char **argv __maybe_unused)
 * {
 *      pr_err("%s: pthread_barrier_t unavailable, disabling this test...\n", __func__);
 *      return 0;
 * }
 */
