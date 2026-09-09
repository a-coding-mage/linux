/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) ST-Ericsson SA 2010 */

// Linux dependencies represented as opaque/external types in this translation.
use core::ffi::c_void;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ab8500_version {
    AB8500_VERSION_AB8500 = 0x0,
    AB8500_VERSION_AB8505 = 0x1,
    AB8500_VERSION_AB9540 = 0x2,
    AB8500_VERSION_AB8540 = 0x4,
    AB8500_VERSION_UNDEFINED,
}

// AB8500 CIDs
pub const AB8500_CUTEARLY: u8 = 0x00;
pub const AB8500_CUT1P0: u8 = 0x10;
pub const AB8500_CUT1P1: u8 = 0x11;
pub const AB8500_CUT1P2: u8 = 0x12; // Only valid for AB8540
pub const AB8500_CUT2P0: u8 = 0x20;
pub const AB8500_CUT3P0: u8 = 0x30;
pub const AB8500_CUT3P3: u8 = 0x33;

// AB8500 bank addresses
pub const AB8500_M_FSM_RANK: u8 = 0x0;
pub const AB8500_SYS_CTRL1_BLOCK: u8 = 0x1;
pub const AB8500_SYS_CTRL2_BLOCK: u8 = 0x2;
pub const AB8500_REGU_CTRL1: u8 = 0x3;
pub const AB8500_REGU_CTRL2: u8 = 0x4;
pub const AB8500_USB: u8 = 0x5;
pub const AB8500_TVOUT: u8 = 0x6;
pub const AB8500_DBI: u8 = 0x7;
pub const AB8500_ECI_AV_ACC: u8 = 0x8;
pub const AB8500_RESERVED: u8 = 0x9;
pub const AB8500_GPADC: u8 = 0xA;
pub const AB8500_CHARGER: u8 = 0xB;
pub const AB8500_GAS_GAUGE: u8 = 0xC;
pub const AB8500_AUDIO: u8 = 0xD;
pub const AB8500_INTERRUPT: u8 = 0xE;
pub const AB8500_RTC: u8 = 0xF;
pub const AB8500_MISC: u8 = 0x10;
pub const AB8500_DEVELOPMENT: u8 = 0x11;
pub const AB8500_DEBUG: u8 = 0x12;
pub const AB8500_PROD_TEST: u8 = 0x13;
pub const AB8500_STE_TEST: u8 = 0x14;
pub const AB8500_OTP_EMUL: u8 = 0x15;
pub const AB8500_DEBUG_FIELD_LAST: u8 = 0x16;

// Interrupt indices. Values retain the original sparse numbering.
pub const AB8500_INT_MAIN_EXT_CH_NOT_OK: i32 = 0;
pub const AB8500_INT_UN_PLUG_TV_DET: i32 = 1;
pub const AB8500_INT_PLUG_TV_DET: i32 = 2;
pub const AB8500_INT_TEMP_WARM: i32 = 3;
pub const AB8500_INT_PON_KEY2DB_F: i32 = 4;
pub const AB8500_INT_PON_KEY2DB_R: i32 = 5;
pub const AB8500_INT_PON_KEY1DB_F: i32 = 6;
pub const AB8500_INT_PON_KEY1DB_R: i32 = 7;
pub const AB8500_INT_BATT_OVV: i32 = 8;
pub const AB8500_INT_MAIN_CH_UNPLUG_DET: i32 = 10;
pub const AB8500_INT_MAIN_CH_PLUG_DET: i32 = 11;
pub const AB8500_INT_VBUS_DET_F: i32 = 14;
pub const AB8500_INT_VBUS_DET_R: i32 = 15;
pub const AB8500_INT_VBUS_CH_DROP_END: i32 = 16;
pub const AB8500_INT_RTC_60S: i32 = 17;
pub const AB8500_INT_RTC_ALARM: i32 = 18;
pub const AB8540_INT_BIF_INT: i32 = 19;
pub const AB8500_INT_BAT_CTRL_INDB: i32 = 20;
pub const AB8500_INT_CH_WD_EXP: i32 = 21;
pub const AB8500_INT_VBUS_OVV: i32 = 22;
pub const AB8500_INT_MAIN_CH_DROP_END: i32 = 23;
pub const AB8500_INT_CCN_CONV_ACC: i32 = 24;
pub const AB8500_INT_INT_AUD: i32 = 25;
pub const AB8500_INT_CCEOC: i32 = 26;
pub const AB8500_INT_CC_INT_CALIB: i32 = 27;
pub const AB8500_INT_LOW_BAT_F: i32 = 28;
pub const AB8500_INT_LOW_BAT_R: i32 = 29;
pub const AB8500_INT_BUP_CHG_NOT_OK: i32 = 30;
pub const AB8500_INT_BUP_CHG_OK: i32 = 31;
pub const AB8500_INT_GP_HW_ADC_CONV_END: i32 = 32;
pub const AB8500_INT_ACC_DETECT_1DB_F: i32 = 33;
pub const AB8500_INT_ACC_DETECT_1DB_R: i32 = 34;
pub const AB8500_INT_ACC_DETECT_22DB_F: i32 = 35;
pub const AB8500_INT_ACC_DETECT_22DB_R: i32 = 36;
pub const AB8500_INT_ACC_DETECT_21DB_F: i32 = 37;
pub const AB8500_INT_ACC_DETECT_21DB_R: i32 = 38;
pub const AB8500_INT_GP_SW_ADC_CONV_END: i32 = 39;
pub const AB8500_INT_GPIO6R: i32 = 40;
pub const AB8500_INT_GPIO7R: i32 = 41;
pub const AB8500_INT_GPIO8R: i32 = 42;
pub const AB8500_INT_GPIO9R: i32 = 43;
pub const AB8500_INT_GPIO10R: i32 = 44;
pub const AB8500_INT_GPIO11R: i32 = 45;
pub const AB8500_INT_GPIO12R: i32 = 46;
pub const AB8500_INT_GPIO13R: i32 = 47;
pub const AB8500_INT_GPIO24R: i32 = 48;
pub const AB8500_INT_GPIO25R: i32 = 49;
pub const AB8500_INT_GPIO36R: i32 = 50;
pub const AB8500_INT_GPIO37R: i32 = 51;
pub const AB8500_INT_GPIO38R: i32 = 52;
pub const AB8500_INT_GPIO39R: i32 = 53;
pub const AB8500_INT_GPIO40R: i32 = 54;
pub const AB8500_INT_GPIO41R: i32 = 55;
pub const AB8500_INT_GPIO6F: i32 = 56;
pub const AB8500_INT_GPIO7F: i32 = 57;
pub const AB8500_INT_GPIO8F: i32 = 58;
pub const AB8500_INT_GPIO9F: i32 = 59;
pub const AB8500_INT_GPIO10F: i32 = 60;
pub const AB8500_INT_GPIO11F: i32 = 61;
pub const AB8500_INT_GPIO12F: i32 = 62;
pub const AB8500_INT_GPIO13F: i32 = 63;
pub const AB8500_INT_GPIO24F: i32 = 64;
pub const AB8500_INT_GPIO25F: i32 = 65;
pub const AB8500_INT_GPIO36F: i32 = 66;
pub const AB8500_INT_GPIO37F: i32 = 67;
pub const AB8500_INT_GPIO38F: i32 = 68;
pub const AB8500_INT_GPIO39F: i32 = 69;
pub const AB8500_INT_GPIO40F: i32 = 70;
pub const AB8500_INT_GPIO41F: i32 = 71;
pub const AB8500_INT_ADP_SOURCE_ERROR: i32 = 72;
pub const AB8500_INT_ADP_SINK_ERROR: i32 = 73;
pub const AB8500_INT_ADP_PROBE_PLUG: i32 = 74;
pub const AB8500_INT_ADP_PROBE_UNPLUG: i32 = 75;
pub const AB8500_INT_ADP_SENSE_OFF: i32 = 76;
pub const AB8500_INT_USB_PHY_POWER_ERR: i32 = 78;
pub const AB8500_INT_USB_LINK_STATUS: i32 = 79;
pub const AB8500_INT_BTEMP_LOW: i32 = 80;
pub const AB8500_INT_BTEMP_LOW_MEDIUM: i32 = 81;
pub const AB8500_INT_BTEMP_MEDIUM_HIGH: i32 = 82;
pub const AB8500_INT_BTEMP_HIGH: i32 = 83;
pub const AB8500_INT_SRP_DETECT: i32 = 88;
pub const AB8500_INT_USB_CHARGER_NOT_OKR: i32 = 89;
pub const AB8500_INT_ID_WAKEUP_R: i32 = 90;
pub const AB8500_INT_ID_DET_PLUGR: i32 = 91;
pub const AB8500_INT_ID_DET_R1R: i32 = 92;
pub const AB8500_INT_ID_DET_R2R: i32 = 93;
pub const AB8500_INT_ID_DET_R3R: i32 = 94;
pub const AB8500_INT_ID_DET_R4R: i32 = 95;
pub const AB8500_INT_ID_WAKEUP_F: i32 = 96;
pub const AB8500_INT_ID_DET_PLUGF: i32 = 97;
pub const AB8500_INT_ID_DET_R1F: i32 = 98;
pub const AB8500_INT_ID_DET_R2F: i32 = 99;
pub const AB8500_INT_ID_DET_R3F: i32 = 100;
pub const AB8500_INT_ID_DET_R4F: i32 = 101;
pub const AB8500_INT_CHAUTORESTARTAFTSEC: i32 = 102;
pub const AB8500_INT_CHSTOPBYSEC: i32 = 103;
pub const AB8500_INT_USB_CH_TH_PROT_F: i32 = 104;
pub const AB8500_INT_USB_CH_TH_PROT_R: i32 = 105;
pub const AB8500_INT_MAIN_CH_TH_PROT_F: i32 = 106;
pub const AB8500_INT_MAIN_CH_TH_PROT_R: i32 = 107;
pub const AB8500_INT_CHCURLIMNOHSCHIRP: i32 = 109;
pub const AB8500_INT_CHCURLIMHSCHIRP: i32 = 110;
pub const AB8500_INT_XTAL32K_KO: i32 = 111;

pub const AB9540_INT_GPIO50R: i32 = 113;
pub const AB9540_INT_GPIO51R: i32 = 114;
pub const AB9540_INT_GPIO52R: i32 = 115;
pub const AB9540_INT_GPIO53R: i32 = 116;
pub const AB9540_INT_GPIO54R: i32 = 117;
pub const AB9540_INT_IEXT_CH_RF_BFN_R: i32 = 118;
pub const AB9540_INT_GPIO50F: i32 = 121;
pub const AB9540_INT_GPIO51F: i32 = 122;
pub const AB9540_INT_GPIO52F: i32 = 123;
pub const AB9540_INT_GPIO53F: i32 = 124;
pub const AB9540_INT_GPIO54F: i32 = 125;
pub const AB9540_INT_IEXT_CH_RF_BFN_F: i32 = 126;
pub const AB8505_INT_KEYSTUCK: i32 = 128;
pub const AB8505_INT_IKR: i32 = 129;
pub const AB8505_INT_IKP: i32 = 130;
pub const AB8505_INT_KP: i32 = 131;
pub const AB8505_INT_KEYDEGLITCH: i32 = 132;
pub const AB8505_INT_MODPWRSTATUSF: i32 = 134;
pub const AB8505_INT_MODPWRSTATUSR: i32 = 135;
pub const AB8500_INT_HOOK_DET_NEG_F: i32 = 138;
pub const AB8500_INT_HOOK_DET_NEG_R: i32 = 139;
pub const AB8500_INT_HOOK_DET_POS_F: i32 = 140;
pub const AB8500_INT_HOOK_DET_POS_R: i32 = 141;
pub const AB8500_INT_PLUG_DET_COMP_F: i32 = 142;
pub const AB8500_INT_PLUG_DET_COMP_R: i32 = 143;
pub const AB8505_INT_COLL: i32 = 144;
pub const AB8505_INT_RESERR: i32 = 145;
pub const AB8505_INT_FRAERR: i32 = 146;
pub const AB8505_INT_COMERR: i32 = 147;
pub const AB8505_INT_SPDSET: i32 = 148;
pub const AB8505_INT_DSENT: i32 = 149;
pub const AB8505_INT_DREC: i32 = 150;
pub const AB8505_INT_ACC_INT: i32 = 151;
pub const AB8505_INT_NOPINT: i32 = 152;

// AB8540 interrupt indices
pub const AB8540_INT_IDPLUGDETCOMPF: i32 = 160;
pub const AB8540_INT_IDPLUGDETCOMPR: i32 = 161;
pub const AB8540_INT_FMDETCOMPLOF: i32 = 162;
pub const AB8540_INT_FMDETCOMPLOR: i32 = 163;
pub const AB8540_INT_FMDETCOMPHIF: i32 = 164;
pub const AB8540_INT_FMDETCOMPHIR: i32 = 165;
pub const AB8540_INT_ID5VDETCOMPF: i32 = 166;
pub const AB8540_INT_ID5VDETCOMPR: i32 = 167;
pub const AB8540_INT_GPIO43F: i32 = 168;
pub const AB8540_INT_GPIO43R: i32 = 169;
pub const AB8540_INT_GPIO44F: i32 = 170;
pub const AB8540_INT_GPIO44R: i32 = 171;
pub const AB8540_INT_KEYPOSDETCOMPF: i32 = 172;
pub const AB8540_INT_KEYPOSDETCOMPR: i32 = 173;
pub const AB8540_INT_KEYNEGDETCOMPF: i32 = 174;
pub const AB8540_INT_KEYNEGDETCOMPR: i32 = 175;
pub const AB8540_INT_GPIO1VBATF: i32 = 176;
pub const AB8540_INT_GPIO1VBATR: i32 = 177;
pub const AB8540_INT_GPIO2VBATF: i32 = 178;
pub const AB8540_INT_GPIO2VBATR: i32 = 179;
pub const AB8540_INT_GPIO3VBATF: i32 = 180;
pub const AB8540_INT_GPIO3VBATR: i32 = 181;
pub const AB8540_INT_GPIO4VBATF: i32 = 182;
pub const AB8540_INT_GPIO4VBATR: i32 = 183;
pub const AB8540_INT_SYSCLKREQ2F: i32 = 184;
pub const AB8540_INT_SYSCLKREQ2R: i32 = 185;
pub const AB8540_INT_SYSCLKREQ3F: i32 = 186;
pub const AB8540_INT_SYSCLKREQ3R: i32 = 187;
pub const AB8540_INT_SYSCLKREQ4F: i32 = 188;
pub const AB8540_INT_SYSCLKREQ4R: i32 = 189;
pub const AB8540_INT_SYSCLKREQ5F: i32 = 190;
pub const AB8540_INT_SYSCLKREQ5R: i32 = 191;
pub const AB8540_INT_PWMOUT1F: i32 = 192;
pub const AB8540_INT_PWMOUT1R: i32 = 193;
pub const AB8540_INT_PWMCTRL0F: i32 = 194;
pub const AB8540_INT_PWMCTRL0R: i32 = 195;
pub const AB8540_INT_PWMCTRL1F: i32 = 196;
pub const AB8540_INT_PWMCTRL1R: i32 = 197;
pub const AB8540_INT_SYSCLKREQ6F: i32 = 198;
pub const AB8540_INT_SYSCLKREQ6R: i32 = 199;
pub const AB8540_INT_PWMEXTVIBRA1F: i32 = 200;
pub const AB8540_INT_PWMEXTVIBRA1R: i32 = 201;
pub const AB8540_INT_PWMEXTVIBRA2F: i32 = 202;
pub const AB8540_INT_PWMEXTVIBRA2R: i32 = 203;
pub const AB8540_INT_PWMOUT2F: i32 = 204;
pub const AB8540_INT_PWMOUT2R: i32 = 205;
pub const AB8540_INT_PWMOUT3F: i32 = 206;
pub const AB8540_INT_PWMOUT3R: i32 = 207;
pub const AB8540_INT_ADDATA2F: i32 = 208;
pub const AB8540_INT_ADDATA2R: i32 = 209;
pub const AB8540_INT_DADATA2F: i32 = 210;
pub const AB8540_INT_DADATA2R: i32 = 211;
pub const AB8540_INT_FSYNC2F: i32 = 212;
pub const AB8540_INT_FSYNC2R: i32 = 213;
pub const AB8540_INT_BITCLK2F: i32 = 214;
pub const AB8540_INT_BITCLK2R: i32 = 215;
pub const AB8540_INT_RTC_1S: i32 = 216;

pub const AB8500_NR_IRQS: i32 = 112;
pub const AB8505_NR_IRQS: i32 = 153;
pub const AB9540_NR_IRQS: i32 = 153;
pub const AB8540_NR_IRQS: i32 = 216;
pub const AB8500_MAX_NR_IRQS: i32 = AB8540_NR_IRQS;
pub const AB8500_NUM_IRQ_REGS: i32 = 14;
pub const AB9540_NUM_IRQ_REGS: i32 = 20;
pub const AB8540_NUM_IRQ_REGS: i32 = 27;

pub const AB8500_POR_ON_VBAT: u8 = 0x01;
pub const AB8500_POW_KEY_1_ON: u8 = 0x02;
pub const AB8500_POW_KEY_2_ON: u8 = 0x04;
pub const AB8500_RTC_ALARM: u8 = 0x08;
pub const AB8500_MAIN_CH_DET: u8 = 0x10;
pub const AB8500_VBUS_DET: u8 = 0x20;
pub const AB8500_USB_ID_DET: u8 = 0x40;

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct irq_domain;
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct atomic_t { _private: [u8; 0] }
pub struct ab8500_codec_platform_data;
pub struct ab8500_sysctrl_platform_data;

#[repr(C)]
pub struct ab8500 {
    pub dev: *mut device,
    pub lock: mutex,
    pub irq_lock: mutex,
    pub transfer_ongoing: atomic_t,
    pub irq: i32,
    pub domain: *mut irq_domain,
    pub version: ab8500_version,
    pub chip_id: u8,
    pub write: Option<unsafe extern "C" fn(*mut ab8500, u16, u8) -> i32>,
    pub write_masked: Option<unsafe extern "C" fn(*mut ab8500, u16, u8, u8) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut ab8500, u16) -> i32>,
    pub tx_buf: [c_ulong; 4],
    pub rx_buf: [c_ulong; 4],
    pub mask: *mut u8,
    pub oldmask: *mut u8,
    pub mask_size: i32,
    pub irq_reg_offset: *const i32,
    pub it_latchhier_num: i32,
}

pub type c_ulong = usize;

#[repr(C)]
pub struct ab8500_platform_data {
    pub init: Option<unsafe extern "C" fn(*mut ab8500)>,
    pub codec: *mut ab8500_codec_platform_data,
    pub sysctrl: *mut ab8500_sysctrl_platform_data,
}

extern "C" {
    pub fn ab8500_suspend(ab8500: *mut ab8500) -> i32;
    pub fn ab8500_override_turn_on_stat(mask: u8, set: u8);
}

#[inline]
pub unsafe fn is_ab8500(ab: *mut ab8500) -> i32 { ((*ab).version == ab8500_version::AB8500_VERSION_AB8500) as i32 }
#[inline]
pub unsafe fn is_ab8505(ab: *mut ab8500) -> i32 { ((*ab).version == ab8500_version::AB8500_VERSION_AB8505) as i32 }
#[inline]
pub unsafe fn is_ab9540(ab: *mut ab8500) -> i32 { ((*ab).version == ab8500_version::AB8500_VERSION_AB9540) as i32 }
#[inline]
pub unsafe fn is_ab8540(ab: *mut ab8500) -> i32 { ((*ab).version == ab8500_version::AB8500_VERSION_AB8540) as i32 }

#[inline] pub unsafe fn is_ab8500_1p0_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8500(ab) != 0 && (*ab).chip_id <= AB8500_CUT1P0) as i32 }
#[inline] pub unsafe fn is_ab8500_1p1_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8500(ab) != 0 && (*ab).chip_id <= AB8500_CUT1P1) as i32 }
#[inline] pub unsafe fn is_ab8500_2p0_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8500(ab) != 0 && (*ab).chip_id <= AB8500_CUT2P0) as i32 }
#[inline] pub unsafe fn is_ab8500_3p3_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8500(ab) != 0 && (*ab).chip_id <= AB8500_CUT3P3) as i32 }
#[inline] pub unsafe fn is_ab8500_2p0(ab: *mut ab8500) -> i32 { (is_ab8500(ab) != 0 && (*ab).chip_id == AB8500_CUT2P0) as i32 }
#[inline] pub unsafe fn is_ab8505_1p0_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8505(ab) != 0 && (*ab).chip_id <= AB8500_CUT1P0) as i32 }
#[inline] pub unsafe fn is_ab8505_2p0(ab: *mut ab8500) -> i32 { (is_ab8505(ab) != 0 && (*ab).chip_id == AB8500_CUT2P0) as i32 }
#[inline] pub unsafe fn is_ab9540_1p0_or_earlier(ab: *mut ab8500) -> i32 { (is_ab9540(ab) != 0 && (*ab).chip_id <= AB8500_CUT1P0) as i32 }
#[inline] pub unsafe fn is_ab9540_2p0(ab: *mut ab8500) -> i32 { (is_ab9540(ab) != 0 && (*ab).chip_id == AB8500_CUT2P0) as i32 }
#[inline] pub unsafe fn is_ab9540_3p0(ab: *mut ab8500) -> i32 { (is_ab9540(ab) != 0 && (*ab).chip_id == AB8500_CUT3P0) as i32 }
#[inline] pub unsafe fn is_ab8540_1p0_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8540(ab) != 0 && (*ab).chip_id <= AB8500_CUT1P0) as i32 }
#[inline] pub unsafe fn is_ab8540_1p1_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8540(ab) != 0 && (*ab).chip_id <= AB8500_CUT1P1) as i32 }
#[inline] pub unsafe fn is_ab8540_1p2_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8540(ab) != 0 && (*ab).chip_id <= AB8500_CUT1P2) as i32 }
#[inline] pub unsafe fn is_ab8540_2p0_or_earlier(ab: *mut ab8500) -> i32 { (is_ab8540(ab) != 0 && (*ab).chip_id <= AB8500_CUT2P0) as i32 }
#[inline] pub unsafe fn is_ab8540_2p0(ab: *mut ab8500) -> i32 { (is_ab8540(ab) != 0 && (*ab).chip_id == AB8500_CUT2P0) as i32 }
#[inline] pub unsafe fn is_ab8505_2p0_earlier(ab: *mut ab8500) -> i32 { (is_ab8505(ab) != 0 && (*ab).chip_id < AB8500_CUT2P0) as i32 }
#[inline] pub unsafe fn is_ab9540_2p0_or_earlier(ab: *mut ab8500) -> i32 { (is_ab9540(ab) != 0 && (*ab).chip_id < AB8500_CUT2P0) as i32 }

#[inline] pub unsafe fn ab8500_dump_all_banks(_dev: *mut device) {}
#[inline] pub unsafe fn ab8500_debug_register_interrupt(_line: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
