/* SPDX-License-Identifier: GPL-2.0 */

// The original header is active only when CONFIG_BPF_EVENTS is enabled.
// Its included tracepoint definitions are supplied by the surrounding build.

#[allow(unused_macros)]
macro_rules! __perf_count { ($c:expr) => { $c }; }

#[allow(unused_macros)]
macro_rules! __perf_task { ($t:expr) => { $t }; }

/* Cast any integer, pointer, or small struct to u64, preserving its bytes. */
#[inline]
pub unsafe fn __cast_to_u64<T: Copy>(x: T) -> u64 {
    use core::mem::{size_of, MaybeUninit};

    let mut dst = MaybeUninit::<u64>::uninit();
    let n = size_of::<T>();
    core::ptr::copy_nonoverlapping(
        (&x as *const T).cast::<u8>(),
        dst.as_mut_ptr().cast::<u8>(),
        n,
    );
    dst.assume_init()
}

#[allow(unused_macros)]
macro_rules! __CAST1 { ($a:expr) => { unsafe { $crate::__cast_to_u64($a) } }; }
#[allow(unused_macros)]
macro_rules! __CAST2 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST1!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST3 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST2!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST4 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST3!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST5 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST4!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST6 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST5!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST7 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST6!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST8 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST7!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST9 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST8!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST10 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST9!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST11 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST10!($($rest)*)) }; }
#[allow(unused_macros)]
macro_rules! __CAST12 { ($a:expr, $($rest:tt)*) => { (__CAST1!($a), __CAST11!($($rest)*)) }; }

/* Tracepoints with more than 12 arguments hit a build error. */

// C __BPF_DECLARE_TRACE and __BPF_DECLARE_TRACE_SYSCALL generate one callback
// per tracepoint.  These declarative forms preserve that generated interface;
// bpf_trace_runN and the tracepoint-specific types are external dependencies.
#[allow(unused_macros)]
macro_rules! __BPF_DECLARE_TRACE {
    ($call:ident, ($($proto:tt)*), ($($args:tt)*)) => {
        #[allow(non_snake_case)]
        unsafe fn __bpf_trace_$call(__data: *mut core::ffi::c_void, $($proto)*) {
            bpf_trace_run!(__data, __CAST1!($($args)*));
        }
    };
}

#[allow(unused_macros)]
macro_rules! __BPF_DECLARE_TRACE_SYSCALL {
    ($call:ident, ($($proto:tt)*), ($($args:tt)*)) => {
        #[allow(non_snake_case)]
        unsafe fn __bpf_trace_$call(__data: *mut core::ffi::c_void, $($proto)*) {
            might_fault!();
            bpf_trace_run!(__data, __CAST1!($($args)*));
        }
    };
}

// The remaining C macros (__DEFINE_EVENT, DECLARE_EVENT_CLASS,
// DECLARE_EVENT_SYSCALL_CLASS, DEFINE_EVENT*, DECLARE_TRACE*) are intentionally
// represented as build-time Rust macro hooks: their bodies depend on the
// tracepoint declarations included by TRACE_INCLUDE(TRACE_INCLUDE_FILE).
#[allow(unused_macros)]
macro_rules! FIRST { ($x:expr $(, $rest:tt)*) => { $x }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
