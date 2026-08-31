// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013  Davidlohr Bueso <davidlohr@hp.com>
 *
 * futex-requeue: Block a bunch of threads on futex1 and requeue them
 *                on futex2, N at a time.
 *
 * This program is particularly useful to measure the latency of nthread
 * requeues without waking up any tasks (in the non-pi case) -- thus
 * mimicking a regular futex_wait.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type u_int32_t = u32;
type pthread_t = usize;
type size_t = usize;

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
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bench_futex_parameters {
    pub nbuckets: c_int,
    pub nrequeue: c_uint,
    pub nthreads: c_uint,
    pub silent: bool,
    pub fshared: bool,
    pub mlockall: bool,
    pub broadcast: bool,
    pub pi: bool,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
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
pub struct timeval {
    pub tv_sec: isize,
    pub tv_usec: isize,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_attr_t {
    _private: [u8; 0],
}

const NULL: *mut c_void = ptr::null_mut();
const EXIT_FAILURE: c_int = 1;
const SIGINT: c_int = 2;
const EAGAIN: c_int = 11;
const MCL_CURRENT: c_int = 1;
const MCL_FUTURE: c_int = 2;
const USEC_PER_MSEC: c_int = 1000;
const FUTEX_PRIVATE_FLAG: c_int = 128;

unsafe extern "C" {
    static mut errno: c_int;
    static mut bench_repeat: c_uint;

    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn rel_stddev_stats(stddev: f64, avg: f64) -> f64;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: c_uint);

    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn cond_init(cond: *mut cond);
    fn cond_wait(cond: *mut cond, lock: *mut mutex);
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

    fn futex_wait(uaddr: *mut u_int32_t, val: u_int32_t, timeout: *mut c_void, flags: c_int) -> c_int;
    fn futex_wait_requeue_pi(
        uaddr: *mut u_int32_t,
        val: u_int32_t,
        uaddr2: *mut u_int32_t,
        timeout: *mut c_void,
        flags: c_int,
    ) -> c_int;
    fn futex_unlock_pi(uaddr: *mut u_int32_t, flags: c_int) -> c_int;
    fn futex_cmp_requeue(
        uaddr: *mut u_int32_t,
        val: u_int32_t,
        uaddr2: *mut u_int32_t,
        wake: c_uint,
        nr_requeue: c_uint,
        flags: c_int,
    ) -> c_int;
    fn futex_cmp_requeue_pi(
        uaddr: *mut u_int32_t,
        val: u_int32_t,
        uaddr2: *mut u_int32_t,
        nr_requeue: c_uint,
        flags: c_int,
    ) -> c_int;
    fn futex_wake(uaddr: *mut u_int32_t, nr_wake: c_uint, flags: c_int) -> c_uint;
    fn futex_print_nbuckets(params: *mut bench_futex_parameters);
    fn futex_set_nbuckets_param(params: *mut bench_futex_parameters);

    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpu: *mut perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(cpu: *mut perf_cpu_map, idx: c_uint) -> perf_cpu;
    fn perf_cpu_map__put(cpu: *mut perf_cpu_map);
    fn cpu__max_cpu() -> perf_cpu;

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, set: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, set: *mut cpu_set_t);
    fn CPU_FREE(set: *mut cpu_set_t);
    fn BUG_ON(cond: bool);

    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(attr: *mut pthread_attr_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn mlockall(flags: c_int) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn getpid() -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn warnx(fmt: *const c_char, ...);
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn exit(status: c_int) -> !;
}

static mut futex1: u_int32_t = 0;
static mut futex2: u_int32_t = 0;

static mut worker: *mut pthread_t = ptr::null_mut();
static mut done: bool = false;
static mut thread_lock: mutex = mutex { _private: [] };
static mut thread_parent: cond = cond { _private: [] };
static mut thread_worker: cond = cond { _private: [] };
static mut requeuetime_stats: stats = stats { _private: [] };
static mut requeued_stats: stats = stats { _private: [] };
static mut threads_starting: c_uint = 0;
static mut futex_flag: c_int = 0;

static mut params: bench_futex_parameters = bench_futex_parameters {
    nbuckets: -1,
    /*
     * How many tasks to requeue at a time.
     * Default to 1 in order to make the kernel work more.
     */
    nrequeue: 1,
    nthreads: 0,
    silent: false,
    fshared: false,
    mlockall: false,
    broadcast: false,
    pi: false,
};

/*
 * Original C option table:
 * OPT_INTEGER( 'b', "buckets", &params.nbuckets, "Specify amount of hash buckets")
 * OPT_UINTEGER('t', "threads",  &params.nthreads, "Specify amount of threads")
 * OPT_UINTEGER('q', "nrequeue", &params.nrequeue, "Specify amount of threads to requeue at once")
 * OPT_BOOLEAN( 's', "silent",   &params.silent, "Silent mode: do not display data/details")
 * OPT_BOOLEAN( 'S', "shared",   &params.fshared, "Use shared futexes instead of private ones")
 * OPT_BOOLEAN( 'm', "mlockall", &params.mlockall, "Lock all current and future memory")
 * OPT_BOOLEAN( 'B', "broadcast", &params.broadcast, "Requeue all threads at once")
 * OPT_BOOLEAN( 'p', "pi", &params.pi, "Use PI-aware variants of FUTEX_CMP_REQUEUE")
 * OPT_END()
 */
static options: [option; 0] = [];

static BENCH_FUTEX_REQUEUE_USAGE_0: &[u8] = b"perf bench futex requeue <options>\0";
static bench_futex_requeue_usage: [*const c_char; 2] = [
    BENCH_FUTEX_REQUEUE_USAGE_0.as_ptr() as *const c_char,
    ptr::null(),
];

unsafe fn timersub(a: *const timeval, b: *const timeval, res: *mut timeval) {
    (*res).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*res).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*res).tv_usec < 0 {
        (*res).tv_sec -= 1;
        (*res).tv_usec += 1000000;
    }
}

unsafe fn print_summary() {
    let requeuetime_avg: f64 = avg_stats(&raw mut requeuetime_stats);
    let requeuetime_stddev: f64 = stddev_stats(&raw mut requeuetime_stats);
    let requeued_avg: c_uint = avg_stats(&raw mut requeued_stats) as c_uint;

    printf(
        c"Requeued %d of %d threads in %.4f ms (+-%.2f%%)\n".as_ptr(),
        requeued_avg,
        params.nthreads,
        requeuetime_avg / USEC_PER_MSEC as f64,
        rel_stddev_stats(requeuetime_stddev, requeuetime_avg),
    );
    futex_print_nbuckets(&raw mut params);
}

unsafe extern "C" fn workerfn(_arg: *mut c_void) -> *mut c_void {
    let mut ret: c_int;

    mutex_lock(&raw mut thread_lock);
    threads_starting -= 1;
    if threads_starting == 0 {
        cond_signal(&raw mut thread_parent);
    }
    cond_wait(&raw mut thread_worker, &raw mut thread_lock);
    mutex_unlock(&raw mut thread_lock);

    loop {
        if !params.pi {
            ret = futex_wait(&raw mut futex1, 0, ptr::null_mut(), futex_flag);
            if ret == 0 {
                break;
            }

            if ret != 0 && errno != EAGAIN {
                if !params.silent {
                    warnx(c"futex_wait".as_ptr());
                }
                break;
            }
        } else {
            ret = futex_wait_requeue_pi(
                &raw mut futex1,
                0,
                &raw mut futex2,
                ptr::null_mut(),
                futex_flag,
            );
            if ret == 0 {
                /* got the lock at futex2 */
                futex_unlock_pi(&raw mut futex2, futex_flag);
                break;
            }

            if ret != 0 && errno != EAGAIN {
                if !params.silent {
                    warnx(c"futex_wait_requeue_pi".as_ptr());
                }
                break;
            }
        }
    }

    ptr::null_mut()
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
        let mut thread_attr: pthread_attr_t = mem::zeroed();

        pthread_attr_init(&mut thread_attr);
        CPU_ZERO_S(size, cpuset);
        CPU_SET_S(
            perf_cpu_map__cpu(cpu, i % perf_cpu_map__nr(cpu)).cpu,
            size,
            cpuset,
        );

        if pthread_attr_setaffinity_np(&mut thread_attr, size, cpuset) != 0 {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, c"pthread_attr_setaffinity_np".as_ptr());
        }

        if pthread_create(w.add(i as usize), &thread_attr, workerfn, ptr::null_mut()) != 0 {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, c"pthread_create".as_ptr());
        }
        pthread_attr_destroy(&mut thread_attr);
        i += 1;
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
pub unsafe extern "C" fn bench_futex_requeue(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut act: sigaction = mem::zeroed();
    let mut cpu: *mut perf_cpu_map;

    argc = parse_options(
        argc,
        argv,
        options.as_ptr(),
        bench_futex_requeue_usage.as_ptr(),
        0,
    );
    if argc != 0 {
        usage_with_options(bench_futex_requeue_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    cpu = perf_cpu_map__new_online_cpus();
    if cpu.is_null() {
        err(EXIT_FAILURE, c"cpu_map__new".as_ptr());
    }

    sigfillset(&mut act.sa_mask);
    act.sa_sigaction = Some(toggle_done);
    sigaction(SIGINT, &act, ptr::null_mut());

    if params.mlockall {
        if mlockall(MCL_CURRENT | MCL_FUTURE) != 0 {
            err(EXIT_FAILURE, c"mlockall".as_ptr());
        }
    }

    if params.nthreads == 0 {
        params.nthreads = perf_cpu_map__nr(cpu);
    }

    worker = calloc(params.nthreads as size_t, mem::size_of::<pthread_t>()) as *mut pthread_t;
    if worker.is_null() {
        err(EXIT_FAILURE, c"calloc".as_ptr());
    }

    if !params.fshared {
        futex_flag = FUTEX_PRIVATE_FLAG;
    }

    if params.nrequeue > params.nthreads {
        params.nrequeue = params.nthreads;
    }

    if params.broadcast {
        params.nrequeue = params.nthreads;
    }

    futex_set_nbuckets_param(&raw mut params);

    printf(
        c"Run summary [PID %d]: Requeuing %d threads (from [%s] %p to %s%p), %d at a time.\n\n"
            .as_ptr(),
        getpid(),
        params.nthreads,
        if params.fshared { c"shared".as_ptr() } else { c"private".as_ptr() },
        &raw mut futex1,
        if params.pi { c"PI ".as_ptr() } else { c"".as_ptr() },
        &raw mut futex2,
        params.nrequeue,
    );

    init_stats(&raw mut requeued_stats);
    init_stats(&raw mut requeuetime_stats);
    mutex_init(&raw mut thread_lock);
    cond_init(&raw mut thread_parent);
    cond_init(&raw mut thread_worker);

    j = 0;
    while j < bench_repeat && !done {
        let mut nrequeued: c_uint = 0;
        let mut wakeups: c_uint = 0;
        let mut start: timeval = mem::zeroed();
        let mut end: timeval = mem::zeroed();
        let mut runtime: timeval = mem::zeroed();

        /* create, launch & block all threads */
        block_threads(worker, cpu);

        /* make sure all threads are already blocked */
        mutex_lock(&raw mut thread_lock);
        while threads_starting != 0 {
            cond_wait(&raw mut thread_parent, &raw mut thread_lock);
        }
        cond_broadcast(&raw mut thread_worker);
        mutex_unlock(&raw mut thread_lock);

        usleep(100000);

        /* Ok, all threads are patiently blocked, start requeueing */
        gettimeofday(&mut start, ptr::null_mut());
        while nrequeued < params.nthreads {
            let r: c_int;

            /*
             * For the regular non-pi case, do not wakeup any tasks
             * blocked on futex1, allowing us to really measure
             * futex_wait functionality. For the PI case the first
             * waiter is always awoken.
             */
            if !params.pi {
                r = futex_cmp_requeue(
                    &raw mut futex1,
                    0,
                    &raw mut futex2,
                    0,
                    params.nrequeue,
                    futex_flag,
                );
            } else {
                r = futex_cmp_requeue_pi(
                    &raw mut futex1,
                    0,
                    &raw mut futex2,
                    params.nrequeue,
                    futex_flag,
                );
                wakeups += 1; /* assume no error */
            }

            if r < 0 {
                err(
                    EXIT_FAILURE,
                    c"couldn't requeue from %p to %p".as_ptr(),
                    &raw mut futex1,
                    &raw mut futex2,
                );
            }

            nrequeued += r as c_uint;
        }

        gettimeofday(&mut end, ptr::null_mut());
        timersub(&end, &start, &mut runtime);

        update_stats(&raw mut requeued_stats, nrequeued);
        update_stats(&raw mut requeuetime_stats, runtime.tv_usec as c_uint);

        if !params.silent {
            if !params.pi {
                printf(
                    c"[Run %d]: Requeued %d of %d threads in %.4f ms\n".as_ptr(),
                    j + 1,
                    nrequeued,
                    params.nthreads,
                    runtime.tv_usec as f64 / USEC_PER_MSEC as f64,
                );
            } else {
                nrequeued -= wakeups;
                printf(
                    c"[Run %d]: Awoke and Requeued (%d+%d) of %d threads in %.4f ms\n".as_ptr(),
                    j + 1,
                    wakeups,
                    nrequeued,
                    params.nthreads,
                    runtime.tv_usec as f64 / USEC_PER_MSEC as f64,
                );
            }
        }

        if !params.pi {
            /* everybody should be blocked on futex2, wake'em up */
            nrequeued = futex_wake(&raw mut futex2, nrequeued, futex_flag);
            if params.nthreads != nrequeued {
                warnx(
                    c"couldn't wakeup all tasks (%d/%d)".as_ptr(),
                    nrequeued,
                    params.nthreads,
                );
            }
        }

        i = 0;
        while i < params.nthreads {
            ret = pthread_join(*worker.add(i as usize), ptr::null_mut());
            if ret != 0 {
                err(EXIT_FAILURE, c"pthread_join".as_ptr());
            }
            i += 1;
        }

        j += 1;
    }

    /* cleanup & report results */
    cond_destroy(&raw mut thread_parent);
    cond_destroy(&raw mut thread_worker);
    mutex_destroy(&raw mut thread_lock);

    print_summary();

    free(worker as *mut c_void);
    perf_cpu_map__put(cpu);
    ret
}
