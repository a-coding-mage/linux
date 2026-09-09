// SPDX-License-Identifier: GPL-2.0
/*
 * Extcon charger detection driver for Intel Cherrytrail Whiskey Cove PMIC
 * Copyright (C) 2017 Hans de Goede <hdegoede@redhat.com>
 *
 * Based on various non upstream patches to support the CHT Whiskey Cove PMIC:
 * Copyright (C) 2013-2015 Intel Corporation. All rights reserved.
 */

// Kernel dependencies supplied externally.

const CHT_WC_PHYCTRL: u32 = 0x5e07;
const CHT_WC_CHGRCTRL0: u32 = 0x5e16;
const CHT_WC_CHGRCTRL0_CHGRRESET: u32 = 1 << 0;
const CHT_WC_CHGRCTRL0_EMRGCHREN: u32 = 1 << 1;
const CHT_WC_CHGRCTRL0_EXTCHRDIS: u32 = 1 << 2;
const CHT_WC_CHGRCTRL0_SWCONTROL: u32 = 1 << 3;
const CHT_WC_CHGRCTRL0_TTLCK: u32 = 1 << 4;
const CHT_WC_CHGRCTRL0_CCSM_OFF: u32 = 1 << 5;
const CHT_WC_CHGRCTRL0_DBPOFF: u32 = 1 << 6;
const CHT_WC_CHGRCTRL0_CHR_WDT_NOKICK: u32 = 1 << 7;
const CHT_WC_CHGRCTRL1: u32 = 0x5e17;
const CHT_WC_CHGRCTRL1_FUSB_INLMT_100: u32 = 1 << 0;
const CHT_WC_CHGRCTRL1_FUSB_INLMT_150: u32 = 1 << 1;
const CHT_WC_CHGRCTRL1_FUSB_INLMT_500: u32 = 1 << 2;
const CHT_WC_CHGRCTRL1_FUSB_INLMT_900: u32 = 1 << 3;
const CHT_WC_CHGRCTRL1_FUSB_INLMT_1500: u32 = 1 << 4;
const CHT_WC_CHGRCTRL1_FTEMP_EVENT: u32 = 1 << 5;
const CHT_WC_CHGRCTRL1_OTGMODE: u32 = 1 << 6;
const CHT_WC_CHGRCTRL1_DBPEN: u32 = 1 << 7;
const CHT_WC_USBSRC: u32 = 0x5e29;
const CHT_WC_USBSRC_STS_MASK: u32 = (1 << 2) - 1;
const CHT_WC_USBSRC_STS_SUCCESS: u32 = 2;
const CHT_WC_USBSRC_STS_FAIL: u32 = 3;
const CHT_WC_USBSRC_TYPE_SHIFT: u32 = 2;
const CHT_WC_USBSRC_TYPE_MASK: u32 = ((1 << 6) - 1) & !((1 << 2) - 1);
const CHT_WC_USBSRC_TYPE_NONE: u32 = 0;
const CHT_WC_USBSRC_TYPE_SDP: u32 = 1;
const CHT_WC_USBSRC_TYPE_DCP: u32 = 2;
const CHT_WC_USBSRC_TYPE_CDP: u32 = 3;
const CHT_WC_USBSRC_TYPE_ACA: u32 = 4;
const CHT_WC_USBSRC_TYPE_SE1: u32 = 5;
const CHT_WC_USBSRC_TYPE_MHL: u32 = 6;
const CHT_WC_USBSRC_TYPE_FLOATING: u32 = 7;
const CHT_WC_USBSRC_TYPE_OTHER: u32 = 8;
const CHT_WC_USBSRC_TYPE_DCP_EXTPHY: u32 = 9;
const CHT_WC_CHGDISCTRL: u32 = 0x5e2f;
const CHT_WC_CHGDISCTRL_OUT: u32 = 1 << 0;
/* 0 - open drain, 1 - regular push-pull output */
const CHT_WC_CHGDISCTRL_DRV: u32 = 1 << 4;
/* 0 - pin is controlled by SW, 1 - by HW */
const CHT_WC_CHGDISCTRL_FN: u32 = 1 << 6;
const CHT_WC_PWRSRC_IRQ: u32 = 0x6e03;
const CHT_WC_PWRSRC_IRQ_MASK: u32 = 0x6e0f;
const CHT_WC_PWRSRC_STS: u32 = 0x6e1e;
const CHT_WC_PWRSRC_VBUS: u32 = 1 << 0;
const CHT_WC_PWRSRC_DC: u32 = 1 << 1;
const CHT_WC_PWRSRC_BATT: u32 = 1 << 2;
const CHT_WC_PWRSRC_USBID_MASK: u32 = ((1 << 5) - 1) & !((1 << 3) - 1);
const CHT_WC_PWRSRC_USBID_SHIFT: u32 = 3;
const CHT_WC_PWRSRC_RID_ACA: u32 = 0;
const CHT_WC_PWRSRC_RID_GND: u32 = 1;
const CHT_WC_PWRSRC_RID_FLOAT: u32 = 2;
const CHT_WC_VBUS_GPIO_CTLO: u32 = 0x6e2d;
const CHT_WC_VBUS_GPIO_CTLO_OUTPUT: u32 = 1 << 0;
const CHT_WC_VBUS_GPIO_CTLO_DRV_OD: u32 = 1 << 4;
const CHT_WC_VBUS_GPIO_CTLO_DIR_OUT: u32 = 1 << 5;

#[repr(C)]
enum cht_wc_mux_select { MUX_SEL_PMIC = 0, MUX_SEL_SOC }

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut i32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_warn(dev: *mut device, fmt: *const u8, ...);
    fn extcon_set_state_sync(edev: *mut extcon_dev, cable: u32, state: bool) -> i32;
    fn regulator_enable(reg: *mut regulator) -> i32;
    fn regulator_disable(reg: *mut regulator) -> i32;
    fn usb_role_switch_set_role(sw: *mut usb_role_switch, role: usb_role) -> i32;
    fn power_supply_changed(psy: *mut power_supply);
    fn msleep(ms: u32);
    fn jiffies() -> u64;
}

#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct extcon_dev { _private: [u8; 0] }
#[repr(C)] struct usb_role_switch { _private: [u8; 0] }
#[repr(C)] struct regulator { _private: [u8; 0] }
#[repr(C)] struct power_supply { _private: [u8; 0] }
#[repr(C)] struct cht_wc_extcon_data {
    dev: *mut device, regmap: *mut regmap, edev: *mut extcon_dev,
    role_sw: *mut usb_role_switch, vbus_boost: *mut regulator,
    psy: *mut power_supply, usb_type: i32, previous_cable: u32,
    usb_host: bool, vbus_boost_enabled: bool,
}

unsafe fn cht_wc_extcon_get_id(_ext: *mut cht_wc_extcon_data, pwrsrc_sts: i32) -> i32 {
    match ((pwrsrc_sts as u32 & CHT_WC_PWRSRC_USBID_MASK) >> CHT_WC_PWRSRC_USBID_SHIFT) {
        CHT_WC_PWRSRC_RID_GND => INTEL_USB_ID_GND,
        CHT_WC_PWRSRC_RID_FLOAT => INTEL_USB_ID_FLOAT,
        /* See the C implementation: all practically available ACAs are treated as RID_A. */
        CHT_WC_PWRSRC_RID_ACA => INTEL_USB_RID_A,
        _ => INTEL_USB_ID_FLOAT,
    }
}

unsafe fn cht_wc_extcon_get_charger(ext: *mut cht_wc_extcon_data, ignore_errors: bool) -> i32 {
    let mut ret: i32; let mut usbsrc = 0; let mut status: u32;
    let timeout = jiffies() + msecs_to_jiffies(800);
    loop {
        ret = regmap_read((*ext).regmap, CHT_WC_USBSRC, &mut usbsrc);
        if ret != 0 { dev_err((*ext).dev, b"Error reading usbsrc: %d\0".as_ptr(), ret); return ret; }
        status = usbsrc as u32 & CHT_WC_USBSRC_STS_MASK;
        if status == CHT_WC_USBSRC_STS_SUCCESS || status == CHT_WC_USBSRC_STS_FAIL { break; }
        msleep(50);
        if !time_before(jiffies(), timeout) { break; }
    }
    if status != CHT_WC_USBSRC_STS_SUCCESS {
        if !ignore_errors { if status == CHT_WC_USBSRC_STS_FAIL { dev_warn((*ext).dev, b"Could not detect charger type\0".as_ptr()); } else { dev_warn((*ext).dev, b"Timeout detecting charger type\0".as_ptr()); } }
        usbsrc = (CHT_WC_USBSRC_TYPE_SDP << CHT_WC_USBSRC_TYPE_SHIFT) as i32;
    }
    match ((usbsrc as u32 & CHT_WC_USBSRC_TYPE_MASK) >> CHT_WC_USBSRC_TYPE_SHIFT) {
        CHT_WC_USBSRC_TYPE_SDP | CHT_WC_USBSRC_TYPE_FLOATING | CHT_WC_USBSRC_TYPE_OTHER => { (*ext).usb_type = POWER_SUPPLY_USB_TYPE_SDP; EXTCON_CHG_USB_SDP }
        CHT_WC_USBSRC_TYPE_CDP => { (*ext).usb_type = POWER_SUPPLY_USB_TYPE_CDP; EXTCON_CHG_USB_CDP }
        CHT_WC_USBSRC_TYPE_DCP | CHT_WC_USBSRC_TYPE_DCP_EXTPHY | CHT_WC_USBSRC_TYPE_MHL => { (*ext).usb_type = POWER_SUPPLY_USB_TYPE_DCP; EXTCON_CHG_USB_DCP }
        CHT_WC_USBSRC_TYPE_ACA => { (*ext).usb_type = POWER_SUPPLY_USB_TYPE_ACA; EXTCON_CHG_USB_ACA }
        _ => { (*ext).usb_type = POWER_SUPPLY_USB_TYPE_SDP; EXTCON_CHG_USB_SDP }
    }
}

unsafe fn cht_wc_extcon_set_phymux(ext: *mut cht_wc_extcon_data, state: u8) { let ret = regmap_write((*ext).regmap, CHT_WC_PHYCTRL, state as u32); if ret != 0 { dev_err((*ext).dev, b"Error writing phyctrl: %d\0".as_ptr(), ret); } }
unsafe fn cht_wc_extcon_set_5v_boost(ext: *mut cht_wc_extcon_data, enable: bool) { let mut val = CHT_WC_VBUS_GPIO_CTLO_DRV_OD | CHT_WC_VBUS_GPIO_CTLO_DIR_OUT; if enable { val |= CHT_WC_VBUS_GPIO_CTLO_OUTPUT; } let ret = regmap_write((*ext).regmap, CHT_WC_VBUS_GPIO_CTLO, val); if ret != 0 { dev_err((*ext).dev, b"Error writing Vbus GPIO CTLO: %d\0".as_ptr(), ret); } }
unsafe fn cht_wc_extcon_set_otgmode(ext: *mut cht_wc_extcon_data, enable: bool) { let val = if enable { CHT_WC_CHGRCTRL1_OTGMODE } else { 0 }; let ret = regmap_update_bits((*ext).regmap, CHT_WC_CHGRCTRL1, CHT_WC_CHGRCTRL1_OTGMODE, val); if ret != 0 { dev_err((*ext).dev, b"Error updating CHGRCTRL1 reg: %d\0".as_ptr(), ret); } if (*ext).vbus_boost != core::ptr::null_mut() && (*ext).vbus_boost_enabled != enable { let r = if enable { regulator_enable((*ext).vbus_boost) } else { regulator_disable((*ext).vbus_boost) }; if r != 0 { dev_err((*ext).dev, b"Error updating Vbus boost regulator: %d\0".as_ptr(), r); } else { (*ext).vbus_boost_enabled = enable; } } }
unsafe fn cht_wc_extcon_enable_charging(ext: *mut cht_wc_extcon_data, enable: bool) { let val = if enable { 0 } else { CHT_WC_CHGDISCTRL_OUT }; let ret = regmap_update_bits((*ext).regmap, CHT_WC_CHGDISCTRL, CHT_WC_CHGDISCTRL_OUT, val); if ret != 0 { dev_err((*ext).dev, b"Error updating CHGDISCTRL reg: %d\0".as_ptr(), ret); } }
unsafe fn cht_wc_extcon_set_state(ext: *mut cht_wc_extcon_data, cable: u32, state: bool) { extcon_set_state_sync((*ext).edev, cable, state); if cable == EXTCON_CHG_USB_SDP { extcon_set_state_sync((*ext).edev, EXTCON_USB, state); } }

// Remaining external kernel integration and driver registration are preserved as declarations.
extern "C" { fn cht_wc_extcon_pwrsrc_event(ext: *mut cht_wc_extcon_data); fn cht_wc_extcon_isr(irq: i32, data: *mut core::ffi::c_void) -> i32; fn cht_wc_extcon_probe(pdev: *mut platform_device) -> i32; fn cht_wc_extcon_remove(pdev: *mut platform_device); }
#[repr(C)] struct platform_device { _private: [u8; 0] }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
