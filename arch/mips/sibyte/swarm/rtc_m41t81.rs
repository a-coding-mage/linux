// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001 Broadcom Corporation
 *
 * Copyright (C) 2002 MontaVista Software Inc.
 * Author: jsun@mvista.com or jsun@junsun.net
 */

// External definitions supplied by the Linux/MIPS environment.

/* M41T81 definitions */

/* Register bits */
const M41T81REG_SC_ST: u8 = 0x80;
const M41T81REG_HR_CB: u8 = 0x40;
const M41T81REG_HR_CEB: u8 = 0x80;
const M41T81REG_CTL_S: u8 = 0x20;
const M41T81REG_CTL_FT: u8 = 0x40;
const M41T81REG_CTL_OUT: u8 = 0x80;
const M41T81REG_WD_RB0: u8 = 0x01;
const M41T81REG_WD_RB1: u8 = 0x02;
const M41T81REG_WD_BMB0: u8 = 0x04;
const M41T81REG_WD_BMB1: u8 = 0x08;
const M41T81REG_WD_BMB2: u8 = 0x10;
const M41T81REG_WD_BMB3: u8 = 0x20;
const M41T81REG_WD_BMB4: u8 = 0x40;
const M41T81REG_AMO_ABE: u8 = 0x20;
const M41T81REG_AMO_SQWE: u8 = 0x40;
const M41T81REG_AMO_AFE: u8 = 0x80;
const M41T81REG_ADT_RPT5: u8 = 0x40;
const M41T81REG_ADT_RPT4: u8 = 0x80;
const M41T81REG_AHR_RPT3: u8 = 0x80;
const M41T81REG_AHR_HT: u8 = 0x40;
const M41T81REG_AMN_RPT2: u8 = 0x80;
const M41T81REG_ASC_RPT1: u8 = 0x80;
const M41T81REG_FLG_AF: u8 = 0x40;
const M41T81REG_FLG_WDF: u8 = 0x80;
const M41T81REG_SQW_RS0: u8 = 0x10;
const M41T81REG_SQW_RS1: u8 = 0x20;
const M41T81REG_SQW_RS2: u8 = 0x40;
const M41T81REG_SQW_RS3: u8 = 0x80;

/* Register numbers */
const M41T81REG_TSC: u8 = 0x00;
const M41T81REG_SC: u8 = 0x01;
const M41T81REG_MN: u8 = 0x02;
const M41T81REG_HR: u8 = 0x03;
const M41T81REG_DY: u8 = 0x04;
const M41T81REG_DT: u8 = 0x05;
const M41T81REG_MO: u8 = 0x06;
const M41T81REG_YR: u8 = 0x07;
const M41T81REG_CTL: u8 = 0x08;
const M41T81REG_WD: u8 = 0x09;
const M41T81REG_AMO: u8 = 0x0A;
const M41T81REG_ADT: u8 = 0x0B;
const M41T81REG_AHR: u8 = 0x0C;
const M41T81REG_AMN: u8 = 0x0D;
const M41T81REG_ASC: u8 = 0x0E;
const M41T81REG_FLG: u8 = 0x0F;
const M41T81REG_SQW: u8 = 0x13;
const M41T81_CCR_ADDRESS: u64 = 0x68;

// SMB_CSR(reg) = IOADDR(A_SMB_REGISTER(1, reg));

extern "C" {
    static mut rtc_lock: core::ffi::c_void;
    fn __raw_readq(addr: u64) -> u64;
    fn __raw_writeq(value: u64, addr: u64);
    fn A_SMB_REGISTER(bus: u64, reg: u64) -> u64;
    fn IOADDR(addr: u64) -> u64;
    fn V_SMB_ADDR(addr: u64) -> u64;
    fn V_SMB_TT_WR1BYTE: u64;
    fn V_SMB_TT_RD1BYTE: u64;
    fn V_SMB_TT_WR2BYTE: u64;
    fn R_SMB_STATUS: u64;
    fn R_SMB_CMD: u64;
    fn R_SMB_START: u64;
    fn R_SMB_DATA: u64;
    fn M_SMB_BUSY: u64;
    fn M_SMB_ERROR: u64;
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut u64);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: u64);
    fn bin2bcd(value: i32) -> i32;
    fn bcd2bin(value: u32) -> u32;
    fn rtc_time64_to_tm(t: i64, tm: *mut rtc_time);
    fn mktime64(year: u32, mon: u32, day: u32, hour: u32, min: u32, sec: u32) -> i64;
}

#[repr(C)]
struct rtc_time {
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

#[inline]
unsafe fn smb_csr(reg: u64) -> u64 {
    IOADDR(A_SMB_REGISTER(1, reg))
}

unsafe fn m41t81_read(addr: u8) -> i32 {
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    __raw_writeq((addr & 0xff) as u64, smb_csr(R_SMB_CMD));
    __raw_writeq(V_SMB_ADDR(M41T81_CCR_ADDRESS) | V_SMB_TT_WR1BYTE(), smb_csr(R_SMB_START));
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    __raw_writeq(V_SMB_ADDR(M41T81_CCR_ADDRESS) | V_SMB_TT_RD1BYTE(), smb_csr(R_SMB_START));
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    if __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_ERROR != 0 {
        __raw_writeq(M_SMB_ERROR, smb_csr(R_SMB_STATUS));
        return -1;
    }
    (__raw_readq(smb_csr(R_SMB_DATA)) & 0xff) as i32
}

unsafe fn m41t81_write(addr: u8, b: i32) -> i32 {
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    __raw_writeq((addr & 0xff) as u64, smb_csr(R_SMB_CMD));
    __raw_writeq((b & 0xff) as u64, smb_csr(R_SMB_DATA));
    __raw_writeq(V_SMB_ADDR(M41T81_CCR_ADDRESS) | V_SMB_TT_WR2BYTE(), smb_csr(R_SMB_START));
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    if __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_ERROR != 0 {
        __raw_writeq(M_SMB_ERROR, smb_csr(R_SMB_STATUS));
        return -1;
    }
    __raw_writeq(V_SMB_ADDR(M41T81_CCR_ADDRESS) | V_SMB_TT_RD1BYTE(), smb_csr(R_SMB_START));
    while __raw_readq(smb_csr(R_SMB_STATUS)) & M_SMB_BUSY != 0 {}
    0
}

pub unsafe fn m41t81_set_time(t: i64) -> i32 {
    let mut tm = core::mem::MaybeUninit::<rtc_time>::uninit();
    let mut flags = 0u64;
    rtc_time64_to_tm(t, tm.as_mut_ptr());
    let tm = tm.assume_init_mut();
    spin_lock_irqsave(&mut rtc_lock, &mut flags);
    tm.tm_sec = bin2bcd(tm.tm_sec); m41t81_write(M41T81REG_SC, tm.tm_sec);
    tm.tm_min = bin2bcd(tm.tm_min); m41t81_write(M41T81REG_MN, tm.tm_min);
    tm.tm_hour = bin2bcd(tm.tm_hour);
    tm.tm_hour = (tm.tm_hour & 0x3f) | (m41t81_read(M41T81REG_HR) & 0xc0);
    m41t81_write(M41T81REG_HR, tm.tm_hour);
    if tm.tm_wday == 0 { tm.tm_wday = 7; }
    tm.tm_wday = bin2bcd(tm.tm_wday); m41t81_write(M41T81REG_DY, tm.tm_wday);
    tm.tm_mday = bin2bcd(tm.tm_mday); m41t81_write(M41T81REG_DT, tm.tm_mday);
    tm.tm_mon += 1; tm.tm_mon = bin2bcd(tm.tm_mon); m41t81_write(M41T81REG_MO, tm.tm_mon);
    tm.tm_year %= 100; tm.tm_year = bin2bcd(tm.tm_year); m41t81_write(M41T81REG_YR, tm.tm_year);
    spin_unlock_irqrestore(&mut rtc_lock, flags);
    0
}

pub unsafe fn m41t81_get_time() -> i64 {
    let (mut year, mut mon, mut day, mut hour, mut min, mut sec): (u32, u32, u32, u32, u32, u32);
    let mut flags = 0u64;
    loop {
        spin_lock_irqsave(&mut rtc_lock, &mut flags);
        sec = m41t81_read(M41T81REG_SC) as u32;
        min = m41t81_read(M41T81REG_MN) as u32;
        if sec == m41t81_read(M41T81REG_SC) as u32 { break; }
        spin_unlock_irqrestore(&mut rtc_lock, flags);
    }
    hour = (m41t81_read(M41T81REG_HR) & 0x3f) as u32;
    day = m41t81_read(M41T81REG_DT) as u32;
    mon = m41t81_read(M41T81REG_MO) as u32;
    year = m41t81_read(M41T81REG_YR) as u32;
    spin_unlock_irqrestore(&mut rtc_lock, flags);
    sec = bcd2bin(sec); min = bcd2bin(min); hour = bcd2bin(hour);
    day = bcd2bin(day); mon = bcd2bin(mon); year = bcd2bin(year);
    year += 2000;
    mktime64(year, mon, day, hour, min, sec)
}

pub unsafe fn m41t81_probe() -> i32 {
    let tmp = m41t81_read(M41T81REG_SC);
    m41t81_write(M41T81REG_SC, tmp & 0x7f);
    if m41t81_read(M41T81REG_SC) != -1 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
