// SPDX-License-Identifier: GPL-2.0

// C dependencies: errno.h, fcntl.h, limits.h, linux/types.h, poll.h,
// pthread.h, sched.h, signal.h, stdio.h, stdlib.h, string.h, syscall.h,
// sys/prctl.h, sys/wait.h, unistd.h, sys/socket.h, linux/kcmp.h, sys/stat.h,
// sys/xattr.h, "pidfd.h", and "kselftest_harness.h".

const CLONE_NEWUSER: libc::c_int = 0x10000000;
const CLONE_NEWPID: libc::c_int = 0x20000000;
const EXIT_SUCCESS: libc::c_int = 0;
const P_PID: libc::c_int = 1;
const WEXITED: libc::c_int = 0x00000004;
const EOPNOTSUPP: libc::c_int = 95;
const EACCES: libc::c_int = 13;
const AT_EMPTY_PATH: libc::c_int = 0x1000;

extern "C" {
    fn create_child(pidfd: *mut libc::c_int, flags: libc::c_int) -> libc::pid_t;
    fn sys_waitid(
        which: libc::c_int,
        pid: libc::pid_t,
        infop: *mut libc::siginfo_t,
        options: libc::c_int,
    ) -> libc::c_int;
    fn execveat(
        dirfd: libc::c_int,
        pathname: *const libc::c_char,
        argv: *const *mut libc::c_char,
        envp: *const *mut libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
}

#[repr(C)]
struct pidfs_setattr {
    child_pid: libc::pid_t,
    child_pidfd: libc::c_int,
}

unsafe fn errno() -> libc::c_int {
    *libc::__errno_location()
}

unsafe fn expect_ge<T>(left: T, right: T)
where
    T: PartialOrd + std::fmt::Debug,
{
    assert!(left >= right, "expected {:?} >= {:?}", left, right);
}

unsafe fn expect_eq<T>(left: T, right: T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_eq!(left, right);
}

unsafe fn assert_lt<T>(left: T, right: T)
where
    T: PartialOrd + std::fmt::Debug,
{
    assert!(left < right, "expected {:?} < {:?}", left, right);
}

unsafe fn assert_eq_value<T>(left: T, right: T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_eq!(left, right);
}

// FIXTURE_SETUP(pidfs_setattr)
unsafe fn pidfs_setattr_setup(self_: *mut pidfs_setattr) {
    (*self_).child_pid = create_child(
        &mut (*self_).child_pidfd,
        CLONE_NEWUSER | CLONE_NEWPID,
    );
    expect_ge((*self_).child_pid, 0);

    if (*self_).child_pid == 0 {
        libc::_exit(EXIT_SUCCESS);
    }
}

// FIXTURE_TEARDOWN(pidfs_setattr)
unsafe fn pidfs_setattr_teardown(self_: *mut pidfs_setattr) {
    sys_waitid(
        P_PID,
        (*self_).child_pid,
        std::ptr::null_mut(),
        WEXITED,
    );
    expect_eq(libc::close((*self_).child_pidfd), 0);
}

// TEST_F(pidfs_setattr, no_chown)
unsafe fn no_chown(self_: *mut pidfs_setattr) {
    assert_lt(libc::fchown((*self_).child_pidfd, 1234, 5678), 0);
    assert_eq_value(errno(), EOPNOTSUPP);
}

// TEST_F(pidfs_setattr, no_chmod)
unsafe fn no_chmod(self_: *mut pidfs_setattr) {
    assert_lt(libc::fchmod((*self_).child_pidfd, 0o777), 0);
    assert_eq_value(errno(), EOPNOTSUPP);
}

// TEST_F(pidfs_setattr, no_exec)
unsafe fn no_exec(self_: *mut pidfs_setattr) {
    let argv: [*mut libc::c_char; 1] = [std::ptr::null_mut()];
    let envp: [*mut libc::c_char; 1] = [std::ptr::null_mut()];

    assert_lt(
        execveat(
            (*self_).child_pidfd,
            b"\0".as_ptr() as *const libc::c_char,
            argv.as_ptr(),
            envp.as_ptr(),
            AT_EMPTY_PATH,
        ),
        0,
    );
    assert_eq_value(errno(), EACCES);
}

// TEST_HARNESS_MAIN
