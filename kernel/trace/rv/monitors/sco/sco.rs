// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/ftrace.h>, <linux/tracepoint.h>, <linux/kernel.h>,
// <linux/module.h>, <linux/init.h>, <linux/rv.h>, <rv/instrumentation.h>,
// <trace/events/sched.h>, <rv_trace.h>, <monitors/sched/sched.h>, "sco.h",
// and <rv/da_monitor.h>.

pub const MODULE_NAME: &str = "sco";

// Build-time configuration: #define RV_MON_TYPE RV_MON_PER_CPU
// The declarations below are supplied by the surrounding kernel/RV modules.

extern "C" {
    fn da_monitor_init() -> core::ffi::c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();
    fn rv_attach_trace_probe(
        name: *const core::ffi::c_char,
        probe: *const core::ffi::c_void,
        handler: *const core::ffi::c_void,
    );
    fn rv_detach_trace_probe(
        name: *const core::ffi::c_char,
        probe: *const core::ffi::c_void,
        handler: *const core::ffi::c_void,
    );
    fn rv_register_monitor(monitor: *mut rv_monitor, schedule_monitor: *mut rv_monitor)
        -> core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
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

extern "C" {
    static mut rv_sched: rv_monitor;
}

unsafe extern "C" fn handle_sched_set_state(
    _data: *mut core::ffi::c_void,
    _tsk: *mut task_struct,
    _state: core::ffi::c_int,
) {
    da_handle_start_event!(sched_set_state_sco);
}

unsafe extern "C" fn handle_schedule_entry(
    _data: *mut core::ffi::c_void,
    _preempt: bool,
) {
    da_handle_event!(schedule_entry_sco);
}

unsafe extern "C" fn handle_schedule_exit(
    _data: *mut core::ffi::c_void,
    _is_switch: bool,
) {
    da_handle_start_event!(schedule_exit_sco);
}

unsafe extern "C" fn enable_sco() -> core::ffi::c_int {
    let retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        b"sco\0".as_ptr() as *const core::ffi::c_char,
        sched_set_state_tp,
        handle_sched_set_state as *const core::ffi::c_void,
    );
    rv_attach_trace_probe(
        b"sco\0".as_ptr() as *const core::ffi::c_char,
        sched_entry_tp,
        handle_schedule_entry as *const core::ffi::c_void,
    );
    rv_attach_trace_probe(
        b"sco\0".as_ptr() as *const core::ffi::c_char,
        sched_exit_tp,
        handle_schedule_exit as *const core::ffi::c_void,
    );

    0
}

unsafe extern "C" fn disable_sco() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        b"sco\0".as_ptr() as *const core::ffi::c_char,
        sched_set_state_tp,
        handle_sched_set_state as *const core::ffi::c_void,
    );
    rv_detach_trace_probe(
        b"sco\0".as_ptr() as *const core::ffi::c_char,
        sched_entry_tp,
        handle_schedule_entry as *const core::ffi::c_void,
    );
    rv_detach_trace_probe(
        b"sco\0".as_ptr() as *const core::ffi::c_char,
        sched_exit_tp,
        handle_schedule_exit as *const core::ffi::c_void,
    );

    da_monitor_destroy();
}

pub static mut rv_this: rv_monitor = rv_monitor {
    name: b"sco\0".as_ptr() as *const core::ffi::c_char,
    description: b"scheduling context operations.\0".as_ptr() as *const core::ffi::c_char,
    enable: Some(enable_sco),
    disable: Some(disable_sco),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_sco() -> core::ffi::c_int {
    rv_register_monitor(&raw mut rv_this, &raw mut rv_sched)
}

unsafe extern "C" fn unregister_sco() {
    rv_unregister_monitor(&raw mut rv_this);
}

// module_init(register_sco);
// module_exit(unregister_sco);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("sco: scheduling context operations.");

// Conditional build section: #if IS_ENABLED(CONFIG_RV_MONITORS_KUNIT_TEST)
// KUnit visibility and sco_kunit.h declarations are supplied by the build.
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
#[repr(C)]
pub struct rv_sco_ops {
    pub mon: rv_monitor_ops,
    pub handle_sched_set_state:
        Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut task_struct, core::ffi::c_int)>,
    pub handle_schedule_entry: Option<unsafe extern "C" fn(*mut core::ffi::c_void, bool)>,
    pub handle_schedule_exit: Option<unsafe extern "C" fn(*mut core::ffi::c_void, bool)>,
}

#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub static rv_sco_ops: rv_sco_ops = rv_sco_ops {
    mon: RV_MON_OPS_INIT!(),
    handle_sched_set_state: Some(handle_sched_set_state),
    handle_schedule_entry: Some(handle_schedule_entry),
    handle_schedule_exit: Some(handle_schedule_exit),
};
// EXPORT_SYMBOL_IF_KUNIT(rv_sco_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
