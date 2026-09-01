// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type pid_t = c_int;
type socklen_t = u32;
type __u64 = u64;
type ssize_t = isize;
type size_t = usize;

const O_RDONLY: c_int = 0;
const SIGKILL: c_int = 9;
const EPERM: c_int = 1;
const EACCES: c_int = 13;
const EINVAL: c_int = 22;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWCGROUP: c_int = 0x02000000;
const CLONE_NEWIPC: c_int = 0x08000000;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWTIME: c_int = 0x00000080;
const CLONE_NEWPID: c_int = 0x20000000;
const CLONE_NEWNET: c_int = 0x40000000;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_NETNS_COOKIE: c_int = 71;
const NS_GET_ID: c_ulong = 0xb705;
const NS_GET_MNTNS_ID: c_ulong = 0x8008b705;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn unshare(flags: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn pause() -> c_int;
    fn _exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
}

macro_rules! ASSERT_GE {
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

macro_rules! SKIP {
    (return, $msg:expr) => {
        return
    };
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

/* Fixture for tests that create child processes */
#[repr(C)]
struct nsid {
    child_pid: pid_t,
    grandchild_pid: pid_t,
}

unsafe fn nsid_setup(self_: *mut nsid) {
    unsafe {
        (*self_).child_pid = 0;
        (*self_).grandchild_pid = 0;
    }
}

unsafe fn nsid_teardown(self_: *mut nsid) {
    /*
     * Kill grandchild first: timens_separate and pidns_separate fork a
     * grandchild that calls pause().  It is reparented to init on child
     * exit and keeps the test runner's tap pipe open, hanging the runner.
     */
    unsafe {
        if (*self_).grandchild_pid > 0 {
            kill((*self_).grandchild_pid, SIGKILL);
            waitpid((*self_).grandchild_pid, core::ptr::null_mut(), 0);
        }
        if (*self_).child_pid > 0 {
            kill((*self_).child_pid, SIGKILL);
            waitpid((*self_).child_pid, core::ptr::null_mut(), 0);
        }
    }
}

unsafe fn nsid_mntns_basic() {
    let mut mnt_ns_id: __u64 = 0;
    let fd_mntns: c_int;
    let mut ret: c_int;

    /* Open the current mount namespace */
    fd_mntns = unsafe { open(c"/proc/self/ns/mnt".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_mntns, 0);

    /* Get the mount namespace ID */
    ret = unsafe { ioctl(fd_mntns, NS_GET_MNTNS_ID, &mut mnt_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(mnt_ns_id, 0);

    /* Verify we can get the same ID again */
    let mut mnt_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_mntns, NS_GET_ID, &mut mnt_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(mnt_ns_id, mnt_ns_id2);

    unsafe { close(fd_mntns) };
}

unsafe fn nsid_separate_common(
    self_: *mut nsid,
    ns_name: *const c_char,
    clone_flag: c_int,
    no_permission_message: &str,
) {
    let mut parent_ns_id: __u64 = 0;
    let mut child_ns_id: __u64 = 0;
    let fd_parent_ns: c_int;
    let fd_child_ns: c_int;
    let mut ret: c_int;
    let pid: pid_t;
    let mut pipefd: [c_int; 2] = [0; 2];

    fd_parent_ns = unsafe { open(ns_name, O_RDONLY) };
    ASSERT_GE!(fd_parent_ns, 0);
    ret = unsafe { ioctl(fd_parent_ns, NS_GET_ID, &mut parent_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(parent_ns_id, 0);

    ASSERT_EQ!(unsafe { pipe(pipefd.as_mut_ptr()) }, 0);

    pid = unsafe { fork() };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        unsafe {
            close(pipefd[0]);

            ret = unshare(clone_flag);
            if ret != 0 {
                if errno() == EPERM || errno() == EACCES {
                    write(pipefd[1], c"S".as_ptr() as *const c_void, 1);
                    _exit(0);
                }
                _exit(1);
            }

            write(pipefd[1], c"Y".as_ptr() as *const c_void, 1);
            close(pipefd[1]);

            pause();
            _exit(0);
        }
    }

    unsafe {
        (*self_).child_pid = pid;
        close(pipefd[1]);
    }

    let mut buf: c_char = 0;
    ASSERT_EQ!(unsafe { read(pipefd[0], &mut buf as *mut c_char as *mut c_void, 1) }, 1);
    unsafe { close(pipefd[0]) };

    if buf == b'S' as c_char {
        unsafe { close(fd_parent_ns) };
        SKIP!(return, no_permission_message);
    }

    ASSERT_EQ!(buf, b'Y' as c_char);

    let mut path: [c_char; 256] = [0; 256];
    unsafe { snprintf(path.as_mut_ptr(), path.len(), c"/proc/%d/ns/%s".as_ptr(), pid, ns_name.offset(14)) };
    fd_child_ns = unsafe { open(path.as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_child_ns, 0);

    ret = unsafe { ioctl(fd_child_ns, NS_GET_ID, &mut child_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(child_ns_id, 0);

    ASSERT_NE!(parent_ns_id, child_ns_id);

    unsafe {
        close(fd_parent_ns);
        close(fd_child_ns);
    }
}

unsafe fn mntns_separate(self_: *mut nsid) {
    unsafe {
        nsid_separate_common(
            self_,
            c"/proc/self/ns/mnt".as_ptr(),
            CLONE_NEWNS,
            "No permission to create mount namespace",
        )
    }
}

unsafe fn nsid_cgroupns_basic() {
    let mut cgroup_ns_id: __u64 = 0;
    let fd_cgroupns: c_int;
    let mut ret: c_int;

    /* Open the current cgroup namespace */
    fd_cgroupns = unsafe { open(c"/proc/self/ns/cgroup".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_cgroupns, 0);

    /* Get the cgroup namespace ID */
    ret = unsafe { ioctl(fd_cgroupns, NS_GET_ID, &mut cgroup_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(cgroup_ns_id, 0);

    /* Verify we can get the same ID again */
    let mut cgroup_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_cgroupns, NS_GET_ID, &mut cgroup_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(cgroup_ns_id, cgroup_ns_id2);

    unsafe { close(fd_cgroupns) };
}

unsafe fn cgroupns_separate(self_: *mut nsid) {
    unsafe {
        nsid_separate_common(
            self_,
            c"/proc/self/ns/cgroup".as_ptr(),
            CLONE_NEWCGROUP,
            "No permission to create cgroup namespace",
        )
    }
}

unsafe fn nsid_ipcns_basic() {
    let mut ipc_ns_id: __u64 = 0;
    let fd_ipcns: c_int;
    let mut ret: c_int;

    /* Open the current IPC namespace */
    fd_ipcns = unsafe { open(c"/proc/self/ns/ipc".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_ipcns, 0);

    /* Get the IPC namespace ID */
    ret = unsafe { ioctl(fd_ipcns, NS_GET_ID, &mut ipc_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(ipc_ns_id, 0);

    /* Verify we can get the same ID again */
    let mut ipc_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_ipcns, NS_GET_ID, &mut ipc_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(ipc_ns_id, ipc_ns_id2);

    unsafe { close(fd_ipcns) };
}

unsafe fn ipcns_separate(self_: *mut nsid) {
    unsafe {
        nsid_separate_common(
            self_,
            c"/proc/self/ns/ipc".as_ptr(),
            CLONE_NEWIPC,
            "No permission to create IPC namespace",
        )
    }
}

unsafe fn nsid_utsns_basic() {
    let mut uts_ns_id: __u64 = 0;
    let fd_utsns: c_int;
    let mut ret: c_int;

    /* Open the current UTS namespace */
    fd_utsns = unsafe { open(c"/proc/self/ns/uts".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_utsns, 0);

    /* Get the UTS namespace ID */
    ret = unsafe { ioctl(fd_utsns, NS_GET_ID, &mut uts_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(uts_ns_id, 0);

    /* Verify we can get the same ID again */
    let mut uts_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_utsns, NS_GET_ID, &mut uts_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(uts_ns_id, uts_ns_id2);

    unsafe { close(fd_utsns) };
}

unsafe fn utsns_separate(self_: *mut nsid) {
    unsafe {
        nsid_separate_common(
            self_,
            c"/proc/self/ns/uts".as_ptr(),
            CLONE_NEWUTS,
            "No permission to create UTS namespace",
        )
    }
}

unsafe fn nsid_userns_basic() {
    let mut user_ns_id: __u64 = 0;
    let fd_userns: c_int;
    let mut ret: c_int;

    /* Open the current user namespace */
    fd_userns = unsafe { open(c"/proc/self/ns/user".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_userns, 0);

    /* Get the user namespace ID */
    ret = unsafe { ioctl(fd_userns, NS_GET_ID, &mut user_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(user_ns_id, 0);

    /* Verify we can get the same ID again */
    let mut user_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_userns, NS_GET_ID, &mut user_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(user_ns_id, user_ns_id2);

    unsafe { close(fd_userns) };
}

unsafe fn userns_separate(self_: *mut nsid) {
    unsafe {
        nsid_separate_common(
            self_,
            c"/proc/self/ns/user".as_ptr(),
            CLONE_NEWUSER,
            "No permission to create user namespace",
        )
    }
}

unsafe fn nsid_timens_basic() {
    let mut time_ns_id: __u64 = 0;
    let fd_timens: c_int;
    let mut ret: c_int;

    /* Open the current time namespace */
    fd_timens = unsafe { open(c"/proc/self/ns/time".as_ptr(), O_RDONLY) };
    if fd_timens < 0 {
        SKIP!(return, "Time namespaces not supported");
    }

    /* Get the time namespace ID */
    ret = unsafe { ioctl(fd_timens, NS_GET_ID, &mut time_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(time_ns_id, 0);

    /* Verify we can get the same ID again */
    let mut time_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_timens, NS_GET_ID, &mut time_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(time_ns_id, time_ns_id2);

    unsafe { close(fd_timens) };
}

unsafe fn timens_separate(self_: *mut nsid) {
    let mut parent_time_ns_id: __u64 = 0;
    let mut child_time_ns_id: __u64 = 0;
    let fd_parent_timens: c_int;
    let fd_child_timens: c_int;
    let mut ret: c_int;
    let pid: pid_t;
    let mut pipefd: [c_int; 2] = [0; 2];

    /* Open the current time namespace */
    fd_parent_timens = unsafe { open(c"/proc/self/ns/time".as_ptr(), O_RDONLY) };
    if fd_parent_timens < 0 {
        SKIP!(return, "Time namespaces not supported");
    }

    /* Get parent's time namespace ID */
    ret = unsafe { ioctl(fd_parent_timens, NS_GET_ID, &mut parent_time_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(parent_time_ns_id, 0);

    /* Create a pipe for synchronization */
    ASSERT_EQ!(unsafe { pipe(pipefd.as_mut_ptr()) }, 0);

    pid = unsafe { fork() };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child process */
        unsafe {
            close(pipefd[0]);

            /* Create new time namespace */
            ret = unshare(CLONE_NEWTIME);
            if ret != 0 {
                /* Skip test if we don't have permission */
                if errno() == EPERM || errno() == EACCES || errno() == EINVAL {
                    write(pipefd[1], c"S".as_ptr() as *const c_void, 1); /* Signal skip */
                    _exit(0);
                }
                _exit(1);
            }

            /* Fork a grandchild to actually enter the new namespace */
            let grandchild: pid_t = fork();
            if grandchild == 0 {
                close(pipefd[1]);
                pause();
                _exit(0);
            } else if grandchild > 0 {
                /* Child writes grandchild PID and waits */
                write(pipefd[1], c"Y".as_ptr() as *const c_void, 1);
                write(
                    pipefd[1],
                    &grandchild as *const pid_t as *const c_void,
                    core::mem::size_of_val(&grandchild),
                );
                close(pipefd[1]);
                pause(); /* Keep the parent alive to maintain the grandchild */
                _exit(0);
            } else {
                _exit(1);
            }
        }
    }

    /* Track child for cleanup */
    unsafe { (*self_).child_pid = pid };

    /* Parent process */
    unsafe { close(pipefd[1]) };

    let mut buf: c_char = 0;
    ASSERT_EQ!(unsafe { read(pipefd[0], &mut buf as *mut c_char as *mut c_void, 1) }, 1);

    if buf == b'S' as c_char {
        /* Child couldn't create namespace, skip test */
        unsafe {
            close(fd_parent_timens);
            close(pipefd[0]);
        }
        SKIP!(return, "Cannot create time namespace");
    }

    ASSERT_EQ!(buf, b'Y' as c_char);

    let mut grandchild_pid: pid_t = 0;
    ASSERT_EQ!(
        unsafe {
            read(
                pipefd[0],
                &mut grandchild_pid as *mut pid_t as *mut c_void,
                core::mem::size_of_val(&grandchild_pid),
            )
        },
        core::mem::size_of_val(&grandchild_pid) as ssize_t
    );
    unsafe {
        (*self_).grandchild_pid = grandchild_pid;
        close(pipefd[0]);
    }

    /* Open grandchild's time namespace */
    let mut path: [c_char; 256] = [0; 256];
    unsafe { snprintf(path.as_mut_ptr(), path.len(), c"/proc/%d/ns/time".as_ptr(), grandchild_pid) };
    fd_child_timens = unsafe { open(path.as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_child_timens, 0);

    /* Get child's time namespace ID */
    ret = unsafe { ioctl(fd_child_timens, NS_GET_ID, &mut child_time_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(child_time_ns_id, 0);

    /* Parent and child should have different time namespace IDs */
    ASSERT_NE!(parent_time_ns_id, child_time_ns_id);

    unsafe {
        close(fd_parent_timens);
        close(fd_child_timens);
    }
}

unsafe fn nsid_pidns_basic() {
    let mut pid_ns_id: __u64 = 0;
    let fd_pidns: c_int;
    let mut ret: c_int;

    /* Open the current PID namespace */
    fd_pidns = unsafe { open(c"/proc/self/ns/pid".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_pidns, 0);

    /* Get the PID namespace ID */
    ret = unsafe { ioctl(fd_pidns, NS_GET_ID, &mut pid_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(pid_ns_id, 0);

    /* Verify we can get the same ID again */
    let mut pid_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_pidns, NS_GET_ID, &mut pid_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(pid_ns_id, pid_ns_id2);

    unsafe { close(fd_pidns) };
}

unsafe fn pidns_separate(self_: *mut nsid) {
    let mut parent_pid_ns_id: __u64 = 0;
    let mut child_pid_ns_id: __u64 = 0;
    let fd_parent_pidns: c_int;
    let fd_child_pidns: c_int;
    let mut ret: c_int;
    let pid: pid_t;
    let mut pipefd: [c_int; 2] = [0; 2];

    /* Get parent's PID namespace ID */
    fd_parent_pidns = unsafe { open(c"/proc/self/ns/pid".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_parent_pidns, 0);
    ret = unsafe { ioctl(fd_parent_pidns, NS_GET_ID, &mut parent_pid_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(parent_pid_ns_id, 0);

    /* Create a pipe for synchronization */
    ASSERT_EQ!(unsafe { pipe(pipefd.as_mut_ptr()) }, 0);

    pid = unsafe { fork() };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child process */
        unsafe {
            close(pipefd[0]);

            /* Create new PID namespace */
            ret = unshare(CLONE_NEWPID);
            if ret != 0 {
                /* Skip test if we don't have permission */
                if errno() == EPERM || errno() == EACCES {
                    write(pipefd[1], c"S".as_ptr() as *const c_void, 1); /* Signal skip */
                    _exit(0);
                }
                _exit(1);
            }

            /* Fork a grandchild to actually enter the new namespace */
            let grandchild: pid_t = fork();
            if grandchild == 0 {
                close(pipefd[1]);
                pause();
                _exit(0);
            } else if grandchild > 0 {
                /* Child writes grandchild PID and waits */
                write(pipefd[1], c"Y".as_ptr() as *const c_void, 1);
                write(
                    pipefd[1],
                    &grandchild as *const pid_t as *const c_void,
                    core::mem::size_of_val(&grandchild),
                );
                close(pipefd[1]);
                pause(); /* Keep the parent alive to maintain the grandchild */
                _exit(0);
            } else {
                _exit(1);
            }
        }
    }

    /* Track child for cleanup */
    unsafe { (*self_).child_pid = pid };

    /* Parent process */
    unsafe { close(pipefd[1]) };

    let mut buf: c_char = 0;
    ASSERT_EQ!(unsafe { read(pipefd[0], &mut buf as *mut c_char as *mut c_void, 1) }, 1);

    if buf == b'S' as c_char {
        /* Child couldn't create namespace, skip test */
        unsafe {
            close(fd_parent_pidns);
            close(pipefd[0]);
        }
        SKIP!(return, "No permission to create PID namespace");
    }

    ASSERT_EQ!(buf, b'Y' as c_char);

    let mut grandchild_pid: pid_t = 0;
    ASSERT_EQ!(
        unsafe {
            read(
                pipefd[0],
                &mut grandchild_pid as *mut pid_t as *mut c_void,
                core::mem::size_of_val(&grandchild_pid),
            )
        },
        core::mem::size_of_val(&grandchild_pid) as ssize_t
    );
    unsafe {
        (*self_).grandchild_pid = grandchild_pid;
        close(pipefd[0]);
    }

    /* Open grandchild's PID namespace */
    let mut path: [c_char; 256] = [0; 256];
    unsafe { snprintf(path.as_mut_ptr(), path.len(), c"/proc/%d/ns/pid".as_ptr(), grandchild_pid) };
    fd_child_pidns = unsafe { open(path.as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_child_pidns, 0);

    /* Get child's PID namespace ID */
    ret = unsafe { ioctl(fd_child_pidns, NS_GET_ID, &mut child_pid_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(child_pid_ns_id, 0);

    /* Parent and child should have different PID namespace IDs */
    ASSERT_NE!(parent_pid_ns_id, child_pid_ns_id);

    unsafe {
        close(fd_parent_pidns);
        close(fd_child_pidns);
    }
}

unsafe fn nsid_netns_basic() {
    let mut net_ns_id: __u64 = 0;
    let mut netns_cookie: __u64 = 0;
    let fd_netns: c_int;
    let sock: c_int;
    let mut optlen: socklen_t;
    let mut ret: c_int;

    /* Open the current network namespace */
    fd_netns = unsafe { open(c"/proc/self/ns/net".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_netns, 0);

    /* Get the network namespace ID via ioctl */
    ret = unsafe { ioctl(fd_netns, NS_GET_ID, &mut net_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(net_ns_id, 0);

    /* Create a socket to get the SO_NETNS_COOKIE */
    sock = unsafe { socket(AF_UNIX, SOCK_STREAM, 0) };
    ASSERT_GE!(sock, 0);

    /* Get the network namespace cookie via socket option */
    optlen = core::mem::size_of_val(&netns_cookie) as socklen_t;
    ret = unsafe {
        getsockopt(
            sock,
            SOL_SOCKET,
            SO_NETNS_COOKIE,
            &mut netns_cookie as *mut __u64 as *mut c_void,
            &mut optlen as *mut socklen_t,
        )
    };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(optlen, core::mem::size_of_val(&netns_cookie) as socklen_t);

    /* The namespace ID and cookie should be identical */
    ASSERT_EQ!(net_ns_id, netns_cookie);

    /* Verify we can get the same ID again */
    let mut net_ns_id2: __u64 = 0;
    ret = unsafe { ioctl(fd_netns, NS_GET_ID, &mut net_ns_id2 as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(net_ns_id, net_ns_id2);

    unsafe {
        close(sock);
        close(fd_netns);
    }
}

unsafe fn netns_separate(self_: *mut nsid) {
    let mut parent_net_ns_id: __u64 = 0;
    let mut parent_netns_cookie: __u64 = 0;
    let mut child_net_ns_id: __u64 = 0;
    let mut child_netns_cookie: __u64 = 0;
    let fd_parent_netns: c_int;
    let fd_child_netns: c_int;
    let parent_sock: c_int;
    let child_sock: c_int;
    let mut optlen: socklen_t;
    let mut ret: c_int;
    let pid: pid_t;
    let mut pipefd: [c_int; 2] = [0; 2];

    /* Get parent's network namespace ID */
    fd_parent_netns = unsafe { open(c"/proc/self/ns/net".as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_parent_netns, 0);
    ret = unsafe { ioctl(fd_parent_netns, NS_GET_ID, &mut parent_net_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(parent_net_ns_id, 0);

    /* Get parent's network namespace cookie */
    parent_sock = unsafe { socket(AF_UNIX, SOCK_STREAM, 0) };
    ASSERT_GE!(parent_sock, 0);
    optlen = core::mem::size_of_val(&parent_netns_cookie) as socklen_t;
    ret = unsafe {
        getsockopt(
            parent_sock,
            SOL_SOCKET,
            SO_NETNS_COOKIE,
            &mut parent_netns_cookie as *mut __u64 as *mut c_void,
            &mut optlen as *mut socklen_t,
        )
    };
    ASSERT_EQ!(ret, 0);

    /* Verify parent's ID and cookie match */
    ASSERT_EQ!(parent_net_ns_id, parent_netns_cookie);

    /* Create a pipe for synchronization */
    ASSERT_EQ!(unsafe { pipe(pipefd.as_mut_ptr()) }, 0);

    pid = unsafe { fork() };
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Child process */
        unsafe {
            close(pipefd[0]);

            /* Create new network namespace */
            ret = unshare(CLONE_NEWNET);
            if ret != 0 {
                /* Skip test if we don't have permission */
                if errno() == EPERM || errno() == EACCES {
                    write(pipefd[1], c"S".as_ptr() as *const c_void, 1); /* Signal skip */
                    _exit(0);
                }
                _exit(1);
            }

            /* Signal success */
            write(pipefd[1], c"Y".as_ptr() as *const c_void, 1);
            close(pipefd[1]);

            /* Keep namespace alive */
            pause();
            _exit(0);
        }
    }

    /* Track child for cleanup */
    unsafe { (*self_).child_pid = pid };

    /* Parent process */
    unsafe { close(pipefd[1]) };

    let mut buf: c_char = 0;
    ASSERT_EQ!(unsafe { read(pipefd[0], &mut buf as *mut c_char as *mut c_void, 1) }, 1);
    unsafe { close(pipefd[0]) };

    if buf == b'S' as c_char {
        /* Child couldn't create namespace, skip test */
        unsafe {
            close(fd_parent_netns);
            close(parent_sock);
        }
        SKIP!(return, "No permission to create network namespace");
    }

    ASSERT_EQ!(buf, b'Y' as c_char);

    /* Open child's network namespace */
    let mut path: [c_char; 256] = [0; 256];
    unsafe { snprintf(path.as_mut_ptr(), path.len(), c"/proc/%d/ns/net".as_ptr(), pid) };
    fd_child_netns = unsafe { open(path.as_ptr(), O_RDONLY) };
    ASSERT_GE!(fd_child_netns, 0);

    /* Get child's network namespace ID */
    ret = unsafe { ioctl(fd_child_netns, NS_GET_ID, &mut child_net_ns_id as *mut __u64) };
    ASSERT_EQ!(ret, 0);
    ASSERT_NE!(child_net_ns_id, 0);

    /* Create socket in child's namespace to get cookie */
    ret = unsafe { setns(fd_child_netns, CLONE_NEWNET) };
    if ret == 0 {
        child_sock = unsafe { socket(AF_UNIX, SOCK_STREAM, 0) };
        ASSERT_GE!(child_sock, 0);

        optlen = core::mem::size_of_val(&child_netns_cookie) as socklen_t;
        ret = unsafe {
            getsockopt(
                child_sock,
                SOL_SOCKET,
                SO_NETNS_COOKIE,
                &mut child_netns_cookie as *mut __u64 as *mut c_void,
                &mut optlen as *mut socklen_t,
            )
        };
        ASSERT_EQ!(ret, 0);

        /* Verify child's ID and cookie match */
        ASSERT_EQ!(child_net_ns_id, child_netns_cookie);

        unsafe { close(child_sock) };

        /* Return to parent namespace */
        unsafe { setns(fd_parent_netns, CLONE_NEWNET) };
    }

    /* Parent and child should have different network namespace IDs */
    ASSERT_NE!(parent_net_ns_id, child_net_ns_id);
    if child_netns_cookie != 0 {
        ASSERT_NE!(parent_netns_cookie, child_netns_cookie);
    }

    unsafe {
        close(fd_parent_netns);
        close(fd_child_netns);
        close(parent_sock);
    }
}

/* TEST_HARNESS_MAIN */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
