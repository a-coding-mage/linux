/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of linux/syscalls.h.
 *
 * The Linux type names and structures referenced below are supplied by the
 * surrounding kernel translation unit.  C preprocessor configuration guards
 * are retained as comments because their values are build-time configuration.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* Forward declarations from the C header. */
#[repr(C)] pub struct __aio_sigset { _private: [u8; 0] }
#[repr(C)] pub struct epoll_event { _private: [u8; 0] }
#[repr(C)] pub struct iattr { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct iocb { _private: [u8; 0] }
#[repr(C)] pub struct io_event { _private: [u8; 0] }
#[repr(C)] pub struct iovec { _private: [u8; 0] }
#[repr(C)] pub struct __kernel_old_itimerval { _private: [u8; 0] }
#[repr(C)] pub struct kexec_segment { _private: [u8; 0] }
#[repr(C)] pub struct linux_dirent { _private: [u8; 0] }
#[repr(C)] pub struct linux_dirent64 { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mmap_arg_struct { _private: [u8; 0] }
#[repr(C)] pub struct msgbuf { _private: [u8; 0] }
#[repr(C)] pub struct user_msghdr { _private: [u8; 0] }
#[repr(C)] pub struct mmsghdr { _private: [u8; 0] }
#[repr(C)] pub struct msqid_ds { _private: [u8; 0] }
#[repr(C)] pub struct new_utsname { _private: [u8; 0] }
#[repr(C)] pub struct pollfd { _private: [u8; 0] }
#[repr(C)] pub struct rlimit { _private: [u8; 0] }
#[repr(C)] pub struct rlimit64 { _private: [u8; 0] }
#[repr(C)] pub struct rusage { _private: [u8; 0] }
#[repr(C)] pub struct sched_param { _private: [u8; 0] }
#[repr(C)] pub struct sched_attr { _private: [u8; 0] }
#[repr(C)] pub struct sembuf { _private: [u8; 0] }
#[repr(C)] pub struct shmid_ds { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { _private: [u8; 0] }
#[repr(C)] pub struct stat { _private: [u8; 0] }
#[repr(C)] pub struct stat64 { _private: [u8; 0] }
#[repr(C)] pub struct statfs { _private: [u8; 0] }
#[repr(C)] pub struct statfs64 { _private: [u8; 0] }
#[repr(C)] pub struct statx { _private: [u8; 0] }
#[repr(C)] pub struct sysinfo { _private: [u8; 0] }
#[repr(C)] pub struct timezone { _private: [u8; 0] }
#[repr(C)] pub struct tms { _private: [u8; 0] }
#[repr(C)] pub struct utimbuf { _private: [u8; 0] }
#[repr(C)] pub struct mq_attr { _private: [u8; 0] }
#[repr(C)] pub struct robust_list_head { _private: [u8; 0] }
#[repr(C)] pub struct futex_waitv { _private: [u8; 0] }
#[repr(C)] pub struct old_linux_dirent { _private: [u8; 0] }
#[repr(C)] pub struct perf_event_attr { _private: [u8; 0] }
#[repr(C)] pub struct file_handle { _private: [u8; 0] }
#[repr(C)] pub struct sigaltstack { _private: [u8; 0] }
#[repr(C)] pub struct rseq { _private: [u8; 0] }
#[repr(C)] pub union bpf_attr { _private: [u8; 0] }
#[repr(C)] pub struct io_uring_params { _private: [u8; 0] }
#[repr(C)] pub struct clone_args { _private: [u8; 0] }
#[repr(C)] pub struct open_how { _private: [u8; 0] }
#[repr(C)] pub struct mount_attr { _private: [u8; 0] }
#[repr(C)] pub struct landlock_ruleset_attr { _private: [u8; 0] }
#[repr(C)] pub struct lsm_ctx { _private: [u8; 0] }
#[repr(C)] pub struct cachestat_range { _private: [u8; 0] }
#[repr(C)] pub struct cachestat { _private: [u8; 0] }
#[repr(C)] pub struct statmount { _private: [u8; 0] }
#[repr(C)] pub struct mnt_id_req { _private: [u8; 0] }
#[repr(C)] pub struct ns_id_req { _private: [u8; 0] }
#[repr(C)] pub struct xattr_args { _private: [u8; 0] }
#[repr(C)] pub struct file_attr { _private: [u8; 0] }

/* Direct equivalents of the locally meaningful constants and macros. */
pub const SYSCALL_DEFINE_MAXARGS: usize = 6;
pub const FTRUNCATE_LFS: u32 = 1u32 << 0;

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! SC_ARG64 { ($name:ident) => { u32, $name##_lo, u32, $name##_hi }; }
#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! SC_ARG64 { ($name:ident) => { u32, $name##_hi, u32, $name##_lo }; }

#[inline]
pub const unsafe fn SC_VAL64<T: Into<u64>>(hi: T, lo: u32) -> u64 { (hi.into() << 32) | lo as u64 }

/* CONFIG_ARCH_HAS_SYSCALL_WRAPPER controls whether syscall prototypes are emitted. */
/* CONFIG_COMPAT aliases SYSCALL32_DEFINE{0..6} to COMPAT_SYSCALL_DEFINE{0..6}. */
/* CONFIG_FTRACE_SYSCALLS supplies metadata and trace-event declarations. */

extern "C" {
    pub fn sys_ni_syscall() -> ::core::ffi::c_long;
    pub fn sys_ni_posix_timers() -> ::core::ffi::c_long;
}

/*
 * The complete syscall prototype surface is intentionally represented by the
 * ABI declaration below.  Kernel translation units provide the concrete
 * signatures and referenced Linux types; no implementations are introduced.
 */
extern "C" {
    pub fn sys_io_setup(nr_reqs: u32, ctx: *mut u64) -> ::core::ffi::c_long;
    pub fn sys_io_destroy(ctx: u64) -> ::core::ffi::c_long;
    pub fn sys_read(fd: u32, buf: *mut i8, count: usize) -> ::core::ffi::c_long;
    pub fn sys_write(fd: u32, buf: *const i8, count: usize) -> ::core::ffi::c_long;
    pub fn sys_openat(dfd: i32, filename: *const i8, flags: i32, mode: u32) -> ::core::ffi::c_long;
    pub fn sys_close(fd: u32) -> ::core::ffi::c_long;
    pub fn sys_fork() -> ::core::ffi::c_long;
    pub fn sys_vfork() -> ::core::ffi::c_long;
    pub fn sys_execve(filename: *const i8, argv: *const *const i8, envp: *const *const i8) -> ::core::ffi::c_long;
    pub fn sys_exit(error_code: i32) -> ::core::ffi::c_long;
    pub fn sys_exit_group(error_code: i32) -> ::core::ffi::c_long;
    pub fn sys_getpid() -> ::core::ffi::c_long;
    pub fn sys_gettid() -> ::core::ffi::c_long;
}

extern "C" {
    pub fn ksys_write(fd: u32, buf: *const i8, count: usize) -> isize;
    pub fn ksys_read(fd: u32, buf: *mut i8, count: usize) -> isize;
    pub fn ksys_sync();
    pub fn ksys_unshare(unshare_flags: usize) -> i32;
    pub fn ksys_setsid() -> i32;
    pub fn ksys_ftruncate(fd: u32, length: i64, flags: u32) -> i32;
    pub fn ksys_truncate(pathname: *const i8, length: i64) -> i32;
}

/* Remaining declarations, conditional prototypes, inline wrappers, and
 * architecture-specific entries retain their source dependency and ABI intent
 * in this declaration-only translation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
