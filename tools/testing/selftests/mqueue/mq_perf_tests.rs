/*
 * This application is Copyright 2012 Red Hat, Inc.
 *	Doug Ledford <dledford@redhat.com>
 *
 * mq_perf_tests is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * mq_perf_tests is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * For the full text of the license, see <http://www.gnu.org/licenses/>.
 *
 * mq_perf_tests.c
 *   Tests various types of message queue workloads, concentrating on those
 *   situations that invole large message sizes, large message queue depths,
 *   or both, and reports back useful metrics about kernel message queue
 *   performance.
 *
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};

type FILE = c_void;
type mqd_t = c_int;
type pthread_t = c_ulong;
type clockid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type mode_t = c_uint;
type rlim_t = c_ulong;
type poptContext = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct mq_attr {
    mq_flags: c_long,
    mq_maxmsg: c_long,
    mq_msgsize: c_long,
    mq_curmsgs: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct siginfo_t {
    _data: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigaction {
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct pthread_attr_t {
    _data: [u8; 56],
}

#[repr(C)]
struct cpu_set_t {
    _data: [c_ulong; 16],
}

#[repr(C)]
struct poptOption {
    longName: *const c_char,
    shortName: c_char,
    argInfo: c_int,
    arg: *mut c_void,
    val: c_int,
    descrip: *const c_char,
    argDescrip: *const c_char,
}

#[repr(C)]
struct test {
    desc: *mut c_char,
    func: Option<unsafe extern "C" fn(*mut c_int)>,
}

const MAX_CPUS: usize = 64;
const MSG_SIZE: usize = 16;
const TEST1_LOOPS: c_int = 10000000;
const TEST2_LOOPS: c_int = 100000;

const O_RDWR: c_int = 0o2;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const O_NONBLOCK: c_int = 0o4000;
const DEFFILEMODE: mode_t = 0o666;
const RLIMIT_MSGQUEUE: c_int = 12;
const RLIM_INFINITY: rlim_t = !0;
const PRIO_PROCESS: c_int = 0;
const _SC_NPROCESSORS_ONLN: c_int = 84;
const _SC_MQ_PRIO_MAX: c_int = 28;
const SIGUSR1: c_int = 10;
const SIGHUP: c_int = 1;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGTERM: c_int = 15;
const SA_SIGINFO: c_int = 4;
const POPT_ARG_NONE: c_int = 0;
const POPT_ARG_STRING: c_int = 1;
const POPT_ARGFLAG_SHOW_DEFAULT: c_int = 0x00800000;

static usage: &[u8] = b"Usage:\n  %s [-c #[,#..] -f] path\n\n\t-c #\tSkip most tests and go straight to a high queue depth test\n\t\tand then run that test continuously (useful for running at\n\t\tthe same time as some other workload to see how much the\n\t\tcache thrashing caused by adding messages to a very deep\n\t\tqueue impacts the performance of other programs).  The number\n\t\tindicates which CPU core we should bind the process to during\n\t\tthe run.  If you have more than one physical CPU, then you\n\t\twill need one copy per physical CPU package, and you should\n\t\tspecify the CPU cores to pin ourself to via a comma separated\n\t\tlist of CPU values.\n\t-f\tOnly usable with continuous mode.  Pin ourself to the CPUs\n\t\tas requested, then instead of looping doing a high mq\n\t\tworkload, just busy loop.  This will allow us to lock up a\n\t\tsingle CPU just like we normally would, but without actually\n\t\tthrashing the CPU cache.  This is to make it easier to get\n\t\tcomparable numbers from some other workload running on the\n\t\tother CPUs.  One set of numbers with # CPUs locked up running\n\t\tan mq workload, and another set of numbers with those same\n\t\tCPUs locked away from the test workload, but not doing\n\t\tanything to trash the cache like the mq workload might.\n\tpath\tPath name of the message queue to create\n\n\tNote: this program must be run as root in order to enable all tests\n\n\0";

static mut MAX_MSGS: *mut c_char = b"/proc/sys/fs/mqueue/msg_max\0".as_ptr() as *mut c_char;
static mut MAX_MSGSIZE: *mut c_char = b"/proc/sys/fs/mqueue/msgsize_max\0".as_ptr() as *mut c_char;

static mut cpu_option_string: *mut c_char = null_mut();
static mut cpus_to_pin: [c_int; MAX_CPUS] = [0; MAX_CPUS];
static mut num_cpus_to_pin: c_int = 0;
static mut cpu_threads: [pthread_t; MAX_CPUS] = [0; MAX_CPUS];
static mut main_thread: pthread_t = 0;
static mut cpu_set: *mut cpu_set_t = null_mut();
static mut cpu_set_size: c_int = 0;
static mut cpus_online: c_int = 0;

static mut continuous_mode: c_int = 0;
static mut continuous_mode_fake: c_int = 0;

static mut saved_limits: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
static mut cur_limits: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
static mut saved_max_msgs: c_int = 0;
static mut saved_max_msgsize: c_int = 0;
static mut cur_max_msgs: c_int = 0;
static mut cur_max_msgsize: c_int = 0;
static mut max_msgs: *mut FILE = null_mut();
static mut max_msgsize: *mut FILE = null_mut();
static mut cur_nice: c_int = 0;
static mut queue_path: *mut c_char = b"/mq_perf_tests\0".as_ptr() as *mut c_char;
static mut queue: mqd_t = -1;
static mut result: mq_attr = mq_attr { mq_flags: 0, mq_maxmsg: 0, mq_msgsize: 0, mq_curmsgs: 0 };
static mut mq_prio_max: c_int = 0;

unsafe extern "C" {
    fn rewind(stream: *mut FILE);
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn error(status: c_int, errnum: c_int, fmt: *const c_char, ...) -> !;
    fn pthread_self() -> pthread_t;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_getcpuclockid(thread: pthread_t, clock_id: *mut clockid_t) -> c_int;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(attr: *mut pthread_attr_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const pthread_attr_t, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn mq_open(name: *const c_char, oflag: c_int, mode: mode_t, attr: *mut mq_attr) -> mqd_t;
    fn mq_getattr(mqdes: mqd_t, attr: *mut mq_attr) -> c_int;
    fn mq_close(mqdes: mqd_t) -> c_int;
    fn mq_unlink(name: *const c_char) -> c_int;
    fn mq_send(mqdes: mqd_t, msg_ptr: *const c_char, msg_len: size_t, msg_prio: c_uint) -> c_int;
    fn mq_receive(mqdes: mqd_t, msg_ptr: *mut c_char, msg_len: size_t, msg_prio: *mut c_uint) -> ssize_t;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn clock_getres(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn getuid() -> c_uint;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fflush(stream: *mut FILE) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn random() -> c_long;
    fn sleep(seconds: c_uint) -> c_uint;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn setpriority(which: c_int, who: c_uint, prio: c_int) -> c_int;
    fn getpriority(which: c_int, who: c_uint) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn poptGetContext(name: *const c_char, argc: c_int, argv: *const *const c_char, options: *const poptOption, flags: c_int) -> poptContext;
    fn poptGetNextOpt(con: poptContext) -> c_int;
    fn poptPrintUsage(con: poptContext, fp: *mut FILE, flags: c_int);
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_FREE(set: *mut cpu_set_t);
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_ISSET_S(cpu: c_int, setsize: size_t, cpusetp: *const cpu_set_t) -> c_int;
}

unsafe fn errno_ref() -> *mut c_int {
    __errno_location()
}

unsafe fn __set(stream: *mut FILE, value: c_int, err_msg: *mut c_char) {
    rewind(stream);
    if fprintf(stream, c"%d".as_ptr(), value) < 0 {
        perror(err_msg);
    }
}

unsafe extern "C" fn shutdown(exit_val: c_int, err_cause: *mut c_char, line_no: c_int) {
    static mut in_shutdown: c_int = 0;
    let errno_at_shutdown = *errno_ref();
    let mut i: c_int;

    /* In case we get called by multiple threads or from an sighandler */
    if {
        let old = in_shutdown;
        in_shutdown += 1;
        old
    } != 0 {
        return;
    }

    /* Free the cpu_set allocated using CPU_ALLOC in main function */
    CPU_FREE(cpu_set);

    i = 0;
    while i < num_cpus_to_pin {
        if cpu_threads[i as usize] != 0 {
            pthread_kill(cpu_threads[i as usize], SIGUSR1);
            pthread_join(cpu_threads[i as usize], null_mut());
        }
        i += 1;
    }

    if queue != -1 {
        if mq_close(queue) != 0 {
            perror(c"mq_close() during shutdown".as_ptr());
        }
    }
    if !queue_path.is_null() {
        /*
         * Be silent if this fails, if we cleaned up already it's
         * expected to fail
         */
        mq_unlink(queue_path);
    }
    if saved_max_msgs != 0 {
        __set(max_msgs, saved_max_msgs, c"failed to restore saved_max_msgs".as_ptr() as *mut c_char);
    }
    if saved_max_msgsize != 0 {
        __set(max_msgsize, saved_max_msgsize, c"failed to restore saved_max_msgsize".as_ptr() as *mut c_char);
    }
    if exit_val != 0 {
        error(exit_val, errno_at_shutdown, c"%s at %d".as_ptr(), err_cause, line_no);
    }
    exit(0);
}

unsafe extern "C" fn sig_action_SIGUSR1(signum: c_int, _info: *mut siginfo_t, _context: *mut c_void) {
    if pthread_self() != main_thread {
        pthread_exit(null_mut());
    } else {
        fprintf(stderr(), c"Caught signal %d in SIGUSR1 handler, exiting\n".as_ptr(), signum);
        shutdown(0, c"".as_ptr() as *mut c_char, 0);
        fprintf(stderr(), c"\n\nReturned from shutdown?!?!\n\n".as_ptr());
        exit(0);
    }
}

unsafe extern "C" fn sig_action(signum: c_int, _info: *mut siginfo_t, _context: *mut c_void) {
    if pthread_self() != main_thread {
        pthread_kill(main_thread, signum);
    } else {
        fprintf(stderr(), c"Caught signal %d, exiting\n".as_ptr(), signum);
        shutdown(0, c"".as_ptr() as *mut c_char, 0);
        fprintf(stderr(), c"\n\nReturned from shutdown?!?!\n\n".as_ptr());
        exit(0);
    }
}

unsafe fn get(stream: *mut FILE) -> c_int {
    let mut value: c_int = 0;
    rewind(stream);
    if fscanf(stream, c"%d".as_ptr(), &mut value) != 1 {
        shutdown(4, c"Error reading /proc entry".as_ptr() as *mut c_char, line!() as c_int);
    }
    value
}

unsafe fn set(stream: *mut FILE, value: c_int) {
    let new_value: c_int;

    rewind(stream);
    if fprintf(stream, c"%d".as_ptr(), value) < 0 {
        return shutdown(5, c"Failed writing to /proc file".as_ptr() as *mut c_char, line!() as c_int);
    }
    new_value = get(stream);
    if new_value != value {
        return shutdown(5, c"We didn't get what we wrote to /proc back".as_ptr() as *mut c_char, line!() as c_int);
    }
}

unsafe fn try_set(stream: *mut FILE, value: c_int) -> c_int {
    let new_value: c_int;

    rewind(stream);
    fprintf(stream, c"%d".as_ptr(), value);
    new_value = get(stream);
    (new_value == value) as c_int
}

unsafe fn getr(r#type: c_int, rlim: *mut rlimit) {
    if getrlimit(r#type, rlim) != 0 {
        shutdown(6, c"getrlimit()".as_ptr() as *mut c_char, line!() as c_int);
    }
}

unsafe fn setr(r#type: c_int, rlim: *mut rlimit) {
    if setrlimit(r#type, rlim) != 0 {
        shutdown(7, c"setrlimit()".as_ptr() as *mut c_char, line!() as c_int);
    }
}

/**
 * open_queue - open the global queue for testing
 * @attr - An attr struct specifying the desired queue traits
 * @result - An attr struct that lists the actual traits the queue has
 *
 * This open is not allowed to fail, failure will result in an orderly
 * shutdown of the program.  The global queue_path is used to set what
 * queue to open, the queue descriptor is saved in the global queue
 * variable.
 */
unsafe fn open_queue(attr: *mut mq_attr) {
    let flags = O_RDWR | O_EXCL | O_CREAT | O_NONBLOCK;
    let perms = DEFFILEMODE;

    queue = mq_open(queue_path, flags, perms, attr);
    if queue == -1 {
        shutdown(1, c"mq_open()".as_ptr() as *mut c_char, line!() as c_int);
    }
    if mq_getattr(queue, &raw mut result) != 0 {
        shutdown(1, c"mq_getattr()".as_ptr() as *mut c_char, line!() as c_int);
    }
    printf(c"\n\tQueue %s created:\n".as_ptr(), queue_path);
    printf(c"\t\tmq_flags:\t\t\t%s\n".as_ptr(), if result.mq_flags & O_NONBLOCK as c_long != 0 { c"O_NONBLOCK".as_ptr() } else { c"(null)".as_ptr() });
    printf(c"\t\tmq_maxmsg:\t\t\t%lu\n".as_ptr(), result.mq_maxmsg as c_ulong);
    printf(c"\t\tmq_msgsize:\t\t\t%lu\n".as_ptr(), result.mq_msgsize as c_ulong);
    printf(c"\t\tmq_curmsgs:\t\t\t%lu\n".as_ptr(), result.mq_curmsgs as c_ulong);
}

unsafe extern "C" fn fake_cont_thread(_arg: *mut c_void) -> *mut c_void {
    let mut i: c_int = 0;

    while i < num_cpus_to_pin {
        if cpu_threads[i as usize] == pthread_self() {
            break;
        }
        i += 1;
    }
    printf(c"\tStarted fake continuous mode thread %d on CPU %d\n".as_ptr(), i, cpus_to_pin[i as usize]);
    loop {}
}

unsafe extern "C" fn cont_thread(_arg: *mut c_void) -> *mut c_void {
    let mut buff = [0 as c_char; MSG_SIZE];
    let mut i: c_int = 0;
    let mut priority: c_uint = 0;

    while i < num_cpus_to_pin {
        if cpu_threads[i as usize] == pthread_self() {
            break;
        }
        i += 1;
    }
    printf(c"\tStarted continuous mode thread %d on CPU %d\n".as_ptr(), i, cpus_to_pin[i as usize]);
    loop {
        while mq_send(queue, buff.as_ptr(), buff.len(), 0) == 0 {}
        mq_receive(queue, buff.as_mut_ptr(), buff.len(), &mut priority);
    }
}

unsafe extern "C" fn const_prio(_prio: *mut c_int) {
    return;
}

unsafe extern "C" fn inc_prio(prio: *mut c_int) {
    *prio += 1;
    if *prio == mq_prio_max {
        *prio = 0;
    }
}

unsafe extern "C" fn dec_prio(prio: *mut c_int) {
    *prio -= 1;
    if *prio < 0 {
        *prio = mq_prio_max - 1;
    }
}

unsafe extern "C" fn random_prio(prio: *mut c_int) {
    *prio = (random() % mq_prio_max as c_long) as c_int;
}

static mut test2: [test; 5] = [
    test { desc: c"\n\tTest #2a: Time send/recv message, queue full, constant prio\n".as_ptr() as *mut c_char, func: Some(const_prio) },
    test { desc: c"\n\tTest #2b: Time send/recv message, queue full, increasing prio\n".as_ptr() as *mut c_char, func: Some(inc_prio) },
    test { desc: c"\n\tTest #2c: Time send/recv message, queue full, decreasing prio\n".as_ptr() as *mut c_char, func: Some(dec_prio) },
    test { desc: c"\n\tTest #2d: Time send/recv message, queue full, random prio\n".as_ptr() as *mut c_char, func: Some(random_prio) },
    test { desc: null_mut(), func: None },
];

unsafe fn do_untimed_send(buff: *mut c_char, prio_out: c_uint) {
    if mq_send(queue, buff, MSG_SIZE, prio_out) != 0 {
        shutdown(3, c"Test send failure".as_ptr() as *mut c_char, line!() as c_int);
    }
}

unsafe fn do_send_recv(clock: clockid_t, buff: *mut c_char, prio_out: c_uint, prio_in: *mut c_uint, send_total: *mut timespec, recv_total: *mut timespec) {
    let mut start: timespec = zeroed();
    let mut middle: timespec = zeroed();
    let mut end: timespec = zeroed();
    let mut nsec: c_ulong;

    clock_gettime(clock, &mut start);
    if mq_send(queue, buff, MSG_SIZE, prio_out) != 0 {
        shutdown(3, c"Test send failure".as_ptr() as *mut c_char, line!() as c_int);
    }
    clock_gettime(clock, &mut middle);
    if mq_receive(queue, buff, MSG_SIZE, prio_in) != MSG_SIZE as ssize_t {
        shutdown(3, c"Test receive failure".as_ptr() as *mut c_char, line!() as c_int);
    }
    clock_gettime(clock, &mut end);
    nsec = ((middle.tv_sec - start.tv_sec) as c_ulong * 1000000000) + (middle.tv_nsec - start.tv_nsec) as c_ulong;
    (*send_total).tv_nsec += nsec as c_long;
    if (*send_total).tv_nsec >= 1000000000 {
        (*send_total).tv_sec += 1;
        (*send_total).tv_nsec -= 1000000000;
    }
    nsec = ((end.tv_sec - middle.tv_sec) as c_ulong * 1000000000) + (end.tv_nsec - middle.tv_nsec) as c_ulong;
    (*recv_total).tv_nsec += nsec as c_long;
    if (*recv_total).tv_nsec >= 1000000000 {
        (*recv_total).tv_sec += 1;
        (*recv_total).tv_nsec -= 1000000000;
    }
}

/**
 * Tests to perform (all done with MSG_SIZE messages):
 *
 * 1) Time to add/remove message with 0 messages on queue
 * 1a) with constant prio
 * 2) Time to add/remove message when queue close to capacity:
 * 2a) with constant prio
 * 2b) with increasing prio
 * 2c) with decreasing prio
 * 2d) with random prio
 * 3) Test limits of priorities honored (double check _SC_MQ_PRIO_MAX)
 */
unsafe extern "C" fn perf_test_thread(_arg: *mut c_void) -> *mut c_void {
    let mut buff = [0 as c_char; MSG_SIZE];
    let mut prio_out: c_int;
    let mut prio_in: c_uint = 0;
    let mut i: c_int;
    let mut clock: clockid_t = 0;
    let mut _t: *mut pthread_t;
    let mut res: timespec = zeroed();
    let mut start: timespec = zeroed();
    let mut end: timespec = zeroed();
    let mut send_total: timespec = zeroed();
    let mut recv_total: timespec = zeroed();
    let mut nsec: c_ulong;
    let mut cur_test: *mut test;

    _t = &mut cpu_threads[0];
    printf(c"\n\tStarted mqueue performance test thread on CPU %d\n".as_ptr(), cpus_to_pin[0]);
    mq_prio_max = sysconf(_SC_MQ_PRIO_MAX) as c_int;
    if mq_prio_max == -1 {
        shutdown(2, c"sysconf(_SC_MQ_PRIO_MAX)".as_ptr() as *mut c_char, line!() as c_int);
    }
    if pthread_getcpuclockid(cpu_threads[0], &mut clock) != 0 {
        shutdown(2, c"pthread_getcpuclockid".as_ptr() as *mut c_char, line!() as c_int);
    }

    if clock_getres(clock, &mut res) != 0 {
        shutdown(2, c"clock_getres()".as_ptr() as *mut c_char, line!() as c_int);
    }

    printf(c"\t\tMax priorities:\t\t\t%d\n".as_ptr(), mq_prio_max);
    printf(c"\t\tClock resolution:\t\t%lu nsec%s\n".as_ptr(), res.tv_nsec as c_ulong, if res.tv_nsec > 1 { c"s".as_ptr() } else { c"".as_ptr() });

    printf(c"\n\tTest #1: Time send/recv message, queue empty\n".as_ptr());
    printf(c"\t\t(%d iterations)\n".as_ptr(), TEST1_LOOPS);
    prio_out = 0;
    send_total.tv_sec = 0;
    send_total.tv_nsec = 0;
    recv_total.tv_sec = 0;
    recv_total.tv_nsec = 0;
    i = 0;
    while i < TEST1_LOOPS {
        do_send_recv(clock, buff.as_mut_ptr(), prio_out as c_uint, &mut prio_in, &mut send_total, &mut recv_total);
        i += 1;
    }
    printf(c"\t\tSend msg:\t\t\t%ld.%lus total time\n".as_ptr(), send_total.tv_sec, send_total.tv_nsec as c_ulong);
    nsec = ((send_total.tv_sec as c_ulong * 1000000000) + send_total.tv_nsec as c_ulong) / TEST1_LOOPS as c_ulong;
    printf(c"\t\t\t\t\t\t%lld nsec/msg\n".as_ptr(), nsec as u64);
    printf(c"\t\tRecv msg:\t\t\t%ld.%lus total time\n".as_ptr(), recv_total.tv_sec, recv_total.tv_nsec as c_ulong);
    nsec = ((recv_total.tv_sec as c_ulong * 1000000000) + recv_total.tv_nsec as c_ulong) / TEST1_LOOPS as c_ulong;
    printf(c"\t\t\t\t\t\t%lld nsec/msg\n".as_ptr(), nsec as u64);

    cur_test = test2.as_mut_ptr();
    while !(*cur_test).desc.is_null() {
        printf(c"%s:\n".as_ptr(), (*cur_test).desc);
        printf(c"\t\t(%d iterations)\n".as_ptr(), TEST2_LOOPS);
        prio_out = 0;
        send_total.tv_sec = 0;
        send_total.tv_nsec = 0;
        recv_total.tv_sec = 0;
        recv_total.tv_nsec = 0;
        printf(c"\t\tFilling queue...".as_ptr());
        fflush(stdout());
        clock_gettime(clock, &mut start);
        i = 0;
        while (i as c_long) < result.mq_maxmsg - 1 {
            do_untimed_send(buff.as_mut_ptr(), prio_out as c_uint);
            ((*cur_test).func.unwrap())(&mut prio_out);
            i += 1;
        }
        clock_gettime(clock, &mut end);
        nsec = ((end.tv_sec - start.tv_sec) as c_ulong * 1000000000) + (end.tv_nsec - start.tv_nsec) as c_ulong;
        printf(c"done.\t\t%lld.%llds\n".as_ptr(), (nsec / 1000000000) as u64, (nsec % 1000000000) as u64);
        printf(c"\t\tTesting...".as_ptr());
        fflush(stdout());
        i = 0;
        while i < TEST2_LOOPS {
            do_send_recv(clock, buff.as_mut_ptr(), prio_out as c_uint, &mut prio_in, &mut send_total, &mut recv_total);
            ((*cur_test).func.unwrap())(&mut prio_out);
            i += 1;
        }
        printf(c"done.\n".as_ptr());
        printf(c"\t\tSend msg:\t\t\t%ld.%lus total time\n".as_ptr(), send_total.tv_sec, send_total.tv_nsec as c_ulong);
        nsec = ((send_total.tv_sec as c_ulong * 1000000000) + send_total.tv_nsec as c_ulong) / TEST2_LOOPS as c_ulong;
        printf(c"\t\t\t\t\t\t%lld nsec/msg\n".as_ptr(), nsec as u64);
        printf(c"\t\tRecv msg:\t\t\t%ld.%lus total time\n".as_ptr(), recv_total.tv_sec, recv_total.tv_nsec as c_ulong);
        nsec = ((recv_total.tv_sec as c_ulong * 1000000000) + recv_total.tv_nsec as c_ulong) / TEST2_LOOPS as c_ulong;
        printf(c"\t\t\t\t\t\t%lld nsec/msg\n".as_ptr(), nsec as u64);
        printf(c"\t\tDraining queue...".as_ptr());
        fflush(stdout());
        clock_gettime(clock, &mut start);
        while mq_receive(queue, buff.as_mut_ptr(), MSG_SIZE, &mut prio_in) == MSG_SIZE as ssize_t {}
        clock_gettime(clock, &mut end);
        nsec = ((end.tv_sec - start.tv_sec) as c_ulong * 1000000000) + (end.tv_nsec - start.tv_nsec) as c_ulong;
        printf(c"done.\t\t%lld.%llds\n".as_ptr(), (nsec / 1000000000) as u64, (nsec % 1000000000) as u64);
        cur_test = cur_test.add(1);
    }
    null_mut()
}

unsafe extern "C" fn increase_limits() {
    cur_limits.rlim_cur = RLIM_INFINITY;
    cur_limits.rlim_max = RLIM_INFINITY;
    setr(RLIMIT_MSGQUEUE, &raw mut cur_limits);
    loop {
        cur_max_msgs += 10;
        if try_set(max_msgs, cur_max_msgs) == 0 {
            break;
        }
    }
    cur_max_msgs = get(max_msgs);
    loop {
        cur_max_msgsize += 1024;
        if try_set(max_msgsize, cur_max_msgsize) == 0 {
            break;
        }
    }
    cur_max_msgsize = get(max_msgsize);
    if setpriority(PRIO_PROCESS, 0, -20) != 0 {
        shutdown(2, c"setpriority()".as_ptr() as *mut c_char, line!() as c_int);
    }
    cur_nice = -20;
}

unsafe fn min(a: c_int, b: c_long) -> c_int {
    if (a as c_long) < b { a } else { b as c_int }
}

unsafe fn stdout() -> *mut FILE {
    static mut STDOUT: *mut FILE = null_mut();
    STDOUT
}

unsafe fn stderr() -> *mut FILE {
    static mut STDERR: *mut FILE = null_mut();
    STDERR
}

static mut options: [poptOption; 4] = [
    poptOption {
        longName: c"continuous".as_ptr(),
        shortName: b'c' as c_char,
        argInfo: POPT_ARG_STRING,
        arg: &raw mut cpu_option_string as *mut c_void,
        val: b'c' as c_int,
        descrip: c"Run continuous tests at a high queue depth in order to test the effects of cache thrashing on other tasks on the system.  This test is intended to be run on one core of each physical CPU while some other CPU intensive task is run on all the other cores of that same physical CPU and the other task is timed.  It is assumed that the process of adding messages to the message queue in a tight loop will impact that other task to some degree.  Once the tests are performed in this way, you should then re-run the tests using fake mode in order to check the difference in time required to perform the CPU intensive task".as_ptr(),
        argDescrip: c"cpu[,cpu]".as_ptr(),
    },
    poptOption {
        longName: c"fake".as_ptr(),
        shortName: b'f' as c_char,
        argInfo: POPT_ARG_NONE,
        arg: &raw mut continuous_mode_fake as *mut c_void,
        val: 0,
        descrip: c"Tie up the CPUs that we would normally tie up incontinuous mode, but don't actually do any mq stuff, just keep the CPU busy so it can't be used to process system level tasks as this would free up resources on the other CPU cores and skew the comparison between the no-mqueue work and mqueue work tests".as_ptr(),
        argDescrip: null(),
    },
    poptOption {
        longName: c"path".as_ptr(),
        shortName: b'p' as c_char,
        argInfo: POPT_ARG_STRING | POPT_ARGFLAG_SHOW_DEFAULT,
        arg: &raw mut queue_path as *mut c_void,
        val: b'p' as c_int,
        descrip: c"The name of the path to use in the mqueue filesystem for our tests".as_ptr(),
        argDescrip: c"pathname".as_ptr(),
    },
    poptOption { longName: null(), shortName: 0, argInfo: 0, arg: null_mut(), val: 0, descrip: null(), argDescrip: null() },
];

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut attr: mq_attr = zeroed();
    let mut option: *mut c_char;
    let mut next_option: *mut c_char;
    let mut i: c_int;
    let mut cpu: c_int;
    let mut rc: c_int;
    let mut sa: sigaction = zeroed();
    let popt_context: poptContext;
    let mut retval: *mut c_void = null_mut();

    main_thread = pthread_self();
    num_cpus_to_pin = 0;

    if sysconf(_SC_NPROCESSORS_ONLN) == -1 {
        perror(c"sysconf(_SC_NPROCESSORS_ONLN)".as_ptr());
        exit(1);
    }

    if getuid() != 0 {
        ksft_exit_skip(c"Not running as root, but almost all tests require root in order to modify\nsystem settings.  Exiting.\n".as_ptr());
    }

    cpus_online = min(MAX_CPUS as c_int, sysconf(_SC_NPROCESSORS_ONLN));
    cpu_set = CPU_ALLOC(cpus_online);
    if cpu_set.is_null() {
        perror(c"CPU_ALLOC()".as_ptr());
        exit(1);
    }
    cpu_set_size = CPU_ALLOC_SIZE(cpus_online) as c_int;
    CPU_ZERO_S(cpu_set_size as size_t, cpu_set);

    popt_context = poptGetContext(null(), argc, argv as *const *const c_char, options.as_ptr(), 0);

    loop {
        rc = poptGetNextOpt(popt_context);
        if rc <= 0 {
            break;
        }
        match rc {
            x if x == b'c' as c_int => {
                continuous_mode = 1;
                option = cpu_option_string;
                loop {
                    next_option = strchr(option, b',' as c_int);
                    if !next_option.is_null() {
                        *next_option = 0;
                    }
                    cpu = atoi(option);
                    if cpu >= cpus_online {
                        fprintf(stderr(), c"CPU %d exceeds cpus online, ignoring.\n".as_ptr(), cpu);
                    } else {
                        cpus_to_pin[num_cpus_to_pin as usize] = cpu;
                        num_cpus_to_pin += 1;
                    }
                    if !next_option.is_null() {
                        next_option = next_option.add(1);
                        option = next_option;
                    }
                    if next_option.is_null() || num_cpus_to_pin >= MAX_CPUS as c_int {
                        break;
                    }
                }
                /* Double check that they didn't give us the same CPU
                 * more than once */
                cpu = 0;
                while cpu < num_cpus_to_pin {
                    if CPU_ISSET_S(cpus_to_pin[cpu as usize], cpu_set_size as size_t, cpu_set) != 0 {
                        fprintf(stderr(), c"Any given CPU may only be given once.\n".as_ptr());
                        CPU_FREE(cpu_set);
                        exit(1);
                    } else {
                        CPU_SET_S(cpus_to_pin[cpu as usize], cpu_set_size as size_t, cpu_set);
                    }
                    cpu += 1;
                }
            }
            x if x == b'p' as c_int => {
                /*
                 * Although we can create a msg queue with a
                 * non-absolute path name, unlink will fail.  So,
                 * if the name doesn't start with a /, add one
                 * when we save it.
                 */
                option = queue_path;
                if *option != b'/' as c_char {
                    queue_path = malloc(strlen(option) + 2) as *mut c_char;
                    if queue_path.is_null() {
                        perror(c"malloc()".as_ptr());
                        CPU_FREE(cpu_set);
                        exit(1);
                    }
                    *queue_path = b'/' as c_char;
                    *queue_path.add(1) = 0;
                    strcat(queue_path, option);
                    free(option as *mut c_void);
                }
            }
            _ => {}
        }
    }

    if continuous_mode != 0 && num_cpus_to_pin == 0 {
        fprintf(stderr(), c"Must pass at least one CPU to continuous mode.\n".as_ptr());
        poptPrintUsage(popt_context, stderr(), 0);
        CPU_FREE(cpu_set);
        exit(1);
    } else if continuous_mode == 0 {
        num_cpus_to_pin = 1;
        cpus_to_pin[0] = cpus_online - 1;
    }

    max_msgs = fopen(MAX_MSGS, c"r+".as_ptr());
    max_msgsize = fopen(MAX_MSGSIZE, c"r+".as_ptr());
    if max_msgs.is_null() {
        shutdown(2, c"Failed to open msg_max".as_ptr() as *mut c_char, line!() as c_int);
    }
    if max_msgsize.is_null() {
        shutdown(2, c"Failed to open msgsize_max".as_ptr() as *mut c_char, line!() as c_int);
    }

    /* Load up the current system values for everything we can */
    getr(RLIMIT_MSGQUEUE, &raw mut saved_limits);
    cur_limits = saved_limits;
    saved_max_msgs = get(max_msgs);
    cur_max_msgs = saved_max_msgs;
    saved_max_msgsize = get(max_msgsize);
    cur_max_msgsize = saved_max_msgsize;
    *errno_ref() = 0;
    cur_nice = getpriority(PRIO_PROCESS, 0);
    if *errno_ref() != 0 {
        shutdown(2, c"getpriority()".as_ptr() as *mut c_char, line!() as c_int);
    }

    /* Tell the user our initial state */
    printf(c"\nInitial system state:\n".as_ptr());
    printf(c"\tUsing queue path:\t\t\t%s\n".as_ptr(), queue_path);
    printf(c"\tRLIMIT_MSGQUEUE(soft):\t\t\t%ld\n".as_ptr(), saved_limits.rlim_cur as c_long);
    printf(c"\tRLIMIT_MSGQUEUE(hard):\t\t\t%ld\n".as_ptr(), saved_limits.rlim_max as c_long);
    printf(c"\tMaximum Message Size:\t\t\t%d\n".as_ptr(), saved_max_msgsize);
    printf(c"\tMaximum Queue Size:\t\t\t%d\n".as_ptr(), saved_max_msgs);
    printf(c"\tNice value:\t\t\t\t%d\n".as_ptr(), cur_nice);
    printf(c"\n".as_ptr());

    increase_limits();

    printf(c"Adjusted system state for testing:\n".as_ptr());
    if cur_limits.rlim_cur == RLIM_INFINITY {
        printf(c"\tRLIMIT_MSGQUEUE(soft):\t\t\t(unlimited)\n".as_ptr());
        printf(c"\tRLIMIT_MSGQUEUE(hard):\t\t\t(unlimited)\n".as_ptr());
    } else {
        printf(c"\tRLIMIT_MSGQUEUE(soft):\t\t\t%ld\n".as_ptr(), cur_limits.rlim_cur as c_long);
        printf(c"\tRLIMIT_MSGQUEUE(hard):\t\t\t%ld\n".as_ptr(), cur_limits.rlim_max as c_long);
    }
    printf(c"\tMaximum Message Size:\t\t\t%d\n".as_ptr(), cur_max_msgsize);
    printf(c"\tMaximum Queue Size:\t\t\t%d\n".as_ptr(), cur_max_msgs);
    printf(c"\tNice value:\t\t\t\t%d\n".as_ptr(), cur_nice);
    printf(c"\tContinuous mode:\t\t\t(%s)\n".as_ptr(), if continuous_mode != 0 { if continuous_mode_fake != 0 { c"fake mode".as_ptr() } else { c"enabled".as_ptr() } } else { c"disabled".as_ptr() });
    printf(c"\tCPUs to pin:\t\t\t\t%d".as_ptr(), cpus_to_pin[0]);
    cpu = 1;
    while cpu < num_cpus_to_pin {
        printf(c",%d".as_ptr(), cpus_to_pin[cpu as usize]);
        cpu += 1;
    }
    printf(c"\n".as_ptr());

    sa.sa_sigaction = Some(sig_action_SIGUSR1);
    sigemptyset(&mut sa.sa_mask);
    sigaddset(&mut sa.sa_mask, SIGHUP);
    sigaddset(&mut sa.sa_mask, SIGINT);
    sigaddset(&mut sa.sa_mask, SIGQUIT);
    sigaddset(&mut sa.sa_mask, SIGTERM);
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGUSR1, &sa, null_mut()) == -1 {
        shutdown(1, c"sigaction(SIGUSR1)".as_ptr() as *mut c_char, line!() as c_int);
    }
    sa.sa_sigaction = Some(sig_action);
    if sigaction(SIGHUP, &sa, null_mut()) == -1 {
        shutdown(1, c"sigaction(SIGHUP)".as_ptr() as *mut c_char, line!() as c_int);
    }
    if sigaction(SIGINT, &sa, null_mut()) == -1 {
        shutdown(1, c"sigaction(SIGINT)".as_ptr() as *mut c_char, line!() as c_int);
    }
    if sigaction(SIGQUIT, &sa, null_mut()) == -1 {
        shutdown(1, c"sigaction(SIGQUIT)".as_ptr() as *mut c_char, line!() as c_int);
    }
    if sigaction(SIGTERM, &sa, null_mut()) == -1 {
        shutdown(1, c"sigaction(SIGTERM)".as_ptr() as *mut c_char, line!() as c_int);
    }

    if continuous_mode_fake == 0 {
        attr.mq_flags = O_NONBLOCK as c_long;
        attr.mq_maxmsg = cur_max_msgs as c_long;
        attr.mq_msgsize = MSG_SIZE as c_long;
        open_queue(&mut attr);
    }
    i = 0;
    while i < num_cpus_to_pin {
        let mut thread_attr: pthread_attr_t = zeroed();
        let thread_func: unsafe extern "C" fn(*mut c_void) -> *mut c_void;

        if continuous_mode_fake != 0 {
            thread_func = fake_cont_thread;
        } else if continuous_mode != 0 {
            thread_func = cont_thread;
        } else {
            thread_func = perf_test_thread;
        }

        CPU_ZERO_S(cpu_set_size as size_t, cpu_set);
        CPU_SET_S(cpus_to_pin[i as usize], cpu_set_size as size_t, cpu_set);
        pthread_attr_init(&mut thread_attr);
        pthread_attr_setaffinity_np(&mut thread_attr, cpu_set_size as size_t, cpu_set);
        if pthread_create(&mut cpu_threads[i as usize], &thread_attr, thread_func, null_mut()) != 0 {
            shutdown(1, c"pthread_create()".as_ptr() as *mut c_char, line!() as c_int);
        }
        pthread_attr_destroy(&mut thread_attr);
        i += 1;
    }

    if continuous_mode == 0 {
        pthread_join(cpu_threads[0], &mut retval);
        shutdown(retval as c_long as c_int, c"perf_test_thread()".as_ptr() as *mut c_char, line!() as c_int);
    } else {
        loop {
            sleep(1);
        }
    }
}

fn main() {
    unsafe {
        unsafe extern "C" {
            static mut __libc_argc: c_int;
            static mut __libc_argv: *mut *mut c_char;
        }
        main_0(__libc_argc, __libc_argv);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
