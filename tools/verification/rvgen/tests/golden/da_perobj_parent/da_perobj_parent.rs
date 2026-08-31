// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// linux/ftrace.h, linux/tracepoint.h, linux/kernel.h, linux/module.h,
// linux/init.h, linux/rv.h, rv/instrumentation.h, rv_trace.h,
// monitors/parent_mon/parent_mon.h, da_perobj_parent.h, rv/da_monitor.h.

use core::ffi::{c_char, c_int, c_void};

const MODULE_NAME: &[u8] = b"da_perobj_parent\0";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
const RV_MON_TYPE: c_int = RV_MON_PER_OBJ;

/* XXX: define the target type */
type monitor_target = *mut c_void;

type TraceProbe = unsafe extern "C" fn(data: *mut c_void);

#[repr(C)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub enable: Option<unsafe extern "C" fn() -> c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: c_int,
}

extern "C" {
    static rv_parent_mon: rv_monitor;

    static event_1_da_perobj_parent: c_int;
    static event_2_da_perobj_parent: c_int;
    static event_3_da_perobj_parent: c_int;

    static RV_MON_PER_OBJ: c_int;

    fn da_handle_start_run_event(id: c_int, t: monitor_target, event: c_int);
    fn da_handle_event(id: c_int, t: monitor_target, event: c_int);
    fn da_destroy_storage(id: c_int);

    fn da_monitor_init() -> c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(name: *const c_char, tracepoint: *const c_void, probe: TraceProbe);
    fn rv_detach_trace_probe(name: *const c_char, tracepoint: *const c_void, probe: TraceProbe);
    fn rv_register_monitor(monitor: *mut rv_monitor, parent: *const rv_monitor) -> c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */
unsafe extern "C" fn handle_event_1(data: *mut c_void) {
    /* XXX: fill header */
    /* XXX: validate that this event is only valid in the initial state */
    /* XXX: how do I get the id? */
    let id: c_int = 0;
    /* XXX: how do I get t? */
    let t: monitor_target = core::ptr::null_mut();
    da_handle_start_run_event(id, t, event_1_da_perobj_parent);
}

unsafe extern "C" fn handle_event_2(data: *mut c_void) {
    /* XXX: fill header */
    /* XXX: how do I get the id? */
    let id: c_int = 0;
    /* XXX: how do I get t? */
    let t: monitor_target = core::ptr::null_mut();
    da_handle_event(id, t, event_2_da_perobj_parent);
}

unsafe extern "C" fn handle_event_3(data: *mut c_void) {
    /* XXX: fill header */
    /* XXX: how do I get the id? */
    let id: c_int = 0;
    /* XXX: how do I get t? */
    let t: monitor_target = core::ptr::null_mut();
    da_handle_event(id, t, event_3_da_perobj_parent);
}

/* XXX: obj is being destroyed, remove if not required (e.g. obj is static) */
unsafe extern "C" fn handle_obj_cleanup(data: *mut c_void) {
    /* XXX: fill header */
    /* XXX: how do I get the id? */
    let id: c_int = 0;
    da_destroy_storage(id);
}

unsafe extern "C" fn enable_da_perobj_parent() -> c_int {
    let retval: c_int;

    retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event_1,
    ); /* XXX: tracepoint */
    rv_attach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event_2,
    ); /* XXX: tracepoint */
    rv_attach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event_3,
    ); /* XXX: tracepoint */
    rv_attach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_obj_cleanup,
    ); /* XXX: cleanup tracepoint */

    0
}

unsafe extern "C" fn disable_da_perobj_parent() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event_1,
    ); /* XXX: tracepoint */
    rv_detach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event_2,
    ); /* XXX: tracepoint */
    rv_detach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event_3,
    ); /* XXX: tracepoint */
    rv_detach_trace_probe(
        MODULE_NAME.as_ptr() as *const c_char,
        core::ptr::null(),
        handle_obj_cleanup,
    ); /* XXX: cleanup tracepoint */

    da_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: b"da_perobj_parent\0".as_ptr() as *const c_char,
    description: b"auto-generated\0".as_ptr() as *const c_char,
    enable: Some(enable_da_perobj_parent),
    disable: Some(disable_da_perobj_parent),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_da_perobj_parent() -> c_int {
    rv_register_monitor(&mut rv_this, &rv_parent_mon)
}

unsafe extern "C" fn unregister_da_perobj_parent() {
    rv_unregister_monitor(&mut rv_this);
}

/* module_init(register_da_perobj_parent); */
/* module_exit(unregister_da_perobj_parent); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_AUTHOR("rvgen: auto-generated"); */
/* MODULE_DESCRIPTION("da_perobj_parent: auto-generated"); */
