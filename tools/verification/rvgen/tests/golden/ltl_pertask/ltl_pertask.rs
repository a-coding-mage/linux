// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/ftrace.h, linux/tracepoint.h, linux/kernel.h,
// linux/module.h, linux/init.h, linux/rv.h, rv/instrumentation.h.

pub const MODULE_NAME: &[u8] = b"ltl_pertask\0";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */
// C dependency: rv_trace.h.

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
// C dependencies: "ltl_pertask.h", rv/ltl_monitor.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ltl_monitor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub enable: Option<unsafe extern "C" fn() -> c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static LTL_EVENT_A: c_int;
    static LTL_EVENT_B: c_int;

    fn ltl_atom_set(mon: *mut ltl_monitor, atom: c_int, value: bool);
    fn ltl_atom_update(task: *mut task_struct, atom: c_int, value: bool);
    fn ltl_monitor_init() -> c_int;
    fn ltl_monitor_destroy();
    fn rv_attach_trace_probe(
        name: *const c_char,
        tracepoint: *const c_void,
        handler: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_detach_trace_probe(
        name: *const c_char,
        tracepoint: *const c_void,
        handler: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_register_monitor(mon: *mut rv_monitor, data: *mut c_void) -> c_int;
    fn rv_unregister_monitor(mon: *mut rv_monitor);
}

unsafe extern "C" fn ltl_atoms_fetch(_task: *mut task_struct, _mon: *mut ltl_monitor) {
    /*
     * This is called everytime the Buchi automaton is triggered.
     *
     * This function could be used to fetch the atomic propositions which
     * are expensive to trace. It is possible only if the atomic proposition
     * does not need to be updated at precise time.
     *
     * It is recommended to use tracepoints and ltl_atom_update() instead.
     */
}

unsafe extern "C" fn ltl_atoms_init(
    _task: *mut task_struct,
    mon: *mut ltl_monitor,
    _task_creation: bool,
) {
    /*
     * This should initialize as many atomic propositions as possible.
     *
     * @task_creation indicates whether the task is being created. This is
     * false if the task is already running before the monitor is enabled.
     */
    unsafe {
        // Original template expression: true/false.
        ltl_atom_set(mon, LTL_EVENT_A, true);
        // Original template expression: true/false.
        ltl_atom_set(mon, LTL_EVENT_B, true);
    }
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 */
unsafe extern "C" fn handle_example_event(data: *mut c_void) {
    // Original C parameter list has an unresolved placeholder:
    // void *data, /* XXX: fill header */
    let task = data as *mut task_struct;

    unsafe {
        // Original template expression: true/false.
        ltl_atom_update(task, LTL_EVENT_A, true);
    }
}

unsafe extern "C" fn enable_ltl_pertask() -> c_int {
    let retval: c_int;

    unsafe {
        retval = ltl_monitor_init();
    }
    if retval != 0 {
        return retval;
    }

    unsafe {
        rv_attach_trace_probe(
            c"ltl_pertask".as_ptr(),
            core::ptr::null(), // XXX: tracepoint
            handle_example_event,
        );
    }

    0
}

unsafe extern "C" fn disable_ltl_pertask() {
    unsafe {
        rv_detach_trace_probe(
            c"ltl_pertask".as_ptr(),
            core::ptr::null(), // XXX: tracepoint
            handle_example_event,
        );

        ltl_monitor_destroy();
    }
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: c"ltl_pertask".as_ptr(),
    description: c"auto-generated".as_ptr(),
    enable: Some(enable_ltl_pertask),
    disable: Some(disable_ltl_pertask),
};

unsafe extern "C" fn register_ltl_pertask() -> c_int {
    unsafe { rv_register_monitor(&raw mut rv_this, core::ptr::null_mut()) }
}

unsafe extern "C" fn unregister_ltl_pertask() {
    unsafe {
        rv_unregister_monitor(&raw mut rv_this);
    }
}

// module_init(register_ltl_pertask);
// module_exit(unregister_ltl_pertask);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("ltl_pertask: auto-generated");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
