/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * External Connector (extcon) framework
 * - linux/include/linux/extcon.h for extcon consumer device driver.
 *
 * Copyright (C) 2015 Samsung Electronics
 * Author: Chanwoo Choi <cw00.choi@samsung.com>
 *
 * Copyright (C) 2012 Samsung Electronics
 * Author: Donggeun Kim <dg77.kim@samsung.com>
 * Author: MyungJoo Ham <myungjoo.ham@samsung.com>
 *
 * based on switch class driver
 * Copyright (C) 2008 Google, Inc.
 * Author: Mike Lockwood <lockwood@android.com>
 */

/* Dependency supplied by the surrounding kernel translation. */

/* Define the type of supported external connectors. */
pub const EXTCON_TYPE_USB: u32 = 1 << 0; // USB connector
pub const EXTCON_TYPE_CHG: u32 = 1 << 1; // Charger connector
pub const EXTCON_TYPE_JACK: u32 = 1 << 2; // Jack connector
pub const EXTCON_TYPE_DISP: u32 = 1 << 3; // Display connector
pub const EXTCON_TYPE_MISC: u32 = 1 << 4; // Miscellaneous connector

/* Define the unique id of supported external connectors. */
pub const EXTCON_NONE: u32 = 0;
pub const EXTCON_USB: u32 = 1;
pub const EXTCON_USB_HOST: u32 = 2;

pub const EXTCON_CHG_USB_SDP: u32 = 5; // Standard Downstream Port
pub const EXTCON_CHG_USB_DCP: u32 = 6; // Dedicated Charging Port
pub const EXTCON_CHG_USB_CDP: u32 = 7; // Charging Downstream Port
pub const EXTCON_CHG_USB_ACA: u32 = 8; // Accessory Charger Adapter
pub const EXTCON_CHG_USB_FAST: u32 = 9;
pub const EXTCON_CHG_USB_SLOW: u32 = 10;
pub const EXTCON_CHG_WPT: u32 = 11; // Wireless Power Transfer
pub const EXTCON_CHG_USB_PD: u32 = 12; // USB Power Delivery

pub const EXTCON_JACK_MICROPHONE: u32 = 20;
pub const EXTCON_JACK_HEADPHONE: u32 = 21;
pub const EXTCON_JACK_LINE_IN: u32 = 22;
pub const EXTCON_JACK_LINE_OUT: u32 = 23;
pub const EXTCON_JACK_VIDEO_IN: u32 = 24;
pub const EXTCON_JACK_VIDEO_OUT: u32 = 25;
pub const EXTCON_JACK_SPDIF_IN: u32 = 26; // Sony Philips Digital InterFace
pub const EXTCON_JACK_SPDIF_OUT: u32 = 27;

pub const EXTCON_DISP_HDMI: u32 = 40; // High-Definition Multimedia Interface
pub const EXTCON_DISP_MHL: u32 = 41; // Mobile High-Definition Link
pub const EXTCON_DISP_DVI: u32 = 42; // Digital Visual Interface
pub const EXTCON_DISP_VGA: u32 = 43; // Video Graphics Array
pub const EXTCON_DISP_DP: u32 = 44; // Display Port
pub const EXTCON_DISP_HMD: u32 = 45; // Head-Mounted Display
pub const EXTCON_DISP_CVBS: u32 = 46; // Composite Video Broadcast Signal
pub const EXTCON_DISP_EDP: u32 = 47; // Embedded Display Port

pub const EXTCON_DOCK: u32 = 60;
pub const EXTCON_JIG: u32 = 61;
pub const EXTCON_MECHANICAL: u32 = 62;
pub const EXTCON_NUM: u32 = 63;

pub const EXTCON_PROP_USB_VBUS: u32 = 0;
pub const EXTCON_PROP_USB_TYPEC_POLARITY: u32 = 1;
pub const EXTCON_PROP_USB_SS: u32 = 2;
pub const EXTCON_PROP_USB_MIN: u32 = 0;
pub const EXTCON_PROP_USB_MAX: u32 = 2;
pub const EXTCON_PROP_USB_CNT: u32 = EXTCON_PROP_USB_MAX - EXTCON_PROP_USB_MIN + 1;

pub const EXTCON_PROP_CHG_MIN: u32 = 50;
pub const EXTCON_PROP_CHG_MAX: u32 = 50;
pub const EXTCON_PROP_CHG_CNT: u32 = EXTCON_PROP_CHG_MAX - EXTCON_PROP_CHG_MIN + 1;
pub const EXTCON_PROP_JACK_MIN: u32 = 100;
pub const EXTCON_PROP_JACK_MAX: u32 = 100;
pub const EXTCON_PROP_JACK_CNT: u32 = EXTCON_PROP_JACK_MAX - EXTCON_PROP_JACK_MIN + 1;
pub const EXTCON_PROP_DISP_HPD: u32 = 150;
pub const EXTCON_PROP_DISP_MIN: u32 = 150;
pub const EXTCON_PROP_DISP_MAX: u32 = 151;
pub const EXTCON_PROP_DISP_CNT: u32 = EXTCON_PROP_DISP_MAX - EXTCON_PROP_DISP_MIN + 1;

#[repr(C)]
pub union extcon_property_value {
    pub intval: i32,
}

pub struct extcon_dev;

/* CONFIG_EXTCON conditional declarations. */
#[cfg(CONFIG_EXTCON)]
extern "C" {
    pub fn extcon_get_state(edev: *mut extcon_dev, id: u32) -> i32;
    pub fn extcon_get_property(edev: *mut extcon_dev, id: u32, prop: u32, prop_val: *mut extcon_property_value) -> i32;
    pub fn extcon_get_property_capability(edev: *mut extcon_dev, id: u32, prop: u32) -> i32;
    pub fn extcon_register_notifier(edev: *mut extcon_dev, id: u32, nb: *mut notifier_block) -> i32;
    pub fn extcon_unregister_notifier(edev: *mut extcon_dev, id: u32, nb: *mut notifier_block) -> i32;
    pub fn devm_extcon_register_notifier(dev: *mut device, edev: *mut extcon_dev, id: u32, nb: *mut notifier_block) -> i32;
    pub fn devm_extcon_unregister_notifier(dev: *mut device, edev: *mut extcon_dev, id: u32, nb: *mut notifier_block);
    pub fn extcon_register_notifier_all(edev: *mut extcon_dev, nb: *mut notifier_block) -> i32;
    pub fn extcon_unregister_notifier_all(edev: *mut extcon_dev, nb: *mut notifier_block) -> i32;
    pub fn devm_extcon_register_notifier_all(dev: *mut device, edev: *mut extcon_dev, nb: *mut notifier_block) -> i32;
    pub fn devm_extcon_unregister_notifier_all(dev: *mut device, edev: *mut extcon_dev, nb: *mut notifier_block);
    pub fn extcon_get_extcon_dev(extcon_name: *const i8) -> *mut extcon_dev;
    pub fn extcon_find_edev_by_node(node: *mut device_node) -> *mut extcon_dev;
    pub fn extcon_get_edev_by_phandle(dev: *mut device, index: i32) -> *mut extcon_dev;
    pub fn extcon_get_edev_name(edev: *mut extcon_dev) -> *const i8;
}

#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_get_state(_edev: *mut extcon_dev, _id: u32) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_get_property(_edev: *mut extcon_dev, _id: u32, _prop: u32, _prop_val: *mut extcon_property_value) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_get_property_capability(_edev: *mut extcon_dev, _id: u32, _prop: u32) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_register_notifier(_edev: *mut extcon_dev, _id: u32, _nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_unregister_notifier(_edev: *mut extcon_dev, _id: u32, _nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn devm_extcon_register_notifier(_dev: *mut device, _edev: *mut extcon_dev, _id: u32, _nb: *mut notifier_block) -> i32 { -ENOSYS }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn devm_extcon_unregister_notifier(_dev: *mut device, _edev: *mut extcon_dev, _id: u32, _nb: *mut notifier_block) {}
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_register_notifier_all(_edev: *mut extcon_dev, _nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_unregister_notifier_all(_edev: *mut extcon_dev, _nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn devm_extcon_register_notifier_all(_dev: *mut device, _edev: *mut extcon_dev, _nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn devm_extcon_unregister_notifier_all(_dev: *mut device, _edev: *mut extcon_dev, _nb: *mut notifier_block) {}
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_get_extcon_dev(_extcon_name: *const i8) -> *mut extcon_dev { core::ptr::null_mut() }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_find_edev_by_node(_node: *mut device_node) -> *mut extcon_dev { core::ptr::null_mut() }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_get_edev_by_phandle(_dev: *mut device, _index: i32) -> *mut extcon_dev { core::ptr::null_mut() }
#[cfg(not(CONFIG_EXTCON))]
pub unsafe fn extcon_get_edev_name(_edev: *mut extcon_dev) -> *const i8 { core::ptr::null() }

#[repr(C)]
pub struct extcon_specific_cable_nb {
    pub user_nb: *mut notifier_block,
    pub cable_index: i32,
    pub edev: *mut extcon_dev,
    pub previous_value: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
