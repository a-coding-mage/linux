// SPDX-License-Identifier: GPL-2.0
// Translated from the C implementation. Kernel include dependencies are
// supplied by the surrounding build.

pub const MODULE_NAME: &str = "snroc";

// The following declarations correspond to symbols supplied by the included
// kernel and RV monitor headers.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
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
    static mut rv_sched: core::ffi::c_void;

    fn da_monitor_init() -> core::ffi::c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();
    fn da_handle_event(tsk: *mut task_struct, event: core::ffi::c_int);
    fn da_handle_start_event(tsk: *mut task_struct, event: core::ffi::c_int);

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
    fn rv_register_monitor(
        monitor: *mut rv_monitor,
        sched: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);

    static sched_set_state_tp: core::ffi::c_void;
    static sched_switch: core::ffi::c_void;
    static sched_set_state_snroc: core::ffi::c_int;
    static sched_switch_out_snroc: core::ffi::c_int;
    static sched_switch_in_snroc: core::ffi::c_int;
}

unsafe extern "C" fn handle_sched_set_state(
    _data: *mut core::ffi::c_void,
    tsk: *mut task_struct,
    _state: core::ffi::c_int,
) {
    da_handle_event(tsk, sched_set_state_snroc);
}

unsafe extern "C" fn handle_sched_switch(
    _data: *mut core::ffi::c_void,
    _preempt: bool,
    prev: *mut task_struct,
    next: *mut task_struct,
    _prev_state: core::ffi::c_uint,
) {
    da_handle_start_event(prev, sched_switch_out_snroc);
    da_handle_event(next, sched_switch_in_snroc);
}

unsafe extern "C" fn enable_snroc() -> core::ffi::c_int {
    let retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        b"snroc\0".as_ptr() as *const core::ffi::c_char,
        &sched_set_state_tp as *const _ as *const core::ffi::c_void,
        handle_sched_set_state as *const () as *const core::ffi::c_void,
    );
    rv_attach_trace_probe(
        b"snroc\0".as_ptr() as *const core::ffi::c_char,
        &sched_switch as *const _ as *const core::ffi::c_void,
        handle_sched_switch as *const () as *const core::ffi::c_void,
    );

    0
}

unsafe extern "C" fn disable_snroc() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        b"snroc\0".as_ptr() as *const core::ffi::c_char,
        &sched_set_state_tp as *const _ as *const core::ffi::c_void,
        handle_sched_set_state as *const () as *const core::ffi::c_void,
    );
    rv_detach_trace_probe(
        b"snroc\0".as_ptr() as *const core::ffi::c_char,
        &sched_switch as *const _ as *const core::ffi::c_void,
        handle_sched_switch as *const () as *const core::ffi::c_void,
    );

    da_monitor_destroy();
}

static mut rv_this: rv_monitor = rv_monitor {
    name: b"snroc\0".as_ptr() as *const core::ffi::c_char,
    description: b"set non runnable on its own context.\0".as_ptr()
        as *const core::ffi::c_char,
    enable: Some(enable_snroc),
    disable: Some(disable_snroc),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_snroc() -> core::ffi::c_int {
    rv_register_monitor(&mut rv_this, &mut rv_sched);
}

unsafe extern "C" fn unregister_snroc() {
    rv_unregister_monitor(&mut rv_this);
}

// C module_init(register_snroc) and module_exit(unregister_snroc) metadata.
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("snroc: set non runnable on its own context.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
