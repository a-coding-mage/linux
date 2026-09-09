// SPDX-License-Identifier: GPL-2.0
/*
 * Real Time Clock interface for Linux on the BVME6000
 *
 * Based on the PC driver by Paul Gortmaker.
 */

// #define RTC_VERSION "1.00"
pub const RTC_VERSION: &[u8] = b"1.00\0";

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn local_irq_save(flags: *mut libc::c_ulong);
    fn local_irq_restore(flags: libc::c_ulong);
    fn capable(cap: libc::c_int) -> libc::c_int;
    fn copy_to_user(to: *mut libc::c_void, from: *const libc::c_void, n: usize) -> libc::c_ulong;
    fn copy_from_user(to: *mut libc::c_void, from: *const libc::c_void, n: usize) -> libc::c_ulong;
    fn misc_register(dev: *mut miscdevice) -> libc::c_int;
    fn pr_info(fmt: *const libc::c_char, ...);
}

extern "C" {
    static BVME_RTC_BASE: libc::uintptr_t;
    static MACH_IS_BVME6000: libc::c_int;
}

const EFAULT: libc::c_int = 14;
const EACCES: libc::c_int = 13;
const EINVAL: libc::c_int = 22;
const EBUSY: libc::c_int = 16;
const ENODEV: libc::c_int = 19;
const CAP_SYS_ADMIN: libc::c_int = 21;

extern "C" {
    static RTC_RD_TIME: libc::c_uint;
    static RTC_SET_TIME: libc::c_uint;
    static RTC_MINOR: libc::c_int;
}

#[repr(C)]
pub struct rtc_time {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
}

#[repr(C)]
pub struct Rtc {
    pub msr: u8,
    pub t0cr_rtmr: u8,
    pub bcd_tenms: u8,
    pub bcd_sec: u8,
    pub bcd_min: u8,
    pub bcd_hr: u8,
    pub bcd_dom: u8,
    pub bcd_mth: u8,
    pub bcd_year: u8,
    pub bcd_dow: u8,
}
pub type RtcPtr_t = *mut Rtc;

#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file_operations {
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, libc::c_uint, libc::c_ulong) -> libc::c_long>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> libc::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> libc::c_int>,
    pub llseek: Option<unsafe extern "C" fn() -> libc::c_long>,
}
#[repr(C)] pub struct miscdevice { pub minor: libc::c_int, pub name: *const libc::c_char, pub fops: *const file_operations }

extern "C" { fn atomic_dec_and_test(v: *mut libc::c_int) -> libc::c_int; fn atomic_inc(v: *mut libc::c_int); fn noop_llseek() -> libc::c_long; }

static mut DAYS_IN_MO: [u8; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
static mut RTC_STATUS: libc::c_int = 1;

#[inline] unsafe fn bcd2bin(x: u8) -> libc::c_int { ((x & 0x0f) as libc::c_int) + 10 * ((x >> 4) as libc::c_int) }
#[inline] unsafe fn bin2bcd(x: libc::c_int) -> u8 { (((x / 10) << 4) | (x % 10)) as u8 }

unsafe extern "C" fn rtc_ioctl(_file: *mut file, cmd: libc::c_uint, arg: libc::c_ulong) -> libc::c_long {
    let rtc = BVME_RTC_BASE as RtcPtr_t;
    let mut msr: u8;
    let mut flags: libc::c_ulong = 0;
    let mut wtime: rtc_time = core::mem::zeroed();
    let argp = arg as *mut libc::c_void;
    match cmd {
        RTC_RD_TIME => {
            local_irq_save(&mut flags); msr = (*rtc).msr & 0xc0; (*rtc).msr = 0x40;
            loop {
                wtime.tm_sec = bcd2bin((*rtc).bcd_sec); wtime.tm_min = bcd2bin((*rtc).bcd_min);
                wtime.tm_hour = bcd2bin((*rtc).bcd_hr); wtime.tm_mday = bcd2bin((*rtc).bcd_dom);
                wtime.tm_mon = bcd2bin((*rtc).bcd_mth) - 1; wtime.tm_year = bcd2bin((*rtc).bcd_year);
                if wtime.tm_year < 70 { wtime.tm_year += 100; } wtime.tm_wday = bcd2bin((*rtc).bcd_dow) - 1;
                if wtime.tm_sec == bcd2bin((*rtc).bcd_sec) { break; }
            }
            (*rtc).msr = msr; local_irq_restore(flags);
            if copy_to_user(argp, &wtime as *const _ as *const libc::c_void, core::mem::size_of::<rtc_time>()) != 0 { -(EFAULT as libc::c_long) } else { 0 }
        }
        RTC_SET_TIME => {
            let mut rtc_tm: rtc_time = core::mem::zeroed();
            if capable(CAP_SYS_ADMIN) == 0 { return -(EACCES as libc::c_long); }
            if copy_from_user(&mut rtc_tm as *mut _ as *mut libc::c_void, argp, core::mem::size_of::<rtc_time>()) != 0 { return -(EFAULT as libc::c_long); }
            let mut yrs = rtc_tm.tm_year; if yrs < 1900 { yrs += 1900; }
            let mon = rtc_tm.tm_mon + 1; let day = rtc_tm.tm_mday; let hrs = rtc_tm.tm_hour; let min = rtc_tm.tm_min; let sec = rtc_tm.tm_sec;
            let leap_yr = ((yrs % 4 == 0 && yrs % 100 != 0) || yrs % 400 == 0);
            if mon > 12 || mon < 1 || day == 0 { return -(EINVAL as libc::c_long); }
            if day > DAYS_IN_MO[mon as usize] as libc::c_int + if mon == 2 && leap_yr { 1 } else { 0 } || hrs >= 24 || min >= 60 || sec >= 60 || yrs >= 2070 { return -(EINVAL as libc::c_long); }
            local_irq_save(&mut flags); msr = (*rtc).msr & 0xc0; (*rtc).msr = 0x40;
            (*rtc).t0cr_rtmr = (yrs % 4) as u8; (*rtc).bcd_tenms = 0; (*rtc).bcd_sec = bin2bcd(sec); (*rtc).bcd_min = bin2bcd(min); (*rtc).bcd_hr = bin2bcd(hrs); (*rtc).bcd_dom = bin2bcd(day); (*rtc).bcd_mth = bin2bcd(mon); (*rtc).bcd_year = bin2bcd(yrs % 100);
            if rtc_tm.tm_wday >= 0 { (*rtc).bcd_dow = bin2bcd(rtc_tm.tm_wday + 1); } (*rtc).t0cr_rtmr = ((yrs % 4) | 0x08) as u8;
            (*rtc).msr = msr; local_irq_restore(flags); 0
        }
        _ => -(EINVAL as libc::c_long),
    }
}

unsafe extern "C" fn rtc_open(_inode: *mut inode, _file: *mut file) -> libc::c_int { if atomic_dec_and_test(&mut RTC_STATUS) == 0 { atomic_inc(&mut RTC_STATUS); -(EBUSY as libc::c_int) } else { 0 } }
unsafe extern "C" fn rtc_release(_inode: *mut inode, _file: *mut file) -> libc::c_int { atomic_inc(&mut RTC_STATUS); 0 }

static RTC_FOPS: file_operations = file_operations { unlocked_ioctl: Some(rtc_ioctl), open: Some(rtc_open), release: Some(rtc_release), llseek: Some(noop_llseek) };
static mut RTC_DEV: miscdevice = miscdevice { minor: 0, name: b"rtc\0".as_ptr() as *const libc::c_char, fops: &RTC_FOPS };

unsafe extern "C" fn rtc_DP8570A_init() -> libc::c_int { if MACH_IS_BVME6000 == 0 { return -ENODEV; } pr_info(b"DP8570A Real Time Clock Driver v%s\n\0".as_ptr() as *const libc::c_char, RTC_VERSION.as_ptr()); misc_register(&mut RTC_DEV) }
// module_init(rtc_DP8570A_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
