/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2004-2014 Freescale Semiconductor, Inc. All Rights Reserved.
 */

// Dependency declarations supplied by other translated units.
pub enum irq_data {}
pub enum platform_device {}
pub enum pt_regs {}
pub enum clk {}
pub enum device_node {}
pub enum reboot_mode {}
pub enum smp_operations {}

extern "C" {
    pub fn mx31_map_io();
    pub fn mx35_map_io();
    pub fn imx31_init_early();
    pub fn imx35_init_early();
    pub fn mxc_set_cpu_type(ty: u32);
    pub fn mxc_restart(mode: reboot_mode, command: *const core::ffi::c_char);
    pub fn mxc_arch_reset_init(base: *mut core::ffi::c_void);
    pub fn imx_set_aips(base: *mut core::ffi::c_void);
    pub fn imx_aips_allow_unprivileged_access(compat: *const core::ffi::c_char);
    pub fn imx_set_soc_revision(rev: u32);
    pub fn imx_init_revision_from_anatop();
    pub fn imx6_enable_rbc(enable: bool);
    pub fn imx_gpc_check_dt();
    pub fn imx_gpc_set_arm_power_in_lpm(power_off: bool);
    pub fn imx_gpc_set_l2_mem_power_in_lpm(power_off: bool);
    pub fn imx_gpc_set_arm_power_up_timing(sw2iso: u32, sw: u32);
    pub fn imx_gpc_set_arm_power_down_timing(sw2iso: u32, sw: u32);
    pub fn imx25_pm_init();
    pub fn imx27_pm_init();
    pub fn imx5_pmu_init();
}

#[repr(C)]
pub enum mxc_cpu_pwr_mode {
    WAIT_CLOCKED,
    WAIT_UNCLOCKED,
    WAIT_UNCLOCKED_POWER_OFF,
    STOP_POWER_ON,
    STOP_POWER_OFF,
}

#[repr(C)]
pub enum ulp_cpu_pwr_mode {
    ULP_PM_HSRUN,
    ULP_PM_RUN,
    ULP_PM_WAIT,
    ULP_PM_STOP,
    ULP_PM_VLPS,
    ULP_PM_VLLS,
}

extern "C" {
    pub fn imx_enable_cpu(cpu: i32, enable: bool);
    pub fn imx_set_cpu_jump(cpu: i32, jump_addr: *mut core::ffi::c_void);
    pub fn imx_get_cpu_arg(cpu: i32) -> u32;
    pub fn imx_set_cpu_arg(cpu: i32, arg: u32);
}

// CONFIG_SMP selects the declarations below; otherwise the C header provides empty inline stubs.
#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn v7_secondary_startup();
    pub fn imx_scu_map_io();
    pub fn imx_smp_prepare();
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub fn imx_scu_map_io() {}
#[cfg(not(feature = "CONFIG_SMP"))]
pub fn imx_smp_prepare() {}

extern "C" {
    pub fn imx_src_init();
    pub fn imx7_src_init();
    pub fn imx_gpc_pre_suspend(arm_power_off: bool);
    pub fn imx_gpc_post_resume();
    pub fn imx_gpc_mask_all();
    pub fn imx_gpc_restore_all();
    pub fn imx_gpc_hwirq_mask(hwirq: u32);
    pub fn imx_gpc_hwirq_unmask(hwirq: u32);
    pub fn imx_gpcv2_set_core1_pdn_pup_by_software(pdn: bool);
    pub fn imx_anatop_init();
    pub fn imx_anatop_pre_suspend();
    pub fn imx_anatop_post_resume();
    pub fn imx6_set_lpm(mode: mxc_cpu_pwr_mode) -> i32;
    pub fn imx6_set_int_mem_clk_lpm(enable: bool);
    pub fn imx_mmdc_get_ddr_type() -> i32;
    pub fn imx7ulp_set_lpm(mode: ulp_cpu_pwr_mode) -> i32;
    pub fn imx_cpu_die(cpu: u32);
    pub fn imx_cpu_kill(cpu: u32) -> i32;
}

#[cfg(feature = "CONFIG_SUSPEND")]
extern "C" {
    pub fn imx53_suspend(ocram_vbase: *mut core::ffi::c_void);
    pub static imx53_suspend_sz: u32;
    pub fn imx6_suspend(ocram_vbase: *mut core::ffi::c_void);
}

#[cfg(not(feature = "CONFIG_SUSPEND"))]
pub fn imx53_suspend(_ocram_vbase: *mut core::ffi::c_void) {}
#[cfg(not(feature = "CONFIG_SUSPEND"))]
pub static imx53_suspend_sz: u32 = 0;
#[cfg(not(feature = "CONFIG_SUSPEND"))]
pub fn imx6_suspend(_ocram_vbase: *mut core::ffi::c_void) {}

extern "C" {
    pub fn v7_cpu_resume();
    pub fn imx6_pm_ccm_init(ccm_compat: *const core::ffi::c_char);
    pub fn imx6q_pm_init();
    pub fn imx6dl_pm_init();
    pub fn imx6sl_pm_init();
    pub fn imx6sx_pm_init();
    pub fn imx6ul_pm_init();
    pub fn imx7ulp_pm_init();
}

#[cfg(feature = "CONFIG_PM")]
extern "C" {
    pub fn imx51_pm_init();
    pub fn imx53_pm_init();
}

#[cfg(not(feature = "CONFIG_PM"))]
pub fn imx51_pm_init() {}
#[cfg(not(feature = "CONFIG_PM"))]
pub fn imx53_pm_init() {}

#[cfg(feature = "CONFIG_NEON")]
extern "C" {
    pub fn mx51_neon_fixup() -> i32;
}

#[cfg(not(feature = "CONFIG_NEON"))]
pub fn mx51_neon_fixup() -> i32 { 0 }

#[cfg(feature = "CONFIG_CACHE_L2X0")]
extern "C" {
    pub fn imx_init_l2cache();
}

#[cfg(not(feature = "CONFIG_CACHE_L2X0"))]
pub fn imx_init_l2cache() {}

extern "C" {
    pub static imx_smp_ops: smp_operations;
    pub static imx7_smp_ops: smp_operations;
    pub static ls1021a_smp_ops: smp_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
