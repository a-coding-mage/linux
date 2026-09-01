// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013  Davidlohr Bueso <davidlohr@hp.com>
 *
 * futex-wake: Block a bunch of threads on a futex and wake'em up, N at a time.
 *
 * This program is particularly useful to measure the latency of nthread wakeups
 * in non-error situations:  all waiters are queued and all wake calls wakeup
 * one or more tasks, and thus the waitqueue is never empty.
 */

/* C dependencies: string.h, pthread.h, signal.h, ../util/mutex.h,
 * ../util/stat.h, subcmd/parse-options.h, linux/compiler.h, linux/kernel.h,
 * linux/time64.h, errno.h, perf/cpumap.h, bench.h, futex.h, err.h,
 * stdlib.h, sys/time.h, sys/mman.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type pthread_t = usize;
type pthread_attr_t = c_void;
type cpu_set_t = c_void;
type bool_ = bool;

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
pub struct timeval {
    pub tv_sec: isize,
    pub tv_usec: isize,
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 0],
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
}

#[repr(C)]
pub struct bench_futex_parameters {
    pub nbuckets: c_int,
    pub nthreads: c_uint,
    pub nwakes: c_uint,
    pub silent: bool_,
    pub fshared: bool_,
    pub mlockall: bool_,
}

const EINTR: c_int = 4;
const EXIT_FAILURE: c_int = 1;
const FUTEX_PRIVATE_FLAG: c_int = 128;
const MCL_CURRENT: c_int = 1;
const MCL_FUTURE: c_int = 2;
const SIGINT: c_int = 2;
const USEC_PER_MSEC: c_int = 1000;

unsafe extern "C" {
    static mut bench_repeat: c_uint;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn cond_signal(cond: *mut cond);
    fn cond_wait(cond: *mut cond, lock: *mut mutex);
    fn cond_broadcast(cond: *mut cond);
    fn cond_init(cond: *mut cond);
    fn cond_destroy(cond: *mut cond);

    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn rel_stddev_stats(stddev: f64, avg: f64) -> f64;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: c_uint);

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);

    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn cpu__max_cpu() -> perf_cpu;

    fn futex_wait(uaddr: *mut u32, val: u32, timeout: *mut c_void, flags: c_int) -> c_int;
    fn futex_wake(uaddr: *mut u32, nr_wake: c_uint, flags: c_int) -> c_uint;
    fn futex_print_nbuckets(params: *mut bench_futex_parameters);

    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(attr: *mut pthread_attr_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpuset: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, cpuset: *mut cpu_set_t);
    fn CPU_FREE(cpuset: *mut cpu_set_t);

    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn mlockall(flags: c_int) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
}

/* all threads will block on the same futex */
static mut futex1: u32 = 0;

static mut worker: *mut pthread_t = core::ptr::null_mut();
static mut done: bool = false;
static mut thread_lock: mutex = mutex { _private: [] };
static mut thread_parent: cond = cond { _private: [] };
static mut thread_worker: cond = cond { _private: [] };
static mut waketime_stats: stats = stats { _private: [] };
static mut wakeup_stats: stats = stats { _private: [] };
static mut threads_starting: c_uint = 0;
static mut futex_flag: c_int = 0;

static mut params: bench_futex_parameters = bench_futex_parameters {
    nbuckets: -1,
    /*
     * How many wakeups to do at a time.
     * Default to 1 in order to make the kernel work more.
     */
    nwakes: 1,
    nthreads: 0,
    silent: false,
    fshared: false,
    mlockall: false,
};

/*
static const struct option options[] = {
    OPT_INTEGER( 'b', "buckets", &params.nbuckets, "Specify amount of hash buckets"),
    OPT_UINTEGER('t', "threads", &params.nthreads, "Specify amount of threads"),
    OPT_UINTEGER('w', "nwakes",  &params.nwakes, "Specify amount of threads to wake at once"),
    OPT_BOOLEAN( 's', "silent",  &params.silent, "Silent mode: do not display data/details"),
    OPT_BOOLEAN( 'S', "shared",  &params.fshared, "Use shared futexes instead of private ones"),
    OPT_BOOLEAN( 'm', "mlockall", &params.mlockall, "Lock all current and future memory"),

    OPT_END()
};
*/
static options: [option; 0] = [];

static bench_futex_wake_usage_0: &[u8] = b"perf bench futex wake <options>\0";
static bench_futex_wake_usage: [*const c_char; 2] = [
    bench_futex_wake_usage_0.as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe extern "C" fn workerfn(_arg: *mut c_void) -> *mut c_void {
    mutex_lock(core::ptr::addr_of_mut!(thread_lock));
    threads_starting = threads_starting.wrapping_sub(1);
    if threads_starting == 0 {
        cond_signal(core::ptr::addr_of_mut!(thread_parent));
    }
    cond_wait(
        core::ptr::addr_of_mut!(thread_worker),
        core::ptr::addr_of_mut!(thread_lock),
    );
    mutex_unlock(core::ptr::addr_of_mut!(thread_lock));

    loop {
        if futex_wait(
            core::ptr::addr_of_mut!(futex1),
            0,
            core::ptr::null_mut(),
            futex_flag,
        ) != EINTR
        {
            break;
        }
    }

    pthread_exit(core::ptr::null_mut());
}

unsafe fn print_summary() {
    let waketime_avg: f64 = avg_stats(core::ptr::addr_of_mut!(waketime_stats));
    let waketime_stddev: f64 = stddev_stats(core::ptr::addr_of_mut!(waketime_stats));
    let wakeup_avg: c_uint = avg_stats(core::ptr::addr_of_mut!(wakeup_stats)) as c_uint;

    printf(
        b"Wokeup %d of %d threads in %.4f ms (+-%.2f%%)\n\0".as_ptr() as *const c_char,
        wakeup_avg,
        params.nthreads,
        waketime_avg / USEC_PER_MSEC as f64,
        rel_stddev_stats(waketime_stddev, waketime_avg),
    );
    futex_print_nbuckets(core::ptr::addr_of_mut!(params));
}

unsafe fn block_threads(w: *mut pthread_t, cpu: *mut perf_cpu_map) {
    let mut cpuset: *mut cpu_set_t;
    let mut i: c_uint;
    let size: size_t;
    let nrcpus: c_int = cpu__max_cpu().cpu;
    threads_starting = params.nthreads;

    cpuset = CPU_ALLOC(nrcpus);
    if cpuset.is_null() {
        core::intrinsics::abort();
    }
    size = CPU_ALLOC_SIZE(nrcpus);

    /* create and block all threads */
    i = 0;
    while i < params.nthreads {
        let mut thread_attr: pthread_attr_t = core::mem::zeroed();

        pthread_attr_init(&mut thread_attr);
        CPU_ZERO_S(size, cpuset);
        CPU_SET_S(
            perf_cpu_map__cpu(cpu, (i % perf_cpu_map__nr(cpu)) as c_int).cpu,
            size,
            cpuset,
        );

        if pthread_attr_setaffinity_np(&mut thread_attr, size, cpuset) != 0 {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, b"pthread_attr_setaffinity_np\0".as_ptr() as *const c_char);
        }

        if pthread_create(
            w.add(i as usize),
            &thread_attr,
            workerfn,
            core::ptr::null_mut(),
        ) != 0
        {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, b"pthread_create\0".as_ptr() as *const c_char);
        }
        pthread_attr_destroy(&mut thread_attr);
        i = i.wrapping_add(1);
    }
    CPU_FREE(cpuset);
}

unsafe extern "C" fn toggle_done(
    _sig: c_int,
    _info: *mut siginfo_t,
    _uc: *mut c_void,
) {
    done = true;
}

#[no_mangle]
pub unsafe extern "C" fn bench_futex_wake(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut act: sigaction = core::mem::zeroed();
    let mut cpu: *mut perf_cpu_map;

    argc = parse_options(
        argc,
        argv,
        options.as_ptr(),
        bench_futex_wake_usage.as_ptr(),
        0,
    );
    if argc != 0 {
        usage_with_options(bench_futex_wake_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    cpu = perf_cpu_map__new_online_cpus();
    if cpu.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
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

    if params.nthreads == 0 {
        params.nthreads = perf_cpu_map__nr(cpu);
    }

    worker = calloc(
        params.nthreads as size_t,
        core::mem::size_of::<pthread_t>(),
    ) as *mut pthread_t;
    if worker.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
    }

    if !params.fshared {
        futex_flag = FUTEX_PRIVATE_FLAG;
    }

    printf(
        b"Run summary [PID %d]: blocking on %d threads (at [%s] futex %p), waking up %d at a time.\n\n\0"
            .as_ptr() as *const c_char,
        getpid(),
        params.nthreads,
        if params.fshared {
            b"shared\0".as_ptr()
        } else {
            b"private\0".as_ptr()
        } as *const c_char,
        core::ptr::addr_of_mut!(futex1),
        params.nwakes,
    );

    init_stats(core::ptr::addr_of_mut!(wakeup_stats));
    init_stats(core::ptr::addr_of_mut!(waketime_stats));
    mutex_init(core::ptr::addr_of_mut!(thread_lock));
    cond_init(core::ptr::addr_of_mut!(thread_parent));
    cond_init(core::ptr::addr_of_mut!(thread_worker));

    j = 0;
    while j < bench_repeat && !done {
        let mut nwoken: c_uint = 0;
        let mut start: timeval = core::mem::zeroed();
        let mut end: timeval = core::mem::zeroed();
        let mut runtime: timeval = core::mem::zeroed();

        /* create, launch & block all threads */
        block_threads(worker, cpu);

        /* make sure all threads are already blocked */
        mutex_lock(core::ptr::addr_of_mut!(thread_lock));
        while threads_starting != 0 {
            cond_wait(
                core::ptr::addr_of_mut!(thread_parent),
                core::ptr::addr_of_mut!(thread_lock),
            );
        }
        cond_broadcast(core::ptr::addr_of_mut!(thread_worker));
        mutex_unlock(core::ptr::addr_of_mut!(thread_lock));

        usleep(100000);

        /* Ok, all threads are patiently blocked, start waking folks up */
        gettimeofday(&mut start, core::ptr::null_mut());
        while nwoken != params.nthreads {
            nwoken = nwoken.wrapping_add(futex_wake(
                core::ptr::addr_of_mut!(futex1),
                params.nwakes,
                futex_flag,
            ));
        }
        gettimeofday(&mut end, core::ptr::null_mut());
        runtime.tv_sec = end.tv_sec - start.tv_sec;
        runtime.tv_usec = end.tv_usec - start.tv_usec;
        if runtime.tv_usec < 0 {
            runtime.tv_sec -= 1;
            runtime.tv_usec += 1000000;
        }

        update_stats(core::ptr::addr_of_mut!(wakeup_stats), nwoken);
        update_stats(core::ptr::addr_of_mut!(waketime_stats), runtime.tv_usec as c_uint);

        if !params.silent {
            printf(
                b"[Run %d]: Wokeup %d of %d threads in %.4f ms\n\0".as_ptr() as *const c_char,
                j + 1,
                nwoken,
                params.nthreads,
                runtime.tv_usec as f64 / USEC_PER_MSEC as f64,
            );
        }

        i = 0;
        while i < params.nthreads {
            ret = pthread_join(*worker.add(i as usize), core::ptr::null_mut());
            if ret != 0 {
                err(EXIT_FAILURE, b"pthread_join\0".as_ptr() as *const c_char);
            }
            i = i.wrapping_add(1);
        }

        j = j.wrapping_add(1);
    }

    /* cleanup & report results */
    cond_destroy(core::ptr::addr_of_mut!(thread_parent));
    cond_destroy(core::ptr::addr_of_mut!(thread_worker));
    mutex_destroy(core::ptr::addr_of_mut!(thread_lock));

    print_summary();

    free(worker as *mut c_void);
    perf_cpu_map__put(cpu);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
