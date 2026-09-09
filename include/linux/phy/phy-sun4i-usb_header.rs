/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 Hans de Goede <hdegoede@redhat.com>
 */

// Dependency provided by "phy.h" in the original header.
#[repr(C)]
pub struct phy {
    _private: [u8; 0],
}

/**
 * sun4i_usb_phy_set_squelch_detect() - Enable/disable squelch detect
 * @phy: reference to a sun4i usb phy
 * @enabled: whether to enable or disable squelch detect
 */
unsafe extern "C" {
    pub fn sun4i_usb_phy_set_squelch_detect(phy: *mut phy, enabled: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
