/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/trace_printk.h. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FtraceDumpMode {
    DumpNone,
    DumpAll,
    DumpOrig,
    DumpParam,
}

#[cfg(feature = "CONFIG_TRACING")]
extern "C" {
    pub fn tracing_on();
    pub fn tracing_off();
    pub fn tracing_is_on() -> ::core::ffi::c_int;
    pub fn tracing_snapshot();
    pub fn tracing_snapshot_alloc();
    pub fn tracing_start();
    pub fn tracing_stop();

    pub fn __trace_bprintk(ip: ::core::ffi::c_ulong, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn __trace_printk(ip: ::core::ffi::c_ulong, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn __trace_bputs(ip: ::core::ffi::c_ulong, string: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn __trace_puts(ip: ::core::ffi::c_ulong, string: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn trace_dump_stack(skip: ::core::ffi::c_int);
    pub fn __ftrace_vbprintk(ip: ::core::ffi::c_ulong, fmt: *const ::core::ffi::c_char, ap: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn __ftrace_vprintk(ip: ::core::ffi::c_ulong, fmt: *const ::core::ffi::c_char, ap: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn ftrace_dump(oops_dump_mode: FtraceDumpMode);
}

#[cfg(feature = "CONFIG_TRACING")]
#[inline(always)]
pub unsafe fn ____trace_printk_check_format(_fmt: *const ::core::ffi::c_char, ...) {}

#[cfg(feature = "CONFIG_TRACING")]
#[macro_export]
macro_rules! __trace_printk_check_format {
    ($fmt:expr $(, $args:expr)*) => {{
        if false {
            unsafe { $crate::____trace_printk_check_format($fmt $(, $args)*) };
        }
    }};
}

/* The original macros use __builtin_constant_p, __stringify, _THIS_IP_,
 * section placement, and compiler attributes supplied by included headers. */
#[cfg(feature = "CONFIG_TRACING")]
#[macro_export]
macro_rules! trace_printk {
    ($fmt:expr $(, $args:expr)*) => {{
        $crate::do_trace_printk!($fmt $(, $args)*);
    }};
}

#[cfg(feature = "CONFIG_TRACING")]
#[macro_export]
macro_rules! do_trace_printk {
    ($fmt:expr $(, $args:expr)*) => {{
        $crate::__trace_printk_check_format!($fmt $(, $args)*);
        unsafe { $crate::__trace_printk(_THIS_IP_, $fmt $(, $args)*) };
    }};
}

#[cfg(feature = "CONFIG_TRACING")]
#[macro_export]
macro_rules! trace_puts {
    ($str:expr) => {{
        unsafe { $crate::__trace_puts(_THIS_IP_, $str) }
    }};
}

#[cfg(feature = "CONFIG_TRACING")]
#[macro_export]
macro_rules! ftrace_vprintk {
    ($fmt:expr, $vargs:expr) => {{
        unsafe { $crate::__ftrace_vprintk(_THIS_IP_, $fmt, $vargs) }
    }};
}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn tracing_start() {}
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn tracing_stop() {}
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn trace_dump_stack(_skip: ::core::ffi::c_int) {}
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn tracing_on() {}
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn tracing_off() {}
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn tracing_is_on() -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn tracing_snapshot() {}
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn tracing_snapshot_alloc() {}
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub unsafe fn trace_printk(_fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub unsafe fn ftrace_vprintk(_fmt: *const ::core::ffi::c_char, _vargs: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline(always)]
pub fn ftrace_dump(_oops_dump_mode: FtraceDumpMode) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
