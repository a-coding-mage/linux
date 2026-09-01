// SPDX-License-Identifier: GPL-2.0
//
// Original C dependencies:
// #include <linux/ftrace.h>
// #include <linux/tracepoint.h>
// #include <linux/kernel.h>
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/rv.h>
// #include <rv/instrumentation.h>

pub const MODULE_NAME: *const ::core::ffi::c_char = b"%%MODEL_NAME%%\0".as_ptr() as *const ::core::ffi::c_char;

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */
// Original C dependency: #include <rv_trace.h>
// %%INCLUDE_PARENT%%

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
// Original C macro: #define RV_MON_TYPE RV_MON_%%MONITOR_TYPE%%
// Original C dependency: #include "%%MODEL_NAME%%.h"
// Original C dependency: #include <rv/%%MONITOR_CLASS%%_monitor.h>

#[repr(C)]
pub struct rv_monitor {
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: ::core::ffi::c_int,
}

unsafe extern "C" {
    fn da_monitor_reset_all();
    fn rv_register_monitor(
        monitor: *mut rv_monitor,
        parent: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
    fn %%MONITOR_CLASS%%_monitor_init() -> ::core::ffi::c_int;
    fn %%MONITOR_CLASS%%_monitor_destroy();
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */
// %%TRACEPOINT_HANDLERS_SKEL%%

unsafe extern "C" fn enable_%%MODEL_NAME%%() -> ::core::ffi::c_int {
    let retval: ::core::ffi::c_int;

    retval = %%MONITOR_CLASS%%_monitor_init();
    if retval != 0 {
        return retval;
    }

    // %%TRACEPOINT_ATTACH%%

    0
}

unsafe extern "C" fn disable_%%MODEL_NAME%%() {
    rv_this.enabled = 0;

    // %%TRACEPOINT_DETACH%%

    %%MONITOR_CLASS%%_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
#[unsafe(no_mangle)]
pub static mut rv_this: rv_monitor = rv_monitor {
    name: b"%%MODEL_NAME%%\0".as_ptr() as *const ::core::ffi::c_char,
    description: b"%%DESCRIPTION%%\0".as_ptr() as *const ::core::ffi::c_char,
    enable: Some(enable_%%MODEL_NAME%%),
    disable: Some(disable_%%MODEL_NAME%%),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_%%MODEL_NAME%%() -> ::core::ffi::c_int {
    rv_register_monitor(&raw mut rv_this, %%PARENT%%)
}

unsafe extern "C" fn unregister_%%MODEL_NAME%%() {
    rv_unregister_monitor(&raw mut rv_this);
}

// Original C module initialization:
// module_init(register_%%MODEL_NAME%%);
// module_exit(unregister_%%MODEL_NAME%%);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("%%MODEL_NAME%%: %%DESCRIPTION%%");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
