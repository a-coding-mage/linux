// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by other translation units.

static mut apic_use_ipi_shorthand: bool = false;

#[cfg(feature = "CONFIG_SMP")]
static mut apic_ipi_shorthand_off: i32 = 0;

#[cfg(feature = "CONFIG_SMP")]
unsafe fn apic_ipi_shorthand(mut str_: *mut core::ffi::c_char) -> i32 {
    get_option(&mut str_, &mut apic_ipi_shorthand_off);
    1
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn print_ipi_mode() -> i32 {
    pr_info!("IPI shorthand broadcast: {}\n", str_disabled_enabled(apic_ipi_shorthand_off));
    0
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn apic_smt_update() {
    /*
     * Do not switch to broadcast mode if:
     * - Disabled on the command line
     * - Only a single CPU is online
     * - Not all present CPUs have been at least booted once
     *
     * The latter is important as the local APIC might be in some
     * random state and a broadcast might cause havoc. That's
     * especially true for NMI broadcasting.
     */
    if apic_ipi_shorthand_off != 0 || num_online_cpus() == 1 ||
        !cpumask_equal(cpu_present_mask, &cpus_booted_once_mask) {
        static_branch_disable(&mut apic_use_ipi_shorthand);
    } else {
        static_branch_enable(&mut apic_use_ipi_shorthand);
    }
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn apic_send_IPI_allbutself(vector: u32) {
    if num_online_cpus() < 2 {
        return;
    }
    if static_branch_likely(&apic_use_ipi_shorthand) {
        __apic_send_IPI_allbutself(vector);
    } else {
        __apic_send_IPI_mask_allbutself(cpu_online_mask, vector);
    }
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn native_smp_send_reschedule(cpu: i32) {
    if unlikely(cpu_is_offline(cpu)) {
        WARN!(1, "sched: Unexpected reschedule of offline CPU#%d!\n", cpu);
        return;
    }
    __apic_send_IPI(cpu, RESCHEDULE_VECTOR);
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn native_send_call_func_single_ipi(cpu: i32) {
    __apic_send_IPI(cpu, CALL_FUNCTION_SINGLE_VECTOR);
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn native_send_call_func_ipi(mask: *const cpumask) {
    if static_branch_likely(&apic_use_ipi_shorthand) {
        let cpu = smp_processor_id();
        if !cpumask_or_equal(mask, cpumask_of(cpu), cpu_online_mask) {
            __apic_send_IPI_mask(mask, CALL_FUNCTION_VECTOR);
            return;
        }
        if cpumask_test_cpu(cpu, mask) {
            __apic_send_IPI_all(CALL_FUNCTION_VECTOR);
        } else if num_online_cpus() > 1 {
            __apic_send_IPI_allbutself(CALL_FUNCTION_VECTOR);
        }
        return;
    }
    __apic_send_IPI_mask(mask, CALL_FUNCTION_VECTOR);
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn apic_send_nmi_to_offline_cpu(cpu: u32) {
    if WARN_ON_ONCE(!(*apic).nmi_to_offline_cpu) {
        return;
    }
    if WARN_ON_ONCE(!cpumask_test_cpu(cpu, &cpus_booted_once_mask)) {
        return;
    }
    ((*apic).send_IPI)(cpu, NMI_VECTOR);
}

unsafe fn __prepare_ICR2(mask: u32) -> i32 {
    SET_XAPIC_DEST_FIELD(mask)
}

unsafe fn apic_mem_wait_icr_idle_timeout() -> u32 {
    let mut cnt = 0;
    while cnt < 1000 {
        if (apic_read(APIC_ICR) & APIC_ICR_BUSY) == 0 {
            return 0;
        }
        irq_stat_inc_and_enable(IRQ_COUNT_ICR_READ_RETRY);
        udelay(100);
        cnt += 1;
    }
    APIC_ICR_BUSY
}

unsafe fn apic_mem_wait_icr_idle() {
    while native_apic_mem_read(APIC_ICR) & APIC_ICR_BUSY != 0 {
        cpu_relax();
    }
}

unsafe fn __default_send_IPI_shortcut(shortcut: u32, vector: i32) {
    if unlikely(vector == NMI_VECTOR) {
        apic_mem_wait_icr_idle_timeout();
    } else {
        apic_mem_wait_icr_idle();
    }
    native_apic_mem_write(APIC_ICR, __prepare_ICR(shortcut, vector, 0));
}

unsafe fn __default_send_IPI_dest_field(dest_mask: u32, vector: i32, dest_mode: u32) {
    if unlikely(vector == NMI_VECTOR) {
        apic_mem_wait_icr_idle_timeout();
    } else {
        apic_mem_wait_icr_idle();
    }
    native_apic_mem_write(APIC_ICR2, __prepare_ICR2(dest_mask));
    native_apic_mem_write(APIC_ICR, __prepare_ICR(0, vector, dest_mode));
}

unsafe fn default_send_IPI_single_phys(cpu: i32, vector: i32) {
    let mut flags = 0;
    local_irq_save(&mut flags);
    __default_send_IPI_dest_field(per_cpu(x86_cpu_to_apicid, cpu), vector, APIC_DEST_PHYSICAL);
    local_irq_restore(flags);
}

unsafe fn default_send_IPI_mask_sequence_phys(mask: *const cpumask, vector: i32) {
    let mut flags = 0;
    local_irq_save(&mut flags);
    for_each_cpu!(cpu, mask, {
        __default_send_IPI_dest_field(per_cpu(x86_cpu_to_apicid, cpu), vector, APIC_DEST_PHYSICAL);
    });
    local_irq_restore(flags);
}

unsafe fn default_send_IPI_mask_allbutself_phys(mask: *const cpumask, vector: i32) {
    let this_cpu = smp_processor_id();
    let mut flags = 0;
    local_irq_save(&mut flags);
    for_each_cpu!(cpu, mask, {
        if cpu == this_cpu { continue; }
        __default_send_IPI_dest_field(per_cpu(x86_cpu_to_apicid, cpu), vector, APIC_DEST_PHYSICAL);
    });
    local_irq_restore(flags);
}

unsafe fn default_send_IPI_single(cpu: i32, vector: i32) {
    __apic_send_IPI_mask(cpumask_of(cpu), vector);
}

unsafe fn default_send_IPI_allbutself(vector: i32) {
    __default_send_IPI_shortcut(APIC_DEST_ALLBUT, vector);
}

unsafe fn default_send_IPI_all(vector: i32) {
    __default_send_IPI_shortcut(APIC_DEST_ALLINC, vector);
}

unsafe fn default_send_IPI_self(vector: i32) {
    __default_send_IPI_shortcut(APIC_DEST_SELF, vector);
}

#[cfg(feature = "CONFIG_X86_32")]
unsafe fn default_send_IPI_mask_sequence_logical(mask: *const cpumask, vector: i32) {
    let mut flags = 0;
    local_irq_save(&mut flags);
    for_each_cpu!(cpu, mask, {
        __default_send_IPI_dest_field(1u32 << cpu, vector, APIC_DEST_LOGICAL);
    });
    local_irq_restore(flags);
}

#[cfg(feature = "CONFIG_X86_32")]
unsafe fn default_send_IPI_mask_allbutself_logical(mask: *const cpumask, vector: i32) {
    let this_cpu = smp_processor_id();
    let mut flags = 0;
    local_irq_save(&mut flags);
    for_each_cpu!(cpu, mask, {
        if cpu == this_cpu { continue; }
        __default_send_IPI_dest_field(1u32 << cpu, vector, APIC_DEST_LOGICAL);
    });
    local_irq_restore(flags);
}

#[cfg(feature = "CONFIG_X86_32")]
unsafe fn default_send_IPI_mask_logical(cpumask: *const cpumask, vector: i32) {
    let mask = cpumask_bits(cpumask)[0];
    let mut flags = 0;
    if mask == 0 { return; }
    local_irq_save(&mut flags);
    WARN_ON!(mask & !cpumask_bits(cpu_online_mask)[0]);
    __default_send_IPI_dest_field(mask, vector, APIC_DEST_LOGICAL);
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
