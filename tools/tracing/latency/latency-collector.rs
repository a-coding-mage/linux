// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017, 2018, 2019, 2021 BMW Car IT GmbH
 * Author: Viktor Rosendahl (viktor.rosendahl@bmw.de)
 */

/* Translated from tracing/latency/latency-collector.c. */

use libc::*;

const OPTIDX_FUNC_TR: usize = 0;
const OPTIDX_DISP_GR: usize = 1;
const OPTIDX_NR: usize = 2;

#[repr(C)]
enum errhandling {
    ERR_EXIT = 0,
    ERR_WARN,
    ERR_CLEANUP,
}

static mut prg_name: *const c_char = 0 as *const c_char;
static prg_unknown: &[u8] = b"unknown program name\0";

static mut fd_stdout: c_int = 0;

static mut sched_policy: c_int = 0;
static mut sched_policy_set: bool = false;

static mut sched_pri: c_int = 0;
static mut sched_pri_set: bool = false;

static mut trace_enable: bool = true;
static mut setup_ftrace: bool = true;
static mut use_random_sleep: bool = false;

static opt_function_trace: &[u8] = b"function-trace\0";
static opt_display_graph: &[u8] = b"display-graph\0";
static optstr: [*const c_char; OPTIDX_NR] = [
    opt_function_trace.as_ptr() as *const c_char,
    opt_display_graph.as_ptr() as *const c_char,
];

static mut use_options: [bool; OPTIDX_NR] = [false; OPTIDX_NR];

static mut inotify_buffer: [c_char; 655360] = [0; 655360];

const DEFAULT_NR_PRINTER_THREADS: c_uint = 3;
static mut nr_threads: c_uint = DEFAULT_NR_PRINTER_THREADS;

const DEFAULT_TABLE_SIZE: c_uint = 2;
static mut table_startsize: c_uint = DEFAULT_TABLE_SIZE;

static mut verbosity: c_int = 0;

#[inline(always)]
unsafe fn verbose_sizechange() -> bool { verbosity >= 1 }
#[inline(always)]
unsafe fn verbose_lostevent() -> bool { verbosity >= 2 }
#[inline(always)]
unsafe fn verbose_ftrace() -> bool { verbosity >= 1 }

#[inline(always)]
unsafe fn was_changed(orig: *const c_char, cur: *const c_char) -> bool { strcmp(orig, cur) != 0 }
#[inline(always)]
unsafe fn needs_change(cur: *const c_char, wanted: *const c_char) -> bool { strcmp(cur, wanted) != 0 }
#[inline(always)]
fn bool2str(x: bool) -> *const c_char {
    if x { b"true\0".as_ptr() as *const c_char } else { b"false\0".as_ptr() as *const c_char }
}

static mut debug_tracefile: *const c_char = 0 as *const c_char;
static mut debug_tracefile_dflt: *const c_char = 0 as *const c_char;
static mut debug_maxlat: *const c_char = 0 as *const c_char;
static mut debug_maxlat_dflt: *const c_char = 0 as *const c_char;
static DEBUG_NOFILE: &[u8] = b"[file not found]\0";

static TR_MAXLAT: &[u8] = b"tracing_max_latency\0";
static TR_THRESH: &[u8] = b"tracing_thresh\0";
static TR_CURRENT: &[u8] = b"current_tracer\0";
static TR_OPTIONS: &[u8] = b"trace_options\0";
static NOP_TRACER: &[u8] = b"nop\0";
static OPT_NO_PREFIX: &[u8] = b"no\0";
static DFLT_THRESHOLD_US: &[u8] = b"0\0";
static mut threshold: *const c_char = DFLT_THRESHOLD_US.as_ptr() as *const c_char;

static DEV_URANDOM: &[u8] = b"/dev/urandom\0";
const RT_DEFAULT_PRI: c_int = 99;
const DEFAULT_PRI: c_int = 0;
const USEC_PER_MSEC: c_long = 1000;
const NSEC_PER_USEC: c_long = 1000;
const NSEC_PER_MSEC: c_long = USEC_PER_MSEC * NSEC_PER_USEC;
const MSEC_PER_SEC: c_long = 1000;
const USEC_PER_SEC: c_long = USEC_PER_MSEC * MSEC_PER_SEC;
const NSEC_PER_SEC: c_long = NSEC_PER_MSEC * MSEC_PER_SEC;
const SLEEP_TIME_MS_DEFAULT: c_long = 1000;
const TRY_PRINTMUTEX_MS: c_int = 1000;
static mut sleep_time: c_long = USEC_PER_MSEC * SLEEP_TIME_MS_DEFAULT;

static queue_full_warning: &[u8] =
    b"Could not queue trace for printing. It is likely that events happen faster\nthan what they can be printed. Probably partly because of random sleeping\n\0";
static no_tracer_msg: &[u8] =
    b"Could not find any tracers! Running this program as root may help!\n\0";
static no_latency_tr_msg: &[u8] =
    b"No latency tracers are supported by your kernel!\n\0";

#[repr(C)]
struct policy {
    name: *const c_char,
    policy: c_int,
    default_pri: c_int,
}

static policy_other: &[u8] = b"other\0";
static policy_batch: &[u8] = b"batch\0";
static policy_idle: &[u8] = b"idle\0";
static policy_rr: &[u8] = b"rr\0";
static policy_fifo: &[u8] = b"fifo\0";

static policies: [policy; 6] = [
    policy { name: policy_other.as_ptr() as *const c_char, policy: SCHED_OTHER, default_pri: DEFAULT_PRI },
    policy { name: policy_batch.as_ptr() as *const c_char, policy: SCHED_BATCH, default_pri: DEFAULT_PRI },
    policy { name: policy_idle.as_ptr() as *const c_char, policy: SCHED_IDLE, default_pri: DEFAULT_PRI },
    policy { name: policy_rr.as_ptr() as *const c_char, policy: SCHED_RR, default_pri: RT_DEFAULT_PRI },
    policy { name: policy_fifo.as_ptr() as *const c_char, policy: SCHED_FIFO, default_pri: RT_DEFAULT_PRI },
    policy { name: 0 as *const c_char, policy: 0, default_pri: DEFAULT_PRI },
];

static relevant_tracers: [*const c_char; 7] = [
    b"preemptirqsoff\0".as_ptr() as *const c_char,
    b"preemptoff\0".as_ptr() as *const c_char,
    b"irqsoff\0".as_ptr() as *const c_char,
    b"wakeup\0".as_ptr() as *const c_char,
    b"wakeup_rt\0".as_ptr() as *const c_char,
    b"wakeup_dl\0".as_ptr() as *const c_char,
    0 as *const c_char,
];

static random_tracers: [*const c_char; 4] = [
    b"preemptirqsoff\0".as_ptr() as *const c_char,
    b"preemptoff\0".as_ptr() as *const c_char,
    b"irqsoff\0".as_ptr() as *const c_char,
    0 as *const c_char,
];

static mut current_tracer: *const c_char = 0 as *const c_char;
static mut force_tracer: bool = false;

#[repr(C)]
pub struct ftrace_state {
    tracer: *mut c_char,
    thresh: *mut c_char,
    opt: [bool; OPTIDX_NR],
    opt_valid: [bool; OPTIDX_NR],
    mutex: pthread_mutex_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct entry {
    ticket: c_int,
    ticket_completed_ref: c_int,
}

#[repr(C)]
struct print_state {
    ticket_counter: c_int,
    ticket_completed: c_int,
    mutex: pthread_mutex_t,
    cond: pthread_cond_t,
    cnt: c_int,
    cnt_mutex: pthread_mutex_t,
}

#[repr(C)]
struct short_msg {
    buf: [c_char; 160],
    len: c_int,
}

static mut printstate: print_state = unsafe { std::mem::zeroed() };
static mut save_state: ftrace_state = unsafe { std::mem::zeroed() };
static mut signal_flag: sig_atomic_t = 0;

const PROB_TABLE_MAX_SIZE: usize = 1000;
static mut probabilities: [c_int; PROB_TABLE_MAX_SIZE] = [0; PROB_TABLE_MAX_SIZE];

#[repr(C)]
struct sleep_table {
    table: *mut c_int,
    size: c_int,
    mutex: pthread_mutex_t,
}

static mut sleeptable: sleep_table = unsafe { std::mem::zeroed() };

const QUEUE_SIZE: usize = 10;

#[repr(C)]
struct queue {
    entries: [entry; QUEUE_SIZE],
    next_prod_idx: c_int,
    next_cons_idx: c_int,
    mutex: pthread_mutex_t,
    cond: pthread_cond_t,
}

const MAX_THREADS: usize = 40;
static mut printqueue: queue = unsafe { std::mem::zeroed() };
static mut printthread: [pthread_t; MAX_THREADS] = unsafe { std::mem::zeroed() };
static mut print_mtx: pthread_mutex_t = unsafe { std::mem::zeroed() };
const PRINT_BUFFER_SIZE: usize = 16 * 1024 * 1024;

extern "C" {
    fn warn(fmt: *const c_char, ...);
    fn warnx(fmt: *const c_char, ...);
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn tracefs_instance_file_read(instance: *mut c_void, file: *const c_char, psize: *mut c_int) -> *mut c_char;
    fn tracefs_instance_file_write(instance: *mut c_void, file: *const c_char, val: *const c_char) -> c_int;
    fn tracefs_tracers(instance: *mut c_void) -> *mut *mut c_char;
    fn tracefs_list_free(list: *mut *mut c_char);
    fn tracefs_get_tracing_file(name: *const c_char) -> *mut c_char;
    fn inotify_init() -> c_int;
    fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: uint32_t) -> c_int;
    fn lrand48_r(buffer: *mut drand48_data, result: *mut c_long) -> c_int;
    fn srand48_r(seedval: c_long, buffer: *mut drand48_data) -> c_int;
    static mut optarg: *mut c_char;
}

#[repr(C)]
struct inotify_event {
    wd: c_int,
    mask: uint32_t,
    cookie: uint32_t,
    len: uint32_t,
}

const IN_MODIFY: uint32_t = 0x00000002;
const __NR_gettid: c_long = 186;

unsafe fn malloc_or_die(size: size_t) -> *mut c_void {
    let ptr = malloc(size);
    if ptr.is_null() {
        warn(b"malloc() failed\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
    ptr
}

unsafe fn malloc_or_die_nocleanup(size: size_t) -> *mut c_void {
    let ptr = malloc(size);
    if ptr.is_null() {
        err(0, b"malloc() failed\0".as_ptr() as *const c_char);
    }
    ptr
}

unsafe fn write_or_die(mut fd: c_int, mut buf: *const c_char, mut count: size_t) {
    let mut r: ssize_t;
    loop {
        r = write(fd, buf as *const c_void, count);
        if r < 0 {
            if *__errno_location() == EINTR { continue; }
            warn(b"write() failed\0".as_ptr() as *const c_char);
            cleanup_exit(EXIT_FAILURE);
        }
        count -= r as size_t;
        buf = buf.add(r as usize);
        if count == 0 { break; }
    }
}

unsafe fn clock_gettime_or_die(clk_id: clockid_t, tp: *mut timespec) {
    if clock_gettime(clk_id, tp) != 0 {
        err(EXIT_FAILURE, b"clock_gettime() failed\0".as_ptr() as *const c_char);
    }
}

unsafe fn sigemptyset_or_die(s: *mut sigset_t) {
    if sigemptyset(s) != 0 {
        warn(b"sigemptyset() failed\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
}

unsafe fn sigaddset_or_die(s: *mut sigset_t, signum: c_int) {
    if sigaddset(s, signum) != 0 {
        warn(b"sigemptyset() failed\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
}

unsafe fn sigaction_or_die(signum: c_int, act: *const sigaction, oldact: *mut sigaction) {
    if sigaction(signum, act, oldact) != 0 {
        warn(b"sigaction() failed\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
}

unsafe fn open_stdout() {
    if setvbuf(stdout, 0 as *mut c_char, _IONBF, 0) != 0 {
        err(EXIT_FAILURE, b"setvbuf() failed\0".as_ptr() as *const c_char);
    }
    fd_stdout = fileno(stdout);
    if fd_stdout < 0 {
        err(EXIT_FAILURE, b"fileno() failed\0".as_ptr() as *const c_char);
    }
}

unsafe fn mutex_lock(mtx: *mut pthread_mutex_t) {
    *__errno_location() = pthread_mutex_lock(mtx);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_mutex_lock() failed\0".as_ptr() as *const c_char); }
}
unsafe fn mutex_unlock(mtx: *mut pthread_mutex_t) {
    *__errno_location() = pthread_mutex_unlock(mtx);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_mutex_unlock() failed\0".as_ptr() as *const c_char); }
}
unsafe fn cond_signal(cond: *mut pthread_cond_t) {
    *__errno_location() = pthread_cond_signal(cond);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_cond_signal() failed\0".as_ptr() as *const c_char); }
}
unsafe fn cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) {
    *__errno_location() = pthread_cond_wait(cond, mutex);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_cond_wait() failed\0".as_ptr() as *const c_char); }
}
unsafe fn cond_broadcast(cond: *mut pthread_cond_t) {
    *__errno_location() = pthread_cond_broadcast(cond);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_cond_broadcast() failed\0".as_ptr() as *const c_char); }
}
unsafe fn mutex_init(mutex: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t) {
    *__errno_location() = pthread_mutex_init(mutex, attr);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_mutex_init() failed\0".as_ptr() as *const c_char); }
}
unsafe fn mutexattr_init(attr: *mut pthread_mutexattr_t) {
    *__errno_location() = pthread_mutexattr_init(attr);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_mutexattr_init() failed\0".as_ptr() as *const c_char); }
}
unsafe fn mutexattr_destroy(attr: *mut pthread_mutexattr_t) {
    *__errno_location() = pthread_mutexattr_destroy(attr);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_mutexattr_destroy() failed\0".as_ptr() as *const c_char); }
}
unsafe fn mutexattr_settype(attr: *mut pthread_mutexattr_t, typ: c_int) {
    *__errno_location() = pthread_mutexattr_settype(attr, typ);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_mutexattr_settype() failed\0".as_ptr() as *const c_char); }
}
unsafe fn condattr_init(attr: *mut pthread_condattr_t) {
    *__errno_location() = pthread_condattr_init(attr);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_condattr_init() failed\0".as_ptr() as *const c_char); }
}
unsafe fn condattr_destroy(attr: *mut pthread_condattr_t) {
    *__errno_location() = pthread_condattr_destroy(attr);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_condattr_destroy() failed\0".as_ptr() as *const c_char); }
}
unsafe fn condattr_setclock(attr: *mut pthread_condattr_t, clock_id: clockid_t) {
    *__errno_location() = pthread_condattr_setclock(attr, clock_id);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_condattr_setclock() failed\0".as_ptr() as *const c_char); }
}
unsafe fn cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) {
    *__errno_location() = pthread_cond_init(cond, attr);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_cond_init() failed\0".as_ptr() as *const c_char); }
}
unsafe fn cond_timedwait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t, abstime: *const timespec) -> c_int {
    *__errno_location() = pthread_cond_timedwait(cond, mutex, abstime);
    if *__errno_location() != 0 && *__errno_location() != ETIMEDOUT {
        err(EXIT_FAILURE, b"pthread_cond_timedwait() failed\0".as_ptr() as *const c_char);
    }
    *__errno_location()
}

unsafe fn init_printstate() {
    let mut cattr: pthread_condattr_t = std::mem::zeroed();
    printstate.ticket_counter = 0;
    printstate.ticket_completed = 0;
    printstate.cnt = 0;
    mutex_init(&mut printstate.mutex, 0 as *const pthread_mutexattr_t);
    condattr_init(&mut cattr);
    condattr_setclock(&mut cattr, CLOCK_MONOTONIC);
    cond_init(&mut printstate.cond, &cattr);
    condattr_destroy(&mut cattr);
}

unsafe fn init_print_mtx() {
    let mut mattr: pthread_mutexattr_t = std::mem::zeroed();
    mutexattr_init(&mut mattr);
    mutexattr_settype(&mut mattr, PTHREAD_MUTEX_RECURSIVE);
    mutex_init(&mut print_mtx, &mattr);
    mutexattr_destroy(&mut mattr);
}

unsafe fn signal_blocking(how: c_int) {
    let mut s: sigset_t = std::mem::zeroed();
    sigemptyset_or_die(&mut s);
    sigaddset_or_die(&mut s, SIGHUP);
    sigaddset_or_die(&mut s, SIGTERM);
    sigaddset_or_die(&mut s, SIGINT);
    *__errno_location() = pthread_sigmask(how, &s, 0 as *mut sigset_t);
    if *__errno_location() != 0 {
        warn(b"pthread_sigmask() failed\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
}

extern "C" fn signal_handler(num: c_int) {
    unsafe { signal_flag = num; }
}

unsafe fn setup_sig_handler() {
    let mut sa: sigaction = std::mem::zeroed();
    sa.sa_sigaction = signal_handler as usize;
    sigaction_or_die(SIGHUP, &sa, 0 as *mut sigaction);
    sigaction_or_die(SIGTERM, &sa, 0 as *mut sigaction);
    sigaction_or_die(SIGINT, &sa, 0 as *mut sigaction);
}

unsafe fn process_signal(signal: c_int) {
    let name = strsignal(signal);
    if name.is_null() {
        printf(b"Received signal %d\n\0".as_ptr() as *const c_char, signal);
    } else {
        printf(b"Received signal %d (%s)\n\0".as_ptr() as *const c_char, signal, name);
    }
    cleanup_exit(EXIT_SUCCESS);
}

unsafe fn check_signals() {
    let signal = signal_flag;
    if signal != 0 { process_signal(signal); }
}

unsafe fn get_time_in_future(future: *mut timespec, time_us: c_long) {
    let mut nsec: c_long;
    clock_gettime_or_die(CLOCK_MONOTONIC, future);
    (*future).tv_sec += time_us / USEC_PER_SEC;
    nsec = (*future).tv_nsec + (time_us * NSEC_PER_USEC) % NSEC_PER_SEC;
    if nsec >= NSEC_PER_SEC {
        (*future).tv_nsec = nsec % NSEC_PER_SEC;
        (*future).tv_sec += 1;
    }
}

unsafe fn time_has_passed(time: *const timespec) -> bool {
    let mut now: timespec = std::mem::zeroed();
    clock_gettime_or_die(CLOCK_MONOTONIC, &mut now);
    if now.tv_sec > (*time).tv_sec { return true; }
    if now.tv_sec < (*time).tv_sec { return false; }
    now.tv_nsec >= (*time).tv_nsec
}

unsafe fn mutex_trylock_limit(mutex: *mut pthread_mutex_t, time_ms: c_int) -> bool {
    let time_us = time_ms as c_long * USEC_PER_MSEC;
    let mut limit: timespec = std::mem::zeroed();
    get_time_in_future(&mut limit, time_us);
    loop {
        *__errno_location() = pthread_mutex_trylock(mutex);
        if *__errno_location() != 0 && *__errno_location() != EBUSY {
            err(EXIT_FAILURE, b"pthread_mutex_trylock() failed\0".as_ptr() as *const c_char);
        }
        if *__errno_location() == 0 || time_has_passed(&limit) { break; }
    }
    *__errno_location() == 0
}

unsafe fn restore_trace_opts(state: *const ftrace_state, cur: *const bool) {
    for i in 0..OPTIDX_NR {
        if (*state).opt_valid[i] && (*state).opt[i] != *cur.add(i) {
            let r = set_trace_opt(optstr[i], (*state).opt[i]);
            if r < 0 {
                warnx(b"Failed to restore the %s option to %s\0".as_ptr() as *const c_char, optstr[i], bool2str((*state).opt[i]));
            } else if verbose_ftrace() {
                printf(b"Restored the %s option in %s to %s\n\0".as_ptr() as *const c_char, optstr[i], TR_OPTIONS.as_ptr(), bool2str((*state).opt[i]));
            }
        }
    }
}

unsafe fn read_file(file: *const c_char, h: errhandling) -> *mut c_char {
    let mut psize: c_int = 0;
    let emsg = b"Failed to read the %s file\0";
    let r = tracefs_instance_file_read(0 as *mut c_void, file, &mut psize);
    if r.is_null() {
        if h as c_int != 0 {
            warn(emsg.as_ptr() as *const c_char, file);
            if h as c_int == errhandling::ERR_CLEANUP as c_int { cleanup_exit(EXIT_FAILURE); }
        } else {
            errx(EXIT_FAILURE, emsg.as_ptr() as *const c_char, file);
        }
    }
    if !r.is_null() && *r.add((psize - 1) as usize) == b'\n' as c_char {
        *r.add((psize - 1) as usize) = 0;
    }
    r
}

unsafe fn restore_file(file: *const c_char, saved: *mut *mut c_char, cur: *const c_char) {
    if !(*saved).is_null() && was_changed(*saved, cur) {
        if tracefs_instance_file_write(0 as *mut c_void, file, *saved) < 0 {
            warnx(b"Failed to restore %s to %s!\0".as_ptr() as *const c_char, file, *saved);
        } else if verbose_ftrace() {
            printf(b"Restored %s to %s\n\0".as_ptr() as *const c_char, file, *saved);
        }
        free(*saved as *mut c_void);
        *saved = 0 as *mut c_char;
    }
}

unsafe fn restore_ftrace() {
    mutex_lock(&mut save_state.mutex);
    restore_file(TR_CURRENT.as_ptr() as *const c_char, &mut save_state.tracer, current_tracer);
    restore_file(TR_THRESH.as_ptr() as *const c_char, &mut save_state.thresh, threshold);
    restore_trace_opts(&save_state, use_options.as_ptr());
    mutex_unlock(&mut save_state.mutex);
}

unsafe fn cleanup_exit(status: c_int) -> ! {
    if !setup_ftrace { exit(status); }
    mutex_trylock_limit(&mut print_mtx, TRY_PRINTMUTEX_MS);
    let maxlat = read_file(TR_MAXLAT.as_ptr() as *const c_char, errhandling::ERR_WARN);
    if !maxlat.is_null() {
        printf(b"The maximum detected latency was: %sus\n\0".as_ptr() as *const c_char, maxlat);
        free(maxlat as *mut c_void);
    }
    restore_ftrace();
    exit(status);
}

unsafe fn init_save_state() {
    let mut mattr: pthread_mutexattr_t = std::mem::zeroed();
    mutexattr_init(&mut mattr);
    mutexattr_settype(&mut mattr, PTHREAD_MUTEX_RECURSIVE);
    mutex_init(&mut save_state.mutex, &mattr);
    mutexattr_destroy(&mut mattr);
    save_state.tracer = 0 as *mut c_char;
    save_state.thresh = 0 as *mut c_char;
    save_state.opt_valid[OPTIDX_FUNC_TR] = false;
    save_state.opt_valid[OPTIDX_DISP_GR] = false;
}

unsafe fn printstate_next_ticket(req: *mut entry) -> c_int {
    printstate.ticket_counter += 1;
    let r = printstate.ticket_counter;
    (*req).ticket = r;
    (*req).ticket_completed_ref = printstate.ticket_completed;
    cond_broadcast(&mut printstate.cond);
    r
}

unsafe fn printstate_mark_req_completed(req: *const entry) {
    if (*req).ticket > printstate.ticket_completed { printstate.ticket_completed = (*req).ticket; }
}
unsafe fn printstate_has_new_req_arrived(req: *const entry) -> bool { printstate.ticket_counter != (*req).ticket }
unsafe fn printstate_cnt_inc() -> c_int {
    mutex_lock(&mut printstate.cnt_mutex);
    printstate.cnt += 1;
    let value = printstate.cnt;
    mutex_unlock(&mut printstate.cnt_mutex);
    value
}
unsafe fn printstate_cnt_dec() -> c_int {
    mutex_lock(&mut printstate.cnt_mutex);
    printstate.cnt -= 1;
    let value = printstate.cnt;
    mutex_unlock(&mut printstate.cnt_mutex);
    value
}
unsafe fn printstate_cnt_read() -> c_int {
    mutex_lock(&mut printstate.cnt_mutex);
    let value = printstate.cnt;
    mutex_unlock(&mut printstate.cnt_mutex);
    value
}
unsafe fn prev_req_won_race(req: *const entry) -> bool { printstate.ticket_completed != (*req).ticket_completed_ref }

unsafe fn sleeptable_resize(size: c_int, printout: bool, msg: *mut short_msg) {
    let mut bytes: c_int;
    if printout {
        (*msg).len = 0;
        if size > PROB_TABLE_MAX_SIZE as c_int {
            bytes = snprintf((*msg).buf.as_mut_ptr(), (*msg).buf.len(), b"Cannot increase probability table to %d (maximum size reached)\n\0".as_ptr() as *const c_char, size);
        } else {
            bytes = snprintf((*msg).buf.as_mut_ptr(), (*msg).buf.len(), b"Increasing probability table to %d\n\0".as_ptr() as *const c_char, size);
        }
        if bytes < 0 { warn(b"snprintf() failed\0".as_ptr() as *const c_char); } else { (*msg).len = bytes; }
    }
    if size < 0 {
        warnx(b"Bad program state at %s:%d\0".as_ptr() as *const c_char, file!().as_ptr(), line!());
        cleanup_exit(EXIT_FAILURE);
    }
    sleeptable.size = size;
    sleeptable.table = probabilities.as_mut_ptr().add(PROB_TABLE_MAX_SIZE - size as usize);
}

unsafe fn init_probabilities() {
    let mut j = 1000;
    for i in 0..PROB_TABLE_MAX_SIZE {
        probabilities[i] = 1000 / j;
        j -= 1;
    }
    mutex_init(&mut sleeptable.mutex, 0 as *const pthread_mutexattr_t);
}

unsafe fn table_get_probability(req: *const entry, msg: *mut short_msg) -> c_int {
    let mut diff = (*req).ticket - (*req).ticket_completed_ref;
    let mut rval = 0;
    (*msg).len = 0;
    diff -= 1;
    if diff < 0 {
        warnx(b"Programmer assumption error at %s:%d\n\0".as_ptr() as *const c_char, file!().as_ptr(), line!());
        cleanup_exit(EXIT_FAILURE);
    }
    mutex_lock(&mut sleeptable.mutex);
    if diff >= sleeptable.size - 1 {
        rval = *sleeptable.table.add((sleeptable.size - 1) as usize);
        sleeptable_resize(sleeptable.size + 1, verbose_sizechange(), msg);
    } else {
        rval = *sleeptable.table.add(diff as usize);
    }
    mutex_unlock(&mut sleeptable.mutex);
    rval
}

unsafe fn init_queue(q: *mut queue) {
    (*q).next_prod_idx = 0;
    (*q).next_cons_idx = 0;
    mutex_init(&mut (*q).mutex, 0 as *const pthread_mutexattr_t);
    *__errno_location() = pthread_cond_init(&mut (*q).cond, 0 as *const pthread_condattr_t);
    if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_cond_init() failed\0".as_ptr() as *const c_char); }
}
unsafe fn queue_len(q: *const queue) -> c_int {
    if (*q).next_prod_idx >= (*q).next_cons_idx {
        (*q).next_prod_idx - (*q).next_cons_idx
    } else {
        QUEUE_SIZE as c_int - (*q).next_cons_idx + (*q).next_prod_idx
    }
}
unsafe fn queue_nr_free(q: *const queue) -> c_int {
    let mut nr_free = QUEUE_SIZE as c_int - queue_len(q);
    if nr_free == 1 { nr_free = 0; }
    nr_free
}
unsafe fn queue_idx_inc(idx: *mut c_int) { *idx = (*idx + 1) % QUEUE_SIZE as c_int; }
unsafe fn queue_push_to_back(q: *mut queue, e: *const entry) {
    (*q).entries[(*q).next_prod_idx as usize] = *e;
    queue_idx_inc(&mut (*q).next_prod_idx);
}
unsafe fn queue_pop_from_front(q: *mut queue) -> entry {
    let e = (*q).entries[(*q).next_cons_idx as usize];
    queue_idx_inc(&mut (*q).next_cons_idx);
    e
}
unsafe fn queue_cond_signal(q: *mut queue) { cond_signal(&mut (*q).cond); }
unsafe fn queue_cond_wait(q: *mut queue) { cond_wait(&mut (*q).cond, &mut (*q).mutex); }
unsafe fn queue_try_to_add_entry(q: *mut queue, e: *const entry) -> c_int {
    let mut r = 0;
    mutex_lock(&mut (*q).mutex);
    if queue_nr_free(q) > 0 {
        queue_push_to_back(q, e);
        cond_signal(&mut (*q).cond);
    } else {
        r = -1;
    }
    mutex_unlock(&mut (*q).mutex);
    r
}
unsafe fn queue_wait_for_entry(q: *mut queue) -> entry {
    let mut e: entry = std::mem::zeroed();
    mutex_lock(&mut (*q).mutex);
    loop {
        if queue_len(&printqueue) > 0 {
            e = queue_pop_from_front(q);
            break;
        }
        queue_cond_wait(q);
    }
    mutex_unlock(&mut (*q).mutex);
    e
}

unsafe fn policy_from_name(name: *const c_char) -> *const policy {
    let mut p = policies.as_ptr();
    while !(*p).name.is_null() {
        if strcmp(name, (*p).name) == 0 { return p; }
        p = p.add(1);
    }
    0 as *const policy
}
unsafe fn policy_name(policy_id: c_int) -> *const c_char {
    let mut p = policies.as_ptr();
    while !(*p).name.is_null() {
        if (*p).policy == policy_id { return (*p).name; }
        p = p.add(1);
    }
    b"unknown\0".as_ptr() as *const c_char
}
unsafe fn is_relevant_tracer(name: *const c_char) -> bool {
    let mut i = 0;
    while !relevant_tracers[i].is_null() {
        if strcmp(name, relevant_tracers[i]) == 0 { return true; }
        i += 1;
    }
    false
}
unsafe fn random_makes_sense(name: *const c_char) -> bool {
    let mut i = 0;
    while !random_tracers[i].is_null() {
        if strcmp(name, random_tracers[i]) == 0 { return true; }
        i += 1;
    }
    false
}

unsafe fn show_available() {
    let tracers = tracefs_tracers(0 as *mut c_void);
    let mut found = 0;
    let mut i = 0;
    while !tracers.is_null() && !(*tracers.add(i)).is_null() {
        if is_relevant_tracer(*tracers.add(i)) { found += 1; }
        i += 1;
    }
    if tracers.is_null() {
        warnx(b"%s\0".as_ptr() as *const c_char, no_tracer_msg.as_ptr());
        return;
    }
    if found == 0 {
        warnx(b"%s\0".as_ptr() as *const c_char, no_latency_tr_msg.as_ptr());
        tracefs_list_free(tracers);
        return;
    }
    printf(b"The following latency tracers are available on your system:\n\0".as_ptr() as *const c_char);
    i = 0;
    while !(*tracers.add(i)).is_null() {
        if is_relevant_tracer(*tracers.add(i)) {
            printf(b"%s\n\0".as_ptr() as *const c_char, *tracers.add(i));
        }
        i += 1;
    }
    tracefs_list_free(tracers);
}

unsafe fn tracer_valid(name: *const c_char, notracer: *mut bool) -> bool {
    *notracer = false;
    let tracers = tracefs_tracers(0 as *mut c_void);
    let mut rval = false;
    if tracers.is_null() {
        *notracer = true;
        return false;
    }
    let mut i = 0;
    while !(*tracers.add(i)).is_null() {
        if strcmp(*tracers.add(i), name) == 0 {
            rval = true;
            break;
        }
        i += 1;
    }
    tracefs_list_free(tracers);
    rval
}

unsafe fn find_default_tracer() -> *const c_char {
    let mut i = 0;
    while !relevant_tracers[i].is_null() {
        let mut notracer = false;
        let valid = tracer_valid(relevant_tracers[i], &mut notracer);
        if notracer { errx(EXIT_FAILURE, b"%s\0".as_ptr() as *const c_char, no_tracer_msg.as_ptr()); }
        if valid { return relevant_tracers[i]; }
        i += 1;
    }
    0 as *const c_char
}

unsafe fn toss_coin(buffer: *mut drand48_data, prob: c_uint) -> bool {
    let mut r: c_long = 0;
    if lrand48_r(buffer, &mut r) != 0 {
        warnx(b"lrand48_r() failed\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
    r %= 1000;
    r < prob as c_long
}

unsafe fn go_to_sleep(req: *const entry) -> c_long {
    let mut future: timespec = std::mem::zeroed();
    let mut delay = sleep_time;
    get_time_in_future(&mut future, delay);
    mutex_lock(&mut printstate.mutex);
    while !printstate_has_new_req_arrived(req) {
        cond_timedwait(&mut printstate.cond, &mut printstate.mutex, &future);
        if time_has_passed(&future) { break; }
    }
    if printstate_has_new_req_arrived(req) { delay = -1; }
    mutex_unlock(&mut printstate.mutex);
    delay
}

unsafe fn set_priority() {
    let mut param: sched_param = std::mem::zeroed();
    param.sched_priority = sched_pri;
    let pid = getpid();
    if sched_setscheduler(pid, sched_policy, &param) != 0 {
        err(EXIT_FAILURE, b"sched_setscheduler() failed\0".as_ptr() as *const c_char);
    }
}

#[no_mangle]
pub unsafe extern "C" fn latency_collector_gettid() -> pid_t {
    syscall(__NR_gettid) as pid_t
}

unsafe fn print_priority() {
    let tid = latency_collector_gettid();
    let mut policy: c_int = 0;
    let mut param: sched_param = std::mem::zeroed();
    let r = pthread_getschedparam(pthread_self(), &mut policy, &mut param);
    if r != 0 {
        warn(b"pthread_getschedparam() failed\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
    mutex_lock(&mut print_mtx);
    printf(b"Thread %d runs with scheduling policy %s and priority %d\n\0".as_ptr() as *const c_char, tid, policy_name(policy), param.sched_priority);
    mutex_unlock(&mut print_mtx);
}

unsafe fn __print_skipmessage(resize_msg: *const short_msg, timestamp: *const timespec, buffer: *mut c_char, mut bufspace: size_t, req: *const entry, excuse: bool, str_: *const c_char) {
    let mut bytes: ssize_t = 0;
    let mut p = buffer;
    let sec = (*timestamp).tv_sec;
    let us = (*timestamp).tv_nsec / 1000;
    let r: c_int;
    if !resize_msg.is_null() && (*resize_msg).len > 0 {
        strncpy(p, (*resize_msg).buf.as_ptr(), (*resize_msg).len as size_t);
        bytes += (*resize_msg).len as ssize_t;
        p = p.add((*resize_msg).len as usize);
        bufspace -= (*resize_msg).len as size_t;
    }
    if excuse {
        r = snprintf(p, bufspace, b"%ld.%06ld Latency %d printout skipped due to %s\n\0".as_ptr() as *const c_char, sec, us, (*req).ticket, str_);
    } else {
        r = snprintf(p, bufspace, b"%ld.%06ld Latency %d detected\n\0".as_ptr() as *const c_char, sec, us, (*req).ticket);
    }
    if r < 0 { warn(b"snprintf() failed\0".as_ptr() as *const c_char); } else { bytes += r as ssize_t; }
    mutex_lock(&mut print_mtx);
    write_or_die(fd_stdout, buffer, bytes as size_t);
    mutex_unlock(&mut print_mtx);
}

unsafe fn print_skipmessage(resize_msg: *const short_msg, timestamp: *const timespec, buffer: *mut c_char, bufspace: size_t, req: *const entry, excuse: bool) {
    __print_skipmessage(resize_msg, timestamp, buffer, bufspace, req, excuse, b"random delay\0".as_ptr() as *const c_char);
}

unsafe fn print_lostmessage(timestamp: *const timespec, buffer: *mut c_char, bufspace: size_t, req: *const entry, reason: *const c_char) {
    __print_skipmessage(0 as *const short_msg, timestamp, buffer, bufspace, req, true, reason);
}

unsafe fn print_tracefile(resize_msg: *const short_msg, timestamp: *const timespec, buffer: *mut c_char, mut bufspace: size_t, slept: c_long, req: *const entry) {
    const RESERVE: ssize_t = 256;
    let mut p = buffer;
    let mut bytes: ssize_t;
    let mut bytes_tot: ssize_t = 0;
    bufspace = bufspace - RESERVE as size_t - 1;
    if !resize_msg.is_null() && (*resize_msg).len > 0 {
        bytes = (*resize_msg).len as ssize_t;
        strncpy(p, (*resize_msg).buf.as_ptr(), bytes as size_t);
        bytes_tot += bytes;
        p = p.add(bytes as usize);
        bufspace -= bytes as size_t;
    }
    let trace_fd = open(debug_tracefile, O_RDONLY);
    if trace_fd < 0 {
        warn(b"open() failed on %s\0".as_ptr() as *const c_char, debug_tracefile);
        return;
    }
    let sec = (*timestamp).tv_sec;
    let us = (*timestamp).tv_nsec / 1000;
    if slept != 0 {
        let slept_ms = slept / 1000;
        bytes = snprintf(p, bufspace, b"%ld.%06ld Latency %d randomly sleep for %ld ms before print\n\0".as_ptr() as *const c_char, sec, us, (*req).ticket, slept_ms) as ssize_t;
    } else {
        bytes = snprintf(p, bufspace, b"%ld.%06ld Latency %d immediate print\n\0".as_ptr() as *const c_char, sec, us, (*req).ticket) as ssize_t;
    }
    if bytes < 0 { warn(b"snprintf() failed\0".as_ptr() as *const c_char); return; }
    p = p.add(bytes as usize); bufspace -= bytes as size_t; bytes_tot += bytes;
    bytes = snprintf(p, bufspace, b">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> BEGIN <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<\n\0".as_ptr() as *const c_char) as ssize_t;
    if bytes < 0 { warn(b"snprintf() failed\0".as_ptr() as *const c_char); return; }
    p = p.add(bytes as usize); bufspace -= bytes as size_t; bytes_tot += bytes;
    loop {
        bytes = read(trace_fd, p as *mut c_void, bufspace) as ssize_t;
        if bytes < 0 {
            if *__errno_location() == EINTR { continue; }
            warn(b"read() failed on %s\0".as_ptr() as *const c_char, debug_tracefile);
            if close(trace_fd) != 0 { warn(b"close() failed on %s\0".as_ptr() as *const c_char, debug_tracefile); }
            return;
        }
        if bytes == 0 { break; }
        p = p.add(bytes as usize); bufspace -= bytes as size_t; bytes_tot += bytes;
    }
    if close(trace_fd) != 0 { warn(b"close() failed on %s\0".as_ptr() as *const c_char, debug_tracefile); }
    printstate_cnt_dec();
    bufspace += RESERVE as size_t;
    bytes = snprintf(p, bufspace, b">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> END <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<\n\n\0".as_ptr() as *const c_char) as ssize_t;
    if bytes < 0 { warn(b"snprintf() failed\0".as_ptr() as *const c_char); return; }
    bytes_tot += bytes;
    mutex_lock(&mut print_mtx);
    write_or_die(fd_stdout, buffer, bytes_tot as size_t);
    mutex_unlock(&mut print_mtx);
}

unsafe fn get_no_opt(opt: *const c_char) -> *mut c_char {
    let s = strlen(opt) + strlen(OPT_NO_PREFIX.as_ptr() as *const c_char) + 1;
    let no_opt = malloc_or_die_nocleanup(s) as *mut c_char;
    strcpy(no_opt, OPT_NO_PREFIX.as_ptr() as *const c_char);
    strcat(no_opt, opt);
    no_opt
}

unsafe fn find_next_optstr(allopt: *const c_char, next: *mut *const c_char) -> *mut c_char {
    if allopt.is_null() { return 0 as *mut c_char; }
    let mut begin = allopt;
    while *begin != 0 {
        if isgraph(*begin as c_int) != 0 { break; }
        begin = begin.add(1);
    }
    if *begin == 0 { return 0 as *mut c_char; }
    let mut end = begin;
    let mut s = 0usize;
    while *end != 0 && isgraph(*end as c_int) != 0 {
        s += 1;
        end = end.add(1);
    }
    let r = malloc_or_die_nocleanup(s + 1) as *mut c_char;
    strncpy(r, begin, s);
    *r.add(s) = 0;
    *next = begin.add(s);
    r
}

unsafe fn get_trace_opt(allopt: *const c_char, opt: *const c_char, found: *mut bool) -> bool {
    *found = false;
    let no_opt = get_no_opt(opt);
    let mut next = allopt;
    let mut rval = false;
    loop {
        let str_ = find_next_optstr(next, &mut next);
        if str_.is_null() { break; }
        if strcmp(str_, opt) == 0 {
            *found = true;
            rval = true;
            free(str_ as *mut c_void);
            break;
        }
        if strcmp(str_, no_opt) == 0 {
            *found = true;
            rval = false;
            free(str_ as *mut c_void);
            break;
        }
        free(str_ as *mut c_void);
    }
    free(no_opt as *mut c_void);
    rval
}

unsafe fn set_trace_opt(opt: *const c_char, value: bool) -> c_int {
    let str_ = if value { strdup(opt) } else { get_no_opt(opt) };
    let r = tracefs_instance_file_write(0 as *mut c_void, TR_OPTIONS.as_ptr() as *const c_char, str_);
    free(str_ as *mut c_void);
    r
}

#[no_mangle]
pub unsafe extern "C" fn save_trace_opts(state: *mut ftrace_state) {
    let mut psize: c_int = 0;
    let allopt = tracefs_instance_file_read(0 as *mut c_void, TR_OPTIONS.as_ptr() as *const c_char, &mut psize);
    if allopt.is_null() {
        errx(EXIT_FAILURE, b"Failed to read the %s file\n\0".as_ptr() as *const c_char, TR_OPTIONS.as_ptr());
    }
    for i in 0..OPTIDX_NR {
        (*state).opt[i] = get_trace_opt(allopt, optstr[i], &mut (*state).opt_valid[i]);
    }
    free(allopt as *mut c_void);
}

unsafe fn write_file(file: *const c_char, cur: *const c_char, new_: *const c_char, h: errhandling) {
    let emsg = b"Failed to write to the %s file!\0";
    if !cur.is_null() && !needs_change(cur, new_) { return; }
    let r = tracefs_instance_file_write(0 as *mut c_void, file, new_);
    if r < 0 {
        if h as c_int != 0 {
            warnx(emsg.as_ptr() as *const c_char, file);
            if h as c_int == errhandling::ERR_CLEANUP as c_int { cleanup_exit(EXIT_FAILURE); }
        } else {
            errx(EXIT_FAILURE, emsg.as_ptr() as *const c_char, file);
        }
    }
    if verbose_ftrace() {
        mutex_lock(&mut print_mtx);
        printf(b"%s was set to %s\n\0".as_ptr() as *const c_char, file, new_);
        mutex_unlock(&mut print_mtx);
    }
}

unsafe fn reset_max_latency() {
    write_file(TR_MAXLAT.as_ptr() as *const c_char, 0 as *const c_char, b"0\0".as_ptr() as *const c_char, errhandling::ERR_CLEANUP);
}

unsafe fn save_and_disable_tracer() {
    let mut need_nop = false;
    mutex_lock(&mut save_state.mutex);
    save_trace_opts(&mut save_state);
    let tracer = read_file(TR_CURRENT.as_ptr() as *const c_char, errhandling::ERR_EXIT);
    let orig_th = read_file(TR_THRESH.as_ptr() as *const c_char, errhandling::ERR_EXIT);
    if needs_change(tracer, NOP_TRACER.as_ptr() as *const c_char) {
        mutex_lock(&mut print_mtx);
        if force_tracer {
            printf(b"The %s tracer is already in use but proceeding anyway!\n\0".as_ptr() as *const c_char, tracer);
        } else {
            printf(b"The %s tracer is already in use, cowardly bailing out!\nThis could indicate that another program or instance is tracing.\nUse the -F [--force] option to disregard the current tracer.\n\0".as_ptr() as *const c_char, tracer);
            exit(0);
        }
        mutex_unlock(&mut print_mtx);
        need_nop = true;
    }
    save_state.tracer = tracer;
    save_state.thresh = orig_th;
    if need_nop {
        write_file(TR_CURRENT.as_ptr() as *const c_char, 0 as *const c_char, NOP_TRACER.as_ptr() as *const c_char, errhandling::ERR_EXIT);
    }
    mutex_unlock(&mut save_state.mutex);
}

#[no_mangle]
pub unsafe extern "C" fn set_trace_opts(state: *mut ftrace_state, new_: *mut bool) {
    for i in 0..OPTIDX_NR {
        if (*state).opt_valid[i] && (*state).opt[i] != *new_.add(i) {
            let r = set_trace_opt(optstr[i], *new_.add(i));
            if r < 0 {
                warnx(b"Failed to set the %s option to %s\0".as_ptr() as *const c_char, optstr[i], bool2str(*new_.add(i)));
                cleanup_exit(EXIT_FAILURE);
            }
            if verbose_ftrace() {
                mutex_lock(&mut print_mtx);
                printf(b"%s in %s was set to %s\n\0".as_ptr() as *const c_char, optstr[i], TR_OPTIONS.as_ptr(), bool2str(*new_.add(i)));
                mutex_unlock(&mut print_mtx);
            }
        }
    }
}

unsafe fn enable_tracer() {
    mutex_lock(&mut save_state.mutex);
    set_trace_opts(&mut save_state, use_options.as_mut_ptr());
    write_file(TR_THRESH.as_ptr() as *const c_char, save_state.thresh, threshold, errhandling::ERR_CLEANUP);
    write_file(TR_CURRENT.as_ptr() as *const c_char, NOP_TRACER.as_ptr() as *const c_char, current_tracer, errhandling::ERR_CLEANUP);
    mutex_unlock(&mut save_state.mutex);
}

unsafe fn tracing_loop() {
    let ifd = inotify_init();
    let bufsize = inotify_buffer.len() as ssize_t;
    let istructsize = std::mem::size_of::<inotify_event>() as ssize_t;
    let buf = inotify_buffer.as_mut_ptr();
    let mut req: entry = std::mem::zeroed();
    let buffer = malloc_or_die(PRINT_BUFFER_SIZE) as *mut c_char;
    let bufspace = PRINT_BUFFER_SIZE;
    let mut timestamp: timespec = std::mem::zeroed();
    print_priority();
    if ifd < 0 { err(EXIT_FAILURE, b"inotify_init() failed!\0".as_ptr() as *const c_char); }
    if setup_ftrace {
        save_and_disable_tracer();
        reset_max_latency();
    }
    let wd = inotify_add_watch(ifd, debug_maxlat, IN_MODIFY);
    if wd < 0 { err(EXIT_FAILURE, b"inotify_add_watch() failed!\0".as_ptr() as *const c_char); }
    if setup_ftrace { enable_tracer(); }
    signal_blocking(SIG_UNBLOCK);
    loop {
        let mut modified = 0;
        check_signals();
        let nr_read = read(ifd, buf as *mut c_void, bufsize as size_t);
        check_signals();
        if nr_read < 0 {
            if *__errno_location() == EINTR { continue; }
            warn(b"read() failed on inotify fd!\0".as_ptr() as *const c_char);
            cleanup_exit(EXIT_FAILURE);
        }
        if nr_read == bufsize { warnx(b"inotify() buffer filled, skipping events\0".as_ptr() as *const c_char); }
        if nr_read < istructsize {
            warnx(b"read() returned too few bytes on inotify fd\0".as_ptr() as *const c_char);
            cleanup_exit(EXIT_FAILURE);
        }
        let mut p = buf;
        while p < buf.add(nr_read as usize) {
            let event = p as *mut inotify_event;
            if ((*event).mask & IN_MODIFY) != 0 { modified += 1; }
            p = p.add(istructsize as usize + (*event).len as usize);
        }
        while modified > 0 {
            check_signals();
            mutex_lock(&mut printstate.mutex);
            check_signals();
            printstate_next_ticket(&mut req);
            if printstate_cnt_read() > 0 {
                printstate_mark_req_completed(&req);
                mutex_unlock(&mut printstate.mutex);
                if verbose_lostevent() {
                    clock_gettime_or_die(CLOCK_MONOTONIC, &mut timestamp);
                    print_lostmessage(&timestamp, buffer, bufspace, &req, b"inotify loop\0".as_ptr() as *const c_char);
                }
                break;
            }
            mutex_unlock(&mut printstate.mutex);
            if queue_try_to_add_entry(&mut printqueue, &req) != 0 {
                check_signals();
                mutex_lock(&mut print_mtx);
                check_signals();
                write_or_die(fd_stdout, queue_full_warning.as_ptr() as *const c_char, strlen(queue_full_warning.as_ptr() as *const c_char));
                mutex_unlock(&mut print_mtx);
            }
            modified -= 1;
        }
    }
}

unsafe extern "C" fn do_printloop(arg: *mut c_void) -> *mut c_void {
    let bufspace = PRINT_BUFFER_SIZE;
    let buffer = malloc_or_die(bufspace) as *mut c_char;
    let rseed = arg as *mut c_long;
    let mut drandbuf: drand48_data = std::mem::zeroed();
    let mut slept: c_long = 0;
    let mut timestamp: timespec = std::mem::zeroed();
    let mut resize_msg: short_msg = std::mem::zeroed();
    print_priority();
    if srand48_r(*rseed, &mut drandbuf) != 0 {
        warn(b"srand48_r() failed!\n\0".as_ptr() as *const c_char);
        cleanup_exit(EXIT_FAILURE);
    }
    loop {
        let req = queue_wait_for_entry(&mut printqueue);
        clock_gettime_or_die(CLOCK_MONOTONIC, &mut timestamp);
        mutex_lock(&mut printstate.mutex);
        if prev_req_won_race(&req) {
            printstate_mark_req_completed(&req);
            mutex_unlock(&mut printstate.mutex);
            if verbose_lostevent() {
                print_lostmessage(&timestamp, buffer, bufspace, &req, b"print loop\0".as_ptr() as *const c_char);
            }
            continue;
        }
        mutex_unlock(&mut printstate.mutex);
        if trace_enable && use_random_sleep {
            slept = 0;
            let prob = table_get_probability(&req, &mut resize_msg);
            if !toss_coin(&mut drandbuf, prob as c_uint) {
                slept = go_to_sleep(&req);
            }
            if slept >= 0 {
                printstate_cnt_inc();
                mutex_lock(&mut printstate.mutex);
                printstate_mark_req_completed(&req);
                mutex_unlock(&mut printstate.mutex);
            }
        }
        if trace_enable {
            if slept >= 0 {
                print_tracefile(&resize_msg, &timestamp, buffer, bufspace, slept, &req);
            } else {
                print_skipmessage(&resize_msg, &timestamp, buffer, bufspace, &req, true);
            }
        } else {
            print_skipmessage(&resize_msg, &timestamp, buffer, bufspace, &req, false);
        }
    }
}

unsafe fn start_printthread() {
    let ufd = open(DEV_URANDOM.as_ptr() as *const c_char, O_RDONLY);
    if nr_threads > MAX_THREADS as c_uint {
        warnx(b"Number of requested print threads was %d, max number is %d\n\0".as_ptr() as *const c_char, nr_threads, MAX_THREADS as c_int);
        nr_threads = MAX_THREADS as c_uint;
    }
    for i in 0..nr_threads as usize {
        let seed = malloc_or_die(std::mem::size_of::<c_long>()) as *mut c_long;
        if ufd < 0 || read(ufd, seed as *mut c_void, std::mem::size_of::<c_long>()) != std::mem::size_of::<c_long>() as ssize_t {
            printf(b"Warning! Using trivial random number seed, since %s not available\n\0".as_ptr() as *const c_char, DEV_URANDOM.as_ptr());
            fflush(stdout);
            *seed = i as c_long;
        }
        *__errno_location() = pthread_create(&mut printthread[i], 0 as *const pthread_attr_t, do_printloop, seed as *mut c_void);
        if *__errno_location() != 0 { err(EXIT_FAILURE, b"pthread_create()\0".as_ptr() as *const c_char); }
    }
    if ufd > 0 && close(ufd) != 0 { warn(b"close() failed\0".as_ptr() as *const c_char); }
}

unsafe fn show_usage() {
    printf(b"Usage: %s [OPTION]...\n\nCollect closely occurring latencies from %s\nwith any of the following tracers: preemptirqsoff, preemptoff, irqsoff, wakeup,\nwakeup_dl, or wakeup_rt.\n\nThe occurrence of a latency is detected by monitoring the file\n%s with inotify.\n\nThe following options are supported:\n\n-l, --list\t\tList the latency tracers that are supported by the\n\t\t\tcurrently running Linux kernel. If you don't see the\n\t\t\ttracer that you want, you will probably need to\n\t\t\tchange your kernel config and build a new kernel.\n\n-t, --tracer TR\t\tUse the tracer TR. The default is to use the first\n\t\t\ttracer that is supported by the kernel in the following\n\t\t\torder of precedence:\n\n\t\t\tpreemptirqsoff\n\t\t\tpreemptoff\n\t\t\tirqsoff\n\t\t\twakeup\n\t\t\twakeup_rt\n\t\t\twakeup_dl\n\n\t\t\tIf TR is not on the list above, then a warning will be\n\t\t\tprinted.\n\n-F, --force\t\tProceed even if another ftrace tracer is active. Without\n\t\t\tthis option, the program will refuse to start tracing if\n\t\t\tany other tracer than the nop tracer is active.\n\n-s, --threshold TH\tConfigure ftrace to use a threshold of TH microseconds\n\t\t\tfor the tracer. The default is 0, which means that\n\t\t\ttracing_max_latency will be used. tracing_max_latency is\n\t\t\tset to 0 when the program is started and contains the\n\t\t\tmaximum of the latencies that have been encountered.\n\n-f, --function\t\tEnable the function-trace option in trace_options. With\n\t\t\tthis option, ftrace will trace the functions that are\n\t\t\texecuted during a latency, without it we only get the\n\t\t\tbeginning, end, and backtrace.\n\n-g, --graph\t\tEnable the display-graph option in trace_option. This\n\t\t\toption causes ftrace to show the graph of how functions\n\t\t\tare calling other functions.\n\n-c, --policy POL\tRun the program with scheduling policy POL. POL can be\n\t\t\tother, batch, idle, rr or fifo. The default is rr. When\n\t\t\tusing rr or fifo, remember that these policies may cause\n\t\t\tother tasks to experience latencies.\n\n-p, --priority PRI\tRun the program with priority PRI. The acceptable range\n\t\t\tof PRI depends on the scheduling policy.\n\n-n, --notrace\t\tIf latency is detected, do not print out the content of\n\t\t\tthe trace file to standard output\n\n-e, --threads NRTHR\tRun NRTHR threads for printing. Default is %d.\n\n-r, --random\t\tArbitrarily sleep a certain amount of time, default\n\t\t\t%ld ms, before reading the trace file. The\n\t\t\tprobabilities for sleep are chosen so that the\n\t\t\tprobability of obtaining any of a cluster of closely\n\t\t\toccurring latencies are equal, i.e. we will randomly\n\t\t\tchoose which one we collect from the trace file.\n\n\t\t\tThis option is probably only useful with the irqsoff,\n\t\t\tpreemptoff, and preemptirqsoff tracers.\n\n-a, --nrlat NRLAT\tFor the purpose of arbitrary delay, assume that there\n\t\t\tare no more than NRLAT clustered latencies. If NRLAT\n\t\t\tlatencies are detected during a run, this value will\n\t\t\tautomatically be increased to NRLAT + 1 and then to\n\t\t\tNRLAT + 2 and so on. The default is %d. This option\n\t\t\timplies -r. We need to know this number in order to\n\t\t\tbe able to calculate the probabilities of sleeping.\n\t\t\tSpecifically, the probabilities of not sleeping, i.e. to\n\t\t\tdo an immediate printout will be:\n\n\t\t\t1/NRLAT  1/(NRLAT - 1) ... 1/3  1/2  1\n\n\t\t\tThe probability of sleeping will be:\n\n\t\t\t1 - P, where P is from the series above\n\n\t\t\tThis descending probability will cause us to choose\n\t\t\tan occurrence at random. Observe that the final\n\t\t\tprobability is 0, it is when we reach this probability\n\t\t\tthat we increase NRLAT automatically. As an example,\n\t\t\twith the default value of 2, the probabilities will be:\n\n\t\t\t1/2  0\n\n\t\t\tThis means, when a latency is detected we will sleep\n\t\t\twith 50%% probability. If we ever detect another latency\n\t\t\tduring the sleep period, then the probability of sleep\n\t\t\twill be 0%% and the table will be expanded to:\n\n\t\t\t1/3  1/2  0\n\n-v, --verbose\t\tIncrease the verbosity. If this option is given once,\n\t\t\tthen print a message every time that the NRLAT value\n\t\t\tis automatically increased. It also causes a message to\n\t\t\tbe printed when the ftrace settings are changed. If this\n\t\t\toption is given at least twice, then also print a\n\t\t\twarning for lost events.\n\n-u, --time TIME\t\tArbitrarily sleep for a specified time TIME ms before\n\t\t\tprinting out the trace from the trace file. The default\n\t\t\tis %ld ms. This option implies -r.\n\n-x, --no-ftrace\t\tDo not configure ftrace. This assume that the user\n\t\t\tconfigures the ftrace files in sysfs such as\n\t\t\t/sys/kernel/tracing/current_tracer or equivalent.\n\n-i, --tracefile FILE\tUse FILE as trace file. The default is\n\t\t\t%s.\n\t\t\tThis options implies -x\n\n-m, --max-lat FILE\tUse FILE as tracing_max_latency file. The default is\n\t\t\t%s.\n\t\t\tThis options implies -x\n\n\0".as_ptr() as *const c_char,
        prg_name, debug_tracefile_dflt, debug_maxlat_dflt, DEFAULT_NR_PRINTER_THREADS,
        SLEEP_TIME_MS_DEFAULT, DEFAULT_TABLE_SIZE, SLEEP_TIME_MS_DEFAULT,
        debug_tracefile_dflt, debug_maxlat_dflt);
}

unsafe fn find_tracefiles() {
    debug_tracefile_dflt = tracefs_get_tracing_file(b"trace\0".as_ptr() as *const c_char);
    if debug_tracefile_dflt.is_null() { debug_tracefile_dflt = DEBUG_NOFILE.as_ptr() as *const c_char; }
    debug_maxlat_dflt = tracefs_get_tracing_file(b"tracing_max_latency\0".as_ptr() as *const c_char);
    if debug_maxlat_dflt.is_null() { debug_maxlat_dflt = DEBUG_NOFILE.as_ptr() as *const c_char; }
    debug_tracefile = debug_tracefile_dflt;
    debug_maxlat = debug_maxlat_dflt;
}

#[no_mangle]
pub unsafe extern "C" fn alldigits(mut s: *const c_char) -> bool {
    while *s != 0 {
        if isdigit(*s as c_int) == 0 { return false; }
        s = s.add(1);
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn check_alldigits(optarg_: *const c_char, argname: *const c_char) {
    if !alldigits(optarg_) {
        errx(EXIT_FAILURE, b"The %s parameter expects a decimal argument\n\0".as_ptr() as *const c_char, argname);
    }
}

unsafe fn scan_arguments(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;
    let mut option_idx: c_int = 0;
    static mut long_options: [option; 19] = [
        option { name: b"list\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'l' as c_int },
        option { name: b"tracer\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 't' as c_int },
        option { name: b"force\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'F' as c_int },
        option { name: b"threshold\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 's' as c_int },
        option { name: b"function\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'f' as c_int },
        option { name: b"graph\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'g' as c_int },
        option { name: b"policy\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 'c' as c_int },
        option { name: b"priority\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 'p' as c_int },
        option { name: b"help\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'h' as c_int },
        option { name: b"notrace\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'n' as c_int },
        option { name: b"random\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'r' as c_int },
        option { name: b"nrlat\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 'a' as c_int },
        option { name: b"threads\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 'e' as c_int },
        option { name: b"time\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 'u' as c_int },
        option { name: b"verbose\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'v' as c_int },
        option { name: b"no-ftrace\0".as_ptr() as *const c_char, has_arg: no_argument, flag: 0 as *mut c_int, val: 'x' as c_int },
        option { name: b"tracefile\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 'i' as c_int },
        option { name: b"max-lat\0".as_ptr() as *const c_char, has_arg: required_argument, flag: 0 as *mut c_int, val: 'm' as c_int },
        option { name: 0 as *const c_char, has_arg: 0, flag: 0 as *mut c_int, val: 0 },
    ];
    find_tracefiles();
    loop {
        c = getopt_long(argc, argv, b"lt:Fs:fgc:p:hnra:e:u:vxi:m:\0".as_ptr() as *const c_char, long_options.as_mut_ptr(), &mut option_idx);
        if c == -1 { break; }
        match c {
            x if x == 'l' as c_int => { show_available(); exit(0); }
            x if x == 't' as c_int => {
                current_tracer = strdup(optarg);
                if !is_relevant_tracer(current_tracer) { warnx(b"%s is not a known latency tracer!\n\0".as_ptr() as *const c_char, current_tracer); }
                let mut notracer = false;
                let valid = tracer_valid(current_tracer, &mut notracer);
                if notracer { errx(EXIT_FAILURE, b"%s\0".as_ptr() as *const c_char, no_tracer_msg.as_ptr()); }
                if !valid { errx(EXIT_FAILURE, b"The tracer %s is not supported by your kernel!\n\0".as_ptr() as *const c_char, current_tracer); }
            }
            x if x == 'F' as c_int => force_tracer = true,
            x if x == 's' as c_int => { check_alldigits(optarg, b"-s [--threshold]\0".as_ptr() as *const c_char); threshold = strdup(optarg); }
            x if x == 'f' as c_int => use_options[OPTIDX_FUNC_TR] = true,
            x if x == 'g' as c_int => use_options[OPTIDX_DISP_GR] = true,
            x if x == 'c' as c_int => {
                let p = policy_from_name(optarg);
                if !p.is_null() {
                    sched_policy = (*p).policy; sched_policy_set = true;
                    if !sched_pri_set { sched_pri = (*p).default_pri; sched_pri_set = true; }
                } else {
                    warnx(b"Unknown scheduling %s\n\0".as_ptr() as *const c_char, optarg); show_usage(); exit(0);
                }
            }
            x if x == 'p' as c_int => { check_alldigits(optarg, b"-p [--priority]\0".as_ptr() as *const c_char); sched_pri = atoi(optarg); sched_pri_set = true; }
            x if x == 'h' as c_int => { show_usage(); exit(0); }
            x if x == 'n' as c_int => { trace_enable = false; use_random_sleep = false; }
            x if x == 'e' as c_int => {
                check_alldigits(optarg, b"-e [--threads]\0".as_ptr() as *const c_char);
                let value = atoi(optarg);
                if value > 0 { nr_threads = value as c_uint; } else { warnx(b"NRTHR must be > 0\n\0".as_ptr() as *const c_char); show_usage(); exit(0); }
            }
            x if x == 'u' as c_int => {
                check_alldigits(optarg, b"-u [--time]\0".as_ptr() as *const c_char);
                let value = atoi(optarg);
                if value < 0 { warnx(b"TIME must be >= 0\n\0".as_ptr() as *const c_char); show_usage(); exit(0); }
                trace_enable = true; use_random_sleep = true; sleep_time = value as c_long * USEC_PER_MSEC;
            }
            x if x == 'v' as c_int => verbosity += 1,
            x if x == 'r' as c_int => { trace_enable = true; use_random_sleep = true; }
            x if x == 'a' as c_int => {
                check_alldigits(optarg, b"-a [--nrlat]\0".as_ptr() as *const c_char);
                let value = atoi(optarg);
                if value <= 0 { warnx(b"NRLAT must be > 0\n\0".as_ptr() as *const c_char); show_usage(); exit(0); }
                trace_enable = true; use_random_sleep = true; table_startsize = value as c_uint;
            }
            x if x == 'x' as c_int => setup_ftrace = false,
            x if x == 'i' as c_int => { setup_ftrace = false; debug_tracefile = strdup(optarg); }
            x if x == 'm' as c_int => { setup_ftrace = false; debug_maxlat = strdup(optarg); }
            _ => { show_usage(); exit(0); }
        }
    }
    if setup_ftrace {
        if current_tracer.is_null() {
            current_tracer = find_default_tracer();
            if current_tracer.is_null() { errx(EXIT_FAILURE, b"No default tracer found and tracer not specified\n\0".as_ptr() as *const c_char); }
        }
        if use_random_sleep && !random_makes_sense(current_tracer) {
            warnx(b"WARNING: The tracer is %s and random sleep has\0".as_ptr() as *const c_char, current_tracer);
            fprintf(stderr, b"been enabled. Random sleep is intended for the following tracers:\n\0".as_ptr() as *const c_char);
            let mut i = 0;
            while !random_tracers[i].is_null() {
                fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, random_tracers[i]);
                i += 1;
            }
            fprintf(stderr, b"\n\0".as_ptr() as *const c_char);
        }
    }
    if debug_tracefile == DEBUG_NOFILE.as_ptr() as *const c_char || debug_maxlat == DEBUG_NOFILE.as_ptr() as *const c_char {
        errx(EXIT_FAILURE, b"Could not find tracing directory e.g. /sys/kernel/tracing\n\0".as_ptr() as *const c_char);
    }
    if !sched_policy_set {
        sched_policy = SCHED_RR;
        sched_policy_set = true;
        if !sched_pri_set { sched_pri = RT_DEFAULT_PRI; sched_pri_set = true; }
    }
    let max = sched_get_priority_max(sched_policy);
    let min = sched_get_priority_min(sched_policy);
    if sched_pri < min {
        printf(b"ATTENTION: Increasing priority to minimum, which is %d\n\0".as_ptr() as *const c_char, min);
        sched_pri = min;
    }
    if sched_pri > max {
        printf(b"ATTENTION: Reducing priority to maximum, which is %d\n\0".as_ptr() as *const c_char, max);
        sched_pri = max;
    }
}

unsafe fn show_params() {
    printf(b"\nRunning with scheduling policy %s and priority %d. Using %d print threads.\n\0".as_ptr() as *const c_char, policy_name(sched_policy), sched_pri, nr_threads);
    if trace_enable {
        if use_random_sleep {
            printf(b"%s will be printed with random delay\nStart size of the probability table:\t\t\t%d\nPrint a message when the prob. table changes size:\t%s\nPrint a warning when an event has been lost:\t\t%s\nSleep time is:\t\t\t\t\t\t%ld ms\n\0".as_ptr() as *const c_char, debug_tracefile, table_startsize, bool2str(verbose_sizechange()), bool2str(verbose_lostevent()), sleep_time / USEC_PER_MSEC);
        } else {
            printf(b"%s will be printed immediately\n\0".as_ptr() as *const c_char, debug_tracefile);
        }
    } else {
        printf(b"%s will not be printed\n\0".as_ptr() as *const c_char, debug_tracefile);
    }
    if setup_ftrace {
        printf(b"Tracer:\t\t\t\t\t\t\t%s\n%s option:\t\t\t\t\t%s\n%s option:\t\t\t\t\t%s\n\0".as_ptr() as *const c_char, current_tracer, optstr[OPTIDX_FUNC_TR], bool2str(use_options[OPTIDX_FUNC_TR]), optstr[OPTIDX_DISP_GR], bool2str(use_options[OPTIDX_DISP_GR]));
        if strcmp(threshold, b"0\0".as_ptr() as *const c_char) == 0 {
            printf(b"Threshold:\t\t\t\t\t\ttracing_max_latency\n\0".as_ptr() as *const c_char);
        } else {
            printf(b"Threshold:\t\t\t\t\t\t%s\n\0".as_ptr() as *const c_char, threshold);
        }
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

fn main() {
    unsafe {
        let args: Vec<std::ffi::CString> = std::env::args()
            .map(|a| std::ffi::CString::new(a).unwrap())
            .collect();
        let mut argv: Vec<*mut c_char> = args.iter().map(|a| a.as_ptr() as *mut c_char).collect();
        argv.push(0 as *mut c_char);
        let argc = args.len() as c_int;

        init_save_state();
        signal_blocking(SIG_BLOCK);
        setup_sig_handler();
        open_stdout();

        if argc >= 1 { prg_name = argv[0]; } else { prg_name = prg_unknown.as_ptr() as *const c_char; }

        scan_arguments(argc, argv.as_mut_ptr());
        show_params();

        init_printstate();
        init_print_mtx();
        if use_random_sleep {
            init_probabilities();
            if verbose_sizechange() {
                printf(b"Initializing probability table to %d\n\0".as_ptr() as *const c_char, table_startsize);
            }
            sleeptable_resize(table_startsize as c_int, false, 0 as *mut short_msg);
        }
        set_priority();
        init_queue(&mut printqueue);
        start_printthread();
        tracing_loop();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
