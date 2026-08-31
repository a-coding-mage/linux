// SPDX-License-Identifier: GPL-2.0
// C includes removed for Rust translation:
// <linux/ftrace.h>, <linux/tracepoint.h>, <linux/kernel.h>,
// <linux/module.h>, <linux/init.h>, <linux/rv.h>,
// <rv/instrumentation.h>, <rv_trace.h>,
// <monitors/ltl_parent/ltl_parent.h>, "test_ltl.h",
// <rv/ltl_monitor.h>

const MODULE_NAME: *const ::core::ffi::c_char = b"test_ltl\0".as_ptr() as *const ::core::ffi::c_char;

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */

#[repr(C)]
pub struct task_struct {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ltl_monitor {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static rv_ltl_parent: rv_monitor;

    fn ltl_atom_set(mon: *mut ltl_monitor, atom: ::core::ffi::c_int, value: bool);
    fn ltl_atom_update(task: *mut task_struct, atom: ::core::ffi::c_int, value: bool);
    fn ltl_monitor_init() -> ::core::ffi::c_int;
    fn ltl_monitor_destroy();
    fn rv_attach_trace_probe(
        name: *const ::core::ffi::c_char,
        tracepoint: *const ::core::ffi::c_void,
        probe: unsafe extern "C" fn(*mut ::core::ffi::c_void),
    );
    fn rv_detach_trace_probe(
        name: *const ::core::ffi::c_char,
        tracepoint: *const ::core::ffi::c_void,
        probe: unsafe extern "C" fn(*mut ::core::ffi::c_void),
    );
    fn rv_register_monitor(mon: *mut rv_monitor, parent: *const rv_monitor) -> ::core::ffi::c_int;
    fn rv_unregister_monitor(mon: *mut rv_monitor);
}

unsafe extern "C" {
    static LTL_EVENT_A: ::core::ffi::c_int;
    static LTL_EVENT_B: ::core::ffi::c_int;
}

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
unsafe extern "C" fn ltl_atoms_fetch(task: *mut task_struct, mon: *mut ltl_monitor) {
    /*
     * This is called everytime the Buchi automaton is triggered.
     *
     * This function could be used to fetch the atomic propositions which
     * are expensive to trace. It is possible only if the atomic proposition
     * does not need to be updated at precise time.
     *
     * It is recommended to use tracepoints and ltl_atom_update() instead.
     */
    let _ = task;
    let _ = mon;
}

unsafe extern "C" fn ltl_atoms_init(
    task: *mut task_struct,
    mon: *mut ltl_monitor,
    task_creation: bool,
) {
    /*
     * This should initialize as many atomic propositions as possible.
     *
     * @task_creation indicates whether the task is being created. This is
     * false if the task is already running before the monitor is enabled.
     */
    let _ = task;
    let _ = task_creation;

    // Original C placeholder expression: true/false.
    ltl_atom_set(mon, LTL_EVENT_A, true /* / false */);
    ltl_atom_set(mon, LTL_EVENT_B, true /* / false */);
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 */
unsafe extern "C" fn handle_example_event(data: *mut ::core::ffi::c_void) {
    // XXX: original C signature contains an unfinished header placeholder.
    let _ = data;

    // Original C references `task` from the unfinished tracepoint header and
    // uses the placeholder expression true/false.
    // ltl_atom_update(task, LTL_EVENT_A, true/false);
}

unsafe extern "C" fn enable_test_ltl() -> ::core::ffi::c_int {
    let retval: ::core::ffi::c_int;

    retval = ltl_monitor_init();
    if retval != 0 {
        return retval;
    }

    // XXX: original C has an unfinished tracepoint placeholder.
    rv_attach_trace_probe(
        b"test_ltl\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null(),
        handle_example_event,
    );

    0
}

unsafe extern "C" fn disable_test_ltl() {
    // XXX: original C has an unfinished tracepoint placeholder.
    rv_detach_trace_probe(
        b"test_ltl\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null(),
        handle_example_event,
    );

    ltl_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: b"test_ltl\0".as_ptr() as *const ::core::ffi::c_char,
    description: b"Simple description\0".as_ptr() as *const ::core::ffi::c_char,
    enable: Some(enable_test_ltl),
    disable: Some(disable_test_ltl),
};

unsafe extern "C" fn register_test_ltl() -> ::core::ffi::c_int {
    rv_register_monitor(&raw mut rv_this, &raw const rv_ltl_parent)
}

unsafe extern "C" fn unregister_test_ltl() {
    rv_unregister_monitor(&raw mut rv_this);
}

// module_init(register_test_ltl);
// module_exit(unregister_test_ltl);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("test_ltl: Simple description");
