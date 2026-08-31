// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// linux/ftrace.h, linux/tracepoint.h, linux/kernel.h, linux/module.h,
// linux/init.h, linux/rv.h, rv/instrumentation.h

use core::ffi::{c_char, c_int, c_void};

const MODULE_NAME: &[u8] = b"test_da\0";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 *
 * C dependency removed from executable Rust: rv_trace.h
 */

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
const RV_MON_TYPE: c_int = RV_MON_PER_CPU;

// C dependencies removed from executable Rust: test_da.h, rv/da_monitor.h

type TraceProbeCallback = unsafe extern "C" fn(*mut c_void);

#[repr(C)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub enable: Option<unsafe extern "C" fn() -> c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: c_int,
}

unsafe extern "C" {
    static event_1_test_da: c_int;
    static event_2_test_da: c_int;

    static RV_MON_PER_CPU: c_int;

    fn da_handle_event(event: c_int);
    fn da_handle_start_event(event: c_int);
    fn da_monitor_init() -> c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(
        name: *const c_char,
        tracepoint: *mut c_void,
        probe: TraceProbeCallback,
    );
    fn rv_detach_trace_probe(
        name: *const c_char,
        tracepoint: *mut c_void,
        probe: TraceProbeCallback,
    );
    fn rv_register_monitor(monitor: *mut rv_monitor, parent: *mut c_void) -> c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */
unsafe extern "C" fn handle_event_1(_data: *mut c_void /* XXX: fill header */) {
    unsafe {
        da_handle_event(event_1_test_da);
    }
}

unsafe extern "C" fn handle_event_2(_data: *mut c_void /* XXX: fill header */) {
    /* XXX: validate that this event always leads to the initial state */
    unsafe {
        da_handle_start_event(event_2_test_da);
    }
}

unsafe extern "C" fn enable_test_da() -> c_int {
    let retval: c_int;

    unsafe {
        retval = da_monitor_init();
    }
    if retval != 0 {
        return retval;
    }

    unsafe {
        rv_attach_trace_probe(
            c"test_da".as_ptr(),
            core::ptr::null_mut(), /* XXX: tracepoint */
            handle_event_1,
        );
        rv_attach_trace_probe(
            c"test_da".as_ptr(),
            core::ptr::null_mut(), /* XXX: tracepoint */
            handle_event_2,
        );
    }

    0
}

unsafe extern "C" fn disable_test_da() {
    unsafe {
        rv_this.enabled = 0;

        rv_detach_trace_probe(
            c"test_da".as_ptr(),
            core::ptr::null_mut(), /* XXX: tracepoint */
            handle_event_1,
        );
        rv_detach_trace_probe(
            c"test_da".as_ptr(),
            core::ptr::null_mut(), /* XXX: tracepoint */
            handle_event_2,
        );

        da_monitor_destroy();
    }
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: c"test_da".as_ptr(),
    description: c"auto-generated".as_ptr(),
    enable: Some(enable_test_da),
    disable: Some(disable_test_da),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_test_da() -> c_int {
    unsafe { rv_register_monitor(&raw mut rv_this, core::ptr::null_mut()) }
}

unsafe extern "C" fn unregister_test_da() {
    unsafe {
        rv_unregister_monitor(&raw mut rv_this);
    }
}

// module_init(register_test_da);
// module_exit(unregister_test_da);
//
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("test_da: auto-generated");
