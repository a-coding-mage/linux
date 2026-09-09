// SPDX-License-Identifier: GPL-2.0
/*
 * Allwinner A1X SoCs timer handling.
 *
 * Copyright (C) 2012 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 *
 * Based on code from
 * Allwinner Technology Co., Ltd. <www.allwinnertech.com>
 * Benn Huang <benn@allwinnertech.com>
 */

const TIMER_IRQ_EN_REG: usize = 0x00;
const TIMER_IRQ_EN: fn(u32) -> u32 = |val| 1u32 << val;
const TIMER_IRQ_ST_REG: usize = 0x04;
const TIMER_IRQ_CLEAR: fn(u32) -> u32 = |val| 1u32 << val;
const TIMER_CTL_REG: fn(u8) -> usize = |val| 0x10 * val as usize + 0x10;
const TIMER_CTL_ENABLE: u32 = 1 << 0;
const TIMER_CTL_RELOAD: u32 = 1 << 1;
const TIMER_CTL_CLK_SRC: fn(u32) -> u32 = |val| (val & 0x3) << 2;
const TIMER_CTL_CLK_SRC_OSC24M: u32 = 1;
const TIMER_CTL_CLK_PRES: fn(u32) -> u32 = |val| (val & 0x7) << 4;
const TIMER_CTL_ONESHOT: u32 = 1 << 7;
const TIMER_INTVAL_REG: fn(u8) -> usize = |val| 0x10 * val as usize + 0x14;
const TIMER_CNTVAL_REG: fn(u8) -> usize = |val| 0x10 * val as usize + 0x18;
const TIMER_SYNC_TICKS: u32 = 3;

/* External kernel types and functions are supplied by the surrounding tree. */
use core::ffi::c_void;
extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn cpu_relax();
    fn to_timer_of(evt: *mut clock_event_device) -> *mut timer_of;
    fn timer_of_base(to: *mut timer_of) -> *mut c_void;
    fn timer_of_period(to: *mut timer_of) -> u32;
    fn timer_of_rate(to: *mut timer_of) -> u32;
    fn timer_of_init(node: *mut device_node, to: *mut timer_of) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn of_machine_is_compatible(compat: *const u8) -> bool;
    fn clocksource_mmio_init(base: *mut c_void, name: *const u8, rate: u32, rating: u32,
                              bits: u32, read: unsafe extern "C" fn(*mut c_void) -> u32) -> i32;
    fn clocksource_mmio_readl_down(base: *mut c_void) -> u32;
    fn clockevents_config_and_register(evt: *mut clock_event_device, rate: u32,
                                       min_delta: u32, max_delta: u32);
    fn pr_err(msg: *const u8);
    static mut cpu_possible_mask: *mut c_void;
}

#[repr(C)] pub struct device_node { pub name: *const u8 }
#[repr(C)] pub struct clock_event_device {
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}
#[repr(C)] pub struct timer_of { pub clkevt: clock_event_device }

unsafe fn sun4i_clkevt_sync(base: *mut c_void) {
    let old = readl((base as usize + TIMER_CNTVAL_REG(1)) as *mut c_void);
    while old.wrapping_sub(readl((base as usize + TIMER_CNTVAL_REG(1)) as *mut c_void)) < TIMER_SYNC_TICKS { cpu_relax(); }
}

unsafe fn sun4i_clkevt_time_stop(base: *mut c_void, timer: u8) {
    let val = readl((base as usize + TIMER_CTL_REG(timer)) as *mut c_void);
    writel(val & !TIMER_CTL_ENABLE, (base as usize + TIMER_CTL_REG(timer)) as *mut c_void);
    sun4i_clkevt_sync(base);
}

unsafe fn sun4i_clkevt_time_setup(base: *mut c_void, timer: u8, delay: u32) {
    writel(delay, (base as usize + TIMER_INTVAL_REG(timer)) as *mut c_void);
}

unsafe fn sun4i_clkevt_time_start(base: *mut c_void, timer: u8, periodic: bool) {
    let mut val = readl((base as usize + TIMER_CTL_REG(timer)) as *mut c_void);
    if periodic { val &= !TIMER_CTL_ONESHOT; } else { val |= TIMER_CTL_ONESHOT; }
    writel(val | TIMER_CTL_ENABLE | TIMER_CTL_RELOAD, (base as usize + TIMER_CTL_REG(timer)) as *mut c_void);
}

unsafe extern "C" fn sun4i_clkevt_shutdown(evt: *mut clock_event_device) -> i32 {
    sun4i_clkevt_time_stop(timer_of_base(to_timer_of(evt)), 0); 0
}
unsafe extern "C" fn sun4i_clkevt_set_oneshot(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt); let base = timer_of_base(to);
    sun4i_clkevt_time_stop(base, 0); sun4i_clkevt_time_start(base, 0, false); 0
}
unsafe extern "C" fn sun4i_clkevt_set_periodic(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt); let base = timer_of_base(to);
    sun4i_clkevt_time_stop(base, 0); sun4i_clkevt_time_setup(base, 0, timer_of_period(to));
    sun4i_clkevt_time_start(base, 0, true); 0
}
unsafe extern "C" fn sun4i_clkevt_next_event(evt: u32, clkevt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clkevt); let base = timer_of_base(to);
    sun4i_clkevt_time_stop(base, 0); sun4i_clkevt_time_setup(base, 0, evt.wrapping_sub(TIMER_SYNC_TICKS));
    sun4i_clkevt_time_start(base, 0, false); 0
}
unsafe fn sun4i_timer_clear_interrupt(base: *mut c_void) { writel(TIMER_IRQ_CLEAR(0), (base as usize + TIMER_IRQ_ST_REG) as *mut c_void); }
unsafe extern "C" fn sun4i_timer_interrupt(_irq: i32, dev_id: *mut c_void) -> i32 {
    let evt = dev_id as *mut clock_event_device; let to = to_timer_of(evt);
    sun4i_timer_clear_interrupt(timer_of_base(to)); if let Some(handler) = (*evt).event_handler { handler(evt); } 1
}

static mut TO: timer_of = timer_of { clkevt: clock_event_device { event_handler: None } };

unsafe extern "C" fn sun4i_timer_sched_read() -> u64 { (!readl((timer_of_base(&mut TO) as usize + TIMER_CNTVAL_REG(1)) as *mut c_void)) as u64 }

unsafe extern "C" fn sun4i_timer_init(node: *mut device_node) -> i32 {
    let mut ret = timer_of_init(node, &mut TO); if ret != 0 { return ret; }
    let base = timer_of_base(&mut TO);
    writel(!0, (base as usize + TIMER_INTVAL_REG(1)) as *mut c_void);
    writel(TIMER_CTL_ENABLE | TIMER_CTL_RELOAD | TIMER_CTL_CLK_SRC(TIMER_CTL_CLK_SRC_OSC24M), (base as usize + TIMER_CTL_REG(1)) as *mut c_void);
    if of_machine_is_compatible(b"allwinner,sun4i-a10\0") || of_machine_is_compatible(b"allwinner,sun5i-a13\0") || of_machine_is_compatible(b"allwinner,sun5i-a10s\0") || of_machine_is_compatible(b"allwinner,suniv-f1c100s\0") { sched_clock_register(sun4i_timer_sched_read, 32, timer_of_rate(&mut TO)); }
    ret = clocksource_mmio_init((base as usize + TIMER_CNTVAL_REG(1)) as *mut c_void, (*node).name, timer_of_rate(&mut TO), 350, 32, clocksource_mmio_readl_down);
    if ret != 0 { pr_err(b"Failed to register clocksource\0".as_ptr()); return ret; }
    writel(TIMER_CTL_CLK_SRC(TIMER_CTL_CLK_SRC_OSC24M), (base as usize + TIMER_CTL_REG(0)) as *mut c_void);
    sun4i_clkevt_time_stop(base, 0); sun4i_timer_clear_interrupt(base);
    clockevents_config_and_register(&mut TO.clkevt, timer_of_rate(&mut TO), TIMER_SYNC_TICKS + 1, 0xffffffff);
    let val = readl((base as usize + TIMER_IRQ_EN_REG) as *mut c_void); writel(val | TIMER_IRQ_EN(0), (base as usize + TIMER_IRQ_EN_REG) as *mut c_void); ret
}

// TIMER_OF_DECLARE(sun4i, "allwinner,sun4i-a10-timer", sun4i_timer_init);
// TIMER_OF_DECLARE(sun8i_a23, "allwinner,sun8i-a23-timer", sun4i_timer_init);
// TIMER_OF_DECLARE(sun8i_v3s, "allwinner,sun8i-v3s-timer", sun4i_timer_init);
// TIMER_OF_DECLARE(suniv, "allwinner,suniv-f1c100s-timer", sun4i_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
