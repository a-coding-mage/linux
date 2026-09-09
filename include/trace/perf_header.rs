/* SPDX-License-Identifier: GPL-2.0 */

// `TRACE_SYSTEM_VAR` is intentionally undefined by the original header.
// The CONFIG_PERF_EVENTS conditional is a build-time C configuration guard.

// The original include supplies trace-event callback declarations and helper
// macros. It remains an external dependency of this translation.

// C macro equivalents used by generated event callbacks.
#[macro_export]
macro_rules! __perf_count {
    ($c:expr) => {
        __count = $c;
    };
}

#[macro_export]
macro_rules! __perf_task {
    ($t:expr) => {
        __task = $t;
    };
}

// The C header defines the following callback-generating macros. Rust cannot
// concatenate an identifier from `$call` without an external identifier
// concatenation facility, so the macro accepts the generated function name
// explicitly while preserving the callback body and ordering.
#[macro_export]
macro_rules! __DECLARE_EVENT_CLASS {
    ($function:ident, $call:ident, ($($proto:tt)*), ($($args:tt)*), ($tstruct:block), ($assign:block), ($print:tt)*) => {
        #[allow(non_snake_case)]
        unsafe fn $function(__data: *mut core::ffi::c_void, $($proto)*) {
            let event_call = __data as *mut trace_event_call;
            let mut __data_offsets = trace_event_data_offsets_$call { };
            let mut entry: *mut trace_event_raw_$call;
            let mut __regs: *mut pt_regs = core::ptr::null_mut();
            let mut __count: u64 = 1;
            let mut __task: *mut task_struct = core::ptr::null_mut();
            let mut head: *mut hlist_head;
            let mut __entry_size: i32;
            let mut __data_size: i32;
            let mut rctx: i32;

            __data_size = trace_event_get_offsets_$call(&mut __data_offsets, $($args)*);
            head = this_cpu_ptr((*event_call).perf_events);
            if !bpf_prog_array_valid(event_call)
                && !__task.is_null()
                && hlist_empty(head)
            {
                return;
            }

            __entry_size = ALIGN(
                __data_size + core::mem::size_of::<trace_event_raw_$call>() as i32
                    + core::mem::size_of::<u32>() as i32,
                core::mem::size_of::<u64>() as i32,
            );
            __entry_size -= core::mem::size_of::<u32>() as i32;

            entry = perf_trace_buf_alloc(__entry_size, &mut __regs, &mut rctx);
            if entry.is_null() {
                return;
            }

            perf_fetch_caller_regs(__regs);
            $tstruct
            $assign
            perf_trace_run_bpf_submit(
                entry,
                __entry_size,
                rctx,
                event_call,
                __count,
                __regs,
                head,
                __task,
            );
        }
    };
}

// Rust-side declaration form for the C DECLARE_EVENT_CLASS expansion. The
// `print` argument is retained because it is part of the source interface.
#[macro_export]
macro_rules! DECLARE_EVENT_CLASS {
    ($function:ident, $call:ident, ($($proto:tt)*), ($($args:tt)*), ($($tstruct:tt)*), ($($assign:tt)*), ($($print:tt)*)) => {
        $crate::__DECLARE_EVENT_CLASS!(
            $function, $call, ($($proto)*), ($($args)*), ({ $($tstruct)* }),
            ({ $($assign)* }), ($($print)*)
        );
    };
}

#[macro_export]
macro_rules! DECLARE_EVENT_SYSCALL_CLASS {
    ($function:ident, $call:ident, ($($proto:tt)*), ($($args:tt)*), ($($tstruct:tt)*), ($($assign:tt)*), ($($print:tt)*)) => {
        $crate::__DECLARE_EVENT_CLASS!(
            $function, $call, ($($proto)*), ($($args)*), ({ $($tstruct)* }),
            ({ $($assign)* }), ($($print)*)
        );
    };
}

// This section is intentionally a build-time callback type check in the C
// source. The referenced type-checking symbols are supplied externally.
#[macro_export]
macro_rules! DEFINE_EVENT {
    ($template:ident, $name:ident, ($($proto:tt)*), ($($args:tt)*)) => {
        #[inline]
        fn $name() {
            check_trace_callback_type_$name(perf_trace_$template);
        }
    };
}

#[macro_export]
macro_rules! DEFINE_EVENT_PRINT {
    ($template:ident, $name:ident, ($($proto:tt)*), ($($args:tt)*), ($($print:tt)*)) => {
        $crate::DEFINE_EVENT!($template, $name, ($($proto)*), ($($args)*));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
