// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 Davidlohr Bueso.
 *
 * Benchmark the various operations allowed for epoll_ctl(2).
 * The idea is to concurrently stress a single epoll instance
 */

// Translated from C under the original HAVE_EVENTFD_SUPPORT conditional.

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

const EXIT_FAILURE: c_int = 1;
const EPOLL_MAXNESTS: c_uint = 4;

const OP_EPOLL_ADD: usize = 0;
const OP_EPOLL_MOD: usize = 1;
const OP_EPOLL_DEL: usize = 2;
const EPOLL_NR_OPS: usize = 3;

const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CTL_DEL: c_int = 2;
const EPOLL_CTL_MOD: c_int = 3;
const EPOLLIN: u32 = 0x001;
const EPOLLOUT: u32 = 0x004;
const EPOLLHUP: u32 = 0x010;
const EFD_NONBLOCK: c_int = 0x800;
const SIGINT: c_int = 2;
const RLIMIT_NOFILE: c_int = 7;

type size_t = usize;
type pthread_t = c_ulong;
type rlim_t = u64;

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
pub struct option {
    _private: [u8; 0],
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
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 128],
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
#[derive(Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union epoll_data {
    pub ptr: *mut c_void,
    pub fd: c_int,
    pub u32_: u32,
    pub u64_: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data,
}

#[repr(C)]
pub struct worker {
    pub tid: c_int,
    pub thread: pthread_t,
    pub ops: [c_ulong; EPOLL_NR_OPS],
    pub fdmap: *mut c_int,
}

unsafe extern "C" {
    static mut bench__start: timeval;
    static mut bench__end: timeval;
    static mut bench__runtime: timeval;

    static options: [option; 0];
    static bench_epoll_ctl_usage: [*const c_char; 0];

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn exit(status: c_int) -> !;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn random() -> c_long;
    fn getpid() -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn close(fd: c_int) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn timersub(a: *const timeval, b: *const timeval, res: *mut timeval);
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;

    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;

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
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, set: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, set: *mut cpu_set_t);
    fn CPU_FREE(set: *mut cpu_set_t);
    fn BUG_ON(cond: bool);

    fn mutex_init(mutex: *mut mutex);
    fn mutex_destroy(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn cond_init(cond: *mut cond);
    fn cond_destroy(cond: *mut cond);
    fn cond_wait(cond: *mut cond, mutex: *mut mutex);
    fn cond_signal(cond: *mut cond);
    fn cond_broadcast(cond: *mut cond);

    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: c_ulong);
    fn avg_stats(stats: *mut stats) -> c_ulong;
    fn stddev_stats(stats: *mut stats) -> c_double;
    fn rel_stddev_stats(stddev: c_double, avg: c_ulong) -> c_double;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *mut perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(map: *mut perf_cpu_map, idx: c_uint) -> perf_cpu;
    fn cpu__max_cpu() -> perf_cpu;
}

static mut nthreads: c_uint = 0;
static mut nsecs: c_uint = 8;
static mut done: bool = false;
static mut __verbose: bool = false;
static mut randomize: bool = false;

/*
 * epoll related shared variables.
 */
static mut epollfd: c_int = 0;
static mut epollfdp: *mut c_int = core::ptr::null_mut();
static mut noaffinity: bool = false;
static mut nested: c_uint = 0;

/* amount of fds to monitor, per thread */
static mut nfds: c_uint = 64;

static mut thread_lock: mutex = mutex { _private: [] };
static mut threads_starting: c_uint = 0;
static mut all_stats: [stats; EPOLL_NR_OPS] = [
    stats { _private: [] },
    stats { _private: [] },
    stats { _private: [] },
];
static mut thread_parent: cond = cond { _private: [] };
static mut thread_worker: cond = cond { _private: [] };

unsafe fn printinfo(fmt: *const c_char, args: impl FnOnce()) {
    if __verbose {
        let _ = fmt;
        args();
    }
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

unsafe fn nest_epollfd() {
    let mut i: c_uint;
    let mut ev = epoll_event {
        events: 0,
        data: epoll_data { u64_: 0 },
    };

    if nested > EPOLL_MAXNESTS {
        nested = EPOLL_MAXNESTS;
    }
    if __verbose {
        printf(c"Nesting level(s): %d\n".as_ptr(), nested);
    }

    epollfdp = calloc(nested as size_t, core::mem::size_of::<c_int>()) as *mut c_int;
    if epollfdp.is_null() {
        err(EXIT_FAILURE, c"calloc".as_ptr());
    }

    i = 0;
    while i < nested {
        *epollfdp.add(i as usize) = epoll_create(1);
        if epollfd < 0 {
            err(EXIT_FAILURE, c"epoll_create".as_ptr());
        }
        i += 1;
    }

    ev.events = EPOLLHUP; /* anything */
    ev.data.u64_ = i as u64; /* any number */

    i = nested.wrapping_sub(1);
    while i != 0 {
        if epoll_ctl(
            *epollfdp.add(i.wrapping_sub(1) as usize),
            EPOLL_CTL_ADD,
            *epollfdp.add(i as usize),
            &mut ev,
        ) < 0
        {
            err(EXIT_FAILURE, c"epoll_ctl".as_ptr());
        }
        i = i.wrapping_sub(1);
    }

    if epoll_ctl(epollfd, EPOLL_CTL_ADD, *epollfdp, &mut ev) < 0 {
        err(EXIT_FAILURE, c"epoll_ctl".as_ptr());
    }
}

unsafe fn do_epoll_op(w: *mut worker, op: c_int, fd: c_int) {
    let error: c_int;
    let mut ev = epoll_event {
        events: 0,
        data: epoll_data { u64_: 0 },
    };

    ev.events = EPOLLIN;
    ev.data.u64_ = fd as u64;

    match op as usize {
        OP_EPOLL_ADD => {
            error = epoll_ctl(epollfd, EPOLL_CTL_ADD, fd, &mut ev);
        }
        OP_EPOLL_MOD => {
            ev.events = EPOLLOUT;
            error = epoll_ctl(epollfd, EPOLL_CTL_MOD, fd, &mut ev);
        }
        OP_EPOLL_DEL => {
            error = epoll_ctl(epollfd, EPOLL_CTL_DEL, fd, core::ptr::null_mut());
        }
        _ => {
            error = 1;
        }
    }

    if error == 0 {
        (*w).ops[op as usize] = (*w).ops[op as usize].wrapping_add(1);
    }
}

unsafe fn do_random_epoll_op(w: *mut worker) {
    let rnd1: c_ulong = random() as c_ulong;
    let rnd2: c_ulong = random() as c_ulong;
    let op: c_int;
    let fd: c_int;

    fd = *(*w).fdmap.add((rnd1 % nfds as c_ulong) as usize);
    op = (rnd2 % EPOLL_NR_OPS as c_ulong) as c_int;

    do_epoll_op(w, op, fd);
}

unsafe extern "C" fn workerfn(arg: *mut c_void) -> *mut c_void {
    let mut i: c_uint;
    let w: *mut worker = arg as *mut worker;
    let ts = timespec {
        tv_sec: 0,
        tv_nsec: 250,
    };

    mutex_lock(&raw mut thread_lock);
    threads_starting = threads_starting.wrapping_sub(1);
    if threads_starting == 0 {
        cond_signal(&raw mut thread_parent);
    }
    cond_wait(&raw mut thread_worker, &raw mut thread_lock);
    mutex_unlock(&raw mut thread_lock);

    /* Let 'em loose */
    loop {
        /* random */
        if randomize {
            do_random_epoll_op(w);
        } else {
            i = 0;
            while i < nfds {
                do_epoll_op(w, OP_EPOLL_ADD as c_int, *(*w).fdmap.add(i as usize));
                do_epoll_op(w, OP_EPOLL_MOD as c_int, *(*w).fdmap.add(i as usize));
                do_epoll_op(w, OP_EPOLL_DEL as c_int, *(*w).fdmap.add(i as usize));
                i += 1;
            }
        }

        nanosleep(&ts, core::ptr::null_mut());
        if done {
            break;
        }
    }

    core::ptr::null_mut()
}

unsafe fn init_fdmaps(w: *mut worker, pct: c_int) {
    let mut i: c_uint;
    let inc: c_int;
    let mut ev = epoll_event {
        events: 0,
        data: epoll_data { fd: 0 },
    };

    if pct == 0 {
        return;
    }

    inc = 100 / pct;
    i = 0;
    while i < nfds {
        ev.data.fd = *(*w).fdmap.add(i as usize);
        ev.events = EPOLLIN;

        if epoll_ctl(epollfd, EPOLL_CTL_ADD, *(*w).fdmap.add(i as usize), &mut ev) < 0 {
            err(EXIT_FAILURE, c"epoll_ct".as_ptr());
        }
        i = i.wrapping_add(inc as c_uint);
    }
}

unsafe fn do_threads(worker: *mut worker, cpu: *mut perf_cpu_map) -> c_int {
    let mut thread_attr = pthread_attr_t { _private: [] };
    let mut attrp: *mut pthread_attr_t = core::ptr::null_mut();
    let mut i: c_uint;
    let mut j: c_uint;
    let mut ret: c_int = 0;
    let nrcpus: c_int;
    let cpuset: *mut cpu_set_t;
    let size: size_t;

    if !noaffinity {
        pthread_attr_init(&mut thread_attr);
    }

    nrcpus = cpu__max_cpu().cpu;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(cpuset.is_null());
    size = CPU_ALLOC_SIZE(nrcpus);

    i = 0;
    while i < nthreads {
        let w = worker.add(i as usize);

        (*w).tid = i as c_int;
        (*w).fdmap = calloc(nfds as size_t, core::mem::size_of::<c_int>()) as *mut c_int;
        if (*w).fdmap.is_null() {
            return 1;
        }

        j = 0;
        while j < nfds {
            *(*w).fdmap.add(j as usize) = eventfd(0, EFD_NONBLOCK);
            if *(*w).fdmap.add(j as usize) < 0 {
                err(EXIT_FAILURE, c"eventfd".as_ptr());
            }
            j += 1;
        }

        /*
         * Lets add 50% of the fdmap to the epoll instance, and
         * do it before any threads are started; otherwise there is
         * an initial bias of the call failing  (mod and del ops).
         */
        if randomize {
            init_fdmaps(w, 50);
        }

        if !noaffinity {
            CPU_ZERO_S(size, cpuset);
            CPU_SET_S(
                perf_cpu_map__cpu(cpu, i % perf_cpu_map__nr(cpu)).cpu,
                size,
                cpuset,
            );

            ret = pthread_attr_setaffinity_np(&mut thread_attr, size, cpuset);
            if ret != 0 {
                CPU_FREE(cpuset);
                err(EXIT_FAILURE, c"pthread_attr_setaffinity_np".as_ptr());
            }

            attrp = &mut thread_attr;
        }

        ret = pthread_create(
            &mut (*w).thread,
            attrp,
            workerfn,
            w as *mut c_void,
        );
        if ret != 0 {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, c"pthread_create".as_ptr());
        }
        i += 1;
    }

    CPU_FREE(cpuset);
    if !noaffinity {
        pthread_attr_destroy(&mut thread_attr);
    }

    ret
}

unsafe fn print_summary() {
    let mut i: c_int;
    let mut avg = [0 as c_ulong; EPOLL_NR_OPS];
    let mut stddev = [0.0 as c_double; EPOLL_NR_OPS];

    i = 0;
    while (i as usize) < EPOLL_NR_OPS {
        avg[i as usize] = avg_stats(&raw mut all_stats[i as usize]);
        stddev[i as usize] = stddev_stats(&raw mut all_stats[i as usize]);
        i += 1;
    }

    printf(
        c"\nAveraged %ld ADD operations (+- %.2f%%)\n".as_ptr(),
        avg[OP_EPOLL_ADD],
        rel_stddev_stats(stddev[OP_EPOLL_ADD], avg[OP_EPOLL_ADD]),
    );
    printf(
        c"Averaged %ld MOD operations (+- %.2f%%)\n".as_ptr(),
        avg[OP_EPOLL_MOD],
        rel_stddev_stats(stddev[OP_EPOLL_MOD], avg[OP_EPOLL_MOD]),
    );
    printf(
        c"Averaged %ld DEL operations (+- %.2f%%)\n".as_ptr(),
        avg[OP_EPOLL_DEL],
        rel_stddev_stats(stddev[OP_EPOLL_DEL], avg[OP_EPOLL_DEL]),
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_epoll_ctl(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut j: c_int;
    let mut ret: c_int = 0;
    let mut act: sigaction = core::mem::zeroed();
    let mut worker: *mut worker = core::ptr::null_mut();
    let cpu: *mut perf_cpu_map;
    let mut rl: rlimit = core::mem::zeroed();
    let mut prevrl: rlimit = core::mem::zeroed();
    let mut i: c_uint;

    argc = parse_options(argc, argv, options.as_ptr(), bench_epoll_ctl_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(bench_epoll_ctl_usage.as_ptr(), options.as_ptr());
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

    cpu = perf_cpu_map__new_online_cpus();
    if cpu.is_null() {
        errmem();
    }

    /* a single, main epoll instance */
    epollfd = epoll_create(1);
    if epollfd < 0 {
        err(EXIT_FAILURE, c"epoll_create".as_ptr());
    }

    /*
     * Deal with nested epolls, if any.
     */
    if nested != 0 {
        nest_epollfd();
    }

    /* default to the number of CPUs */
    if nthreads == 0 {
        nthreads = perf_cpu_map__nr(cpu);
    }

    worker = calloc(nthreads as size_t, core::mem::size_of::<worker>()) as *mut worker;
    if worker.is_null() {
        errmem();
    }

    if getrlimit(RLIMIT_NOFILE, &mut prevrl) != 0 {
        err(EXIT_FAILURE, c"getrlimit".as_ptr());
    }
    rl.rlim_cur = (nfds as rlim_t)
        .wrapping_mul(nthreads as rlim_t)
        .wrapping_mul(2)
        .wrapping_add(50);
    rl.rlim_max = rl.rlim_cur;
    if __verbose {
        printf(
            c"Setting RLIMIT_NOFILE rlimit from %lu to: %lu\n".as_ptr(),
            prevrl.rlim_max,
            rl.rlim_max,
        );
    }
    if setrlimit(RLIMIT_NOFILE, &rl) < 0 {
        err(EXIT_FAILURE, c"setrlimit".as_ptr());
    }

    printf(
        c"Run summary [PID %d]: %d threads doing epoll_ctl ops %d file-descriptors for %d secs.\n\n"
            .as_ptr(),
        getpid(),
        nthreads,
        nfds,
        nsecs,
    );

    i = 0;
    while (i as usize) < EPOLL_NR_OPS {
        init_stats(&raw mut all_stats[i as usize]);
        i += 1;
    }

    mutex_init(&raw mut thread_lock);
    cond_init(&raw mut thread_parent);
    cond_init(&raw mut thread_worker);

    threads_starting = nthreads;

    gettimeofday(&raw mut bench__start, core::ptr::null_mut());

    do_threads(worker, cpu);

    mutex_lock(&raw mut thread_lock);
    while threads_starting != 0 {
        cond_wait(&raw mut thread_parent, &raw mut thread_lock);
    }
    cond_broadcast(&raw mut thread_worker);
    mutex_unlock(&raw mut thread_lock);

    sleep(nsecs);
    toggle_done(0, core::ptr::null_mut(), core::ptr::null_mut());
    if __verbose {
        printf(c"main thread: toggling done\n".as_ptr());
    }

    i = 0;
    while i < nthreads {
        ret = pthread_join((*worker.add(i as usize)).thread, core::ptr::null_mut());
        if ret != 0 {
            err(EXIT_FAILURE, c"pthread_join".as_ptr());
        }
        i += 1;
    }

    /* cleanup & report results */
    cond_destroy(&raw mut thread_parent);
    cond_destroy(&raw mut thread_worker);
    mutex_destroy(&raw mut thread_lock);

    i = 0;
    while i < nthreads {
        let mut t = [0 as c_ulong; EPOLL_NR_OPS];

        j = 0;
        while (j as usize) < EPOLL_NR_OPS {
            t[j as usize] = (*worker.add(i as usize)).ops[j as usize];
            update_stats(&raw mut all_stats[j as usize], t[j as usize]);
            j += 1;
        }

        if nfds == 1 {
            printf(
                c"[thread %2d] fdmap: %p [ add: %04ld; mod: %04ld; del: %04lds ops ]\n"
                    .as_ptr(),
                (*worker.add(i as usize)).tid,
                (*worker.add(i as usize)).fdmap.add(0),
                t[OP_EPOLL_ADD],
                t[OP_EPOLL_MOD],
                t[OP_EPOLL_DEL],
            );
        } else {
            printf(
                c"[thread %2d] fdmap: %p ... %p [ add: %04ld ops; mod: %04ld ops; del: %04ld ops ]\n"
                    .as_ptr(),
                (*worker.add(i as usize)).tid,
                (*worker.add(i as usize)).fdmap.add(0),
                (*worker.add(i as usize)).fdmap.add(nfds.wrapping_sub(1) as usize),
                t[OP_EPOLL_ADD],
                t[OP_EPOLL_MOD],
                t[OP_EPOLL_DEL],
            );
        }
        i += 1;
    }

    print_summary();

    close(epollfd);
    perf_cpu_map__put(cpu);
    i = 0;
    while i < nthreads {
        free((*worker.add(i as usize)).fdmap as *mut c_void);
        i += 1;
    }

    free(worker as *mut c_void);
    ret
}

unsafe fn errmem() -> ! {
    err(EXIT_FAILURE, c"calloc".as_ptr());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
