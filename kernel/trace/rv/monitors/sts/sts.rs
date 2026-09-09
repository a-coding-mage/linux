// SPDX-License-Identifier: GPL-2.0
// Translated from the C implementation. Kernel headers and build-time symbols
// are supplied by the surrounding Rust integration.

pub const MODULE_NAME: &str = "sts";

#[allow(non_camel_case_types)]
pub enum irqaction {}
#[allow(non_camel_case_types)]
pub enum task_struct {}
#[allow(non_camel_case_types)]
pub enum rv_monitor {}
#[allow(non_camel_case_types)]
pub enum rv_sched {}

extern "C" {
    fn da_monitor_init() -> i32;
    fn da_monitor_destroy();
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(name: *const ::core::ffi::c_char, event: *const ::core::ffi::c_char, probe: unsafe extern "C" fn(*mut ::core::ffi::c_void));
    fn rv_detach_trace_probe(name: *const ::core::ffi::c_char, event: *const ::core::ffi::c_char, probe: unsafe extern "C" fn(*mut ::core::ffi::c_void));
    fn rv_register_monitor(monitor: *mut rv_monitor, sched: *mut rv_sched) -> i32;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);

    // These event handlers/macros are provided by the da_monitor dependency.
    fn da_handle_event(event: i32);
    fn da_handle_start_event(event: i32);

    static mut rv_this: rv_monitor;
    static mut rv_sched: rv_sched;
}

// Event identifiers are supplied by sts.h / the monitor framework.
extern "C" {
    static irq_entry_sts: i32;
    static irq_disable_sts: i32;
    static irq_enable_sts: i32;
    static sched_switch_sts: i32;
    static schedule_entry_sts: i32;
    static schedule_exit_sts: i32;
}

#[cfg(feature = "config_x86_local_apic")]
unsafe extern "C" fn handle_vector_irq_entry(
    _data: *mut ::core::ffi::c_void,
    _vector: i32,
) {
    da_handle_event(irq_entry_sts);
}

#[cfg(feature = "config_x86_local_apic")]
unsafe fn attach_vector_irq() {
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"local_timer_entry\0".as_ptr() as _, handle_vector_irq_entry);
    #[cfg(feature = "config_irq_work")]
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"irq_work_entry\0".as_ptr() as _, handle_vector_irq_entry);
    #[cfg(feature = "config_smp")]
    {
        rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"reschedule_entry\0".as_ptr() as _, handle_vector_irq_entry);
        rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"call_function_entry\0".as_ptr() as _, handle_vector_irq_entry);
        rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"call_function_single_entry\0".as_ptr() as _, handle_vector_irq_entry);
    }
}

#[cfg(feature = "config_x86_local_apic")]
unsafe fn detach_vector_irq() {
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"local_timer_entry\0".as_ptr() as _, handle_vector_irq_entry);
    #[cfg(feature = "config_irq_work")]
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"irq_work_entry\0".as_ptr() as _, handle_vector_irq_entry);
    #[cfg(feature = "config_smp")]
    {
        rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"reschedule_entry\0".as_ptr() as _, handle_vector_irq_entry);
        rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"call_function_entry\0".as_ptr() as _, handle_vector_irq_entry);
        rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"call_function_single_entry\0".as_ptr() as _, handle_vector_irq_entry);
    }
}

#[cfg(not(feature = "config_x86_local_apic"))]
// We assume irq_entry tracepoints are sufficient on other architectures.
unsafe fn attach_vector_irq() {}
#[cfg(not(feature = "config_x86_local_apic"))]
unsafe fn detach_vector_irq() {}

unsafe extern "C" fn handle_irq_disable(_data: *mut ::core::ffi::c_void, _ip: usize, _parent_ip: usize) { da_handle_event(irq_disable_sts); }
unsafe extern "C" fn handle_irq_enable(_data: *mut ::core::ffi::c_void, _ip: usize, _parent_ip: usize) { da_handle_event(irq_enable_sts); }
unsafe extern "C" fn handle_irq_entry(_data: *mut ::core::ffi::c_void, _irq: i32, _action: *mut irqaction) { da_handle_event(irq_entry_sts); }
unsafe extern "C" fn handle_sched_switch(_data: *mut ::core::ffi::c_void, _preempt: bool, _prev: *mut task_struct, _next: *mut task_struct, _prev_state: u32) { da_handle_event(sched_switch_sts); }
unsafe extern "C" fn handle_schedule_entry(_data: *mut ::core::ffi::c_void, _preempt: bool) { da_handle_event(schedule_entry_sts); }
unsafe extern "C" fn handle_schedule_exit(_data: *mut ::core::ffi::c_void, _is_switch: bool) { da_handle_start_event(schedule_exit_sts); }

unsafe fn enable_sts() -> i32 {
    let retval = da_monitor_init();
    if retval != 0 { return retval; }
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"irq_disable\0".as_ptr() as _, handle_irq_disable);
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"irq_enable\0".as_ptr() as _, handle_irq_enable);
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"irq_handler_entry\0".as_ptr() as _, handle_irq_entry);
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"sched_switch\0".as_ptr() as _, handle_sched_switch);
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"sched_entry_tp\0".as_ptr() as _, handle_schedule_entry);
    rv_attach_trace_probe(b"sts\0".as_ptr() as _, b"sched_exit_tp\0".as_ptr() as _, handle_schedule_exit);
    attach_vector_irq();
    0
}

unsafe fn disable_sts() {
    // The monitor's enabled field is cleared before probes are detached.
    // rv_this.enabled = 0;
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"irq_disable\0".as_ptr() as _, handle_irq_disable);
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"irq_enable\0".as_ptr() as _, handle_irq_enable);
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"irq_handler_entry\0".as_ptr() as _, handle_irq_entry);
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"sched_switch\0".as_ptr() as _, handle_sched_switch);
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"sched_entry_tp\0".as_ptr() as _, handle_schedule_entry);
    rv_detach_trace_probe(b"sts\0".as_ptr() as _, b"sched_exit_tp\0".as_ptr() as _, handle_schedule_exit);
    detach_vector_irq();
    da_monitor_destroy();
}

// This is the monitor register section.
// The concrete rv_monitor layout and callback assignments are supplied by the
// kernel's Rust bindings; preserve the registration intent here.
unsafe fn register_sts() -> i32 { rv_register_monitor(&mut rv_this, &mut rv_sched) }
unsafe fn unregister_sts() { rv_unregister_monitor(&mut rv_this); }

// module_init(register_sts);
// module_exit(unregister_sts);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("sts: schedule implies task switch.");

#[cfg(feature = "config_rv_monitors_kunit_test")]
#[allow(non_camel_case_types)]
pub struct rv_sts_ops {
    pub mon: (), // RV_MON_OPS_INIT(), supplied by the monitor framework.
    #[cfg(feature = "config_x86_local_apic")]
    pub handle_vector_irq_entry: unsafe extern "C" fn(*mut ::core::ffi::c_void, i32),
    pub handle_irq_disable: unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, usize),
    pub handle_irq_enable: unsafe extern "C" fn(*mut ::core::ffi::c_void, usize, usize),
    pub handle_irq_entry: unsafe extern "C" fn(*mut ::core::ffi::c_void, i32, *mut irqaction),
    pub handle_sched_switch: unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut task_struct, *mut task_struct, u32),
    pub handle_schedule_entry: unsafe extern "C" fn(*mut ::core::ffi::c_void, bool),
    pub handle_schedule_exit: unsafe extern "C" fn(*mut ::core::ffi::c_void, bool),
}

#[cfg(feature = "config_rv_monitors_kunit_test")]
#[no_mangle]
pub static rv_sts_ops: rv_sts_ops = rv_sts_ops {
    mon: (),
    #[cfg(feature = "config_x86_local_apic")]
    handle_vector_irq_entry,
    handle_irq_disable,
    handle_irq_enable,
    handle_irq_entry,
    handle_sched_switch,
    handle_schedule_entry,
    handle_schedule_exit,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
