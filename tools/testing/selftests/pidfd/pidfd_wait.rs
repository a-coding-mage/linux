/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C source dependencies removed from executable Rust:
 * errno, linux/sched.h, linux/types.h, signal.h, stdint.h, stdio.h,
 * stdlib.h, sched.h, string.h, sys/resource.h, sys/time.h, sys/types.h,
 * sys/wait.h, unistd.h, "pidfd.h", and "kselftest_harness.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;

type pid_t = c_int;
type __u64 = u64;

const EXIT_SUCCESS: c_int = 0;
const O_RDONLY: c_int = 0;
const O_DIRECTORY: c_int = 0o200000;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o4000;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const SIGCHLD: c_int = 17;
const SIGCONT: c_int = 18;
const SIGSTOP: c_int = 19;
const SIGKILL: c_int = 9;
const CLD_EXITED: c_int = 1;
const CLD_KILLED: c_int = 2;
const CLD_STOPPED: c_int = 5;
const CLD_CONTINUED: c_int = 6;
const WNOHANG: c_int = 0x00000001;
const WUNTRACED: c_int = 0x00000002;
const WSTOPPED: c_int = WUNTRACED;
const WEXITED: c_int = 0x00000004;
const WCONTINUED: c_int = 0x00000008;
const P_PIDFD: c_int = 3;
const ECHILD: c_int = 10;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const CLONE_PARENT_SETTID: u64 = 0x00100000;
const CLONE_PIDFD: u64 = 0x00001000;
const PIDFD_NONBLOCK: c_uint = O_NONBLOCK as c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
struct __clone_args {
    flags: __u64,
    pidfd: __u64,
    child_tid: __u64,
    parent_tid: __u64,
    exit_signal: __u64,
    stack: __u64,
    stack_size: __u64,
    tls: __u64,
    set_tid: __u64,
    set_tid_size: __u64,
    cgroup: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    si_pid: pid_t,
    si_uid: c_uint,
    si_status: c_int,
    _pad: [u8; 104],
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn getpid() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;

    fn sys_clone3(args: *mut __clone_args, size: usize) -> pid_t;
    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;
    fn sys_pidfd_send_signal(
        pidfd: c_int,
        sig: c_int,
        info: *mut siginfo_t,
        flags: c_uint,
    ) -> c_int;
    fn sys_waitid(which: c_int, pid: pid_t, info: *mut siginfo_t, options: c_int) -> pid_t;
}

fn ptr_to_u64<T>(ptr: *mut T) -> __u64 {
    ptr as usize as __u64
}

fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/* Attempt to de-conflict with the selftests tree. */
macro_rules! skip {
    ($ret:expr, $($arg:tt)*) => {{
        let _ = format_args!($($arg)*);
        $ret
    }};
}

#[test]
fn wait_simple() {
    unsafe {
        let mut pidfd: c_int = -1;
        let mut parent_tid: pid_t = -1;
        let mut args = __clone_args {
            parent_tid: ptr_to_u64(&mut parent_tid),
            pidfd: ptr_to_u64(&mut pidfd),
            flags: CLONE_PIDFD | CLONE_PARENT_SETTID,
            exit_signal: SIGCHLD as __u64,
            child_tid: 0,
            stack: 0,
            stack_size: 0,
            tls: 0,
            set_tid: 0,
            set_tid_size: 0,
            cgroup: 0,
        };
        let mut pid: pid_t;
        let mut info: siginfo_t = mem::zeroed();
        info.si_signo = 0;

        pidfd = open(c"/proc/self".as_ptr(), O_DIRECTORY | O_RDONLY | O_CLOEXEC);
        assert!(pidfd >= 0);

        pid = sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED);
        assert_ne!(pid, 0);
        assert_eq!(close(pidfd), 0);
        pidfd = -1;

        pidfd = open(c"/dev/null".as_ptr(), O_RDONLY | O_CLOEXEC);
        assert!(pidfd >= 0);

        pid = sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED);
        assert_ne!(pid, 0);
        assert_eq!(close(pidfd), 0);
        pidfd = -1;

        pid = sys_clone3(&mut args, mem::size_of_val(&args));
        assert!(pid >= 0);

        if pid == 0 {
            exit(EXIT_SUCCESS);
        }

        pid = sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED);
        assert!(pid >= 0);
        assert_eq!(wifexited(info.si_status), true);
        assert_eq!(wexitstatus(info.si_status), 0);
        assert_eq!(close(pidfd), 0);

        assert_eq!(info.si_signo, SIGCHLD);
        assert_eq!(info.si_code, CLD_EXITED);
        assert_eq!(info.si_pid, parent_tid);
    }
}

#[test]
fn wait_states() {
    unsafe {
        let mut pidfd: c_int = -1;
        let mut parent_tid: pid_t = -1;
        let mut args = __clone_args {
            parent_tid: ptr_to_u64(&mut parent_tid),
            pidfd: ptr_to_u64(&mut pidfd),
            flags: CLONE_PIDFD | CLONE_PARENT_SETTID,
            exit_signal: SIGCHLD as __u64,
            child_tid: 0,
            stack: 0,
            stack_size: 0,
            tls: 0,
            set_tid: 0,
            set_tid_size: 0,
            cgroup: 0,
        };
        let mut pfd: [c_int; 2] = [0; 2];
        let pid: pid_t;
        let mut info: siginfo_t = mem::zeroed();
        info.si_signo = 0;

        assert_eq!(pipe(pfd.as_mut_ptr()), 0);
        pid = sys_clone3(&mut args, mem::size_of_val(&args));
        assert!(pid >= 0);

        if pid == 0 {
            let mut buf: [c_char; 2] = [0; 2];

            close(pfd[1]);
            kill(getpid(), SIGSTOP);
            assert_eq!(read(pfd[0], buf.as_mut_ptr().cast::<c_void>(), 1), 1);
            close(pfd[0]);
            kill(getpid(), SIGSTOP);
            exit(EXIT_SUCCESS);
        }

        close(pfd[0]);
        assert_eq!(sys_waitid(P_PIDFD, pidfd, &mut info, WSTOPPED), 0);
        assert_eq!(info.si_signo, SIGCHLD);
        assert_eq!(info.si_code, CLD_STOPPED);
        assert_eq!(info.si_pid, parent_tid);

        assert_eq!(sys_pidfd_send_signal(pidfd, SIGCONT, core::ptr::null_mut(), 0), 0);

        assert_eq!(sys_waitid(P_PIDFD, pidfd, &mut info, WCONTINUED), 0);
        assert_eq!(write(pfd[1], c"C".as_ptr().cast::<c_void>(), 1), 1);
        close(pfd[1]);
        assert_eq!(info.si_signo, SIGCHLD);
        assert_eq!(info.si_code, CLD_CONTINUED);
        assert_eq!(info.si_pid, parent_tid);

        assert_eq!(sys_waitid(P_PIDFD, pidfd, &mut info, WUNTRACED), 0);
        assert_eq!(info.si_signo, SIGCHLD);
        assert_eq!(info.si_code, CLD_STOPPED);
        assert_eq!(info.si_pid, parent_tid);

        assert_eq!(sys_pidfd_send_signal(pidfd, SIGKILL, core::ptr::null_mut(), 0), 0);

        assert_eq!(sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED), 0);
        assert_eq!(info.si_signo, SIGCHLD);
        assert_eq!(info.si_code, CLD_KILLED);
        assert_eq!(info.si_pid, parent_tid);

        assert_eq!(close(pidfd), 0);
    }
}

#[test]
fn wait_nonblock() {
    unsafe {
        let mut pidfd: c_int;
        let mut flags: c_uint = 0;
        let mut parent_tid: pid_t = -1;
        let mut args = __clone_args {
            parent_tid: ptr_to_u64(&mut parent_tid),
            flags: CLONE_PARENT_SETTID,
            exit_signal: SIGCHLD as __u64,
            pidfd: 0,
            child_tid: 0,
            stack: 0,
            stack_size: 0,
            tls: 0,
            set_tid: 0,
            set_tid_size: 0,
            cgroup: 0,
        };
        let mut ret: c_int;
        let pid: pid_t;
        let mut info: siginfo_t = mem::zeroed();
        info.si_signo = 0;

        /*
         * Callers need to see ECHILD with non-blocking pidfds when no child
         * processes exists.
         */
        pidfd = sys_pidfd_open(getpid(), PIDFD_NONBLOCK);
        if pidfd < 0 {
            /* pidfd_open() doesn't support PIDFD_NONBLOCK. */
            assert_eq!(errno(), EINVAL);
            skip!(return, "Skipping PIDFD_NONBLOCK test");
        }

        ret = sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED);
        assert!(ret < 0);
        assert_eq!(errno(), ECHILD);
        assert_eq!(close(pidfd), 0);

        pid = sys_clone3(&mut args, mem::size_of_val(&args));
        assert!(pid >= 0);

        if pid == 0 {
            kill(getpid(), SIGSTOP);
            exit(EXIT_SUCCESS);
        }

        pidfd = sys_pidfd_open(pid, PIDFD_NONBLOCK);
        if pidfd < 0 {
            /* pidfd_open() doesn't support PIDFD_NONBLOCK. */
            assert_eq!(errno(), EINVAL);
            skip!(return, "Skipping PIDFD_NONBLOCK test");
        }

        flags = fcntl(pidfd, F_GETFL, 0) as c_uint;
        assert!(flags > 0);
        assert!((flags & O_NONBLOCK as c_uint) > 0);

        /*
         * Callers need to see EAGAIN/EWOULDBLOCK with non-blocking pidfd when
         * child processes exist but none have exited.
         */
        ret = sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED);
        assert!(ret < 0);
        assert_eq!(errno(), EAGAIN);

        /*
         * Callers need to continue seeing 0 with non-blocking pidfd and
         * WNOHANG raised explicitly when child processes exist but none have
         * exited.
         */
        ret = sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED | WNOHANG);
        assert_eq!(ret, 0);

        assert_eq!(fcntl(pidfd, F_SETFL, flags & !(O_NONBLOCK as c_uint)), 0);

        assert_eq!(sys_waitid(P_PIDFD, pidfd, &mut info, WSTOPPED), 0);
        assert_eq!(info.si_signo, SIGCHLD);
        assert_eq!(info.si_code, CLD_STOPPED);
        assert_eq!(info.si_pid, parent_tid);

        assert_eq!(sys_pidfd_send_signal(pidfd, SIGCONT, core::ptr::null_mut(), 0), 0);

        assert_eq!(sys_waitid(P_PIDFD, pidfd, &mut info, WEXITED), 0);
        assert_eq!(info.si_signo, SIGCHLD);
        assert_eq!(info.si_code, CLD_EXITED);
        assert_eq!(info.si_pid, parent_tid);

        assert_eq!(close(pidfd), 0);
    }
}

/* TEST_HARNESS_MAIN */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
