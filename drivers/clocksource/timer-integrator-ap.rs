// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Integrator/AP timer driver
 * Copyright (C) 2000-2003 Deep Blue Solutions Ltd
 * Copyright (c) 2014, Linaro Limited
 */

// Linux kernel headers and "timer-sp.h" provide the declarations and constants
// referenced below.

use core::ffi::c_void;

static mut SCHED_CLK_BASE: *mut c_void = core::ptr::null_mut();
static mut TIMER_RELOAD: usize = 0;
static mut CLKEVT_BASE: *mut c_void = core::ptr::null_mut();

extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: usize);
    fn clocksource_mmio_init(base: *mut c_void, name: *const u8, rating: u32, bits: u32,
                              shift: u32, read: *const c_void) -> i32;
    fn clocksource_mmio_readl_down() -> u32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> i32,
                   flags: u32, name: *const u8, dev_id: *mut c_void) -> i32;
    fn clockevents_config_and_register(dev: *mut clock_event_device, rate: usize,
                                       min_delta: u32, max_delta: u32);
    fn of_io_request_and_map(node: *mut device_node, index: i32, name: *const u8) -> *mut c_void;
    fn of_clk_get(node: *mut device_node, index: i32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn of_property_read_string(aliases: *mut device_node, name: *const u8, out: *mut *const u8) -> i32;
    fn of_find_node_by_path(path: *const u8) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn irq_of_parse_and_map(node: *mut device_node, index: i32) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
}

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clock_event_device {
    pub name: *const u8,
    pub features: u32,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub rating: i32,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

extern "C" {
    static mut of_aliases: *mut device_node;
}

const TIMER_VALUE: usize = 0;
const TIMER_LOAD: usize = 4;
const TIMER_CTRL: usize = 8;
const TIMER_INTCLR: usize = 12;
const TIMER_CTRL_ENABLE: u32 = 1;
const TIMER_CTRL_PERIODIC: u32 = 2;
const TIMER_CTRL_DIV16: u32 = 4;
const TIMER_CTRL_DIV256: u32 = 8;
const HZ: usize = 100;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 2;
const IRQ_HANDLED: i32 = 1;
const IRQF_TIMER: u32 = 0;
const IRQF_IRQPOLL: u32 = 0;

unsafe extern "C" fn integrator_read_sched_clock() -> u64 {
    (0u32.wrapping_sub(readl(SCHED_CLK_BASE.add(TIMER_VALUE)))) as u64
}

unsafe extern "C" fn integrator_clocksource_init(inrate: usize, base: *mut c_void) -> i32 {
    let mut ctrl = TIMER_CTRL_ENABLE | TIMER_CTRL_PERIODIC;
    let mut rate = inrate;
    if rate >= 1_500_000 { rate /= 16; ctrl |= TIMER_CTRL_DIV16; }
    writel(0xffff, base.add(TIMER_LOAD));
    writel(ctrl, base.add(TIMER_CTRL));
    let ret = clocksource_mmio_init(base.add(TIMER_VALUE), b"timer2\0".as_ptr(), 200, 16, 0,
                                    clocksource_mmio_readl_down as *const c_void);
    if ret != 0 { return ret; }
    SCHED_CLK_BASE = base;
    sched_clock_register(integrator_read_sched_clock, 16, rate);
    0
}

unsafe extern "C" fn integrator_timer_interrupt(_irq: i32, dev_id: *mut c_void) -> i32 {
    writel(1, CLKEVT_BASE.add(TIMER_INTCLR));
    let evt = dev_id as *mut clock_event_device;
    if let Some(handler) = (*evt).event_handler { handler(evt); }
    IRQ_HANDLED
}

unsafe extern "C" fn clkevt_shutdown(_evt: *mut clock_event_device) -> i32 {
    let ctrl = readl(CLKEVT_BASE.add(TIMER_CTRL)) & !TIMER_CTRL_ENABLE;
    writel(ctrl, CLKEVT_BASE.add(TIMER_CTRL)); 0
}

unsafe extern "C" fn clkevt_set_oneshot(_evt: *mut clock_event_device) -> i32 {
    let ctrl = readl(CLKEVT_BASE.add(TIMER_CTRL)) & !(TIMER_CTRL_ENABLE | TIMER_CTRL_PERIODIC);
    writel(ctrl, CLKEVT_BASE.add(TIMER_CTRL)); 0
}

unsafe extern "C" fn clkevt_set_periodic(_evt: *mut clock_event_device) -> i32 {
    let mut ctrl = readl(CLKEVT_BASE.add(TIMER_CTRL)) & !TIMER_CTRL_ENABLE;
    writel(ctrl, CLKEVT_BASE.add(TIMER_CTRL));
    writel(TIMER_RELOAD as u32, CLKEVT_BASE.add(TIMER_LOAD));
    ctrl |= TIMER_CTRL_PERIODIC | TIMER_CTRL_ENABLE;
    writel(ctrl, CLKEVT_BASE.add(TIMER_CTRL)); 0
}

unsafe extern "C" fn clkevt_set_next_event(next: usize, _evt: *mut clock_event_device) -> i32 {
    let ctrl = readl(CLKEVT_BASE.add(TIMER_CTRL));
    writel(ctrl & !TIMER_CTRL_ENABLE, CLKEVT_BASE.add(TIMER_CTRL));
    writel(next as u32, CLKEVT_BASE.add(TIMER_LOAD));
    writel(ctrl | TIMER_CTRL_ENABLE, CLKEVT_BASE.add(TIMER_CTRL)); 0
}

static mut INTEGRATOR_CLOCKEVENT: clock_event_device = clock_event_device {
    name: b"timer1\0".as_ptr(),
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    set_state_shutdown: Some(clkevt_shutdown), set_state_periodic: Some(clkevt_set_periodic),
    set_state_oneshot: Some(clkevt_set_oneshot), tick_resume: Some(clkevt_shutdown),
    set_next_event: Some(clkevt_set_next_event), rating: 300, event_handler: None,
};

unsafe extern "C" fn integrator_clockevent_init(inrate: usize, base: *mut c_void, irq: i32) -> i32 {
    let mut rate = inrate; let mut ctrl = 0u32; CLKEVT_BASE = base;
    if rate > 0x100000 * HZ { rate /= 256; ctrl |= TIMER_CTRL_DIV256; }
    else if rate > 0x10000 * HZ { rate /= 16; ctrl |= TIMER_CTRL_DIV16; }
    TIMER_RELOAD = rate / HZ; writel(ctrl, CLKEVT_BASE.add(TIMER_CTRL));
    let ret = request_irq(irq, integrator_timer_interrupt, IRQF_TIMER | IRQF_IRQPOLL,
                          b"timer\0".as_ptr(), &raw mut INTEGRATOR_CLOCKEVENT as *mut c_void);
    if ret != 0 { return ret; }
    clockevents_config_and_register(&raw mut INTEGRATOR_CLOCKEVENT, rate, 1, 0xffff); 0
}

unsafe extern "C" fn integrator_ap_timer_init_of(node: *mut device_node) -> i32 {
    let mut path: *const u8 = core::ptr::null();
    let base = of_io_request_and_map(node, 0, b"integrator-timer\0".as_ptr());
    if base.is_null() { return -1; }
    let clk = of_clk_get(node, 0);
    if clk.is_null() { return -1; }
    clk_prepare_enable(clk);
    let rate = clk_get_rate(clk);
    writel(0, base.add(TIMER_CTRL));

    let mut err = of_property_read_string(of_aliases, b"arm,timer-primary\0".as_ptr(), &mut path);
    if err != 0 { return err; }
    let mut alias_node = of_find_node_by_path(path);
    of_node_put(alias_node);
    if node == alias_node { return integrator_clocksource_init(rate, base); }

    err = of_property_read_string(of_aliases, b"arm,timer-secondary\0".as_ptr(), &mut path);
    if err != 0 { return err; }
    alias_node = of_find_node_by_path(path);
    of_node_put(alias_node);
    if node == alias_node {
        let irq = irq_of_parse_and_map(node, 0);
        return integrator_clockevent_init(rate, base, irq);
    }
    clk_disable_unprepare(clk);
    0
}

// The device-tree registration macro supplies the kernel's init-table entry.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
