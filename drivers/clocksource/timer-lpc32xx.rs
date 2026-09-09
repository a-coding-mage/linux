// SPDX-License-Identifier: GPL-2.0
/*
 * Clocksource driver for NXP LPC32xx/18xx/43xx timer
 *
 * Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
 *
 * Based on:
 * time-efm32 Copyright (C) 2013 Pengutronix
 * mach-lpc32xx/timer.c Copyright (C) 2009 - 2010 NXP Semiconductors
 */

// Linux dependencies supplied by the surrounding kernel Rust bindings.

const LPC32XX_TIMER_IR: usize = 0x000;
const LPC32XX_TIMER_IR_MR0INT: u32 = 1 << 0;
const LPC32XX_TIMER_TCR: usize = 0x004;
const LPC32XX_TIMER_TCR_CEN: u32 = 1 << 0;
const LPC32XX_TIMER_TCR_CRST: u32 = 1 << 1;
const LPC32XX_TIMER_TC: usize = 0x008;
const LPC32XX_TIMER_PR: usize = 0x00c;
const LPC32XX_TIMER_MCR: usize = 0x014;
const LPC32XX_TIMER_MCR_MR0I: u32 = 1 << 0;
const LPC32XX_TIMER_MCR_MR0R: u32 = 1 << 1;
const LPC32XX_TIMER_MCR_MR0S: u32 = 1 << 2;
const LPC32XX_TIMER_MR0: usize = 0x018;
const LPC32XX_TIMER_CTCR: usize = 0x070;

#[repr(C)]
struct lpc32xx_clock_event_ddata {
    evtdev: clock_event_device,
    base: *mut core::ffi::c_void,
    ticks_per_jiffy: u32,
}

// Needed for the sched clock
static mut clocksource_timer_counter: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn of_clk_get_by_name(np: *mut device_node, name: *const core::ffi::c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn clocksource_mmio_init(base: *mut core::ffi::c_void, name: *const core::ffi::c_char,
                             rate: usize, rating: u32, bits: u32, read: unsafe extern "C" fn(*mut core::ffi::c_void) -> u32) -> i32;
    fn clocksource_mmio_readl_up(base: *mut core::ffi::c_void) -> u32;
    fn register_current_timer_delay(timer: *mut delay_timer);
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: usize);
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> u32;
    fn clockevents_config_and_register(evtdev: *mut clock_event_device, rate: usize, min_delta: u32, max_delta: i32);
    fn request_irq(irq: u32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const core::ffi::c_char, dev_id: *mut core::ffi::c_void) -> i32;
}

#[repr(C)] struct clock_event_device {
    name: *const core::ffi::c_char,
    features: u32,
    rating: u32,
    set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
}
#[repr(C)] struct delay_timer { read_current_timer: Option<unsafe extern "C" fn() -> usize>, freq: usize }
#[repr(C)] struct device_node;
#[repr(C)] struct clk;
type irqreturn_t = i32;

const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 0;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1 << 1;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TIMER: u32 = 1 << 4;
const IRQF_IRQPOLL: u32 = 1 << 8;
const HZ: usize = 100;
const EADDRNOTAVAIL: i32 = 99;
const ENOENT: i32 = 2;

unsafe extern "C" fn lpc32xx_read_sched_clock() -> u64 {
    readl(clocksource_timer_counter) as u64
}

unsafe extern "C" fn lpc32xx_delay_timer_read() -> usize {
    readl(clocksource_timer_counter) as usize
}

static mut lpc32xx_delay_timer: delay_timer = delay_timer { read_current_timer: Some(lpc32xx_delay_timer_read), freq: 0 };

unsafe extern "C" fn lpc32xx_clkevt_next_event(delta: usize, evtdev: *mut clock_event_device) -> i32 {
    let ddata = (evtdev as *mut u8).sub(core::mem::offset_of!(lpc32xx_clock_event_ddata, evtdev)) as *mut lpc32xx_clock_event_ddata;
    writel_relaxed(LPC32XX_TIMER_TCR_CRST, (*ddata).base.add(LPC32XX_TIMER_TCR));
    writel_relaxed(delta as u32, (*ddata).base.add(LPC32XX_TIMER_MR0));
    writel_relaxed(LPC32XX_TIMER_TCR_CEN, (*ddata).base.add(LPC32XX_TIMER_TCR));
    0
}

unsafe extern "C" fn lpc32xx_clkevt_shutdown(evtdev: *mut clock_event_device) -> i32 {
    let ddata = (evtdev as *mut u8).sub(core::mem::offset_of!(lpc32xx_clock_event_ddata, evtdev)) as *mut lpc32xx_clock_event_ddata;
    writel_relaxed(0, (*ddata).base.add(LPC32XX_TIMER_TCR));
    0
}

unsafe extern "C" fn lpc32xx_clkevt_oneshot(evtdev: *mut clock_event_device) -> i32 {
    let ddata = (evtdev as *mut u8).sub(core::mem::offset_of!(lpc32xx_clock_event_ddata, evtdev)) as *mut lpc32xx_clock_event_ddata;
    writel_relaxed(0, (*ddata).base.add(LPC32XX_TIMER_TCR));
    writel_relaxed(LPC32XX_TIMER_MCR_MR0I | LPC32XX_TIMER_MCR_MR0R | LPC32XX_TIMER_MCR_MR0S, (*ddata).base.add(LPC32XX_TIMER_MCR));
    0
}

unsafe extern "C" fn lpc32xx_clkevt_periodic(evtdev: *mut clock_event_device) -> i32 {
    let ddata = (evtdev as *mut u8).sub(core::mem::offset_of!(lpc32xx_clock_event_ddata, evtdev)) as *mut lpc32xx_clock_event_ddata;
    writel_relaxed(LPC32XX_TIMER_MCR_MR0I | LPC32XX_TIMER_MCR_MR0R, (*ddata).base.add(LPC32XX_TIMER_MCR));
    writel_relaxed(LPC32XX_TIMER_TCR_CRST, (*ddata).base.add(LPC32XX_TIMER_TCR));
    writel_relaxed((*ddata).ticks_per_jiffy, (*ddata).base.add(LPC32XX_TIMER_MR0));
    writel_relaxed(LPC32XX_TIMER_TCR_CEN, (*ddata).base.add(LPC32XX_TIMER_TCR));
    0
}

unsafe extern "C" fn lpc32xx_clock_event_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let ddata = dev_id as *mut lpc32xx_clock_event_ddata;
    writel_relaxed(LPC32XX_TIMER_IR_MR0INT, (*ddata).base.add(LPC32XX_TIMER_IR));
    0 // (*ddata).evtdev.event_handler(&mut (*ddata).evtdev)
}

static mut lpc32xx_clk_event_ddata: lpc32xx_clock_event_ddata = lpc32xx_clock_event_ddata {
    evtdev: clock_event_device {
        name: b"lpc3220 clockevent\0".as_ptr() as *const _,
        features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC,
        rating: 300,
        set_next_event: Some(lpc32xx_clkevt_next_event),
        set_state_shutdown: Some(lpc32xx_clkevt_shutdown),
        set_state_oneshot: Some(lpc32xx_clkevt_oneshot),
        set_state_periodic: Some(lpc32xx_clkevt_periodic),
    },
    base: core::ptr::null_mut(), ticks_per_jiffy: 0,
};

// Initialization routines retain the source control flow; external kernel registration APIs are dependencies.
unsafe extern "C" fn lpc32xx_clocksource_init(np: *mut device_node) -> i32 {
    let clk = of_clk_get_by_name(np, b"timerclk\0".as_ptr() as *const _);
    if clk.is_null() { return -1; }
    let mut ret = clk_prepare_enable(clk);
    if ret != 0 { clk_put(clk); return ret; }
    let base = of_iomap(np, 0);
    if base.is_null() { ret = -EADDRNOTAVAIL; clk_disable_unprepare(clk); clk_put(clk); return ret; }
    writel_relaxed(LPC32XX_TIMER_TCR_CRST, base.add(LPC32XX_TIMER_TCR));
    writel_relaxed(0, base.add(LPC32XX_TIMER_PR));
    writel_relaxed(0, base.add(LPC32XX_TIMER_MCR));
    writel_relaxed(0, base.add(LPC32XX_TIMER_CTCR));
    writel_relaxed(LPC32XX_TIMER_TCR_CEN, base.add(LPC32XX_TIMER_TCR));
    let rate = clk_get_rate(clk);
    ret = clocksource_mmio_init(base.add(LPC32XX_TIMER_TC), b"lpc3220 timer\0".as_ptr() as *const _, rate, 300, 32, clocksource_mmio_readl_up);
    if ret != 0 { iounmap(base); clk_disable_unprepare(clk); clk_put(clk); return ret; }
    clocksource_timer_counter = base.add(LPC32XX_TIMER_TC);
    lpc32xx_delay_timer.freq = rate;
    register_current_timer_delay(&mut lpc32xx_delay_timer);
    sched_clock_register(lpc32xx_read_sched_clock, 32, rate);
    0
}

unsafe extern "C" fn lpc32xx_clockevent_init(np: *mut device_node) -> i32 {
    let clk = of_clk_get_by_name(np, b"timerclk\0".as_ptr() as *const _);
    if clk.is_null() { return -1; }
    let mut ret = clk_prepare_enable(clk);
    if ret != 0 { clk_put(clk); return ret; }
    let base = of_iomap(np, 0);
    if base.is_null() { clk_disable_unprepare(clk); clk_put(clk); return -EADDRNOTAVAIL; }
    let irq = irq_of_parse_and_map(np, 0);
    if irq == 0 { iounmap(base); clk_disable_unprepare(clk); clk_put(clk); return -ENOENT; }
    writel_relaxed(0, base.add(LPC32XX_TIMER_TCR));
    writel_relaxed(0, base.add(LPC32XX_TIMER_PR));
    writel_relaxed(0, base.add(LPC32XX_TIMER_CTCR));
    writel_relaxed(LPC32XX_TIMER_IR_MR0INT, base.add(LPC32XX_TIMER_IR));
    let rate = clk_get_rate(clk);
    lpc32xx_clk_event_ddata.base = base;
    lpc32xx_clk_event_ddata.ticks_per_jiffy = (rate + HZ / 2) / HZ;
    clockevents_config_and_register(&mut lpc32xx_clk_event_ddata.evtdev, rate, 1, -1);
    ret = request_irq(irq, lpc32xx_clock_event_handler, IRQF_TIMER | IRQF_IRQPOLL, b"lpc3220 clockevent\0".as_ptr() as *const _, &mut lpc32xx_clk_event_ddata as *mut _ as *mut _);
    if ret != 0 { iounmap(base); clk_disable_unprepare(clk); clk_put(clk); return ret; }
    0
}

unsafe extern "C" fn lpc32xx_timer_init(np: *mut device_node) -> i32 {
    static mut has_clocksource: i32 = 0;
    static mut has_clockevent: i32 = 0;
    let mut ret = 0;
    if has_clocksource == 0 {
        ret = lpc32xx_clocksource_init(np);
        if ret == 0 { has_clocksource = 1; return 0; }
    }
    if has_clockevent == 0 {
        ret = lpc32xx_clockevent_init(np);
        if ret == 0 { has_clockevent = 1; return 0; }
    }
    ret
}

// TIMER_OF_DECLARE(lpc32xx_timer, "nxp,lpc3220-timer", lpc32xx_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
