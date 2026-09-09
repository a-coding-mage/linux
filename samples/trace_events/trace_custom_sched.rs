// SPDX-License-Identifier: GPL-2.0
/*
 * event tracer
 *
 * Copyright (C) 2022 Google Inc, Steven Rostedt <rostedt@goodmis.org>
 */

// pr_fmt(fmt) fmt
// C dependencies: <linux/trace_events.h>, <linux/module.h>, <linux/sched.h>,
// <trace/events/sched.h>, and "trace_custom_sched.h".
// CREATE_CUSTOM_TRACE_EVENTS is defined before including the custom header.

use core::ffi::c_void;

#[repr(C)]
pub struct tracepoint {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn trace_custom_event_sched_switch_update(tp: *mut tracepoint);
    fn trace_custom_event_sched_waking_update(tp: *mut tracepoint);

    // C macro: for_each_kernel_tracepoint(fct, NULL)
    fn for_each_kernel_tracepoint(
        callback: unsafe extern "C" fn(*mut tracepoint, *mut c_void),
        priv_: *mut c_void,
    );
}

/*
 * As the trace events are not exported to modules, the use of
 * for_each_kernel_tracepoint() is needed to find the trace event
 * to attach to. The fct() function below, is a callback that
 * will be called for every event.
 *
 * Helper functions are created by the TRACE_CUSTOM_EVENT() macro
 * update the event. Those are of the form:
 *
 *    trace_custom_event_<event>_update()
 *
 * Where <event> is the event to attach.
 */
unsafe extern "C" fn fct(tp: *mut tracepoint, _priv: *mut c_void) {
    trace_custom_event_sched_switch_update(tp);
    trace_custom_event_sched_waking_update(tp);
}

unsafe extern "C" fn trace_sched_init() -> i32 {
    for_each_kernel_tracepoint(fct, core::ptr::null_mut());
    0
}

unsafe extern "C" fn trace_sched_exit() {}

// module_init(trace_sched_init);
// module_exit(trace_sched_exit);
// MODULE_AUTHOR("Steven Rostedt");
// MODULE_DESCRIPTION("Custom scheduling events");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
