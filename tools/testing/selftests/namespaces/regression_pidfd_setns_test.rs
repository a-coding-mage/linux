// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and included errno.h, sched.h, signal.h, stdio.h,
// stdlib.h, string.h, sys/socket.h, unistd.h, ../pidfd/pidfd.h, and
// ../kselftest_harness.h.

/*
 * Regression tests for the setns(pidfd) active reference counting bug.
 *
 * These tests are based on the reproducers that triggered the race condition
 * fixed by commit 1c465d0518dc ("ns: handle setns(pidfd, ...) cleanly").
 *
 * The bug: When using setns() with a pidfd, if the target task exits between
 * prepare_nsset() and commit_nsset(), the namespaces would become inactive.
 * Then ns_ref_active_get() would increment from 0 without properly resurrecting
 * the owner chain, causing active reference count underflows.
 */

extern "C" {
    fn signal(signum: libc::c_int, handler: libc::sighandler_t) -> libc::sighandler_t;
    fn socketpair(
        domain: libc::c_int,
        type_: libc::c_int,
        protocol: libc::c_int,
        sv: *mut libc::c_int,
    ) -> libc::c_int;
    fn close(fd: libc::c_int) -> libc::c_int;
    fn unshare(flags: libc::c_int) -> libc::c_int;
    fn _exit(status: libc::c_int) -> !;
    fn sleep(seconds: libc::c_uint) -> libc::c_uint;
    fn setns(fd: libc::c_int, nstype: libc::c_int) -> libc::c_int;

    fn create_child(pidfd: *mut libc::c_int, flags: libc::c_int) -> libc::pid_t;
    fn write_nointr(fd: libc::c_int, buf: *const libc::c_void, count: libc::size_t) -> libc::ssize_t;
    fn read_nointr(fd: libc::c_int, buf: *mut libc::c_void, count: libc::size_t) -> libc::ssize_t;
}

/*
 * Simple pidfd setns test using create_child()+unshare().
 *
 * Without the fix, this would trigger active refcount warnings when the
 * parent exits after doing setns(pidfd) on a child that has already exited.
 */
fn simple_pidfd_setns() {
    let child_pid: libc::pid_t;
    let mut pidfd: libc::c_int = -1;
    let mut ret: libc::c_int;
    let mut sv: [libc::c_int; 2] = [0; 2];
    let mut c: libc::c_char = 0;

    unsafe {
        /* Ignore SIGCHLD for autoreap */
        ASSERT_NE!(signal(libc::SIGCHLD, libc::SIG_IGN), libc::SIG_ERR);

        ASSERT_EQ!(
            socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv.as_mut_ptr()),
            0
        );

        /* Create a child process without namespaces initially */
        child_pid = create_child(&mut pidfd, 0);
        ASSERT_GE!(child_pid, 0);

        if child_pid == 0 {
            close(sv[0]);

            if unshare(libc::CLONE_NEWUTS | libc::CLONE_NEWIPC | libc::CLONE_NEWNET | libc::CLONE_NEWUSER) < 0 {
                close(sv[1]);
                _exit(1);
            }

            /* Signal parent that namespaces are ready */
            if write_nointr(sv[1], b"1".as_ptr() as *const libc::c_void, 1) < 0 {
                close(sv[1]);
                _exit(1);
            }

            close(sv[1]);
            _exit(0);
        }
        ASSERT_GE!(pidfd, 0);
        EXPECT_EQ!(close(sv[1]), 0);

        ret = read_nointr(sv[0], &mut c as *mut libc::c_char as *mut libc::c_void, 1) as libc::c_int;
        ASSERT_EQ!(ret, 1);
        EXPECT_EQ!(close(sv[0]), 0);

        /* Set to child's namespaces via pidfd */
        ret = setns(pidfd, libc::CLONE_NEWUTS | libc::CLONE_NEWIPC);
        TH_LOG!("setns() returned %d", ret);
        close(pidfd);
    }
}

/*
 * Simple pidfd setns test using create_child().
 *
 * This variation uses create_child() with namespace flags directly.
 * Namespaces are created immediately at clone time.
 */
fn simple_pidfd_setns_clone() {
    let child_pid: libc::pid_t;
    let mut pidfd: libc::c_int = -1;
    let ret: libc::c_int;

    unsafe {
        /* Ignore SIGCHLD for autoreap */
        ASSERT_NE!(signal(libc::SIGCHLD, libc::SIG_IGN), libc::SIG_ERR);

        /* Create a child process with new namespaces using create_child() */
        child_pid = create_child(
            &mut pidfd,
            libc::CLONE_NEWUSER | libc::CLONE_NEWUTS | libc::CLONE_NEWIPC | libc::CLONE_NEWNET,
        );
        ASSERT_GE!(child_pid, 0);

        if child_pid == 0 {
            /* Child: sleep for a while so parent can setns to us */
            sleep(2);
            _exit(0);
        }

        /* Parent: pidfd was already created by create_child() */
        ASSERT_GE!(pidfd, 0);

        /* Set to child's namespaces via pidfd */
        ret = setns(pidfd, libc::CLONE_NEWUTS | libc::CLONE_NEWIPC);
        close(pidfd);
        TH_LOG!("setns() returned %d", ret);
    }
}

TEST_HARNESS_MAIN!();
