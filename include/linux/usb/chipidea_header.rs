/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Platform data for the chipidea USB dual role controller
 */

use core::ffi::{c_char, c_int, c_ulong};

/* External declarations supplied by the surrounding kernel bindings. */
#[repr(C)]
pub struct extcon_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct phy {
    _private: [u8; 0],
}
#[repr(C)]
pub struct usb_phy {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pinctrl {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pinctrl_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ci_hdrc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct usb_otg_caps {
    _private: [u8; 0],
}

pub type usb_phy_interface = u32;
pub type usb_dr_mode = u32;

#[repr(C)]
pub struct ci_hdrc_cable {
    pub connected: bool,
    pub changed: bool,
    pub enabled: bool,
    pub edev: *mut extcon_dev,
    pub ci: *mut ci_hdrc,
    pub nb: notifier_block,
}

pub const CI_HDRC_REGS_SHARED: c_ulong = 1 << 0;
pub const CI_HDRC_DISABLE_DEVICE_STREAMING: c_ulong = 1 << 1;
pub const CI_HDRC_SUPPORTS_RUNTIME_PM: c_ulong = 1 << 2;
pub const CI_HDRC_DISABLE_HOST_STREAMING: c_ulong = 1 << 3;
pub const CI_HDRC_DISABLE_STREAMING: c_ulong =
    CI_HDRC_DISABLE_DEVICE_STREAMING | CI_HDRC_DISABLE_HOST_STREAMING;
pub const CI_HDRC_DUAL_ROLE_NOT_OTG: c_ulong = 1 << 4;
pub const CI_HDRC_IMX28_WRITE_FIX: c_ulong = 1 << 5;
pub const CI_HDRC_FORCE_FULLSPEED: c_ulong = 1 << 6;
pub const CI_HDRC_TURN_VBUS_EARLY_ON: c_ulong = 1 << 7;
pub const CI_HDRC_SET_NON_ZERO_TTHA: c_ulong = 1 << 8;
pub const CI_HDRC_OVERRIDE_AHB_BURST: c_ulong = 1 << 9;
pub const CI_HDRC_OVERRIDE_TX_BURST: c_ulong = 1 << 10;
pub const CI_HDRC_OVERRIDE_RX_BURST: c_ulong = 1 << 11;
pub const CI_HDRC_OVERRIDE_PHY_CONTROL: c_ulong = 1 << 12;
pub const CI_HDRC_REQUIRES_ALIGNED_DMA: c_ulong = 1 << 13;
pub const CI_HDRC_IMX_IS_HSIC: c_ulong = 1 << 14;
pub const CI_HDRC_PMQOS: c_ulong = 1 << 15;
pub const CI_HDRC_PHY_VBUS_CONTROL: c_ulong = 1 << 16;
pub const CI_HDRC_HAS_PORTSC_PEC_MISSED: c_ulong = 1 << 17;
pub const CI_HDRC_FORCE_VBUS_ACTIVE_ALWAYS: c_ulong = 1 << 18;
pub const CI_HDRC_HAS_SHORT_PKT_LIMIT: c_ulong = 1 << 19;
pub const CI_HDRC_OUT_BAND_WAKEUP: c_ulong = 1 << 20;

pub const CI_HDRC_CONTROLLER_RESET_EVENT: u32 = 0;
pub const CI_HDRC_CONTROLLER_STOPPED_EVENT: u32 = 1;
pub const CI_HDRC_IMX_HSIC_ACTIVE_EVENT: u32 = 2;
pub const CI_HDRC_IMX_HSIC_SUSPEND_EVENT: u32 = 3;
pub const CI_HDRC_CONTROLLER_VBUS_EVENT: u32 = 4;
pub const CI_HDRC_CONTROLLER_PULLUP_EVENT: u32 = 5;

#[repr(C)]
pub struct ci_hdrc_platform_data {
    pub name: *const c_char,
    pub capoffset: usize,
    pub power_budget: u32,
    pub phy: *mut phy,
    pub usb_phy: *mut usb_phy,
    pub phy_mode: usb_phy_interface,
    pub flags: c_ulong,
    pub dr_mode: usb_dr_mode,
    pub notify_event:
        Option<unsafe extern "C" fn(ci: *mut ci_hdrc, event: u32) -> c_int>,
    pub reg_vbus: *mut regulator,
    pub ci_otg_caps: usb_otg_caps,
    pub tpl_support: bool,
    pub itc_setting: u32,
    pub ahb_burst_config: u32,
    pub tx_burst_size: u32,
    pub rx_burst_size: u32,
    pub vbus_extcon: ci_hdrc_cable,
    pub id_extcon: ci_hdrc_cable,
    pub phy_clkgate_delay_us: u32,
    pub pctl: *mut pinctrl,
    pub pins_default: *mut pinctrl_state,
    pub pins_host: *mut pinctrl_state,
    pub pins_device: *mut pinctrl_state,
    pub hub_control: Option<unsafe extern "C" fn(
        ci: *mut ci_hdrc,
        type_req: u16,
        w_value: u16,
        w_index: u16,
        buf: *mut c_char,
        w_length: u16,
        done: *mut bool,
        flags: *mut c_ulong,
    ) -> c_int>,
    pub enter_lpm: Option<unsafe extern "C" fn(ci: *mut ci_hdrc, enable: bool)>,
}

pub const DEF_CAPOFFSET: usize = 0x100;

unsafe extern "C" {
    pub fn ci_hdrc_add_device(
        dev: *mut device,
        res: *mut resource,
        nres: c_int,
        platdata: *mut ci_hdrc_platform_data,
    ) -> *mut platform_device;
    pub fn ci_hdrc_remove_device(pdev: *mut platform_device);
    pub fn ci_hdrc_query_available_role(pdev: *mut platform_device) -> usb_dr_mode;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
