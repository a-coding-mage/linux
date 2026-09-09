// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2000-2001 Deep Blue Solutions
// Copyright (C) 2002 Shane Nay (shane@minirl.com)
// Copyright (C) 2006-2007 Pavel Pisa (ppisa@pikron.com)
// Copyright (C) 2008 Juergen Beisert (kernel@pengutronix.de)
// Copyright (C) 2010 Freescale Semiconductor, Inc. All Rights Reserved.

// There are 2 versions of the timrot on Freescale MXS-based SoCs.
// The v1 on MX23 only gets 16 bits counter, while v2 on MX28
// extends the counter to 32 bits.
//
// The implementation uses two timers, one for clock_event and
// another for clocksource. MX28 uses timrot 0 and 1, while MX23
// uses 0 and 2.

const MX23_TIMROT_VERSION_OFFSET: usize = 0x0a0;
const MX28_TIMROT_VERSION_OFFSET: usize = 0x120;
const BP_TIMROT_MAJOR_VERSION: u32 = 24;
const BV_TIMROT_VERSION_1: u32 = 0x01;
const BV_TIMROT_VERSION_2: u32 = 0x02;

const HW_TIMROT_ROTCTRL: usize = 0x00;
const fn hw_timrot_timctrln(n: usize) -> usize { 0x20 + n * 0x40 }
const fn hw_timrot_timcountn(n: usize) -> usize { 0x30 + n * 0x40 }
const fn hw_timrot_running_countn(n: usize) -> usize { 0x30 + n * 0x40 }
const fn hw_timrot_fixed_countn(n: usize) -> usize { 0x40 + n * 0x40 }

const BM_TIMROT_TIMCTRLN_RELOAD: u32 = 1 << 6;
const BM_TIMROT_TIMCTRLN_UPDATE: u32 = 1 << 7;
const BM_TIMROT_TIMCTRLN_IRQ_EN: u32 = 1 << 14;
const BM_TIMROT_TIMCTRLN_IRQ: u32 = 1 << 15;
const BV_TIMROTV1_TIMCTRLN_SELECT_32KHZ_XTAL: u32 = 0x8;
const BV_TIMROTV2_TIMCTRLN_SELECT_TICK_ALWAYS: u32 = 0xf;

const STMP_OFFSET_REG_CLR: usize = 0x8;
const STMP_OFFSET_REG_SET: usize = 0x4;
const IRQ_HANDLED: i32 = 1;
const IRQF_TIMER: u32 = 0x00000002;
const IRQF_IRQPOLL: u32 = 0x00001000;

#[repr(C)] pub struct clock_event_device { pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)> , pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>, pub cpumask: *const core::ffi::c_void }
#[repr(C)] pub struct clocksource;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct device_node;

type U64 = u64;

extern "C" {
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn __raw_readl(addr: *mut core::ffi::c_void) -> u32;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn clockevent_state_oneshot(evt: *mut clock_event_device) -> bool;
    fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u32, min: u32, max: u32);
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
    fn clocksource_mmio_init(addr: *mut core::ffi::c_void, name: *const u8, hz: u32, rating: u32, bits: u32, read: *const core::ffi::c_void) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, hz: u32);
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn stmp_reset_block(addr: *mut core::ffi::c_void);
    fn of_device_is_compatible(np: *mut device_node, compat: *const u8) -> bool;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
}

static mut mxs_clockevent_device: clock_event_device = clock_event_device { event_handler: None, set_next_event: Some(timrotv2_set_next_event), cpumask: core::ptr::null() };
static mut clocksource_mxs: clocksource = clocksource;
static mut mxs_timrot_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut timrot_major_version: u32 = 0;

#[inline] unsafe fn timrot_is_v1() -> bool { timrot_major_version == BV_TIMROT_VERSION_1 }

unsafe fn timrot_irq_disable() { __raw_writel(BM_TIMROT_TIMCTRLN_IRQ_EN, mxs_timrot_base.add(hw_timrot_timctrln(0) + STMP_OFFSET_REG_CLR)); }
unsafe fn timrot_irq_enable() { __raw_writel(BM_TIMROT_TIMCTRLN_IRQ_EN, mxs_timrot_base.add(hw_timrot_timctrln(0) + STMP_OFFSET_REG_SET)); }
unsafe fn timrot_irq_acknowledge() { __raw_writel(BM_TIMROT_TIMCTRLN_IRQ, mxs_timrot_base.add(hw_timrot_timctrln(0) + STMP_OFFSET_REG_CLR)); }

unsafe extern "C" fn timrotv1_get_cycles(_cs: *mut clocksource) -> U64 { (!((__raw_readl(mxs_timrot_base.add(hw_timrot_timcountn(1))) & 0xffff0000) >> 16)) as U64 }
unsafe extern "C" fn timrotv1_set_next_event(evt: usize, _dev: *mut clock_event_device) -> i32 { __raw_writel(evt as u32, mxs_timrot_base.add(hw_timrot_timcountn(0))); 0 }
unsafe extern "C" fn timrotv2_set_next_event(evt: usize, _dev: *mut clock_event_device) -> i32 { __raw_writel(evt as u32, mxs_timrot_base.add(hw_timrot_fixed_countn(0))); 0 }

unsafe extern "C" fn mxs_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    timrot_irq_acknowledge();
    let evt = dev_id as *mut clock_event_device;
    if let Some(handler) = (*evt).event_handler { handler(evt); }
    IRQ_HANDLED
}

unsafe fn mxs_irq_clear(state: *const u8) {
    timrot_irq_disable();
    if timrot_is_v1() { __raw_writel(0xffff, mxs_timrot_base.add(hw_timrot_timcountn(1))); }
    else { __raw_writel(0xffffffff, mxs_timrot_base.add(hw_timrot_fixed_countn(1))); }
    timrot_irq_acknowledge();
    // pr_debug!("%s: changing mode to %s\n", __func__, state);
    let _ = state;
}

unsafe extern "C" fn mxs_shutdown(_evt: *mut clock_event_device) -> i32 { mxs_irq_clear(b"shutdown\0".as_ptr()); 0 }
unsafe extern "C" fn mxs_set_oneshot(evt: *mut clock_event_device) -> i32 { if clockevent_state_oneshot(evt) { mxs_irq_clear(b"oneshot\0".as_ptr()); } timrot_irq_enable(); 0 }

unsafe extern "C" fn mxs_read_sched_clock_v2() -> u64 { (!readl_relaxed(mxs_timrot_base.add(hw_timrot_running_countn(1)))) as u64 }

unsafe extern "C" fn mxs_clocksource_init(timer_clk: *mut clk) -> i32 {
    let c = clk_get_rate(timer_clk);
    if timrot_is_v1() { clocksource_register_hz(&mut clocksource_mxs, c); }
    else { clocksource_mmio_init(mxs_timrot_base.add(hw_timrot_running_countn(1)), b"mxs_timer\0".as_ptr(), c, 200, 32, core::ptr::null()); sched_clock_register(mxs_read_sched_clock_v2, 32, c); }
    0
}

unsafe extern "C" fn mxs_clockevent_init(timer_clk: *mut clk) -> i32 {
    mxs_clockevent_device.set_next_event = Some(if timrot_is_v1() { timrotv1_set_next_event } else { timrotv2_set_next_event });
    mxs_clockevent_device.cpumask = cpumask_of(0);
    clockevents_config_and_register(&mut mxs_clockevent_device, clk_get_rate(timer_clk), if timrot_is_v1() { 0xf } else { 0x2 }, if timrot_is_v1() { 0xfffe } else { 0xfffffffe });
    0
}

pub unsafe extern "C" fn mxs_timer_init(np: *mut device_node) -> i32 {
    mxs_timrot_base = of_iomap(np, 0);
    let timer_clk = of_clk_get(np, 0);
    if timer_clk.is_null() { return -1; }
    let ret = clk_prepare_enable(timer_clk); if ret != 0 { return ret; }
    stmp_reset_block(mxs_timrot_base.add(HW_TIMROT_ROTCTRL));
    let offset = if of_device_is_compatible(np, b"fsl,imx23-timrot\0".as_ptr()) { MX23_TIMROT_VERSION_OFFSET } else { MX28_TIMROT_VERSION_OFFSET };
    timrot_major_version = __raw_readl(mxs_timrot_base.add(offset)) >> BP_TIMROT_MAJOR_VERSION;
    let select = if timrot_is_v1() { BV_TIMROTV1_TIMCTRLN_SELECT_32KHZ_XTAL } else { BV_TIMROTV2_TIMCTRLN_SELECT_TICK_ALWAYS };
    __raw_writel(select | BM_TIMROT_TIMCTRLN_UPDATE | BM_TIMROT_TIMCTRLN_IRQ_EN, mxs_timrot_base.add(hw_timrot_timctrln(0)));
    __raw_writel(select | BM_TIMROT_TIMCTRLN_RELOAD, mxs_timrot_base.add(hw_timrot_timctrln(1)));
    if timrot_is_v1() { __raw_writel(0xffff, mxs_timrot_base.add(hw_timrot_timcountn(1))); } else { __raw_writel(0xffffffff, mxs_timrot_base.add(hw_timrot_fixed_countn(1))); }
    let ret = mxs_clocksource_init(timer_clk); if ret != 0 { return ret; }
    let ret = mxs_clockevent_init(timer_clk); if ret != 0 { return ret; }
    let irq = irq_of_parse_and_map(np, 0); if irq <= 0 { return -22; }
    request_irq(irq, mxs_timer_interrupt, IRQF_TIMER | IRQF_IRQPOLL, b"MXS Timer Tick\0".as_ptr(), &mut mxs_clockevent_device as *mut _ as *mut core::ffi::c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
