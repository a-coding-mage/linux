/*
 * arch/m68k/sun3/intersil.c
 *
 * Basic routines for accessing the intersil clock within the sun3 machines.
 *
 * Translated from the original C source.  Definitions supplied by the kernel
 * headers are referenced here as external dependencies.
 */

use core::ffi::c_ulong;

/* External kernel/architecture declarations supplied by the corresponding headers. */
extern "C" {
    static mut intersil_clock: *mut intersil_clock_regs;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
}

/* Layouts supplied by asm/intersil.h and linux/rtc.h. */
#[repr(C)]
pub struct intersil_dt {
    pub csec: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub month: u8,
    pub day: u8,
    pub year: u8,
    pub weekday: u8,
}

#[repr(C)]
pub struct intersil_clock_regs {
    pub counter: intersil_dt,
    pub cmd_reg: u8,
}

#[repr(C)]
pub struct rtc_time {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
}

/* Constants supplied by asm/intersil.h. */
extern "C" {
    static INTERSIL_STOP: u8;
    static INTERSIL_INT_ENABLE: u8;
    static INTERSIL_24H_MODE: u8;
    static INTERSIL_RUN: u8;
}

/* get/set hwclock */
pub unsafe fn sun3_hwclk(set: i32, t: *mut rtc_time) -> i32 {
    let todintersil: *mut intersil_dt =
        core::ptr::addr_of_mut!((*intersil_clock).counter);
    let mut flags: c_ulong = 0;
    /* bits to set for start/run of the intersil */
    let stop_val = INTERSIL_STOP | INTERSIL_INT_ENABLE | INTERSIL_24H_MODE;
    let start_val = INTERSIL_RUN | INTERSIL_INT_ENABLE | INTERSIL_24H_MODE;

    local_irq_save(&mut flags as *mut c_ulong);

    core::ptr::write_volatile(
        core::ptr::addr_of_mut!((*intersil_clock).cmd_reg),
        stop_val,
    );

    /* set or read the clock */
    if set != 0 {
        (*todintersil).csec = 0;
        (*todintersil).hour = (*t).tm_hour as u8;
        (*todintersil).minute = (*t).tm_min as u8;
        (*todintersil).second = (*t).tm_sec as u8;
        (*todintersil).month = ((*t).tm_mon + 1) as u8;
        (*todintersil).day = (*t).tm_mday as u8;
        (*todintersil).year = (((*t).tm_year - 68) % 100) as u8;
        (*todintersil).weekday = (*t).tm_wday as u8;
    } else {
        /* read clock */
        (*t).tm_sec = (*todintersil).csec as i32;
        (*t).tm_hour = (*todintersil).hour as i32;
        (*t).tm_min = (*todintersil).minute as i32;
        (*t).tm_sec = (*todintersil).second as i32;
        (*t).tm_mon = (*todintersil).month as i32 - 1;
        (*t).tm_mday = (*todintersil).day as i32;
        (*t).tm_year = (*todintersil).year as i32 + 68;
        (*t).tm_wday = (*todintersil).weekday as i32;
        if (*t).tm_year < 70 {
            (*t).tm_year += 100;
        }
    }

    core::ptr::write_volatile(
        core::ptr::addr_of_mut!((*intersil_clock).cmd_reg),
        start_val,
    );

    local_irq_restore(flags);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
