// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// #include <linux/ftrace.h>
// #include <linux/tracepoint.h>
// #include <linux/kernel.h>
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/rv.h>
// #include <rv/instrumentation.h>

use core::ffi::{c_char, c_int, c_void};

const MODULE_NAME: *const c_char = b"da_global\0".as_ptr() as *const c_char;

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */
// C dependency: #include <rv_trace.h>

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
const RV_MON_TYPE: c_int = RV_MON_GLOBAL;
// C dependencies:
// #include "da_global.h"
// #include <rv/da_monitor.h>

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
    static event_1_da_global: c_int;
    static event_2_da_global: c_int;

    static RV_MON_GLOBAL: c_int;

    fn da_handle_event(event: c_int);
    fn da_handle_start_event(event: c_int);
    fn da_monitor_init() -> c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(
        monitor: *const c_char,
        tracepoint: *mut c_void,
        probe: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_detach_trace_probe(
        monitor: *const c_char,
        tracepoint: *mut c_void,
        probe: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_register_monitor(monitor: *mut rv_monitor, data: *mut c_void) -> c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */
unsafe extern "C" fn handle_event_1(data: *mut c_void /* XXX: fill header */) {
    let _ = data;
    unsafe {
        da_handle_event(event_1_da_global);
    }
}

unsafe extern "C" fn handle_event_2(data: *mut c_void /* XXX: fill header */) {
    let _ = data;
    /* XXX: validate that this event always leads to the initial state */
    unsafe {
        da_handle_start_event(event_2_da_global);
    }
}

unsafe extern "C" fn enable_da_global() -> c_int {
    let retval: c_int;

    unsafe {
        retval = da_monitor_init();
    }
    if retval != 0 {
        return retval;
    }

    unsafe {
        rv_attach_trace_probe(
            MODULE_NAME,
            todo!("XXX: tracepoint"),
            handle_event_1,
        );
        rv_attach_trace_probe(
            MODULE_NAME,
            todo!("XXX: tracepoint"),
            handle_event_2,
        );
    }

    0
}

unsafe extern "C" fn disable_da_global() {
    unsafe {
        rv_this.enabled = 0;

        rv_detach_trace_probe(
            MODULE_NAME,
            todo!("XXX: tracepoint"),
            handle_event_1,
        );
        rv_detach_trace_probe(
            MODULE_NAME,
            todo!("XXX: tracepoint"),
            handle_event_2,
        );

        da_monitor_destroy();
    }
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: MODULE_NAME,
    description: b"auto-generated\0".as_ptr() as *const c_char,
    enable: Some(enable_da_global),
    disable: Some(disable_da_global),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_da_global() -> c_int {
    unsafe { rv_register_monitor(&raw mut rv_this, core::ptr::null_mut()) }
}

unsafe extern "C" fn unregister_da_global() {
    unsafe {
        rv_unregister_monitor(&raw mut rv_this);
    }
}

// module_init(register_da_global);
// module_exit(unregister_da_global);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("da_global: auto-generated");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
