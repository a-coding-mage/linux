// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// errno.h, fcntl.h, limits.h, linux/types.h, poll.h, sched.h, signal.h,
// stdio.h, stdlib.h, string.h, syscall.h, sys/prctl.h, sys/wait.h, unistd.h,
// sys/socket.h, linux/kcmp.h, "pidfd.h", and "kselftest_harness.h".

use libc::{
    c_char, c_int, c_long, c_ulong, c_void, close, fcntl, fork, getpid, getuid, kill, pid_t, poll,
    pollfd, prctl, recv, send, seteuid, socketpair, strerror, syscall, waitpid, EBADF, EINVAL,
    ENOSYS, EPERM, ESRCH, EXIT_FAILURE, EXIT_SUCCESS, FD_CLOEXEC, F_GETFD, PF_LOCAL, POLLIN,
    PR_SET_DUMPABLE, PR_SET_PDEATHSIG, SIGKILL, SOCK_SEQPACKET,
};

/*
 * UNKNOWN_FD is an fd number that should never exist in the child, as it is
 * used to check the negative case.
 */
const UNKNOWN_FD: c_int = 111;
const UID_NOBODY: c_int = 65535;
const KCMP_FILE: c_int = 0;
const KSFT_SKIP: c_int = 4;

unsafe extern "C" {
    fn sys_memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;
    fn sys_pidfd_getfd(pidfd: c_int, targetfd: c_int, flags: c_uint) -> c_int;
    fn __errno_location() -> *mut c_int;
}

type c_uint = libc::c_uint;

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn set_errno(value: c_int) {
    unsafe {
        *__errno_location() = value;
    }
}

unsafe fn wait_for_pid(pid: pid_t) -> c_int {
    let mut status: c_int = 0;
    let ret = unsafe { waitpid(pid, &mut status, 0) };
    if ret < 0 {
        -1
    } else {
        status
    }
}

unsafe fn sys_kcmp(
    pid1: pid_t,
    pid2: pid_t,
    type_: c_int,
    idx1: c_ulong,
    idx2: c_ulong,
) -> c_int {
    unsafe { syscall(libc::SYS_kcmp as c_long, pid1, pid2, type_, idx1, idx2) as c_int }
}

unsafe fn __child(sk: c_int, memfd: c_int) -> c_int {
    let mut ret: c_int;
    let mut buf: c_char = 0;

    /*
     * Ensure we don't leave around a bunch of orphaned children if our
     * tests fail.
     */
    ret = unsafe { prctl(PR_SET_PDEATHSIG, SIGKILL) };
    if ret != 0 {
        unsafe {
            fprintf!(
                stderr,
                "%s: Child could not set DEATHSIG\n",
                strerror(errno())
            );
        }
        return -1;
    }

    ret = unsafe {
        send(
            sk,
            &memfd as *const c_int as *const c_void,
            core::mem::size_of_val(&memfd),
            0,
        ) as c_int
    };
    if ret != core::mem::size_of_val(&memfd) as c_int {
        unsafe {
            fprintf!(
                stderr,
                "%s: Child failed to send fd number\n",
                strerror(errno())
            );
        }
        return -1;
    }

    /*
     * The fixture setup is completed at this point. The tests will run.
     *
     * This blocking recv enables the parent to message the child.
     * Either we will read 'P' off of the sk, indicating that we need
     * to disable ptrace, or we will read a 0, indicating that the other
     * side has closed the sk. This occurs during fixture teardown time,
     * indicating that the child should exit.
     */
    loop {
        ret = unsafe {
            recv(
                sk,
                &mut buf as *mut c_char as *mut c_void,
                core::mem::size_of_val(&buf),
                0,
            ) as c_int
        };
        if ret <= 0 {
            break;
        }

        if buf == b'P' as c_char {
            ret = unsafe { prctl(PR_SET_DUMPABLE, 0) };
            if ret < 0 {
                unsafe {
                    fprintf!(
                        stderr,
                        "%s: Child failed to disable ptrace\n",
                        strerror(errno())
                    );
                }
                return -1;
            }
        } else {
            unsafe {
                fprintf!(stderr, "Child received unknown command %c\n", buf);
            }
            return -1;
        }
        ret = unsafe {
            send(
                sk,
                &buf as *const c_char as *const c_void,
                core::mem::size_of_val(&buf),
                0,
            ) as c_int
        };
        if ret != 1 {
            unsafe {
                fprintf!(stderr, "%s: Child failed to ack\n", strerror(errno()));
            }
            return -1;
        }
    }
    if ret < 0 {
        unsafe {
            fprintf!(
                stderr,
                "%s: Child failed to read from socket\n",
                strerror(errno())
            );
        }
        return -1;
    }

    0
}

unsafe fn child(sk: c_int) -> c_int {
    let memfd: c_int;
    let ret: c_int;

    memfd = unsafe { sys_memfd_create(c"test".as_ptr(), 0) };
    if memfd < 0 {
        unsafe {
            fprintf!(
                stderr,
                "%s: Child could not create memfd\n",
                strerror(errno())
            );
        }
        ret = -1;
    } else {
        ret = unsafe { __child(sk, memfd) };
        unsafe {
            close(memfd);
        }
    }

    unsafe {
        close(sk);
    }
    ret
}

#[repr(C)]
struct child_fixture {
    /*
     * remote_fd is the number of the FD which we are trying to retrieve
     * from the child.
     */
    remote_fd: c_int,
    /* pid points to the child which we are fetching FDs from */
    pid: pid_t,
    /* pidfd is the pidfd of the child */
    pidfd: c_int,
    /*
     * sk is our side of the socketpair used to communicate with the child.
     * When it is closed, the child will exit.
     */
    sk: c_int,
    ignore_child_result: bool,
}

FIXTURE_SETUP!(child, |self_: *mut child_fixture| unsafe {
    let mut ret: c_int;
    let mut sk_pair: [c_int; 2] = [0; 2];

    ASSERT_EQ!(
        0,
        socketpair(PF_LOCAL, SOCK_SEQPACKET, 0, sk_pair.as_mut_ptr()),
        {
            TH_LOG!("%s: failed to create socketpair", strerror(errno()));
        }
    );
    (*self_).sk = sk_pair[0];

    (*self_).pid = fork();
    ASSERT_GE!((*self_).pid, 0);

    if (*self_).pid == 0 {
        close(sk_pair[0]);
        if child(sk_pair[1]) != 0 {
            libc::_exit(EXIT_FAILURE);
        }
        libc::_exit(EXIT_SUCCESS);
    }

    close(sk_pair[1]);

    (*self_).pidfd = sys_pidfd_open((*self_).pid, 0);
    ASSERT_GE!((*self_).pidfd, 0);

    /*
     * Wait for the child to complete setup. It'll send the remote memfd's
     * number when ready.
     */
    ret = recv(
        sk_pair[0],
        &mut (*self_).remote_fd as *mut c_int as *mut c_void,
        core::mem::size_of_val(&(*self_).remote_fd),
        0,
    ) as c_int;
    ASSERT_EQ!(core::mem::size_of_val(&(*self_).remote_fd) as c_int, ret);
});

FIXTURE_TEARDOWN!(child, |self_: *mut child_fixture| unsafe {
    let ret: c_int;

    EXPECT_EQ!(0, close((*self_).pidfd));
    EXPECT_EQ!(0, close((*self_).sk));

    ret = wait_for_pid((*self_).pid);
    if !(*self_).ignore_child_result {
        EXPECT_EQ!(0, ret);
    }
});

TEST_F!(child, disable_ptrace, |self_: *mut child_fixture| unsafe {
    let uid: c_int;
    let fd: c_int;
    let mut c: c_char = 0;

    /*
     * Turn into nobody if we're root, to avoid CAP_SYS_PTRACE
     *
     * The tests should run in their own process, so even this test fails,
     * it shouldn't result in subsequent tests failing.
     */
    uid = getuid() as c_int;
    if uid == 0 {
        ASSERT_EQ!(0, seteuid(UID_NOBODY as libc::uid_t));
    }

    ASSERT_EQ!(1, send((*self_).sk, c"P".as_ptr() as *const c_void, 1, 0) as c_int);
    ASSERT_EQ!(
        1,
        recv(
            (*self_).sk,
            &mut c as *mut c_char as *mut c_void,
            1,
            0
        ) as c_int
    );

    fd = sys_pidfd_getfd((*self_).pidfd, (*self_).remote_fd, 0);
    EXPECT_EQ!(-1, fd);
    EXPECT_EQ!(EPERM, errno());

    if uid == 0 {
        ASSERT_EQ!(0, seteuid(0));
    }
});

TEST_F!(child, fetch_fd, |self_: *mut child_fixture| unsafe {
    let fd: c_int;
    let mut ret: c_int;

    fd = sys_pidfd_getfd((*self_).pidfd, (*self_).remote_fd, 0);
    ASSERT_GE!(fd, 0);

    ret = sys_kcmp(
        getpid(),
        (*self_).pid,
        KCMP_FILE,
        fd as c_ulong,
        (*self_).remote_fd as c_ulong,
    );
    if ret < 0 && errno() == ENOSYS {
        SKIP!(return, "kcmp() syscall not supported");
    }
    EXPECT_EQ!(ret, 0);

    ret = fcntl(fd, F_GETFD);
    ASSERT_GE!(ret, 0);
    EXPECT_GE!(ret & FD_CLOEXEC, 0);

    close(fd);
});

TEST_F!(child, test_unknown_fd, |self_: *mut child_fixture| unsafe {
    let fd: c_int;

    fd = sys_pidfd_getfd((*self_).pidfd, UNKNOWN_FD, 0);
    EXPECT_EQ!(-1, fd, {
        TH_LOG!("getfd succeeded while fetching unknown fd");
    });
    EXPECT_EQ!(EBADF, errno(), {
        TH_LOG!("%s: getfd did not get EBADF", strerror(errno()));
    });
});

TEST!(flags_set, || unsafe {
    ASSERT_EQ!(-1, sys_pidfd_getfd(0, 0, 1));
    EXPECT_EQ!(errno(), EINVAL);
});

TEST_F!(child, no_strange_EBADF, |self_: *mut child_fixture| unsafe {
    let mut fds: pollfd = core::mem::zeroed();

    (*self_).ignore_child_result = true;

    fds.fd = (*self_).pidfd;
    fds.events = POLLIN;

    ASSERT_EQ!(kill((*self_).pid, SIGKILL), 0);
    ASSERT_EQ!(poll(&mut fds, 1, 5000), 1);

    /*
     * It used to be that pidfd_getfd() could race with the exiting thread
     * between exit_files() and release_task(), and get a non-null task
     * with a NULL files struct, and you'd get EBADF, which was slightly
     * confusing.
     */
    set_errno(0);
    EXPECT_EQ!(sys_pidfd_getfd((*self_).pidfd, (*self_).remote_fd, 0), -1);
    EXPECT_EQ!(errno(), ESRCH);
});

// Original build-time condition:
// #if __NR_pidfd_getfd == -1
#[cfg(pidfd_getfd_unavailable)]
fn main() -> c_int {
    unsafe {
        fprintf!(
            stderr,
            "__NR_pidfd_getfd undefined. The pidfd_getfd syscall is unavailable. Test aborting\n"
        );
    }
    KSFT_SKIP
}

#[cfg(not(pidfd_getfd_unavailable))]
TEST_HARNESS_MAIN!();
