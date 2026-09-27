/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Generic RTC interface. */

// Dependencies supplied by the corresponding Linux UAPI definitions:
// _IO, _IOR, _IOW, and _BITUL.

#[repr(C)]
pub struct rtc_time {
    pub tm_sec: ::kernel::ffi::c_int,
    pub tm_min: ::kernel::ffi::c_int,
    pub tm_hour: ::kernel::ffi::c_int,
    pub tm_mday: ::kernel::ffi::c_int,
    pub tm_mon: ::kernel::ffi::c_int,
    pub tm_year: ::kernel::ffi::c_int,
    pub tm_wday: ::kernel::ffi::c_int,
    pub tm_yday: ::kernel::ffi::c_int,
    pub tm_isdst: ::kernel::ffi::c_int,
}

#[repr(C)]
pub struct rtc_wkalrm {
    pub enabled: u8, // 0 = alarm disabled, 1 = alarm enabled
    pub pending: u8, // 0 = alarm not pending, 1 = alarm pending
    pub time: rtc_time, // time the alarm is set to
}

#[repr(C)]
pub struct rtc_pll_info {
    pub pll_ctrl: ::kernel::ffi::c_int,
    pub pll_value: ::kernel::ffi::c_int,
    pub pll_max: ::kernel::ffi::c_int,
    pub pll_min: ::kernel::ffi::c_int,
    pub pll_posmult: ::kernel::ffi::c_int,
    pub pll_negmult: ::kernel::ffi::c_int,
    pub pll_clock: ::kernel::ffi::c_long,
}

#[repr(C)]
pub union rtc_param_value {
    pub uvalue: u64,
    pub svalue: i64,
    pub ptr: u64,
}

#[repr(C)]
pub struct rtc_param {
    pub param: u64,
    pub value: rtc_param_value,
    pub index: u32,
    pub __pad: u32,
}

pub const RTC_AIE_ON: u32 = _IO(b'p' as _, 0x01);
pub const RTC_AIE_OFF: u32 = _IO(b'p' as _, 0x02);
pub const RTC_UIE_ON: u32 = _IO(b'p' as _, 0x03);
pub const RTC_UIE_OFF: u32 = _IO(b'p' as _, 0x04);
pub const RTC_PIE_ON: u32 = _IO(b'p' as _, 0x05);
pub const RTC_PIE_OFF: u32 = _IO(b'p' as _, 0x06);
pub const RTC_WIE_ON: u32 = _IO(b'p' as _, 0x0f);
pub const RTC_WIE_OFF: u32 = _IO(b'p' as _, 0x10);

pub const RTC_ALM_SET: u32 = _IOW(b'p' as _, 0x07, rtc_time);
pub const RTC_ALM_READ: u32 = _IOR(b'p' as _, 0x08, rtc_time);
pub const RTC_RD_TIME: u32 = _IOR(b'p' as _, 0x09, rtc_time);
pub const RTC_SET_TIME: u32 = _IOW(b'p' as _, 0x0a, rtc_time);
pub const RTC_IRQP_READ: u32 = _IOR(b'p' as _, 0x0b, ::kernel::ffi::c_ulong);
pub const RTC_IRQP_SET: u32 = _IOW(b'p' as _, 0x0c, ::kernel::ffi::c_ulong);
pub const RTC_EPOCH_READ: u32 = _IOR(b'p' as _, 0x0d, ::kernel::ffi::c_ulong);
pub const RTC_EPOCH_SET: u32 = _IOW(b'p' as _, 0x0e, ::kernel::ffi::c_ulong);
pub const RTC_WKALM_SET: u32 = _IOW(b'p' as _, 0x0f, rtc_wkalrm);
pub const RTC_WKALM_RD: u32 = _IOR(b'p' as _, 0x10, rtc_wkalrm);
pub const RTC_PLL_GET: u32 = _IOR(b'p' as _, 0x11, rtc_pll_info);
pub const RTC_PLL_SET: u32 = _IOW(b'p' as _, 0x12, rtc_pll_info);
pub const RTC_PARAM_GET: u32 = _IOW(b'p' as _, 0x13, rtc_param);
pub const RTC_PARAM_SET: u32 = _IOW(b'p' as _, 0x14, rtc_param);

pub const RTC_VL_DATA_INVALID: _ = _BITUL(0);
pub const RTC_VL_BACKUP_LOW: _ = _BITUL(1);
pub const RTC_VL_BACKUP_EMPTY: _ = _BITUL(2);
pub const RTC_VL_ACCURACY_LOW: _ = _BITUL(3);
pub const RTC_VL_BACKUP_SWITCH: _ = _BITUL(4);
pub const RTC_VL_READ: u32 = _IOR(b'p' as _, 0x13, ::kernel::ffi::c_uint);
pub const RTC_VL_CLR: u32 = _IO(b'p' as _, 0x14);

pub const RTC_IRQF: u8 = 0x80;
pub const RTC_PF: u8 = 0x40;
pub const RTC_AF: u8 = 0x20;
pub const RTC_UF: u8 = 0x10;

pub const RTC_FEATURE_ALARM: u32 = 0;
pub const RTC_FEATURE_ALARM_RES_MINUTE: u32 = 1;
pub const RTC_FEATURE_NEED_WEEK_DAY: u32 = 2;
pub const RTC_FEATURE_ALARM_RES_2S: u32 = 3;
pub const RTC_FEATURE_UPDATE_INTERRUPT: u32 = 4;
pub const RTC_FEATURE_CORRECTION: u32 = 5;
pub const RTC_FEATURE_BACKUP_SWITCH_MODE: u32 = 6;
pub const RTC_FEATURE_ALARM_WAKEUP_ONLY: u32 = 7;
pub const RTC_FEATURE_CNT: u32 = 8;

pub const RTC_PARAM_FEATURES: u32 = 0;
pub const RTC_PARAM_CORRECTION: u32 = 1;
pub const RTC_PARAM_BACKUP_SWITCH_MODE: u32 = 2;

pub const RTC_BSM_DISABLED: u32 = 0;
pub const RTC_BSM_DIRECT: u32 = 1;
pub const RTC_BSM_LEVEL: u32 = 2;
pub const RTC_BSM_STANDBY: u32 = 3;

pub const RTC_MAX_FREQ: u32 = 8192;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
