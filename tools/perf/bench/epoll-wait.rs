// SPDX-License-Identifier: GPL-2.0
// C source was compiled only under HAVE_EVENTFD_SUPPORT.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type pthread_t = c_ulong;
type rlim_t = u64;
type time_t = c_long;
type suseconds_t = c_long;

const EXIT_FAILURE: c_int = 1;
const NULL: *mut c_void = ptr::null_mut();
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;
const RAND_MAX: c_int = 2147483647;
const SIGINT: c_int = 2;
const RLIMIT_NOFILE: c_int = 7;
const EFD_NONBLOCK: c_int = 0o4000;
const EPOLLIN: u32 = 0x001;
const EPOLLHUP: u32 = 0x010;
const EPOLLET: u32 = 1u32 << 31;
const EPOLLONESHOT: u32 = 1u32 << 30;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CTL_MOD: c_int = 3;

/* Maximum number of nesting allowed inside epoll sets */
const EPOLL_MAXNESTS: c_uint = 4;

#[repr(C)]
struct option {
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

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

#[repr(C)]
struct sigset_t {
    _private: [u8; 0],
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
struct sigaction {
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

#[repr(C)]
struct worker {
    tid: c_int,
    epollfd: c_int, /* for --multiq */
    thread: pthread_t,
    ops: c_ulong,
    fdmap: *mut c_int,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut bench__start: timeval;
    static mut bench__end: timeval;
    static mut bench__runtime: timeval;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    static mut stdout: *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn rand() -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn getpid() -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(attr: *mut pthread_attr_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const pthread_attr_t, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, set: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, set: *mut cpu_set_t);
    fn CPU_FREE(set: *mut cpu_set_t);
    fn BUG_ON(condition: bool);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn cond_init(cond: *mut cond);
    fn cond_wait(cond: *mut cond, lock: *mut mutex);
    fn cond_signal(cond: *mut cond);
    fn cond_broadcast(cond: *mut cond);
    fn cond_destroy(cond: *mut cond);
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: c_ulong);
    fn avg_stats(stats: *mut stats) -> c_ulong;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn rel_stddev_stats(stddev: f64, avg: c_ulong) -> f64;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_uint) -> perf_cpu;
    fn cpu__max_cpu() -> perf_cpu;
    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option, usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn exit(status: c_int) -> !;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
}

static mut nthreads: c_uint = 0;
static mut nsecs: c_uint = 8;
static mut wdone: bool = false;
static mut done: bool = false;
static mut __verbose: bool = false;
static mut randomize: bool = false;
static mut nonblocking: bool = false;

/*
 * epoll related shared variables.
 */
static mut epollfd: c_int = 0;
static mut epollfdp: *mut c_int = ptr::null_mut();
static mut noaffinity: bool = false;
static mut nested: c_uint = 0;
static mut et: bool = false; /* edge-trigger */
static mut oneshot: bool = false;
static mut multiq: bool = false; /* use an epoll instance per thread */

/* amount of fds to monitor, per thread */
static mut nfds: c_uint = 64;

static mut thread_lock: mutex = mutex { _private: [] };
static mut threads_starting: c_uint = 0;
static mut throughput_stats: stats = stats { _private: [] };
static mut thread_parent: cond = cond { _private: [] };
static mut thread_worker: cond = cond { _private: [] };

// Original C initializes this with OPT_* parse-options macros:
// threads, runtime, nfds, noaffinity, randomize, verbose, multiq,
// nonblocking, nested, oneshot, edge, and OPT_END.
static options: [option; 0] = [];

static BENCH_EPOLL_WAIT_USAGE_0: &[u8] = b"perf bench epoll wait <options>\0";
static bench_epoll_wait_usage: [*const c_char; 2] = [
    BENCH_EPOLL_WAIT_USAGE_0.as_ptr() as *const c_char,
    ptr::null(),
];

unsafe fn printinfo(fmt: *const c_char) {
    if __verbose {
        printf(fmt);
        fflush(stdout);
    }
}

unsafe fn timersub(a: *const timeval, b: *const timeval, res: *mut timeval) {
    (*res).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*res).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*res).tv_usec < 0 {
        (*res).tv_sec -= 1;
        (*res).tv_usec += 1_000_000;
    }
}

/*
 * Arrange the N elements of ARRAY in random order.
 * Only effective if N is much smaller than RAND_MAX;
 * if this may not be the case, use a better random
 * number generator. -- Ben Pfaff.
 */
unsafe fn shuffle(array: *mut c_void, n: size_t, size: size_t) {
    let carray = array as *mut c_char;
    let aux: *mut c_void;
    let mut i: size_t;

    if n <= 1 {
        return;
    }

    aux = calloc(1, size);
    if aux.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
    }

    i = 1;
    while i < n {
        let mut j = i + (rand() as size_t) / ((RAND_MAX as size_t) / (n - i) + 1);
        j *= size;

        memcpy(aux, carray.add(j) as *const c_void, size);
        memcpy(carray.add(j) as *mut c_void, carray.add(i * size) as *const c_void, size);
        memcpy(carray.add(i * size) as *mut c_void, aux, size);
        i += 1;
    }

    free(aux);
}

unsafe extern "C" fn workerfn(arg: *mut c_void) -> *mut c_void {
    let mut fd: c_int;
    let mut ret: c_int;
    let mut r: ssize_t;
    let w = arg as *mut worker;
    let mut ops = (*w).ops;
    let mut ev: epoll_event = mem::zeroed();
    let mut val: u64 = 0;
    let to: c_int = if nonblocking { 0 } else { -1 };
    let efd: c_int = if multiq { (*w).epollfd } else { epollfd };

    mutex_lock(&raw mut thread_lock);
    threads_starting -= 1;
    if threads_starting == 0 {
        cond_signal(&raw mut thread_parent);
    }
    cond_wait(&raw mut thread_worker, &raw mut thread_lock);
    mutex_unlock(&raw mut thread_lock);

    loop {
        /*
         * Block indefinitely waiting for the IN event.
         * In order to stress the epoll_wait(2) syscall,
         * call it event per event, instead of a larger
         * batch (max)limit.
         */
        loop {
            ret = epoll_wait(efd, &mut ev, 1, to);
            if !(ret < 0 && errno == EINTR) {
                break;
            }
        }
        if ret < 0 {
            err(EXIT_FAILURE, b"epoll_wait\0".as_ptr() as *const c_char);
        }

        fd = ev.data.fd;

        loop {
            r = read(fd, &mut val as *mut u64 as *mut c_void, mem::size_of_val(&val));
            if !(!done && (r < 0 && errno == EAGAIN)) {
                break;
            }
        }

        if et {
            ev.events = EPOLLIN | EPOLLET;
            ret = epoll_ctl(efd, EPOLL_CTL_ADD, fd, &mut ev);
        }

        if oneshot {
            /* rearm the file descriptor with a new event mask */
            ev.events |= EPOLLIN | EPOLLONESHOT;
            ret = epoll_ctl(efd, EPOLL_CTL_MOD, fd, &mut ev);
        }

        ops += 1;
        if done {
            break;
        }
    }

    if multiq {
        close((*w).epollfd);
    }

    (*w).ops = ops;
    ptr::null_mut()
}

unsafe fn nest_epollfd(w: *mut worker) {
    let mut i: c_uint;
    let mut ev: epoll_event = mem::zeroed();
    let efd: c_int = if multiq { (*w).epollfd } else { epollfd };

    if nested > EPOLL_MAXNESTS {
        nested = EPOLL_MAXNESTS;
    }

    epollfdp = calloc(nested as size_t, mem::size_of::<c_int>()) as *mut c_int;
    if epollfdp.is_null() {
        err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
    }

    i = 0;
    while i < nested {
        *epollfdp.add(i as size_t) = epoll_create(1);
        if *epollfdp.add(i as size_t) < 0 {
            err(EXIT_FAILURE, b"epoll_create\0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    ev.events = EPOLLHUP; /* anything */
    ev.data.u64_ = i as u64; /* any number */

    i = nested - 1;
    while i != 0 {
        if epoll_ctl(*epollfdp.add((i - 1) as size_t), EPOLL_CTL_ADD, *epollfdp.add(i as size_t), &mut ev) < 0 {
            err(EXIT_FAILURE, b"epoll_ctl\0".as_ptr() as *const c_char);
        }
        i -= 1;
    }

    if epoll_ctl(efd, EPOLL_CTL_ADD, *epollfdp, &mut ev) < 0 {
        err(EXIT_FAILURE, b"epoll_ctl\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn toggle_done(_sig: c_int, _info: *mut siginfo_t, _uc: *mut c_void) {
    /* inform all threads that we're done for the day */
    done = true;
    gettimeofday(&raw mut bench__end, ptr::null_mut());
    timersub(&raw const bench__end, &raw const bench__start, &raw mut bench__runtime);
}

unsafe fn print_summary() {
    let avg: c_ulong = avg_stats(&raw mut throughput_stats);
    let stddev: f64 = stddev_stats(&raw mut throughput_stats);

    printf(
        b"\nAveraged %ld operations/sec (+- %.2f%%), total secs = %d\n\0".as_ptr() as *const c_char,
        avg,
        rel_stddev_stats(stddev, avg),
        bench__runtime.tv_sec as c_int,
    );
}

unsafe fn do_threads(worker: *mut worker, cpu: *mut perf_cpu_map) -> c_int {
    let mut thread_attr: pthread_attr_t = mem::zeroed();
    let mut attrp: *mut pthread_attr_t = ptr::null_mut();
    let cpuset: *mut cpu_set_t;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut ret: c_int = 0;
    let mut events: u32 = EPOLLIN;
    let nrcpus: c_int;
    let size: size_t;

    if oneshot {
        events |= EPOLLONESHOT;
    }
    if et {
        events |= EPOLLET;
    }

    if __verbose {
        printf(
            b"starting worker/consumer %sthreads%s\n\0".as_ptr() as *const c_char,
            if noaffinity { b"\0".as_ptr() } else { b"CPU affinity \0".as_ptr() } as *const c_char,
            if nonblocking { b" (nonblocking)\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char,
        );
        fflush(stdout);
    }
    if !noaffinity {
        pthread_attr_init(&mut thread_attr);
    }

    nrcpus = cpu__max_cpu().cpu;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(cpuset.is_null());
    size = CPU_ALLOC_SIZE(nrcpus);

    i = 0;
    while i < nthreads {
        let w = worker.add(i as size_t);

        if multiq {
            (*w).epollfd = epoll_create(1);
            if (*w).epollfd < 0 {
                err(EXIT_FAILURE, b"epoll_create\0".as_ptr() as *const c_char);
            }

            if nested != 0 {
                nest_epollfd(w);
            }
        }

        (*w).tid = i as c_int;
        (*w).fdmap = calloc(nfds as size_t, mem::size_of::<c_int>()) as *mut c_int;
        if (*w).fdmap.is_null() {
            return 1;
        }

        j = 0;
        while j < nfds {
            let efd: c_int = if multiq { (*w).epollfd } else { epollfd };
            let mut ev: epoll_event = mem::zeroed();

            *(*w).fdmap.add(j as size_t) = eventfd(0, EFD_NONBLOCK);
            if *(*w).fdmap.add(j as size_t) < 0 {
                err(EXIT_FAILURE, b"eventfd\0".as_ptr() as *const c_char);
            }

            ev.data.fd = *(*w).fdmap.add(j as size_t);
            ev.events = events;

            ret = epoll_ctl(efd, EPOLL_CTL_ADD, *(*w).fdmap.add(j as size_t), &mut ev);
            if ret < 0 {
                err(EXIT_FAILURE, b"epoll_ctl\0".as_ptr() as *const c_char);
            }
            j += 1;
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
                err(EXIT_FAILURE, b"pthread_attr_setaffinity_np\0".as_ptr() as *const c_char);
            }

            attrp = &mut thread_attr;
        }

        ret = pthread_create(&mut (*w).thread, attrp, workerfn, w as *mut c_void);
        if ret != 0 {
            CPU_FREE(cpuset);
            err(EXIT_FAILURE, b"pthread_create\0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    CPU_FREE(cpuset);
    if !noaffinity {
        pthread_attr_destroy(&mut thread_attr);
    }

    ret
}

unsafe extern "C" fn writerfn(p: *mut c_void) -> *mut c_void {
    let worker = p as *mut worker;
    let mut i: size_t;
    let mut j: size_t;
    let mut iter: size_t;
    let val: u64 = 1;
    let mut sz: ssize_t;
    let ts = timespec {
        tv_sec: 0,
        tv_nsec: 500,
    };

    if __verbose {
        printf(
            b"starting writer-thread: doing %s writes ...\n\0".as_ptr() as *const c_char,
            if randomize { b"random\0".as_ptr() } else { b"lineal\0".as_ptr() } as *const c_char,
        );
        fflush(stdout);
    }

    iter = 0;
    while !wdone {
        if randomize {
            shuffle(worker as *mut c_void, nthreads as size_t, mem::size_of::<worker>());
        }

        i = 0;
        while i < nthreads as size_t {
            let w = worker.add(i);

            if randomize {
                shuffle((*w).fdmap as *mut c_void, nfds as size_t, mem::size_of::<c_int>());
            }

            j = 0;
            while j < nfds as size_t {
                loop {
                    sz = write(*(*w).fdmap.add(j), &val as *const u64 as *const c_void, mem::size_of_val(&val));
                    if !(!wdone && (sz < 0 && errno == EAGAIN)) {
                        break;
                    }
                }
                j += 1;
            }
            i += 1;
        }

        nanosleep(&ts, ptr::null_mut());
        iter += 1;
    }

    if __verbose {
        printf(
            b"exiting writer-thread (total full-loops: %zd)\n\0".as_ptr() as *const c_char,
            iter,
        );
        fflush(stdout);
    }
    ptr::null_mut()
}

unsafe extern "C" fn cmpworker(p1: *const c_void, p2: *const c_void) -> c_int {
    let w1 = p1 as *mut worker;
    let w2 = p2 as *mut worker;

    if (*w1).tid > (*w2).tid {
        return 1;
    }
    if (*w1).tid < (*w2).tid {
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bench_epoll_wait(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut act: sigaction = mem::zeroed();
    let mut i: c_uint;
    let mut worker: *mut worker = ptr::null_mut();
    let cpu: *mut perf_cpu_map;
    let mut wthread: pthread_t = 0;
    let mut rl: rlimit = mem::zeroed();
    let mut prevrl: rlimit = mem::zeroed();

    argc = parse_options(argc, argv, options.as_ptr(), bench_epoll_wait_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(bench_epoll_wait_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    memset(&mut act as *mut sigaction as *mut c_void, 0, mem::size_of_val(&act));
    sigfillset(&mut act.sa_mask);
    act.sa_sigaction = Some(toggle_done);
    sigaction(SIGINT, &act, ptr::null_mut());

    cpu = perf_cpu_map__new_online_cpus();
    if cpu.is_null() {
        errmem();
    }

    /* a single, main epoll instance */
    if !multiq {
        epollfd = epoll_create(1);
        if epollfd < 0 {
            err(EXIT_FAILURE, b"epoll_create\0".as_ptr() as *const c_char);
        }

        /*
         * Deal with nested epolls, if any.
         */
        if nested != 0 {
            nest_epollfd(ptr::null_mut());
        }
    }

    if __verbose {
        printf(
            b"Using %s queue model\n\0".as_ptr() as *const c_char,
            if multiq { b"multi\0".as_ptr() } else { b"single\0".as_ptr() } as *const c_char,
        );
        fflush(stdout);
        printf(b"Nesting level(s): %d\n\0".as_ptr() as *const c_char, nested);
        fflush(stdout);
    }

    /* default to the number of CPUs and leave one for the writer pthread */
    if nthreads == 0 {
        nthreads = perf_cpu_map__nr(cpu) - 1;
    }

    worker = calloc(nthreads as size_t, mem::size_of::<worker>()) as *mut worker;
    if worker.is_null() {
        errmem();
    }

    if getrlimit(RLIMIT_NOFILE, &mut prevrl) != 0 {
        err(EXIT_FAILURE, b"getrlimit\0".as_ptr() as *const c_char);
    }
    rl.rlim_max = (nfds * nthreads * 2 + 50) as rlim_t;
    rl.rlim_cur = rl.rlim_max;
    if __verbose {
        printf(
            b"Setting RLIMIT_NOFILE rlimit from %lu to: %lu\n\0".as_ptr() as *const c_char,
            prevrl.rlim_max as u64,
            rl.rlim_max as u64,
        );
        fflush(stdout);
    }
    if setrlimit(RLIMIT_NOFILE, &rl) < 0 {
        err(EXIT_FAILURE, b"setrlimit\0".as_ptr() as *const c_char);
    }

    printf(
        b"Run summary [PID %d]: %d threads monitoring%s on %d file-descriptors for %d secs.\n\n\0".as_ptr() as *const c_char,
        getpid(),
        nthreads,
        if oneshot { b" (EPOLLONESHOT semantics)\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char,
        nfds,
        nsecs,
    );

    init_stats(&raw mut throughput_stats);
    mutex_init(&raw mut thread_lock);
    cond_init(&raw mut thread_parent);
    cond_init(&raw mut thread_worker);

    threads_starting = nthreads;

    gettimeofday(&raw mut bench__start, ptr::null_mut());

    do_threads(worker, cpu);

    mutex_lock(&raw mut thread_lock);
    while threads_starting != 0 {
        cond_wait(&raw mut thread_parent, &raw mut thread_lock);
    }
    cond_broadcast(&raw mut thread_worker);
    mutex_unlock(&raw mut thread_lock);

    /*
     * At this point the workers should be blocked waiting for read events
     * to become ready. Launch the writer which will constantly be writing
     * to each thread's fdmap.
     */
    ret = pthread_create(&mut wthread, ptr::null(), writerfn, worker as *mut c_void);
    if ret != 0 {
        err(EXIT_FAILURE, b"pthread_create\0".as_ptr() as *const c_char);
    }

    sleep(nsecs);
    toggle_done(0, ptr::null_mut(), ptr::null_mut());
    if __verbose {
        printf(b"main thread: toggling done\n\0".as_ptr() as *const c_char);
        fflush(stdout);
    }

    sleep(1); /* meh */
    wdone = true;
    ret = pthread_join(wthread, ptr::null_mut());
    if ret != 0 {
        err(EXIT_FAILURE, b"pthread_join\0".as_ptr() as *const c_char);
    }

    /* cleanup & report results */
    cond_destroy(&raw mut thread_parent);
    cond_destroy(&raw mut thread_worker);
    mutex_destroy(&raw mut thread_lock);

    /* sort the array back before reporting */
    if randomize {
        qsort(worker as *mut c_void, nthreads as size_t, mem::size_of::<worker>(), cmpworker);
    }

    i = 0;
    while i < nthreads {
        let t: c_ulong = if bench__runtime.tv_sec > 0 {
            (*worker.add(i as size_t)).ops / bench__runtime.tv_sec as c_ulong
        } else {
            0
        };

        update_stats(&raw mut throughput_stats, t);

        if nfds == 1 {
            printf(
                b"[thread %2d] fdmap: %p [ %04ld ops/sec ]\n\0".as_ptr() as *const c_char,
                (*worker.add(i as size_t)).tid,
                (*worker.add(i as size_t)).fdmap.add(0),
                t,
            );
        } else {
            printf(
                b"[thread %2d] fdmap: %p ... %p [ %04ld ops/sec ]\n\0".as_ptr() as *const c_char,
                (*worker.add(i as size_t)).tid,
                (*worker.add(i as size_t)).fdmap.add(0),
                (*worker.add(i as size_t)).fdmap.add((nfds - 1) as size_t),
                t,
            );
        }
        i += 1;
    }

    print_summary();

    close(epollfd);
    perf_cpu_map__put(cpu);
    i = 0;
    while i < nthreads {
        free((*worker.add(i as size_t)).fdmap as *mut c_void);
        i += 1;
    }

    free(worker as *mut c_void);
    ret
}

unsafe fn errmem() -> ! {
    err(EXIT_FAILURE, b"calloc\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
