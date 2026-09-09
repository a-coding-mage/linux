// SPDX-License-Identifier: GPL-2.0-only
/* OMAP MPUSS low power code. C include dependencies and build-time CONFIG_PM/
 * CONFIG_SMP conditions are supplied by the surrounding kernel translation. */

static mut sar_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut old_cpu1_ns_pa_addr: u32 = 0;

#[repr(C)]
struct omap4_cpu_pm_info {
    pwrdm: *mut powerdomain,
    scu_sar_addr: *mut core::ffi::c_void,
    wkup_sar_addr: *mut core::ffi::c_void,
    l2x0_sar_addr: *mut core::ffi::c_void,
}

#[repr(C)]
struct cpu_pm_ops {
    finish_suspend: Option<unsafe extern "C" fn(unsigned_long: core::ffi::c_ulong) -> i32>,
    resume: Option<unsafe extern "C" fn()>,
    scu_prepare: Option<unsafe extern "C" fn(u32, u32)>,
    hotplug_restart: Option<unsafe extern "C" fn()>,
}

// DEFINE_PER_CPU(struct omap4_cpu_pm_info, omap4_pm_info)
static mut omap4_pm_info: [omap4_cpu_pm_info; 2] = [
    omap4_cpu_pm_info { pwrdm: core::ptr::null_mut(), scu_sar_addr: core::ptr::null_mut(), wkup_sar_addr: core::ptr::null_mut(), l2x0_sar_addr: core::ptr::null_mut() },
    omap4_cpu_pm_info { pwrdm: core::ptr::null_mut(), scu_sar_addr: core::ptr::null_mut(), wkup_sar_addr: core::ptr::null_mut(), l2x0_sar_addr: core::ptr::null_mut() },
];
static mut mpuss_pd: *mut powerdomain = core::ptr::null_mut();
static mut cpu_context_offset: u32 = 0;

unsafe extern "C" fn default_finish_suspend(_cpu_state: core::ffi::c_ulong) -> i32 { omap_do_wfi(); 0 }
unsafe extern "C" fn dummy_cpu_resume() {}
unsafe extern "C" fn dummy_scu_prepare(_cpu_id: u32, _cpu_state: u32) {}
static mut omap_pm_ops: cpu_pm_ops = cpu_pm_ops { finish_suspend: Some(default_finish_suspend), resume: Some(dummy_cpu_resume), scu_prepare: Some(dummy_scu_prepare), hotplug_restart: Some(dummy_cpu_resume) };

unsafe fn set_cpu_wakeup_addr(cpu_id: u32, addr: u32) {
    let pm_info = &mut omap4_pm_info[cpu_id as usize];
    if !pm_info.wkup_sar_addr.is_null() { writel_relaxed(addr, pm_info.wkup_sar_addr); }
}

unsafe fn scu_pwrst_prepare(cpu_id: u32, cpu_state: u32) {
    let pm_info = &mut omap4_pm_info[cpu_id as usize];
    let scu_pwr_st = match cpu_state { PWRDM_POWER_RET => SCU_PM_DORMANT, PWRDM_POWER_OFF => SCU_PM_POWEROFF, _ => SCU_PM_NORMAL };
    if !pm_info.scu_sar_addr.is_null() { writel_relaxed(scu_pwr_st, pm_info.scu_sar_addr); }
}

unsafe fn mpuss_clear_prev_logic_pwrst() {
    let reg = omap4_prminst_read_inst_reg(OMAP4430_PRM_PARTITION, OMAP4430_PRM_MPU_INST, OMAP4_RM_MPU_MPU_CONTEXT_OFFSET);
    omap4_prminst_write_inst_reg(reg, OMAP4430_PRM_PARTITION, OMAP4430_PRM_MPU_INST, OMAP4_RM_MPU_MPU_CONTEXT_OFFSET);
}
unsafe fn cpu_clear_prev_logic_pwrst(cpu_id: u32) {
    let inst = if cpu_id != 0 { OMAP4430_PRCM_MPU_CPU1_INST } else { OMAP4430_PRCM_MPU_CPU0_INST };
    let reg = omap4_prcm_mpu_read_inst_reg(inst, cpu_context_offset);
    omap4_prcm_mpu_write_inst_reg(reg, inst, cpu_context_offset);
}
unsafe fn l2x0_pwrst_prepare(cpu_id: u32, save_state: u32) {
    let pm_info = &mut omap4_pm_info[cpu_id as usize];
    if !pm_info.l2x0_sar_addr.is_null() { writel_relaxed(save_state, pm_info.l2x0_sar_addr); }
}

unsafe fn save_l2x0_context() {
    let l2x0_base = omap4_get_l2cache_base();
    if !l2x0_base.is_null() && !sar_base.is_null() {
        writel_relaxed(l2x0_saved_regs.aux_ctrl, sar_base.add(L2X0_AUXCTRL_OFFSET as usize));
        writel_relaxed(l2x0_saved_regs.prefetch_ctrl, sar_base.add(L2X0_PREFETCH_CTRL_OFFSET as usize));
    }
}

unsafe fn omap4_enter_lowpower(cpu: u32, power_state: u32, rcuidle: bool) -> i32 {
    let pm_info = &mut omap4_pm_info[cpu as usize];
    let mut save_state = 0u32;
    let mut cpu_logic_state = PWRDM_POWER_RET;
    if omap_rev() == OMAP4430_REV_ES1_0 { return -ENXIO; }
    match power_state {
        PWRDM_POWER_ON | PWRDM_POWER_INACTIVE => {},
        PWRDM_POWER_OFF => { cpu_logic_state = PWRDM_POWER_OFF; save_state = 1; },
        PWRDM_POWER_RET => { if IS_PM44XX_ERRATUM(PM_OMAP4_CPU_OSWR_DISABLE) { save_state = 0; } },
        _ => { WARN_ON(1); return -ENXIO; }
    }
    pwrdm_pre_transition(core::ptr::null_mut());
    mpuss_clear_prev_logic_pwrst();
    if pwrdm_read_next_pwrst(mpuss_pd) == PWRDM_POWER_RET && pwrdm_read_logic_retst(mpuss_pd) == PWRDM_POWER_OFF { save_state = 2; }
    cpu_clear_prev_logic_pwrst(cpu);
    pwrdm_set_next_pwrst(pm_info.pwrdm, power_state);
    pwrdm_set_logic_retst(pm_info.pwrdm, cpu_logic_state);
    if rcuidle { ct_cpuidle_enter(); }
    set_cpu_wakeup_addr(cpu, __pa_symbol(omap_pm_ops.resume));
    (omap_pm_ops.scu_prepare.unwrap())(cpu, power_state);
    l2x0_pwrst_prepare(cpu, save_state);
    if save_state != 0 { cpu_suspend(save_state, omap_pm_ops.finish_suspend); } else { (omap_pm_ops.finish_suspend.unwrap())(save_state as core::ffi::c_ulong); }
    if IS_PM44XX_ERRATUM(PM_OMAP4_ROM_SMP_BOOT_ERRATUM_GICD) && cpu != 0 { gic_dist_enable(); }
    if rcuidle { ct_cpuidle_exit(); }
    pwrdm_set_next_pwrst(pm_info.pwrdm, PWRDM_POWER_ON);
    pwrdm_post_transition(core::ptr::null_mut());
    0
}

unsafe fn omap4_hotplug_cpu(cpu: u32, mut power_state: u32) -> i32 {
    let pm_info = &mut omap4_pm_info[cpu as usize];
    if omap_rev() == OMAP4430_REV_ES1_0 { return -ENXIO; }
    power_state = pwrdm_get_valid_lp_state(pm_info.pwrdm, false, power_state);
    let cpu_state = if power_state == PWRDM_POWER_OFF { 1 } else { 0 };
    pwrdm_clear_all_prev_pwrst(pm_info.pwrdm); pwrdm_set_next_pwrst(pm_info.pwrdm, power_state);
    set_cpu_wakeup_addr(cpu, __pa_symbol(omap_pm_ops.hotplug_restart));
    (omap_pm_ops.scu_prepare.unwrap())(cpu, power_state);
    (omap_pm_ops.finish_suspend.unwrap())(cpu_state);
    pwrdm_set_next_pwrst(pm_info.pwrdm, PWRDM_POWER_ON); 0
}

unsafe fn enable_mercury_retention_mode() {
    let mut reg = omap4_prcm_mpu_read_inst_reg(OMAP54XX_PRCM_MPU_DEVICE_INST, OMAP54XX_PRCM_MPU_PRM_PSCON_COUNT_OFFSET);
    reg |= BIT(24) | BIT(25);
    omap4_prcm_mpu_write_inst_reg(reg, OMAP54XX_PRCM_MPU_DEVICE_INST, OMAP54XX_PRCM_MPU_PRM_PSCON_COUNT_OFFSET);
}

unsafe fn omap4_mpuss_init() -> i32 {
    if omap_rev() == OMAP4430_REV_ES1_0 { WARN(1, "Power Management not supported on OMAP4430 ES1.0\n"); return -ENODEV; }
    for cpu in 0..2 { let p = &mut omap4_pm_info[cpu]; if !sar_base.is_null() { p.scu_sar_addr = sar_base.add(if cpu == 0 { SCU_OFFSET0 } else { SCU_OFFSET1 } as usize); p.wkup_sar_addr = sar_base.add((if cpu == 0 { CPU0_WAKEUP_NS_PA_ADDR_OFFSET } else { CPU1_WAKEUP_NS_PA_ADDR_OFFSET }) as usize); p.l2x0_sar_addr = sar_base.add((if cpu == 0 { L2X0_SAVE_OFFSET0 } else { L2X0_SAVE_OFFSET1 }) as usize); } p.pwrdm = pwrdm_lookup(if cpu == 0 { "cpu0_pwrdm" } else { "cpu1_pwrdm" }); if p.pwrdm.is_null() { return -ENODEV; } pwrdm_clear_all_prev_pwrst(p.pwrdm); cpu_clear_prev_logic_pwrst(cpu as u32); pwrdm_set_next_pwrst(p.pwrdm, PWRDM_POWER_ON); }
    mpuss_pd = pwrdm_lookup("mpu_pwrdm"); if mpuss_pd.is_null() { return -ENODEV; } pwrdm_clear_all_prev_pwrst(mpuss_pd); mpuss_clear_prev_logic_pwrst();
    if !sar_base.is_null() { writel_relaxed((omap_type() != OMAP2_DEVICE_TYPE_GP) as u32, sar_base.add(OMAP_TYPE_OFFSET as usize)); save_l2x0_context(); }
    if cpu_is_omap44xx() { omap_pm_ops.finish_suspend = Some(omap4_finish_suspend); omap_pm_ops.resume = Some(omap4_cpu_resume); omap_pm_ops.scu_prepare = Some(scu_pwrst_prepare); omap_pm_ops.hotplug_restart = Some(omap4_secondary_startup); cpu_context_offset = OMAP4_RM_CPU0_CPU0_CONTEXT_OFFSET; } else if soc_is_omap54xx() || soc_is_dra7xx() { cpu_context_offset = OMAP54XX_RM_CPU0_CPU0_CONTEXT_OFFSET; enable_mercury_retention_mode(); } if cpu_is_omap446x() { omap_pm_ops.hotplug_restart = Some(omap4460_secondary_startup); } 0
}

unsafe fn omap4_get_cpu1_ns_pa_addr() -> u32 { old_cpu1_ns_pa_addr }

unsafe fn omap4_mpuss_early_init() {
    if !(soc_is_omap44xx() || soc_is_omap54xx()) { return; }
    sar_base = omap4_get_sar_ram_base();
    let ns_pa_addr = sar_base.add((if soc_is_omap44xx() { CPU1_WAKEUP_NS_PA_ADDR_OFFSET } else { OMAP5_CPU1_WAKEUP_NS_PA_ADDR_OFFSET }) as usize);
    old_cpu1_ns_pa_addr = readl_relaxed(ns_pa_addr);
    let startup_pa = if soc_is_omap443x() { __pa_symbol(omap4_secondary_startup) } else if soc_is_omap446x() { __pa_symbol(omap4460_secondary_startup) } else if (__boot_cpu_mode & MODE_MASK) == HYP_MODE { __pa_symbol(omap5_secondary_hyp_startup) } else { __pa_symbol(omap5_secondary_startup) };
    writel_relaxed(startup_pa, ns_pa_addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
