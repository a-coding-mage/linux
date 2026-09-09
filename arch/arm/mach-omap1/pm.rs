/*
 * linux/arch/arm/mach-omap1/pm.c
 *
 * OMAP Power Management Routines
 *
 * Direct Rust translation. C headers, configuration conditionals, and
 * externally supplied macros/functions are intentionally left as external
 * dependencies.
 */

static mut arm_sleep_save: [u32; ARM_SLEEP_SAVE_SIZE] = [0; ARM_SLEEP_SAVE_SIZE];
static mut dsp_sleep_save: [u16; DSP_SLEEP_SAVE_SIZE] = [0; DSP_SLEEP_SAVE_SIZE];
static mut ulpd_sleep_save: [u16; ULPD_SLEEP_SAVE_SIZE] = [0; ULPD_SLEEP_SAVE_SIZE];
static mut mpui1510_sleep_save: [u32; MPUI1510_SLEEP_SAVE_SIZE] = [0; MPUI1510_SLEEP_SAVE_SIZE];
static mut mpui1610_sleep_save: [u32; MPUI1610_SLEEP_SAVE_SIZE] = [0; MPUI1610_SLEEP_SAVE_SIZE];
static mut enable_dyn_sleep: u16 = 0;
static mut sleep_while_idle_attr: kobj_attribute = kobj_attribute::zeroed();

unsafe fn idle_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut i8) -> isize {
    sprintf(buf, "%hu\n", enable_dyn_sleep)
}

unsafe fn idle_store(_kobj: *mut kobject, _attr: *mut kobj_attribute,
                     buf: *const i8, n: usize) -> isize {
    let mut value: u16 = 0;
    if sscanf(buf, "%hu", &mut value) != 1
        || (value != 0 && value != 1)
        || (value != 0 && !IS_ENABLED(CONFIG_OMAP_32K_TIMER)) {
        pr_err!("idle_sleep_store: Invalid value\n");
        return -EINVAL as isize;
    }
    enable_dyn_sleep = value;
    n as isize
}

static mut omap_sram_suspend: Option<unsafe extern "C" fn(usize, usize)> = None;

pub unsafe fn omap1_pm_idle() {
    let mut use_idlect1: u32 = arm_idlect1_mask;
    local_fiq_disable();

    // Build-time CONFIG_OMAP_MPU_TIMER/CONFIG_OMAP_DM_TIMER branches preserved by intent.
    #[cfg(all(CONFIG_OMAP_MPU_TIMER, not(CONFIG_OMAP_DM_TIMER)))]
    { use_idlect1 &= !(1 << 9); }
    #[cfg(CONFIG_OMAP_DM_TIMER)]
    { use_idlect1 = omap_dm_timer_modify_idlect_mask(use_idlect1); }

    if omap_dma_running() { use_idlect1 &= !(1 << 6); }
    if use_idlect1 != !0 || enable_dyn_sleep == 0 {
        let saved_idlect1 = omap_readl(ARM_IDLECT1);
        if cpu_is_omap15xx() { use_idlect1 &= OMAP1510_BIG_SLEEP_REQUEST; }
        else { use_idlect1 &= OMAP1610_IDLECT1_SLEEP_VAL; }
        omap_writel(use_idlect1, ARM_IDLECT1);
        core::arch::asm!("mcr p15, 0, r0, c7, c0, 4");
        omap_writel(saved_idlect1, ARM_IDLECT1);
        local_fiq_enable();
        return;
    }
    if let Some(f) = omap_sram_suspend { f(omap_readl(ARM_IDLECT1) as usize, omap_readl(ARM_IDLECT2) as usize); }
    local_fiq_enable();
}

unsafe fn omap_pm_wakeup_setup() {
    let mut level1_wake: u32 = 0;
    let mut level2_wake: u32 = OMAP_IRQ_BIT(INT_UART2);
    if cpu_is_omap15xx() { level1_wake = OMAP_IRQ_BIT(INT_GPIO_BANK1) | OMAP_IRQ_BIT(INT_1510_IH2_IRQ); }
    else if cpu_is_omap16xx() { level1_wake = OMAP_IRQ_BIT(INT_GPIO_BANK1) | OMAP_IRQ_BIT(INT_1610_IH2_IRQ); }
    omap_writel(!level1_wake, OMAP_IH1_MIR);
    if cpu_is_omap15xx() {
        level2_wake |= OMAP_IRQ_BIT(INT_KEYBOARD); omap_writel(!level2_wake, OMAP_IH2_MIR);
    } else if cpu_is_omap16xx() {
        level2_wake |= OMAP_IRQ_BIT(INT_KEYBOARD); omap_writel(!level2_wake, OMAP_IH2_0_MIR);
        omap_writel(!OMAP_IRQ_BIT(INT_1610_WAKE_UP_REQ), OMAP_IH2_1_MIR);
        omap_writel(!0, OMAP_IH2_2_MIR); omap_writel(!0, OMAP_IH2_3_MIR);
    }
    omap_writel(1, OMAP_IH2_CONTROL); omap_writel(1, OMAP_IH1_CONTROL);
}

const EN_DSPCK: u32 = 13;
const EN_APICK: u32 = 6;
const DSP_EN: u32 = 1;

pub unsafe fn omap1_pm_suspend() {
    let mut arg0: usize = 0; let mut arg1: usize = 0;
    printk!(KERN_INFO "PM: OMAP%x is trying to enter deep sleep...\n", omap_rev());
    omap_serial_wake_trigger(1);
    if !cpu_is_omap15xx() { omap_writew(0xffff, ULPD_SOFT_DISABLE_REQ_REG); }
    local_irq_disable(); local_fiq_disable();
    if cpu_is_omap15xx() {
        MPUI1510_SAVE!(OMAP_IH1_MIR); MPUI1510_SAVE!(OMAP_IH2_MIR); MPUI1510_SAVE!(MPUI_CTRL); MPUI1510_SAVE!(MPUI_DSP_BOOT_CONFIG); MPUI1510_SAVE!(MPUI_DSP_API_CONFIG); MPUI1510_SAVE!(EMIFS_CONFIG); MPUI1510_SAVE!(EMIFF_SDRAM_CONFIG);
    } else if cpu_is_omap16xx() {
        MPUI1610_SAVE!(OMAP_IH1_MIR); MPUI1610_SAVE!(OMAP_IH2_0_MIR); MPUI1610_SAVE!(OMAP_IH2_1_MIR); MPUI1610_SAVE!(OMAP_IH2_2_MIR); MPUI1610_SAVE!(OMAP_IH2_3_MIR); MPUI1610_SAVE!(MPUI_CTRL); MPUI1610_SAVE!(MPUI_DSP_BOOT_CONFIG); MPUI1610_SAVE!(MPUI_DSP_API_CONFIG); MPUI1610_SAVE!(EMIFS_CONFIG); MPUI1610_SAVE!(EMIFF_SDRAM_CONFIG);
    }
    ARM_SAVE!(ARM_CKCTL); ARM_SAVE!(ARM_IDLECT1); ARM_SAVE!(ARM_IDLECT2); if !cpu_is_omap15xx() { ARM_SAVE!(ARM_IDLECT3); } ARM_SAVE!(ARM_EWUPCT); ARM_SAVE!(ARM_RSTCT1); ARM_SAVE!(ARM_RSTCT2); ARM_SAVE!(ARM_SYSST); ULPD_SAVE!(ULPD_CLOCK_CTRL); ULPD_SAVE!(ULPD_STATUS_REQ);
    omap_writew(omap_readw(ARM_RSTCT1) & !(1 << DSP_EN), ARM_RSTCT1); omap_writew(omap_readw(ARM_CKCTL) & !(1 << EN_DSPCK), ARM_CKCTL); omap_writew(omap_readw(ARM_IDLECT2) | (1 << EN_APICK), ARM_IDLECT2); DSP_SAVE!(DSP_IDLECT2); __raw_writew(0, DSP_IDLECT2);
    omap_pm_wakeup_setup(); omap_writel(0x00F5, OMAP_WDT_TIMER_MODE); omap_writel(0x00A0, OMAP_WDT_TIMER_MODE);
    arg0 = arm_sleep_save[ARM_SLEEP_SAVE_ARM_IDLECT1]; arg1 = arm_sleep_save[ARM_SLEEP_SAVE_ARM_IDLECT2];
    if let Some(f) = omap_sram_suspend { f(arg0, arg1); }
    omap_writew(omap_readw(ARM_IDLECT2) | (1 << EN_APICK), ARM_IDLECT2); DSP_RESTORE!(DSP_IDLECT2);
    if !cpu_is_omap15xx() { ARM_RESTORE!(ARM_IDLECT3); } ARM_RESTORE!(ARM_CKCTL); ARM_RESTORE!(ARM_EWUPCT); ARM_RESTORE!(ARM_RSTCT1); ARM_RESTORE!(ARM_RSTCT2); ARM_RESTORE!(ARM_SYSST); ULPD_RESTORE!(ULPD_CLOCK_CTRL); ULPD_RESTORE!(ULPD_STATUS_REQ);
    if cpu_is_omap15xx() { MPUI1510_RESTORE!(MPUI_CTRL); MPUI1510_RESTORE!(MPUI_DSP_BOOT_CONFIG); MPUI1510_RESTORE!(MPUI_DSP_API_CONFIG); MPUI1510_RESTORE!(EMIFS_CONFIG); MPUI1510_RESTORE!(EMIFF_SDRAM_CONFIG); MPUI1510_RESTORE!(OMAP_IH1_MIR); MPUI1510_RESTORE!(OMAP_IH2_MIR); }
    else if cpu_is_omap16xx() { MPUI1610_RESTORE!(MPUI_CTRL); MPUI1610_RESTORE!(MPUI_DSP_BOOT_CONFIG); MPUI1610_RESTORE!(MPUI_DSP_API_CONFIG); MPUI1610_RESTORE!(EMIFS_CONFIG); MPUI1610_RESTORE!(EMIFF_SDRAM_CONFIG); MPUI1610_RESTORE!(OMAP_IH1_MIR); MPUI1610_RESTORE!(OMAP_IH2_0_MIR); MPUI1610_RESTORE!(OMAP_IH2_1_MIR); MPUI1610_RESTORE!(OMAP_IH2_2_MIR); MPUI1610_RESTORE!(OMAP_IH2_3_MIR); }
    if !cpu_is_omap15xx() { omap_writew(0, ULPD_SOFT_DISABLE_REQ_REG); } local_irq_enable(); local_fiq_enable(); omap_serial_wake_trigger(0); printk!(KERN_INFO "PM: OMAP%x is re-starting from deep sleep...\n", omap_rev());
}

unsafe fn omap_pm_prepare() -> i32 { cpu_idle_poll_ctrl(true); 0 }
unsafe fn omap_pm_enter(state: suspend_state_t) -> i32 { if state == PM_SUSPEND_MEM { omap1_pm_suspend(); 0 } else { -EINVAL } }
unsafe fn omap_pm_finish() { cpu_idle_poll_ctrl(false); }
unsafe extern "C" fn omap_wakeup_interrupt(_irq: i32, _dev: *mut core::ffi::c_void) -> irqreturn_t { IRQ_HANDLED }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn omap_pm_debug_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    ARM_SAVE!(ARM_CKCTL); ARM_SAVE!(ARM_IDLECT1); ARM_SAVE!(ARM_IDLECT2);
    if !cpu_is_omap15xx() { ARM_SAVE!(ARM_IDLECT3); }
    ARM_SAVE!(ARM_EWUPCT); ARM_SAVE!(ARM_RSTCT1); ARM_SAVE!(ARM_RSTCT2); ARM_SAVE!(ARM_SYSST);
    ULPD_SAVE!(ULPD_IT_STATUS); ULPD_SAVE!(ULPD_CLOCK_CTRL); ULPD_SAVE!(ULPD_SOFT_REQ);
    ULPD_SAVE!(ULPD_STATUS_REQ); ULPD_SAVE!(ULPD_DPLL_CTRL); ULPD_SAVE!(ULPD_POWER_CTRL);
    if cpu_is_omap15xx() {
        MPUI1510_SAVE!(MPUI_CTRL); MPUI1510_SAVE!(MPUI_DSP_STATUS); MPUI1510_SAVE!(MPUI_DSP_BOOT_CONFIG); MPUI1510_SAVE!(MPUI_DSP_API_CONFIG); MPUI1510_SAVE!(EMIFF_SDRAM_CONFIG); MPUI1510_SAVE!(EMIFS_CONFIG);
    } else if cpu_is_omap16xx() {
        MPUI1610_SAVE!(MPUI_CTRL); MPUI1610_SAVE!(MPUI_DSP_STATUS); MPUI1610_SAVE!(MPUI_DSP_BOOT_CONFIG); MPUI1610_SAVE!(MPUI_DSP_API_CONFIG); MPUI1610_SAVE!(EMIFF_SDRAM_CONFIG); MPUI1610_SAVE!(EMIFS_CONFIG);
    }
    seq_printf!(m, "ARM_CKCTL_REG: 0x%-8x\nARM_IDLECT1_REG: 0x%-8x\nARM_IDLECT2_REG: 0x%-8x\nARM_IDLECT3_REG: 0x%-8x\nARM_EWUPCT_REG: 0x%-8x\nARM_RSTCT1_REG: 0x%-8x\nARM_RSTCT2_REG: 0x%-8x\nARM_SYSST_REG: 0x%-8x\n", ARM_SHOW!(ARM_CKCTL), ARM_SHOW!(ARM_IDLECT1), ARM_SHOW!(ARM_IDLECT2), ARM_SHOW!(ARM_IDLECT3), ARM_SHOW!(ARM_EWUPCT), ARM_SHOW!(ARM_RSTCT1), ARM_SHOW!(ARM_RSTCT2), ARM_SHOW!(ARM_SYSST));
    0
}

static omap_pm_ops: platform_suspend_ops = platform_suspend_ops {
    prepare: Some(omap_pm_prepare), enter: Some(omap_pm_enter), finish: Some(omap_pm_finish), valid: Some(suspend_valid_only_mem),
};

// CONFIG_DEBUG_FS debug-register display and __initcall registration are retained as external integration points.
unsafe fn omap_pm_init() -> i32 {
    if !cpu_class_is_omap1() { return -ENODEV; }
    pr_info!("Power Management for TI OMAP.\n");
    if IS_ENABLED(CONFIG_OMAP_32K_TIMER) && IS_ENABLED(CONFIG_OMAP_DM_TIMER) { enable_dyn_sleep = 1; }
    omap_sram_suspend = if cpu_is_omap15xx() { Some(omap_sram_push(omap1510_cpu_suspend, omap1510_cpu_suspend_sz)) } else if cpu_is_omap16xx() { Some(omap_sram_push(omap1610_cpu_suspend, omap1610_cpu_suspend_sz)) } else { None };
    if omap_sram_suspend.is_none() { printk!(KERN_ERR "PM not initialized: Missing SRAM support\n"); return -ENODEV; }
    arm_pm_idle = Some(omap1_pm_idle);
    let irq = if cpu_is_omap16xx() { INT_1610_WAKE_UP_REQ } else { -1 };
    if irq >= 0 && request_irq(irq, omap_wakeup_interrupt, 0, "peripheral wakeup", core::ptr::null_mut()) != 0 { pr_err!("Failed to request irq %d (peripheral wakeup)\n", irq); }
    omap_writew(ULPD_SETUP_ANALOG_CELL_3_VAL, ULPD_SETUP_ANALOG_CELL_3); omap_writew(ULPD_POWER_CTRL_REG_VAL, ULPD_POWER_CTRL);
    if cpu_is_omap16xx() { omap_writel(OMAP1610_IDLECT3_VAL, OMAP1610_IDLECT3); }
    suspend_set_ops(&omap_pm_ops); let error = sysfs_create_file(power_kobj, &sleep_while_idle_attr.attr); if error != 0 { pr_err!("sysfs_create_file failed: %d\n", error); }
    if cpu_is_omap16xx() { omap_cfg_reg(T20_1610_LOW_PWR); } error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
