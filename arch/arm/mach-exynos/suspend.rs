// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2011-2014 Samsung Electronics Co., Ltd.
// Exynos - Suspend support

// Dependencies are supplied by the surrounding kernel translation.

const REG_TABLE_END: u32 = (-1i32) as u32;
const EXYNOS5420_CPU_STATE: usize = 0x28;

#[repr(C)]
struct exynos_wkup_irq { hwirq: u32, mask: u32 }

#[repr(C)]
struct exynos_pm_data {
    wkup_irq: *const exynos_wkup_irq,
    wake_disable_mask: u32,
    pm_prepare: Option<unsafe extern "C" fn()>,
    pm_resume_prepare: Option<unsafe extern "C" fn()>,
    cpu_suspend: Option<unsafe extern "C" fn(usize) -> i32>,
    syscore_ops: *const syscore_ops,
}

#[repr(C)]
struct exynos_pm_state {
    cpu_state: i32,
    pmu_spare3: u32,
    sysram_base: *mut u8,
    sysram_phys: usize,
    secure_firmware: bool,
}

static mut pm_data: *const exynos_pm_data = core::ptr::null();
static mut pm_state: exynos_pm_state = exynos_pm_state { cpu_state: 0, pmu_spare3: 0, sysram_base: core::ptr::null_mut(), sysram_phys: 0, secure_firmware: false };
static mut exynos_irqwake_intmask: u32 = 0xffff_ffff;

static exynos3250_wkup_irq: [exynos_wkup_irq; 3] = [exynos_wkup_irq { hwirq: 73, mask: 1 << 1 }, exynos_wkup_irq { hwirq: 74, mask: 1 << 2 }, exynos_wkup_irq { hwirq: 0, mask: 0 }];
static exynos4_wkup_irq: [exynos_wkup_irq; 3] = [exynos_wkup_irq { hwirq: 44, mask: 1 << 1 }, exynos_wkup_irq { hwirq: 45, mask: 1 << 2 }, exynos_wkup_irq { hwirq: 0, mask: 0 }];
static exynos5250_wkup_irq: [exynos_wkup_irq; 3] = [exynos_wkup_irq { hwirq: 43, mask: 1 << 1 }, exynos_wkup_irq { hwirq: 44, mask: 1 << 2 }, exynos_wkup_irq { hwirq: 0, mask: 0 }];

unsafe fn exynos_read_eint_wakeup_mask() -> u32 { pmu_raw_readl(EXYNOS_EINT_WAKEUP_MASK) }

unsafe fn exynos_irq_set_wake(data: *mut irq_data, state: u32) -> i32 {
    if pm_data.is_null() || (*pm_data).wkup_irq.is_null() { return -ENOENT; }
    let mut irq = (*pm_data).wkup_irq;
    while (*irq).mask != 0 {
        if (*irq).hwirq == (*data).hwirq {
            if state == 0 { exynos_irqwake_intmask |= (*irq).mask; }
            else { exynos_irqwake_intmask &= !(*irq).mask; }
            return 0;
        }
        irq = irq.add(1);
    }
    -ENOENT
}

unsafe fn exynos_pmu_domain_translate(_d: *mut irq_domain, f: *mut irq_fwspec, h: *mut usize, t: *mut u32) -> i32 {
    if !is_of_node((*f).fwnode) || (*f).param_count != 3 || (*f).param[0] != 0 { return -EINVAL; }
    *h = (*f).param[1] as usize; *t = (*f).param[2]; 0
}

unsafe fn exynos_pmu_domain_alloc(domain: *mut irq_domain, virq: u32, nr_irqs: u32, data: *mut core::ffi::c_void) -> i32 {
    let f = data as *mut irq_fwspec;
    if (*f).param_count != 3 || (*f).param[0] != 0 { return -EINVAL; }
    let h = (*f).param[1] as usize;
    for i in 0..nr_irqs { irq_domain_set_hwirq_and_chip(domain, virq + i, h + i as usize, &exynos_pmu_chip, core::ptr::null_mut()); }
    let mut parent = *f; parent.fwnode = (*domain).parent.fwnode;
    irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &mut parent as *mut _ as *mut core::ffi::c_void)
}

unsafe fn exynos_pmu_irq_init(node: *mut device_node, parent: *mut device_node) -> i32 {
    if parent.is_null() { pr_err!("%pOF: no parent, giving up\n", node); return -ENODEV; }
    let pd = irq_find_host(parent); if pd.is_null() { pr_err!("%pOF: unable to obtain parent domain\n", node); return -ENXIO; }
    pmu_base_addr = of_iomap(node, 0); if pmu_base_addr.is_null() { pr_err!("%pOF: failed to find exynos pmu register\n", node); return -ENOMEM; }
    let d = irq_domain_create_hierarchy(pd, 0, 0, of_fwnode_handle(node), &exynos_pmu_domain_ops, core::ptr::null_mut());
    if d.is_null() { iounmap(pmu_base_addr); pmu_base_addr = core::ptr::null_mut(); return -ENOMEM; }
    of_node_clear_flag(node, OF_POPULATED); 0
}

unsafe fn exynos_cpu_do_idle() -> i32 { cpu_do_idle(); pr_info!("Failed to suspend the system\n"); 1 }
unsafe fn exynos_flush_cache_all() { flush_cache_all(); outer_flush_all(); }
unsafe fn exynos_cpu_suspend(_arg: usize) -> i32 { exynos_flush_cache_all(); exynos_cpu_do_idle() }
unsafe fn exynos3250_cpu_suspend(_arg: usize) -> i32 { flush_cache_all(); exynos_cpu_do_idle() }
unsafe fn exynos5420_cpu_suspend(_arg: usize) -> i32 {
    let mpidr = read_cpuid_mpidr(); let cluster = MPIDR_AFFINITY_LEVEL(mpidr, 1); let cpu = MPIDR_AFFINITY_LEVEL(mpidr, 0);
    if IS_ENABLED(CONFIG_EXYNOS_MCPM) { mcpm_set_entry_vector(cpu, cluster, exynos_cpu_resume); mcpm_cpu_suspend(); }
    pr_info!("Failed to suspend the system\n"); 1
}

unsafe fn exynos_pm_set_wakeup_mask() { pmu_raw_writel(exynos_irqwake_intmask & !(1 << 31), S5P_WAKEUP_MASK); }
unsafe fn exynos_pm_enter_sleep_mode() { exynos_sys_powerdown_conf(SYS_SLEEP); pmu_raw_writel(EXYNOS_SLEEP_MAGIC, S5P_INFORM1); }
unsafe fn exynos_pm_prepare() { exynos_set_delayed_reset_assertion(false); exynos_pm_set_wakeup_mask(); exynos_pm_enter_sleep_mode(); pmu_raw_writel(__pa_symbol(exynos_cpu_resume), S5P_INFORM0); }
unsafe fn exynos3250_pm_prepare() { exynos_pm_set_wakeup_mask(); let mut t = pmu_raw_readl(EXYNOS3_ARM_L2_OPTION); t &= !EXYNOS5_OPTION_USE_RETENTION; pmu_raw_writel(t, EXYNOS3_ARM_L2_OPTION); exynos_pm_enter_sleep_mode(); pmu_raw_writel(__pa_symbol(exynos_cpu_resume), S5P_INFORM0); }

unsafe fn exynos_pm_suspend(_data: *mut core::ffi::c_void) -> i32 { exynos_pm_central_suspend(); pmu_raw_writel(S5P_USE_STANDBY_WFI0 | S5P_USE_STANDBY_WFE0, S5P_CENTRAL_SEQ_OPTION); if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 { exynos_cpu_save_register(); } 0 }
unsafe fn exynos_pm_resume(_data: *mut core::ffi::c_void) { let cpuid = read_cpuid_part(); if exynos_pm_central_resume() { pmu_raw_writel(0, S5P_INFORM1); exynos_set_delayed_reset_assertion(true); return; } if cpuid == ARM_CPU_PART_CORTEX_A9 { exynos_scu_enable(); } if call_firmware_op(resume) == -ENOSYS && cpuid == ARM_CPU_PART_CORTEX_A9 { exynos_cpu_restore_register(); } pmu_raw_writel(0, S5P_INFORM1); exynos_set_delayed_reset_assertion(true); }

unsafe fn exynos5420_pm_prepare() {
    exynos_pm_set_wakeup_mask(); pm_state.pmu_spare3 = pmu_raw_readl(S5P_PMU_SPARE3);
    pm_state.cpu_state = readl_relaxed(pm_state.sysram_base.add(EXYNOS5420_CPU_STATE));
    writel_relaxed(0, pm_state.sysram_base.add(EXYNOS5420_CPU_STATE));
    if pm_state.secure_firmware { exynos_smc(SMC_CMD_REG, SMC_REG_ID_SFR_W(pm_state.sysram_phys + EXYNOS5420_CPU_STATE), 0, 0); }
    exynos_pm_enter_sleep_mode(); if IS_ENABLED(CONFIG_EXYNOS_MCPM) { pmu_raw_writel(__pa_symbol(mcpm_entry_point), S5P_INFORM0); }
    let mut t = pmu_raw_readl(EXYNOS_L2_OPTION(0)); t &= !EXYNOS_L2_USE_RETENTION; pmu_raw_writel(t, EXYNOS_L2_OPTION(0));
    t = pmu_raw_readl(EXYNOS5420_SFR_AXI_CGDIS1); t |= EXYNOS5420_UFS; pmu_raw_writel(t, EXYNOS5420_SFR_AXI_CGDIS1);
    t = pmu_raw_readl(EXYNOS5420_ARM_COMMON_OPTION); t &= !EXYNOS5420_L2RSTDISABLE_VALUE; pmu_raw_writel(t, EXYNOS5420_ARM_COMMON_OPTION);
    t = pmu_raw_readl(EXYNOS5420_FSYS2_OPTION); t |= EXYNOS5420_EMULATION; pmu_raw_writel(t, EXYNOS5420_FSYS2_OPTION);
    t = pmu_raw_readl(EXYNOS5420_PSGEN_OPTION); t |= EXYNOS5420_EMULATION; pmu_raw_writel(t, EXYNOS5420_PSGEN_OPTION);
}
unsafe fn exynos5420_pm_suspend(_d: *mut core::ffi::c_void) -> i32 { exynos_pm_central_suspend(); let c = MPIDR_AFFINITY_LEVEL(read_cpuid_mpidr(), 1); pmu_raw_writel(if c == 0 { EXYNOS5420_ARM_USE_STANDBY_WFI0 } else { EXYNOS5420_KFC_USE_STANDBY_WFI0 }, S5P_CENTRAL_SEQ_OPTION); 0 }
unsafe fn exynos3250_pm_resume(_d: *mut core::ffi::c_void) { let c=read_cpuid_part(); if exynos_pm_central_resume()==0 { pmu_raw_writel(S5P_USE_STANDBY_WFI_ALL,S5P_CENTRAL_SEQ_OPTION); if call_firmware_op(resume)==-ENOSYS && c==ARM_CPU_PART_CORTEX_A9 { exynos_cpu_restore_register(); } } pmu_raw_writel(0,S5P_INFORM1); }
unsafe fn exynos5420_prepare_pm_resume() { let c=MPIDR_AFFINITY_LEVEL(read_cpuid_mpidr(),1); if IS_ENABLED(CONFIG_EXYNOS_MCPM) { WARN_ON(mcpm_cpu_powered_up()); } if IS_ENABLED(CONFIG_HW_PERF_EVENTS) && c!=0 { pmu_raw_writel(S5P_CORE_LOCAL_PWR_EN,EXYNOS_COMMON_CONFIGURATION(0)); pmu_raw_writel(0,EXYNOS_COMMON_CONFIGURATION(0)); } }
unsafe fn exynos5420_pm_resume(_d: *mut core::ffi::c_void) { let mut t=pmu_raw_readl(EXYNOS5_ARM_CORE0_SYS_PWR_REG); pmu_raw_writel(t|S5P_CORE_LOCAL_PWR_EN,EXYNOS5_ARM_CORE0_SYS_PWR_REG); writel_relaxed(pm_state.cpu_state,pm_state.sysram_base.add(EXYNOS5420_CPU_STATE)); if pm_state.secure_firmware { exynos_smc(SMC_CMD_REG,SMC_REG_ID_SFR_W(pm_state.sysram_phys+EXYNOS5420_CPU_STATE),EXYNOS_AFTR_MAGIC,0); } pmu_raw_writel(EXYNOS5420_USE_STANDBY_WFI_ALL,S5P_CENTRAL_SEQ_OPTION); if exynos_pm_central_resume()==0 { pmu_raw_writel(pm_state.pmu_spare3,S5P_PMU_SPARE3); } t=pmu_raw_readl(EXYNOS5420_SFR_AXI_CGDIS1); pmu_raw_writel(t&!EXYNOS5420_UFS,EXYNOS5420_SFR_AXI_CGDIS1); t=pmu_raw_readl(EXYNOS5420_FSYS2_OPTION); pmu_raw_writel(t&!EXYNOS5420_EMULATION,EXYNOS5420_FSYS2_OPTION); t=pmu_raw_readl(EXYNOS5420_PSGEN_OPTION); pmu_raw_writel(t&!EXYNOS5420_EMULATION,EXYNOS5420_PSGEN_OPTION); pmu_raw_writel(0,S5P_INFORM1); }

// The remaining declarations and platform registration mirror the source; kernel-provided types and symbols are intentionally external.
unsafe fn exynos_suspend_enter(_state: suspend_state_t) -> i32 { let e = exynos_read_eint_wakeup_mask(); if exynos_irqwake_intmask == u32::MAX && e == EXYNOS_EINT_WAKEUP_MASK_DISABLED { pr_err!("No wake-up sources!\n"); return -EINVAL; } if let Some(f) = (*pm_data).pm_prepare { f(); } flush_cache_all(); let mut r = call_firmware_op(suspend); if r == -ENOSYS { r = cpu_suspend(0, (*pm_data).cpu_suspend); } if r != 0 { return r; } if let Some(f) = (*pm_data).pm_resume_prepare { f(); } 0 }

unsafe fn exynos_suspend_prepare() -> i32 { let r=regulator_suspend_prepare(PM_SUSPEND_MEM); if r!=0 { pr_err!("Failed to prepare regulators for suspend (%d)\n",r); } r }
unsafe fn exynos_suspend_finish() { let r=regulator_suspend_finish(); if r!=0 { pr_warn!("Failed to resume regulators from suspend (%d)\n",r); } }
unsafe fn exynos_pm_init() { let mut m=core::ptr::null(); let np=of_find_matching_node_and_match(core::ptr::null_mut(),exynos_pmu_of_device_ids.as_ptr(),&mut m); if np.is_null(){pr_err!("Failed to find PMU node\n");return;} pm_data=(*m).data as *const exynos_pm_data; let mut t=pmu_raw_readl(S5P_WAKEUP_MASK); t|=(*pm_data).wake_disable_mask; pmu_raw_writel(t,S5P_WAKEUP_MASK); if exynos_secure_firmware_available(){pm_state.sysram_phys=sysram_base_phys;pm_state.sysram_base=sysram_ns_base_addr;pm_state.secure_firmware=true}else{pm_state.sysram_base=sysram_base_addr;} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
