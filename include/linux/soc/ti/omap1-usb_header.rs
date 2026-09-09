/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Constants in this file are used all over the place, in platform
 * code, as well as the udc, phy and ohci drivers.
 * This is not a great design, but unlikely to get fixed after
 * such a long time. Don't do this elsewhere.
 */

pub const OMAP1_OTG_BASE: usize = 0xfffb0400;
pub const OMAP1_UDC_BASE: usize = 0xfffb4000;

pub const OMAP2_UDC_BASE: usize = 0x4805e200;
pub const OMAP2_OTG_BASE: usize = 0x4805e300;
pub const OTG_BASE: usize = OMAP1_OTG_BASE;
pub const UDC_BASE: usize = OMAP1_UDC_BASE;

/*
 * OTG and transceiver registers, for OMAPs starting with ARM926
 */
pub const OTG_REV: usize = OTG_BASE + 0x00;
pub const OTG_SYSCON_1: usize = OTG_BASE + 0x04;
#[inline]
pub const fn USB2_TRX_MODE(w: u32) -> u32 { (w >> 24) & 0x07 }
#[inline]
pub const fn USB1_TRX_MODE(w: u32) -> u32 { (w >> 20) & 0x07 }
#[inline]
pub const fn USB0_TRX_MODE(w: u32) -> u32 { (w >> 16) & 0x07 }
pub const OTG_IDLE_EN: u32 = 1 << 15;
pub const HST_IDLE_EN: u32 = 1 << 14;
pub const DEV_IDLE_EN: u32 = 1 << 13;
pub const OTG_RESET_DONE: u32 = 1 << 2;
pub const OTG_SOFT_RESET: u32 = 1 << 1;
pub const OTG_SYSCON_2: usize = OTG_BASE + 0x08;
pub const OTG_EN: u32 = 1 << 31;
pub const USBX_SYNCHRO: u32 = 1 << 30;
pub const OTG_MST16: u32 = 1 << 29;
pub const SRP_GPDATA: u32 = 1 << 28;
pub const SRP_GPDVBUS: u32 = 1 << 27;
#[inline]
pub const fn SRP_GPUVBUS(w: u32) -> u32 { (w >> 24) & 0x07 }
#[inline]
pub const fn A_WAIT_VRISE(w: u32) -> u32 { (w >> 20) & 0x07 }
#[inline]
pub const fn B_ASE_BRST(w: u32) -> u32 { (w >> 16) & 0x07 }
pub const SRP_DPW: u32 = 1 << 14;
pub const SRP_DATA: u32 = 1 << 13;
pub const SRP_VBUS: u32 = 1 << 12;
pub const OTG_PADEN: u32 = 1 << 10;
pub const HMC_PADEN: u32 = 1 << 9;
pub const UHOST_EN: u32 = 1 << 8;
pub const HMC_TLLSPEED: u32 = 1 << 7;
pub const HMC_TLLATTACH: u32 = 1 << 6;
#[inline]
pub const fn OTG_HMC(w: u32) -> u32 { (w >> 0) & 0x3f }
pub const OTG_CTRL: usize = OTG_BASE + 0x0c;
pub const OTG_USB2_EN: u32 = 1 << 29;
pub const OTG_USB2_DP: u32 = 1 << 28;
pub const OTG_USB2_DM: u32 = 1 << 27;
pub const OTG_USB1_EN: u32 = 1 << 26;
pub const OTG_USB1_DP: u32 = 1 << 25;
pub const OTG_USB1_DM: u32 = 1 << 24;
pub const OTG_USB0_EN: u32 = 1 << 23;
pub const OTG_USB0_DP: u32 = 1 << 22;
pub const OTG_USB0_DM: u32 = 1 << 21;
pub const OTG_ASESSVLD: u32 = 1 << 20;
pub const OTG_BSESSEND: u32 = 1 << 19;
pub const OTG_BSESSVLD: u32 = 1 << 18;
pub const OTG_VBUSVLD: u32 = 1 << 17;
pub const OTG_ID: u32 = 1 << 16;
pub const OTG_DRIVER_SEL: u32 = 1 << 15;
pub const OTG_A_SETB_HNPEN: u32 = 1 << 12;
pub const OTG_A_BUSREQ: u32 = 1 << 11;
pub const OTG_B_HNPEN: u32 = 1 << 9;
pub const OTG_B_BUSREQ: u32 = 1 << 8;
pub const OTG_BUSDROP: u32 = 1 << 7;
pub const OTG_PULLDOWN: u32 = 1 << 5;
pub const OTG_PULLUP: u32 = 1 << 4;
pub const OTG_DRV_VBUS: u32 = 1 << 3;
pub const OTG_PD_VBUS: u32 = 1 << 2;
pub const OTG_PU_VBUS: u32 = 1 << 1;
pub const OTG_PU_ID: u32 = 1 << 0;
pub const OTG_IRQ_EN: usize = OTG_BASE + 0x10; /* 16-bit */
pub const DRIVER_SWITCH: u32 = 1 << 15;
pub const A_VBUS_ERR: u32 = 1 << 13;
pub const A_REQ_TMROUT: u32 = 1 << 12;
pub const A_SRP_DETECT: u32 = 1 << 11;
pub const B_HNP_FAIL: u32 = 1 << 10;
pub const B_SRP_TMROUT: u32 = 1 << 9;
pub const B_SRP_DONE: u32 = 1 << 8;
pub const B_SRP_STARTED: u32 = 1 << 7;
pub const OPRT_CHG: u32 = 1 << 0;
pub const OTG_IRQ_SRC: usize = OTG_BASE + 0x14; /* 16-bit */
// same bits as in IRQ_EN
pub const OTG_OUTCTRL: usize = OTG_BASE + 0x18; /* 16-bit */
pub const OTGVPD: u32 = 1 << 14;
pub const OTGVPU: u32 = 1 << 13;
pub const OTGPUID: u32 = 1 << 12;
pub const USB2VDR: u32 = 1 << 10;
pub const USB2PDEN: u32 = 1 << 9;
pub const USB2PUEN: u32 = 1 << 8;
pub const USB1VDR: u32 = 1 << 6;
pub const USB1PDEN: u32 = 1 << 5;
pub const USB1PUEN: u32 = 1 << 4;
pub const USB0VDR: u32 = 1 << 2;
pub const USB0PDEN: u32 = 1 << 1;
pub const USB0PUEN: u32 = 1 << 0;
pub const OTG_TEST: usize = OTG_BASE + 0x20; /* 16-bit */
pub const OTG_VENDOR_CODE: usize = OTG_BASE + 0xfc; /* 16-bit */

/*-------------------------------------------------------------------------*/

/* OMAP1 */
pub const USB_TRANSCEIVER_CTRL: usize = 0xfffe1000 + 0x0064;
pub const CONF_USB2_UNI_R: u32 = 1 << 8;
pub const CONF_USB1_UNI_R: u32 = 1 << 7;
#[inline]
pub const fn CONF_USB_PORT0_R(x: u32) -> u32 { (x >> 4) & 0x7 }
pub const CONF_USB0_ISOLATE_R: u32 = 1 << 3;
pub const CONF_USB_PWRDN_DM_R: u32 = 1 << 2;
pub const CONF_USB_PWRDN_DP_R: u32 = 1 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
