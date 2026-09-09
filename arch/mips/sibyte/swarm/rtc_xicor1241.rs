// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001 Broadcom Corporation
 *
 * Copyright (C) 2002 MontaVista Software Inc.
 * Author: jsun@mvista.com or jsun@junsun.net
 */

/* External declarations supplied by the kernel headers. */
use core::ffi::c_void;

type Time64T = i64;

#[repr(C)]
struct RtcTime {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
}

extern "C" {
    static mut rtc_lock: c_void;
    fn rtc_time64_to_tm(time: Time64T, tm: *mut RtcTime);
    fn bin2bcd(value: i32) -> i32;
    fn bcd2bin(value: u32) -> u32;
    fn mktime64(year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Time64T;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
    fn __raw_readq(addr: u64) -> u64;
    fn __raw_writeq(value: u64, addr: u64);
    fn A_SMB_REGISTER(bus: u64, reg: u64) -> u64;
    fn IOADDR(addr: u64) -> u64;
    fn V_SMB_ADDR(addr: u64) -> u64;
    fn V_SMB_TT_WR2BYTE() -> u64;
    fn V_SMB_TT_RD1BYTE() -> u64;
    fn V_SMB_TT_WR3BYTE() -> u64;
}

const X1241REG_SR_BAT: u64 = 0x80;
const X1241REG_SR_RWEL: u64 = 0x04;
const X1241REG_SR_WEL: u64 = 0x02;
const X1241REG_SR_RTCF: u64 = 0x01;
const X1241REG_BL_BP2: u64 = 0x80;
const X1241REG_BL_BP1: u64 = 0x40;
const X1241REG_BL_BP0: u64 = 0x20;
const X1241REG_BL_WD1: u64 = 0x10;
const X1241REG_BL_WD0: u64 = 0x08;
const X1241REG_HR_MIL: i32 = 0x80;

const X1241REG_BL: u8 = 0x10;
const X1241REG_INT: u8 = 0x11;
const X1241REG_SC: u8 = 0x30;
const X1241REG_MN: u8 = 0x31;
const X1241REG_HR: u8 = 0x32;
const X1241REG_DT: u8 = 0x33;
const X1241REG_MO: u8 = 0x34;
const X1241REG_YR: u8 = 0x35;
const X1241REG_DW: u8 = 0x36;
const X1241REG_Y2K: u8 = 0x37;
const X1241REG_SR: u8 = 0x3f;
const X1241_CCR_ADDRESS: u64 = 0x6f;

const R_SMB_STATUS: u64 = 0;
const R_SMB_CMD: u64 = 0;
const R_SMB_DATA: u64 = 0;
const R_SMB_START: u64 = 0;
const M_SMB_BUSY: u64 = 0;
const M_SMB_ERROR: u64 = 0;

#[inline]
unsafe fn smb_csr(reg: u64) -> u64 {
    IOADDR(A_SMB_REGISTER(1, reg))
}

unsafe fn xicor_read(addr: u8) -> i32 {
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    __raw_writeq(((addr as u64) >> 8) & 0x7, smb_csr(R_SMB_CMD));
    __raw_writeq((addr as u64) & 0xff, smb_csr(R_SMB_DATA));
    __raw_writeq(V_SMB_ADDR(X1241_CCR_ADDRESS) | V_SMB_TT_WR2BYTE(), smb_csr(R_SMB_START));
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    __raw_writeq(V_SMB_ADDR(X1241_CCR_ADDRESS) | V_SMB_TT_RD1BYTE(), smb_csr(R_SMB_START));
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    if __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_ERROR != 0 {
        /* Clear error bit by writing a 1 */
        __raw_writeq(M_SMB_ERROR, smb_csr(R_SMB_STATUS));
        return -1;
    }
    (__raw_readq(smb_csr(R_SMB_DATA)) & 0xff) as i32
}

unsafe fn xicor_write(addr: u8, b: i32) -> i32 {
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    __raw_writeq(addr as u64, smb_csr(R_SMB_CMD));
    __raw_writeq((addr as u64 & 0xff) | (((b as u64) & 0xff) << 8), smb_csr(R_SMB_DATA));
    __raw_writeq(V_SMB_ADDR(X1241_CCR_ADDRESS) | V_SMB_TT_WR3BYTE(), smb_csr(R_SMB_START));
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    if __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_ERROR != 0 {
        /* Clear error bit by writing a 1 */
        __raw_writeq(M_SMB_ERROR, smb_csr(R_SMB_STATUS));
        -1
    } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn xicor_set_time(t: Time64T) -> i32 {
    let mut tm = core::mem::MaybeUninit::<RtcTime>::uninit();
    rtc_time64_to_tm(t, tm.as_mut_ptr());
    let mut tm = tm.assume_init();
    let mut tmp: i32;
    let mut flags: usize = 0;
    tm.tm_year += 1900;
    spin_lock_irqsave(&mut rtc_lock, &mut flags);
    xicor_write(X1241REG_SR, X1241REG_SR_WEL as i32);
    xicor_write(X1241REG_SR, (X1241REG_SR_WEL | X1241REG_SR_RWEL) as i32);
    tm.tm_sec = bin2bcd(tm.tm_sec); xicor_write(X1241REG_SC, tm.tm_sec);
    tm.tm_min = bin2bcd(tm.tm_min); xicor_write(X1241REG_MN, tm.tm_min);
    tm.tm_mday = bin2bcd(tm.tm_mday); xicor_write(X1241REG_DT, tm.tm_mday);
    tm.tm_mon += 1; tm.tm_mon = bin2bcd(tm.tm_mon); xicor_write(X1241REG_MO, tm.tm_mon);
    tmp = tm.tm_year / 100; tm.tm_year %= 100;
    xicor_write(X1241REG_YR, tm.tm_year); xicor_write(X1241REG_Y2K, tmp);
    tmp = xicor_read(X1241REG_HR);
    if tmp & X1241REG_HR_MIL != 0 { tm.tm_hour = bin2bcd(tm.tm_hour); tmp = (tmp & !0x3f) | (tm.tm_hour & 0x3f); }
    else { tmp &= !0x3f; if tm.tm_hour >= 12 { tmp |= 0x20; tm.tm_hour -= 12; } tm.tm_hour = bin2bcd(tm.tm_hour); tmp |= tm.tm_hour; }
    xicor_write(X1241REG_HR, tmp); xicor_write(X1241REG_SR, 0);
    spin_unlock_irqrestore(&mut rtc_lock, flags); 0
}

#[no_mangle]
pub unsafe extern "C" fn xicor_get_time() -> Time64T {
    let (mut sec, mut min, mut hour, mut day, mut mon, mut year, mut y2k): (u32,u32,u32,u32,u32,u32,u32);
    let mut flags = 0usize; spin_lock_irqsave(&mut rtc_lock, &mut flags);
    sec=xicor_read(X1241REG_SC) as u32; min=xicor_read(X1241REG_MN) as u32; hour=xicor_read(X1241REG_HR) as u32;
    if hour & X1241REG_HR_MIL as u32 != 0 { hour &= 0x3f; } else if hour & 0x20 != 0 { hour=(hour&0xf)+0x12; }
    day=xicor_read(X1241REG_DT) as u32; mon=xicor_read(X1241REG_MO) as u32; year=xicor_read(X1241REG_YR) as u32; y2k=xicor_read(X1241REG_Y2K) as u32;
    spin_unlock_irqrestore(&mut rtc_lock, flags);
    sec=bcd2bin(sec); min=bcd2bin(min); hour=bcd2bin(hour); day=bcd2bin(day); mon=bcd2bin(mon); year=bcd2bin(year); y2k=bcd2bin(y2k);
    year += y2k * 100; mktime64(year, mon, day, hour, min, sec)
}

#[no_mangle]
pub unsafe extern "C" fn xicor_probe() -> i32 { (xicor_read(X1241REG_SC) != -1) as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
