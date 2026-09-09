// SPDX-License-Identifier: GPL-2.0
/*
 * xt_time
 * Copyright © CC Computer Consultants GmbH, 2007
 *
 * based on ipt_time by Fabrice MARIE <fabrice@netfilter.org>
 * This is a module which is used for time matching
 * It is using some modified code from dietlibc (localtime() function)
 */

// Kernel headers and build-time macros are supplied by the surrounding kernel
// translation unit.

#[repr(C)]
pub struct xtm {
    pub month: u8,
    pub monthday: u8,
    pub weekday: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub dse: u32,
}

extern "C" {
    pub static mut sys_tz: timezone;
    fn ktime_get_real_seconds() -> i64;
    fn xt_register_match(reg: *mut xt_match) -> i32;
    fn xt_unregister_match(reg: *mut xt_match);
    fn pr_info_ratelimited(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}

#[repr(C)]
pub struct sk_buff {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const xt_time_info,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const xt_time_info,
}

#[repr(C)]
pub struct xt_time_info {
    pub date_start: u32,
    pub date_stop: u32,
    pub daytime_start: u32,
    pub daytime_stop: u32,
    pub monthdays_match: u32,
    pub weekdays_match: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const core::ffi::c_char,
    pub family: u16,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub matchsize: usize,
    pub me: *mut core::ffi::c_void,
}

pub const DSE_FIRST: u32 = 2039;
pub const SECONDS_PER_DAY: u64 = 86400;

pub const XT_TIME_LOCAL_TZ: u32 = 1;
pub const XT_TIME_CONTIGUOUS: u32 = 2;
pub const XT_TIME_ALL_FLAGS: u32 = XT_TIME_LOCAL_TZ | XT_TIME_CONTIGUOUS;
pub const XT_TIME_MAX_DAYTIME: u32 = 86399;
pub const XT_TIME_ALL_MONTHDAYS: u32 = 0xffff_fffe;
pub const NFPROTO_UNSPEC: u16 = 0;

static DAYS_SINCE_YEAR: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
static DAYS_SINCE_LEAPYEAR: [u16; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
static DAYS_SINCE_EPOCH: [u16; 70] = [
    25202,24837,24472,24106,23741,23376,23011,22645,22280,21915,
    21550,21184,20819,20454,20089,19723,19358,18993,18628,18262,
    17897,17532,17167,16801,16436,16071,15706,15340,14975,14610,
    14245,13879,13514,13149,12784,12418,12053,11688,11323,10957,
    10592,10227,9862,9496,9131,8766,8401,8035,7670,7305,
    6940,6574,6209,5844,5479,5113,4748,4383,4018,3652,
    3287,2922,2557,2191,1826,1461,1096,730,365,0,
];

#[inline]
unsafe fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[inline]
unsafe fn localtime_1(r: *mut xtm, time: i64) -> u32 {
    let v = (time as u64 % SECONDS_PER_DAY) as u32;
    (*r).second = (v % 60) as u8;
    let w = v / 60;
    (*r).minute = (w % 60) as u8;
    (*r).hour = (w / 60) as u8;
    v
}

#[inline]
unsafe fn localtime_2(r: *mut xtm, time: i64) {
    (*r).dse = (time as u64 / SECONDS_PER_DAY) as u32;
    (*r).weekday = ((4 + (*r).dse - 1) % 7 + 1) as u8;
}

unsafe fn localtime_3(r: *mut xtm, _time: i64) {
    let mut year = DSE_FIRST;
    let mut i = 0usize;
    let mut w = (*r).dse;
    while DAYS_SINCE_EPOCH[i] as u32 > w {
        i += 1;
        year -= 1;
    }
    w -= DAYS_SINCE_EPOCH[i] as u32;
    if is_leap_year(year) {
        i = DAYS_SINCE_LEAPYEAR.len() - 1;
        while i > 0 && DAYS_SINCE_LEAPYEAR[i] as u32 > w { i -= 1; }
        (*r).monthday = (w - DAYS_SINCE_LEAPYEAR[i] as u32 + 1) as u8;
    } else {
        i = DAYS_SINCE_YEAR.len() - 1;
        while i > 0 && DAYS_SINCE_YEAR[i] as u32 > w { i -= 1; }
        (*r).monthday = (w - DAYS_SINCE_YEAR[i] as u32 + 1) as u8;
    }
    (*r).month = (i + 1) as u8;
}

unsafe extern "C" fn time_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let _ = skb;
    let info = (*par).matchinfo;
    let mut current_time = xtm { month: 0, monthday: 0, weekday: 0, hour: 0, minute: 0, second: 0, dse: 0 };
    let mut stamp = ktime_get_real_seconds();
    if (*info).flags & XT_TIME_LOCAL_TZ != 0 { stamp -= 60 * sys_tz.tz_minuteswest as i64; }
    if stamp < (*info).date_start as i64 || stamp > (*info).date_stop as i64 { return false; }
    let packet_time = localtime_1(&mut current_time, stamp);
    if (*info).daytime_start < (*info).daytime_stop {
        if packet_time < (*info).daytime_start || packet_time > (*info).daytime_stop { return false; }
    } else {
        if packet_time < (*info).daytime_start && packet_time > (*info).daytime_stop { return false; }
        if (*info).flags & XT_TIME_CONTIGUOUS != 0 && packet_time <= (*info).daytime_stop { stamp -= SECONDS_PER_DAY as i64; }
    }
    localtime_2(&mut current_time, stamp);
    if (*info).weekdays_match & (1u32 << current_time.weekday) == 0 { return false; }
    if (*info).monthdays_match != XT_TIME_ALL_MONTHDAYS {
        localtime_3(&mut current_time, stamp);
        if (*info).monthdays_match & (1u32 << current_time.monthday) == 0 { return false; }
    }
    true
}

unsafe extern "C" fn time_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo;
    if (*info).daytime_start > XT_TIME_MAX_DAYTIME || (*info).daytime_stop > XT_TIME_MAX_DAYTIME { return -33; }
    if (*info).flags & !XT_TIME_ALL_FLAGS != 0 { return -22; }
    if (*info).flags & XT_TIME_CONTIGUOUS != 0 && (*info).daytime_start < (*info).daytime_stop { return -22; }
    0
}

static mut XT_TIME_MT_REG: xt_match = xt_match { name: b"time\0".as_ptr() as *const _, family: NFPROTO_UNSPEC, match_: Some(time_mt), checkentry: Some(time_mt_check), matchsize: core::mem::size_of::<xt_time_info>(), me: core::ptr::null_mut() };

#[no_mangle]
pub unsafe extern "C" fn time_mt_init() -> i32 { xt_register_match(&mut XT_TIME_MT_REG) }

#[no_mangle]
pub unsafe extern "C" fn time_mt_exit() { xt_unregister_match(&mut XT_TIME_MT_REG); }

// module_init(time_mt_init); module_exit(time_mt_exit);
// MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
// MODULE_DESCRIPTION("Xtables: time-based matching");
// MODULE_LICENSE("GPL"); MODULE_ALIAS("ipt_time"); MODULE_ALIAS("ip6t_time");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
