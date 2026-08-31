/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2013, Michael Ellerman, IBM Corp.
 */

/* C header guard and include directives removed.  The original header depended
 * on stdint.h, stdio.h, stdbool.h, sys/signal.h, linux/auxvec.h,
 * linux/perf_event.h, asm/cputable.h, reg.h, unistd.h, and conditionally
 * sys/syscall.h.
 */

pub use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type pid_t = c_int;

/* Avoid headaches with PRI?64 - just use %ll? always */
pub type u64 = ::core::ffi::c_ulonglong;
pub type s64 = ::core::ffi::c_longlong;

/* Just for familiarity */
pub type u32 = u32;
pub type u16 = u16;
pub type u8 = u8;

/* #define __cacheline_aligned __attribute__((aligned(128))) */
#[repr(C, align(128))]
pub struct __cacheline_aligned<T>(pub T);

#[macro_export]
macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        (::core::mem::size_of_val(&$x) / ::core::mem::size_of_val(&$x[0]))
    };
}

pub const BIND_CPU_ANY: c_int = -1;

unsafe extern "C" {
    pub fn test_harness_set_timeout(time: u64);
    pub fn test_harness(test_function: Option<unsafe extern "C" fn() -> c_int>, name: *const c_char) -> c_int;

    pub fn read_auxv(buf: *mut c_char, buf_size: ssize_t) -> c_int;
    pub fn find_auxv_entry(type_: c_int, auxv: *mut c_char) -> *mut c_void;
    pub fn get_auxv_entry(type_: c_int) -> *mut c_void;

    pub fn pick_online_cpu() -> c_int;
    pub fn bind_to_cpu(cpu: c_int) -> c_int;

    pub fn parse_intmax(buffer: *const c_char, count: size_t, result: *mut intmax_t, base: c_int) -> c_int;
    pub fn parse_uintmax(buffer: *const c_char, count: size_t, result: *mut uintmax_t, base: c_int) -> c_int;
    pub fn parse_int(buffer: *const c_char, count: size_t, result: *mut c_int, base: c_int) -> c_int;
    pub fn parse_uint(buffer: *const c_char, count: size_t, result: *mut c_uint, base: c_int) -> c_int;
    pub fn parse_long(buffer: *const c_char, count: size_t, result: *mut c_long, base: c_int) -> c_int;
    pub fn parse_ulong(buffer: *const c_char, count: size_t, result: *mut c_ulong, base: c_int) -> c_int;

    pub fn read_file(path: *const c_char, buf: *mut c_char, count: size_t, len: *mut size_t) -> c_int;
    pub fn write_file(path: *const c_char, buf: *const c_char, count: size_t) -> c_int;
    pub fn read_file_alloc(path: *const c_char, buf: *mut *mut c_char, len: *mut size_t) -> c_int;
    pub fn read_long(path: *const c_char, result: *mut c_long, base: c_int) -> c_int;
    pub fn write_long(path: *const c_char, result: c_long, base: c_int) -> c_int;
    pub fn read_ulong(path: *const c_char, result: *mut c_ulong, base: c_int) -> c_int;
    pub fn write_ulong(path: *const c_char, result: c_ulong, base: c_int) -> c_int;
    pub fn read_debugfs_file(debugfs_file: *const c_char, buf: *mut c_char, count: size_t) -> c_int;
    pub fn write_debugfs_file(debugfs_file: *const c_char, buf: *const c_char, count: size_t) -> c_int;
    pub fn read_debugfs_int(debugfs_file: *const c_char, result: *mut c_int) -> c_int;
    pub fn write_debugfs_int(debugfs_file: *const c_char, result: c_int) -> c_int;
    pub fn read_sysfs_file(debugfs_file: *mut c_char, result: *mut c_char, result_size: size_t) -> c_int;
    pub fn perf_event_open_counter(type_: c_uint, config: c_ulong, group_fd: c_int) -> c_int;
    pub fn perf_event_enable(fd: c_int) -> c_int;
    pub fn perf_event_disable(fd: c_int) -> c_int;
    pub fn perf_event_reset(fd: c_int) -> c_int;

    pub fn is_ppc64le() -> bool;
    pub fn using_hash_mmu(using_hash: *mut bool) -> c_int;

    pub fn push_signal_handler(sig: c_int, fn_: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>) -> sigaction;
    pub fn pop_signal_handler(sig: c_int, old_handler: sigaction) -> sigaction;
}

#[repr(C)]
pub struct perf_event_read {
    pub nr: u64,
    pub l1d_misses: u64,
}

/* Types supplied by <sys/signal.h>. */
pub type siginfo_t = libc::siginfo_t;
pub type sigaction = libc::sigaction;

/* Original condition: #if !defined(__GLIBC_PREREQ) || !__GLIBC_PREREQ(2, 30) */
#[inline]
pub unsafe fn gettid() -> pid_t {
    unsafe { libc::syscall(libc::SYS_gettid) as pid_t }
}

pub const AT_HWCAP: c_ulong = 16;
pub const AT_PLATFORM: c_ulong = 15;
pub const AT_BASE_PLATFORM: c_ulong = 24;
pub const AT_HWCAP2: c_ulong = 26;

#[inline]
pub unsafe fn have_hwcap(ftr: c_ulong) -> bool {
    unsafe { ((get_auxv_entry(AT_HWCAP as c_int) as c_ulong) & ftr) == ftr }
}

/* Original had an #ifdef AT_HWCAP2 fallback returning false when absent. */
#[inline]
pub unsafe fn have_hwcap2(ftr2: c_ulong) -> bool {
    unsafe { ((get_auxv_entry(AT_HWCAP2 as c_int) as c_ulong) & ftr2) == ftr2 }
}

#[inline]
pub unsafe fn auxv_base_platform() -> *mut c_char {
    unsafe { get_auxv_entry(AT_BASE_PLATFORM as c_int) as *mut c_char }
}

#[inline]
pub unsafe fn auxv_platform() -> *mut c_char {
    unsafe { get_auxv_entry(AT_PLATFORM as c_int) as *mut c_char }
}

/* Yes, this is evil */
#[macro_export]
macro_rules! FAIL_IF {
    ($x:expr) => {
        do {
            if $x {
                unsafe {
                    libc::fprintf(
                        libc::stderr,
                        b"[FAIL] Test FAILED on line %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        line!() as ::core::ffi::c_int,
                    );
                }
                return 1;
            }
        }
    };
}

#[macro_export]
macro_rules! FAIL_IF_MSG {
    ($x:expr, $msg:expr) => {
        do {
            if $x {
                unsafe {
                    libc::fprintf(
                        libc::stderr,
                        b"[FAIL] Test FAILED on line %d: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        line!() as ::core::ffi::c_int,
                        $msg,
                    );
                }
                return 1;
            }
        }
    };
}

#[macro_export]
macro_rules! FAIL_IF_EXIT {
    ($x:expr) => {
        do {
            if $x {
                unsafe {
                    libc::fprintf(
                        libc::stderr,
                        b"[FAIL] Test FAILED on line %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        line!() as ::core::ffi::c_int,
                    );
                    libc::_exit(1);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! FAIL_IF_EXIT_MSG {
    ($x:expr, $msg:expr) => {
        do {
            if $x {
                unsafe {
                    libc::fprintf(
                        libc::stderr,
                        b"[FAIL] Test FAILED on line %d: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        line!() as ::core::ffi::c_int,
                        $msg,
                    );
                    libc::_exit(1);
                }
            }
        }
    };
}

/* The test harness uses this, yes it's gross */
pub const MAGIC_SKIP_RETURN_VALUE: c_int = 99;

#[macro_export]
macro_rules! SKIP_IF {
    ($x:expr) => {
        do {
            if $x {
                unsafe {
                    libc::fprintf(
                        libc::stderr,
                        b"[SKIP] Test skipped on line %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        line!() as ::core::ffi::c_int,
                    );
                }
                return $crate::MAGIC_SKIP_RETURN_VALUE;
            }
        }
    };
}

#[macro_export]
macro_rules! SKIP_IF_MSG {
    ($x:expr, $msg:expr) => {
        do {
            if $x {
                unsafe {
                    libc::fprintf(
                        libc::stderr,
                        b"[SKIP] Test skipped on line %d: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        line!() as ::core::ffi::c_int,
                        $msg,
                    );
                }
                return $crate::MAGIC_SKIP_RETURN_VALUE;
            }
        }
    };
}

#[macro_export]
macro_rules! _str {
    ($s:tt) => {
        stringify!($s)
    };
}

#[macro_export]
macro_rules! str {
    ($s:tt) => {
        stringify!($s)
    };
}

#[macro_export]
macro_rules! sigsafe_err {
    ($msg:expr) => {{
        let nbytes: $crate::ssize_t;
        nbytes = unsafe {
            libc::write(
                libc::STDERR_FILENO,
                $msg as *const ::core::ffi::c_void,
                libc::strlen($msg as *const ::core::ffi::c_char),
            )
        };
        let _ = nbytes;
    }};
}

/* POWER9 feature */
pub const PPC_FEATURE2_ARCH_3_00: c_ulong = 0x00800000;

/* POWER10 feature */
pub const PPC_FEATURE2_ARCH_3_1: c_ulong = 0x00040000;

/* POWER10 features */
pub const PPC_FEATURE2_MMA: c_ulong = 0x00020000;

#[cfg(target_arch = "powerpc64")]
#[macro_export]
macro_rules! UCONTEXT_NIA {
    ($UC:expr) => {
        (*$UC).uc_mcontext.gp_regs[PT_NIP]
    };
}

#[cfg(target_arch = "powerpc64")]
#[macro_export]
macro_rules! UCONTEXT_MSR {
    ($UC:expr) => {
        (*$UC).uc_mcontext.gp_regs[PT_MSR]
    };
}

#[cfg(target_arch = "powerpc")]
#[macro_export]
macro_rules! UCONTEXT_NIA {
    ($UC:expr) => {
        (*(*$UC).uc_mcontext.uc_regs).gregs[PT_NIP]
    };
}

#[cfg(target_arch = "powerpc")]
#[macro_export]
macro_rules! UCONTEXT_MSR {
    ($UC:expr) => {
        (*(*$UC).uc_mcontext.uc_regs).gregs[PT_MSR]
    };
}

#[cfg(not(any(target_arch = "powerpc64", target_arch = "powerpc")))]
compile_error!("implement UCONTEXT_NIA");
