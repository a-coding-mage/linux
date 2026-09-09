// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010 Google, Inc.
 */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct tegra_phy_soc_config {
    pub utmi_pll_config_in_car_module: bool,
    pub has_hostpc: bool,
    pub requires_usbmode_setup: bool,
    pub requires_extra_tuning_parameters: bool,
    pub requires_pmc_ao_power_up: bool,
    pub uhsic_registers_offset: u32,
    pub uhsic_tx_rtune: u32,
    pub uhsic_pts_value: u32,
    pub portsc1_offset: u32,
}

#[repr(C)]
pub struct tegra_utmip_config {
    pub hssync_start_delay: u8,
    pub elastic_limit: u8,
    pub idle_wait_delay: u8,
    pub term_range_adj: u8,
    pub xcvr_setup_use_fuses: bool,
    pub xcvr_setup: u8,
    pub xcvr_lsfslew: u8,
    pub xcvr_lsrslew: u8,
    pub xcvr_hsslew: u8,
    pub hssquelch_level: u8,
    pub hsdiscon_level: u8,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum tegra_usb_phy_port_speed {
    TEGRA_USB_PHY_PORT_SPEED_FULL = 0,
    TEGRA_USB_PHY_PORT_SPEED_LOW,
    TEGRA_USB_PHY_PORT_SPEED_HIGH,
}

// Opaque type declared by the surrounding translation unit.
pub struct tegra_xtal_freq;

#[repr(C)]
pub struct tegra_usb_phy {
    pub irq: i32,
    pub instance: i32,
    pub freq: *const tegra_xtal_freq,
    pub regs: *mut c_void,
    pub pad_regs: *mut c_void,
    pub clk: *mut clk,
    pub pll_u: *mut clk,
    pub pad_clk: *mut clk,
    pub vbus: *mut regulator,
    pub pmc_regmap: *mut regmap,
    pub mode: usb_dr_mode,
    pub config: *mut c_void,
    pub soc_config: *const tegra_phy_soc_config,
    pub ulpi: *mut usb_phy,
    pub u_phy: usb_phy,
    pub is_legacy_phy: bool,
    pub phy_type: usb_phy_interface,
    pub reset_gpio: *mut gpio_desc,
    pub pad_rst: *mut reset_control,
    pub wakeup_enabled: bool,
    pub pad_wakeup: bool,
    pub powered_on: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
