/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of trace_custom_events.h.
 * This header is a staged macro template for custom trace events.
 * The original C includes and build-time stage expansion are represented by
 * comments; symbols supplied by the kernel trace-event machinery remain
 * external dependencies.
 */

// All custom events are placed in the custom group.
pub const TRACE_SYSTEM: &str = "custom";

// The init stage creates the system string and enum mappings.
// C source: #include "stages/init.h"

#[macro_export]
macro_rules! TRACE_CUSTOM_EVENT {
    ($name:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        DECLARE_CUSTOM_EVENT_CLASS!($name, $proto, $args, $tstruct, $assign, $print);
        DEFINE_CUSTOM_EVENT!($name, $name, $proto, $args);
    };
}

// Stage 1 creates the structure of the recorded event layout.
#[macro_export]
macro_rules! DECLARE_CUSTOM_EVENT_CLASS {
    ($name:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        #[repr(C)]
        pub struct trace_custom_event_raw_$name {
            pub ent: trace_entry,
            pub __data: [u8; 0],
        }
        static mut custom_event_class_$name: trace_event_class = trace_event_class::default();
    };
}

#[macro_export]
macro_rules! DEFINE_CUSTOM_EVENT {
    ($template:ident, $name:ident, $proto:tt, $args:tt) => {
        static mut custom_event_$name: trace_event_call = trace_event_call::default();
    };
}

// C source: #include TRACE_INCLUDE(TRACE_INCLUDE_FILE)
// Stage 2 creates the custom class.
#[macro_export]
macro_rules! DECLARE_CUSTOM_EVENT_CLASS_STAGE2 {
    ($call:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        #[repr(C)]
        pub struct trace_custom_event_data_offsets_$call {
            pub fields: $tstruct,
        }
    };
}

// Stage 3 creates the way to print the custom event.
#[macro_export]
macro_rules! DECLARE_CUSTOM_EVENT_CLASS_STAGE3 {
    ($call:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        unsafe fn trace_custom_raw_output_$call(
            iter: *mut trace_iterator,
            flags: i32,
            trace_event: *mut trace_event,
        ) -> print_line_t {
            let s = &mut (*iter).seq;
            let _p = &mut (*iter).tmp_seq;
            let field = (*iter).ent as *mut trace_custom_event_raw_$call;
            let ret = trace_raw_output_prep(iter, trace_event);
            if ret != TRACE_TYPE_HANDLED {
                return ret;
            }
            trace_event_printf(iter, $print);
            let _ = field;
            trace_handle_return(s)
        }
        static mut trace_custom_event_type_funcs_$call: trace_event_functions =
            trace_event_functions { trace: Some(trace_custom_raw_output_$call) };
    };
}

// Stage 4 creates the offset layout for the fields.
#[macro_export]
macro_rules! DECLARE_CUSTOM_EVENT_CLASS_STAGE4 {
    ($call:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        static mut trace_custom_event_fields_$call: [trace_event_fields; 1] =
            [trace_event_fields::default()];
    };
}

// Stage 5 creates the helper function for dynamic fields.
#[macro_export]
macro_rules! DECLARE_CUSTOM_EVENT_CLASS_STAGE5 {
    ($call:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        unsafe fn trace_custom_event_get_offsets_$call(
            _data_offsets: *mut trace_custom_event_data_offsets_$call,
            $proto,
        ) -> i32 {
            let __data_size: i32 = 0;
            $tstruct
            __data_size
        }
    };
}

// Stage 6 creates the probe function that records the event.
#[macro_export]
macro_rules! DECLARE_CUSTOM_EVENT_CLASS_STAGE6 {
    ($call:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:tt) => {
        unsafe fn trace_custom_event_raw_event_$call(__data: *mut core::ffi::c_void, $proto) {
            let trace_file = __data as *mut trace_event_file;
            let mut __data_offsets = trace_custom_event_data_offsets_$call::default();
            let mut fbuffer = trace_event_buffer::default();
            if trace_trigger_soft_disabled(trace_file) { return; }
            let __data_size = trace_custom_event_get_offsets_$call(&mut __data_offsets, $args);
            let entry = trace_event_buffer_reserve(&mut fbuffer, trace_file, __data_size);
            if entry.is_null() { return; }
            $tstruct
            { $assign }
            trace_event_buffer_commit(&mut fbuffer);
        }
    };
}

// Build-time callback type check from the C header.
#[macro_export]
macro_rules! DEFINE_CUSTOM_EVENT_PROBE_CHECK {
    ($template:ident, $call:ident, $proto:tt, $args:tt) => {
        #[inline]
        fn ftrace_test_custom_probe_$call() {
            check_trace_callback_type_$call(trace_custom_event_raw_event_$template);
        }
    };
}

// Stage 7 creates the actual class and event structure for the custom event.
#[macro_export]
macro_rules! DECLARE_CUSTOM_EVENT_CLASS_STAGE7 {
    ($call:ident, $proto:tt, $args:tt, $tstruct:tt, $assign:tt, $print:expr) => {
        static mut custom_print_fmt_$call: &str = $print;
        static mut custom_event_class_$call: trace_event_class = trace_event_class {
            system: TRACE_SYSTEM_STRING,
            fields_array: trace_custom_event_fields_$call,
            fields: LIST_HEAD_INIT!(custom_event_class_$call.fields),
            raw_init: trace_event_raw_init,
            probe: trace_custom_event_raw_event_$call,
            reg: trace_event_reg,
        };
    };
}

#[macro_export]
macro_rules! DEFINE_CUSTOM_EVENT_STAGE7 {
    ($template:ident, $call:ident, $proto:tt, $args:tt) => {
        static mut custom_event_$call: trace_event_call = trace_event_call {
            name: stringify!($call),
            class: &custom_event_class_$template,
            event_funcs: &trace_custom_event_type_funcs_$template,
            print_fmt: custom_print_fmt_$template,
            flags: TRACE_EVENT_FL_CUSTOM,
        };
        unsafe fn trace_custom_event_$call##_update(tp: *mut tracepoint) -> i32 {
            if !(*tp).name.is_null() && strcmp((*tp).name, stringify!($call)) == 0 {
                (*(&mut custom_event_$call)).tp = tp;
                (*(&mut custom_event_$call)).flags = TRACE_EVENT_FL_TRACEPOINT;
                return 1;
            }
            0
        }
        #[used]
        static mut __custom_event_$call: *mut trace_event_call = &raw mut custom_event_$call;
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
