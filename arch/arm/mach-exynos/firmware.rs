// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2012 Samsung Electronics.
// Kyungmin Park <kyungmin.park@samsung.com>
// Tomasz Figa <t.figa@samsung.com>

// Linux and architecture headers are supplied by the surrounding translation.

const EXYNOS_BOOT_ADDR: usize = 0x8;
const EXYNOS_BOOT_FLAG: usize = 0xc;

unsafe fn exynos_save_cp15() {
    /* Save Power control and Diagnostic registers */
    core::arch::asm!(
        "mrc p15, 0, {0}, c15, c0, 0",
        "mrc p15, 0, {1}, c15, c0, 1",
        out(reg) cp15_save_power,
        out(reg) cp15_save_diag,
        options(nostack)
    );
}

unsafe fn exynos_do_idle(mode: c_ulong) -> c_int {
    match mode {
        FW_DO_IDLE_AFTR => {
            if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 { exynos_save_cp15(); }
            writel_relaxed(__pa_symbol(exynos_cpu_resume_ns), sysram_ns_base_addr.add(0x24));
            writel_relaxed(EXYNOS_AFTR_MAGIC, sysram_ns_base_addr.add(0x20));
            if soc_is_exynos3250() {
                flush_cache_all();
                exynos_smc(SMC_CMD_SAVE, OP_TYPE_CORE, SMC_POWERSTATE_IDLE, 0);
                exynos_smc(SMC_CMD_SHUTDOWN, OP_TYPE_CLUSTER, SMC_POWERSTATE_IDLE, 0);
            } else {
                exynos_smc(SMC_CMD_CPU0AFTR, 0, 0, 0);
            }
        }
        FW_DO_IDLE_SLEEP => exynos_smc(SMC_CMD_SLEEP, 0, 0, 0),
        _ => {}
    }
    0
}

unsafe fn exynos_cpu_boot(mut cpu: c_int) -> c_int {
    /* Exynos3250 does not need an SMC command; most Exynos5 calls are ignored. */
    if !soc_is_exynos4210() && !soc_is_exynos4212() && !soc_is_exynos4412()
        && !of_machine_is_compatible(c"google,manta".as_ptr() as *const c_char) { return 0; }
    /* Exynos4212 has only one secondary CPU. */
    if soc_is_exynos4212() { cpu = 0; }
    exynos_smc(SMC_CMD_CPU1BOOT, cpu as c_ulong, 0, 0);
    0
}

unsafe fn exynos_set_cpu_boot_addr(cpu: c_int, boot_addr: c_ulong) -> c_int {
    if sysram_ns_base_addr.is_null() { return -ENODEV; }
    let mut boot_reg = sysram_ns_base_addr.add(0x1c);
    if soc_is_exynos4412() { boot_reg = boot_reg.add((4 * cpu) as usize); }
    writel_relaxed(boot_addr, boot_reg);
    0
}

unsafe fn exynos_get_cpu_boot_addr(cpu: c_int, boot_addr: *mut c_ulong) -> c_int {
    if sysram_ns_base_addr.is_null() { return -ENODEV; }
    let mut boot_reg = sysram_ns_base_addr.add(0x1c);
    if soc_is_exynos4412() { boot_reg = boot_reg.add((4 * cpu) as usize); }
    *boot_addr = readl_relaxed(boot_reg);
    0
}

unsafe fn exynos_cpu_suspend(_arg: c_ulong) -> c_int {
    flush_cache_all();
    outer_flush_all();
    exynos_smc(SMC_CMD_SLEEP, 0, 0, 0);
    pr_info!("Failed to suspend the system\n");
    writel(0, sysram_ns_base_addr.add(EXYNOS_BOOT_FLAG));
    1
}

unsafe fn exynos_suspend() -> c_int {
    if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 { exynos_save_cp15(); }
    writel(EXYNOS_SLEEP_MAGIC, sysram_ns_base_addr.add(EXYNOS_BOOT_FLAG));
    writel(__pa_symbol(exynos_cpu_resume_ns), sysram_ns_base_addr.add(EXYNOS_BOOT_ADDR));
    cpu_suspend(0, exynos_cpu_suspend)
}

unsafe fn exynos_resume() -> c_int {
    writel(0, sysram_ns_base_addr.add(EXYNOS_BOOT_FLAG));
    0
}

static mut exynos_firmware_ops: firmware_ops = firmware_ops {
    do_idle: Some(exynos_do_idle),
    set_cpu_boot_addr: Some(exynos_set_cpu_boot_addr),
    get_cpu_boot_addr: Some(exynos_get_cpu_boot_addr),
    cpu_boot: Some(exynos_cpu_boot),
    suspend: Some(exynos_suspend),
    resume: Some(exynos_resume),
};

unsafe fn exynos_l2_write_sec(val: c_ulong, reg: c_uint) {
    static mut l2cache_enabled: c_int = 0;
    match reg {
        L2X0_CTRL => {
            if val & L2X0_CTRL_EN != 0 {
                if l2cache_enabled == 0 {
                    exynos_smc(SMC_CMD_L2X0INVALL, 0, 0, 0);
                    l2cache_enabled = 1;
                }
            } else { l2cache_enabled = 0; }
            exynos_smc(SMC_CMD_L2X0CTRL, val, 0, 0);
        }
        L2X0_DEBUG_CTRL => exynos_smc(SMC_CMD_L2X0DEBUG, val, 0, 0),
        _ => warn_once!("{}: ignoring write to reg 0x{:x}\n", "exynos_l2_write_sec", reg),
    }
}

unsafe fn exynos_l2_configure(regs: *const l2x0_regs) {
    exynos_smc(SMC_CMD_L2X0SETUP1, (*regs).tag_latency, (*regs).data_latency, (*regs).prefetch_ctrl);
    exynos_smc(SMC_CMD_L2X0SETUP2, (*regs).pwr_ctrl, (*regs).aux_ctrl, 0);
}

unsafe fn exynos_secure_firmware_available() -> bool {
    let nd = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"samsung,secure-firmware".as_ptr() as *const c_char);
    if nd.is_null() { return false; }
    let addr = of_get_address(nd, 0, core::ptr::null_mut(), core::ptr::null_mut());
    of_node_put(nd);
    if addr.is_null() { pr_err!("{}: No address specified.\n", "exynos_secure_firmware_available"); return false; }
    true
}

unsafe fn exynos_firmware_init() {
    if !exynos_secure_firmware_available() { return; }
    pr_info!("Running under secure firmware.\n");
    register_firmware_ops(&raw mut exynos_firmware_ops);
    if IS_ENABLED_CONFIG_CACHE_L2X0 && read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 {
        outer_cache.write_sec = Some(exynos_l2_write_sec);
        outer_cache.configure = Some(exynos_l2_configure);
    }
}

const BOOT_MODE_MASK: c_uint = 0x1f;

unsafe fn exynos_set_boot_flag(cpu: c_uint, mode: c_uint) {
    let addr = sysram_ns_base_addr.add(0x28 + (cpu * 4) as usize);
    let mut tmp = readl_relaxed(addr);
    if mode & BOOT_MODE_MASK != 0 { tmp &= !BOOT_MODE_MASK; }
    tmp |= mode;
    writel_relaxed(tmp, addr);
}

unsafe fn exynos_clear_boot_flag(cpu: c_uint, mode: c_uint) {
    let addr = sysram_ns_base_addr.add(0x28 + (cpu * 4) as usize);
    let mut tmp = readl_relaxed(addr);
    tmp &= !mode;
    writel_relaxed(tmp, addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
