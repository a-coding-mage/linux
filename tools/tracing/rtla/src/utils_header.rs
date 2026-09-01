// SPDX-License-Identifier: GPL-2.0

// Translated from tracing/rtla/src/utils.h.
// C dependencies: stdint.h, string.h, time.h, sched.h, stdbool.h, stdlib.h,
// and linux/container_of.h.

use core::ffi::{c_char, c_double, c_int, c_long, c_longlong, c_uint, c_void};

pub type size_t = usize;
pub type time_t = c_long;
pub type pid_t = c_int;

// Provided by sched.h in C.
pub type cpu_set_t = c_void;

/*
 * '18446744073709551615\0'
 */
pub const BUFF_U64_STR_SIZE: usize = 24;
pub const MAX_PATH: usize = 1024;
pub const MAX_NICE: c_int = 20;
pub const MIN_NICE: c_int = -19;

// ARRAY_SIZE(x): sizeof(x) / sizeof(*(x))
#[macro_export]
macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        ::core::mem::size_of_val(&$x) / ::core::mem::size_of_val(&$x[0])
    };
}

// Calculate string length at compile time (excluding null terminator).
#[macro_export]
macro_rules! STRING_LENGTH {
    ($s:expr) => {
        ARRAY_SIZE!($s) - ::core::mem::size_of_val(&$s[0])
    };
}

// Compare string with static string, length determined at compile time.
#[macro_export]
macro_rules! strncmp_static {
    ($s1:expr, $s2:expr) => {
        strncmp($s1, $s2, ARRAY_SIZE!($s2))
    };
}

unsafe extern "C" {
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strlen(s: *const c_char) -> size_t;
}

/**
 * str_has_prefix - Test if a string has a given prefix
 * @str: The string to test
 * @prefix: The string to see if @str starts with
 *
 * Returns: true if @str starts with @prefix, false otherwise
 */
#[inline]
pub unsafe fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> bool {
    unsafe { strncmp(str_, prefix, strlen(prefix)) == 0 }
}

unsafe extern "C" {
    pub static mut config_debug: bool;
    pub fn debug_msg(fmt: *const c_char, ...);
    pub fn err_msg(fmt: *const c_char, ...);
    pub fn fatal(fmt: *const c_char, ...);

    pub fn parse_seconds_duration(val: *mut c_char) -> c_long;
    pub fn get_duration(start_time: time_t, output: *mut c_char, output_size: c_int);

    pub fn get_llong_from_str(start: *mut c_char) -> c_longlong;
}

#[inline]
pub unsafe fn update_min(a: *mut u64, b: *mut u64) {
    unsafe {
        if *a > *b {
            *a = *b;
        }
    }
}

#[inline]
pub unsafe fn update_max(a: *mut u64, b: *mut u64) {
    unsafe {
        if *a < *b {
            *a = *b;
        }
    }
}

#[inline]
pub unsafe fn update_sum(a: *mut u64, b: *mut u64) {
    unsafe {
        *a = (*a).wrapping_add(*b);
    }
}

// C condition: defined only when SCHED_ATTR_SIZE_VER0 is not available.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_attr {
    pub size: u32,
    pub sched_policy: u32,
    pub sched_flags: u64,
    pub sched_nice: i32,
    pub sched_priority: u32,
    pub sched_runtime: u64,
    pub sched_deadline: u64,
    pub sched_period: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum stack_format {
    STACK_FORMAT_TRUNCATE,
    STACK_FORMAT_SKIP,
    STACK_FORMAT_FULL,
}

unsafe extern "C" {
    pub fn parse_prio(arg: *mut c_char, sched_param: *mut sched_attr) -> c_int;
    pub fn parse_cpu_set(cpu_list: *mut c_char, set: *mut cpu_set_t) -> c_int;
    pub fn parse_stack_format(arg: *mut c_char) -> c_int;
    pub fn __set_sched_attr(pid: c_int, attr: *mut sched_attr) -> c_int;
    pub fn set_comm_sched_attr(comm_prefix: *const c_char, attr: *mut sched_attr) -> c_int;
    pub fn set_comm_cgroup(comm_prefix: *const c_char, cgroup: *const c_char) -> c_int;
    pub fn set_pid_cgroup(pid: pid_t, cgroup: *const c_char) -> c_int;
    pub fn set_cpu_dma_latency(latency: i32) -> c_int;
    pub fn calloc_fatal(n: size_t, size: size_t) -> *mut c_void;
    pub fn reallocarray_fatal(p: *mut c_void, n: size_t, size: size_t) -> *mut c_void;
    pub fn strdup_fatal(s: *const c_char) -> *mut c_char;
}

// C condition: HAVE_LIBCPUPOWER_SUPPORT.
// When enabled, these are external declarations. Otherwise, the inline
// fallbacks below preserve the header-local behavior.
#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
unsafe extern "C" {
    pub fn save_cpu_idle_disable_state(cpu: c_uint) -> c_int;
    pub fn restore_cpu_idle_disable_state(cpu: c_uint) -> c_int;
    pub fn free_cpu_idle_disable_states();
    pub fn set_deepest_cpu_idle_state(cpu: c_uint, state: c_uint) -> c_int;
}

#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
#[inline]
pub fn have_libcpupower_support() -> c_int {
    1
}

#[cfg(not(HAVE_LIBCPUPOWER_SUPPORT))]
#[inline]
pub fn save_cpu_idle_disable_state(_cpu: c_uint) -> c_int {
    -1
}

#[cfg(not(HAVE_LIBCPUPOWER_SUPPORT))]
#[inline]
pub fn restore_cpu_idle_disable_state(_cpu: c_uint) -> c_int {
    -1
}

#[cfg(not(HAVE_LIBCPUPOWER_SUPPORT))]
#[inline]
pub fn free_cpu_idle_disable_states() {}

#[cfg(not(HAVE_LIBCPUPOWER_SUPPORT))]
#[inline]
pub fn set_deepest_cpu_idle_state(_cpu: c_uint, _state: c_uint) -> c_int {
    -1
}

#[cfg(not(HAVE_LIBCPUPOWER_SUPPORT))]
#[inline]
pub fn have_libcpupower_support() -> c_int {
    0
}

unsafe extern "C" {
    pub fn auto_house_keeping(monitored_cpus: *mut cpu_set_t) -> c_int;

    #[must_use]
    pub fn strtoi(s: *const c_char, res: *mut c_int) -> c_int;
}

#[inline]
pub fn ns_to_usf(x: impl Into<c_double>) -> c_double {
    x.into() / 1000.0
}

#[inline]
pub fn ns_to_per(total: impl Into<c_double>, part: impl Into<c_double>) -> c_double {
    (part.into() * 100.0) / total.into()
}

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum result {
    PASSED = EXIT_SUCCESS as isize,
    ERROR = EXIT_FAILURE as isize,
    FAILED,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
