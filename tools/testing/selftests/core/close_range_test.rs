// SPDX-License-Identifier: GPL-2.0

// C dependencies: errno.h, fcntl.h, linux/kernel.h, limits.h, stdbool.h,
// stdio.h, stdlib.h, string.h, syscall.h, unistd.h, sys/resource.h,
// linux/close_range.h, kselftest_harness.h, and clone3_selftests.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const F_LINUX_SPECIFIC_BASE: c_int = 1024;
const F_DUPFD_QUERY: c_int = F_LINUX_SPECIFIC_BASE + 3;
const F_CREATED_QUERY: c_int = F_LINUX_SPECIFIC_BASE + 4;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_CLOEXEC: c_int = 0o2000000;
const F_GETFL: c_int = 3;
const F_GETFD: c_int = 1;
const FD_CLOEXEC: c_int = 1;
const ENOENT: c_int = 2;
const ENOSYS: c_int = 38;
const EINVAL: c_int = 22;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const SIGCHLD: c_ulong = 17;
const CLONE_FILES: u64 = 0x00000400;
const CLOSE_RANGE_UNSHARE: c_uint = 1 << 1;
const CLOSE_RANGE_CLOEXEC: c_uint = 1 << 2;
const RLIMIT_NOFILE: c_int = 7;
const UINT_MAX: c_uint = c_uint::MAX;
const PATH_MAX: usize = 4096;
const __NR_CLOSE_RANGE: c_long = 436;

#[repr(C)]
struct rlimit {
    rlim_cur: c_ulong,
    rlim_max: c_ulong,
}

#[repr(C)]
struct __clone_args {
    flags: u64,
    pidfd: u64,
    child_tid: u64,
    parent_tid: u64,
    exit_signal: u64,
    stack: u64,
    stack_size: u64,
    tls: u64,
    set_tid: u64,
    set_tid_size: u64,
    cgroup: u64,
}

type pid_t = c_int;

unsafe extern "C" {
    static mut errno: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn dup(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn sprintf(str_: *mut c_char, format: *const c_char, ...) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn sys_clone3(args: *mut __clone_args, size: usize) -> pid_t;
}

#[inline]
unsafe fn sys_close_range(fd: c_uint, max_fd: c_uint, flags: c_uint) -> c_int {
    unsafe { syscall(__NR_CLOSE_RANGE, fd, max_fd, flags) as c_int }
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

// TEST(core_close_range)
unsafe fn core_close_range() {
    let mut i: c_int;
    let mut ret: c_int;
    let mut open_fds = [0 as c_int; 101];

    i = 0;
    while i < open_fds.len() as c_int {
        let fd: c_int;

        fd = unsafe { open(c"/dev/null".as_ptr(), O_RDONLY | O_CLOEXEC) };
        ASSERT_GE!(fd, 0, {
            if unsafe { errno } == ENOENT {
                SKIP!(return, "Skipping test since /dev/null does not exist");
            }
        });

        open_fds[i as usize] = fd;
        i += 1;
    }

    EXPECT_EQ!(-1, unsafe { sys_close_range(open_fds[0] as c_uint, open_fds[100] as c_uint, -1_i32 as c_uint) }, {
        if unsafe { errno } == ENOSYS {
            SKIP!(return, "close_range() syscall not supported");
        }
    });

    i = 0;
    while i < 100 {
        ret = unsafe { fcntl(open_fds[i as usize], F_DUPFD_QUERY, open_fds[(i + 1) as usize]) };
        if ret < 0 {
            EXPECT_EQ!(unsafe { errno }, EINVAL);
        } else {
            EXPECT_EQ!(ret, 0);
        }
        i += 1;
    }

    EXPECT_EQ!(0, unsafe { sys_close_range(open_fds[0] as c_uint, open_fds[50] as c_uint, 0) });

    i = 0;
    while i <= 50 {
        EXPECT_EQ!(-1, unsafe { fcntl(open_fds[i as usize], F_GETFL) });
        i += 1;
    }

    i = 51;
    while i <= 100 {
        EXPECT_GT!(unsafe { fcntl(open_fds[i as usize], F_GETFL) }, -1);
        i += 1;
    }

    /* create a couple of gaps */
    unsafe {
        close(57);
        close(78);
        close(81);
        close(82);
        close(84);
        close(90);
    }

    EXPECT_EQ!(0, unsafe { sys_close_range(open_fds[51] as c_uint, open_fds[92] as c_uint, 0) });

    i = 51;
    while i <= 92 {
        EXPECT_EQ!(-1, unsafe { fcntl(open_fds[i as usize], F_GETFL) });
        i += 1;
    }

    i = 93;
    while i <= 100 {
        EXPECT_GT!(unsafe { fcntl(open_fds[i as usize], F_GETFL) }, -1);
        i += 1;
    }

    /* test that the kernel caps and still closes all fds */
    EXPECT_EQ!(0, unsafe { sys_close_range(open_fds[93] as c_uint, open_fds[99] as c_uint, 0) });

    i = 93;
    while i <= 99 {
        EXPECT_EQ!(-1, unsafe { fcntl(open_fds[i as usize], F_GETFL) });
        i += 1;
    }

    EXPECT_GT!(unsafe { fcntl(open_fds[i as usize], F_GETFL) }, -1);

    EXPECT_EQ!(0, unsafe { sys_close_range(open_fds[100] as c_uint, open_fds[100] as c_uint, 0) });

    EXPECT_EQ!(-1, unsafe { fcntl(open_fds[100], F_GETFL) });
}

// TEST(close_range_unshare)
unsafe fn close_range_unshare() {
    let mut i: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let pid: pid_t;
    let mut open_fds = [0 as c_int; 101];
    let mut args = __clone_args {
        flags: CLONE_FILES,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    i = 0;
    while i < open_fds.len() as c_int {
        let fd = unsafe { open(c"/dev/null".as_ptr(), O_RDONLY | O_CLOEXEC) };
        ASSERT_GE!(fd, 0, {
            if unsafe { errno } == ENOENT {
                SKIP!(return, "Skipping test since /dev/null does not exist");
            }
        });
        open_fds[i as usize] = fd;
        i += 1;
    }

    pid = unsafe { sys_clone3(&mut args, core::mem::size_of_val(&args)) };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        ret = unsafe { sys_close_range(open_fds[0] as c_uint, open_fds[50] as c_uint, CLOSE_RANGE_UNSHARE) };
        if ret != 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        i = 0;
        while i <= 50 {
            if unsafe { fcntl(open_fds[i as usize], F_GETFL) } != -1 {
                unsafe { exit(EXIT_FAILURE) };
            }
            i += 1;
        }

        i = 51;
        while i <= 100 {
            if unsafe { fcntl(open_fds[i as usize], F_GETFL) } == -1 {
                unsafe { exit(EXIT_FAILURE) };
            }
            i += 1;
        }

        /* create a couple of gaps */
        unsafe {
            close(57);
            close(78);
            close(81);
            close(82);
            close(84);
            close(90);
        }

        ret = unsafe { sys_close_range(open_fds[51] as c_uint, open_fds[92] as c_uint, CLOSE_RANGE_UNSHARE) };
        if ret != 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        i = 51;
        while i <= 92 {
            if unsafe { fcntl(open_fds[i as usize], F_GETFL) } != -1 {
                unsafe { exit(EXIT_FAILURE) };
            }
            i += 1;
        }

        i = 93;
        while i <= 100 {
            if unsafe { fcntl(open_fds[i as usize], F_GETFL) } == -1 {
                unsafe { exit(EXIT_FAILURE) };
            }
            i += 1;
        }

        /* test that the kernel caps and still closes all fds */
        ret = unsafe { sys_close_range(open_fds[93] as c_uint, open_fds[99] as c_uint, CLOSE_RANGE_UNSHARE) };
        if ret != 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        i = 93;
        while i <= 99 {
            if unsafe { fcntl(open_fds[i as usize], F_GETFL) } != -1 {
                unsafe { exit(EXIT_FAILURE) };
            }
            i += 1;
        }

        if unsafe { fcntl(open_fds[100], F_GETFL) } == -1 {
            unsafe { exit(EXIT_FAILURE) };
        }

        ret = unsafe { sys_close_range(open_fds[100] as c_uint, open_fds[100] as c_uint, CLOSE_RANGE_UNSHARE) };
        if ret != 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        if unsafe { fcntl(open_fds[100], F_GETFL) } != -1 {
            unsafe { exit(EXIT_FAILURE) };
        }

        unsafe { exit(EXIT_SUCCESS) };
    }

    EXPECT_EQ!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    EXPECT_EQ!(true, WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));
}

// TEST(close_range_unshare_capped)
unsafe fn close_range_unshare_capped() {
    let mut i: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let pid: pid_t;
    let mut open_fds = [0 as c_int; 101];
    let mut args = __clone_args {
        flags: CLONE_FILES,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    i = 0;
    while i < open_fds.len() as c_int {
        let fd = unsafe { open(c"/dev/null".as_ptr(), O_RDONLY | O_CLOEXEC) };
        ASSERT_GE!(fd, 0, {
            if unsafe { errno } == ENOENT {
                SKIP!(return, "Skipping test since /dev/null does not exist");
            }
        });
        open_fds[i as usize] = fd;
        i += 1;
    }

    pid = unsafe { sys_clone3(&mut args, core::mem::size_of_val(&args)) };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        ret = unsafe { sys_close_range(open_fds[0] as c_uint, UINT_MAX, CLOSE_RANGE_UNSHARE) };
        if ret != 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        i = 0;
        while i <= 100 {
            if unsafe { fcntl(open_fds[i as usize], F_GETFL) } != -1 {
                unsafe { exit(EXIT_FAILURE) };
            }
            i += 1;
        }

        unsafe { exit(EXIT_SUCCESS) };
    }

    EXPECT_EQ!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    EXPECT_EQ!(true, WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));
}

// TEST(close_range_cloexec)
unsafe fn close_range_cloexec() {
    let mut i: c_int;
    let mut ret: c_int;
    let mut open_fds = [0 as c_int; 101];
    let mut rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };

    i = 0;
    while i < open_fds.len() as c_int {
        let fd = unsafe { open(c"/dev/null".as_ptr(), O_RDONLY) };
        ASSERT_GE!(fd, 0, {
            if unsafe { errno } == ENOENT {
                SKIP!(return, "Skipping test since /dev/null does not exist");
            }
        });
        open_fds[i as usize] = fd;
        i += 1;
    }

    ret = unsafe { sys_close_range(1000, 1000, CLOSE_RANGE_CLOEXEC) };
    if ret < 0 {
        if unsafe { errno } == ENOSYS {
            SKIP!(return, "close_range() syscall not supported");
        }
        if unsafe { errno } == EINVAL {
            SKIP!(return, "close_range() doesn't support CLOSE_RANGE_CLOEXEC");
        }
    }

    /* Ensure the FD_CLOEXEC bit is set also with a resource limit in place.  */
    ASSERT_EQ!(0, unsafe { getrlimit(RLIMIT_NOFILE, &mut rlimit) });
    rlimit.rlim_cur = 25;
    ASSERT_EQ!(0, unsafe { setrlimit(RLIMIT_NOFILE, &rlimit) });

    /* Set close-on-exec for two ranges: [0-50] and [75-100].  */
    ret = unsafe { sys_close_range(open_fds[0] as c_uint, open_fds[50] as c_uint, CLOSE_RANGE_CLOEXEC) };
    ASSERT_EQ!(0, ret);
    ret = unsafe { sys_close_range(open_fds[75] as c_uint, open_fds[100] as c_uint, CLOSE_RANGE_CLOEXEC) };
    ASSERT_EQ!(0, ret);

    i = 0;
    while i <= 50 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);
        i += 1;
    }

    i = 51;
    while i <= 74 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, 0);
        i += 1;
    }

    i = 75;
    while i <= 100 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);
        i += 1;
    }

    /* Test a common pattern.  */
    ret = unsafe { sys_close_range(3, UINT_MAX, CLOSE_RANGE_CLOEXEC) };
    i = 0;
    while i <= 100 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);
        i += 1;
    }
}

// TEST(close_range_cloexec_unshare)
unsafe fn close_range_cloexec_unshare() {
    let mut i: c_int;
    let mut ret: c_int;
    let mut open_fds = [0 as c_int; 101];
    let mut rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };

    i = 0;
    while i < open_fds.len() as c_int {
        let fd = unsafe { open(c"/dev/null".as_ptr(), O_RDONLY) };
        ASSERT_GE!(fd, 0, {
            if unsafe { errno } == ENOENT {
                SKIP!(return, "Skipping test since /dev/null does not exist");
            }
        });
        open_fds[i as usize] = fd;
        i += 1;
    }

    ret = unsafe { sys_close_range(1000, 1000, CLOSE_RANGE_CLOEXEC) };
    if ret < 0 {
        if unsafe { errno } == ENOSYS {
            SKIP!(return, "close_range() syscall not supported");
        }
        if unsafe { errno } == EINVAL {
            SKIP!(return, "close_range() doesn't support CLOSE_RANGE_CLOEXEC");
        }
    }

    /* Ensure the FD_CLOEXEC bit is set also with a resource limit in place.  */
    ASSERT_EQ!(0, unsafe { getrlimit(RLIMIT_NOFILE, &mut rlimit) });
    rlimit.rlim_cur = 25;
    ASSERT_EQ!(0, unsafe { setrlimit(RLIMIT_NOFILE, &rlimit) });

    /* Set close-on-exec for two ranges: [0-50] and [75-100].  */
    ret = unsafe { sys_close_range(open_fds[0] as c_uint, open_fds[50] as c_uint, CLOSE_RANGE_CLOEXEC | CLOSE_RANGE_UNSHARE) };
    ASSERT_EQ!(0, ret);
    ret = unsafe { sys_close_range(open_fds[75] as c_uint, open_fds[100] as c_uint, CLOSE_RANGE_CLOEXEC | CLOSE_RANGE_UNSHARE) };
    ASSERT_EQ!(0, ret);

    i = 0;
    while i <= 50 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);
        i += 1;
    }

    i = 51;
    while i <= 74 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, 0);
        i += 1;
    }

    i = 75;
    while i <= 100 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);
        i += 1;
    }

    /* Test a common pattern.  */
    ret = unsafe { sys_close_range(3, UINT_MAX, CLOSE_RANGE_CLOEXEC | CLOSE_RANGE_UNSHARE) };
    i = 0;
    while i <= 100 {
        let flags = unsafe { fcntl(open_fds[i as usize], F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);
        i += 1;
    }
}

/*
 * Regression test for syzbot+96cfd2b22b3213646a93@syzkaller.appspotmail.com
 */
// TEST(close_range_cloexec_syzbot)
unsafe fn close_range_cloexec_syzbot() {
    let mut fd1: c_int;
    let mut fd2: c_int;
    let mut fd3: c_int;
    let mut fd4: c_int;
    let mut flags: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let pid: pid_t;
    let mut args = __clone_args {
        flags: CLONE_FILES,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    /* Create a huge gap in the fd table. */
    fd1 = unsafe { open(c"/dev/null".as_ptr(), O_RDWR) };
    EXPECT_GT!(fd1, 0);

    fd2 = unsafe { dup2(fd1, 1000) };
    EXPECT_GT!(fd2, 0);

    flags = unsafe { fcntl(fd1, F_DUPFD_QUERY, fd2) };
    if flags < 0 {
        EXPECT_EQ!(unsafe { errno }, EINVAL);
    } else {
        EXPECT_EQ!(flags, 1);
    }

    pid = unsafe { sys_clone3(&mut args, core::mem::size_of_val(&args)) };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        ret = unsafe { sys_close_range(3, !0_u32, CLOSE_RANGE_CLOEXEC) };
        if ret != 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        /*
         * We now have a private file descriptor table and all
         * our open fds should still be open but made
         * close-on-exec.
         */
        flags = unsafe { fcntl(fd1, F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);

        flags = unsafe { fcntl(fd2, F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);

        fd3 = unsafe { dup2(fd1, 42) };
        EXPECT_GT!(fd3, 0);

        flags = unsafe { fcntl(fd1, F_DUPFD_QUERY, fd3) };
        if flags < 0 {
            EXPECT_EQ!(unsafe { errno }, EINVAL);
        } else {
            EXPECT_EQ!(flags, 1);
        }

        /*
         * Duplicating the file descriptor must remove the
         * FD_CLOEXEC flag.
         */
        flags = unsafe { fcntl(fd3, F_GETFD) };
        EXPECT_GT!(flags, -1);
        EXPECT_EQ!(flags & FD_CLOEXEC, 0);

        unsafe { exit(EXIT_SUCCESS) };
    }

    EXPECT_EQ!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    EXPECT_EQ!(true, WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));

    /*
     * We had a shared file descriptor table before along with requesting
     * close-on-exec so the original fds must not be close-on-exec.
     */
    flags = unsafe { fcntl(fd1, F_GETFD) };
    EXPECT_GT!(flags, -1);
    EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);

    flags = unsafe { fcntl(fd2, F_GETFD) };
    EXPECT_GT!(flags, -1);
    EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);

    fd3 = unsafe { dup2(fd1, 42) };
    EXPECT_GT!(fd3, 0);

    flags = unsafe { fcntl(fd1, F_DUPFD_QUERY, fd3) };
    if flags < 0 {
        EXPECT_EQ!(unsafe { errno }, EINVAL);
    } else {
        EXPECT_EQ!(flags, 1);
    }

    fd4 = unsafe { open(c"/dev/null".as_ptr(), O_RDWR) };
    EXPECT_GT!(fd4, 0);

    /* Same inode, different file pointers. */
    flags = unsafe { fcntl(fd1, F_DUPFD_QUERY, fd4) };
    if flags < 0 {
        EXPECT_EQ!(unsafe { errno }, EINVAL);
    } else {
        EXPECT_EQ!(flags, 0);
    }

    flags = unsafe { fcntl(fd3, F_GETFD) };
    EXPECT_GT!(flags, -1);
    EXPECT_EQ!(flags & FD_CLOEXEC, 0);

    EXPECT_EQ!(unsafe { close(fd1) }, 0);
    EXPECT_EQ!(unsafe { close(fd2) }, 0);
    EXPECT_EQ!(unsafe { close(fd3) }, 0);
    EXPECT_EQ!(unsafe { close(fd4) }, 0);
}

/*
 * Regression test for syzbot+96cfd2b22b3213646a93@syzkaller.appspotmail.com
 */
// TEST(close_range_cloexec_unshare_syzbot)
unsafe fn close_range_cloexec_unshare_syzbot() {
    let mut i: c_int;
    let mut fd1: c_int;
    let mut fd2: c_int;
    let mut fd3: c_int;
    let mut flags: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let pid: pid_t;
    let mut args = __clone_args {
        flags: CLONE_FILES,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    /*
     * Create a huge gap in the fd table. When we now call
     * CLOSE_RANGE_UNSHARE with a shared fd table and and with ~0U as upper
     * bound the kernel will only copy up to fd1 file descriptors into the
     * new fd table. If the kernel is buggy and doesn't handle
     * CLOSE_RANGE_CLOEXEC correctly it will not have copied all file
     * descriptors and we will oops!
     *
     * On a buggy kernel this should immediately oops. But let's loop just
     * to be sure.
     */
    fd1 = unsafe { open(c"/dev/null".as_ptr(), O_RDWR) };
    EXPECT_GT!(fd1, 0);

    fd2 = unsafe { dup2(fd1, 1000) };
    EXPECT_GT!(fd2, 0);

    i = 0;
    while i < 100 {
        pid = unsafe { sys_clone3(&mut args, core::mem::size_of_val(&args)) };
        ASSERT_GE!(pid, 0);

        if pid == 0 {
            ret = unsafe { sys_close_range(3, !0_u32, CLOSE_RANGE_UNSHARE | CLOSE_RANGE_CLOEXEC) };
            if ret != 0 {
                unsafe { exit(EXIT_FAILURE) };
            }

            /*
             * We now have a private file descriptor table and all
             * our open fds should still be open but made
             * close-on-exec.
             */
            flags = unsafe { fcntl(fd1, F_GETFD) };
            EXPECT_GT!(flags, -1);
            EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);

            flags = unsafe { fcntl(fd2, F_GETFD) };
            EXPECT_GT!(flags, -1);
            EXPECT_EQ!(flags & FD_CLOEXEC, FD_CLOEXEC);

            fd3 = unsafe { dup2(fd1, 42) };
            EXPECT_GT!(fd3, 0);

            /*
             * Duplicating the file descriptor must remove the
             * FD_CLOEXEC flag.
             */
            flags = unsafe { fcntl(fd3, F_GETFD) };
            EXPECT_GT!(flags, -1);
            EXPECT_EQ!(flags & FD_CLOEXEC, 0);

            EXPECT_EQ!(unsafe { close(fd1) }, 0);
            EXPECT_EQ!(unsafe { close(fd2) }, 0);
            EXPECT_EQ!(unsafe { close(fd3) }, 0);

            unsafe { exit(EXIT_SUCCESS) };
        }

        EXPECT_EQ!(unsafe { waitpid(pid, &mut status, 0) }, pid);
        EXPECT_EQ!(true, WIFEXITED(status));
        EXPECT_EQ!(0, WEXITSTATUS(status));
        i += 1;
    }

    /*
     * We created a private file descriptor table before along with
     * requesting close-on-exec so the original fds must not be
     * close-on-exec.
     */
    flags = unsafe { fcntl(fd1, F_GETFD) };
    EXPECT_GT!(flags, -1);
    EXPECT_EQ!(flags & FD_CLOEXEC, 0);

    flags = unsafe { fcntl(fd2, F_GETFD) };
    EXPECT_GT!(flags, -1);
    EXPECT_EQ!(flags & FD_CLOEXEC, 0);

    fd3 = unsafe { dup2(fd1, 42) };
    EXPECT_GT!(fd3, 0);

    flags = unsafe { fcntl(fd3, F_GETFD) };
    EXPECT_GT!(flags, -1);
    EXPECT_EQ!(flags & FD_CLOEXEC, 0);

    EXPECT_EQ!(unsafe { close(fd1) }, 0);
    EXPECT_EQ!(unsafe { close(fd2) }, 0);
    EXPECT_EQ!(unsafe { close(fd3) }, 0);
}

// TEST(close_range_bitmap_corruption)
unsafe fn close_range_bitmap_corruption() {
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut args = __clone_args {
        flags: CLONE_FILES,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    /* get the first 128 descriptors open */
    let mut i: c_int = 2;
    while i < 128 {
        EXPECT_GE!(unsafe { dup2(0, i) }, 0);
        i += 1;
    }

    /* get descriptor table shared */
    pid = unsafe { sys_clone3(&mut args, core::mem::size_of_val(&args)) };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* unshare and truncate descriptor table down to 64 */
        if unsafe { sys_close_range(64, !0_u32, CLOSE_RANGE_UNSHARE) } != 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        ASSERT_EQ!(unsafe { fcntl(64, F_GETFD) }, -1);
        /* ... and verify that the range 64..127 is not
           stuck "fully used" according to secondary bitmap */
        EXPECT_EQ!(unsafe { dup(0) }, 64, {
            unsafe { exit(EXIT_FAILURE) };
        });
        unsafe { exit(EXIT_SUCCESS) };
    }

    EXPECT_EQ!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    EXPECT_EQ!(true, WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));
}

// TEST(fcntl_created)
unsafe fn fcntl_created() {
    let mut i: c_int = 0;
    while i < 101 {
        let mut fd: c_int;
        let mut path = [0 as c_char; PATH_MAX];

        fd = unsafe { open(c"/dev/null".as_ptr(), O_RDONLY | O_CLOEXEC) };
        ASSERT_GE!(fd, 0, {
            if unsafe { errno } == ENOENT {
                SKIP!(return, "Skipping test since /dev/null does not exist");
            }
        });

        /* We didn't create "/dev/null". */
        EXPECT_EQ!(unsafe { fcntl(fd, F_CREATED_QUERY, 0) }, 0);
        unsafe { close(fd) };

        unsafe { sprintf(path.as_mut_ptr(), c"aaaa_%d".as_ptr(), i) };
        fd = unsafe { open(path.as_ptr(), O_CREAT | O_RDONLY | O_CLOEXEC, 0o600) };
        ASSERT_GE!(fd, 0);

        /* We created "aaaa_%d". */
        EXPECT_EQ!(unsafe { fcntl(fd, F_CREATED_QUERY, 0) }, 1);
        unsafe { close(fd) };

        fd = unsafe { open(path.as_ptr(), O_RDONLY | O_CLOEXEC) };
        ASSERT_GE!(fd, 0);

        /* We're opening it again, so no positive creation check. */
        EXPECT_EQ!(unsafe { fcntl(fd, F_CREATED_QUERY, 0) }, 0);
        unsafe { close(fd) };
        unsafe { unlink(path.as_ptr()) };

        i += 1;
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
