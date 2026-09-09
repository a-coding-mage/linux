// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SMP support for PowerNV machines.
 *
 * Copyright 2011 IBM Corp.
 */

// Linux and PowerPC dependencies supplied by other translation units.

#[allow(non_camel_case_types)]
pub unsafe fn pnv_smp_setup_cpu(cpu: i32) {
    /* P9 workaround for CI vector load (see traps.c), enable the corresponding HMI interrupt. */
    if pvr_version_is(PVR_POWER9) {
        mtspr(SPRN_HMEER, mfspr(SPRN_HMEER) | PPC_BIT(17));
    }
    if xive_enabled() {
        xive_smp_setup_cpu();
    } else if cpu != boot_cpuid {
        xics_setup_cpu();
    }
}

pub unsafe fn pnv_smp_kick_cpu(nr: i32) -> i32 {
    let pcpu: u32;
    let start_here: usize = __pa(ppc_function_entry(generic_secondary_smp_init));
    let mut rc: i64;
    let mut status: u8 = 0;

    if nr < 0 || nr >= nr_cpu_ids { return -EINVAL; }
    pcpu = get_hard_smp_processor_id(nr);
    if (*paca_ptrs[nr as usize]).cpu_start != 0 || !firmware_has_feature(FW_FEATURE_OPAL) { return smp_generic_kick_cpu(nr); }
    rc = opal_query_cpu_status(pcpu, &mut status);
    if rc != OPAL_SUCCESS {
        pr_warn("OPAL Error %ld querying CPU %d state\n", rc, nr);
        return -ENODEV;
    }
    if status == OPAL_THREAD_STARTED { return smp_generic_kick_cpu(nr); }
    if status == OPAL_THREAD_INACTIVE {
        pr_devel!("OPAL: Starting CPU %d (HW 0x%x)...\n", nr, pcpu);
        rc = opal_start_cpu(pcpu, start_here);
        if rc != OPAL_SUCCESS {
            pr_warn("OPAL Error %ld starting CPU %d\n", rc, nr);
            return -ENODEV;
        }
    } else {
        pr_devel!("OPAL: CPU %d (HW 0x%x) is unavailable (status %d)...\n", nr, pcpu, status);
        return -ENODEV;
    }
    smp_generic_kick_cpu(nr)
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn pnv_smp_cpu_disable() -> i32 {
    let cpu = smp_processor_id();
    set_cpu_online(cpu, false);
    #[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)] { (*systemcfg).processorCount -= 1; }
    if cpu == boot_cpuid { boot_cpuid = cpumask_any(cpu_online_mask); }
    if xive_enabled() { xive_smp_disable_cpu(); } else { xics_migrate_irqs_away(); }
    cleanup_cpu_mmu_context();
    0
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn pnv_flush_interrupts() {
    if cpu_has_feature(CPU_FTR_ARCH_300) {
        if xive_enabled() { xive_flush_interrupt(); } else { icp_opal_flush_interrupt(); }
    } else { icp_native_flush_interrupt(); }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn pnv_cpu_offline_self() {
    let mut srr1: usize;
    let mut unexpected_mask: usize;
    let mut wmask = SRR1_WAKEMASK;
    let cpu = smp_processor_id();
    let mut lpcr_val: u64;
    idle_task_exit();
    DBG!("CPU%d offline\n", cpu);
    generic_set_cpu_dead(cpu); smp_wmb();
    if cpu_has_feature(CPU_FTR_ARCH_207S) { wmask = SRR1_WAKEMASK_P8; }
    hard_irq_disable();
    if generic_check_cpu_restart(cpu) { return; }
    unexpected_mask = !(PACA_IRQ_DEC | PACA_IRQ_HMI | PACA_IRQ_HARD_DIS);
    if local_paca.irq_happened & unexpected_mask != 0 {
        if local_paca.irq_happened & PACA_IRQ_EE != 0 { pnv_flush_interrupts(); }
        DBG!("CPU%d Unexpected exit while offline irq_happened=%lx!\n", cpu, local_paca.irq_happened);
    }
    local_paca.irq_happened = PACA_IRQ_HARD_DIS;
    lpcr_val = mfspr(SPRN_LPCR) & !(LPCR_PECE1 as u64);
    pnv_program_cpu_hotplug_lpcr(cpu, lpcr_val);
    while !generic_check_cpu_restart(cpu) {
        kvmppc_clear_host_ipi(cpu);
        srr1 = pnv_cpu_offline(cpu);
        WARN_ON_ONCE(!irqs_disabled()); WARN_ON(lazy_irq_pending());
        if (srr1 & wmask) == SRR1_WAKEEE || (srr1 & wmask) == SRR1_WAKEHVI { pnv_flush_interrupts(); }
        else if (srr1 & wmask) == SRR1_WAKEHDBELL { let msg = PPC_DBELL_TYPE(PPC_DBELL_SERVER); asm!(PPC_MSGCLR!("{0}"), in(reg) msg); }
        else if (srr1 & wmask) == SRR1_WAKERESET { irq_set_pending_from_srr1(srr1); }
        smp_mb();
        if kdump_in_progress() {
            let mut regs = pt_regs { /* supplied by the kernel */ };
            ppc_save_regs(&mut regs); crash_ipi_callback(&mut regs);
        }
        if cpu_core_split_required() { continue; }
        if srr1 != 0 && !generic_check_cpu_restart(cpu) { DBG!("CPU%d Unexpected exit while offline srr1=%lx!\n", cpu, srr1); }
    }
    lpcr_val = mfspr(SPRN_LPCR) | LPCR_PECE1 as u64;
    pnv_program_cpu_hotplug_lpcr(cpu, lpcr_val);
    DBG!("CPU%d coming online...\n", cpu);
}

pub unsafe fn pnv_cpu_bootable(nr: u32) -> i32 { if cpu_has_feature(CPU_FTR_ARCH_207S) { 1 } else { smp_generic_cpu_bootable(nr) } }
pub unsafe fn pnv_smp_prepare_cpu(cpu: i32) -> i32 { if xive_enabled() { xive_smp_prepare_cpu(cpu) } else { 0 } }

static mut ic_cause_ipi: Option<unsafe extern "C" fn(i32)> = None;
pub unsafe fn pnv_cause_ipi(cpu: i32) { if doorbell_try_core_ipi(cpu) { return; } (ic_cause_ipi.unwrap())(cpu); }

pub unsafe fn pnv_smp_probe() {
    if xive_enabled() { if xive_smp_probe() < 0 { return; } } else { xics_smp_probe(); }
    if cpu_has_feature(CPU_FTR_DBELL) {
        ic_cause_ipi = Some((*smp_ops).cause_ipi.unwrap());
        WARN_ON(ic_cause_ipi.is_none());
        (*smp_ops).cause_ipi = Some(if cpu_has_feature(CPU_FTR_ARCH_300) { doorbell_global_ipi } else { pnv_cause_ipi });
    }
}

pub unsafe fn pnv_system_reset_exception(regs: *mut pt_regs) -> i32 { if smp_handle_nmi_ipi(regs) { 1 } else { 0 } }

pub unsafe fn pnv_cause_nmi_ipi(cpu: i32) -> i32 {
    let mut rc: i64;
    if cpu >= 0 {
        let h = get_hard_smp_processor_id(cpu);
        if opal_check_token(OPAL_QUIESCE) { opal_quiesce(QUIESCE_HOLD, h); }
        rc = opal_signal_system_reset(h);
        if opal_check_token(OPAL_QUIESCE) { opal_quiesce(QUIESCE_RESUME, h); }
        return if rc == OPAL_SUCCESS { 1 } else { 0 };
    } else if cpu == NMI_IPI_ALL_OTHERS {
        let mut success = true;
        if opal_check_token(OPAL_QUIESCE) { opal_quiesce(QUIESCE_HOLD, -1); }
        for_each_online_cpu!(c, { if c != smp_processor_id() { rc = opal_signal_system_reset(get_hard_smp_processor_id(c)); if rc != OPAL_SUCCESS { success = false; } } });
        if opal_check_token(OPAL_QUIESCE) { opal_quiesce(QUIESCE_RESUME, -1); }
        if success { return 1; }
    }
    0
}

#[repr(C)]
pub struct smp_ops_t {
    pub message_pass: Option<unsafe extern "C" fn()>,
    pub cause_ipi: Option<unsafe extern "C" fn(i32)>,
    pub cause_nmi_ipi: Option<unsafe extern "C" fn(i32) -> i32>,
    pub probe: Option<unsafe extern "C" fn()>,
    pub prepare_cpu: Option<unsafe extern "C" fn(i32) -> i32>,
    pub kick_cpu: Option<unsafe extern "C" fn(i32) -> i32>,
    pub setup_cpu: Option<unsafe extern "C" fn(i32)>,
    pub cpu_bootable: Option<unsafe extern "C" fn(u32) -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)] pub cpu_disable: Option<unsafe extern "C" fn() -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)] pub cpu_die: Option<unsafe extern "C" fn(i32)>,
    #[cfg(CONFIG_HOTPLUG_CPU)] pub cpu_offline_self: Option<unsafe extern "C" fn()>,
}

static mut pnv_smp_ops: smp_ops_t = smp_ops_t {
    message_pass: None,
    cause_ipi: None,
    cause_nmi_ipi: None,
    probe: Some(pnv_smp_probe),
    prepare_cpu: Some(pnv_smp_prepare_cpu),
    kick_cpu: Some(pnv_smp_kick_cpu),
    setup_cpu: Some(pnv_smp_setup_cpu),
    cpu_bootable: Some(pnv_cpu_bootable),
    #[cfg(CONFIG_HOTPLUG_CPU)] cpu_disable: Some(pnv_smp_cpu_disable),
    #[cfg(CONFIG_HOTPLUG_CPU)] cpu_die: Some(generic_cpu_die),
    #[cfg(CONFIG_HOTPLUG_CPU)] cpu_offline_self: Some(pnv_cpu_offline_self),
};

pub unsafe fn pnv_smp_init() {
    if opal_check_token(OPAL_SIGNAL_SYSTEM_RESET) {
        (*ppc_md).system_reset_exception = Some(pnv_system_reset_exception);
        pnv_smp_ops.cause_nmi_ipi = Some(pnv_cause_nmi_ipi);
    }
    smp_ops = &mut pnv_smp_ops;
    #[cfg(CONFIG_CRASH_DUMP)] { crash_wake_offline = 1; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
