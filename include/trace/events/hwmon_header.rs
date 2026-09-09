/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the tracepoint header.  The C preprocessor guard and
// TRACE_HEADER_MULTI_READ condition are intentionally represented as a
// comment; their meaning is supplied by the tracepoint build machinery.

pub const TRACE_SYSTEM: &[u8] = b"hwmon\0";

use core::ffi::{c_char, c_int, c_longlong};

#[repr(C)]
pub struct HwmonAttrClassEntry {
    pub index: c_int,
    pub attr_name: *const c_char,
    pub val: c_longlong,
}

#[repr(C)]
pub struct HwmonAttrShowStringEntry {
    pub index: c_int,
    pub attr_name: *const c_char,
    pub label: *const c_char,
}

// DECLARE_EVENT_CLASS(hwmon_attr_class)
// TP_PROTO(int index, const char *attr_name, long long val)
// TP_ARGS(index, attr_name, val)
// TP_fast_assign:
//     __entry->index = index;
//     __assign_str(attr_name);
//     __entry->val = val;
// TP_printk("index=%d, attr_name=%s, val=%lld",
//           __entry->index, __get_str(attr_name), __entry->val)

// DEFINE_EVENT(hwmon_attr_class, hwmon_attr_show)
// DEFINE_EVENT(hwmon_attr_class, hwmon_attr_store)
// The tracepoint implementations and registration are provided by the
// external Linux tracepoint dependency.
extern "C" {
    pub fn hwmon_attr_show(index: c_int, attr_name: *const c_char, val: c_longlong);
    pub fn hwmon_attr_store(index: c_int, attr_name: *const c_char, val: c_longlong);
}

// TRACE_EVENT(hwmon_attr_show_string)
// TP_PROTO(int index, const char *attr_name, const char *s)
// TP_ARGS(index, attr_name, s)
// TP_fast_assign:
//     __entry->index = index;
//     __assign_str(attr_name);
//     __assign_str(label);
// TP_printk("index=%d, attr_name=%s, val=%s",
//           __entry->index, __get_str(attr_name), __get_str(label))
extern "C" {
    pub fn hwmon_attr_show_string(
        index: c_int,
        attr_name: *const c_char,
        s: *const c_char,
    );
}

// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
