/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014, Fuzhou Rockchip Electronics Co., Ltd
 * Author: Tony Xie <tony.xie@rock-chips.com>
 */

unsafe extern "C" {
    pub static mut rkpm_bootdata_cpusp: ::core::ffi::c_ulong;
    pub static mut rkpm_bootdata_cpu_code: ::core::ffi::c_ulong;
    pub static mut rkpm_bootdata_l2ctlr_f: ::core::ffi::c_ulong;
    pub static mut rkpm_bootdata_l2ctlr: ::core::ffi::c_ulong;
    pub static mut rkpm_bootdata_ddr_code: ::core::ffi::c_ulong;
    pub static mut rkpm_bootdata_ddr_data: ::core::ffi::c_ulong;
    pub static mut rk3288_bootram_sz: ::core::ffi::c_ulong;

    pub fn rockchip_slp_cpu_resume();
}

// CONFIG_PM_SLEEP controls whether this function is externally defined.
#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" {
    pub fn rockchip_suspend_init();
}

#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
#[inline]
pub fn rockchip_suspend_init() {}

/* following is rk3288 defined */
pub const RK3288_PMU_WAKEUP_CFG0: u32 = 0x00;
pub const RK3288_PMU_WAKEUP_CFG1: u32 = 0x04;
pub const RK3288_PMU_PWRMODE_CON: u32 = 0x18;
pub const RK3288_PMU_OSC_CNT: u32 = 0x20;
pub const RK3288_PMU_PLL_CNT: u32 = 0x24;
pub const RK3288_PMU_STABL_CNT: u32 = 0x28;
pub const RK3288_PMU_DDR0IO_PWRON_CNT: u32 = 0x2c;
pub const RK3288_PMU_DDR1IO_PWRON_CNT: u32 = 0x30;
pub const RK3288_PMU_CORE_PWRDWN_CNT: u32 = 0x34;
pub const RK3288_PMU_CORE_PWRUP_CNT: u32 = 0x38;
pub const RK3288_PMU_GPU_PWRDWN_CNT: u32 = 0x3c;
pub const RK3288_PMU_GPU_PWRUP_CNT: u32 = 0x40;
pub const RK3288_PMU_WAKEUP_RST_CLR_CNT: u32 = 0x44;
pub const RK3288_PMU_PWRMODE_CON1: u32 = 0x90;

pub const RK3288_SGRF_SOC_CON0: u32 = 0x0000;
pub const RK3288_SGRF_FAST_BOOT_ADDR: u32 = 0x0120;
pub const SGRF_PCLK_WDT_GATE: u32 = 1 << 6;
pub const SGRF_PCLK_WDT_GATE_WRITE: u32 = 1 << 22;
pub const SGRF_FAST_BOOT_EN: u32 = 1 << 8;
pub const SGRF_FAST_BOOT_EN_WRITE: u32 = 1 << 24;

pub const RK3288_SGRF_CPU_CON0: u32 = 0x40;
pub const SGRF_DAPDEVICEEN: u32 = 1 << 0;
pub const SGRF_DAPDEVICEEN_WRITE: u32 = 1 << 16;

/* PMU_WAKEUP_CFG1 bits */
pub const PMU_ARMINT_WAKEUP_EN: u32 = 1 << 0;
pub const PMU_GPIOINT_WAKEUP_EN: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rk3288_pwr_mode_con {
    PMU_PWR_MODE_EN = 0,
    PMU_CLK_CORE_SRC_GATE_EN,
    PMU_GLOBAL_INT_DISABLE,
    PMU_L2FLUSH_EN,
    PMU_BUS_PD_EN,
    PMU_A12_0_PD_EN,
    PMU_SCU_EN,
    PMU_PLL_PD_EN,
    PMU_CHIP_PD_EN, /* POWER OFF PIN ENABLE */
    PMU_PWROFF_COMB,
    PMU_ALIVE_USE_LF,
    PMU_PMU_USE_LF,
    PMU_OSC_24M_DIS,
    PMU_INPUT_CLAMP_EN,
    PMU_WAKEUP_RESET_EN,
    PMU_SREF0_ENTER_EN,
    PMU_SREF1_ENTER_EN,
    PMU_DDR0IO_RET_EN,
    PMU_DDR1IO_RET_EN,
    PMU_DDR0_GATING_EN,
    PMU_DDR1_GATING_EN,
    PMU_DDR0IO_RET_DE_REQ,
    PMU_DDR1IO_RET_DE_REQ,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rk3288_pwr_mode_con1 {
    PMU_CLR_BUS = 0,
    PMU_CLR_CORE,
    PMU_CLR_CPUP,
    PMU_CLR_ALIVE,
    PMU_CLR_DMA,
    PMU_CLR_PERI,
    PMU_CLR_GPU,
    PMU_CLR_VIDEO,
    PMU_CLR_HEVC,
    PMU_CLR_VIO,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
