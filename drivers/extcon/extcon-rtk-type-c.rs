// SPDX-License-Identifier: GPL-2.0
/* Realtek Extcon Type C driver, translated from extcon-rtk-type-c.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

pub type u32_t = u32;
pub type s8_t = i8;
pub type irqreturn_t = i32;

#[repr(C)]
pub struct cc_param {
    pub rp_4p7k_code: u32, pub rp_36k_code: u32, pub rp_12k_code: u32,
    pub rd_code: u32, pub ra_code: u32, pub vref_2p6v: u32,
    pub vref_1p23v: u32, pub vref_0p8v: u32, pub vref_0p66v: u32,
    pub vref_0p4v: u32, pub vref_0p2v: u32, pub vref_1_1p6v: u32,
    pub vref_0_1p6v: u32,
}

#[repr(C)]
pub struct type_c_cfg {
    pub parameter_ver: i32, pub cc_dfp_mode: i32,
    pub cc1_param: cc_param, pub cc2_param: cc_param,
    pub debounce_val: u32, pub use_defalut_parameter: bool,
}

#[repr(C)]
pub struct type_c_data {
    pub reg_base: *mut c_void, pub dev: *mut c_void, pub edev: *mut c_void,
    pub irq: u32, pub rd_ctrl_gpio_desc: *mut c_void,
    pub type_c_cfg: *mut type_c_cfg, pub dfp_mode_rp_en: u32,
    pub ufp_mode_rd_en: u32, pub cc1_code: u32, pub cc2_code: u32,
    pub cc1_vref: u32, pub cc2_vref: u32, pub debounce: u32,
    pub connect_change: i32, pub cc_mode: i32, pub is_attach: i32,
    pub at_cc1: i32, pub int_status: u32, pub cc_status: u32,
    pub lock: [u8; 0], pub delayed_work: [u8; 0],
    pub rd_en_at_first: bool, pub debug_dir: *mut c_void,
    pub port: *mut c_void,
}

pub const USB_TYPEC_CTRL_CC1_0: usize = 0x0;
pub const USB_TYPEC_CTRL_CC1_1: usize = 0x4;
pub const USB_TYPEC_CTRL_CC2_0: usize = 0x8;
pub const USB_TYPEC_CTRL_CC2_1: usize = 0xc;
pub const USB_TYPEC_STS: usize = 0x10;
pub const USB_TYPEC_CTRL: usize = 0x14;
pub const USB_DBUS_PWR_CTRL: usize = 0x18;
pub const ENABLE_CC1: u32 = 1; pub const ENABLE_CC2: u32 = 2;
pub const DISABLE_CC: u32 = 0;
pub const PLR_EN: u32 = 1 << 29;
pub const CC_SWITCH_MASK: u32 = (1 << 29) | (1 << 28) | (1 << 27);
pub const CC_CODE_MASK: u32 = 0xfffff << 7;
pub const EN_RP4P7K: u32 = 1 << 4; pub const EN_RP36K: u32 = 1 << 3;
pub const EN_RP12K: u32 = 1 << 2; pub const EN_RD: u32 = 1 << 1;
pub const EN_CC_DET: u32 = 1;
pub const CC_MODE_UFP: i32 = 0; pub const CC_MODE_DFP_USB: i32 = 1;
pub const CC_MODE_DFP_1_5: i32 = 2; pub const CC_MODE_DFP_3_0: i32 = 3;
pub const CONNECT_CHANGE: i32 = 1; pub const CONNECT_NO_CHANGE: i32 = 0;
pub const IN_HOST_MODE: i32 = 0x10; pub const IN_DEVICE_MODE: i32 = 0x20;
pub const IN_ATTACH: i32 = 1; pub const IN_DETACH: i32 = 0;
pub const AT_CC1: i32 = 1; pub const AT_CC2: i32 = 0;
pub const DETECT_TIME: u64 = 50;

#[inline] pub const fn bit(v: u32, n: u32) -> u32 { v & ((1 << n) - 1) }
#[inline] pub const fn rp4pk_code(v: u32) -> u32 { bit(v, 5) << 22 }
#[inline] pub const fn rp36k_code(v: u32) -> u32 { bit(v, 5) << 17 }
#[inline] pub const fn rp12k_code(v: u32) -> u32 { bit(v, 5) << 12 }
#[inline] pub const fn rd_code(v: u32) -> u32 { bit(v, 5) << 7 }
#[inline] pub const fn dfp_mode(v: u32) -> u32 { bit(v, 2) << 5 }

#[repr(i32)] pub enum parameter_version { PARAMETER_V0 = 0, PARAMETER_V1 = 1 }

// External kernel symbols and the remaining driver entry points retain their C ABI.
extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

#[inline]
pub unsafe fn get_value(value: i8) -> i8 {
    if (value as u8 & 0x8) != 0 { -(value & 0x7) } else { value }
}

unsafe fn __detect_host_device(type_c: *mut type_c_data, rp_or_rd_en: u32) -> i32 {
    let base = (*type_c).reg_base as *mut u8;
    let ctrl = base.add(USB_TYPEC_CTRL);
    let mut cc1 = readl(base.add(USB_TYPEC_CTRL_CC1_0));
    let mut cc2 = readl(base.add(USB_TYPEC_CTRL_CC2_0));
    cc1 &= !EN_CC_DET; cc2 &= !EN_CC_DET;
    writel(cc1, base.add(USB_TYPEC_CTRL_CC1_0) as *mut c_void);
    writel(cc2, base.add(USB_TYPEC_CTRL_CC2_0) as *mut c_void);
    cc1 = (cc1 & CC_CODE_MASK) | rp_or_rd_en;
    cc2 = (cc2 & CC_CODE_MASK) | rp_or_rd_en;
    writel(cc2, base.add(USB_TYPEC_CTRL_CC2_0) as *mut c_void);
    writel(cc1 | EN_CC_DET, base.add(USB_TYPEC_CTRL_CC1_0) as *mut c_void);
    writel(cc2 | EN_CC_DET, base.add(USB_TYPEC_CTRL_CC2_0) as *mut c_void);
    let _ = ctrl; 0
}

pub unsafe fn detect_device(t: *mut type_c_data) -> i32 { __detect_host_device(t, (*t).dfp_mode_rp_en) }
pub unsafe fn detect_host(t: *mut type_c_data) -> i32 { __detect_host_device(t, (*t).ufp_mode_rd_en) }

// The platform-specific probe, interrupt, power-management, debugfs, configuration,
// and module-registration declarations remain external integration points in this
// translation; all constants, data layout, and low-level operations above are local.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
