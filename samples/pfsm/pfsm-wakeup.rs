// SPDX-License-Identifier: GPL-2.0
/*
 * TPS6594 PFSM userspace example
 *
 * Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com/
 *
 * This example shows how to use PFSMs from a userspace application,
 * on TI j721s2 platform. The PMIC is armed to be triggered by a RTC
 * alarm to execute state transition (RETENTION to ACTIVE).
 */

// C headers supplied by the surrounding build provide these declarations:
// fcntl.h, stdio.h, sys/ioctl.h, unistd.h, linux/rtc.h,
// linux/tps6594_pfsm.h.

const ALARM_DELTA_SEC: i32 = 30;

const RTC_A: &[u8] = b"/dev/rtc0\0";

const PMIC_NB: usize = 3;
const PMIC_A: &[u8] = b"/dev/pfsm-0-0x48\0";
const PMIC_B: &[u8] = b"/dev/pfsm-0-0x4c\0";
const PMIC_C: &[u8] = b"/dev/pfsm-2-0x58\0";

static DEV_PFSM: [&[u8]; PMIC_NB] = [PMIC_A, PMIC_B, PMIC_C];

// External C types and constants are supplied by the included Linux headers.
use libc::{c_char, c_int, c_ulong};

#[repr(C)]
struct rtc_time {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
}

#[repr(C)]
struct pmic_state_opt {
    ddr_retention: c_int,
}

extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ... ) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn sleep(seconds: libc::c_uint) -> libc::c_uint;
    fn read(fd: c_int, buf: *mut libc::c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
}

// These ioctl request values are supplied by linux/rtc.h and
// linux/tps6594_pfsm.h in the target environment.
extern "C" {
    static RTC_RD_TIME: c_ulong;
    static RTC_ALM_SET: c_ulong;
    static RTC_AIE_ON: c_ulong;
    static RTC_AIE_OFF: c_ulong;
    static PMIC_SET_RETENTION_STATE: c_ulong;
    static PMIC_SET_ACTIVE_STATE: c_ulong;
}

pub unsafe fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let mut fd_rtc: c_int;
    let mut fd_pfsm: [c_int; PMIC_NB] = [0; PMIC_NB];
    let mut rtc_tm: rtc_time = core::mem::zeroed();
    let mut pmic_opt: pmic_state_opt = core::mem::zeroed();
    let mut data: c_ulong = 0;

    fd_rtc = open(RTC_A.as_ptr() as *const c_char, libc::O_RDONLY);
    if fd_rtc < 0 {
        perror(b"Failed to open RTC device.\0".as_ptr() as *const c_char);
        goto_out(&mut fd_rtc, &mut fd_pfsm);
        return 0;
    }

    i = 0;
    while i < PMIC_NB as c_int {
        fd_pfsm[i as usize] = open(DEV_PFSM[i as usize].as_ptr() as *const c_char, libc::O_RDWR);
        if fd_pfsm[i as usize] < 0 {
            perror(b"Failed to open PFSM device.\0".as_ptr() as *const c_char);
            goto_out(&mut fd_rtc, &mut fd_pfsm);
            return 0;
        }
        i += 1;
    }

    ret = ioctl(fd_rtc, RTC_RD_TIME, &mut rtc_tm);
    if ret < 0 {
        perror(b"Failed to read RTC date/time.\0".as_ptr() as *const c_char);
        goto_out(&mut fd_rtc, &mut fd_pfsm);
        return 0;
    }
    printf(b"Current RTC date/time is %d-%d-%d, %02d:%02d:%02d.\n\0".as_ptr() as *const c_char,
           rtc_tm.tm_mday, rtc_tm.tm_mon + 1, rtc_tm.tm_year + 1900,
           rtc_tm.tm_hour, rtc_tm.tm_min, rtc_tm.tm_sec);

    rtc_tm.tm_sec += ALARM_DELTA_SEC;
    if rtc_tm.tm_sec >= 60 { rtc_tm.tm_sec %= 60; rtc_tm.tm_min += 1; }
    if rtc_tm.tm_min == 60 { rtc_tm.tm_min = 0; rtc_tm.tm_hour += 1; }
    if rtc_tm.tm_hour == 24 { rtc_tm.tm_hour = 0; }
    ret = ioctl(fd_rtc, RTC_ALM_SET, &rtc_tm);
    if ret < 0 { perror(b"Failed to set RTC alarm.\0".as_ptr() as *const c_char); goto_out(&mut fd_rtc, &mut fd_pfsm); return 0; }
    ret = ioctl(fd_rtc, RTC_AIE_ON, 0);
    if ret < 0 { perror(b"Failed to enable alarm interrupts.\0".as_ptr() as *const c_char); goto_out(&mut fd_rtc, &mut fd_pfsm); return 0; }
    printf(b"Waiting %d seconds for alarm...\n\0".as_ptr() as *const c_char, ALARM_DELTA_SEC);

    pmic_opt.ddr_retention = 1;
    i = PMIC_NB as c_int - 1;
    while i >= 0 {
        printf(b"Set RETENTION state for PMIC_%d.\n\0".as_ptr() as *const c_char, i);
        sleep(1);
        ret = ioctl(fd_pfsm[i as usize], PMIC_SET_RETENTION_STATE, &mut pmic_opt);
        if ret < 0 { perror(b"Failed to set RETENTION state.\0".as_ptr() as *const c_char); break; }
        i -= 1;
    }
    if i < 0 {
        ret = read(fd_rtc, &mut data as *mut c_ulong as *mut libc::c_void, core::mem::size_of::<c_ulong>()) as c_int;
        if ret < 0 { perror(b"Failed to get RTC alarm.\0".as_ptr() as *const c_char); }
        else { puts(b"Alarm rang.\n\0".as_ptr() as *const c_char); }
    }
    ioctl(fd_rtc, RTC_AIE_OFF, 0);
    ioctl(fd_pfsm[0], PMIC_SET_ACTIVE_STATE, 0);
    goto_out(&mut fd_rtc, &mut fd_pfsm);
    0
}

unsafe fn goto_out(fd_rtc: &mut c_int, fd_pfsm: &mut [c_int; PMIC_NB]) {
    for fd in fd_pfsm.iter() { if *fd != 0 { close(*fd); } }
    if *fd_rtc != 0 { close(*fd_rtc); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
