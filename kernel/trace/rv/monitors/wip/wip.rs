// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C includes provide these declarations and types.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn da_handle_event(event: c_int);
    fn da_handle_start_event(event: c_int);
    fn da_monitor_init() -> c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();
    fn rv_attach_trace_probe(name: *const c_char, probe: *mut c_void, handler: *mut c_void);
    fn rv_detach_trace_probe(name: *const c_char, probe: *mut c_void, handler: *mut c_void);
    fn rv_register_monitor(monitor: *mut rv_monitor, parent: *mut c_void) -> c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);

    static mut preempt_enable: c_void;
    static mut sched_waking: c_void;
    static mut preempt_disable: c_void;
    static mut preempt_disable_wip: c_int;
    static mut preempt_enable_wip: c_int;
    static mut sched_waking_wip: c_int;
}

#[repr(C)]
struct rv_monitor {
    name: *const c_char,
    description: *const c_char,
    enable: Option<unsafe extern "C" fn() -> c_int>,
    disable: Option<unsafe extern "C" fn()>,
    reset: Option<unsafe extern "C" fn()>,
    enabled: c_int,
}

unsafe extern "C" fn handle_preempt_disable(
    _data: *mut c_void,
    _ip: c_ulong,
    _parent_ip: c_ulong,
) {
    da_handle_event(preempt_disable_wip);
}

unsafe extern "C" fn handle_preempt_enable(
    _data: *mut c_void,
    _ip: c_ulong,
    _parent_ip: c_ulong,
) {
    da_handle_start_event(preempt_enable_wip);
}

unsafe extern "C" fn handle_sched_waking(_data: *mut c_void, _task: *mut task_struct) {
    da_handle_event(sched_waking_wip);
}

unsafe extern "C" fn enable_wip() -> c_int {
    let retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        b"wip\0".as_ptr() as *const c_char,
        &mut preempt_enable as *mut c_void,
        handle_preempt_enable as *mut c_void,
    );
    rv_attach_trace_probe(
        b"wip\0".as_ptr() as *const c_char,
        &mut sched_waking as *mut c_void,
        handle_sched_waking as *mut c_void,
    );
    rv_attach_trace_probe(
        b"wip\0".as_ptr() as *const c_char,
        &mut preempt_disable as *mut c_void,
        handle_preempt_disable as *mut c_void,
    );

    0
}

unsafe extern "C" fn disable_wip() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        b"wip\0".as_ptr() as *const c_char,
        &mut preempt_disable as *mut c_void,
        handle_preempt_disable as *mut c_void,
    );
    rv_detach_trace_probe(
        b"wip\0".as_ptr() as *const c_char,
        &mut preempt_enable as *mut c_void,
        handle_preempt_enable as *mut c_void,
    );
    rv_detach_trace_probe(
        b"wip\0".as_ptr() as *const c_char,
        &mut sched_waking as *mut c_void,
        handle_sched_waking as *mut c_void,
    );

    da_monitor_destroy();
}

static mut rv_this: rv_monitor = rv_monitor {
    name: b"wip\0".as_ptr() as *const c_char,
    description: b"wakeup in preemptive per-cpu testing monitor.\0".as_ptr() as *const c_char,
    enable: Some(enable_wip),
    disable: Some(disable_wip),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_wip() -> c_int {
    rv_register_monitor(&mut rv_this, core::ptr::null_mut())
}

unsafe extern "C" fn unregister_wip() {
    rv_unregister_monitor(&mut rv_this);
}

// module_init(register_wip);
// module_exit(unregister_wip);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Daniel Bristot de Oliveira <bristot@kernel.org>");
// MODULE_DESCRIPTION("wip: wakeup in preemptive - per-cpu sample monitor.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
