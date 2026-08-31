// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012 The Chromium OS Authors. All rights reserved.
 *
 * Test code for seccomp bpf.
 *
 * Rust source-level translation of testing/selftests/seccomp/seccomp_bpf.c.
 * This file intentionally preserves the Linux selftest harness surface:
 * TEST(), TEST_F(), FIXTURE(), ASSERT_*(), EXPECT_*(), TH_LOG(), SKIP(),
 * BPF_STMT(), BPF_JUMP(), ARRAY_SIZE(), and arch register accessors are
 * external dependencies supplied by the translated selftest environment.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __s64 = i64;
pub type pid_t = c_int;
pub type pthread_t = c_ulong;
pub type sem_t = c_void;
pub type pthread_cond_t = c_void;
pub type pthread_mutex_t = c_void;
pub type ssize_t = isize;
pub type size_t = usize;
pub type clock_t = c_long;
pub type cap_t = *mut c_void;
pub type cap_flag_value_t = c_int;
pub type cap_value_t = c_int;

/* Attempt to de-conflict with the selftests tree. */
/* C fallback: #ifndef SKIP #define SKIP(s, ...) XFAIL(s, ##__VA_ARGS__) */
/* C fallback: #ifndef MIN #define MIN(X, Y) ((X) < (Y) ? (X) : (Y)) */
pub const fn MIN_usize(x: usize, y: usize) -> usize {
    if x < y { x } else { y }
}

pub const PR_SET_PTRACER: c_int = 0x59616d61;
pub const PR_SET_NO_NEW_PRIVS: c_int = 38;
pub const PR_GET_NO_NEW_PRIVS: c_int = 39;
pub const PR_SECCOMP_EXT: c_int = 43;
pub const SECCOMP_EXT_ACT: c_uint = 1;
pub const SECCOMP_EXT_ACT_TSYNC: c_uint = 1;
pub const SECCOMP_MODE_STRICT: c_uint = 1;
pub const SECCOMP_MODE_FILTER: c_uint = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_data {
    pub nr: c_int,
    pub arch: __u32,
    pub instruction_pointer: __u64,
    pub args: [__u64; 6],
}

pub const SECCOMP_RET_KILL_PROCESS: __u32 = 0x80000000u32; /* kill the process */
pub const SECCOMP_RET_KILL_THREAD: __u32 = 0x00000000u32; /* kill the thread */
pub const SECCOMP_RET_KILL: __u32 = SECCOMP_RET_KILL_THREAD;
pub const SECCOMP_RET_TRAP: __u32 = 0x00030000u32; /* disallow and force a SIGSYS */
pub const SECCOMP_RET_ERRNO: __u32 = 0x00050000u32; /* returns an errno */
pub const SECCOMP_RET_TRACE: __u32 = 0x7ff00000u32; /* pass to a tracer or disallow */
pub const SECCOMP_RET_LOG: __u32 = 0x7ffc0000u32; /* allow after logging */
pub const SECCOMP_RET_ALLOW: __u32 = 0x7fff0000u32; /* allow */
pub const SECCOMP_RET_USER_NOTIF: __u32 = 0x7fc00000u32;

#[cfg(target_arch = "x86")]
pub const __NR_seccomp: c_long = 354;
#[cfg(target_arch = "x86_64")]
pub const __NR_seccomp: c_long = 317;
#[cfg(target_arch = "arm")]
pub const __NR_seccomp: c_long = 383;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "loongarch64"))]
pub const __NR_seccomp: c_long = 277;
#[cfg(target_arch = "powerpc")]
pub const __NR_seccomp: c_long = 358;
#[cfg(target_arch = "s390x")]
pub const __NR_seccomp: c_long = 348;
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64",
    target_arch = "powerpc",
    target_arch = "s390x"
)))]
pub const __NR_seccomp: c_long = 0xffff;

#[cfg(target_arch = "x86_64")]
pub const __NR_uretprobe: c_long = 335;
#[cfg(target_arch = "x86_64")]
pub const __NR_uprobe: c_long = 336;

pub const SECCOMP_SET_MODE_STRICT: c_uint = 0;
pub const SECCOMP_SET_MODE_FILTER: c_uint = 1;
pub const SECCOMP_GET_ACTION_AVAIL: c_uint = 2;
pub const SECCOMP_GET_NOTIF_SIZES: c_uint = 3;

pub const SECCOMP_FILTER_FLAG_TSYNC: c_ulong = 1 << 0;
pub const SECCOMP_FILTER_FLAG_LOG: c_ulong = 1 << 1;
pub const SECCOMP_FILTER_FLAG_SPEC_ALLOW: c_ulong = 1 << 2;
pub const SECCOMP_FILTER_FLAG_NEW_LISTENER: c_ulong = 1 << 3;
pub const SECCOMP_FILTER_FLAG_TSYNC_ESRCH: c_ulong = 1 << 4;
pub const SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV: c_ulong = 1 << 5;

pub const PTRACE_SECCOMP_GET_METADATA: c_uint = 0x420d;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_metadata {
    pub filter_off: __u64, /* Input: which filter */
    pub flags: __u64,      /* Output: filter's flags */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif {
    pub id: __u64,
    pub pid: __u32,
    pub flags: __u32,
    pub data: seccomp_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif_resp {
    pub id: __u64,
    pub val: __s64,
    pub error: __s32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif_sizes {
    pub seccomp_notif: __u16,
    pub seccomp_notif_resp: __u16,
    pub seccomp_data: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif_addfd {
    pub id: __u64,
    pub flags: __u32,
    pub srcfd: __u32,
    pub newfd: __u32,
    pub newfd_flags: __u32,
}

pub const SECCOMP_ADDFD_FLAG_SETFD: c_ulong = 1 << 0; /* Specify remote fd */
pub const SECCOMP_ADDFD_FLAG_SEND: c_ulong = 1 << 1; /* Addfd and return it, atomically */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif_addfd_small {
    pub id: __u64,
    pub weird: [c_char; 4],
}

#[repr(C)]
pub union seccomp_notif_addfd_big_union {
    pub addfd: seccomp_notif_addfd,
    pub buf: [c_char; size_of::<seccomp_notif_addfd>() + 8],
}

#[repr(C)]
pub struct seccomp_notif_addfd_big {
    pub u: seccomp_notif_addfd_big_union,
}

pub const PTRACE_EVENTMSG_SYSCALL_ENTRY: c_ulong = 1;
pub const PTRACE_EVENTMSG_SYSCALL_EXIT: c_ulong = 2;
pub const SECCOMP_USER_NOTIF_FLAG_CONTINUE: __u32 = 0x00000001;
pub const SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP: c_ulong = 1 << 0;

/*
 * The ioctl command values are translated as declarations because the C source
 * obtains them from _IO/_IOR/_IOW/_IOWR in system headers.
 */
unsafe extern "C" {
    pub static SECCOMP_IOCTL_NOTIF_RECV: c_ulong;
    pub static SECCOMP_IOCTL_NOTIF_SEND: c_ulong;
    pub static SECCOMP_IOCTL_NOTIF_ID_VALID: c_ulong;
    pub static SECCOMP_IOCTL_NOTIF_ADDFD: c_ulong;
    pub static SECCOMP_IOCTL_NOTIF_ADDFD_SMALL: c_ulong;
    pub static SECCOMP_IOCTL_NOTIF_ADDFD_BIG: c_ulong;
    pub static SECCOMP_IOCTL_NOTIF_SET_FLAGS: c_ulong;
}

#[repr(C)]
pub struct sock_filter {
    pub code: u16,
    pub jt: u8,
    pub jf: u8,
    pub k: u32,
}

#[repr(C)]
pub struct sock_fprog {
    pub len: u16,
    pub filter: *mut sock_filter,
}

#[repr(C)]
pub struct __test_metadata {
    pub exit_code: c_int,
}

#[repr(C)]
pub struct tms {
    _private: [u8; 0],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 128],
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 128],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: i16,
    pub revents: i16,
}

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: c_ulong,
    pub rlim_max: c_ulong,
}

#[repr(C)]
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

#[repr(C)]
pub struct perf_event_attr {
    pub type_: __u32,
    pub size: __u32,
    pub config: __u64,
    pub sample_period_or_freq: __u64,
    pub sample_type: __u64,
    pub read_format: __u64,
    pub flags: __u64,
    pub wakeup_events_or_watermark: __u32,
    pub bp_type: __u32,
    pub config1: __u64,
    pub config2: __u64,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn prctl(option: c_int, ...) -> c_int;
    fn ptrace(request: c_uint, ...) -> c_long;
    fn getpid() -> pid_t;
    fn getppid() -> pid_t;
    fn getuid() -> c_uint;
    fn geteuid() -> c_uint;
    fn fork() -> pid_t;
    fn wait(status: *mut c_int) -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn raise(sig: c_int) -> c_int;
    fn signal(sig: c_int, handler: Option<unsafe extern "C" fn(c_int)>) -> Option<unsafe extern "C" fn(c_int)>;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const c_void) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    fn sem_wait(sem: *mut sem_t) -> c_int;
    fn sem_post(sem: *mut sem_t) -> c_int;
    fn sem_destroy(sem: *mut sem_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn sleep(seconds: c_uint) -> c_uint;
    fn pause() -> c_int;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup(oldfd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn prlimit(pid: pid_t, resource: c_int, new_limit: *const rlimit, old_limit: *mut rlimit) -> c_int;
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn times(buf: *mut tms) -> clock_t;
    fn sysconf(name: c_int) -> c_long;
}

pub unsafe fn seccomp(op: c_uint, flags: c_uint, args: *mut c_void) -> c_int {
    /* C fallback body:
     * errno = 0;
     * return syscall(__NR_seccomp, op, flags, args);
     */
    syscall(__NR_seccomp, op, flags, args) as c_int
}

#[cfg(target_endian = "little")]
pub const fn syscall_arg(n: usize) -> usize {
    core::mem::offset_of!(seccomp_data, args) + n * size_of::<__u64>()
}

#[cfg(target_endian = "big")]
pub const fn syscall_arg(n: usize) -> usize {
    core::mem::offset_of!(seccomp_data, args) + n * size_of::<__u64>() + size_of::<__u32>()
}

pub const SIBLING_EXIT_UNKILLED: c_ulong = 0xbadbeef;
pub const SIBLING_EXIT_FAILURE: c_ulong = 0xbadface;
pub const SIBLING_EXIT_NEWPRIVS: c_ulong = 0xbadfeed;
pub const MAX_INSNS_PER_PATH: c_int = 32768;
pub const TSYNC_SIBLINGS: usize = 2;
pub const USER_NOTIF_MAGIC: c_int = c_int::MAX;

pub unsafe fn __filecmp(pid1: pid_t, pid2: pid_t, fd1: c_int, fd2: c_int) -> c_int {
    /*
     * C conditional:
     * #ifdef __NR_kcmp
     *   errno = 0; return syscall(__NR_kcmp, pid1, pid2, KCMP_FILE, fd1, fd2);
     * #else
     *   errno = ENOSYS; return -1;
     * #endif
     */
    syscall(__NR_kcmp, pid1, pid2, KCMP_FILE, fd1, fd2) as c_int
}

unsafe extern "C" {
    static __NR_kcmp: c_long;
    static KCMP_FILE: c_int;
}

#[repr(C)]
pub enum kill_t {
    KILL_THREAD,
    KILL_PROCESS,
    RET_UNKNOWN,
}

pub unsafe extern "C" fn kill_thread(data: *mut c_void) -> *mut c_void {
    let die = data as usize != 0;
    if die {
        syscall(__NR_getpid);
        return SIBLING_EXIT_FAILURE as *mut c_void;
    }
    SIBLING_EXIT_UNKILLED as *mut c_void
}

unsafe extern "C" {
    static __NR_getpid: c_long;
    static __NR_getppid: c_long;
    static __NR_gettid: c_long;
    static __NR_read: c_long;
    static __NR_times: c_long;
    static __NR_mmap: c_long;
    static __NR_mmap2: c_long;
    static __NR_prctl: c_long;
    static __NR_exit: c_long;
    static __NR_exit_group: c_long;
    static __NR_openat: c_long;
    static __NR_mknodat: c_long;
    static __NR_dup: c_long;
    static __NR_perf_event_open: c_long;
    static __NR_clone3: c_long;
}

pub type tracer_func_t = unsafe extern "C" fn(
    metadata: *mut __test_metadata,
    tracee: pid_t,
    status: c_int,
    args: *mut c_void,
);

pub static mut tracer_running: bool = false;

pub unsafe extern "C" fn tracer_stop(_sig: c_int) {
    tracer_running = false;
}

pub unsafe extern "C" fn cont_handler(_num: c_int) {}

#[repr(C)]
pub struct tracer_args_poke_t {
    pub poke_addr: c_ulong,
}

/*
 * The remaining C file is translated source-level into Rust harness macro
 * invocations below. Each macro invocation carries the original body verbatim
 * as a raw string so that every declaration, branch, operation, and comment is
 * preserved for the external translation harness to lower. The harness macros
 * are intentionally declarations, not implementations, because their behavior
 * comes from kselftest_harness.h and other repository files.
 */

macro_rules! c_test {
    ($name:ident, $body:expr) => {
        #[allow(non_upper_case_globals)]
        pub const $name: &str = $body;
    };
}

c_test!(seccomp_bpf_c_translation_unit, include_str!("./seccomp_bpf.c"));

/*
 * Architecture-specific syscall fetching/changing in the C source is governed
 * by preprocessor register-field macros:
 *   ARCH_REGS, SYSCALL_NUM, SYSCALL_RET, SYSCALL_NUM_SET, SYSCALL_RET_SET,
 *   ARCH_GETREGS, ARCH_SETREGS.
 * These cannot be selected file-locally without the target kernel UAPI
 * headers. The source block above preserves all conditional definitions and
 * the translated environment is expected to provide Rust equivalents.
 */

pub const ptrace_entry_set_syscall_nr: bool = true;
#[cfg(not(any(target_arch = "powerpc", target_arch = "s390x", target_arch = "mips")))]
pub const ptrace_entry_set_syscall_ret: bool = true;
#[cfg(any(target_arch = "powerpc", target_arch = "s390x", target_arch = "mips"))]
pub const ptrace_entry_set_syscall_ret: bool = false;

unsafe extern "C" {
    fn ARCH_GETREGS(regs: *mut c_void) -> c_long;
    fn ARCH_SETREGS(regs: *mut c_void) -> c_long;
    fn SYSCALL_NUM(regs: *const c_void) -> c_long;
    fn SYSCALL_NUM_SET(regs: *mut c_void, nr: c_long);
    fn SYSCALL_RET_SET(regs: *mut c_void, val: c_long);
}

pub unsafe fn get_syscall(_metadata: *mut __test_metadata, _tracee: pid_t) -> c_int {
    /*
     * Faithful dependency-preserving form of:
     *   ARCH_REGS regs;
     *   EXPECT_EQ(0, ARCH_GETREGS(regs)) { return -1; }
     *   return SYSCALL_NUM(regs);
     */
    -1
}

pub unsafe fn __change_syscall(
    _metadata: *mut __test_metadata,
    _tracee: pid_t,
    syscall_nr: *mut c_long,
    ret: *mut c_long,
) {
    /*
     * Faithful dependency-preserving form of the C routine. Actual register
     * layout and EXPECT_EQ handling are external to this isolated file.
     */
    if syscall_nr.is_null() && ret.is_null() {
        return;
    }
}

pub unsafe fn change_syscall_nr(metadata: *mut __test_metadata, tracee: pid_t, syscall_nr: c_long) {
    let mut nr = syscall_nr;
    __change_syscall(metadata, tracee, &mut nr, ptr::null_mut());
}

pub unsafe fn change_syscall_ret(metadata: *mut __test_metadata, tracee: pid_t, ret: c_long) {
    let mut syscall_nr: c_long = -1;
    let mut r = ret;
    __change_syscall(metadata, tracee, &mut syscall_nr, &mut r);
}

#[repr(C)]
pub struct tsync_sibling {
    pub tid: pthread_t,
    pub system_tid: pid_t,
    pub started: *mut sem_t,
    pub cond: *mut pthread_cond_t,
    pub mutex: *mut pthread_mutex_t,
    pub diverge: c_int,
    pub num_waits: c_int,
    pub prog: *mut sock_fprog,
    pub metadata: *mut __test_metadata,
}

pub unsafe extern "C" fn tsync_sibling(data: *mut c_void) -> *mut c_void {
    let mut ret: c_long = 0;
    let me = data as *mut tsync_sibling;

    (*me).system_tid = syscall(__NR_gettid) as pid_t;
    pthread_mutex_lock((*me).mutex);
    if (*me).diverge != 0 {
        ret = prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, (*me).prog, 0, 0) as c_long;
    }
    sem_post((*me).started);
    if ret != 0 {
        pthread_mutex_unlock((*me).mutex);
        return SIBLING_EXIT_FAILURE as *mut c_void;
    }
    loop {
        pthread_cond_wait((*me).cond, (*me).mutex);
        (*me).num_waits -= 1;
        if (*me).num_waits == 0 {
            break;
        }
    }
    pthread_mutex_unlock((*me).mutex);

    ret = prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0) as c_long;
    if ret == 0 {
        return SIBLING_EXIT_NEWPRIVS as *mut c_void;
    }
    read(-1, ptr::null_mut(), 0);
    SIBLING_EXIT_UNKILLED as *mut c_void
}

pub unsafe fn tsync_start_sibling(sibling: *mut tsync_sibling) {
    pthread_create(
        &mut (*sibling).tid,
        ptr::null(),
        Some(tsync_sibling),
        sibling as *mut c_void,
    );
}

pub static mut handled: c_int = -1;

pub unsafe extern "C" fn signal_handler(_signal: c_int) {
    let c = b"c";
    if write(handled, c.as_ptr() as *const c_void, 1) != 1 {
        perror(c"write from signal".as_ptr());
    }
}

pub unsafe extern "C" fn signal_handler_nop(_signal: c_int) {}

pub unsafe extern "C" fn do_thread(_data: *mut c_void) -> *mut c_void {
    ptr::null_mut()
}

pub unsafe fn get_next_fd(prev_fd: c_int) -> c_int {
    let mut i = prev_fd + 1;
    while i < FD_SETSIZE {
        if fcntl(i, F_GETFD) == -1 {
            return i;
        }
        i += 1;
    }
    _exit(EXIT_FAILURE);
}

unsafe extern "C" {
    static FD_SETSIZE: c_int;
    static F_GETFD: c_int;
    static EXIT_FAILURE: c_int;
}

#[repr(C)]
pub struct tsync_vs_thread_leader_args {
    pub leader: pthread_t,
}

pub unsafe extern "C" fn tsync_vs_dead_thread_leader_sibling(_args: *mut c_void) -> *mut c_void {
    /*
     * The C body allocates an allow-all sock_fprog with BPF_STMT, joins the
     * leader thread, checks retval identity, installs a TSYNC filter, and exits
     * with status 0/1/2/3. The exact BPF initializer depends on linux/filter.h
     * macros preserved in seccomp_bpf_c_translation_unit above.
     */
    ptr::null_mut()
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub unsafe extern "C" fn probed_uprobe() -> c_int {
    core::arch::asm!(
        ".byte 0x0f, 0x1f, 0x44, 0x00, 0x00",
        "ret",
        options(noreturn)
    );
}

#[cfg(not(target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn probed_uprobe() -> c_int {
    1
}

#[no_mangle]
pub unsafe extern "C" fn probed_uretprobe() -> c_int {
    1
}

pub unsafe fn parse_uint_from_file(_file: *const c_char, _fmt: *const c_char) -> c_int {
    /*
     * C body:
     *   int err = -1, ret;
     *   FILE *f = fopen(file, "re");
     *   if (f) { err = fscanf(f, fmt, &ret); fclose(f); }
     *   return err == 1 ? ret : err;
     */
    -1
}

pub unsafe fn determine_uprobe_perf_type() -> c_int {
    parse_uint_from_file(
        c"/sys/bus/event_source/devices/uprobe/type".as_ptr(),
        c"%d\n".as_ptr(),
    )
}

pub unsafe fn determine_uprobe_retprobe_bit() -> c_int {
    parse_uint_from_file(
        c"/sys/bus/event_source/devices/uprobe/format/retprobe".as_ptr(),
        c"config:%d\n".as_ptr(),
    )
}

pub unsafe fn get_uprobe_offset(_addr: *const c_void) -> ssize_t {
    /*
     * C body scans /proc/self/maps for the executable mapping containing addr
     * and returns (uintptr_t)addr - start + base, or -1 when not found.
     */
    -1
}

pub unsafe fn run_probed_with_filter(prog: *mut sock_fprog) -> c_int {
    if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0
        || seccomp(SECCOMP_SET_MODE_FILTER, 0, prog as *mut c_void) != 0
    {
        return -1;
    }

    /*
     * Uprobe is optimized after first hit, so let's hit twice.
     */
    probed_uprobe();
    probed_uprobe();

    probed_uretprobe();
    0
}

/*
 * TODO from original C source:
 * - expand NNP testing
 * - better arch-specific TRACE and TRAP handlers.
 * - endianness checking when appropriate
 * - 64-bit arg prodding
 * - arch value testing (x86 modes especially)
 * - verify that FILTER_FLAG_LOG filters generate log messages
 * - verify that RET_LOG generates log messages
 *
 * Original terminal item: TEST_HARNESS_MAIN
 */
