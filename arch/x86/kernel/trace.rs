use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel translation.

#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe extern "C" fn trace_intel_irq_entry(_data: *mut c_void, vector: c_int) {
    osnoise_trace_irq_entry(vector);
}

#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe extern "C" fn trace_intel_irq_exit(data: *mut c_void, vector: c_int) {
    let vector_desc = data as *mut c_char;
    osnoise_trace_irq_exit(vector, vector_desc);
}

#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
pub unsafe fn osnoise_arch_register() -> c_int {
    let mut ret: c_int;

    ret = register_trace_local_timer_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
    if ret != 0 { return osnoise_arch_register_out_err(); }
    ret = register_trace_local_timer_exit(Some(trace_intel_irq_exit), b"local_timer\0".as_ptr() as *const c_char);
    if ret != 0 { unregister_trace_local_timer_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); return -EINVAL; }

    #[cfg(CONFIG_X86_THERMAL_VECTOR)] {
        ret = register_trace_thermal_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
        if ret != 0 { goto_timer_exit(); return -EINVAL; }
        ret = register_trace_thermal_apic_exit(Some(trace_intel_irq_exit), b"thermal_apic\0".as_ptr() as *const c_char);
        if ret != 0 { unregister_trace_thermal_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_timer_exit(); return -EINVAL; }
    }
    #[cfg(CONFIG_X86_MCE_AMD)] {
        ret = register_trace_deferred_error_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
        if ret != 0 { goto_thermal_exit(); return -EINVAL; }
        ret = register_trace_deferred_error_apic_exit(Some(trace_intel_irq_exit), b"deferred_error\0".as_ptr() as *const c_char);
        if ret != 0 { unregister_trace_deferred_error_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_thermal_exit(); return -EINVAL; }
    }
    #[cfg(CONFIG_X86_MCE_THRESHOLD)] {
        ret = register_trace_threshold_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
        if ret != 0 { goto_deferred_exit(); return -EINVAL; }
        ret = register_trace_threshold_apic_exit(Some(trace_intel_irq_exit), b"threshold_apic\0".as_ptr() as *const c_char);
        if ret != 0 { unregister_trace_threshold_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_deferred_exit(); return -EINVAL; }
    }
    #[cfg(CONFIG_SMP)] {
        ret = register_trace_call_function_single_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
        if ret != 0 { goto_threshold_exit(); return -EINVAL; }
        ret = register_trace_call_function_single_exit(Some(trace_intel_irq_exit), b"call_function_single\0".as_ptr() as *const c_char);
        if ret != 0 { unregister_trace_call_function_single_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_threshold_exit(); return -EINVAL; }
        ret = register_trace_call_function_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
        if ret != 0 { unregister_trace_call_function_single_exit(Some(trace_intel_irq_exit), b"call_function_single\0".as_ptr() as *const c_char); unregister_trace_call_function_single_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_threshold_exit(); return -EINVAL; }
        ret = register_trace_call_function_exit(Some(trace_intel_irq_exit), b"call_function\0".as_ptr() as *const c_char);
        if ret != 0 { unregister_trace_call_function_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_call_function_single_exit(); return -EINVAL; }
        ret = register_trace_reschedule_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
        if ret != 0 { unregister_trace_call_function_exit(Some(trace_intel_irq_exit), b"call_function\0".as_ptr() as *const c_char); goto_call_function_entry(); return -EINVAL; }
        ret = register_trace_reschedule_exit(Some(trace_intel_irq_exit), b"reschedule\0".as_ptr() as *const c_char);
        if ret != 0 { unregister_trace_reschedule_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_call_function_exit(); return -EINVAL; }
    }
    #[cfg(CONFIG_IRQ_WORK)] {
        ret = register_trace_irq_work_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
        if ret != 0 { goto_reschedule_exit(); return -EINVAL; }
        ret = register_trace_irq_work_exit(Some(trace_intel_irq_exit), b"irq_work\0".as_ptr() as *const c_char);
        if ret != 0 { unregister_trace_irq_work_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_reschedule_exit(); return -EINVAL; }
    }
    ret = register_trace_x86_platform_ipi_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
    if ret != 0 { goto_irq_work_exit(); return -EINVAL; }
    ret = register_trace_x86_platform_ipi_exit(Some(trace_intel_irq_exit), b"x86_platform_ipi\0".as_ptr() as *const c_char);
    if ret != 0 { unregister_trace_x86_platform_ipi_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_irq_work_exit(); return -EINVAL; }
    ret = register_trace_error_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
    if ret != 0 { goto_x86_ipi_exit(); return -EINVAL; }
    ret = register_trace_error_apic_exit(Some(trace_intel_irq_exit), b"error_apic\0".as_ptr() as *const c_char);
    if ret != 0 { unregister_trace_error_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_x86_ipi_exit(); return -EINVAL; }
    ret = register_trace_spurious_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
    if ret != 0 { goto_error_apic_exit(); return -EINVAL; }
    ret = register_trace_spurious_apic_exit(Some(trace_intel_irq_exit), b"spurious_apic\0".as_ptr() as *const c_char);
    if ret != 0 { unregister_trace_spurious_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); goto_error_apic_exit(); return -EINVAL; }
    0
}

// The C error labels are represented by the same ordered cleanup operations.
// These helpers are declaration-level placeholders for the surrounding kernel bindings.
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn osnoise_arch_register_out_err() -> c_int { -EINVAL }
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_timer_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_thermal_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_deferred_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_threshold_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_call_function_single_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_call_function_entry() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_call_function_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_reschedule_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_irq_work_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_x86_ipi_exit() {}
#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
unsafe fn goto_error_apic_exit() {}

#[cfg(all(CONFIG_OSNOISE_TRACER, CONFIG_X86_LOCAL_APIC))]
pub unsafe fn osnoise_arch_unregister() {
    unregister_trace_spurious_apic_exit(Some(trace_intel_irq_exit), b"spurious_apic\0".as_ptr() as *const c_char);
    unregister_trace_spurious_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
    unregister_trace_error_apic_exit(Some(trace_intel_irq_exit), b"error_apic\0".as_ptr() as *const c_char);
    unregister_trace_error_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
    unregister_trace_x86_platform_ipi_exit(Some(trace_intel_irq_exit), b"x86_platform_ipi\0".as_ptr() as *const c_char);
    unregister_trace_x86_platform_ipi_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
    #[cfg(CONFIG_IRQ_WORK)] { unregister_trace_irq_work_exit(Some(trace_intel_irq_exit), b"irq_work\0".as_ptr() as *const c_char); unregister_trace_irq_work_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); }
    #[cfg(CONFIG_SMP)] { unregister_trace_reschedule_exit(Some(trace_intel_irq_exit), b"reschedule\0".as_ptr() as *const c_char); unregister_trace_reschedule_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); unregister_trace_call_function_exit(Some(trace_intel_irq_exit), b"call_function\0".as_ptr() as *const c_char); unregister_trace_call_function_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); unregister_trace_call_function_single_exit(Some(trace_intel_irq_exit), b"call_function_single\0".as_ptr() as *const c_char); unregister_trace_call_function_single_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); }
    #[cfg(CONFIG_X86_MCE_THRESHOLD)] { unregister_trace_threshold_apic_exit(Some(trace_intel_irq_exit), b"threshold_apic\0".as_ptr() as *const c_char); unregister_trace_threshold_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); }
    #[cfg(CONFIG_X86_MCE_AMD)] { unregister_trace_deferred_error_apic_exit(Some(trace_intel_irq_exit), b"deferred_error\0".as_ptr() as *const c_char); unregister_trace_deferred_error_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); }
    #[cfg(CONFIG_X86_THERMAL_VECTOR)] { unregister_trace_thermal_apic_exit(Some(trace_intel_irq_exit), b"thermal_apic\0".as_ptr() as *const c_char); unregister_trace_thermal_apic_entry(Some(trace_intel_irq_entry), core::ptr::null_mut()); }
    unregister_trace_local_timer_exit(Some(trace_intel_irq_exit), b"local_timer\0".as_ptr() as *const c_char);
    unregister_trace_local_timer_entry(Some(trace_intel_irq_entry), core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
