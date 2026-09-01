// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// linux/ftrace.h, linux/tracepoint.h, linux/kernel.h, linux/module.h,
// linux/init.h, linux/rv.h, rv/instrumentation.h, rv_trace.h,
// test_da_kunit.h, rv/da_monitor.h.

use core::ffi::{c_char, c_int, c_void};

const MODULE_NAME: &[u8] = b"test_da_kunit\0";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
// #define RV_MON_TYPE RV_MON_PER_CPU

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
    static event_1_test_da_kunit: c_int;
    static event_2_test_da_kunit: c_int;

    fn da_handle_event(event: c_int);
    fn da_handle_start_event(event: c_int);
    fn da_monitor_init() -> c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(
        monitor_name: *const c_char,
        tracepoint: *mut c_void,
        probe: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_detach_trace_probe(
        monitor_name: *const c_char,
        tracepoint: *mut c_void,
        probe: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_register_monitor(mon: *mut rv_monitor, data: *mut c_void) -> c_int;
    fn rv_unregister_monitor(mon: *mut rv_monitor);
}

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */
unsafe extern "C" fn handle_event_1(_data: *mut c_void) {
    // XXX: fill header
    unsafe {
        da_handle_event(event_1_test_da_kunit);
    }
}

unsafe extern "C" fn handle_event_2(_data: *mut c_void) {
    // XXX: fill header
    /* XXX: validate that this event always leads to the initial state */
    unsafe {
        da_handle_start_event(event_2_test_da_kunit);
    }
}

unsafe extern "C" fn enable_test_da_kunit() -> c_int {
    let retval: c_int;

    unsafe {
        retval = da_monitor_init();
    }
    if retval != 0 {
        return retval;
    }

    unsafe {
        rv_attach_trace_probe(
            MODULE_NAME.as_ptr().cast(),
            core::ptr::null_mut(), /* XXX: tracepoint */
            handle_event_1,
        );
        rv_attach_trace_probe(
            MODULE_NAME.as_ptr().cast(),
            core::ptr::null_mut(), /* XXX: tracepoint */
            handle_event_2,
        );
    }

    0
}

unsafe extern "C" fn disable_test_da_kunit() {
    unsafe {
        rv_this.enabled = 0;

        rv_detach_trace_probe(
            MODULE_NAME.as_ptr().cast(),
            core::ptr::null_mut(), /* XXX: tracepoint */
            handle_event_1,
        );
        rv_detach_trace_probe(
            MODULE_NAME.as_ptr().cast(),
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
    name: MODULE_NAME.as_ptr().cast(),
    description: b"auto-generated\0".as_ptr().cast(),
    enable: Some(enable_test_da_kunit),
    disable: Some(disable_test_da_kunit),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_test_da_kunit() -> c_int {
    unsafe { rv_register_monitor(&raw mut rv_this, core::ptr::null_mut()) }
}

unsafe extern "C" fn unregister_test_da_kunit() {
    unsafe {
        rv_unregister_monitor(&raw mut rv_this);
    }
}

// module_init(register_test_da_kunit);
// module_exit(unregister_test_da_kunit);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("test_da_kunit: auto-generated");

// Original condition: IS_ENABLED(CONFIG_RV_MONITORS_KUNIT_TEST)
#[cfg(CONFIG_RV_MONITORS_KUNIT_TEST)]
mod kunit_exports {
    use super::*;

    // C dependencies removed from executable Rust:
    // kunit/visibility.h, test_da_kunit_kunit.h.

    #[repr(C)]
    pub struct rv_monitor_ops {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct rv_test_da_kunit_ops {
        pub mon: rv_monitor_ops,
        pub handle_event_1: Option<unsafe extern "C" fn(*mut c_void)>,
        pub handle_event_2: Option<unsafe extern "C" fn(*mut c_void)>,
    }

    // RV_MON_OPS_INIT()
    unsafe extern "C" {
        static RV_MON_OPS_INIT_VALUE: rv_monitor_ops;
    }

    #[unsafe(no_mangle)]
    pub static rv_test_da_kunit_ops: rv_test_da_kunit_ops = rv_test_da_kunit_ops {
        mon: unsafe { RV_MON_OPS_INIT_VALUE },
        handle_event_1: Some(handle_event_1),
        handle_event_2: Some(handle_event_2),
    };

    // EXPORT_SYMBOL_IF_KUNIT(rv_test_da_kunit_ops);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
