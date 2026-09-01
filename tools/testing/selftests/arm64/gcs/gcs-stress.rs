// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022-3 ARM Limited.
 */

// C source defined _GNU_SOURCE and _POSIX_C_SOURCE before including libc,
// kernel, and kselftest headers.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type sigset_t = [c_ulong; 16];

const EXIT_FAILURE: c_int = 1;
const NULL: *mut c_void = ptr::null_mut();

const EINTR: c_int = 4;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGCHLD: c_int = 17;
const SIGUSR1: c_int = 10;

const SA_RESTART: c_int = 0x10000000;
const SA_SIGINFO: c_int = 4;

const _SC_NPROCESSORS_CONF: c_int = 83;
const AT_HWCAP: c_ulong = 16;
const HWCAP_GCS: c_ulong = 1 << 32;

const EPOLLIN: u32 = 0x001;
const EPOLLHUP: u32 = 0x010;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CLOEXEC: c_int = 0o2000000;

const REQUIRED_ARGUMENT: c_int = 1;

#[repr(C)]
struct child_data {
    name: *mut c_char,
    output: *mut c_char,
    pid: pid_t,
    stdout: c_int,
    output_seen: bool,
    exited: bool,
    exit_status: c_int,
    exit_signal: c_int,
}

#[repr(C)]
union epoll_data_t {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data_t,
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    _pad0: c_int,
    si_pid: pid_t,
    si_uid: c_uint,
    si_status: c_int,
    _rest: [u8; 128 - 28],
}

#[repr(C)]
struct sigaction {
    sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: *mut c_void,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

static mut EPOLL_FD: c_int = 0;
static mut CHILDREN: *mut child_data = ptr::null_mut();
static mut EVS: *mut epoll_event = ptr::null_mut();
static mut TESTS: c_int = 0;
static mut NUM_CHILDREN: c_int = 0;
static mut TERMINATE: bool = false;

static mut STARTUP_PIPE: [c_int; 2] = [0; 2];

static OPTIONS: [option; 2] = [
    option {
        name: b"timeout\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: b't' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;

    fn sysconf(name: c_int) -> c_long;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    fn free(ptr: *mut c_void);
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn epoll_create1(flags: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_test_result(pass: bool, msg: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_finished();
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn wifsignaled(status: c_int) -> bool {
    let termsig = status & 0x7f;
    termsig != 0 && termsig != 0x7f
}

fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn num_processors() -> c_int {
    let nproc = sysconf(_SC_NPROCESSORS_CONF);
    if nproc < 0 {
        perror(c"Unable to read number of processors\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    nproc as c_int
}

unsafe fn start_thread(child: *mut child_data, id: c_int) {
    let mut ret: c_int;
    let mut pipefd = [0 as c_int; 2];
    let mut i: c_int;
    let mut ev: epoll_event = mem::zeroed();

    ret = pipe(pipefd.as_mut_ptr());
    if ret != 0 {
        ksft_exit_fail_msg(
            c"Failed to create stdout pipe: %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
    }

    (*child).pid = fork();
    if (*child).pid == -1 {
        ksft_exit_fail_msg(c"fork() failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }

    if (*child).pid == 0 {
        /*
         * In child, replace stdout with the pipe, errors to
         * stderr from here as kselftest prints to stdout.
         */
        ret = dup2(pipefd[1], 1);
        if ret == -1 {
            fprintf(stderr, c"dup2() %d\n".as_ptr(), errno);
            exit(EXIT_FAILURE);
        }

        /*
         * Duplicate the read side of the startup pipe to
         * FD 3 so we can close everything else.
         */
        ret = dup2(STARTUP_PIPE[0], 3);
        if ret == -1 {
            fprintf(stderr, c"dup2() %d\n".as_ptr(), errno);
            exit(EXIT_FAILURE);
        }

        /*
         * Very dumb mechanism to clean open FDs other than
         * stdio. We don't want O_CLOEXEC for the pipes...
         */
        i = 4;
        while i < 8192 {
            close(i);
            i += 1;
        }

        /*
         * Read from the startup pipe, there should be no data
         * and we should block until it is closed.  We just
         * carry on on error since this isn't super critical.
         */
        ret = read(3, &mut i as *mut c_int as *mut c_void, mem::size_of_val(&i)) as c_int;
        if ret < 0 {
            fprintf(
                stderr,
                c"read(startp pipe) failed: %s (%d)\n".as_ptr(),
                strerror(errno),
                errno,
            );
        }
        if ret > 0 {
            fprintf(stderr, c"%d bytes of data on startup pipe\n".as_ptr(), ret);
        }
        close(3);

        ret = execl(
            c"gcs-stress-thread".as_ptr(),
            c"gcs-stress-thread".as_ptr(),
            ptr::null::<c_char>(),
        );
        let _ = ret;
        fprintf(
            stderr,
            c"execl(gcs-stress-thread) failed: %d (%s)\n".as_ptr(),
            errno,
            strerror(errno),
        );

        exit(EXIT_FAILURE);
    } else {
        /*
         * In parent, remember the child and close our copy of the
         * write side of stdout.
         */
        close(pipefd[1]);
        (*child).stdout = pipefd[0];
        (*child).output = ptr::null_mut();
        (*child).exited = false;
        (*child).output_seen = false;

        ev.events = EPOLLIN | EPOLLHUP;
        ev.data.ptr = child as *mut c_void;

        ret = asprintf(&mut (*child).name, c"Thread-%d".as_ptr(), id);
        if ret == -1 {
            ksft_exit_fail_msg(c"asprintf() failed\n".as_ptr());
        }

        ret = epoll_ctl(EPOLL_FD, EPOLL_CTL_ADD, (*child).stdout, &mut ev);
        if ret < 0 {
            ksft_exit_fail_msg(
                c"%s EPOLL_CTL_ADD failed: %s (%d)\n".as_ptr(),
                (*child).name,
                strerror(errno),
                errno,
            );
        }
    }

    ksft_print_msg(c"Started %s\n".as_ptr(), (*child).name);
    NUM_CHILDREN += 1;
}

unsafe fn child_output_read(child: *mut child_data) -> bool {
    let mut read_data = [0 as c_char; 1024];
    let mut work = [0 as c_char; 1024];
    let mut ret: c_int;
    let len: c_int;
    let mut cur_work: c_int;
    let mut cur_read: c_int;

    ret = read(
        (*child).stdout,
        read_data.as_mut_ptr() as *mut c_void,
        mem::size_of_val(&read_data),
    ) as c_int;
    if ret < 0 {
        if errno == EINTR {
            return true;
        }

        ksft_print_msg(
            c"%s: read() failed: %s (%d)\n".as_ptr(),
            (*child).name,
            strerror(errno),
            errno,
        );
        return false;
    }
    len = ret;

    (*child).output_seen = true;

    /* Pick up any partial read */
    if !(*child).output.is_null() {
        strncpy(work.as_mut_ptr(), (*child).output, mem::size_of_val(&work) - 1);
        cur_work = strnlen(work.as_ptr(), mem::size_of_val(&work)) as c_int;
        free((*child).output as *mut c_void);
        (*child).output = ptr::null_mut();
    } else {
        cur_work = 0;
    }

    cur_read = 0;
    while cur_read < len {
        work[cur_work as usize] = read_data[cur_read as usize];
        cur_read += 1;

        if work[cur_work as usize] == b'\n' as c_char {
            work[cur_work as usize] = b'\0' as c_char;
            ksft_print_msg(c"%s: %s\n".as_ptr(), (*child).name, work.as_ptr());
            cur_work = 0;
        } else {
            cur_work += 1;
        }
    }

    if cur_work != 0 {
        work[cur_work as usize] = b'\0' as c_char;
        ret = asprintf(&mut (*child).output, c"%s".as_ptr(), work.as_ptr());
        if ret == -1 {
            ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
        }
    }

    false
}

unsafe fn child_output(child: *mut child_data, events: u32, mut flush: bool) {
    let mut read_more: bool;

    if events & EPOLLIN != 0 {
        loop {
            read_more = child_output_read(child);
            if !read_more {
                break;
            }
        }
    }

    if events & EPOLLHUP != 0 {
        close((*child).stdout);
        (*child).stdout = -1;
        flush = true;
    }

    if flush && !(*child).output.is_null() {
        ksft_print_msg(c"%s: %s<EOF>\n".as_ptr(), (*child).name, (*child).output);
        free((*child).output as *mut c_void);
        (*child).output = ptr::null_mut();
    }
}

unsafe fn child_tickle(child: *mut child_data) {
    if (*child).output_seen && !(*child).exited {
        kill((*child).pid, SIGUSR1);
    }
}

unsafe fn child_stop(child: *mut child_data) {
    if !(*child).exited {
        kill((*child).pid, SIGTERM);
    }
}

unsafe fn child_cleanup(child: *mut child_data) {
    let mut ret: pid_t;
    let mut status: c_int = 0;
    let mut fail = false;

    if !(*child).exited {
        loop {
            ret = waitpid((*child).pid, &mut status, 0);
            if ret == -1 && errno == EINTR {
                continue;
            }

            if ret == -1 {
                ksft_print_msg(
                    c"waitpid(%d) failed: %s (%d)\n".as_ptr(),
                    (*child).pid,
                    strerror(errno),
                    errno,
                );
                fail = true;
                break;
            }

            if wifexited(status) {
                (*child).exit_status = wexitstatus(status);
                (*child).exited = true;
            }

            if wifsignaled(status) {
                (*child).exit_signal = wtermsig(status);
                ksft_print_msg(
                    c"%s: Exited due to signal %d\n".as_ptr(),
                    (*child).name,
                    (*child).exit_signal,
                );
                fail = true;
                (*child).exited = true;
            }

            if (*child).exited {
                break;
            }
        }
    }

    if !(*child).output_seen {
        ksft_print_msg(c"%s no output seen\n".as_ptr(), (*child).name);
        fail = true;
    }

    if (*child).exit_status != 0 {
        ksft_print_msg(
            c"%s exited with error code %d\n".as_ptr(),
            (*child).name,
            (*child).exit_status,
        );
        fail = true;
    }

    ksft_test_result(!fail, c"%s\n".as_ptr(), (*child).name);
}

extern "C" fn handle_child_signal(_sig: c_int, info: *mut siginfo_t, _context: *mut c_void) {
    unsafe {
        let mut i: c_int;
        let mut found = false;

        i = 0;
        while i < NUM_CHILDREN {
            let child = CHILDREN.add(i as usize);
            if (*child).pid == (*info).si_pid {
                (*child).exited = true;
                (*child).exit_status = (*info).si_status;
                found = true;
                break;
            }
            i += 1;
        }

        if !found {
            ksft_print_msg(
                c"SIGCHLD for unknown PID %d with status %d\n".as_ptr(),
                (*info).si_pid,
                (*info).si_status,
            );
        }
    }
}

extern "C" fn handle_exit_signal(_sig: c_int, _info: *mut siginfo_t, _context: *mut c_void) {
    unsafe {
        let mut i: c_int;

        /* If we're already exiting then don't signal again */
        if TERMINATE {
            return;
        }

        ksft_print_msg(c"Got signal, exiting...\n".as_ptr());

        TERMINATE = true;

        /*
         * This should be redundant, the main loop should clean up
         * after us, but for safety stop everything we can here.
         */
        i = 0;
        while i < NUM_CHILDREN {
            child_stop(CHILDREN.add(i as usize));
            i += 1;
        }
    }
}

/* Handle any pending output without blocking */
unsafe fn drain_output(flush: bool) {
    let mut ret: c_int = 1;
    let mut i: c_int;

    while ret > 0 {
        ret = epoll_wait(EPOLL_FD, EVS, TESTS, 0);
        if ret < 0 {
            if errno == EINTR {
                continue;
            }
            ksft_print_msg(c"epoll_wait() failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }

        i = 0;
        while i < ret {
            let ev = EVS.add(i as usize);
            child_output((*ev).data.ptr as *mut child_data, (*ev).events, flush);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut seen_children: c_int;
    let mut all_children_started = false;
    let gcs_threads: c_int;
    let mut timeout: c_int = 10;
    let mut ret: c_int;
    let cpus: c_int;
    let mut i: c_int;
    let mut c: c_int;
    let mut sa: sigaction = mem::zeroed();

    loop {
        c = getopt_long(argc, argv, c"t:".as_ptr(), OPTIONS.as_ptr(), ptr::null_mut());
        if c == -1 {
            break;
        }

        match c {
            x if x == b't' as c_int => {
                ret = sscanf(optarg, c"%d".as_ptr(), &mut timeout);
                if ret != 1 {
                    ksft_exit_fail_msg(c"Failed to parse timeout %s\n".as_ptr(), optarg);
                }
            }
            _ => {
                ksft_exit_fail_msg(c"Unknown argument\n".as_ptr());
            }
        }
    }

    cpus = num_processors();
    TESTS = 0;

    if getauxval(AT_HWCAP) & HWCAP_GCS != 0 {
        /* One extra thread, trying to trigger migrations */
        gcs_threads = cpus + 1;
        TESTS += gcs_threads;
    } else {
        gcs_threads = 0;
    }

    ksft_print_header();
    ksft_set_plan(TESTS);

    ksft_print_msg(c"%d CPUs, %d GCS threads\n".as_ptr(), cpus, gcs_threads);

    if TESTS == 0 {
        ksft_exit_skip(c"No tests scheduled\n".as_ptr());
    }

    if timeout > 0 {
        ksft_print_msg(c"Will run for %ds\n".as_ptr(), timeout);
    } else {
        ksft_print_msg(c"Will run until terminated\n".as_ptr());
    }

    CHILDREN = calloc(mem::size_of::<child_data>(), TESTS as size_t) as *mut child_data;
    if CHILDREN.is_null() {
        ksft_exit_fail_msg(c"Unable to allocate child data\n".as_ptr());
    }

    ret = epoll_create1(EPOLL_CLOEXEC);
    if ret < 0 {
        ksft_exit_fail_msg(c"epoll_create1() failed: %s (%d)\n".as_ptr(), strerror(errno), ret);
    }
    EPOLL_FD = ret;

    /* Create a pipe which children will block on before execing */
    ret = pipe(STARTUP_PIPE.as_mut_ptr());
    if ret != 0 {
        ksft_exit_fail_msg(
            c"Failed to create startup pipe: %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
    }

    /* Get signal handers ready before we start any children */
    memset(
        &mut sa as *mut sigaction as *mut c_void,
        0,
        mem::size_of_val(&sa),
    );
    sa.sa_sigaction = handle_exit_signal;
    sa.sa_flags = SA_RESTART | SA_SIGINFO;
    sigemptyset(&mut sa.sa_mask);
    ret = sigaction(SIGINT, &sa, ptr::null_mut());
    if ret < 0 {
        ksft_print_msg(
            c"Failed to install SIGINT handler: %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
    }
    ret = sigaction(SIGTERM, &sa, ptr::null_mut());
    if ret < 0 {
        ksft_print_msg(
            c"Failed to install SIGTERM handler: %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
    }
    sa.sa_sigaction = handle_child_signal;
    ret = sigaction(SIGCHLD, &sa, ptr::null_mut());
    if ret < 0 {
        ksft_print_msg(
            c"Failed to install SIGCHLD handler: %s (%d)\n".as_ptr(),
            strerror(errno),
            errno,
        );
    }

    EVS = calloc(TESTS as size_t, mem::size_of::<epoll_event>()) as *mut epoll_event;
    if EVS.is_null() {
        ksft_exit_fail_msg(c"Failed to allocate %d epoll events\n".as_ptr(), TESTS);
    }

    i = 0;
    while i < gcs_threads {
        start_thread(CHILDREN.add(i as usize), i);
        i += 1;
    }

    /*
     * All children started, close the startup pipe and let them
     * run.
     */
    close(STARTUP_PIPE[0]);
    close(STARTUP_PIPE[1]);

    timeout *= 10;
    loop {
        /* Did we get a signal asking us to exit? */
        if TERMINATE {
            break;
        }

        /*
         * Timeout is counted in 100ms with no output, the
         * tests print during startup then are silent when
         * running so this should ensure they all ran enough
         * to install the signal handler, this is especially
         * useful in emulation where we will both be slow and
         * likely to have a large set of VLs.
         */
        ret = epoll_wait(EPOLL_FD, EVS, TESTS, 100);
        if ret < 0 {
            if errno == EINTR {
                continue;
            }
            ksft_exit_fail_msg(c"epoll_wait() failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }

        /* Output? */
        if ret > 0 {
            i = 0;
            while i < ret {
                let ev = EVS.add(i as usize);
                child_output((*ev).data.ptr as *mut child_data, (*ev).events, false);
                i += 1;
            }
            continue;
        }

        /* Otherwise epoll_wait() timed out */

        /*
         * If the child processes have not produced output they
         * aren't actually running the tests yet.
         */
        if !all_children_started {
            seen_children = 0;

            i = 0;
            while i < NUM_CHILDREN {
                let child = CHILDREN.add(i as usize);
                if (*child).output_seen || (*child).exited {
                    seen_children += 1;
                }
                i += 1;
            }

            if seen_children != NUM_CHILDREN {
                ksft_print_msg(c"Waiting for %d children\n".as_ptr(), NUM_CHILDREN - seen_children);
                continue;
            }

            all_children_started = true;
        }

        ksft_print_msg(c"Sending signals, timeout remaining: %d00ms\n".as_ptr(), timeout);

        i = 0;
        while i < NUM_CHILDREN {
            child_tickle(CHILDREN.add(i as usize));
            i += 1;
        }

        /* Negative timeout means run indefinitely */
        if timeout < 0 {
            continue;
        }
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }

    ksft_print_msg(c"Finishing up...\n".as_ptr());
    TERMINATE = true;

    i = 0;
    while i < TESTS {
        child_stop(CHILDREN.add(i as usize));
        i += 1;
    }

    drain_output(false);

    i = 0;
    while i < TESTS {
        child_cleanup(CHILDREN.add(i as usize));
        i += 1;
    }

    drain_output(true);

    ksft_finished();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
