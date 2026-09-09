// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008-2009 Manuel Lauss <manuel.lauss@gmail.com>
 *
 * Previous incarnations were:
 * Copyright (C) 2001, 2006, 2008 MontaVista Software, <source@mvista.com>
 * Copied and modified Carsten Langgaard's time.c
 *
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 1999,2000 MIPS Technologies, Inc.  All rights reserved.
 *
 * Clocksource/event using the 32.768kHz-clocked Counter1 ('RTC' in the
 * databooks).
 */

// Kernel and architecture dependencies are supplied by the surrounding translation unit.

const CNTR_OK: u32 = SYS_CNTRL_E0 | SYS_CNTRL_32S;

unsafe extern "C" {
    fn alchemy_rdsys(reg: u32) -> u64;
    fn alchemy_wrsys(value: u64, reg: u32);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32);
    fn div_sc(n: u32, d: u32, shift: u32) -> u32;
    fn clockevent_delta2ns(delta: u32, cd: *const clock_event_device) -> u64;
    fn clockevents_register_device(cd: *mut clock_event_device);
    fn request_irq(irq: u32, handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t>, flags: u32, name: *const u8, dev_id: *mut core::ffi::c_void) -> i32;
    fn pr_err(message: *const u8);
    fn printk(level: u32, message: *const u8);
    fn alchemy_get_cputype() -> i32;
}

#[repr(C)]
struct clocksource {
    name: *const u8,
    read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    mask: u64,
    flags: u32,
    rating: i32,
}

#[repr(C)]
struct clock_event_device {
    name: *const u8,
    features: u32,
    rating: i32,
    set_next_event: Option<unsafe extern "C" fn(u64, *mut clock_event_device) -> i32>,
    cpumask: *mut core::ffi::c_void,
    irq: u32,
    shift: u32,
    mult: u32,
    max_delta_ns: u64,
    max_delta_ticks: u32,
    min_delta_ns: u64,
    min_delta_ticks: u32,
    event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

type irqreturn_t = i32;

unsafe extern "C" fn au1x_counter1_read(_cs: *mut clocksource) -> u64 {
    alchemy_rdsys(AU1000_SYS_RTCREAD)
}

static mut au1x_counter1_clocksource: clocksource = clocksource {
    name: b"alchemy-counter1\0".as_ptr(),
    read: Some(au1x_counter1_read),
    mask: 0xffff_ffff,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    rating: 1500,
};

unsafe extern "C" fn au1x_rtcmatch2_set_next_event(delta: u64, _cd: *mut clock_event_device) -> i32 {
    let delta = delta.wrapping_add(alchemy_rdsys(AU1000_SYS_RTCREAD));
    while (alchemy_rdsys(AU1000_SYS_CNTRCTRL) as u32 & SYS_CNTRL_M21) != 0 {}
    alchemy_wrsys(delta, AU1000_SYS_RTCMATCH2);
    0
}

unsafe extern "C" fn au1x_rtcmatch2_irq(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let cd = dev_id as *mut clock_event_device;
    if let Some(handler) = (*cd).event_handler {
        handler(cd);
    }
    IRQ_HANDLED
}

static mut au1x_rtcmatch2_clockdev: clock_event_device = clock_event_device {
    name: b"rtcmatch2\0".as_ptr(),
    features: CLOCK_EVT_FEAT_ONESHOT,
    rating: 1500,
    set_next_event: Some(au1x_rtcmatch2_set_next_event),
    cpumask: cpu_possible_mask,
    irq: 0,
    shift: 0,
    mult: 0,
    max_delta_ns: 0,
    max_delta_ticks: 0,
    min_delta_ns: 0,
    min_delta_ticks: 0,
    event_handler: None,
};

unsafe extern "C" fn alchemy_time_init(m2int: u32) -> i32 {
    let cd = &mut au1x_rtcmatch2_clockdev;
    let mut t: u32;
    au1x_rtcmatch2_clockdev.irq = m2int;

    if CNTR_OK != (alchemy_rdsys(AU1000_SYS_CNTRCTRL) as u32 & CNTR_OK) { return -1; }

    t = 0x00ff_ffff;
    while (alchemy_rdsys(AU1000_SYS_CNTRCTRL) as u32 & SYS_CNTRL_T1S) != 0 && { t = t.wrapping_sub(1); t != 0 } {
        core::arch::asm!("nop");
    }
    if t == 0 { return -1; }
    alchemy_wrsys(0, AU1000_SYS_RTCTRIM);

    t = 0x00ff_ffff;
    while (alchemy_rdsys(AU1000_SYS_CNTRCTRL) as u32 & SYS_CNTRL_C1S) != 0 && { t = t.wrapping_sub(1); t != 0 } { core::arch::asm!("nop"); }
    if t == 0 { return -1; }
    alchemy_wrsys(0, AU1000_SYS_RTCWRITE);

    t = 0x00ff_ffff;
    while (alchemy_rdsys(AU1000_SYS_CNTRCTRL) as u32 & SYS_CNTRL_C1S) != 0 && { t = t.wrapping_sub(1); t != 0 } { core::arch::asm!("nop"); }
    if t == 0 { return -1; }

    clocksource_register_hz(&mut au1x_counter1_clocksource, 32768);
    cd.shift = 32;
    cd.mult = div_sc(32768, NSEC_PER_SEC, cd.shift);
    cd.max_delta_ns = clockevent_delta2ns(0xffff_ffff, cd);
    cd.max_delta_ticks = 0xffff_ffff;
    cd.min_delta_ns = clockevent_delta2ns(9, cd);
    cd.min_delta_ticks = 9;
    clockevents_register_device(cd);
    if request_irq(m2int, Some(au1x_rtcmatch2_irq), IRQF_TIMER, b"timer\0".as_ptr(), cd as *mut _ as *mut _) != 0 { pr_err(b"Failed to register timer interrupt\n\0".as_ptr()); }
    printk(KERN_INFO, b"Alchemy clocksource installed\n\0".as_ptr());
    0
}

static alchemy_m2inttab: [u32; 6] = [AU1000_RTC_MATCH2_INT, AU1500_RTC_MATCH2_INT, AU1100_RTC_MATCH2_INT, AU1550_RTC_MATCH2_INT, AU1200_RTC_MATCH2_INT, AU1300_RTC_MATCH2_INT];

pub unsafe extern "C" fn plat_time_init() {
    let t = alchemy_get_cputype();
    if t == ALCHEMY_CPU_UNKNOWN || alchemy_time_init(alchemy_m2inttab[t as usize]) != 0 { cpu_wait = core::ptr::null_mut(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
