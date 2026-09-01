// SPDX-License-Identifier: GPL-2.0

// C dependencies: errno.h, fcntl.h, limits.h, linux/types.h, poll.h,
// pthread.h, sched.h, signal.h, stdio.h, stdlib.h, string.h, syscall.h,
// sys/prctl.h, sys/wait.h, unistd.h, sys/socket.h, linux/kcmp.h, sys/stat.h,
// "pidfd.h", and "kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type pthread_t = c_ulong;
type ssize_t = isize;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const AF_LOCAL: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SIGKILL: c_int = 9;
const ESRCH: c_int = 3;
const ENOENT: c_int = 2;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const AT_FDCWD: c_int = -100;
const P_PID: c_int = 1;
const P_PIDFD: c_int = 3;
const WEXITED: c_int = 0x00000004;
const POLLIN: c_short = 0x0001;
const POLLHUP: c_short = 0x0010;
const __NR_exit: c_long = 60;

// Values supplied by linux/pidfd.h in the original build.
const PIDFD_GET_INFO: c_ulong = 0;
const PIDFD_THREAD: c_uint = 0;
const PIDFD_INFO_PID: u64 = 1 << 0;
const PIDFD_INFO_CREDS: u64 = 1 << 1;
const PIDFD_INFO_CGROUPID: u64 = 1 << 2;
const PIDFD_INFO_EXIT: u64 = 1 << 3;
const PIDFD_INFO_COREDUMP: u64 = 1 << 4;
const PIDFD_INFO_SUPPORTED_MASK: u64 = 1 << 5;
const PIDFD_INFO_COREDUMP_SIGNAL: u64 = 1 << 6;
const PIDFD_INFO_COREDUMP_CODE: u64 = 1 << 7;

type c_short = i16;

#[repr(C)]
#[derive(Copy, Clone)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

impl Default for pollfd {
    fn default() -> Self {
        Self {
            fd: 0,
            events: 0,
            revents: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct pidfd_info {
    mask: u64,
    cgroupid: u64,
    pid: pid_t,
    tgid: pid_t,
    ppid: pid_t,
    ruid: u32,
    rgid: u32,
    euid: u32,
    egid: u32,
    suid: u32,
    sgid: u32,
    fsuid: u32,
    fsgid: u32,
    exit_code: c_int,
    coredump_mask: u64,
    coredump_signal: c_int,
    coredump_code: c_int,
    supported_mask: u64,
}

impl Default for pidfd_info {
    fn default() -> Self {
        Self {
            mask: 0,
            cgroupid: 0,
            pid: 0,
            tgid: 0,
            ppid: 0,
            ruid: 0,
            rgid: 0,
            euid: 0,
            egid: 0,
            suid: 0,
            sgid: 0,
            fsuid: 0,
            fsgid: 0,
            exit_code: 0,
            coredump_mask: 0,
            coredump_signal: 0,
            coredump_code: 0,
            supported_mask: 0,
        }
    }
}

#[repr(C)]
struct pidfd_info_fixture {
    child_pid1: pid_t,
    child_pidfd1: c_int,

    child_pid2: pid_t,
    child_pidfd2: c_int,

    child_pid3: pid_t,
    child_pidfd3: c_int,

    child_pid4: pid_t,
    child_pidfd4: c_int,
}

extern "C" {
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pause() -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn fork() -> pid_t;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn _exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;

    fn create_child(pidfd: *mut c_int, flags: c_int) -> pid_t;
    fn write_nointr(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn read_nointr(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn sys_pidfd_send_signal(
        pidfd: c_int,
        sig: c_int,
        info: *mut c_void,
        flags: c_uint,
    ) -> c_int;
    fn sys_waitid(which: c_int, pid: pid_t, infop: *mut c_void, options: c_int) -> c_int;
    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;
    fn wait_for_pid(pid: pid_t) -> c_int;
    fn gettid() -> pid_t;
    fn sys_execveat(
        dirfd: c_int,
        pathname: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
        flags: c_int,
    ) -> c_int;
}

unsafe fn errno_value() -> c_int {
    *__errno_location()
}

fn WIFSIGNALED(status: c_int) -> bool {
    ((status & 0x7f) + 1) as i8 >= 2
}

fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

fn WIFEXITED(status: c_int) -> bool {
    WTERMSIG(status) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! EXPECT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! ASSERT_LT {
    ($left:expr, $right:expr) => {
        assert!($left < $right)
    };
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {
        assert!($expr)
    };
}

macro_rules! ASSERT_FALSE {
    ($expr:expr) => {
        assert!(!$expr)
    };
}

unsafe fn pidfd_info_setup(self_: *mut pidfd_info_fixture) {
    let mut ret: c_int;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    (*self_).child_pid1 = create_child(&mut (*self_).child_pidfd1, 0);
    EXPECT_GE!((*self_).child_pid1, 0);

    if (*self_).child_pid1 == 0 {
        close(ipc_sockets[0]);

        if write_nointr(ipc_sockets[1], b"1\0".as_ptr() as *const c_void, 1) < 0 {
            _exit(EXIT_FAILURE);
        }

        close(ipc_sockets[1]);

        pause();
        _exit(EXIT_SUCCESS);
    }

    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    /* SIGKILL but don't reap. */
    EXPECT_EQ!(
        sys_pidfd_send_signal((*self_).child_pidfd1, SIGKILL, ptr::null_mut(), 0),
        0
    );

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    (*self_).child_pid2 = create_child(&mut (*self_).child_pidfd2, 0);
    EXPECT_GE!((*self_).child_pid2, 0);

    if (*self_).child_pid2 == 0 {
        close(ipc_sockets[0]);

        if write_nointr(ipc_sockets[1], b"1\0".as_ptr() as *const c_void, 1) < 0 {
            _exit(EXIT_FAILURE);
        }

        close(ipc_sockets[1]);

        pause();
        _exit(EXIT_SUCCESS);
    }

    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    /* SIGKILL and reap. */
    EXPECT_EQ!(
        sys_pidfd_send_signal((*self_).child_pidfd2, SIGKILL, ptr::null_mut(), 0),
        0
    );
    EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid2, ptr::null_mut(), WEXITED), 0);

    (*self_).child_pid3 =
        create_child(&mut (*self_).child_pidfd3, CLONE_NEWUSER | CLONE_NEWPID);
    EXPECT_GE!((*self_).child_pid3, 0);

    if (*self_).child_pid3 == 0 {
        _exit(EXIT_SUCCESS);
    }

    (*self_).child_pid4 =
        create_child(&mut (*self_).child_pidfd4, CLONE_NEWUSER | CLONE_NEWPID);
    EXPECT_GE!((*self_).child_pid4, 0);

    if (*self_).child_pid4 == 0 {
        _exit(EXIT_SUCCESS);
    }

    EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid4, ptr::null_mut(), WEXITED), 0);
}

unsafe fn pidfd_info_teardown(self_: *mut pidfd_info_fixture) {
    sys_pidfd_send_signal((*self_).child_pidfd1, SIGKILL, ptr::null_mut(), 0);
    if (*self_).child_pidfd1 >= 0 {
        EXPECT_EQ!(0, close((*self_).child_pidfd1));
    }

    sys_waitid(P_PID, (*self_).child_pid1, ptr::null_mut(), WEXITED);

    sys_pidfd_send_signal((*self_).child_pidfd2, SIGKILL, ptr::null_mut(), 0);
    if (*self_).child_pidfd2 >= 0 {
        EXPECT_EQ!(0, close((*self_).child_pidfd2));
    }

    sys_waitid(P_PID, (*self_).child_pid2, ptr::null_mut(), WEXITED);
    sys_waitid(P_PID, (*self_).child_pid3, ptr::null_mut(), WEXITED);
    sys_waitid(P_PID, (*self_).child_pid4, ptr::null_mut(), WEXITED);
}

unsafe fn pidfd_info_sigkill_exit(self_: *mut pidfd_info_fixture) {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID,
        ..Default::default()
    };

    /* Process has exited but not been reaped so this must work. */
    ASSERT_EQ!(ioctl((*self_).child_pidfd1, PIDFD_GET_INFO, &mut info), 0);

    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl((*self_).child_pidfd1, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_CREDS) != 0);
    /* Process has exited but not been reaped, so no PIDFD_INFO_EXIT information yet. */
    ASSERT_FALSE!((info.mask & PIDFD_INFO_EXIT) != 0);
}

unsafe fn pidfd_info_sigkill_reaped(self_: *mut pidfd_info_fixture) {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID,
        ..Default::default()
    };

    /* Process has already been reaped and PIDFD_INFO_EXIT hasn't been set. */
    ASSERT_NE!(ioctl((*self_).child_pidfd2, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_EQ!(errno_value(), ESRCH);

    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl((*self_).child_pidfd2, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFSIGNALED(info.exit_code));
    ASSERT_EQ!(WTERMSIG(info.exit_code), SIGKILL);
}

unsafe fn pidfd_info_success_exit(self_: *mut pidfd_info_fixture) {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID,
        ..Default::default()
    };

    /* Process has exited but not been reaped so this must work. */
    ASSERT_EQ!(ioctl((*self_).child_pidfd3, PIDFD_GET_INFO, &mut info), 0);

    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl((*self_).child_pidfd3, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_CREDS) != 0);
    /* Process has exited but not been reaped, so no PIDFD_INFO_EXIT information yet. */
    ASSERT_FALSE!((info.mask & PIDFD_INFO_EXIT) != 0);
}

unsafe fn pidfd_info_success_reaped(self_: *mut pidfd_info_fixture) {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID,
        ..Default::default()
    };

    /* Process has already been reaped and PIDFD_INFO_EXIT hasn't been set. */
    ASSERT_NE!(ioctl((*self_).child_pidfd4, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_EQ!(errno_value(), ESRCH);

    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl((*self_).child_pidfd4, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 0);
}

unsafe fn pidfd_info_success_reaped_poll(self_: *mut pidfd_info_fixture) {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT,
        ..Default::default()
    };
    let mut fds = pollfd::default();
    let mut nevents: c_int;

    fds.events = POLLIN;
    fds.fd = (*self_).child_pidfd2;

    nevents = poll(&mut fds, 1, -1);
    ASSERT_EQ!(nevents, 1);
    ASSERT_TRUE!((fds.revents & POLLIN) != 0);
    ASSERT_TRUE!((fds.revents & POLLHUP) != 0);

    ASSERT_EQ!(ioctl((*self_).child_pidfd2, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!(WIFSIGNALED(info.exit_code));
    ASSERT_EQ!(WTERMSIG(info.exit_code), SIGKILL);
}

unsafe extern "C" fn pidfd_info_pause_thread(arg: *mut c_void) -> *mut c_void {
    let mut pid_thread: pid_t = gettid();
    let ipc_socket: c_int = *(arg as *mut c_int);

    /* Inform the grand-parent what the tid of this thread is. */
    if write_nointr(
        ipc_socket,
        &mut pid_thread as *mut _ as *const c_void,
        size_of::<pid_t>(),
    ) != size_of::<pid_t>() as ssize_t
    {
        return ptr::null_mut();
    }

    close(ipc_socket);

    /* Sleep until we're killed. */
    pause();
    ptr::null_mut()
}

unsafe fn pidfd_info_thread_group(_self_: *mut pidfd_info_fixture) {
    let mut pid_leader: pid_t;
    let mut pid_poller: pid_t;
    let mut pid_thread: pid_t = 0;
    let mut thread: pthread_t = 0;
    let mut nevents: c_int;
    let mut pidfd_leader: c_int = 0;
    let mut pidfd_thread: c_int;
    let mut pidfd_leader_thread: c_int;
    let mut ret: c_int;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut fds = pollfd::default();
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT,
        ..Default::default()
    };
    let mut info2 = pidfd_info::default();

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    pid_leader = create_child(&mut pidfd_leader, 0);
    EXPECT_GE!(pid_leader, 0);

    if pid_leader == 0 {
        close(ipc_sockets[0]);

        /* The thread will outlive the thread-group leader. */
        if pthread_create(
            &mut thread,
            ptr::null(),
            Some(pidfd_info_pause_thread),
            &mut ipc_sockets[1] as *mut _ as *mut c_void,
        ) != 0
        {
            syscall(__NR_exit, EXIT_FAILURE);
        }

        /* Make the thread-group leader exit prematurely. */
        syscall(__NR_exit, EXIT_SUCCESS);
    }

    /*
     * Opening a PIDFD_THREAD aka thread-specific pidfd based on a
     * thread-group leader must succeed.
     */
    pidfd_leader_thread = sys_pidfd_open(pid_leader, PIDFD_THREAD);
    ASSERT_GE!(pidfd_leader_thread, 0);

    pid_poller = fork();
    ASSERT_GE!(pid_poller, 0);
    if pid_poller == 0 {
        /*
         * We can't poll and wait for the old thread-group
         * leader to exit using a thread-specific pidfd. The
         * thread-group leader exited prematurely and
         * notification is delayed until all subthreads have
         * exited.
         */
        fds.events = POLLIN;
        fds.fd = pidfd_leader_thread;
        nevents = poll(&mut fds, 1, 10000 /* wait 5 seconds */);
        if nevents != 0 {
            _exit(EXIT_FAILURE);
        }
        if (fds.revents & POLLIN) != 0 {
            _exit(EXIT_FAILURE);
        }
        if (fds.revents & POLLHUP) != 0 {
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }

    /* Retrieve the tid of the thread. */
    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(
        read_nointr(
            ipc_sockets[0],
            &mut pid_thread as *mut _ as *mut c_void,
            size_of::<pid_t>(),
        ),
        size_of::<pid_t>() as ssize_t
    );
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    /* Opening a thread as a thread-group leader must fail. */
    pidfd_thread = sys_pidfd_open(pid_thread, 0);
    ASSERT_LT!(pidfd_thread, 0);
    ASSERT_EQ!(errno_value(), ENOENT);

    /* Opening a thread as a PIDFD_THREAD must succeed. */
    pidfd_thread = sys_pidfd_open(pid_thread, PIDFD_THREAD);
    ASSERT_GE!(pidfd_thread, 0);

    ASSERT_EQ!(wait_for_pid(pid_poller), 0);

    /*
     * Note that pidfd_leader is a thread-group pidfd, so polling on it
     * would only notify us once all thread in the thread-group have
     * exited. So we can't poll before we have taken down the whole
     * thread-group.
     */

    /* Get PIDFD_GET_INFO using the thread-group leader pidfd. */
    ASSERT_EQ!(ioctl(pidfd_leader, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_CREDS) != 0);
    /* Process has exited but not been reaped, so no PIDFD_INFO_EXIT information yet. */
    ASSERT_FALSE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_EQ!(info.pid, pid_leader);

    /*
     * Now retrieve the same info using the thread specific pidfd
     * for the thread-group leader.
     */
    info2.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_leader_thread, PIDFD_GET_INFO, &mut info2), 0);
    ASSERT_TRUE!((info2.mask & PIDFD_INFO_CREDS) != 0);
    /* Process has exited but not been reaped, so no PIDFD_INFO_EXIT information yet. */
    ASSERT_FALSE!((info2.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_EQ!(info2.pid, pid_leader);

    /* Now try the thread-specific pidfd. */
    ASSERT_EQ!(ioctl(pidfd_thread, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_CREDS) != 0);
    /* The thread hasn't exited, so no PIDFD_INFO_EXIT information yet. */
    ASSERT_FALSE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_EQ!(info.pid, pid_thread);

    /*
     * Take down the whole thread-group. The thread-group leader
     * exited successfully but the thread will now be SIGKILLed.
     * This must be reflected in the recorded exit information.
     */
    EXPECT_EQ!(sys_pidfd_send_signal(pidfd_leader, SIGKILL, ptr::null_mut(), 0), 0);
    EXPECT_EQ!(sys_waitid(P_PIDFD, pidfd_leader, ptr::null_mut(), WEXITED), 0);

    fds.events = POLLIN;
    fds.fd = pidfd_leader;
    nevents = poll(&mut fds, 1, -1);
    ASSERT_EQ!(nevents, 1);
    ASSERT_TRUE!((fds.revents & POLLIN) != 0);
    /* The thread-group leader has been reaped. */
    ASSERT_TRUE!((fds.revents & POLLHUP) != 0);

    /*
     * Retrieve exit information for the thread-group leader via the
     * thread-group leader pidfd.
     */
    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_leader, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    /* Even though the thread-group exited successfully it will still report the group exit code. */
    ASSERT_TRUE!(WIFSIGNALED(info.exit_code));
    ASSERT_EQ!(WTERMSIG(info.exit_code), SIGKILL);

    /*
     * Retrieve exit information for the thread-group leader via the
     * thread-specific pidfd.
     */
    info2.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_leader_thread, PIDFD_GET_INFO, &mut info2), 0);
    ASSERT_FALSE!((info2.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info2.mask & PIDFD_INFO_EXIT) != 0);

    /* Even though the thread-group exited successfully it will still report the group exit code. */
    ASSERT_TRUE!(WIFSIGNALED(info2.exit_code));
    ASSERT_EQ!(WTERMSIG(info2.exit_code), SIGKILL);

    /* Retrieve exit information for the thread. */
    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_thread, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);

    /* The thread got SIGKILLed. */
    ASSERT_TRUE!(WIFSIGNALED(info.exit_code));
    ASSERT_EQ!(WTERMSIG(info.exit_code), SIGKILL);

    EXPECT_EQ!(close(pidfd_leader), 0);
    EXPECT_EQ!(close(pidfd_thread), 0);
}

unsafe extern "C" fn pidfd_info_thread_exec(arg: *mut c_void) -> *mut c_void {
    let mut pid_thread: pid_t = gettid();
    let ipc_socket: c_int = *(arg as *mut c_int);

    /* Inform the grand-parent what the tid of this thread is. */
    if write_nointr(
        ipc_socket,
        &mut pid_thread as *mut _ as *const c_void,
        size_of::<pid_t>(),
    ) != size_of::<pid_t>() as ssize_t
    {
        return ptr::null_mut();
    }

    if read_nointr(
        ipc_socket,
        &mut pid_thread as *mut _ as *mut c_void,
        size_of::<pid_t>(),
    ) != size_of::<pid_t>() as ssize_t
    {
        return ptr::null_mut();
    }

    close(ipc_socket);

    sys_execveat(
        AT_FDCWD,
        b"pidfd_exec_helper\0".as_ptr() as *const c_char,
        ptr::null(),
        ptr::null(),
        0,
    );
    ptr::null_mut()
}

unsafe fn pidfd_info_thread_group_exec(_self_: *mut pidfd_info_fixture) {
    let mut pid_leader: pid_t;
    let mut pid_poller: pid_t;
    let mut pid_thread: pid_t = 0;
    let mut thread: pthread_t = 0;
    let mut nevents: c_int;
    let mut pidfd_leader: c_int = 0;
    let mut pidfd_leader_thread: c_int;
    let mut pidfd_thread: c_int;
    let mut ret: c_int;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut fds = pollfd::default();
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT,
        ..Default::default()
    };

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    pid_leader = create_child(&mut pidfd_leader, 0);
    EXPECT_GE!(pid_leader, 0);

    if pid_leader == 0 {
        close(ipc_sockets[0]);

        /* The thread will outlive the thread-group leader. */
        if pthread_create(
            &mut thread,
            ptr::null(),
            Some(pidfd_info_thread_exec),
            &mut ipc_sockets[1] as *mut _ as *mut c_void,
        ) != 0
        {
            syscall(__NR_exit, EXIT_FAILURE);
        }

        /* Make the thread-group leader exit prematurely. */
        syscall(__NR_exit, EXIT_SUCCESS);
    }

    /* Open a thread-specific pidfd for the thread-group leader. */
    pidfd_leader_thread = sys_pidfd_open(pid_leader, PIDFD_THREAD);
    ASSERT_GE!(pidfd_leader_thread, 0);

    pid_poller = fork();
    ASSERT_GE!(pid_poller, 0);
    if pid_poller == 0 {
        /*
         * We can't poll and wait for the old thread-group
         * leader to exit using a thread-specific pidfd. The
         * thread-group leader exited prematurely and
         * notification is delayed until all subthreads have
         * exited.
         *
         * When the thread has execed it will taken over the old
         * thread-group leaders struct pid. Calling poll after
         * the thread execed will thus block again because a new
         * thread-group has started.
         */
        fds.events = POLLIN;
        fds.fd = pidfd_leader_thread;
        nevents = poll(&mut fds, 1, 10000 /* wait 5 seconds */);
        if nevents != 0 {
            _exit(EXIT_FAILURE);
        }
        if (fds.revents & POLLIN) != 0 {
            _exit(EXIT_FAILURE);
        }
        if (fds.revents & POLLHUP) != 0 {
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }

    /* Retrieve the tid of the thread. */
    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(
        read_nointr(
            ipc_sockets[0],
            &mut pid_thread as *mut _ as *mut c_void,
            size_of::<pid_t>(),
        ),
        size_of::<pid_t>() as ssize_t
    );

    /* Opening a thread as a PIDFD_THREAD must succeed. */
    pidfd_thread = sys_pidfd_open(pid_thread, PIDFD_THREAD);
    ASSERT_GE!(pidfd_thread, 0);

    /* Now that we've opened a thread-specific pidfd the thread can exec. */
    ASSERT_EQ!(
        write_nointr(
            ipc_sockets[0],
            &mut pid_thread as *mut _ as *const c_void,
            size_of::<pid_t>(),
        ),
        size_of::<pid_t>() as ssize_t
    );
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    ASSERT_EQ!(wait_for_pid(pid_poller), 0);

    /* Wait until the kernel has SIGKILLed the thread. */
    fds.events = POLLHUP;
    fds.fd = pidfd_thread;
    nevents = poll(&mut fds, 1, -1);
    ASSERT_EQ!(nevents, 1);
    /* The thread has been reaped. */
    ASSERT_TRUE!((fds.revents & POLLHUP) != 0);

    /* Retrieve thread-specific exit info from pidfd. */
    ASSERT_EQ!(ioctl(pidfd_thread, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    /*
     * While the kernel will have SIGKILLed the whole thread-group
     * during exec it will cause the individual threads to exit
     * cleanly.
     */
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 0);

    /*
     * The thread-group leader is still alive, the thread has taken
     * over its struct pid and thus its pid number.
     */
    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_leader, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_EQ!(info.pid, pid_leader);

    /* Take down the thread-group leader. */
    EXPECT_EQ!(sys_pidfd_send_signal(pidfd_leader, SIGKILL, ptr::null_mut(), 0), 0);

    /*
     * Afte the exec we're dealing with an empty thread-group so now
     * we must see an exit notification on the thread-specific pidfd
     * for the thread-group leader as there's no subthread that can
     * revive the struct pid.
     */
    fds.events = POLLIN;
    fds.fd = pidfd_leader_thread;
    nevents = poll(&mut fds, 1, -1);
    ASSERT_EQ!(nevents, 1);
    ASSERT_TRUE!((fds.revents & POLLIN) != 0);
    ASSERT_FALSE!((fds.revents & POLLHUP) != 0);

    EXPECT_EQ!(sys_waitid(P_PIDFD, pidfd_leader, ptr::null_mut(), WEXITED), 0);

    /* Retrieve exit information for the thread-group leader. */
    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_leader, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);

    EXPECT_EQ!(close(pidfd_leader), 0);
    EXPECT_EQ!(close(pidfd_thread), 0);
}

unsafe extern "C" fn pidfd_info_thread_exec_sane(arg: *mut c_void) -> *mut c_void {
    let mut pid_thread: pid_t = gettid();
    let ipc_socket: c_int = *(arg as *mut c_int);

    /* Inform the grand-parent what the tid of this thread is. */
    if write_nointr(
        ipc_socket,
        &mut pid_thread as *mut _ as *const c_void,
        size_of::<pid_t>(),
    ) != size_of::<pid_t>() as ssize_t
    {
        return ptr::null_mut();
    }

    if read_nointr(
        ipc_socket,
        &mut pid_thread as *mut _ as *mut c_void,
        size_of::<pid_t>(),
    ) != size_of::<pid_t>() as ssize_t
    {
        return ptr::null_mut();
    }

    close(ipc_socket);

    sys_execveat(
        AT_FDCWD,
        b"pidfd_exec_helper\0".as_ptr() as *const c_char,
        ptr::null(),
        ptr::null(),
        0,
    );
    ptr::null_mut()
}

unsafe fn pidfd_info_thread_group_exec_thread(_self_: *mut pidfd_info_fixture) {
    let mut pid_leader: pid_t;
    let mut pid_poller: pid_t;
    let mut pid_thread: pid_t = 0;
    let mut thread: pthread_t = 0;
    let mut nevents: c_int;
    let mut pidfd_leader: c_int = 0;
    let mut pidfd_leader_thread: c_int;
    let mut pidfd_thread: c_int;
    let mut ret: c_int;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut fds = pollfd::default();
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT,
        ..Default::default()
    };

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    pid_leader = create_child(&mut pidfd_leader, 0);
    EXPECT_GE!(pid_leader, 0);

    if pid_leader == 0 {
        close(ipc_sockets[0]);

        /* The thread will outlive the thread-group leader. */
        if pthread_create(
            &mut thread,
            ptr::null(),
            Some(pidfd_info_thread_exec_sane),
            &mut ipc_sockets[1] as *mut _ as *mut c_void,
        ) != 0
        {
            syscall(__NR_exit, EXIT_FAILURE);
        }

        /*
         * Pause the thread-group leader. It will be killed once
         * the subthread execs.
         */
        pause();
        syscall(__NR_exit, EXIT_SUCCESS);
    }

    /* Retrieve the tid of the thread. */
    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(
        read_nointr(
            ipc_sockets[0],
            &mut pid_thread as *mut _ as *mut c_void,
            size_of::<pid_t>(),
        ),
        size_of::<pid_t>() as ssize_t
    );

    /* Opening a thread as a PIDFD_THREAD must succeed. */
    pidfd_thread = sys_pidfd_open(pid_thread, PIDFD_THREAD);
    ASSERT_GE!(pidfd_thread, 0);

    /* Open a thread-specific pidfd for the thread-group leader. */
    pidfd_leader_thread = sys_pidfd_open(pid_leader, PIDFD_THREAD);
    ASSERT_GE!(pidfd_leader_thread, 0);

    pid_poller = fork();
    ASSERT_GE!(pid_poller, 0);
    if pid_poller == 0 {
        /*
         * The subthread will now exec. The struct pid of the old
         * thread-group leader will be assumed by the subthread which
         * becomes the new thread-group leader. So no exit notification
         * must be generated. Wait for 5 seconds and call it a success
         * if no notification has been received.
         */
        fds.events = POLLIN;
        fds.fd = pidfd_leader_thread;
        nevents = poll(&mut fds, 1, 10000 /* wait 5 seconds */);
        if nevents != 0 {
            _exit(EXIT_FAILURE);
        }
        if (fds.revents & POLLIN) != 0 {
            _exit(EXIT_FAILURE);
        }
        if (fds.revents & POLLHUP) != 0 {
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }

    /* Now that we've opened a thread-specific pidfd the thread can exec. */
    ASSERT_EQ!(
        write_nointr(
            ipc_sockets[0],
            &mut pid_thread as *mut _ as *const c_void,
            size_of::<pid_t>(),
        ),
        size_of::<pid_t>() as ssize_t
    );
    EXPECT_EQ!(close(ipc_sockets[0]), 0);
    ASSERT_EQ!(wait_for_pid(pid_poller), 0);

    /* Wait until the kernel has SIGKILLed the thread. */
    fds.events = POLLHUP;
    fds.fd = pidfd_thread;
    nevents = poll(&mut fds, 1, -1);
    ASSERT_EQ!(nevents, 1);
    /* The thread has been reaped. */
    ASSERT_TRUE!((fds.revents & POLLHUP) != 0);

    /* Retrieve thread-specific exit info from pidfd. */
    ASSERT_EQ!(ioctl(pidfd_thread, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);
    /*
     * While the kernel will have SIGKILLed the whole thread-group
     * during exec it will cause the individual threads to exit
     * cleanly.
     */
    ASSERT_TRUE!(WIFEXITED(info.exit_code));
    ASSERT_EQ!(WEXITSTATUS(info.exit_code), 0);

    /*
     * The thread-group leader is still alive, the thread has taken
     * over its struct pid and thus its pid number.
     */
    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_leader, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_EQ!(info.pid, pid_leader);

    /* Take down the thread-group leader. */
    EXPECT_EQ!(sys_pidfd_send_signal(pidfd_leader, SIGKILL, ptr::null_mut(), 0), 0);

    /*
     * Afte the exec we're dealing with an empty thread-group so now
     * we must see an exit notification on the thread-specific pidfd
     * for the thread-group leader as there's no subthread that can
     * revive the struct pid.
     */
    fds.events = POLLIN;
    fds.fd = pidfd_leader_thread;
    nevents = poll(&mut fds, 1, -1);
    ASSERT_EQ!(nevents, 1);
    ASSERT_TRUE!((fds.revents & POLLIN) != 0);
    ASSERT_FALSE!((fds.revents & POLLHUP) != 0);

    EXPECT_EQ!(sys_waitid(P_PIDFD, pidfd_leader, ptr::null_mut(), WEXITED), 0);

    /* Retrieve exit information for the thread-group leader. */
    info.mask = PIDFD_INFO_CGROUPID | PIDFD_INFO_EXIT;
    ASSERT_EQ!(ioctl(pidfd_leader, PIDFD_GET_INFO, &mut info), 0);
    ASSERT_FALSE!((info.mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_EXIT) != 0);

    EXPECT_EQ!(close(pidfd_leader), 0);
    EXPECT_EQ!(close(pidfd_thread), 0);
}

/*
 * Test: PIDFD_INFO_SUPPORTED_MASK field
 *
 * Verify that when PIDFD_INFO_SUPPORTED_MASK is requested, the kernel
 * returns the supported_mask field indicating which flags the kernel supports.
 */
unsafe fn supported_mask_field() {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_SUPPORTED_MASK,
        ..Default::default()
    };
    let mut pidfd: c_int = 0;
    let mut pid: pid_t;

    pid = create_child(&mut pidfd, 0);
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        pause();
    }

    /* Request supported_mask field */
    ASSERT_EQ!(ioctl(pidfd, PIDFD_GET_INFO, &mut info), 0);

    /* Verify PIDFD_INFO_SUPPORTED_MASK is set in the reply */
    ASSERT_TRUE!((info.mask & PIDFD_INFO_SUPPORTED_MASK) != 0);

    /* Verify supported_mask contains expected flags */
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_PID) != 0);
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_CREDS) != 0);
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_CGROUPID) != 0);
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_EXIT) != 0);
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_COREDUMP) != 0);
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_SUPPORTED_MASK) != 0);
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_COREDUMP_SIGNAL) != 0);
    ASSERT_TRUE!((info.supported_mask & PIDFD_INFO_COREDUMP_CODE) != 0);

    /* Clean up */
    sys_pidfd_send_signal(pidfd, SIGKILL, ptr::null_mut(), 0);
    sys_waitid(P_PIDFD, pidfd, ptr::null_mut(), WEXITED);
    close(pidfd);
}

/*
 * Test: PIDFD_INFO_SUPPORTED_MASK always available
 *
 * Verify that supported_mask is returned even when other fields are requested.
 */
unsafe fn supported_mask_with_other_fields() {
    let mut info = pidfd_info {
        mask: PIDFD_INFO_CGROUPID | PIDFD_INFO_SUPPORTED_MASK,
        ..Default::default()
    };
    let mut pidfd: c_int = 0;
    let mut pid: pid_t;

    pid = create_child(&mut pidfd, 0);
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        pause();
    }

    ASSERT_EQ!(ioctl(pidfd, PIDFD_GET_INFO, &mut info), 0);

    /* Both fields should be present */
    ASSERT_TRUE!((info.mask & PIDFD_INFO_CGROUPID) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_SUPPORTED_MASK) != 0);
    ASSERT_NE!(info.supported_mask, 0);

    /* Clean up */
    sys_pidfd_send_signal(pidfd, SIGKILL, ptr::null_mut(), 0);
    sys_waitid(P_PIDFD, pidfd, ptr::null_mut(), WEXITED);
    close(pidfd);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
