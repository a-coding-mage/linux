// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the kernel and monitor framework are referenced externally.

use core::ffi::c_void;

const MODULE_NAME: &str = "nrp";

// The CONFIG_X86_LOCAL_APIC condition is supplied by the kernel build configuration.
#[cfg(CONFIG_X86_LOCAL_APIC)]
unsafe extern "C" {
    fn rv_attach_trace_probe(name: *const i8, probe: *const c_void, handler: *const c_void);
    fn rv_detach_trace_probe(name: *const i8, probe: *const c_void, handler: *const c_void);
    fn da_handle_event(task: *mut task_struct, event: i32);
    fn da_handle_start_event(task: *mut task_struct, event: i32);
    fn da_monitor_init() -> i32;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();
    fn rv_register_monitor(monitor: *mut rv_monitor, sched: *mut c_void) -> i32;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
    static mut rv_this: rv_monitor;
    static mut rv_sched: c_void;
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irqaction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const i8,
    pub description: *const i8,
    pub enable: Option<unsafe extern "C" fn() -> i32>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: i32,
}

// External declarations from the included kernel headers.
extern "C" {
    static mut current: *mut task_struct;
    static sched_need_resched_tp: c_void;
    static sched_entry_tp: c_void;
    static irq_handler_entry: c_void;
    static local_timer_entry: c_void;
    static irq_work_entry: c_void;
    static reschedule_entry: c_void;
    static call_function_entry: c_void;
    static call_function_single_entry: c_void;
}

// Event identifiers supplied by nrp.h.
extern "C" {
    static irq_entry_nrp: i32;
    static sched_need_resched_nrp: i32;
    static schedule_entry_preempt_nrp: i32;
    static schedule_entry_nrp: i32;
    static TIF_NEED_RESCHED: i32;
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
unsafe extern "C" fn handle_vector_irq_entry(_data: *mut c_void, _vector: i32) {
    da_handle_event(current, irq_entry_nrp);
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
unsafe fn attach_vector_irq() {
    rv_attach_trace_probe(
        b"nrp\0".as_ptr() as *const i8,
        &local_timer_entry,
        handle_vector_irq_entry as *const c_void,
    );
    // IS_ENABLED(CONFIG_IRQ_WORK) is controlled by the kernel build configuration.
    rv_attach_trace_probe(b"nrp\0".as_ptr() as *const i8, &irq_work_entry, handle_vector_irq_entry as *const c_void);
    // IS_ENABLED(CONFIG_SMP) is controlled by the kernel build configuration.
    rv_attach_trace_probe(b"nrp\0".as_ptr() as *const i8, &reschedule_entry, handle_vector_irq_entry as *const c_void);
    rv_attach_trace_probe(b"nrp\0".as_ptr() as *const i8, &call_function_entry, handle_vector_irq_entry as *const c_void);
    rv_attach_trace_probe(b"nrp\0".as_ptr() as *const i8, &call_function_single_entry, handle_vector_irq_entry as *const c_void);
}

#[cfg(not(CONFIG_X86_LOCAL_APIC))]
unsafe fn attach_vector_irq() {}

#[cfg(CONFIG_X86_LOCAL_APIC)]
unsafe fn detach_vector_irq() {
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &local_timer_entry, handle_vector_irq_entry as *const c_void);
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &irq_work_entry, handle_vector_irq_entry as *const c_void);
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &reschedule_entry, handle_vector_irq_entry as *const c_void);
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &call_function_entry, handle_vector_irq_entry as *const c_void);
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &call_function_single_entry, handle_vector_irq_entry as *const c_void);
}

#[cfg(not(CONFIG_X86_LOCAL_APIC))]
unsafe fn detach_vector_irq() {}

unsafe extern "C" fn handle_irq_entry(_data: *mut c_void, _irq: i32, _action: *mut irqaction) {
    da_handle_event(current, irq_entry_nrp);
}

unsafe extern "C" fn handle_sched_need_resched(_data: *mut c_void, tsk: *mut task_struct, _cpu: i32, tif: i32) {
    // Although need_resched leads to both the rescheduling and preempt_irq states,
    // it is safer to start the monitor always in preempt_irq.
    if tif == TIF_NEED_RESCHED {
        da_handle_start_event(tsk, sched_need_resched_nrp);
    }
}

unsafe extern "C" fn handle_schedule_entry(_data: *mut c_void, preempt: bool) {
    if preempt {
        da_handle_event(current, schedule_entry_preempt_nrp);
    } else {
        da_handle_event(current, schedule_entry_nrp);
    }
}

unsafe extern "C" fn enable_nrp() -> i32 {
    let retval = da_monitor_init();
    if retval != 0 { return retval; }
    rv_attach_trace_probe(b"nrp\0".as_ptr() as *const i8, &irq_handler_entry, handle_irq_entry as *const c_void);
    rv_attach_trace_probe(b"nrp\0".as_ptr() as *const i8, &sched_need_resched_tp, handle_sched_need_resched as *const c_void);
    rv_attach_trace_probe(b"nrp\0".as_ptr() as *const i8, &sched_entry_tp, handle_schedule_entry as *const c_void);
    attach_vector_irq();
    0
}

unsafe extern "C" fn disable_nrp() {
    rv_this.enabled = 0;
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &irq_handler_entry, handle_irq_entry as *const c_void);
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &sched_need_resched_tp, handle_sched_need_resched as *const c_void);
    rv_detach_trace_probe(b"nrp\0".as_ptr() as *const i8, &sched_entry_tp, handle_schedule_entry as *const c_void);
    detach_vector_irq();
    da_monitor_destroy();
}

static mut rv_this: rv_monitor = rv_monitor {
    name: b"nrp\0".as_ptr() as *const i8,
    description: b"need resched preempts.\0".as_ptr() as *const i8,
    enable: Some(enable_nrp),
    disable: Some(disable_nrp),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_nrp() -> i32 {
    rv_register_monitor(&raw mut rv_this, &raw mut rv_sched)
}

unsafe extern "C" fn unregister_nrp() {
    rv_unregister_monitor(&raw mut rv_this);
}

// module_init(register_nrp); module_exit(unregister_nrp);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("nrp: need resched preempts.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
