/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2015, Heiner Kallweit <hkallweit1@gmail.com>
 */

/* C conditional: IS_ENABLED(CONFIG_BT_LEDS). */
#[cfg(feature = "CONFIG_BT_LEDS")]
extern "C" {
    pub fn hci_leds_update_powered(hdev: *mut hci_dev, enabled: bool);
    pub fn hci_leds_init(hdev: *mut hci_dev);

    pub fn bt_leds_init();
    pub fn bt_leds_cleanup();
}

#[cfg(not(feature = "CONFIG_BT_LEDS"))]
#[inline]
pub unsafe fn hci_leds_update_powered(_hdev: *mut hci_dev, _enabled: bool) {}

#[cfg(not(feature = "CONFIG_BT_LEDS"))]
#[inline]
pub unsafe fn hci_leds_init(_hdev: *mut hci_dev) {}

#[cfg(not(feature = "CONFIG_BT_LEDS"))]
#[inline]
pub unsafe fn bt_leds_init() {}

#[cfg(not(feature = "CONFIG_BT_LEDS"))]
#[inline]
pub unsafe fn bt_leds_cleanup() {}

/* Opaque declaration supplied by the surrounding Bluetooth implementation. */
#[repr(C)]
pub struct hci_dev {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
