/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of testing/selftests/pidfd/pidfd.h.
 *
 * C include/header-guard directives are intentionally not executable Rust.
 * This header depends on errno/fcntl/sched/signal/stdio/stdlib/string/syscall/
 * ioctl/types/wait declarations, kselftest.h, and clone3_selftests.h.
 */

pub type pid_t = i32;
pub type ssize_t = isize;
pub type size_t = usize;

pub type __u64 = u64;
pub type __u32 = u32;
pub type __s32 = i32;

pub const FD_PIDFS_ROOT: i32 = -10002;

pub const P_PIDFD: i32 = 3;

pub const CLONE_NEWTIME: u64 = 0x00000080;
pub const CLONE_PIDFD: u64 = 0x00001000;

pub const __NR_pidfd_open: i64 = 434;
pub const __NR_pidfd_send_signal: i64 = 424;
pub const __NR_clone3: i64 = 435;
pub const __NR_pidfd_getfd: i64 = 438;

pub const O_NONBLOCK: i32 = 0o0004000;
pub const O_EXCL: i32 = 0o0000200;

pub const PIDFD_NONBLOCK: i32 = O_NONBLOCK;

pub const PIDFD_SELF_THREAD: i32 = -10000; /* Current thread. */
pub const PIDFD_SELF_THREAD_GROUP: i32 = -10001; /* Current thread group leader. */

pub const PIDFD_SELF: i32 = PIDFD_SELF_THREAD;
pub const PIDFD_SELF_PROCESS: i32 = PIDFD_SELF_THREAD_GROUP;

pub const PIDFS_IOCTL_MAGIC: u32 = 0xFF;

pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;

pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;

pub const fn _IOC(dir: u32, type_: u32, nr: u32, size: u32) -> u64 {
    ((dir as u64) << _IOC_DIRSHIFT)
        | ((type_ as u64) << _IOC_TYPESHIFT)
        | ((nr as u64) << _IOC_NRSHIFT)
        | ((size as u64) << _IOC_SIZESHIFT)
}

pub const fn _IO(type_: u32, nr: u32) -> u64 {
    _IOC(_IOC_NONE, type_, nr, 0)
}

pub const fn _IOWR<T>(type_: u32, nr: u32) -> u64 {
    _IOC(
        _IOC_READ | _IOC_WRITE,
        type_,
        nr,
        core::mem::size_of::<T>() as u32,
    )
}

pub const PIDFD_GET_CGROUP_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 1);
pub const PIDFD_GET_IPC_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 2);
pub const PIDFD_GET_MNT_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 3);
pub const PIDFD_GET_NET_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 4);
pub const PIDFD_GET_PID_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 5);
pub const PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 6);
pub const PIDFD_GET_TIME_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 7);
pub const PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 8);
pub const PIDFD_GET_USER_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 9);
pub const PIDFD_GET_UTS_NAMESPACE: u64 = _IO(PIDFS_IOCTL_MAGIC, 10);
pub const PIDFD_GET_INFO: u64 = _IOWR::<pidfd_info>(PIDFS_IOCTL_MAGIC, 11);

pub const PIDFD_INFO_PID: u64 = 1u64 << 0; /* Always returned, even if not requested */
pub const PIDFD_INFO_CREDS: u64 = 1u64 << 1; /* Always returned, even if not requested */
pub const PIDFD_INFO_CGROUPID: u64 = 1u64 << 2; /* Always returned if available, even if not requested */
pub const PIDFD_INFO_EXIT: u64 = 1u64 << 3; /* Always returned if available, even if not requested */
pub const PIDFD_INFO_COREDUMP: u64 = 1u64 << 4;
pub const PIDFD_INFO_SUPPORTED_MASK: u64 = 1u64 << 5;
pub const PIDFD_INFO_COREDUMP_SIGNAL: u64 = 1u64 << 6;
pub const PIDFD_INFO_COREDUMP_CODE: u64 = 1u64 << 7;

pub const PIDFD_COREDUMPED: u32 = 1u32 << 0; /* Did crash and... */
pub const PIDFD_COREDUMP_SKIP: u32 = 1u32 << 1; /* coredumping generation was skipped. */
pub const PIDFD_COREDUMP_USER: u32 = 1u32 << 2; /* coredump was done as the user. */
pub const PIDFD_COREDUMP_ROOT: u32 = 1u32 << 3; /* coredump was done as root. */

pub const PIDFD_THREAD: i32 = O_EXCL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pidfd_info {
    pub mask: __u64,
    pub cgroupid: __u64,
    pub pid: __u32,
    pub tgid: __u32,
    pub ppid: __u32,
    pub ruid: __u32,
    pub rgid: __u32,
    pub euid: __u32,
    pub egid: __u32,
    pub suid: __u32,
    pub sgid: __u32,
    pub fsuid: __u32,
    pub fsgid: __u32,
    pub exit_code: __s32,
    pub coredump_mask: __u32,
    pub coredump_signal: __u32,
    pub coredump_code: __u32,
    pub supported_mask: __u64,
}

/*
 * The kernel reserves 300 pids via RESERVED_PIDS in kernel/pid.c
 * That means, when it wraps around any pid < 300 will be skipped.
 * So we need to use a pid > 300 in order to test recycling.
 */
pub const PID_RECYCLE: i32 = 1000;

/*
 * Define a few custom error codes for the child process to clearly indicate
 * what is happening. This way we can tell the difference between a system
 * error, a test error, etc.
 */
pub const PIDFD_PASS: i32 = 0;
pub const PIDFD_FAIL: i32 = 1;
pub const PIDFD_ERROR: i32 = 2;
pub const PIDFD_SKIP: i32 = 3;
pub const PIDFD_XFAIL: i32 = 4;

pub const EINTR: i32 = 4;
pub const SIGCHLD: i32 = 17;
pub const __NR_waitid: i64 = 247;
pub const __NR_memfd_create: i64 = 319;
pub const __NR_execveat: i64 = 322;

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __clone_args {
    pub flags: __u64,
    pub pidfd: __u64,
    pub child_tid: __u64,
    pub parent_tid: __u64,
    pub exit_signal: __u64,
    pub stack: __u64,
    pub stack_size: __u64,
    pub tls: __u64,
    pub set_tid: __u64,
    pub set_tid_size: __u64,
    pub cgroup: __u64,
}

unsafe extern "C" {
    pub fn syscall(num: i64, ...) -> i64;
    pub fn waitpid(pid: pid_t, status: *mut i32, options: i32) -> pid_t;
    pub fn read(fd: i32, buf: *mut core::ffi::c_void, count: size_t) -> ssize_t;
    pub fn write(fd: i32, buf: *const core::ffi::c_void, count: size_t) -> ssize_t;

    pub fn __errno_location() -> *mut i32;

    pub fn ksft_print_msg(fmt: *const core::ffi::c_char, ...);
    pub fn ptr_to_u64(ptr: *const core::ffi::c_void) -> __u64;
    pub fn sys_clone3(args: *mut __clone_args, size: size_t) -> pid_t;
}

pub unsafe fn errno() -> i32 {
    unsafe { *__errno_location() }
}

pub fn WIFEXITED(status: i32) -> bool {
    (status & 0x7f) == 0
}

pub fn WEXITSTATUS(status: i32) -> i32 {
    (status & 0xff00) >> 8
}

pub fn WIFSIGNALED(status: i32) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

pub fn WTERMSIG(status: i32) -> i32 {
    status & 0x7f
}

pub unsafe fn sys_waitid(
    which: i32,
    pid: pid_t,
    info: *mut siginfo_t,
    options: i32,
) -> i32 {
    unsafe {
        syscall(
            __NR_waitid,
            which,
            pid,
            info,
            options,
            core::ptr::null_mut::<core::ffi::c_void>(),
        ) as i32
    }
}

pub unsafe fn wait_for_pid(pid: pid_t) -> i32 {
    let mut status: i32 = 0;
    let mut ret: i32;

    loop {
        ret = unsafe { waitpid(pid, &mut status, 0) };
        if ret == -1 {
            if unsafe { errno() } == EINTR {
                continue;
            }

            unsafe {
                ksft_print_msg(
                    c"waitpid returned -1, errno=%d\n".as_ptr(),
                    errno(),
                );
            }
            return -1;
        }

        break;
    }

    if !WIFEXITED(status) {
        unsafe {
            ksft_print_msg(
                c"waitpid !WIFEXITED, WIFSIGNALED=%d, WTERMSIG=%d\n".as_ptr(),
                WIFSIGNALED(status) as i32,
                WTERMSIG(status),
            );
        }
        return -1;
    }

    ret = WEXITSTATUS(status);
    ret
}

pub unsafe fn sys_pidfd_open(pid: pid_t, flags: u32) -> i32 {
    unsafe { syscall(__NR_pidfd_open, pid, flags) as i32 }
}

pub unsafe fn sys_pidfd_send_signal(
    pidfd: i32,
    sig: i32,
    info: *mut siginfo_t,
    flags: u32,
) -> i32 {
    unsafe { syscall(__NR_pidfd_send_signal, pidfd, sig, info, flags) as i32 }
}

pub unsafe fn sys_pidfd_getfd(pidfd: i32, fd: i32, flags: i32) -> i32 {
    unsafe { syscall(__NR_pidfd_getfd, pidfd, fd, flags) as i32 }
}

pub unsafe fn sys_memfd_create(name: *const core::ffi::c_char, flags: u32) -> i32 {
    unsafe { syscall(__NR_memfd_create, name, flags) as i32 }
}

pub unsafe fn create_child(pidfd: *mut i32, flags: u32) -> pid_t {
    let mut args = __clone_args {
        flags: CLONE_PIDFD | flags as u64,
        exit_signal: SIGCHLD as u64,
        pidfd: unsafe { ptr_to_u64(pidfd as *const core::ffi::c_void) },
        child_tid: 0,
        parent_tid: 0,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    unsafe { sys_clone3(&mut args, core::mem::size_of::<__clone_args>()) }
}

pub unsafe fn read_nointr(fd: i32, buf: *mut core::ffi::c_void, count: size_t) -> ssize_t {
    let mut ret: ssize_t;

    loop {
        ret = unsafe { read(fd, buf, count) };
        if !(ret < 0 && unsafe { errno() } == EINTR) {
            break;
        }
    }

    ret
}

pub unsafe fn write_nointr(fd: i32, buf: *const core::ffi::c_void, count: size_t) -> ssize_t {
    let mut ret: ssize_t;

    loop {
        ret = unsafe { write(fd, buf, count) };
        if !(ret < 0 && unsafe { errno() } == EINTR) {
            break;
        }
    }

    ret
}

pub unsafe fn sys_execveat(
    dirfd: i32,
    pathname: *const core::ffi::c_char,
    argv: *const *mut core::ffi::c_char,
    envp: *const *mut core::ffi::c_char,
    flags: i32,
) -> i32 {
    unsafe { syscall(__NR_execveat, dirfd, pathname, argv, envp, flags) as i32 }
}
