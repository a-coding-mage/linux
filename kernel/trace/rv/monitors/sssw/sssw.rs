// SPDX-License-Identifier: GPL-2.0
// Dependency includes from the C implementation are supplied by the surrounding kernel/RV build.

pub const MODULE_NAME: &[u8] = b"sssw\0";

// Build-time conditional: RV_MON_TYPE is RV_MON_PER_TASK in the C source.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernel_siginfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct k_sigaction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tracepoint {
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
    static mut current: *mut task_struct;
    static mut rv_this: rv_monitor;
    static mut rv_sched: rv_monitor;

    fn da_monitor_init() -> core::ffi::c_int;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();
    fn da_handle_start_event(tsk: *mut task_struct, event: core::ffi::c_int);
    fn da_handle_event(tsk: *mut task_struct, event: core::ffi::c_int);

    fn rv_attach_trace_probe(
        name: *const core::ffi::c_char,
        tp: *const tracepoint,
        probe: unsafe extern "C" fn(
            *mut core::ffi::c_void,
            ...,
        ),
    );
    fn rv_detach_trace_probe(
        name: *const core::ffi::c_char,
        tp: *const tracepoint,
        probe: unsafe extern "C" fn(*mut core::ffi::c_void, ...),
    );
    fn rv_register_monitor(mon: *mut rv_monitor, sched: *mut rv_monitor) -> core::ffi::c_int;
    fn rv_unregister_monitor(mon: *mut rv_monitor);

    static sched_set_state_tp: tracepoint;
    static sched_switch: tracepoint;
    static sched_wakeup: tracepoint;
    static signal_deliver: tracepoint;

    static sched_set_state_runnable_sssw: core::ffi::c_int;
    static sched_set_state_sleepable_sssw: core::ffi::c_int;
    static sched_switch_preempt_sssw: core::ffi::c_int;
    static sched_switch_yield_sssw: core::ffi::c_int;
    static sched_switch_blocking_sssw: core::ffi::c_int;
    static sched_switch_suspend_sssw: core::ffi::c_int;
    static sched_switch_in_sssw: core::ffi::c_int;
    static sched_wakeup_sssw: core::ffi::c_int;
    static signal_deliver_sssw: core::ffi::c_int;
}

pub const TASK_RUNNING: core::ffi::c_int = 0;
pub const TASK_RTLOCK_WAIT: core::ffi::c_uint = 1;

#[no_mangle]
pub unsafe extern "C" fn handle_sched_set_state(
    _data: *mut core::ffi::c_void,
    tsk: *mut task_struct,
    state: core::ffi::c_int,
) {
    if state == TASK_RUNNING {
        da_handle_start_event(tsk, sched_set_state_runnable_sssw);
    } else {
        da_handle_event(tsk, sched_set_state_sleepable_sssw);
    }
}

#[no_mangle]
pub unsafe extern "C" fn handle_sched_switch(
    _data: *mut core::ffi::c_void,
    preempt: bool,
    prev: *mut task_struct,
    next: *mut task_struct,
    prev_state: core::ffi::c_uint,
) {
    if preempt {
        da_handle_event(prev, sched_switch_preempt_sssw);
    } else if prev_state == TASK_RUNNING as core::ffi::c_uint {
        da_handle_event(prev, sched_switch_yield_sssw);
    } else if prev_state == TASK_RTLOCK_WAIT {
        // special case of sleeping task with racy conditions
        da_handle_event(prev, sched_switch_blocking_sssw);
    } else {
        da_handle_event(prev, sched_switch_suspend_sssw);
    }
    da_handle_event(next, sched_switch_in_sssw);
}

#[no_mangle]
pub unsafe extern "C" fn handle_sched_wakeup(
    _data: *mut core::ffi::c_void,
    p: *mut task_struct,
) {
    // Wakeup can also lead to signal_wakeup although the system is
    // actually runnable. The monitor can safely start with this event.
    da_handle_start_event(p, sched_wakeup_sssw);
}

#[no_mangle]
pub unsafe extern "C" fn handle_signal_deliver(
    _data: *mut core::ffi::c_void,
    _sig: core::ffi::c_int,
    _info: *mut kernel_siginfo,
    _ka: *mut k_sigaction,
) {
    da_handle_event(current, signal_deliver_sssw);
}

unsafe extern "C" fn enable_sssw() -> core::ffi::c_int {
    let retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(b"sssw\0".as_ptr() as *const _, &sched_set_state_tp, handle_sched_set_state as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    rv_attach_trace_probe(b"sssw\0".as_ptr() as *const _, &sched_switch, handle_sched_switch as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    rv_attach_trace_probe(b"sssw\0".as_ptr() as *const _, &sched_wakeup, handle_sched_wakeup as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    rv_attach_trace_probe(b"sssw\0".as_ptr() as *const _, &signal_deliver, handle_signal_deliver as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    0
}

unsafe extern "C" fn disable_sssw() {
    rv_this.enabled = 0;
    rv_detach_trace_probe(b"sssw\0".as_ptr() as *const _, &sched_set_state_tp, handle_sched_set_state as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    rv_detach_trace_probe(b"sssw\0".as_ptr() as *const _, &sched_switch, handle_sched_switch as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    rv_detach_trace_probe(b"sssw\0".as_ptr() as *const _, &sched_wakeup, handle_sched_wakeup as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    rv_detach_trace_probe(b"sssw\0".as_ptr() as *const _, &signal_deliver, handle_signal_deliver as unsafe extern "C" fn(*mut core::ffi::c_void, ...));
    da_monitor_destroy();
}

#[no_mangle]
pub static mut rv_this: rv_monitor = rv_monitor {
    name: b"sssw\0".as_ptr() as *const _,
    description: b"set state sleep and wakeup.\0".as_ptr() as *const _,
    enable: Some(enable_sssw),
    disable: Some(disable_sssw),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_sssw() -> core::ffi::c_int {
    rv_register_monitor(&mut rv_this, &mut rv_sched)
}

unsafe extern "C" fn unregister_sssw() {
    rv_unregister_monitor(&mut rv_this);
}

// module_init(register_sssw); module_exit(unregister_sssw);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("sssw: set state sleep and wakeup.");

// Preserved conditional intent: this block is present when
// CONFIG_RV_MONITORS_KUNIT_TEST is enabled.
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub struct rv_sssw_ops {
    pub mon: (),
    pub handle_sched_set_state: unsafe extern "C" fn(*mut core::ffi::c_void, *mut task_struct, core::ffi::c_int),
    pub handle_sched_switch: unsafe extern "C" fn(*mut core::ffi::c_void, bool, *mut task_struct, *mut task_struct, core::ffi::c_uint),
    pub handle_sched_wakeup: unsafe extern "C" fn(*mut core::ffi::c_void, *mut task_struct),
    pub handle_signal_deliver: unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int, *mut kernel_siginfo, *mut k_sigaction),
}

#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
#[no_mangle]
pub static rv_sssw_ops_instance: rv_sssw_ops = rv_sssw_ops {
    mon: (),
    handle_sched_set_state,
    handle_sched_switch,
    handle_sched_wakeup,
    handle_signal_deliver,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
