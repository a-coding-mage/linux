/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of linux/compat.h.  Types supplied by the kernel crate
//! remain external dependencies, as they do in the original header.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* Build-time configuration and C preprocessor interfaces are retained here
 * as Rust configuration notes; syscall-wrapper macros are expanded by the
 * architecture-specific implementation. */

pub const COMPAT_USE_64BIT_TIME: usize = 0;

#[repr(C)]
pub struct compat_iovec {
    pub iov_base: compat_uptr_t,
    pub iov_len: compat_size_t,
}

#[repr(C)]
pub struct compat_sigaltstack {
    pub ss_sp: compat_uptr_t,
    pub ss_flags: i32,
    pub ss_size: compat_size_t,
}
pub type compat_stack_t = compat_sigaltstack;

pub type compat_uid_t = __compat_uid32_t;
pub type compat_gid_t = __compat_gid32_t;

#[repr(C)]
pub struct compat_tms {
    pub tms_utime: compat_clock_t,
    pub tms_stime: compat_clock_t,
    pub tms_cutime: compat_clock_t,
    pub tms_cstime: compat_clock_t,
}

pub const _COMPAT_NSIG_WORDS: usize = _COMPAT_NSIG as usize / _COMPAT_NSIG_BPW as usize;
#[repr(C)]
pub struct compat_sigset_t { pub sig: [compat_sigset_word; _COMPAT_NSIG_WORDS] }

#[repr(C)]
pub struct compat_sigaction {
    #[cfg(not(feature = "arch_has_irix_sigaction"))]
    pub sa_handler: compat_uptr_t,
    #[cfg(not(feature = "arch_has_irix_sigaction"))]
    pub sa_flags: compat_ulong_t,
    #[cfg(feature = "arch_has_irix_sigaction")]
    pub sa_flags: compat_uint_t,
    #[cfg(feature = "arch_has_irix_sigaction")]
    pub sa_handler: compat_uptr_t,
    #[cfg(feature = "arch_has_sa_restorer")]
    pub sa_restorer: compat_uptr_t,
    pub sa_mask: compat_sigset_t,
}

#[repr(C)]
pub union compat_sigval { pub sival_int: compat_int_t, pub sival_ptr: compat_uptr_t }
pub type compat_sigval_t = compat_sigval;

#[repr(C)]
pub struct compat_siginfo {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub _sifields: compat_siginfo_fields,
}
#[repr(C)]
pub union compat_siginfo_fields {
    pub _pad: [i32; 125],
    pub _kill: compat_siginfo_kill,
    pub _timer: compat_siginfo_timer,
    pub _rt: compat_siginfo_rt,
    pub _sigchld: compat_siginfo_sigchld,
    pub _sigfault: compat_siginfo_sigfault,
    pub _sigpoll: compat_siginfo_sigpoll,
    pub _sigsys: compat_siginfo_sigsys,
}
#[repr(C)] pub struct compat_siginfo_kill { pub _pid: compat_pid_t, pub _uid: __compat_uid32_t }
#[repr(C)] pub struct compat_siginfo_timer { pub _tid: compat_timer_t, pub _overrun: i32, pub _sigval: compat_sigval_t }
#[repr(C)] pub struct compat_siginfo_rt { pub _pid: compat_pid_t, pub _uid: __compat_uid32_t, pub _sigval: compat_sigval_t }
#[repr(C)] pub struct compat_siginfo_sigchld { pub _pid: compat_pid_t, pub _uid: __compat_uid32_t, pub _status: i32, pub _utime: compat_clock_t, pub _stime: compat_clock_t }
#[repr(C)] pub struct compat_siginfo_sigfault { pub _addr: compat_uptr_t, pub _trapno: i32, pub _addr_lsb: i16, pub _lower: compat_uptr_t, pub _upper: compat_uptr_t, pub _pkey: u32, pub _data: compat_ulong_t, pub _type: u32, pub _flags: u32 }
#[repr(C)] pub struct compat_siginfo_sigpoll { pub _band: compat_long_t, pub _fd: i32 }
#[repr(C)] pub struct compat_siginfo_sigsys { pub _call_addr: compat_uptr_t, pub _syscall: i32, pub _arch: u32 }
pub type compat_siginfo_t = compat_siginfo;

#[repr(C)] pub struct compat_rlimit { pub rlim_cur: compat_ulong_t, pub rlim_max: compat_ulong_t }
#[repr(C)] pub struct compat_flock { pub l_type: i16, pub l_whence: i16, pub l_start: compat_off_t, pub l_len: compat_off_t, pub l_pid: compat_pid_t }
#[repr(C, packed)] pub struct compat_flock64 { pub l_type: i16, pub l_whence: i16, pub l_start: compat_loff_t, pub l_len: compat_loff_t, pub l_pid: compat_pid_t }
#[repr(C)] pub struct compat_rusage { pub ru_utime: old_timeval32, pub ru_stime: old_timeval32, pub ru_maxrss: compat_long_t, pub ru_ixrss: compat_long_t, pub ru_idrss: compat_long_t, pub ru_isrss: compat_long_t, pub ru_minflt: compat_long_t, pub ru_majflt: compat_long_t, pub ru_nswap: compat_long_t, pub ru_inblock: compat_long_t, pub ru_oublock: compat_long_t, pub ru_msgsnd: compat_long_t, pub ru_msgrcv: compat_long_t, pub ru_nsignals: compat_long_t, pub ru_nvcsw: compat_long_t, pub ru_nivcsw: compat_long_t }
#[repr(C)] pub struct compat_dirent { pub d_ino: u32, pub d_off: compat_off_t, pub d_reclen: u16, pub d_name: [i8; 256] }
#[repr(C)] pub struct compat_ustat { pub f_tfree: compat_daddr_t, pub f_tinode: compat_ino_t, pub f_fname: [i8; 6], pub f_fpack: [i8; 6] }
#[repr(C)] pub struct compat_robust_list { pub next: compat_uptr_t }
#[repr(C)] pub struct compat_robust_list_head { pub list: compat_robust_list, pub futex_offset: compat_long_t, pub list_op_pending: compat_uptr_t }
#[repr(C)] pub struct compat_keyctl_kdf_params { pub hashname: compat_uptr_t, pub otherinfo: compat_uptr_t, pub otherinfolen: u32, pub __spare: [u32; 8] }

extern "C" {
    pub fn set_compat_user_sigmask(umask: *const compat_sigset_t, sigsetsize: usize) -> i32;
    pub fn put_compat_rusage(rusage: *const rusage, compat: *mut compat_rusage) -> i32;
    pub fn get_compat_sigset(set: *mut sigset_t, compat: *const compat_sigset_t) -> i32;
    pub fn compat_restore_altstack(uss: *const compat_stack_t) -> i32;
    pub fn __compat_save_altstack(uss: *mut compat_stack_t, sp: usize) -> i32;
    pub fn compat_get_bitmap(mask: *mut usize, umask: *const compat_ulong_t, bitmap_size: usize) -> i64;
    pub fn kcompat_sys_statfs64(pathname: *const i8, sz: compat_size_t, buf: *mut compat_statfs64) -> i32;
    pub fn kcompat_sys_fstatfs64(fd: u32, sz: compat_size_t, buf: *mut compat_statfs64) -> i32;
}

#[inline]
pub unsafe fn in_compat_syscall() -> bool { is_compat_task() }
pub const BITS_PER_COMPAT_LONG: usize = 8 * core::mem::size_of::<compat_long_t>();
#[inline] pub const fn BITS_TO_COMPAT_LONGS(bits: usize) -> usize { (bits + BITS_PER_COMPAT_LONG - 1) / BITS_PER_COMPAT_LONG }

/* The remaining architecture-selected syscall prototypes are declarations
 * only in the source header. Their Rust ABI declarations are supplied by the
 * architecture syscall layer, preserving CONFIG_ARCH_HAS_SYSCALL_WRAPPER and
 * the __ARCH_WANT_* conditional interfaces. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
