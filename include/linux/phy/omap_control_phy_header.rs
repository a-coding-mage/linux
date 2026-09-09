/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * omap_control_phy.h - Header file for the PHY part of control module.
 *
 * Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com
 * Author: Kishon Vijay Abraham I <kishon@ti.com>
 */

// C header guard: __OMAP_CONTROL_PHY_H__

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum omap_control_phy_type {
    OMAP_CTRL_TYPE_OTGHS = 1, // Mailbox OTGHS_CONTROL
    OMAP_CTRL_TYPE_USB2,      // USB2_PHY, power down in CONTROL_DEV_CONF
    OMAP_CTRL_TYPE_PIPE3,     // PIPE3 PHY, DPLL & seperate Rx/Tx power
    OMAP_CTRL_TYPE_PCIE,      // RX TX control of ACSPCIE
    OMAP_CTRL_TYPE_DRA7USB2,  // USB2 PHY, power and power_aux e.g. DRA7
    OMAP_CTRL_TYPE_AM437USB2, // USB2 PHY, power e.g. AM437x
}

#[repr(C)]
pub struct omap_control_phy {
    pub dev: *mut device,

    pub otghs_control: *mut u32,
    pub power: *mut u32,
    pub power_aux: *mut u32,
    pub pcie_pcs: *mut u32,

    pub sys_clk: *mut clk,

    pub type_: omap_control_phy_type,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum omap_control_usb_mode {
    USB_MODE_UNDEFINED = 0,
    USB_MODE_HOST,
    USB_MODE_DEVICE,
    USB_MODE_DISCONNECT,
}

pub const OMAP_CTRL_DEV_PHY_PD: u32 = 1u32 << 0;

pub const OMAP_CTRL_DEV_AVALID: u32 = 1u32 << 0;
pub const OMAP_CTRL_DEV_BVALID: u32 = 1u32 << 1;
pub const OMAP_CTRL_DEV_VBUSVALID: u32 = 1u32 << 2;
pub const OMAP_CTRL_DEV_SESSEND: u32 = 1u32 << 3;
pub const OMAP_CTRL_DEV_IDDIG: u32 = 1u32 << 4;

pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_CMD_MASK: u32 = 0x003FC000;
pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_CMD_SHIFT: u32 = 0xE;

pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_FREQ_MASK: u32 = 0xFFC00000;
pub const OMAP_CTRL_PIPE3_PHY_PWRCTL_CLK_FREQ_SHIFT: u32 = 0x16;

pub const OMAP_CTRL_PIPE3_PHY_TX_RX_POWERON: u32 = 0x3;
pub const OMAP_CTRL_PIPE3_PHY_TX_RX_POWEROFF: u32 = 0x0;

pub const OMAP_CTRL_PCIE_PCS_MASK: u32 = 0xff;
pub const OMAP_CTRL_PCIE_PCS_DELAY_COUNT_SHIFT: u32 = 16;

pub const OMAP_CTRL_USB2_PHY_PD: u32 = 1u32 << 28;

pub const AM437X_CTRL_USB2_PHY_PD: u32 = 1u32 << 0;
pub const AM437X_CTRL_USB2_OTG_PD: u32 = 1u32 << 1;
pub const AM437X_CTRL_USB2_OTGVDET_EN: u32 = 1u32 << 19;
pub const AM437X_CTRL_USB2_OTGSESSEND_EN: u32 = 1u32 << 20;

// The following declarations are selected by the C build-time condition
// IS_ENABLED(CONFIG_OMAP_CONTROL_PHY).
#[cfg(feature = "CONFIG_OMAP_CONTROL_PHY")]
extern "C" {
    pub fn omap_control_phy_power(dev: *mut device, on: i32);
    pub fn omap_control_usb_set_mode(dev: *mut device, mode: omap_control_usb_mode);
    pub fn omap_control_pcie_pcs(dev: *mut device, delay: u8);
}

#[cfg(not(feature = "CONFIG_OMAP_CONTROL_PHY"))]
#[inline]
pub unsafe fn omap_control_phy_power(_dev: *mut device, _on: i32) {}

#[cfg(not(feature = "CONFIG_OMAP_CONTROL_PHY"))]
#[inline]
pub unsafe fn omap_control_usb_set_mode(_dev: *mut device, _mode: omap_control_usb_mode) {}

#[cfg(not(feature = "CONFIG_OMAP_CONTROL_PHY"))]
#[inline]
pub unsafe fn omap_control_pcie_pcs(_dev: *mut device, _delay: u8) {}

// External C types supplied by other headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
