// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Intel SMP support routines.
 *
 * Translated from smp.c. Linux kernel declarations supplied by included
 * headers are intentionally left as external dependencies.
 */

static mut stopping_cpu: atomic_t = ATOMIC_INIT(-1);
static mut smp_no_nmi_ipi: bool = false;

unsafe extern "C" fn smp_stop_nmi_callback(val: ::core::ffi::c_uint,
                                            regs: *mut pt_regs) -> ::core::ffi::c_int {
    // We are registered on stopping cpu too, avoid spurious NMI
    if raw_smp_processor_id() == atomic_read(&stopping_cpu) {
        return NMI_HANDLED;
    }

    x86_virt_emergency_disable_virtualization_cpu();
    stop_this_cpu(::core::ptr::null_mut());

    NMI_HANDLED
}

/* this function calls the 'stop' function on all other CPUs in the system. */
unsafe extern "C" fn sysvec_reboot() {
    apic_eoi();
    x86_virt_emergency_disable_virtualization_cpu();
    stop_this_cpu(::core::ptr::null_mut());
}

unsafe fn register_stop_handler() -> ::core::ffi::c_int {
    register_nmi_handler(NMI_LOCAL, Some(smp_stop_nmi_callback), NMI_FLAG_FIRST, c"smp_stop".as_ptr())
}

unsafe fn native_stop_other_cpus(wait: ::core::ffi::c_int) {
    let mut old_cpu: ::core::ffi::c_uint;
    let this_cpu: ::core::ffi::c_uint;
    let mut flags: ::core::ffi::c_ulong;
    let mut timeout: ::core::ffi::c_ulong;

    if reboot_force {
        return;
    }

    // Only proceed if this is the first CPU to reach this code
    old_cpu = (-1i32) as ::core::ffi::c_uint;
    this_cpu = smp_processor_id();
    if !atomic_try_cmpxchg(&mut stopping_cpu, &mut old_cpu, this_cpu) {
        return;
    }

    // For kexec, ensure that offline CPUs are out of MWAIT and in HLT
    if kexec_in_progress {
        smp_kick_mwait_play_dead();
    }

    cpumask_copy(&mut cpus_stop_mask, cpu_online_mask);
    cpumask_clear_cpu(this_cpu, &mut cpus_stop_mask);

    if !cpumask_empty(&cpus_stop_mask) {
        apic_send_IPI_allbutself(REBOOT_VECTOR);

        // Don't wait longer than a second for IPI completion.
        timeout = USEC_PER_SEC;
        while !cpumask_empty(&cpus_stop_mask) && timeout != 0 {
            timeout = timeout.wrapping_sub(1);
            udelay(1);
        }
    }

    // If the REBOOT_VECTOR didn't work, try with the NMI.
    if !cpumask_empty(&cpus_stop_mask) {
        if !smp_no_nmi_ipi && register_stop_handler() == 0 {
            let mut cpu: ::core::ffi::c_uint;

            pr_emerg!(c"Shutting down cpus with NMI");

            for_each_cpu!(cpu, &cpus_stop_mask) {
                __apic_send_IPI(cpu, NMI_VECTOR);
            }
        }

        timeout = USEC_PER_MSEC * 10;
        while !cpumask_empty(&cpus_stop_mask) && (wait != 0 || timeout != 0) {
            if timeout != 0 {
                timeout = timeout.wrapping_sub(1);
            }
            udelay(1);
        }
    }

    local_irq_save(&mut flags);
    disable_local_APIC();
    mcheck_cpu_clear(this_cpu_ptr(&mut cpu_info));
    local_irq_restore(flags);

    // Ensure that the cpus_stop_mask cache lines are invalidated on the other CPUs.
    cpumask_clear(&mut cpus_stop_mask);
}

/* Reschedule callback. KVM uses this interrupt to force a cpu out of guest mode. */
unsafe extern "C" fn sysvec_reschedule_ipi() {
    apic_eoi();
    trace_reschedule_entry(RESCHEDULE_VECTOR);
    inc_irq_stat(RESCHEDULE);
    scheduler_ipi();
    trace_reschedule_exit(RESCHEDULE_VECTOR);
}

unsafe extern "C" fn sysvec_call_function() {
    apic_eoi();
    trace_call_function_entry(CALL_FUNCTION_VECTOR);
    inc_irq_stat(CALL_FUNCTION);
    generic_smp_call_function_interrupt();
    trace_call_function_exit(CALL_FUNCTION_VECTOR);
}

unsafe extern "C" fn sysvec_call_function_single() {
    apic_eoi();
    trace_call_function_single_entry(CALL_FUNCTION_SINGLE_VECTOR);
    inc_irq_stat(CALL_FUNCTION);
    generic_smp_call_function_single_interrupt();
    trace_call_function_single_exit(CALL_FUNCTION_SINGLE_VECTOR);
}

unsafe extern "C" fn nonmi_ipi_setup(_str: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    smp_no_nmi_ipi = true;
    1
}

// __setup("nonmi_ipi", nonmi_ipi_setup);

static mut smp_ops: struct_smp_ops = struct_smp_ops {
    smp_prepare_boot_cpu: Some(native_smp_prepare_boot_cpu),
    smp_prepare_cpus: Some(native_smp_prepare_cpus),
    smp_cpus_done: Some(native_smp_cpus_done),
    stop_other_cpus: Some(native_stop_other_cpus),
    // CONFIG_CRASH_DUMP: crash_stop_other_cpus = Some(kdump_nmi_shootdown_cpus),
    smp_send_reschedule: Some(native_smp_send_reschedule),
    kick_ap_alive: Some(native_kick_ap),
    cpu_disable: Some(native_cpu_disable),
    play_dead: Some(native_play_dead),
    send_call_func_ipi: Some(native_send_call_func_ipi),
    send_call_func_single_ipi: Some(native_send_call_func_single_ipi),
};

pub unsafe extern "C" fn arch_cpu_rescan_dead_smt_siblings() -> ::core::ffi::c_int {
    let old: cpuhp_smt_control = cpu_smt_control;
    let mut ret: ::core::ffi::c_int;

    /*
     * If SMT has been disabled and SMT siblings are in HLT, bring them back
     * online and offline them again so that they end up in MWAIT proper.
     *
     * Called with hotplug enabled.
     */
    if old != CPU_SMT_DISABLED && old != CPU_SMT_FORCE_DISABLED {
        return 0;
    }

    ret = cpuhp_smt_enable();
    if ret != 0 {
        return ret;
    }

    ret = cpuhp_smt_disable(old);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
