/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016-2022, NVIDIA CORPORATION.  All rights reserved.
 */

#[repr(C)]
pub struct tegra_xusb_padctl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct phy {
    _private: [u8; 0],
}

pub type usb_device_speed = core::ffi::c_int;

extern "C" {
    pub fn tegra_xusb_padctl_get(dev: *mut device) -> *mut tegra_xusb_padctl;
    pub fn tegra_xusb_padctl_put(padctl: *mut tegra_xusb_padctl);

    pub fn tegra_xusb_padctl_usb3_save_context(
        padctl: *mut tegra_xusb_padctl,
        port: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_hsic_set_idle(
        padctl: *mut tegra_xusb_padctl,
        port: core::ffi::c_uint,
        idle: bool,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_usb3_set_lfps_detect(
        padctl: *mut tegra_xusb_padctl,
        port: core::ffi::c_uint,
        enable: bool,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_set_vbus_override(
        padctl: *mut tegra_xusb_padctl,
        val: bool,
    ) -> core::ffi::c_int;
    pub fn tegra_phy_xusb_utmi_pad_power_on(phy: *mut phy);
    pub fn tegra_phy_xusb_utmi_pad_power_down(phy: *mut phy);
    pub fn tegra_phy_xusb_utmi_port_reset(phy: *mut phy) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_get_usb3_companion(
        padctl: *mut tegra_xusb_padctl,
        port: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_get_port_number(phy: *mut phy) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_enable_phy_sleepwalk(
        padctl: *mut tegra_xusb_padctl,
        phy: *mut phy,
        speed: usb_device_speed,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_disable_phy_sleepwalk(
        padctl: *mut tegra_xusb_padctl,
        phy: *mut phy,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_enable_phy_wake(
        padctl: *mut tegra_xusb_padctl,
        phy: *mut phy,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_disable_phy_wake(
        padctl: *mut tegra_xusb_padctl,
        phy: *mut phy,
    ) -> core::ffi::c_int;
    pub fn tegra_xusb_padctl_remote_wake_detected(
        padctl: *mut tegra_xusb_padctl,
        phy: *mut phy,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
