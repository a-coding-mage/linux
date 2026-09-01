// SPDX-License-Identifier: GPL-2.0

// Translated from C. External constants, syscalls, helpers, and kselftest
// harness macros are expected to be supplied by the surrounding test tree.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type pid_t = c_int;

const PIDFD_NS_USER: usize = 0;
const PIDFD_NS_MNT: usize = 1;
const PIDFD_NS_PID: usize = 2;
const PIDFD_NS_UTS: usize = 3;
const PIDFD_NS_IPC: usize = 4;
const PIDFD_NS_NET: usize = 5;
const PIDFD_NS_CGROUP: usize = 6;
const PIDFD_NS_PIDCLD: usize = 7;
const PIDFD_NS_TIME: usize = 8;
const PIDFD_NS_TIMECLD: usize = 9;
const PIDFD_NS_MAX: usize = 10;

#[repr(C)]
struct ns_info {
    name: *const c_char,
    flag: c_int,
    pidfd_ioctl: c_uint,
}

static ns_info: [ns_info; PIDFD_NS_MAX] = [
    ns_info { name: b"user\0".as_ptr() as *const c_char, flag: CLONE_NEWUSER, pidfd_ioctl: PIDFD_GET_USER_NAMESPACE },
    ns_info { name: b"mnt\0".as_ptr() as *const c_char, flag: CLONE_NEWNS, pidfd_ioctl: PIDFD_GET_MNT_NAMESPACE },
    ns_info { name: b"pid\0".as_ptr() as *const c_char, flag: CLONE_NEWPID, pidfd_ioctl: PIDFD_GET_PID_NAMESPACE },
    ns_info { name: b"uts\0".as_ptr() as *const c_char, flag: CLONE_NEWUTS, pidfd_ioctl: PIDFD_GET_UTS_NAMESPACE },
    ns_info { name: b"ipc\0".as_ptr() as *const c_char, flag: CLONE_NEWIPC, pidfd_ioctl: PIDFD_GET_IPC_NAMESPACE },
    ns_info { name: b"net\0".as_ptr() as *const c_char, flag: CLONE_NEWNET, pidfd_ioctl: PIDFD_GET_NET_NAMESPACE },
    ns_info { name: b"cgroup\0".as_ptr() as *const c_char, flag: CLONE_NEWCGROUP, pidfd_ioctl: PIDFD_GET_CGROUP_NAMESPACE },
    ns_info { name: b"pid_for_children\0".as_ptr() as *const c_char, flag: 0, pidfd_ioctl: PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE },
    ns_info { name: b"time\0".as_ptr() as *const c_char, flag: CLONE_NEWTIME, pidfd_ioctl: PIDFD_GET_TIME_NAMESPACE },
    ns_info { name: b"time_for_children\0".as_ptr() as *const c_char, flag: 0, pidfd_ioctl: PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE },
];

#[repr(C)]
struct current_nsset {
    pid: pid_t,
    pidfd: c_int,
    nsfds: [c_int; PIDFD_NS_MAX],
    child_pidfd_derived_nsfds: [c_int; PIDFD_NS_MAX],

    child_pid_exited: pid_t,
    child_pidfd_exited: c_int,

    child_pid1: pid_t,
    child_pidfd1: c_int,
    child_nsfds1: [c_int; PIDFD_NS_MAX],
    child_pidfd_derived_nsfds1: [c_int; PIDFD_NS_MAX],

    child_pid2: pid_t,
    child_pidfd2: c_int,
    child_nsfds2: [c_int; PIDFD_NS_MAX],
    child_pidfd_derived_nsfds2: [c_int; PIDFD_NS_MAX],
}

unsafe fn switch_timens() -> bool {
    let fd: c_int;
    let ret: c_int;

    if unshare(CLONE_NEWTIME) != 0 {
        return false;
    }

    fd = open(b"/proc/self/ns/time_for_children\0".as_ptr() as *const c_char, O_RDONLY | O_CLOEXEC);
    if fd < 0 {
        return false;
    }

    ret = setns(fd, CLONE_NEWTIME);
    close(fd);
    ret == 0
}

fixture_setup!(current_nsset, |self_: *mut current_nsset| unsafe {
    let mut i: c_int;
    let proc_fd: c_int;
    let mut ret: c_int;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;
    let self_ = &mut *self_;

    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        self_.nsfds[i as usize] = -EBADF;
        self_.child_nsfds1[i as usize] = -EBADF;
        self_.child_nsfds2[i as usize] = -EBADF;
        self_.child_pidfd_derived_nsfds[i as usize] = -EBADF;
        self_.child_pidfd_derived_nsfds1[i as usize] = -EBADF;
        self_.child_pidfd_derived_nsfds2[i as usize] = -EBADF;
        i += 1;
    }

    proc_fd = open(b"/proc/self/ns\0".as_ptr() as *const c_char, O_DIRECTORY | O_CLOEXEC);
    ASSERT_GE!(proc_fd, 0, {
        TH_LOG!("%m - Failed to open /proc/self/ns");
    });

    self_.pid = getpid();
    self_.pidfd = sys_pidfd_open(self_.pid, 0);
    EXPECT_GT!(self_.pidfd, 0, {
        TH_LOG!("%m - Failed to open pidfd for process %d", self_.pid);
    });

    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];
        self_.nsfds[i as usize] = openat(proc_fd, info.name, O_RDONLY | O_CLOEXEC);
        if self_.nsfds[i as usize] < 0 {
            EXPECT_EQ!(errno(), ENOENT, {
                TH_LOG!("%m - Failed to open %s namespace for process %d", info.name, self_.pid);
            });
        }

        self_.child_pidfd_derived_nsfds[i as usize] = ioctl(self_.pidfd, info.pidfd_ioctl, 0);
        if self_.child_pidfd_derived_nsfds[i as usize] < 0 {
            EXPECT_EQ!(errno(), EOPNOTSUPP, {
                TH_LOG!("%m - Failed to derive %s namespace from pidfd of process %d", info.name, self_.pid);
            });
        }
        i += 1;
    }

    /* Create task that exits right away. */
    self_.child_pid_exited = create_child(&mut self_.child_pidfd_exited, 0);
    EXPECT_GE!(self_.child_pid_exited, 0);

    if self_.child_pid_exited == 0 {
        if self_.nsfds[PIDFD_NS_USER] >= 0 && unshare(CLONE_NEWUSER) < 0 {
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_NET] >= 0 && unshare(CLONE_NEWNET) < 0 {
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }

    ASSERT_EQ!(sys_waitid(P_PID, self_.child_pid_exited, ptr::null_mut(), WEXITED | WNOWAIT), 0);

    self_.pidfd = sys_pidfd_open(self_.pid, 0);
    EXPECT_GE!(self_.pidfd, 0, {
        TH_LOG!("%m - Failed to open pidfd for process %d", self_.pid);
    });

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    /* Create tasks that will be stopped. */
    if self_.nsfds[PIDFD_NS_USER] >= 0 && self_.nsfds[PIDFD_NS_PID] >= 0 {
        self_.child_pid1 = create_child(&mut self_.child_pidfd1, CLONE_NEWUSER | CLONE_NEWPID);
    } else if self_.nsfds[PIDFD_NS_PID] >= 0 {
        self_.child_pid1 = create_child(&mut self_.child_pidfd1, CLONE_NEWPID);
    } else if self_.nsfds[PIDFD_NS_USER] >= 0 {
        self_.child_pid1 = create_child(&mut self_.child_pidfd1, CLONE_NEWUSER);
    } else {
        self_.child_pid1 = create_child(&mut self_.child_pidfd1, 0);
    }
    EXPECT_GE!(self_.child_pid1, 0);

    if self_.child_pid1 == 0 {
        close(ipc_sockets[0]);

        if self_.nsfds[PIDFD_NS_MNT] >= 0 && unshare(CLONE_NEWNS) < 0 {
            TH_LOG!("%m - Failed to unshare mount namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_CGROUP] >= 0 && unshare(CLONE_NEWCGROUP) < 0 {
            TH_LOG!("%m - Failed to unshare cgroup namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_IPC] >= 0 && unshare(CLONE_NEWIPC) < 0 {
            TH_LOG!("%m - Failed to unshare ipc namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_UTS] >= 0 && unshare(CLONE_NEWUTS) < 0 {
            TH_LOG!("%m - Failed to unshare uts namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_NET] >= 0 && unshare(CLONE_NEWNET) < 0 {
            TH_LOG!("%m - Failed to unshare net namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_TIME] >= 0 && !switch_timens() {
            TH_LOG!("%m - Failed to unshare time namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }

        if write_nointr(ipc_sockets[1], b"1\0".as_ptr() as *const c_void, 1) < 0 {
            _exit(EXIT_FAILURE);
        }

        close(ipc_sockets[1]);

        pause();
        _exit(EXIT_SUCCESS);
    }

    close(ipc_sockets[1]);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    close(ipc_sockets[0]);

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    if self_.nsfds[PIDFD_NS_USER] >= 0 && self_.nsfds[PIDFD_NS_PID] >= 0 {
        self_.child_pid2 = create_child(&mut self_.child_pidfd2, CLONE_NEWUSER | CLONE_NEWPID);
    } else if self_.nsfds[PIDFD_NS_PID] >= 0 {
        self_.child_pid2 = create_child(&mut self_.child_pidfd2, CLONE_NEWPID);
    } else if self_.nsfds[PIDFD_NS_USER] >= 0 {
        self_.child_pid2 = create_child(&mut self_.child_pidfd2, CLONE_NEWUSER);
    } else {
        self_.child_pid2 = create_child(&mut self_.child_pidfd2, 0);
    }
    EXPECT_GE!(self_.child_pid2, 0);

    if self_.child_pid2 == 0 {
        close(ipc_sockets[0]);

        if self_.nsfds[PIDFD_NS_MNT] >= 0 && unshare(CLONE_NEWNS) < 0 {
            TH_LOG!("%m - Failed to unshare mount namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_CGROUP] >= 0 && unshare(CLONE_NEWCGROUP) < 0 {
            TH_LOG!("%m - Failed to unshare cgroup namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_IPC] >= 0 && unshare(CLONE_NEWIPC) < 0 {
            TH_LOG!("%m - Failed to unshare ipc namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_UTS] >= 0 && unshare(CLONE_NEWUTS) < 0 {
            TH_LOG!("%m - Failed to unshare uts namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_NET] >= 0 && unshare(CLONE_NEWNET) < 0 {
            TH_LOG!("%m - Failed to unshare net namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }
        if self_.nsfds[PIDFD_NS_TIME] >= 0 && !switch_timens() {
            TH_LOG!("%m - Failed to unshare time namespace for process %d", self_.pid);
            _exit(EXIT_FAILURE);
        }

        if write_nointr(ipc_sockets[1], b"1\0".as_ptr() as *const c_void, 1) < 0 {
            _exit(EXIT_FAILURE);
        }

        close(ipc_sockets[1]);

        pause();
        _exit(EXIT_SUCCESS);
    }

    close(ipc_sockets[1]);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    close(ipc_sockets[0]);

    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let mut p: [c_char; 100] = [0; 100];
        let info = &ns_info[i as usize];

        self_.nsfds[i as usize] = openat(proc_fd, info.name, O_RDONLY | O_CLOEXEC);
        if self_.nsfds[i as usize] < 0 {
            EXPECT_EQ!(errno(), ENOENT, {
                TH_LOG!("%m - Failed to open %s namespace for process %d", info.name, self_.pid);
            });
        }

        ret = snprintf(p.as_mut_ptr(), p.len(), b"/proc/%d/ns/%s\0".as_ptr() as *const c_char, self_.child_pid1, info.name);
        EXPECT_GT!(ret, 0);
        EXPECT_LT!(ret as usize, p.len());

        self_.child_nsfds1[i as usize] = open(p.as_ptr(), O_RDONLY | O_CLOEXEC);
        if self_.child_nsfds1[i as usize] < 0 {
            EXPECT_EQ!(errno(), ENOENT, {
                TH_LOG!("%m - Failed to open %s namespace for process %d", info.name, self_.child_pid1);
            });
        }

        ret = snprintf(p.as_mut_ptr(), p.len(), b"/proc/%d/ns/%s\0".as_ptr() as *const c_char, self_.child_pid2, info.name);
        EXPECT_GT!(ret, 0);
        EXPECT_LT!(ret as usize, p.len());

        self_.child_nsfds2[i as usize] = open(p.as_ptr(), O_RDONLY | O_CLOEXEC);
        if self_.child_nsfds2[i as usize] < 0 {
            EXPECT_EQ!(errno(), ENOENT, {
                TH_LOG!("%m - Failed to open %s namespace for process %d", info.name, self_.child_pid1);
            });
        }

        self_.child_pidfd_derived_nsfds1[i as usize] = ioctl(self_.child_pidfd1, info.pidfd_ioctl, 0);
        if self_.child_pidfd_derived_nsfds1[i as usize] < 0 {
            EXPECT_EQ!(errno(), EOPNOTSUPP, {
                TH_LOG!("%m - Failed to derive %s namespace from pidfd of process %d", info.name, self_.child_pid1);
            });
        }

        self_.child_pidfd_derived_nsfds2[i as usize] = ioctl(self_.child_pidfd2, info.pidfd_ioctl, 0);
        if self_.child_pidfd_derived_nsfds2[i as usize] < 0 {
            EXPECT_EQ!(errno(), EOPNOTSUPP, {
                TH_LOG!("%m - Failed to derive %s namespace from pidfd of process %d", info.name, self_.child_pid2);
            });
        }
        i += 1;
    }

    close(proc_fd);
});

fixture_teardown!(current_nsset, |self_: *mut current_nsset| unsafe {
    let mut i: c_int;
    let self_ = &mut *self_;

    ASSERT_EQ!(sys_pidfd_send_signal(self_.child_pidfd1, SIGKILL, ptr::null_mut(), 0), 0);
    ASSERT_EQ!(sys_pidfd_send_signal(self_.child_pidfd2, SIGKILL, ptr::null_mut(), 0), 0);

    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        if self_.nsfds[i as usize] >= 0 {
            close(self_.nsfds[i as usize]);
        }
        if self_.child_nsfds1[i as usize] >= 0 {
            close(self_.child_nsfds1[i as usize]);
        }
        if self_.child_nsfds2[i as usize] >= 0 {
            close(self_.child_nsfds2[i as usize]);
        }
        if self_.child_pidfd_derived_nsfds[i as usize] >= 0 {
            close(self_.child_pidfd_derived_nsfds[i as usize]);
        }
        if self_.child_pidfd_derived_nsfds1[i as usize] >= 0 {
            close(self_.child_pidfd_derived_nsfds1[i as usize]);
        }
        if self_.child_pidfd_derived_nsfds2[i as usize] >= 0 {
            close(self_.child_pidfd_derived_nsfds2[i as usize]);
        }
        i += 1;
    }

    if self_.child_pidfd1 >= 0 {
        EXPECT_EQ!(0, close(self_.child_pidfd1));
    }
    if self_.child_pidfd2 >= 0 {
        EXPECT_EQ!(0, close(self_.child_pidfd2));
    }
    ASSERT_EQ!(sys_waitid(P_PID, self_.child_pid_exited, ptr::null_mut(), WEXITED), 0);
    ASSERT_EQ!(sys_waitid(P_PID, self_.child_pid1, ptr::null_mut(), WEXITED), 0);
    ASSERT_EQ!(sys_waitid(P_PID, self_.child_pid2, ptr::null_mut(), WEXITED), 0);
});

unsafe fn preserve_ns(pid: c_int, ns: *const c_char) -> c_int {
    let ret: c_int;
    let mut path: [c_char; 50] = [0; 50];

    ret = snprintf(path.as_mut_ptr(), path.len(), b"/proc/%d/ns/%s\0".as_ptr() as *const c_char, pid, ns);
    if ret < 0 || ret as usize >= path.len() {
        return -EIO;
    }

    open(path.as_ptr(), O_RDONLY | O_CLOEXEC)
}

unsafe fn in_same_namespace(ns_fd1: c_int, pid2: pid_t, ns: *const c_char) -> c_int {
    let mut ns_fd2: c_int = -EBADF;
    let mut ret: c_int = -1;
    let mut ns_st1: stat = core::mem::zeroed();
    let mut ns_st2: stat = core::mem::zeroed();

    ret = fstat(ns_fd1, &mut ns_st1);
    if ret < 0 {
        return -1;
    }

    ns_fd2 = preserve_ns(pid2, ns);
    if ns_fd2 < 0 {
        return -1;
    }

    ret = fstat(ns_fd2, &mut ns_st2);
    close(ns_fd2);
    if ret < 0 {
        return -1;
    }

    /* processes are in the same namespace */
    if ns_st1.st_dev == ns_st2.st_dev && ns_st1.st_ino == ns_st2.st_ino {
        return 1;
    }

    /* processes are in different namespaces */
    0
}

/* Test that we can't pass garbage to the kernel. */
test_f!(current_nsset, invalid_flags, |self_: *mut current_nsset| unsafe {
    let self_ = &mut *self_;

    ASSERT_NE!(setns(self_.pidfd, 0), 0);
    EXPECT_EQ!(errno(), EINVAL);

    ASSERT_NE!(setns(self_.pidfd, -1), 0);
    EXPECT_EQ!(errno(), EINVAL);

    ASSERT_NE!(setns(self_.pidfd, CLONE_VM), 0);
    EXPECT_EQ!(errno(), EINVAL);

    ASSERT_NE!(setns(self_.pidfd, CLONE_NEWUSER | CLONE_VM), 0);
    EXPECT_EQ!(errno(), EINVAL);
});

/* Test that we can't attach to a task that has already exited. */
test_f!(current_nsset, pidfd_exited_child, |self_: *mut current_nsset| unsafe {
    let mut i: c_int;
    let pid: pid_t;
    let self_ = &mut *self_;

    ASSERT_NE!(setns(self_.child_pidfd_exited, CLONE_NEWUSER | CLONE_NEWNET), 0);
    EXPECT_EQ!(errno(), ESRCH);

    pid = getpid();
    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];
        /* Verify that we haven't changed any namespaces. */
        if self_.nsfds[i as usize] >= 0 {
            ASSERT_EQ!(in_same_namespace(self_.nsfds[i as usize], pid, info.name), 1);
        }
        i += 1;
    }
});

test_f!(current_nsset, pidfd_incremental_setns, |self_: *mut current_nsset| unsafe {
    let mut i: c_int;
    let pid: pid_t;
    let self_ = &mut *self_;

    pid = getpid();
    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];
        let nsfd: c_int;

        if self_.child_nsfds1[i as usize] < 0 {
            i += 1;
            continue;
        }

        if info.flag != 0 {
            ASSERT_EQ!(setns(self_.child_pidfd1, info.flag), 0, {
                TH_LOG!("%m - Failed to setns to %s namespace of %d via pidfd %d", info.name, self_.child_pid1, self_.child_pidfd1);
            });
        }

        /* Verify that we have changed to the correct namespaces. */
        if info.flag == CLONE_NEWPID {
            nsfd = self_.nsfds[i as usize];
        } else {
            nsfd = self_.child_nsfds1[i as usize];
        }
        ASSERT_EQ!(in_same_namespace(nsfd, pid, info.name), 1, {
            TH_LOG!("setns failed to place us correctly into %s namespace of %d via pidfd %d", info.name, self_.child_pid1, self_.child_pidfd1);
        });
        TH_LOG!("Managed to correctly setns to %s namespace of %d via pidfd %d", info.name, self_.child_pid1, self_.child_pidfd1);
        i += 1;
    }
});

test_f!(current_nsset, nsfd_incremental_setns, |self_: *mut current_nsset| unsafe {
    let mut i: c_int;
    let pid: pid_t;
    let self_ = &mut *self_;

    pid = getpid();
    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];
        let nsfd: c_int;

        if self_.child_nsfds1[i as usize] < 0 {
            i += 1;
            continue;
        }

        if info.flag != 0 {
            ASSERT_EQ!(setns(self_.child_nsfds1[i as usize], info.flag), 0, {
                TH_LOG!("%m - Failed to setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid1, self_.child_nsfds1[i as usize]);
            });
        }

        /* Verify that we have changed to the correct namespaces. */
        if info.flag == CLONE_NEWPID {
            nsfd = self_.nsfds[i as usize];
        } else {
            nsfd = self_.child_nsfds1[i as usize];
        }
        ASSERT_EQ!(in_same_namespace(nsfd, pid, info.name), 1, {
            TH_LOG!("setns failed to place us correctly into %s namespace of %d via nsfd %d", info.name, self_.child_pid1, self_.child_nsfds1[i as usize]);
        });
        TH_LOG!("Managed to correctly setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid1, self_.child_nsfds1[i as usize]);
        i += 1;
    }
});

test_f!(current_nsset, pidfd_derived_nsfd_incremental_setns, |self_: *mut current_nsset| unsafe {
    let mut i: c_int;
    let pid: pid_t;
    let self_ = &mut *self_;

    pid = getpid();
    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];
        let nsfd: c_int;

        if self_.child_pidfd_derived_nsfds1[i as usize] < 0 {
            i += 1;
            continue;
        }

        if info.flag != 0 {
            ASSERT_EQ!(setns(self_.child_pidfd_derived_nsfds1[i as usize], info.flag), 0, {
                TH_LOG!("%m - Failed to setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid1, self_.child_pidfd_derived_nsfds1[i as usize]);
            });
        }

        /* Verify that we have changed to the correct namespaces. */
        if info.flag == CLONE_NEWPID {
            nsfd = self_.child_pidfd_derived_nsfds[i as usize];
        } else {
            nsfd = self_.child_pidfd_derived_nsfds1[i as usize];
        }
        ASSERT_EQ!(in_same_namespace(nsfd, pid, info.name), 1, {
            TH_LOG!("setns failed to place us correctly into %s namespace of %d via nsfd %d", info.name, self_.child_pid1, self_.child_pidfd_derived_nsfds1[i as usize]);
        });
        TH_LOG!("Managed to correctly setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid1, self_.child_pidfd_derived_nsfds1[i as usize]);
        i += 1;
    }
});

test_f!(current_nsset, pidfd_one_shot_setns, |self_: *mut current_nsset| unsafe {
    let mut flags: c_uint = 0;
    let mut i: c_int;
    let pid: pid_t;
    let self_ = &mut *self_;

    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];

        if self_.child_nsfds1[i as usize] < 0 {
            i += 1;
            continue;
        }

        flags |= info.flag as c_uint;
        TH_LOG!("Adding %s namespace of %d to list of namespaces to attach to", info.name, self_.child_pid1);
        i += 1;
    }

    ASSERT_EQ!(setns(self_.child_pidfd1, flags as c_int), 0, {
        TH_LOG!("%m - Failed to setns to namespaces of %d", self_.child_pid1);
    });

    pid = getpid();
    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];
        let nsfd: c_int;

        if self_.child_nsfds1[i as usize] < 0 {
            i += 1;
            continue;
        }

        /* Verify that we have changed to the correct namespaces. */
        if info.flag == CLONE_NEWPID {
            nsfd = self_.nsfds[i as usize];
        } else {
            nsfd = self_.child_nsfds1[i as usize];
        }
        ASSERT_EQ!(in_same_namespace(nsfd, pid, info.name), 1, {
            TH_LOG!("setns failed to place us correctly into %s namespace of %d", info.name, self_.child_pid1);
        });
        TH_LOG!("Managed to correctly setns to %s namespace of %d", info.name, self_.child_pid1);
        i += 1;
    }
});

test_f!(current_nsset, no_foul_play, |self_: *mut current_nsset| unsafe {
    let mut flags: c_uint = 0;
    let mut i: c_int;
    let self_ = &mut *self_;

    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];

        if self_.child_nsfds1[i as usize] < 0 {
            i += 1;
            continue;
        }

        flags |= info.flag as c_uint;
        if info.flag != 0 {
            /* No use logging pid_for_children. */
            TH_LOG!("Adding %s namespace of %d to list of namespaces to attach to", info.name, self_.child_pid1);
        }
        i += 1;
    }

    ASSERT_EQ!(setns(self_.child_pidfd1, flags as c_int), 0, {
        TH_LOG!("%m - Failed to setns to namespaces of %d vid pidfd %d", self_.child_pid1, self_.child_pidfd1);
    });

    /*
     * Can't setns to a user namespace outside of our hierarchy since we
     * don't have caps in there and didn't create it. That means that under
     * no circumstances should we be able to setns to any of the other
     * ones since they aren't owned by our user namespace.
     */
    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];

        if self_.child_nsfds2[i as usize] < 0 || info.flag == 0 {
            i += 1;
            continue;
        }

        ASSERT_NE!(setns(self_.child_pidfd2, info.flag), 0, {
            TH_LOG!("Managed to setns to %s namespace of %d via pidfd %d", info.name, self_.child_pid2, self_.child_pidfd2);
        });
        TH_LOG!("%m - Correctly failed to setns to %s namespace of %d via pidfd %d", info.name, self_.child_pid2, self_.child_pidfd2);

        ASSERT_NE!(setns(self_.child_nsfds2[i as usize], info.flag), 0, {
            TH_LOG!("Managed to setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid2, self_.child_nsfds2[i as usize]);
        });
        TH_LOG!("%m - Correctly failed to setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid2, self_.child_nsfds2[i as usize]);
        i += 1;
    }

    /*
     * Can't setns to a user namespace outside of our hierarchy since we
     * don't have caps in there and didn't create it. That means that under
     * no circumstances should we be able to setns to any of the other
     * ones since they aren't owned by our user namespace.
     */
    i = 0;
    while i < PIDFD_NS_MAX as c_int {
        let info = &ns_info[i as usize];

        if self_.child_pidfd_derived_nsfds2[i as usize] < 0 || info.flag == 0 {
            i += 1;
            continue;
        }

        ASSERT_NE!(setns(self_.child_pidfd_derived_nsfds2[i as usize], info.flag), 0, {
            TH_LOG!("Managed to setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid2, self_.child_pidfd_derived_nsfds2[i as usize]);
        });
        TH_LOG!("%m - Correctly failed to setns to %s namespace of %d via nsfd %d", info.name, self_.child_pid2, self_.child_pidfd_derived_nsfds2[i as usize]);
        i += 1;
    }
});

test!(setns_einval, || unsafe {
    let fd: c_int;

    fd = sys_memfd_create(b"rostock\0".as_ptr() as *const c_char, 0);
    EXPECT_GT!(fd, 0);

    ASSERT_NE!(setns(fd, 0), 0);
    EXPECT_EQ!(errno(), EINVAL);
    close(fd);
});

test_harness_main!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
