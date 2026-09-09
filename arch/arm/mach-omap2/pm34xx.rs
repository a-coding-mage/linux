// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3 Power Management Routines
 *
 * Copyright (C) 2006-2008 Nokia Corporation
 * Tony Lindgren <tony@atomide.com>
 * Jouni Hogander
 *
 * Copyright (C) 2007 Texas Instruments, Inc.
 * Rajendra Nayak <rnayak@ti.com>
 *
 * Copyright (C) 2005 Texas Instruments, Inc.
 * Richard Woodruff <r-woodruff2@ti.com>
 *
 * Based on pm.c for omap1
 */

// C dependencies are supplied by the surrounding kernel translation.
// Build-time CONFIG_SUSPEND conditionals are preserved below.

#[repr(C)]
pub struct PowerState {
    pub pwrdm: *mut powerdomain,
    pub next_state: u32,
    #[cfg(feature = "CONFIG_SUSPEND")]
    pub saved_state: u32,
    pub node: list_head,
}

pub static mut pm34xx_errata: u16 = 0;
static mut pwrst_list: list_head = LIST_HEAD_INIT;
pub static mut omap3_do_wfi_sram: Option<unsafe extern "C" fn()> = None;
static mut mpu_pwrdm: *mut powerdomain = core::ptr::null_mut();
static mut neon_pwrdm: *mut powerdomain = core::ptr::null_mut();
static mut core_pwrdm: *mut powerdomain = core::ptr::null_mut();
static mut per_pwrdm: *mut powerdomain = core::ptr::null_mut();

unsafe fn omap3_core_save_context() {
    omap3_ctrl_save_padconf();
    omap_ctrl_writel(omap_ctrl_readl(OMAP343X_PADCONF_ETK_D14),
                     OMAP343X_CONTROL_MEM_WKUP + 0x2a0);
    omap_intc_save_context();
    omap3_control_save_context();
}

unsafe fn omap3_core_restore_context() {
    omap3_control_restore_context();
    omap_intc_restore_context();
}

unsafe fn omap3_save_secure_ram_context() {
    let mut ret: u32;
    let mpu_next_state = pwrdm_read_next_pwrst(mpu_pwrdm);
    if omap_type() != OMAP2_DEVICE_TYPE_GP {
        pwrdm_set_next_pwrst(mpu_pwrdm, PWRDM_POWER_ON);
        ret = omap3_save_secure_ram(omap3_secure_ram_storage, OMAP3_SAVE_SECURE_RAM_SZ);
        pwrdm_set_next_pwrst(mpu_pwrdm, mpu_next_state);
        if ret != 0 {
            pr_err!("save_secure_sram() returns {:08x}\n", ret);
            loop {}
        }
    }
}

unsafe extern "C" fn _prcm_int_handle_io(_irq: i32, _unused: *mut core::ffi::c_void) -> irqreturn_t {
    let c = omap_prm_clear_mod_irqs(WKUP_MOD, 1, OMAP3430_ST_IO_MASK | OMAP3430_ST_IO_CHAIN_MASK);
    if c != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe extern "C" fn _prcm_int_handle_wakeup(_irq: i32, _unused: *mut core::ffi::c_void) -> irqreturn_t {
    let mut c = omap_prm_clear_mod_irqs(WKUP_MOD, 1, !(OMAP3430_ST_IO_MASK | OMAP3430_ST_IO_CHAIN_MASK));
    c += omap_prm_clear_mod_irqs(CORE_MOD, 1, !0);
    c += omap_prm_clear_mod_irqs(OMAP3430_PER_MOD, 1, !0);
    if omap_rev() > OMAP3430_REV_ES1_0 {
        c += omap_prm_clear_mod_irqs(CORE_MOD, 3, !0);
        c += omap_prm_clear_mod_irqs(OMAP3430ES2_USBHOST_MOD, 1, !0);
    }
    if c != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn omap34xx_save_context(mut save: *mut u32) {
    let mut val: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1", out(reg) val);
    *save = 1; save = save.add(1); *save = val; save = save.add(1);
    core::arch::asm!("mrc p15, 1, {0}, c9, c0, 2", out(reg) val);
    *save = 1; save = save.add(1); *save = val;
}

unsafe fn omap34xx_do_sram_idle(save_state: usize) -> i32 {
    omap34xx_cpu_suspend(save_state);
    0
}

pub unsafe fn omap_sram_idle(rcuidle: bool) {
    let mut save_state = 0;
    let mpu_next_state = pwrdm_read_next_pwrst(mpu_pwrdm);
    let per_next_state;
    let core_next_state;
    let mut sdrc_pwr = 0;
    let error;
    match mpu_next_state {
        PWRDM_POWER_ON | PWRDM_POWER_RET => save_state = 0,
        PWRDM_POWER_OFF => save_state = 3,
        _ => { pr_err!("Invalid mpu state in sram_idle\n"); return; }
    }
    if pwrdm_read_pwrst(neon_pwrdm) == PWRDM_POWER_ON { pwrdm_set_next_pwrst(neon_pwrdm, mpu_next_state); }
    per_next_state = pwrdm_read_next_pwrst(per_pwrdm);
    core_next_state = pwrdm_read_next_pwrst(core_pwrdm);
    pwrdm_pre_transition(core::ptr::null_mut());
    if per_next_state == PWRDM_POWER_OFF {
        error = cpu_cluster_pm_enter();
        if error != 0 { return; }
    }
    if core_next_state < PWRDM_POWER_ON && core_next_state == PWRDM_POWER_OFF {
        omap3_core_save_context(); omap3_cm_save_context();
    }
    omap3_vc_set_pmic_signaling(core_next_state);
    omap3_intc_prepare_idle();
    if cpu_is_omap3430() && omap_rev() >= OMAP3430_REV_ES3_0 &&
       (omap_type() == OMAP2_DEVICE_TYPE_EMU || omap_type() == OMAP2_DEVICE_TYPE_SEC) &&
       core_next_state == PWRDM_POWER_OFF { sdrc_pwr = sdrc_read_reg(SDRC_POWER); }
    if save_state != 0 { omap34xx_save_context(omap3_arm_context); }
    if rcuidle { ct_cpuidle_enter(); }
    if save_state == 1 || save_state == 3 { cpu_suspend(save_state as usize, omap34xx_do_sram_idle); }
    else { omap34xx_do_sram_idle(save_state as usize); }
    if rcuidle { ct_cpuidle_exit(); }
    if cpu_is_omap3430() && omap_rev() >= OMAP3430_REV_ES3_0 &&
       (omap_type() == OMAP2_DEVICE_TYPE_EMU || omap_type() == OMAP2_DEVICE_TYPE_SEC) &&
       core_next_state == PWRDM_POWER_OFF { sdrc_write_reg(sdrc_pwr, SDRC_POWER); }
    if core_next_state < PWRDM_POWER_ON && pwrdm_read_prev_pwrst(core_pwrdm) == PWRDM_POWER_OFF {
        omap3_core_restore_context(); omap3_cm_restore_context(); omap3_sram_restore_context();
        omap2_sms_restore_context();
    } else { omap3_intc_resume_idle(); }
    pwrdm_post_transition(core::ptr::null_mut());
    if per_next_state == PWRDM_POWER_OFF { cpu_cluster_pm_exit(); }
}

unsafe fn omap3_pm_idle() { if !omap_irq_pending() { omap3_do_wfi(); } }

#[cfg(feature = "CONFIG_SUSPEND")]
unsafe fn omap3_pm_suspend() -> i32 {
    let mut ret = 0;
    let mut pwrst: *mut PowerState;
    list_for_each_entry!(pwrst, &mut pwrst_list, node) { (*pwrst).saved_state = pwrdm_read_next_pwrst((*pwrst).pwrdm); }
    list_for_each_entry!(pwrst, &mut pwrst_list, node) {
        if omap_set_pwrdm_state((*pwrst).pwrdm, (*pwrst).next_state) != 0 || pwrdm_clear_all_prev_pwrst((*pwrst).pwrdm) != 0 { break; }
    }
    omap3_intc_suspend(); omap_sram_idle(false);
    list_for_each_entry!(pwrst, &mut pwrst_list, node) {
        let state = pwrdm_read_prev_pwrst((*pwrst).pwrdm);
        if state > (*pwrst).next_state { pr_info!("Powerdomain (%s) didn't enter target state %d\n", (*pwrst).pwrdm.name, (*pwrst).next_state); ret = -1; }
        omap_set_pwrdm_state((*pwrst).pwrdm, (*pwrst).saved_state);
    }
    if ret != 0 { pr_err!("Could not enter target state in pm_suspend\n"); } else { pr_info!("Successfully put all powerdomains to target state\n"); }
    ret
}

// When CONFIG_SUSPEND is disabled, the C source defines omap3_pm_suspend as NULL.

unsafe fn prcm_setup_regs() { omap3_ctrl_init(); omap3_prm_init_pm(cpu_is_omap3630(), omap3_has_iva()); }

pub unsafe fn omap3_pm_off_mode_enable(enable: i32) {
    let state = if enable != 0 { PWRDM_POWER_OFF } else { PWRDM_POWER_RET };
    let mut pwrst: *mut PowerState;
    list_for_each_entry!(pwrst, &mut pwrst_list, node) {
        if IS_PM34XX_ERRATUM(PM_SDRC_WAKEUP_ERRATUM_i583) && (*pwrst).pwrdm == core_pwrdm && state == PWRDM_POWER_OFF {
            (*pwrst).next_state = PWRDM_POWER_RET; pr_warn!("%s: Core OFF disabled due to errata i583\n", "omap3_pm_off_mode_enable");
        } else { (*pwrst).next_state = state; }
        omap_set_pwrdm_state((*pwrst).pwrdm, (*pwrst).next_state);
    }
}

pub unsafe fn omap3_pm_get_suspend_state(pwrdm: *mut powerdomain) -> i32 {
    let mut pwrst: *mut PowerState;
    list_for_each_entry!(pwrst, &mut pwrst_list, node) { if (*pwrst).pwrdm == pwrdm { return (*pwrst).next_state; } }
    -EINVAL
}

pub unsafe fn omap3_pm_set_suspend_state(pwrdm: *mut powerdomain, state: i32) -> i32 {
    let mut pwrst: *mut PowerState;
    list_for_each_entry!(pwrst, &mut pwrst_list, node) { if (*pwrst).pwrdm == pwrdm { (*pwrst).next_state = state; return 0; } }
    -EINVAL
}

unsafe fn pwrdms_setup(pwrdm: *mut powerdomain, _unused: *mut core::ffi::c_void) -> i32 {
    if (*pwrdm).pwrsts.is_null() { return 0; }
    let pwrst = kmalloc_obj::<PowerState>(GFP_ATOMIC);
    if pwrst.is_null() { return -ENOMEM; }
    (*pwrst).pwrdm = pwrdm;
    (*pwrst).next_state = if enable_off_mode != 0 { PWRDM_POWER_OFF } else { PWRDM_POWER_RET };
    list_add(&mut (*pwrst).node, &mut pwrst_list);
    if pwrdm_has_hdwr_sar(pwrdm) { pwrdm_enable_hdwr_sar(pwrdm); }
    omap_set_pwrdm_state((*pwrst).pwrdm, (*pwrst).next_state)
}

pub unsafe fn omap_push_sram_idle() { omap3_do_wfi_sram = Some(omap_sram_push(omap3_do_wfi, omap3_do_wfi_sz)); }

unsafe fn pm_errata_configure() {
    if cpu_is_omap3630() { pm34xx_errata |= PM_RTA_ERRATUM_i608; enable_omap3630_toggle_l2_on_restore(); if omap_rev() < OMAP3630_REV_ES1_2 { pm34xx_errata |= PM_SDRC_WAKEUP_ERRATUM_i583 | PM_PER_MEMORIES_ERRATUM_i582; } }
    else if cpu_is_omap34xx() { pm34xx_errata |= PM_PER_MEMORIES_ERRATUM_i582; }
}

unsafe fn omap3_pm_check_pmic() {
    let mut np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ti,twl4030-power-idle");
    if np.is_null() { np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ti,twl4030-power-idle-osc-off"); }
    if !np.is_null() { of_node_put(np); enable_off_mode = 1; } else { enable_off_mode = 0; }
}

pub unsafe fn omap3_pm_init() -> i32 {
    if !omap3_has_io_chain_ctrl() { pr_warn!("PM: no software I/O chain control; some wakeups may be lost\n"); }
    pm_errata_configure(); prcm_setup_regs();
    let mut ret = request_irq(omap_prcm_event_to_irq("wkup"), _prcm_int_handle_wakeup, IRQF_NO_SUSPEND, "pm_wkup", core::ptr::null_mut());
    if ret != 0 { pr_err!("pm: Failed to request pm_wkup irq\n"); return ret; }
    ret = request_irq(omap_prcm_event_to_irq("io"), _prcm_int_handle_io, IRQF_SHARED | IRQF_NO_SUSPEND, "pm_io", omap3_pm_init as *mut _);
    if ret != 0 { pr_err!("pm: Failed to request pm_io irq\n"); free_irq(omap_prcm_event_to_irq("wkup"), core::ptr::null_mut()); return ret; }
    omap3_pm_check_pmic();
    ret = pwrdm_for_each(pwrdms_setup, core::ptr::null_mut());
    if ret != 0 { pr_err!("Failed to setup powerdomains\n"); free_irq(omap_prcm_event_to_irq("io"), omap3_pm_init as *mut _); free_irq(omap_prcm_event_to_irq("wkup"), core::ptr::null_mut()); return ret; }
    let _ = clkdm_for_each(omap_pm_clkdms_setup, core::ptr::null_mut());
    mpu_pwrdm = pwrdm_lookup("mpu_pwrdm");
    if mpu_pwrdm.is_null() { pr_err!("Failed to get mpu_pwrdm\n"); ret = -EINVAL; free_irq(omap_prcm_event_to_irq("io"), omap3_pm_init as *mut _); free_irq(omap_prcm_event_to_irq("wkup"), core::ptr::null_mut()); return ret; }
    neon_pwrdm = pwrdm_lookup("neon_pwrdm");
    per_pwrdm = pwrdm_lookup("per_pwrdm");
    core_pwrdm = pwrdm_lookup("core_pwrdm");
    let neon_clkdm = clkdm_lookup("neon_clkdm");
    let mpu_clkdm = clkdm_lookup("mpu_clkdm");
    let per_clkdm = clkdm_lookup("per_clkdm");
    let wkup_clkdm = clkdm_lookup("wkup_clkdm");
    #[cfg(feature = "CONFIG_SUSPEND")]
    omap_common_suspend_init(Some(omap3_pm_suspend));
    #[cfg(not(feature = "CONFIG_SUSPEND"))]
    omap_common_suspend_init(None);
    arm_pm_idle = Some(omap3_pm_idle);
    omap3_idle_init();
    if IS_PM34XX_ERRATUM(PM_RTA_ERRATUM_i608) { omap3630_ctrl_disable_rta(); }
    if IS_PM34XX_ERRATUM(PM_PER_MEMORIES_ERRATUM_i582) { clkdm_add_wkdep(per_clkdm, wkup_clkdm); }
    clkdm_add_wkdep(neon_clkdm, mpu_clkdm);
    if omap_type() != OMAP2_DEVICE_TYPE_GP {
        omap3_secure_ram_storage = kmalloc(OMAP3_SAVE_SECURE_RAM_SZ, GFP_KERNEL);
        if omap3_secure_ram_storage.is_null() { pr_err!("Memory allocation failed when allocating for secure sram context\n"); }
        local_irq_disable(); omap3_save_secure_ram_context(); local_irq_enable();
    }
    omap3_save_scratchpad_contents();
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
