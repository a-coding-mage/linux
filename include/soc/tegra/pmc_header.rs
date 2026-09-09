/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010 Google, Inc
 * Copyright (c) 2014 NVIDIA Corporation
 *
 * Author:
 *	Colin Cross <ccross@google.com>
 */

/* Translated from soc/tegra/pmc.h. C includes and configuration conditions
 * are represented by the declarations and cfg-related comments below. */

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tegra_pmc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type tegra_suspend_mode = i32;
pub const TEGRA_SUSPEND_NONE: tegra_suspend_mode = 0;

pub const TEGRA_POWERGATE_CPU: i32 = 0;
pub const TEGRA_POWERGATE_3D: i32 = 1;
pub const TEGRA_POWERGATE_VENC: i32 = 2;
pub const TEGRA_POWERGATE_PCIE: i32 = 3;
pub const TEGRA_POWERGATE_VDEC: i32 = 4;
pub const TEGRA_POWERGATE_L2: i32 = 5;
pub const TEGRA_POWERGATE_MPE: i32 = 6;
pub const TEGRA_POWERGATE_HEG: i32 = 7;
pub const TEGRA_POWERGATE_SATA: i32 = 8;
pub const TEGRA_POWERGATE_CPU1: i32 = 9;
pub const TEGRA_POWERGATE_CPU2: i32 = 10;
pub const TEGRA_POWERGATE_CPU3: i32 = 11;
pub const TEGRA_POWERGATE_CELP: i32 = 12;
pub const TEGRA_POWERGATE_3D1: i32 = 13;
pub const TEGRA_POWERGATE_CPU0: i32 = 14;
pub const TEGRA_POWERGATE_C0NC: i32 = 15;
pub const TEGRA_POWERGATE_C1NC: i32 = 16;
pub const TEGRA_POWERGATE_SOR: i32 = 17;
pub const TEGRA_POWERGATE_DIS: i32 = 18;
pub const TEGRA_POWERGATE_DISB: i32 = 19;
pub const TEGRA_POWERGATE_XUSBA: i32 = 20;
pub const TEGRA_POWERGATE_XUSBB: i32 = 21;
pub const TEGRA_POWERGATE_XUSBC: i32 = 22;
pub const TEGRA_POWERGATE_VIC: i32 = 23;
pub const TEGRA_POWERGATE_IRAM: i32 = 24;
pub const TEGRA_POWERGATE_NVDEC: i32 = 25;
pub const TEGRA_POWERGATE_NVJPG: i32 = 26;
pub const TEGRA_POWERGATE_AUD: i32 = 27;
pub const TEGRA_POWERGATE_DFD: i32 = 28;
pub const TEGRA_POWERGATE_VE2: i32 = 29;
pub const TEGRA_POWERGATE_MAX: i32 = TEGRA_POWERGATE_VE2;
pub const TEGRA_POWERGATE_3D0: i32 = TEGRA_POWERGATE_3D;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tegra_io_pad {
    TEGRA_IO_PAD_AUDIO,
    TEGRA_IO_PAD_AUDIO_HV,
    TEGRA_IO_PAD_BB,
    TEGRA_IO_PAD_CAM,
    TEGRA_IO_PAD_COMP,
    TEGRA_IO_PAD_CONN,
    TEGRA_IO_PAD_CSIA,
    TEGRA_IO_PAD_CSIB,
    TEGRA_IO_PAD_CSIC,
    TEGRA_IO_PAD_CSID,
    TEGRA_IO_PAD_CSIE,
    TEGRA_IO_PAD_CSIF,
    TEGRA_IO_PAD_CSIG,
    TEGRA_IO_PAD_CSIH,
    TEGRA_IO_PAD_DAP3,
    TEGRA_IO_PAD_DAP5,
    TEGRA_IO_PAD_DBG,
    TEGRA_IO_PAD_DEBUG_NONAO,
    TEGRA_IO_PAD_DMIC,
    TEGRA_IO_PAD_DMIC_HV,
    TEGRA_IO_PAD_DP,
    TEGRA_IO_PAD_DSI,
    TEGRA_IO_PAD_DSIB,
    TEGRA_IO_PAD_DSIC,
    TEGRA_IO_PAD_DSID,
    TEGRA_IO_PAD_EDP,
    TEGRA_IO_PAD_EMMC,
    TEGRA_IO_PAD_EMMC2,
    TEGRA_IO_PAD_EQOS,
    TEGRA_IO_PAD_GPIO,
    TEGRA_IO_PAD_GP_PWM2,
    TEGRA_IO_PAD_GP_PWM3,
    TEGRA_IO_PAD_HDMI,
    TEGRA_IO_PAD_HDMI_DP0,
    TEGRA_IO_PAD_HDMI_DP1,
    TEGRA_IO_PAD_HDMI_DP2,
    TEGRA_IO_PAD_HDMI_DP3,
    TEGRA_IO_PAD_HSIC,
    TEGRA_IO_PAD_HV,
    TEGRA_IO_PAD_LVDS,
    TEGRA_IO_PAD_MIPI_BIAS,
    TEGRA_IO_PAD_NAND,
    TEGRA_IO_PAD_PEX_BIAS,
    TEGRA_IO_PAD_PEX_CLK_BIAS,
    TEGRA_IO_PAD_PEX_CLK1,
    TEGRA_IO_PAD_PEX_CLK2,
    TEGRA_IO_PAD_PEX_CLK3,
    TEGRA_IO_PAD_PEX_CLK_2_BIAS,
    TEGRA_IO_PAD_PEX_CLK_2,
    TEGRA_IO_PAD_PEX_CNTRL,
    TEGRA_IO_PAD_PEX_CTL2,
    TEGRA_IO_PAD_PEX_L0_RST,
    TEGRA_IO_PAD_PEX_L1_RST,
    TEGRA_IO_PAD_PEX_L5_RST,
    TEGRA_IO_PAD_PWR_CTL,
    TEGRA_IO_PAD_SDMMC1,
    TEGRA_IO_PAD_SDMMC1_HV,
    TEGRA_IO_PAD_SDMMC2,
    TEGRA_IO_PAD_SDMMC2_HV,
    TEGRA_IO_PAD_SDMMC3,
    TEGRA_IO_PAD_SDMMC3_HV,
    TEGRA_IO_PAD_SDMMC4,
    TEGRA_IO_PAD_SOC_GPIO10,
    TEGRA_IO_PAD_SOC_GPIO12,
    TEGRA_IO_PAD_SOC_GPIO13,
    TEGRA_IO_PAD_SOC_GPIO53,
    TEGRA_IO_PAD_SPI,
    TEGRA_IO_PAD_SPI_HV,
    TEGRA_IO_PAD_SYS_DDC,
    TEGRA_IO_PAD_UART,
    TEGRA_IO_PAD_UART4,
    TEGRA_IO_PAD_UART5,
    TEGRA_IO_PAD_UFS,
    TEGRA_IO_PAD_USB0,
    TEGRA_IO_PAD_USB1,
    TEGRA_IO_PAD_USB2,
    TEGRA_IO_PAD_USB3,
    TEGRA_IO_PAD_USB_BIAS,
    TEGRA_IO_PAD_AO_HV,
}

/* CONFIG_SOC_TEGRA_PMC declarations. */
extern "C" {
    pub fn devm_tegra_pmc_get(dev: *mut device) -> *mut tegra_pmc;
    pub fn tegra_pmc_powergate_power_on(pmc: *mut tegra_pmc, id: u32) -> i32;
    pub fn tegra_pmc_powergate_power_off(pmc: *mut tegra_pmc, id: u32) -> i32;
    pub fn tegra_pmc_powergate_remove_clamping(pmc: *mut tegra_pmc, id: u32) -> i32;
    pub fn tegra_pmc_powergate_sequence_power_up(pmc: *mut tegra_pmc, id: u32,
        clk: *mut clk, rst: *mut reset_control) -> i32;
    pub fn tegra_pmc_io_pad_power_enable(pmc: *mut tegra_pmc, id: tegra_io_pad) -> i32;
    pub fn tegra_pmc_io_pad_power_disable(pmc: *mut tegra_pmc, id: tegra_io_pad) -> i32;
    pub fn tegra_pmc_cpu_is_powered(cpuid: u32) -> bool;
    pub fn tegra_pmc_cpu_power_on(cpuid: u32) -> i32;
    pub fn tegra_pmc_cpu_remove_clamping(cpuid: u32) -> i32;
    pub fn tegra_pmc_core_domain_state_synced() -> bool;
    pub fn tegra_pmc_get_suspend_mode() -> tegra_suspend_mode;
    pub fn tegra_pmc_set_suspend_mode(mode: tegra_suspend_mode);
    pub fn tegra_pmc_enter_suspend_mode(mode: tegra_suspend_mode);
}

/* CONFIG_SOC_TEGRA_PMC-disabled inline implementations return -ENOSYS. */
pub const ENOSYS: i32 = 38;
#[inline] pub unsafe fn devm_tegra_pmc_get_disabled(_: *mut device) -> *mut tegra_pmc { (-ENOSYS as isize) as *mut tegra_pmc }
#[inline] pub unsafe fn tegra_pmc_powergate_power_on_disabled(_: *mut tegra_pmc, _: u32) -> i32 { -ENOSYS }
#[inline] pub unsafe fn tegra_pmc_powergate_power_off_disabled(_: *mut tegra_pmc, _: u32) -> i32 { -ENOSYS }
#[inline] pub unsafe fn tegra_pmc_powergate_remove_clamping_disabled(_: *mut tegra_pmc, _: u32) -> i32 { -ENOSYS }
#[inline] pub unsafe fn tegra_pmc_io_pad_power_enable_disabled(_: *mut tegra_pmc, _: tegra_io_pad) -> i32 { -ENOSYS }
#[inline] pub unsafe fn tegra_pmc_io_pad_power_disable_disabled(_: *mut tegra_pmc, _: tegra_io_pad) -> i32 { -ENOSYS }
#[inline] pub fn tegra_pmc_core_domain_state_synced_disabled() -> bool { false }
#[inline] pub fn tegra_pmc_get_suspend_mode_disabled() -> tegra_suspend_mode { TEGRA_SUSPEND_NONE }
#[inline] pub fn tegra_pmc_set_suspend_mode_disabled(_: tegra_suspend_mode) {}
#[inline] pub fn tegra_pmc_enter_suspend_mode_disabled(_: tegra_suspend_mode) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
