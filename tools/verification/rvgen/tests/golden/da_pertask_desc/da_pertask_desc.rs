// SPDX-License-Identifier: GPL-2.0
//
// Dependencies from the original C source:
// linux/ftrace.h, linux/tracepoint.h, linux/kernel.h, linux/module.h,
// linux/init.h, linux/rv.h, rv/instrumentation.h, rv_trace.h,
// da_pertask_desc.h, and rv/da_monitor.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

const MODULE_NAME: &[u8] = b"da_pertask_desc\0";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
// RV_MON_TYPE is RV_MON_PER_TASK for the generated da_pertask_desc monitor.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const core::ffi::c_char,
    pub description: *const core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: core::ffi::c_int,
}

unsafe extern "C" {
    static event_1_da_pertask_desc: core::ffi::c_int;
    static event_2_da_pertask_desc: core::ffi::c_int;
    static event_3_da_pertask_desc: core::ffi::c_int;

    fn da_handle_start_run_event(
        p: *mut task_struct,
        event: core::ffi::c_int,
    );
    fn da_handle_event(p: *mut task_struct, event: core::ffi::c_int);
    fn da_monitor_init() -> core::ffi::c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();
    fn rv_attach_trace_probe(
        name: *const core::ffi::c_char,
        tracepoint: *const core::ffi::c_void,
        probe: *const core::ffi::c_void,
    );
    fn rv_detach_trace_probe(
        name: *const core::ffi::c_char,
        tracepoint: *const core::ffi::c_void,
        probe: *const core::ffi::c_void,
    );
    fn rv_register_monitor(
        monitor: *mut rv_monitor,
        parent: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */
unsafe extern "C" fn handle_event_1(data: *mut core::ffi::c_void) {
    let _ = data;

    /* XXX: validate that this event is only valid in the initial state */
    /* XXX: how do I get p? */
    let p: *mut task_struct = core::ptr::null_mut();
    da_handle_start_run_event(p, event_1_da_pertask_desc);
}

unsafe extern "C" fn handle_event_2(data: *mut core::ffi::c_void) {
    let _ = data;

    /* XXX: how do I get p? */
    let p: *mut task_struct = core::ptr::null_mut();
    da_handle_event(p, event_2_da_pertask_desc);
}

unsafe extern "C" fn handle_event_3(data: *mut core::ffi::c_void) {
    let _ = data;

    /* XXX: how do I get p? */
    let p: *mut task_struct = core::ptr::null_mut();
    da_handle_event(p, event_3_da_pertask_desc);
}

unsafe extern "C" fn enable_da_pertask_desc() -> core::ffi::c_int {
    let retval: core::ffi::c_int;

    retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        b"da_pertask_desc\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null(),
        handle_event_1 as *const core::ffi::c_void,
    );
    rv_attach_trace_probe(
        b"da_pertask_desc\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null(),
        handle_event_2 as *const core::ffi::c_void,
    );
    rv_attach_trace_probe(
        b"da_pertask_desc\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null(),
        handle_event_3 as *const core::ffi::c_void,
    );

    0
}

unsafe extern "C" fn disable_da_pertask_desc() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        b"da_pertask_desc\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null(),
        handle_event_1 as *const core::ffi::c_void,
    );
    rv_detach_trace_probe(
        b"da_pertask_desc\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null(),
        handle_event_2 as *const core::ffi::c_void,
    );
    rv_detach_trace_probe(
        b"da_pertask_desc\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null(),
        handle_event_3 as *const core::ffi::c_void,
    );

    da_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: b"da_pertask_desc\0".as_ptr() as *const core::ffi::c_char,
    description: b"Custom description for testing\0".as_ptr() as *const core::ffi::c_char,
    enable: Some(enable_da_pertask_desc),
    disable: Some(disable_da_pertask_desc),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_da_pertask_desc() -> core::ffi::c_int {
    rv_register_monitor(&mut rv_this, core::ptr::null_mut())
}

unsafe extern "C" fn unregister_da_pertask_desc() {
    rv_unregister_monitor(&mut rv_this);
}

// module_init(register_da_pertask_desc);
// module_exit(unregister_da_pertask_desc);
//
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("da_pertask_desc: Custom description for testing");
