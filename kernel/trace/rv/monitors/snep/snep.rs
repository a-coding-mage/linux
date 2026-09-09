// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the kernel/RV monitor environment are intentionally
// left as external symbols.

const MODULE_NAME: &str = "snep";

// Build-time monitor selection from the C source: RV_MON_TYPE RV_MON_PER_CPU.

extern "C" {
    static mut rv_sched: rv_monitor;

    fn da_monitor_init() -> i32;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(name: *const ::std::os::raw::c_char, probe: usize, handler: usize);
    fn rv_detach_trace_probe(name: *const ::std::os::raw::c_char, probe: usize, handler: usize);
    fn rv_register_monitor(monitor: *mut rv_monitor, scheduler: *mut rv_monitor) -> i32;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[repr(C)]
struct rv_monitor {
    name: *const ::std::os::raw::c_char,
    description: *const ::std::os::raw::c_char,
    enable: Option<unsafe extern "C" fn() -> i32>,
    disable: Option<unsafe extern "C" fn()>,
    reset: Option<unsafe extern "C" fn()>,
    enabled: i32,
}

unsafe extern "C" fn handle_preempt_disable(
    _data: *mut ::std::ffi::c_void,
    _ip: ::std::os::raw::c_ulong,
    _parent_ip: ::std::os::raw::c_ulong,
) {
    // C macro from <rv/da_monitor.h>.
    da_handle_start_event(preempt_disable_snep);
}

unsafe extern "C" fn handle_preempt_enable(
    _data: *mut ::std::ffi::c_void,
    _ip: ::std::os::raw::c_ulong,
    _parent_ip: ::std::os::raw::c_ulong,
) {
    // C macro from <rv/da_monitor.h>.
    da_handle_start_event(preempt_enable_snep);
}

unsafe extern "C" fn handle_schedule_entry(
    _data: *mut ::std::ffi::c_void,
    _preempt: bool,
) {
    // C macro from <rv/da_monitor.h>.
    da_handle_event(schedule_entry_snep);
}

unsafe extern "C" fn handle_schedule_exit(
    _data: *mut ::std::ffi::c_void,
    _is_switch: bool,
) {
    // C macro from <rv/da_monitor.h>.
    da_handle_start_event(schedule_exit_snep);
}

unsafe extern "C" fn enable_snep() -> i32 {
    let retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(b"snep\0".as_ptr() as *const _, preempt_disable, handle_preempt_disable as usize);
    rv_attach_trace_probe(b"snep\0".as_ptr() as *const _, preempt_enable, handle_preempt_enable as usize);
    rv_attach_trace_probe(b"snep\0".as_ptr() as *const _, sched_entry_tp, handle_schedule_entry as usize);
    rv_attach_trace_probe(b"snep\0".as_ptr() as *const _, sched_exit_tp, handle_schedule_exit as usize);

    0
}

unsafe extern "C" fn disable_snep() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(b"snep\0".as_ptr() as *const _, preempt_disable, handle_preempt_disable as usize);
    rv_detach_trace_probe(b"snep\0".as_ptr() as *const _, preempt_enable, handle_preempt_enable as usize);
    rv_detach_trace_probe(b"snep\0".as_ptr() as *const _, sched_entry_tp, handle_schedule_entry as usize);
    rv_detach_trace_probe(b"snep\0".as_ptr() as *const _, sched_exit_tp, handle_schedule_exit as usize);

    da_monitor_destroy();
}

static mut rv_this: rv_monitor = rv_monitor {
    name: b"snep\0".as_ptr() as *const _,
    description: b"schedule does not enable preempt.\0".as_ptr() as *const _,
    enable: Some(enable_snep),
    disable: Some(disable_snep),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_snep() -> i32 {
    rv_register_monitor(&mut rv_this, &mut rv_sched)
}

unsafe extern "C" fn unregister_snep() {
    rv_unregister_monitor(&mut rv_this);
}

// C module_init(register_snep) and module_exit(unregister_snep).
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("snep: schedule does not enable preempt.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
