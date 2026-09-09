// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada 370/XP SoC timer handling.
 *
 * Copyright (C) 2012 Marvell
 *
 * Lior Amsalem <alior@marvell.com>
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 * Timer 0 is used as free-running clocksource, while timer 1 is
 * used as clock_event_device.
 *
 * Clocksource driver for Armada 370 and Armada XP SoC.
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

const TIMER_CTRL_OFF: usize = 0x0000;
const TIMER0_EN: u32 = 1 << 0;
const TIMER0_RELOAD_EN: u32 = 1 << 1;
const TIMER0_25MHZ: u32 = 1 << 11;
const TIMER1_EN: u32 = 1 << 2;
const TIMER1_RELOAD_EN: u32 = 1 << 3;
const TIMER1_25MHZ: u32 = 1 << 12;
const TIMER_EVENTS_STATUS: usize = 0x0004;
const TIMER0_CLR_MASK: u32 = !0x1;
const TIMER1_CLR_MASK: u32 = !0x100;
const TIMER0_RELOAD_OFF: usize = 0x0010;
const TIMER0_VAL_OFF: usize = 0x0014;
const TIMER1_RELOAD_OFF: usize = 0x0018;
const TIMER1_VAL_OFF: usize = 0x001c;
const LCL_TIMER_EVENTS_STATUS: usize = 0x0028;
const TIMER_DIVIDER_SHIFT: u32 = 5;
const TIMER_DIVIDER: u32 = 1 << TIMER_DIVIDER_SHIFT;

#[inline]
const fn timer0_div(div: u32) -> u32 { div << 19 }
#[inline]
const fn timer1_div(div: u32) -> u32 { div << 22 }

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clock_event_device {
    pub name: *const u8, pub features: u32, pub shift: u32, pub rating: u32,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub irq: i32, pub cpumask: *const c_void,
    pub event_handler: unsafe extern "C" fn(*mut clock_event_device),
}
#[repr(C)] pub struct delay_timer { pub read_current_timer: Option<unsafe extern "C" fn() -> usize>, pub freq: u32 }
#[repr(C)] pub struct syscore_ops { pub suspend: Option<unsafe extern "C" fn(*mut c_void) -> i32>, pub resume: Option<unsafe extern "C" fn(*mut c_void)> }
#[repr(C)] pub struct syscore { pub ops: *const syscore_ops }

static mut timer_base: *mut u8 = core::ptr::null_mut();
static mut local_base: *mut u8 = core::ptr::null_mut();
static mut timer_clk: u32 = 0;
static mut timer25Mhz: bool = true;
static mut enable_mask: u32 = 0;
static mut ticks_per_jiffy: u32 = 0;
static mut armada_370_xp_evt: *mut clock_event_device = core::ptr::null_mut();

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(value: u32, addr: *mut u8);
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn of_clk_get_by_name(np: *mut device_node, name: *const u8) -> *mut clk;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32; fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> u32; fn ptr_err(p: *mut c_void) -> i32;
    fn atomic_io_modify(addr: *mut u8, clear: u32, set: u32);
    fn clockevents_config_and_register(e: *mut clock_event_device, freq: u32, min: u32, max: u32);
    fn enable_percpu_irq(irq: i32, flags: u32) -> i32; fn disable_percpu_irq(irq: i32);
    fn request_percpu_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> i32, name: *const u8, dev: *mut clock_event_device) -> i32;
    fn register_syscore(syscore: *mut syscore); fn register_current_timer_delay(timer: *mut delay_timer);
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn clocksource_mmio_init(addr: *mut u8, name: *const u8, rate: u32, rating: u32, shift: u32, read: *const c_void) -> i32;
    fn cpumask_of(cpu: u32) -> *const c_void;
}

unsafe fn local_timer_ctrl_clrset(clr: u32, set: u32) {
    writel((readl(local_base.add(TIMER_CTRL_OFF)) & !clr) | set, local_base.add(TIMER_CTRL_OFF));
}

unsafe extern "C" fn armada_370_xp_read_sched_clock() -> u64 { (!readl(timer_base.add(TIMER0_VAL_OFF))) as u64 }

unsafe extern "C" fn armada_370_xp_clkevt_next_event(delta: usize, _dev: *mut clock_event_device) -> i32 {
    writel(TIMER0_CLR_MASK, local_base.add(LCL_TIMER_EVENTS_STATUS)); writel(delta as u32, local_base.add(TIMER0_VAL_OFF));
    local_timer_ctrl_clrset(TIMER0_RELOAD_EN, enable_mask); 0
}
unsafe extern "C" fn armada_370_xp_clkevt_shutdown(_evt: *mut clock_event_device) -> i32 {
    local_timer_ctrl_clrset(TIMER0_EN, 0); writel(TIMER0_CLR_MASK, local_base.add(LCL_TIMER_EVENTS_STATUS)); 0
}
unsafe extern "C" fn armada_370_xp_clkevt_set_periodic(_evt: *mut clock_event_device) -> i32 {
    writel(ticks_per_jiffy - 1, local_base.add(TIMER0_RELOAD_OFF)); writel(ticks_per_jiffy - 1, local_base.add(TIMER0_VAL_OFF));
    local_timer_ctrl_clrset(0, TIMER0_RELOAD_EN | enable_mask); 0
}

static mut armada_370_xp_clkevt_irq: i32 = 0;
unsafe extern "C" fn armada_370_xp_timer_interrupt(_irq: i32, dev_id: *mut c_void) -> i32 {
    writel(TIMER0_CLR_MASK, local_base.add(LCL_TIMER_EVENTS_STATUS)); ((* (dev_id as *mut clock_event_device)).event_handler)(dev_id as *mut clock_event_device); 1
}

unsafe extern "C" fn armada_370_xp_timer_starting_cpu(cpu: u32) -> i32 {
    let evt = armada_370_xp_evt.add(cpu as usize); let (clr, set) = if timer25Mhz { (0, TIMER0_25MHZ) } else { (TIMER0_25MHZ, 0) }; local_timer_ctrl_clrset(clr, set);
    (*evt).name = b"armada_370_xp_per_cpu_tick\0".as_ptr(); (*evt).features = 0x3; (*evt).shift = 32; (*evt).rating = 300;
    (*evt).set_next_event = Some(armada_370_xp_clkevt_next_event); (*evt).set_state_shutdown = Some(armada_370_xp_clkevt_shutdown); (*evt).set_state_periodic = Some(armada_370_xp_clkevt_set_periodic); (*evt).set_state_oneshot = Some(armada_370_xp_clkevt_shutdown); (*evt).tick_resume = Some(armada_370_xp_clkevt_shutdown); (*evt).irq = armada_370_xp_clkevt_irq; (*evt).cpumask = cpumask_of(cpu); clockevents_config_and_register(evt, timer_clk, 1, 0xfffffffe); enable_percpu_irq((*evt).irq, 0); 0
}
unsafe extern "C" fn armada_370_xp_timer_dying_cpu(cpu: u32) -> i32 { disable_percpu_irq((*armada_370_xp_evt.add(cpu as usize)).irq); 0 }

static mut timer0_ctrl_reg: u32 = 0; static mut timer0_local_ctrl_reg: u32 = 0;
unsafe extern "C" fn armada_370_xp_timer_suspend(_data: *mut c_void) -> i32 { timer0_ctrl_reg = readl(timer_base.add(TIMER_CTRL_OFF)); timer0_local_ctrl_reg = readl(local_base.add(TIMER_CTRL_OFF)); 0 }
unsafe extern "C" fn armada_370_xp_timer_resume(_data: *mut c_void) { writel(0xffffffff, timer_base.add(TIMER0_VAL_OFF)); writel(0xffffffff, timer_base.add(TIMER0_RELOAD_OFF)); writel(timer0_ctrl_reg, timer_base.add(TIMER_CTRL_OFF)); writel(timer0_local_ctrl_reg, local_base.add(TIMER_CTRL_OFF)); }
static armada_370_xp_timer_syscore_ops: syscore_ops = syscore_ops { suspend: Some(armada_370_xp_timer_suspend), resume: Some(armada_370_xp_timer_resume) };
static mut armada_370_xp_timer_syscore: syscore = syscore { ops: &armada_370_xp_timer_syscore_ops };
unsafe extern "C" fn armada_370_delay_timer_read() -> usize { (!readl(timer_base.add(TIMER0_VAL_OFF))) as usize }
static mut armada_370_delay_timer: delay_timer = delay_timer { read_current_timer: Some(armada_370_delay_timer_read), freq: 0 };

unsafe extern "C" fn armada_370_xp_timer_common_init(np: *mut device_node) -> i32 {
    let (clr, set) = if timer25Mhz { (0, TIMER0_25MHZ) } else { (TIMER0_25MHZ, 0) };
    timer_base = of_iomap(np, 0); if timer_base.is_null() { return -6; }
    local_base = of_iomap(np, 1); if local_base.is_null() { return -6; }
    enable_mask = if timer25Mhz { TIMER0_EN } else { TIMER0_EN | timer0_div(TIMER_DIVIDER_SHIFT) };
    atomic_io_modify(timer_base.add(TIMER_CTRL_OFF), clr | set, set); local_timer_ctrl_clrset(clr, set);
    armada_370_xp_clkevt_irq = irq_of_parse_and_map(np, 4); ticks_per_jiffy = (timer_clk + 100 / 2) / 100;
    writel(0xffffffff, timer_base.add(TIMER0_VAL_OFF)); writel(0xffffffff, timer_base.add(TIMER0_RELOAD_OFF));
    atomic_io_modify(timer_base.add(TIMER_CTRL_OFF), TIMER0_RELOAD_EN | enable_mask, TIMER0_RELOAD_EN | enable_mask);
    armada_370_delay_timer.freq = timer_clk; register_current_timer_delay(&mut armada_370_delay_timer);
    sched_clock_register(armada_370_xp_read_sched_clock, 32, timer_clk);
    armada_370_xp_evt = core::ptr::null_mut();
    register_syscore(&mut armada_370_xp_timer_syscore); 0
}

unsafe extern "C" fn armada_xp_timer_init(np: *mut device_node) -> i32 {
    let clk = of_clk_get_by_name(np, b"fixed\0".as_ptr()); if clk.is_null() { return -6; }
    let ret = clk_prepare_enable(clk); if ret != 0 { return ret; } timer_clk = clk_get_rate(clk);
    let ret = armada_370_xp_timer_common_init(np); if ret != 0 { clk_disable_unprepare(clk); } ret
}
unsafe extern "C" fn armada_375_timer_init(np: *mut device_node) -> i32 {
    let clk = of_clk_get_by_name(np, b"fixed\0".as_ptr());
    if clk.is_null() { let fallback = of_clk_get(np, 0); if fallback.is_null() { return -6; } let ret = clk_prepare_enable(fallback); if ret != 0 { return ret; } timer_clk = clk_get_rate(fallback) / TIMER_DIVIDER; timer25Mhz = false; return armada_370_xp_timer_common_init(np); }
    let ret = clk_prepare_enable(clk); if ret != 0 { return ret; } timer_clk = clk_get_rate(clk); let ret = armada_370_xp_timer_common_init(np); if ret != 0 { clk_disable_unprepare(clk); } ret
}
unsafe extern "C" fn armada_370_timer_init(np: *mut device_node) -> i32 {
    let clk = of_clk_get(np, 0); if clk.is_null() { return -6; } let ret = clk_prepare_enable(clk); if ret != 0 { return ret; }
    timer_clk = clk_get_rate(clk) / TIMER_DIVIDER; timer25Mhz = false; let ret = armada_370_xp_timer_common_init(np); if ret != 0 { clk_disable_unprepare(clk); } ret
}

// TIMER_OF_DECLARE(armada_xp, "marvell,armada-xp-timer", armada_xp_timer_init);
// TIMER_OF_DECLARE(armada_xp, "marvell,armada-xp-timer", armada_xp_timer_init);
// TIMER_OF_DECLARE(armada_375, "marvell,armada-375-timer", armada_375_timer_init);
// TIMER_OF_DECLARE(armada_370, "marvell,armada-370-timer", armada_370_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
