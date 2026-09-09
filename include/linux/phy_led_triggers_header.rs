/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2016 National Instruments Corp.
 */

/*
 * The declarations below are enabled when CONFIG_LED_TRIGGER_PHY is set.
 * The referenced kernel types and constants are supplied by their respective
 * dependencies.
 */

#[repr(C)]
pub struct phy_device {
    _private: [u8; 0],
}

#[cfg(CONFIG_LED_TRIGGER_PHY)]
pub const PHY_LED_TRIGGER_SPEED_SUFFIX_SIZE: usize = 11;

#[cfg(CONFIG_LED_TRIGGER_PHY)]
pub const PHY_LINK_LED_TRIGGER_NAME_SIZE: usize =
    MII_BUS_ID_SIZE + core::mem::size_of::<mdio_device>() + PHY_LED_TRIGGER_SPEED_SUFFIX_SIZE;

#[cfg(CONFIG_LED_TRIGGER_PHY)]
#[repr(C)]
pub struct phy_led_trigger {
    pub trigger: led_trigger,
    pub name: [::core::ffi::c_char; PHY_LINK_LED_TRIGGER_NAME_SIZE],
    pub speed: ::core::ffi::c_uint,
}

#[cfg(CONFIG_LED_TRIGGER_PHY)]
unsafe extern "C" {
    pub fn phy_led_triggers_register(phy: *mut phy_device) -> ::core::ffi::c_int;
    pub fn phy_led_triggers_unregister(phy: *mut phy_device);
    pub fn phy_led_trigger_change_speed(phy: *mut phy_device);
}

#[cfg(not(CONFIG_LED_TRIGGER_PHY))]
#[inline]
pub unsafe fn phy_led_triggers_register(_phy: *mut phy_device) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_LED_TRIGGER_PHY))]
#[inline]
pub unsafe fn phy_led_triggers_unregister(_phy: *mut phy_device) {}

#[cfg(not(CONFIG_LED_TRIGGER_PHY))]
#[inline]
pub unsafe fn phy_led_trigger_change_speed(_phy: *mut phy_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
