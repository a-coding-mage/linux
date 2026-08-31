// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/ftrace.h, linux/tracepoint.h, linux/kernel.h,
// linux/module.h, linux/init.h, linux/rv.h, rv/instrumentation.h

const MODULE_NAME: &str = "%%MODEL_NAME%%";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */
// C dependencies: rv_trace.h
%%INCLUDE_PARENT%%

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
// C dependencies: "%%MODEL_NAME%%.h", rv/ltl_monitor.h

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
    pub name: *const core::ffi::c_char,
    pub description: *const core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
}

extern "C" {
    fn ltl_monitor_init() -> core::ffi::c_int;
    fn ltl_monitor_destroy();
    fn rv_register_monitor(
        monitor: *mut rv_monitor,
        parent: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

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
    let _ = mon;
    let _ = task_creation;
%%ATOMS_INIT%%
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 */
%%TRACEPOINT_HANDLERS_SKEL%%
unsafe extern "C" fn enable_%%MODEL_NAME%%() -> core::ffi::c_int {
    let retval: core::ffi::c_int;

    retval = ltl_monitor_init();
    if retval != 0 {
        return retval;
    }

%%TRACEPOINT_ATTACH%%

    return 0;
}

unsafe extern "C" fn disable_%%MODEL_NAME%%() {
%%TRACEPOINT_DETACH%%

    ltl_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: b"%%MODEL_NAME%%\0".as_ptr() as *const core::ffi::c_char,
    description: b"%%DESCRIPTION%%\0".as_ptr() as *const core::ffi::c_char,
    enable: Some(enable_%%MODEL_NAME%%),
    disable: Some(disable_%%MODEL_NAME%%),
};

unsafe extern "C" fn register_%%MODEL_NAME%%() -> core::ffi::c_int {
    return rv_register_monitor(&mut rv_this, %%PARENT%%);
}

unsafe extern "C" fn unregister_%%MODEL_NAME%%() {
    rv_unregister_monitor(&mut rv_this);
}

// C module wiring:
// module_init(register_%%MODEL_NAME%%);
// module_exit(unregister_%%MODEL_NAME%%);
//
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("%%MODEL_NAME%%: %%DESCRIPTION%%");
