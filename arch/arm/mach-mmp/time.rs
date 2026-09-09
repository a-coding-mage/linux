// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-mmp/time.c
 *
 *   Support for clocksource and clockevents
 *
 * Copyright (C) 2008 Marvell International Ltd.
 * All rights reserved.
 *
 *   2008-04-11: Jason Chagas <Jason.chagas@marvell.com>
 *   2008-10-08: Bin Yang <bin.yang@marvell.com>
 *
 * The timers module actually includes three timers, each timer with up to
 * three match comparators. Timer #0 is used here in free-running mode as the
 * clock source, and match comparator #1 used as clock event device.
 */

use core::ffi::c_void;

const MAX_DELTA: c_ulong = 0xfffffffe;
const MIN_DELTA: c_ulong = 16;

type c_ulong = usize;
type u32_ = u32;
type u64_ = u64;
type irqreturn_t = i32;

#[repr(C)]
struct clock_event_device {
    name: *const u8,
    features: u32,
    rating: i32,
    set_next_event: Option<unsafe extern "C" fn(c_ulong, *mut clock_event_device) -> i32>,
    set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    cpumask: *const c_void,
    event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

#[repr(C)]
struct clocksource {
    name: *const u8,
    rating: i32,
    read: Option<unsafe extern "C" fn(*mut clocksource) -> u64_>,
    mask: u64_,
    flags: u32,
}

extern "C" {
    fn __raw_writel(value: u32, addr: *mut c_void);
    fn __raw_readl(addr: *mut c_void) -> u32;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn cpumask_of(cpu: u32) -> *const c_void;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64_, bits: u32, rate: c_ulong);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t,
                   flags: u32, name: *const u8, dev_id: *mut c_void) -> i32;
    fn clocksource_register_hz(cs: *mut clocksource, rate: c_ulong) -> i32;
    fn clockevents_config_and_register(dev: *mut clock_event_device, rate: c_ulong,
                                       min_delta: c_ulong, max_delta: c_ulong);
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut c_void;
    fn cpu_is_mmp2() -> bool;
    fn cpu_is_mmp3() -> bool;
    fn cpu_is_pj4() -> bool;
    fn pr_err(fmt: *const u8, ...);
}

#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct clk { _private: [u8; 0] }

static mut mmp_timer_base: *mut c_void = core::ptr::null_mut();

// Timer register macros and platform constants are supplied by regs-timers.h and Linux headers.
extern "C" {
    fn TMR_CVWR(timer: u32) -> usize;
    fn TMR_ICR(timer: u32) -> usize;
    fn TMR_CER() -> usize;
    fn TMR_IER(timer: u32) -> usize;
    fn TMR_TN_MM(timer: u32, match_: u32) -> usize;
    fn TMR_CCR() -> usize;
    fn TMR_CMR() -> usize;
    fn TMR_PLCR(timer: u32) -> usize;
    fn TMR_CCR_CS_0(value: u32) -> u32;
    fn TMR_CCR_CS_1(value: u32) -> u32;
}

unsafe extern "C" fn timer_read() -> u32_ {
    let mut val = 0;
    let mut delay = 3;
    __raw_writel(1, mmp_timer_base.add(TMR_CVWR(1)));
    while delay != 0 {
        delay -= 1;
        val = __raw_readl(mmp_timer_base.add(TMR_CVWR(1)));
    }
    val
}

unsafe extern "C" fn mmp_read_sched_clock() -> u64_ { timer_read() as u64 }

unsafe extern "C" fn timer_interrupt(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    __raw_writel(0x01, mmp_timer_base.add(TMR_ICR(0)));
    __raw_writel(0x02, mmp_timer_base.add(TMR_CER()));
    let c = dev_id as *mut clock_event_device;
    if let Some(handler) = (*c).event_handler { handler(c); }
    1
}

unsafe extern "C" fn timer_set_next_event(delta: c_ulong, _dev: *mut clock_event_device) -> i32 {
    let mut flags = 0;
    local_irq_save(&mut flags);
    __raw_writel(0x02, mmp_timer_base.add(TMR_CER()));
    __raw_writel(0x01, mmp_timer_base.add(TMR_ICR(0)));
    __raw_writel(0x01, mmp_timer_base.add(TMR_IER(0)));
    __raw_writel((delta - 1) as u32, mmp_timer_base.add(TMR_TN_MM(0, 0)));
    __raw_writel(0x03, mmp_timer_base.add(TMR_CER()));
    local_irq_restore(flags);
    0
}

unsafe extern "C" fn timer_set_shutdown(_evt: *mut clock_event_device) -> i32 {
    let mut flags = 0;
    local_irq_save(&mut flags);
    __raw_writel(0, mmp_timer_base.add(TMR_IER(0)));
    local_irq_restore(flags);
    0
}

static mut ckevt: clock_event_device = clock_event_device {
    name: b"clockevent\0".as_ptr(), features: 1, rating: 200,
    set_next_event: Some(timer_set_next_event), set_state_shutdown: Some(timer_set_shutdown),
    set_state_oneshot: Some(timer_set_shutdown), cpumask: core::ptr::null(), event_handler: None,
};

unsafe extern "C" fn clksrc_read(_cs: *mut clocksource) -> u64_ { timer_read() as u64 }
static mut cksrc: clocksource = clocksource {
    name: b"clocksource\0".as_ptr(), rating: 200, read: Some(clksrc_read),
    mask: 0xffff_ffff, flags: 1,
};

unsafe extern "C" fn timer_config() {
    let mut ccr = __raw_readl(mmp_timer_base.add(TMR_CCR()));
    __raw_writel(0, mmp_timer_base.add(TMR_CER()));
    ccr &= if cpu_is_mmp2() || cpu_is_mmp3() { TMR_CCR_CS_0(0) | TMR_CCR_CS_1(0) } else { TMR_CCR_CS_0(3) | TMR_CCR_CS_1(3) };
    __raw_writel(ccr, mmp_timer_base.add(TMR_CCR()));
    __raw_writel(0x2, mmp_timer_base.add(TMR_CMR()));
    __raw_writel(0x1, mmp_timer_base.add(TMR_PLCR(0))); __raw_writel(0x7, mmp_timer_base.add(TMR_ICR(0))); __raw_writel(0, mmp_timer_base.add(TMR_IER(0)));
    __raw_writel(0, mmp_timer_base.add(TMR_PLCR(1))); __raw_writel(0x7, mmp_timer_base.add(TMR_ICR(1))); __raw_writel(0, mmp_timer_base.add(TMR_IER(1)));
    __raw_writel(0x2, mmp_timer_base.add(TMR_CER()));
}

unsafe extern "C" fn mmp_timer_init(irq: i32, rate: c_ulong) {
    timer_config(); sched_clock_register(mmp_read_sched_clock, 32, rate);
    ckevt.cpumask = cpumask_of(0);
    if request_irq(irq, timer_interrupt, 0, b"timer\0".as_ptr(), &mut ckevt as *mut _ as *mut c_void) != 0 { pr_err(b"Failed to request irq %d (timer)\n\0".as_ptr(), irq); }
    clocksource_register_hz(&mut cksrc, rate); clockevents_config_and_register(&mut ckevt, rate, MIN_DELTA, MAX_DELTA);
}

unsafe extern "C" fn mmp_dt_init_timer(np: *mut device_node) -> i32 {
    let clk = of_clk_get(np, 0); let rate;
    if !clk.is_null() { let ret = clk_prepare_enable(clk); if ret != 0 { return ret; } rate = clk_get_rate(clk); }
    else if cpu_is_pj4() { rate = 6500000; } else { rate = 3250000; }
    let irq = irq_of_parse_and_map(np, 0); if irq == 0 { return -22; }
    mmp_timer_base = of_iomap(np, 0); if mmp_timer_base.is_null() { return -12; }
    mmp_timer_init(irq, rate); 0
}

// TIMER_OF_DECLARE(mmp_timer, "mrvl,mmp-timer", mmp_dt_init_timer);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
