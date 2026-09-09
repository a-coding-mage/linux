// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel, RV instrumentation, and the
// monitor headers are intentionally left as external declarations.

use core::ffi::c_void;

const MODULE_NAME: &[u8] = b"wwnr\0";
const RV_MON_TYPE: i32 = RV_MON_PER_TASK;

extern "C" {
    fn da_handle_start_event(task: *mut task_struct, event: switch_out_wwnr);
    fn da_handle_event(task: *mut task_struct, event: switch_out_wwnr);
    fn da_monitor_init() -> i32;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(
        name: *const core::ffi::c_char,
        probe: *const c_void,
        handler: *const c_void,
    );
    fn rv_detach_trace_probe(
        name: *const core::ffi::c_char,
        probe: *const c_void,
        handler: *const c_void,
    );
    fn rv_register_monitor(monitor: *mut rv_monitor, data: *mut c_void) -> i32;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const core::ffi::c_char,
    pub description: *const core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> i32>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: i32,
}

// External monitor/event symbols supplied by the included monitor headers.
extern "C" {
    static RV_MON_PER_TASK: i32;
    static switch_out_wwnr: switch_out_wwnr;
    static switch_in_wwnr: switch_out_wwnr;
    static wakeup_wwnr: switch_out_wwnr;
}

#[repr(C)]
pub struct switch_out_wwnr {
    _private: [u8; 0],
}

unsafe extern "C" fn handle_switch(
    _data: *mut c_void,
    _preempt: bool,
    p: *mut task_struct,
    n: *mut task_struct,
    prev_state: u32,
) {
    // start monitoring only after the first suspension
    if prev_state == TASK_INTERRUPTIBLE {
        da_handle_start_event(p, switch_out_wwnr);
    } else {
        da_handle_event(p, switch_out_wwnr);
    }

    da_handle_event(n, switch_in_wwnr);
}

unsafe extern "C" fn handle_wakeup(_data: *mut c_void, p: *mut task_struct) {
    da_handle_event(p, wakeup_wwnr);
}

unsafe extern "C" fn enable_wwnr() -> i32 {
    let retval = da_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        b"wwnr\0".as_ptr() as *const core::ffi::c_char,
        sched_switch,
        handle_switch as *const c_void,
    );
    rv_attach_trace_probe(
        b"wwnr\0".as_ptr() as *const core::ffi::c_char,
        sched_wakeup,
        handle_wakeup as *const c_void,
    );

    0
}

unsafe extern "C" fn disable_wwnr() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        b"wwnr\0".as_ptr() as *const core::ffi::c_char,
        sched_switch,
        handle_switch as *const c_void,
    );
    rv_detach_trace_probe(
        b"wwnr\0".as_ptr() as *const core::ffi::c_char,
        sched_wakeup,
        handle_wakeup as *const c_void,
    );

    da_monitor_destroy();
}

#[no_mangle]
pub static mut rv_this: rv_monitor = rv_monitor {
    name: b"wwnr\0".as_ptr() as *const core::ffi::c_char,
    description: b"wakeup while not running per-task testing model.\0".as_ptr()
        as *const core::ffi::c_char,
    enable: Some(enable_wwnr),
    disable: Some(disable_wwnr),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_wwnr() -> i32 {
    rv_register_monitor(&mut rv_this, core::ptr::null_mut())
}

unsafe extern "C" fn unregister_wwnr() {
    rv_unregister_monitor(&mut rv_this);
}

// C module metadata and init/exit registration are provided by the kernel
// module environment.
const MODULE_LICENSE: &str = "GPL";
const MODULE_AUTHOR: &str = "Daniel Bristot de Oliveira <bristot@kernel.org>";
const MODULE_DESCRIPTION: &str = "wwnr: wakeup while not running monitor";

extern "C" {
    static TASK_INTERRUPTIBLE: u32;
    static sched_switch: *const c_void;
    static sched_wakeup: *const c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
