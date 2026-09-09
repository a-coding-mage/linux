/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust source-level translation of trace_events.h.
 * Kernel includes and TRACE_INCLUDE expansions are external dependencies and
 * are intentionally retained as comments rather than implemented here.
 */

// #include <linux/trace_events.h>
// #include <linux/btf_ids.h>
// #include "stages/init.h"

#[allow(unused_macros)]
macro_rules! trace_event {
    ($name:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        declare_event_class!($name, $proto, $args, $tstruct, $assign, $print);
        define_event!($name, $name, $proto, $args);
    };
}

#[allow(unused_macros)]
macro_rules! trace_event_syscall {
    ($name:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt, $reg:tt, $unreg:tt) => {
        declare_event_syscall_class!($name, $proto, $args, $tstruct, $assign, $print);
        define_event!($name, $name, $proto, $args);
    };
}

/* Stage 1: raw event structures. */
// TRACE_INCLUDE(TRACE_INCLUDE_FILE) is supplied by the including tracepoint.

#[allow(unused_macros)]
macro_rules! declare_event_class {
    ($name:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        #[repr(C)]
        pub struct trace_event_raw_$name {
            pub ent: trace_entry,
            pub __data: [core::ffi::c_char; 0],
        }
        static mut event_class_$name: trace_event_class = trace_event_class::default();
    };
}

#[allow(unused_macros)]
macro_rules! declare_event_syscall_class { ($($tokens:tt)*) => { declare_event_class!($($tokens)*); }; }
#[allow(unused_macros)]
macro_rules! define_event { ($template:ident, $name:ident, $proto:tt, $args:tt) => {
    #[repr(C, align(4))]
    static mut event_$name: trace_event_call = trace_event_call::default();
}; }
#[allow(unused_macros)]
macro_rules! define_event_fn { ($template:ident, $name:ident, $proto:tt, $args:tt, $reg:tt, $unreg:tt) => { define_event!($template, $name, $proto, $args); }; }
#[allow(unused_macros)]
macro_rules! define_event_print { ($template:ident, $name:ident, $proto:tt, $args:tt, $print:tt) => { define_event!($template, $name, $proto, $args); }; }

/* Callbacks are meaningless to ftrace; conditional flags and permissions are
 * delegated to the external trace-event implementation. */
#[allow(unused_macros)]
macro_rules! trace_event_fn { ($($tokens:tt)*) => { trace_event!($($tokens)*); }; }
#[allow(unused_macros)]
macro_rules! trace_event_fn_cond { ($($tokens:tt)*) => { trace_event_condition!($($tokens)*); }; }
#[allow(unused_macros)]
macro_rules! trace_event_flags { ($name:ident, $value:expr) => { __trace_event_flags!($name, $value); }; }
#[allow(unused_macros)]
macro_rules! trace_event_perf_perm { ($name:ident, $($expr:tt)*) => { __trace_event_perf_perm!($name, $($expr)*); }; }

// TRACE_INCLUDE(TRACE_INCLUDE_FILE)

/* Stage 2: dynamic-data offsets. */
#[allow(unused_macros)]
macro_rules! declare_event_data_offsets {
    ($call:ident, $tstruct:tt) => {
        #[repr(C)]
        pub struct trace_event_data_offsets_$call { $tstruct }
    };
}
// DEFINE_EVENT, DEFINE_EVENT_PRINT, TRACE_EVENT_FLAGS and TRACE_EVENT_PERF_PERM
// expand to no items during this stage.

// TRACE_INCLUDE(TRACE_INCLUDE_FILE)

/* Stage 3: trace output callbacks. */
#[allow(unused_macros)]
macro_rules! declare_trace_output {
    ($call:ident, $proto:tt, $print:tt) => {
        unsafe fn trace_raw_output_$call(
            iter: *mut trace_iterator, flags: i32, trace_event: *mut trace_event,
        ) -> print_line_t {
            let s = &mut (*iter).seq;
            let _p = &mut (*iter).tmp_seq;
            let _field = (*iter).ent as *mut trace_event_raw_$call;
            let ret = trace_raw_output_prep(iter, trace_event);
            if ret != TRACE_TYPE_HANDLED { return ret; }
            trace_event_printf(iter, $print);
            trace_handle_return(s)
        }
    };
}

// TRACE_INCLUDE(TRACE_INCLUDE_FILE)

/* Stage 4: event fields. */
#[allow(unused_macros)]
macro_rules! declare_event_fields {
    ($call:ident, $tstruct:tt) => {
        static mut trace_event_fields_$call: [trace_event_fields; 1] = [trace_event_fields::default()];
    };
}
// TRACE_INCLUDE(TRACE_INCLUDE_FILE)

/* Stage 5: offset calculation. */
#[allow(unused_macros)]
macro_rules! declare_event_get_offsets {
    ($call:ident, $proto:tt, $tstruct:tt) => {
        unsafe fn trace_event_get_offsets_$call(
            __data_offsets: *mut trace_event_data_offsets_$call, $proto
        ) -> i32 {
            let mut __data_size: i32 = 0;
            let mut __item_length: i32 = 0;
            let _entry: *mut trace_event_raw_$call = core::ptr::null_mut();
            $tstruct
            __data_size
        }
    };
}
// TRACE_INCLUDE(TRACE_INCLUDE_FILE)

/* Stage 6 callback and buffering logic. */
#[cfg(feature = "CONFIG_PERF_EVENTS")]
macro_rules! trace_perf_proto { ($call:ident, $proto:tt) => { unsafe fn perf_trace_$call($data: *mut core::ffi::c_void, $proto); }; }
#[cfg(feature = "CONFIG_PERF_EVENTS")]
macro_rules! trace_perf_init { ($call:ident) => { perf_probe: perf_trace_$call, }; }
#[cfg(not(feature = "CONFIG_PERF_EVENTS"))]
macro_rules! trace_perf_proto { ($call:ident, $proto:tt) => {}; }
#[cfg(not(feature = "CONFIG_PERF_EVENTS"))]
macro_rules! trace_perf_init { ($call:ident) => {}; }

#[cfg(all(feature = "CONFIG_BPF_EVENTS", feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! trace_btf_ids_declare { ($call:ident) => { /* BTF_ID_LIST(__bpf_trace_btf_ids_$call), FUNC and STRUCT entries. */ }; }
#[cfg(all(feature = "CONFIG_BPF_EVENTS", feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! trace_btf_ids_init { ($call:ident) => { btf_ids: __bpf_trace_btf_ids_$call, }; }
#[cfg(not(all(feature = "CONFIG_BPF_EVENTS", feature = "CONFIG_DEBUG_INFO_BTF")))]
macro_rules! trace_btf_ids_declare { ($call:ident) => {}; }
#[cfg(not(all(feature = "CONFIG_BPF_EVENTS", feature = "CONFIG_DEBUG_INFO_BTF")))]
macro_rules! trace_btf_ids_init { ($call:ident) => {}; }

// TRACE_INCLUDE(TRACE_INCLUDE_FILE)

/* Stage 7 class definitions and event registrations. */
#[allow(unused_macros)]
macro_rules! declare_final_event_class {
    ($call:ident, $proto:tt, $print:tt) => {
        static mut print_fmt_$call: [core::ffi::c_char; 0] = [];
        static mut event_class_$call: trace_event_class = trace_event_class::default();
    };
}
#[allow(unused_macros)]
macro_rules! define_final_event {
    ($template:ident, $call:ident, $proto:tt, $args:tt) => {
        static mut event_$call: trace_event_call = trace_event_call::default();
        static mut __event_$call: *mut trace_event_call = core::ptr::addr_of_mut!(event_$call);
    };
}
#[allow(unused_macros)]
macro_rules! define_final_event_print { ($template:ident, $call:ident, $proto:tt, $args:tt, $print:tt) => { define_final_event!($template, $call, $proto, $args); }; }

// The original file's final TRACE_INCLUDE(TRACE_INCLUDE_FILE) expansion is
// intentionally left to the including tracepoint header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
