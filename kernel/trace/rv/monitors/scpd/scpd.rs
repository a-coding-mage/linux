// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the kernel, RV instrumentation, and monitor headers.

pub const MODULE_NAME: &str = "scpd";

// Build-time monitor configuration: RV_MON_TYPE is RV_MON_PER_CPU.

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
    fn da_handle_event(event: core::ffi::c_int);
    fn da_handle_start_event(event: core::ffi::c_int);
    fn da_monitor_init() -> core::ffi::c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();
    fn rv_attach_trace_probe(
        module: *const core::ffi::c_char,
        probe: *const core::ffi::c_void,
        handler: *const core::ffi::c_void,
    );
    fn rv_detach_trace_probe(
        module: *const core::ffi::c_char,
        probe: *const core::ffi::c_void,
        handler: *const core::ffi::c_void,
    );
    fn rv_register_monitor(monitor: *mut rv_monitor, sched: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);

    static mut rv_sched: core::ffi::c_void;
    static preempt_disable: core::ffi::c_void;
    static preempt_enable: core::ffi::c_void;
    static sched_entry_tp: core::ffi::c_void;
    static sched_exit_tp: core::ffi::c_void;
    static preempt_disable_scpd: core::ffi::c_int;
    static preempt_enable_scpd: core::ffi::c_int;
    static schedule_entry_scpd: core::ffi::c_int;
    static schedule_exit_scpd: core::ffi::c_int;
}

unsafe extern "C" fn handle_preempt_disable(
    _data: *mut core::ffi::c_void,
    _ip: c_ulong,
    _parent_ip: c_ulong,
) {
    da_handle_event(preempt_disable_scpd);
}

unsafe extern "C" fn handle_preempt_enable(
    _data: *mut core::ffi::c_void,
    _ip: c_ulong,
    _parent_ip: c_ulong,
) {
    da_handle_start_event(preempt_enable_scpd);
}

unsafe extern "C" fn handle_schedule_entry(_data: *mut core::ffi::c_void, _preempt: bool) {
    da_handle_event(schedule_entry_scpd);
}

unsafe extern "C" fn handle_schedule_exit(_data: *mut core::ffi::c_void, _is_switch: bool) {
    da_handle_event(schedule_exit_scpd);
}

type c_ulong = core::ffi::c_ulong;

unsafe extern "C" fn enable_scpd() -> core::ffi::c_int {
    let retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(b"scpd\0".as_ptr() as *const _, &preempt_disable as *const _ as *const _, handle_preempt_disable as *const _);
    rv_attach_trace_probe(b"scpd\0".as_ptr() as *const _, &preempt_enable as *const _ as *const _, handle_preempt_enable as *const _);
    rv_attach_trace_probe(b"scpd\0".as_ptr() as *const _, &sched_entry_tp as *const _ as *const _, handle_schedule_entry as *const _);
    rv_attach_trace_probe(b"scpd\0".as_ptr() as *const _, &sched_exit_tp as *const _ as *const _, handle_schedule_exit as *const _);

    0
}

unsafe extern "C" fn disable_scpd() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(b"scpd\0".as_ptr() as *const _, &preempt_disable as *const _ as *const _, handle_preempt_disable as *const _);
    rv_detach_trace_probe(b"scpd\0".as_ptr() as *const _, &preempt_enable as *const _ as *const _, handle_preempt_enable as *const _);
    rv_detach_trace_probe(b"scpd\0".as_ptr() as *const _, &sched_entry_tp as *const _ as *const _, handle_schedule_entry as *const _);
    rv_detach_trace_probe(b"scpd\0".as_ptr() as *const _, &sched_exit_tp as *const _ as *const _, handle_schedule_exit as *const _);

    da_monitor_destroy();
}

#[no_mangle]
pub static mut rv_this: rv_monitor = rv_monitor {
    name: b"scpd\0".as_ptr() as *const _,
    description: b"schedule called with preemption disabled.\0".as_ptr() as *const _,
    enable: Some(enable_scpd),
    disable: Some(disable_scpd),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_scpd() -> core::ffi::c_int {
    rv_register_monitor(&mut rv_this, &mut rv_sched)
}

unsafe extern "C" fn unregister_scpd() {
    rv_unregister_monitor(&mut rv_this);
}

// module_init(register_scpd);
// module_exit(unregister_scpd);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("scpd: schedule called with preemption disabled.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
