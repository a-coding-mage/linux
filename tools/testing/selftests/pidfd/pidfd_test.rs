/* SPDX-License-Identifier: GPL-2.0 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type pid_t = c_int;
type pthread_t = c_ulong;
type size_t = usize;
type time_t = c_long;

const CHILD_THREAD_MIN_WAIT: c_uint = 3; /* seconds */
const MAX_EVENTS: usize = 5;

const EXIT_SUCCESS: c_int = 0;
const EINVAL: c_int = 22;
const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const ESRCH: c_int = 3;

const O_CLOEXEC: c_int = 0o2000000;
const O_DIRECTORY: c_int = 0o200000;
const SIGCHLD: c_int = 17;
const SIGCONT: c_int = 18;
const SIGKILL: c_int = 9;
const SIGSTOP: c_int = 19;
const SIGUSR1: c_int = 10;

const CLONE_NEWPID: c_int = 0x20000000;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_PIDFD: c_int = 0x00001000;
const EPOLLIN: u32 = 0x001;
const EPOLL_CLOEXEC: c_int = O_CLOEXEC;
const EPOLL_CTL_ADD: c_int = 1;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_SHARED: c_int = 0x01;
const MNT_DETACH: c_int = 0x00000002;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_REC: c_ulong = 16384;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const SYS_exit: c_long = 60;
const SYS_gettid: c_long = 186;

/*
 * Constants and helpers supplied by pidfd.h/kselftest.h in the original
 * repository source.
 */
unsafe extern "C" {
    static PIDFD_SELF_THREAD_GROUP: c_int;
    static PIDFD_SELF_THREAD: c_int;
    static PID_RECYCLE: pid_t;
    static PIDFD_PASS: c_int;
    static PIDFD_FAIL: c_int;
    static PIDFD_SKIP: c_int;
    static PIDFD_XFAIL: c_int;
    static PIDFD_ERROR: c_int;

    fn sys_pidfd_send_signal(pidfd: c_int, sig: c_int, info: *mut c_void, flags: c_uint) -> c_int;
    fn wait_for_pid(pid: pid_t) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_exit_pass() -> !;
}

#[repr(C)]
#[derive(Copy, Clone)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn __clone2(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack_base: *mut c_void,
        stack_size: size_t,
        flags: c_int,
        arg: *mut c_void,
        ptid: *mut c_int,
    ) -> pid_t;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
        ptid: *mut c_int,
    ) -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_self() -> pthread_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn sleep(seconds: c_uint) -> c_uint;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn time(tloc: *mut time_t) -> time_t;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;
}

static mut have_pidfd_send_signal: bool = false;

unsafe fn pidfd_clone(flags: c_int, pidfd: *mut c_int, fn_: unsafe extern "C" fn(*mut c_void) -> c_int) -> pid_t {
    let stack_size: size_t = 1024;
    let mut stack: [c_char; 1024] = [0; 1024];

    /* Original C uses __clone2() on __ia64__; translate the non-ia64 path here. */
    clone(
        fn_,
        stack.as_mut_ptr().add(stack_size) as *mut c_void,
        flags | SIGCHLD,
        ptr::null_mut(),
        pidfd,
    )
}

static mut signal_received: pthread_t = 0;

unsafe extern "C" fn set_signal_received_on_sigusr1(sig: c_int) {
    if sig == SIGUSR1 {
        signal_received = pthread_self();
    }
}

unsafe fn send_signal(pidfd: c_int) -> c_int {
    let mut ret: c_int = 0;

    if sys_pidfd_send_signal(pidfd, SIGUSR1, ptr::null_mut(), 0) < 0 {
        ret = -EINVAL;
    } else if signal_received != pthread_self() {
        ret = -EINVAL;
    }

    signal_received = 0;
    ret
}

unsafe extern "C" fn send_signal_worker(arg: *mut c_void) -> *mut c_void {
    let pidfd: c_int = arg as isize as c_int;
    let ret: c_int;

    /* We forward any errors for the caller to handle. */
    ret = send_signal(pidfd);
    ret as isize as *mut c_void
}

/*
 * Straightforward test to see whether pidfd_send_signal() works is to send
 * a signal to ourself.
 */
unsafe fn test_pidfd_send_signal_simple_success() -> c_int {
    let mut pidfd: c_int;
    let test_name = c"pidfd_send_signal send SIGUSR1";
    let mut thread: pthread_t = 0;
    let mut thread_res: *mut c_void = ptr::null_mut();
    let mut err: c_int;

    if !have_pidfd_send_signal {
        ksft_test_result_skip(c"%s test: pidfd_send_signal() syscall not supported\n".as_ptr(), test_name.as_ptr());
        return 0;
    }

    signal(SIGUSR1, set_signal_received_on_sigusr1);

    /* Try sending a signal to ourselves via /proc/self. */
    pidfd = open(c"/proc/self".as_ptr(), O_DIRECTORY | O_CLOEXEC);
    if pidfd < 0 {
        ksft_exit_fail_msg(c"%s test: Failed to open process file descriptor\n".as_ptr(), test_name.as_ptr());
    }
    err = send_signal(pidfd);
    if err != 0 {
        ksft_exit_fail_msg(c"%s test: Error %d on sending pidfd signal\n".as_ptr(), test_name.as_ptr(), err);
    }
    close(pidfd);

    /* Now try the same thing only using PIDFD_SELF_THREAD_GROUP. */
    err = send_signal(PIDFD_SELF_THREAD_GROUP);
    if err != 0 {
        ksft_exit_fail_msg(c"%s test: Error %d on PIDFD_SELF_THREAD_GROUP signal\n".as_ptr(), test_name.as_ptr(), err);
    }

    /*
     * Now try the same thing in a thread and assert thread ID is equal to
     * worker thread ID.
     */
    if pthread_create(&mut thread, ptr::null(), send_signal_worker, PIDFD_SELF_THREAD as isize as *mut c_void) != 0 {
        ksft_exit_fail_msg(c"%s test: Failed to create thread\n".as_ptr(), test_name.as_ptr());
    }
    if pthread_join(thread, &mut thread_res) != 0 {
        ksft_exit_fail_msg(c"%s test: Failed to join thread\n".as_ptr(), test_name.as_ptr());
    }
    err = thread_res as isize as c_int;
    if err != 0 {
        ksft_exit_fail_msg(c"%s test: Error %d on PIDFD_SELF_THREAD signal\n".as_ptr(), test_name.as_ptr(), err);
    }

    ksft_test_result_pass(c"%s test: Sent signal\n".as_ptr(), test_name.as_ptr());
    0
}

unsafe fn test_pidfd_send_signal_exited_fail() -> c_int {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let saved_errno: c_int;
    let mut buf: [c_char; 256] = [0; 256];
    let pid: pid_t;
    let test_name = c"pidfd_send_signal signal exited process";

    if !have_pidfd_send_signal {
        ksft_test_result_skip(c"%s test: pidfd_send_signal() syscall not supported\n".as_ptr(), test_name.as_ptr());
        return 0;
    }

    pid = fork();
    if pid < 0 {
        ksft_exit_fail_msg(c"%s test: Failed to create new process\n".as_ptr(), test_name.as_ptr());
    }

    if pid == 0 {
        _exit(EXIT_SUCCESS);
    }

    snprintf(buf.as_mut_ptr(), mem::size_of_val(&buf), c"/proc/%d".as_ptr(), pid);

    pidfd = open(buf.as_ptr(), O_DIRECTORY | O_CLOEXEC);

    ret = wait_for_pid(pid);
    ksft_print_msg(c"waitpid WEXITSTATUS=%d\n".as_ptr(), ret);

    if pidfd < 0 {
        ksft_exit_fail_msg(c"%s test: Failed to open process file descriptor\n".as_ptr(), test_name.as_ptr());
    }

    ret = sys_pidfd_send_signal(pidfd, 0, ptr::null_mut(), 0);
    saved_errno = errno;
    close(pidfd);
    if ret == 0 {
        ksft_exit_fail_msg(c"%s test: Managed to send signal to process even though it should have failed\n".as_ptr(), test_name.as_ptr());
    }

    if saved_errno != ESRCH {
        ksft_exit_fail_msg(c"%s test: Expected to receive ESRCH as errno value but received %d instead\n".as_ptr(), test_name.as_ptr(), saved_errno);
    }

    ksft_test_result_pass(c"%s test: Failed to send signal as expected\n".as_ptr(), test_name.as_ptr());
    0
}

/*
 * Maximum number of cycles we allow. This is equivalent to PID_MAX_DEFAULT.
 * If users set a higher limit or we have cycled PIDFD_MAX_DEFAULT number of
 * times then we skip the test to not go into an infinite loop or block for a
 * long time.
 */
const PIDFD_MAX_DEFAULT: c_int = 0x8000;

unsafe fn test_pidfd_send_signal_recycled_pid_fail() -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let pid1: pid_t;
    let test_name = c"pidfd_send_signal signal recycled pid";

    if !have_pidfd_send_signal {
        ksft_test_result_skip(c"%s test: pidfd_send_signal() syscall not supported\n".as_ptr(), test_name.as_ptr());
        return 0;
    }

    ret = unshare(CLONE_NEWPID);
    if ret < 0 {
        if errno == EPERM {
            ksft_test_result_skip(c"%s test: Unsharing pid namespace not permitted\n".as_ptr(), test_name.as_ptr());
            return 0;
        }
        ksft_exit_fail_msg(c"%s test: Failed to unshare pid namespace\n".as_ptr(), test_name.as_ptr());
    }

    ret = unshare(CLONE_NEWNS);
    if ret < 0 {
        if errno == EPERM {
            ksft_test_result_skip(c"%s test: Unsharing mount namespace not permitted\n".as_ptr(), test_name.as_ptr());
            return 0;
        }
        ksft_exit_fail_msg(c"%s test: Failed to unshare mount namespace\n".as_ptr(), test_name.as_ptr());
    }

    ret = mount(ptr::null(), c"/".as_ptr(), ptr::null(), MS_REC | MS_PRIVATE, ptr::null());
    if ret < 0 {
        ksft_exit_fail_msg(c"%s test: Failed to remount / private\n".as_ptr(), test_name.as_ptr());
    }

    /* pid 1 in new pid namespace */
    pid1 = fork();
    if pid1 < 0 {
        ksft_exit_fail_msg(c"%s test: Failed to create new process\n".as_ptr(), test_name.as_ptr());
    }

    if pid1 == 0 {
        let mut buf: [c_char; 256] = [0; 256];
        let mut pid2: pid_t = 0;
        let mut pidfd: c_int = -1;

        umount2(c"/proc".as_ptr(), MNT_DETACH);
        ret = mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, ptr::null());
        if ret < 0 {
            _exit(PIDFD_ERROR);
        }

        /* grab pid PID_RECYCLE */
        i = 0;
        while i <= PIDFD_MAX_DEFAULT {
            pid2 = fork();
            if pid2 < 0 {
                _exit(PIDFD_ERROR);
            }

            if pid2 == 0 {
                _exit(PIDFD_PASS);
            }

            if pid2 == PID_RECYCLE {
                snprintf(buf.as_mut_ptr(), mem::size_of_val(&buf), c"/proc/%d".as_ptr(), pid2);
                ksft_print_msg(c"pid to recycle is %d\n".as_ptr(), pid2);
                pidfd = open(buf.as_ptr(), O_DIRECTORY | O_CLOEXEC);
            }

            if wait_for_pid(pid2) != 0 {
                _exit(PIDFD_ERROR);
            }

            if pid2 >= PID_RECYCLE {
                break;
            }
            i += 1;
        }

        /*
         * We want to be as predictable as we can so if we haven't been
         * able to grab pid PID_RECYCLE skip the test.
         */
        if pid2 != PID_RECYCLE {
            /* skip test */
            close(pidfd);
            _exit(PIDFD_SKIP);
        }

        if pidfd < 0 {
            _exit(PIDFD_ERROR);
        }

        i = 0;
        while i <= PIDFD_MAX_DEFAULT {
            let mut c: c_char = 0;
            let mut pipe_fds: [c_int; 2] = [0; 2];
            let recycled_pid: pid_t;
            let mut child_ret: c_int = PIDFD_PASS;

            ret = pipe2(pipe_fds.as_mut_ptr(), O_CLOEXEC);
            if ret < 0 {
                _exit(PIDFD_ERROR);
            }

            recycled_pid = fork();
            if recycled_pid < 0 {
                _exit(PIDFD_ERROR);
            }

            if recycled_pid == 0 {
                close(pipe_fds[1]);
                read(pipe_fds[0], &mut c as *mut c_char as *mut c_void, 1);
                close(pipe_fds[0]);

                _exit(PIDFD_PASS);
            }

            /*
             * Stop the child so we can inspect whether we have
             * recycled pid PID_RECYCLE.
             */
            close(pipe_fds[0]);
            ret = kill(recycled_pid, SIGSTOP);
            close(pipe_fds[1]);
            if ret != 0 {
                wait_for_pid(recycled_pid);
                _exit(PIDFD_ERROR);
            }

            /*
             * We have recycled the pid. Try to signal it. This
             * needs to fail since this is a different process than
             * the one the pidfd refers to.
             */
            if recycled_pid == PID_RECYCLE {
                ret = sys_pidfd_send_signal(pidfd, SIGCONT, ptr::null_mut(), 0);
                if ret != 0 && errno == ESRCH {
                    child_ret = PIDFD_XFAIL;
                } else {
                    child_ret = PIDFD_FAIL;
                }
            }

            /* let the process move on */
            ret = kill(recycled_pid, SIGCONT);
            if ret != 0 {
                kill(recycled_pid, SIGKILL);
            }

            if wait_for_pid(recycled_pid) != 0 {
                _exit(PIDFD_ERROR);
            }

            if child_ret == PIDFD_FAIL {
                /* fallthrough */
                _exit(child_ret);
            } else if child_ret == PIDFD_XFAIL {
                _exit(child_ret);
            } else if child_ret == PIDFD_PASS {
            } else {
                /* not reached */
                _exit(PIDFD_ERROR);
            }

            /*
             * If the user set a custom pid_max limit we could be
             * in the millions.
             * Skip the test in this case.
             */
            if recycled_pid > PIDFD_MAX_DEFAULT {
                _exit(PIDFD_SKIP);
            }
            i += 1;
        }

        /* failed to recycle pid */
        _exit(PIDFD_SKIP);
    }

    ret = wait_for_pid(pid1);
    if ret == PIDFD_FAIL {
        ksft_exit_fail_msg(c"%s test: Managed to signal recycled pid %d\n".as_ptr(), test_name.as_ptr(), PID_RECYCLE);
    } else if ret == PIDFD_PASS {
        ksft_exit_fail_msg(c"%s test: Failed to recycle pid %d\n".as_ptr(), test_name.as_ptr(), PID_RECYCLE);
    } else if ret == PIDFD_SKIP {
        ksft_test_result_skip(c"%s test: Skipping test\n".as_ptr(), test_name.as_ptr());
        ret = 0;
    } else if ret == PIDFD_XFAIL {
        ksft_test_result_pass(c"%s test: Failed to signal recycled pid as expected\n".as_ptr(), test_name.as_ptr());
        ret = 0;
    } else {
        /* PIDFD_ERROR */
        ksft_exit_fail_msg(c"%s test: Error while running tests\n".as_ptr(), test_name.as_ptr());
    }

    ret
}

unsafe fn test_pidfd_send_signal_syscall_support() -> c_int {
    let pidfd: c_int;
    let ret: c_int;
    let test_name = c"pidfd_send_signal check for support";

    pidfd = open(c"/proc/self".as_ptr(), O_DIRECTORY | O_CLOEXEC);
    if pidfd < 0 {
        ksft_exit_fail_msg(c"%s test: Failed to open process file descriptor\n".as_ptr(), test_name.as_ptr());
    }

    ret = sys_pidfd_send_signal(pidfd, 0, ptr::null_mut(), 0);
    if ret < 0 {
        if errno == ENOSYS {
            ksft_test_result_skip(c"%s test: pidfd_send_signal() syscall not supported\n".as_ptr(), test_name.as_ptr());
            return 0;
        }
        ksft_exit_fail_msg(c"%s test: Failed to send signal\n".as_ptr(), test_name.as_ptr());
    }

    have_pidfd_send_signal = true;
    close(pidfd);
    ksft_test_result_pass(c"%s test: pidfd_send_signal() syscall is supported. Tests can be executed\n".as_ptr(), test_name.as_ptr());
    0
}

unsafe extern "C" fn test_pidfd_poll_exec_thread(priv_: *mut c_void) -> *mut c_void {
    ksft_print_msg(
        c"Child Thread: starting. pid %d tid %ld ; and sleeping\n".as_ptr(),
        getpid(),
        syscall(SYS_gettid),
    );
    ksft_print_msg(c"Child Thread: doing exec of sleep\n".as_ptr());

    execl(c"/bin/sleep".as_ptr(), c"sleep".as_ptr(), c"3".as_ptr(), ptr::null::<c_char>());

    ksft_print_msg(
        c"Child Thread: DONE. pid %d tid %ld\n".as_ptr(),
        getpid(),
        syscall(SYS_gettid),
    );
    ptr::null_mut()
}

unsafe fn poll_pidfd(test_name: *const c_char, pidfd: c_int) {
    let mut c: c_int;
    let epoll_fd = epoll_create1(EPOLL_CLOEXEC);
    let mut event: epoll_event = mem::zeroed();
    let mut events: [epoll_event; MAX_EVENTS] = [mem::zeroed(); MAX_EVENTS];

    if epoll_fd == -1 {
        ksft_exit_fail_msg(
            c"%s test: Failed to create epoll file descriptor (errno %d)\n".as_ptr(),
            test_name,
            errno,
        );
    }

    event.events = EPOLLIN;
    event.data.fd = pidfd;

    if epoll_ctl(epoll_fd, EPOLL_CTL_ADD, pidfd, &mut event) != 0 {
        ksft_exit_fail_msg(
            c"%s test: Failed to add epoll file descriptor (errno %d)\n".as_ptr(),
            test_name,
            errno,
        );
    }

    c = epoll_wait(epoll_fd, events.as_mut_ptr(), MAX_EVENTS as c_int, 5000);
    if c != 1 || (events[0].events & EPOLLIN) == 0 {
        ksft_exit_fail_msg(
            c"%s test: Unexpected epoll_wait result (c=%d, events=%x) (errno %d)\n".as_ptr(),
            test_name,
            c,
            events[0].events,
            errno,
        );
    }

    close(epoll_fd);
    return;
}

unsafe extern "C" fn child_poll_exec_test(args: *mut c_void) -> c_int {
    let mut t1: pthread_t = 0;

    ksft_print_msg(
        c"Child (pidfd): starting. pid %d tid %ld\n".as_ptr(),
        getpid(),
        syscall(SYS_gettid),
    );
    pthread_create(&mut t1, ptr::null(), test_pidfd_poll_exec_thread, ptr::null_mut());
    /*
     * Exec in the non-leader thread will destroy the leader immediately.
     * If the wait in the parent returns too soon, the test fails.
     */
    loop {
        sleep(1);
    }
}

unsafe fn test_pidfd_poll_exec(use_waitpid: c_int) {
    let mut pidfd: c_int = 0;
    let pid: c_int;
    let mut status: c_int = 0;
    let mut ret: c_int;
    let prog_start: time_t = time(ptr::null_mut());
    let test_name = c"pidfd_poll check for premature notification on child thread exec";

    ksft_print_msg(c"Parent: pid: %d\n".as_ptr(), getpid());
    pid = pidfd_clone(CLONE_PIDFD, &mut pidfd, child_poll_exec_test);
    if pid < 0 {
        ksft_exit_fail_msg(c"%s test: pidfd_clone failed (ret %d, errno %d)\n".as_ptr(), test_name.as_ptr(), pid, errno);
    }

    ksft_print_msg(c"Parent: Waiting for Child (%d) to complete.\n".as_ptr(), pid);

    if use_waitpid != 0 {
        ret = waitpid(pid, &mut status, 0);
        if ret == -1 {
            ksft_print_msg(c"Parent: error\n".as_ptr());
        }

        if ret == pid {
            ksft_print_msg(c"Parent: Child process waited for.\n".as_ptr());
        }
    } else {
        poll_pidfd(test_name.as_ptr(), pidfd);
    }

    let prog_time: time_t = time(ptr::null_mut()) - prog_start;

    ksft_print_msg(c"Time waited for child: %lu\n".as_ptr(), prog_time);

    close(pidfd);

    if prog_time < CHILD_THREAD_MIN_WAIT as time_t || prog_time > (CHILD_THREAD_MIN_WAIT + 2) as time_t {
        ksft_exit_fail_msg(c"%s test: Failed\n".as_ptr(), test_name.as_ptr());
    } else {
        ksft_test_result_pass(c"%s test: Passed\n".as_ptr(), test_name.as_ptr());
    }
}

unsafe extern "C" fn test_pidfd_poll_leader_exit_thread(priv_: *mut c_void) -> *mut c_void {
    ksft_print_msg(
        c"Child Thread: starting. pid %d tid %ld ; and sleeping\n".as_ptr(),
        getpid(),
        syscall(SYS_gettid),
    );
    sleep(CHILD_THREAD_MIN_WAIT);
    ksft_print_msg(
        c"Child Thread: DONE. pid %d tid %ld\n".as_ptr(),
        getpid(),
        syscall(SYS_gettid),
    );
    ptr::null_mut()
}

static mut child_exit_secs: *mut time_t = ptr::null_mut();

unsafe extern "C" fn child_poll_leader_exit_test(args: *mut c_void) -> c_int {
    let mut t1: pthread_t = 0;
    let mut t2: pthread_t = 0;

    ksft_print_msg(
        c"Child: starting. pid %d tid %ld\n".as_ptr(),
        getpid(),
        syscall(SYS_gettid),
    );
    pthread_create(&mut t1, ptr::null(), test_pidfd_poll_leader_exit_thread, ptr::null_mut());
    pthread_create(&mut t2, ptr::null(), test_pidfd_poll_leader_exit_thread, ptr::null_mut());

    /*
     * glibc exit calls exit_group syscall, so explicitly call exit only
     * so that only the group leader exits, leaving the threads alone.
     */
    *child_exit_secs = time(ptr::null_mut());
    syscall(SYS_exit, 0);
    /* Never reached, but appeases compiler thinking we should return. */
    exit(0);
}

unsafe fn test_pidfd_poll_leader_exit(use_waitpid: c_int) {
    let mut pidfd: c_int = 0;
    let pid: c_int;
    let mut status: c_int = 0;
    let mut ret: c_int = 0;
    let test_name = c"pidfd_poll check for premature notification on non-emptygroup leader exit";

    child_exit_secs = mmap(
        ptr::null_mut(),
        mem::size_of::<time_t>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut time_t;

    if child_exit_secs == (-1isize) as *mut time_t {
        ksft_exit_fail_msg(c"%s test: mmap failed (errno %d)\n".as_ptr(), test_name.as_ptr(), errno);
    }

    ksft_print_msg(c"Parent: pid: %d\n".as_ptr(), getpid());
    pid = pidfd_clone(CLONE_PIDFD, &mut pidfd, child_poll_leader_exit_test);
    if pid < 0 {
        ksft_exit_fail_msg(c"%s test: pidfd_clone failed (ret %d, errno %d)\n".as_ptr(), test_name.as_ptr(), pid, errno);
    }

    ksft_print_msg(c"Parent: Waiting for Child (%d) to complete.\n".as_ptr(), pid);

    if use_waitpid != 0 {
        ret = waitpid(pid, &mut status, 0);
        if ret == -1 {
            ksft_print_msg(c"Parent: error\n".as_ptr());
        }
    } else {
        /*
         * This sleep tests for the case where if the child exits, and is in
         * EXIT_ZOMBIE, but the thread group leader is non-empty, then the poll
         * doesn't prematurely return even though there are active threads
         */
        sleep(1);
        poll_pidfd(test_name.as_ptr(), pidfd);
    }

    if ret == pid {
        ksft_print_msg(c"Parent: Child process waited for.\n".as_ptr());
    }

    let since_child_exit: time_t = time(ptr::null_mut()) - *child_exit_secs;

    ksft_print_msg(c"Time since child exit: %lu\n".as_ptr(), since_child_exit);

    close(pidfd);

    if since_child_exit < CHILD_THREAD_MIN_WAIT as time_t || since_child_exit > (CHILD_THREAD_MIN_WAIT + 2) as time_t {
        ksft_exit_fail_msg(c"%s test: Failed\n".as_ptr(), test_name.as_ptr());
    } else {
        ksft_test_result_pass(c"%s test: Passed\n".as_ptr(), test_name.as_ptr());
    }
}

fn main() {
    unsafe {
        ksft_print_header();
        ksft_set_plan(8);

        test_pidfd_poll_exec(0);
        test_pidfd_poll_exec(1);
        test_pidfd_poll_leader_exit(0);
        test_pidfd_poll_leader_exit(1);
        test_pidfd_send_signal_syscall_support();
        test_pidfd_send_signal_simple_success();
        test_pidfd_send_signal_exited_fail();
        test_pidfd_send_signal_recycled_pid_fail();

        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
