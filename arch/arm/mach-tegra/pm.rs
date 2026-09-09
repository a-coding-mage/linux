// SPDX-License-Identifier: GPL-2.0-only
/* CPU complex suspend & resume functions for Tegra SoCs */

// Kernel dependencies supplied by other translation units are intentionally not implemented here.

#[cfg(CONFIG_PM_SLEEP)]
static mut tegra_lp2_lock: spinlock_t = unsafe { core::mem::zeroed() };
#[cfg(CONFIG_PM_SLEEP)]
static mut iram_save_size: u32 = 0;
#[cfg(CONFIG_PM_SLEEP)]
static mut iram_save_addr: *mut core::ffi::c_void = core::ptr::null_mut();
#[cfg(CONFIG_PM_SLEEP)]
static mut tegra_lp1_iram: tegra_lp1_iram = unsafe { core::mem::zeroed() };
#[cfg(CONFIG_PM_SLEEP)]
static mut tegra_tear_down_cpu: Option<unsafe extern "C" fn()> = None;
#[cfg(CONFIG_PM_SLEEP)]
static mut tegra_sleep_core_finish: Option<unsafe extern "C" fn(usize)> = None;
#[cfg(CONFIG_PM_SLEEP)]
static mut tegra_sleep_func: Option<unsafe extern "C" fn(usize) -> i32> = None;

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn tegra_tear_down_cpu_init() {
    match tegra_get_chip_id() {
        TEGRA20 => {
            if IS_ENABLED(CONFIG_ARCH_TEGRA_2x_SOC) { tegra_tear_down_cpu = Some(tegra20_tear_down_cpu); }
        }
        TEGRA30 | TEGRA114 | TEGRA124 => {
            if IS_ENABLED(CONFIG_ARCH_TEGRA_3x_SOC) || IS_ENABLED(CONFIG_ARCH_TEGRA_114_SOC) || IS_ENABLED(CONFIG_ARCH_TEGRA_124_SOC) {
                tegra_tear_down_cpu = Some(tegra30_tear_down_cpu);
            }
        }
        _ => {}
    }
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn restore_cpu_complex() {
    let mut cpu = smp_processor_id();
    BUG_ON(cpu != 0);
    #[cfg(CONFIG_SMP)] { cpu = cpu_logical_map(cpu); }
    tegra_cpu_clock_resume();
    flowctrl_cpu_suspend_exit(cpu);
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn suspend_cpu_complex() {
    let mut cpu = smp_processor_id();
    BUG_ON(cpu != 0);
    #[cfg(CONFIG_SMP)] { cpu = cpu_logical_map(cpu); }
    tegra_cpu_clock_suspend();
    flowctrl_cpu_suspend_enter(cpu);
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn tegra_pm_clear_cpu_in_lp2() {
    let phy_cpu_id = cpu_logical_map(smp_processor_id());
    let cpu_in_lp2 = tegra_cpu_lp2_mask;
    spin_lock(&raw mut tegra_lp2_lock);
    BUG_ON(!(*cpu_in_lp2 & BIT(phy_cpu_id)));
    *cpu_in_lp2 &= !BIT(phy_cpu_id);
    spin_unlock(&raw mut tegra_lp2_lock);
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn tegra_pm_set_cpu_in_lp2() {
    let phy_cpu_id = cpu_logical_map(smp_processor_id());
    let cpu_in_lp2 = tegra_cpu_lp2_mask;
    spin_lock(&raw mut tegra_lp2_lock);
    BUG_ON(*cpu_in_lp2 & BIT(phy_cpu_id));
    *cpu_in_lp2 |= BIT(phy_cpu_id);
    spin_unlock(&raw mut tegra_lp2_lock);
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn tegra_sleep_cpu(v2p: usize) -> i32 {
    if tegra_cpu_car_ops.rail_off_ready && WARN_ON(!tegra_cpu_rail_off_ready()) { return -EBUSY; }
    #[cfg(CONFIG_OUTER_CACHE)]
    if trusted_foundations_registered() && outer_cache.disable { outer_cache.disable(); }
    call_firmware_op(prepare_idle, TF_PM_MODE_LP2);
    setup_mm_for_reboot();
    if let Some(f) = tegra_sleep_cpu_finish { f(v2p); }
    BUG();
    0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn tegra_pm_set(mode: tegra_suspend_mode) {
    let mut value: u32;
    match tegra_get_chip_id() {
        TEGRA20 | TEGRA30 => {}
        _ => {
            value = flowctrl_read_cpu_csr(0);
            value &= !FLOW_CTRL_CSR_ENABLE_EXT_MASK;
            value |= FLOW_CTRL_CSR_ENABLE_EXT_CRAIL;
            flowctrl_write_cpu_csr(0, value);
        }
    }
    tegra_pmc_enter_suspend_mode(mode);
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn tegra_pm_enter_lp2() -> i32 {
    tegra_pm_set(TEGRA_SUSPEND_LP2);
    cpu_cluster_pm_enter();
    suspend_cpu_complex();
    let err = cpu_suspend(PHYS_OFFSET - PAGE_OFFSET, Some(tegra_sleep_cpu));
    outer_resume();
    restore_cpu_complex();
    cpu_cluster_pm_exit();
    call_firmware_op(prepare_idle, TF_PM_MODE_NONE);
    err
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn tegra_pm_validate_suspend_mode(mut mode: tegra_suspend_mode) -> tegra_suspend_mode {
    if mode > TEGRA_SUSPEND_LP1 { mode = TEGRA_SUSPEND_LP1; }
    mode
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn tegra_sleep_core(v2p: usize) -> i32 {
    if trusted_foundations_registered() { outer_disable(); }
    call_firmware_op(prepare_idle, TF_PM_MODE_LP1);
    setup_mm_for_reboot();
    if let Some(f) = tegra_sleep_core_finish { f(v2p); }
    BUG();
    0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn tegra_lp1_iram_hook() -> bool {
    match tegra_get_chip_id() {
        TEGRA20 if IS_ENABLED(CONFIG_ARCH_TEGRA_2x_SOC) => tegra20_lp1_iram_hook(),
        TEGRA30 | TEGRA114 | TEGRA124 if IS_ENABLED(CONFIG_ARCH_TEGRA_3x_SOC) || IS_ENABLED(CONFIG_ARCH_TEGRA_114_SOC) || IS_ENABLED(CONFIG_ARCH_TEGRA_124_SOC) => tegra30_lp1_iram_hook(),
        _ => {}
    }
    if tegra_lp1_iram.start_addr.is_null() || tegra_lp1_iram.end_addr.is_null() { return false; }
    iram_save_size = tegra_lp1_iram.end_addr.offset_from(tegra_lp1_iram.start_addr) as u32;
    iram_save_addr = kmalloc(iram_save_size as usize, GFP_KERNEL);
    if iram_save_addr.is_null() { return false; }
    true
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn tegra_sleep_core_init() -> bool {
    match tegra_get_chip_id() {
        TEGRA20 if IS_ENABLED(CONFIG_ARCH_TEGRA_2x_SOC) => tegra20_sleep_core_init(),
        TEGRA30 | TEGRA114 | TEGRA124 if IS_ENABLED(CONFIG_ARCH_TEGRA_3x_SOC) || IS_ENABLED(CONFIG_ARCH_TEGRA_114_SOC) || IS_ENABLED(CONFIG_ARCH_TEGRA_124_SOC) => tegra30_sleep_core_init(),
        _ => {}
    }
    tegra_sleep_core_finish.is_some()
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn tegra_suspend_enter_lp1() {
    memcpy(iram_save_addr, IO_ADDRESS(TEGRA_IRAM_LPx_RESUME_AREA), iram_save_size as usize);
    memcpy(IO_ADDRESS(TEGRA_IRAM_LPx_RESUME_AREA), tegra_lp1_iram.start_addr, iram_save_size as usize);
    *(tegra_cpu_lp1_mask as *mut u32) = 1;
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn tegra_suspend_exit_lp1() {
    memcpy(IO_ADDRESS(TEGRA_IRAM_LPx_RESUME_AREA), iram_save_addr, iram_save_size as usize);
    *(tegra_cpu_lp1_mask as *mut u32) = 0;
}

#[cfg(CONFIG_PM_SLEEP)]
static lp_state: [&str; TEGRA_MAX_SUSPEND_MODE as usize] = ["none", "LP2", "LP1", "LP0"];

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn tegra_suspend_enter(_state: suspend_state_t) -> i32 {
    let mode = tegra_pmc_get_suspend_mode();
    if WARN_ON(mode < TEGRA_SUSPEND_NONE || mode >= TEGRA_MAX_SUSPEND_MODE) { return -EINVAL; }
    pr_info!("Entering suspend state {}\n", lp_state[mode as usize]);
    tegra_pm_set(mode);
    local_fiq_disable();
    suspend_cpu_complex();
    match mode { TEGRA_SUSPEND_LP1 => tegra_suspend_enter_lp1(), TEGRA_SUSPEND_LP2 => tegra_pm_set_cpu_in_lp2(), _ => {} }
    cpu_suspend(PHYS_OFFSET - PAGE_OFFSET, tegra_sleep_func);
    outer_resume();
    match mode { TEGRA_SUSPEND_LP1 => tegra_suspend_exit_lp1(), TEGRA_SUSPEND_LP2 => tegra_pm_clear_cpu_in_lp2(), _ => {} }
    restore_cpu_complex();
    local_fiq_enable();
    call_firmware_op(prepare_idle, TF_PM_MODE_NONE);
    0
}

#[cfg(CONFIG_PM_SLEEP)]
static tegra_suspend_ops: platform_suspend_ops = platform_suspend_ops { valid: Some(suspend_valid_only_mem), enter: Some(tegra_suspend_enter) };

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn tegra_pm_init_suspend() {
    let mut mode = tegra_pmc_get_suspend_mode();
    if mode == TEGRA_SUSPEND_NONE { return; }
    tegra_tear_down_cpu_init();
    if mode >= TEGRA_SUSPEND_LP1 && (!tegra_lp1_iram_hook() || !tegra_sleep_core_init()) {
        pr_err!("{}: unable to allocate memory for SDRAMself-refresh -- LP0/LP1 unavailable\n", __func__);
        tegra_pmc_set_suspend_mode(TEGRA_SUSPEND_LP2);
        mode = TEGRA_SUSPEND_LP2;
    }
    match mode { TEGRA_SUSPEND_LP1 => tegra_sleep_func = Some(tegra_sleep_core), TEGRA_SUSPEND_LP2 => tegra_sleep_func = Some(tegra_sleep_cpu), _ => {} }
    suspend_set_ops(&tegra_suspend_ops);
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn tegra_pm_park_secondary_cpu(cpu: usize) -> i32 {
    if cpu > 0 {
        tegra_disable_clean_inv_dcache(TEGRA_FLUSH_CACHE_LOUIS);
        if tegra_get_chip_id() == TEGRA20 { tegra20_hotplug_shutdown(); } else { tegra30_hotplug_shutdown(); }
    }
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
