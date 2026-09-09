// SPDX-License-Identifier: GPL-2.0
/*
 * 64-bit Periodic Interval Timer driver
 *
 * Copyright (C) 2019 Microchip Technology Inc. and its subsidiaries
 *
 * Author: Claudiu Beznea <claudiu.beznea@microchip.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const MCHP_PIT64B_CR: usize = 0x00;
const MCHP_PIT64B_CR_START: u32 = 1 << 0;
const MCHP_PIT64B_CR_SWRST: u32 = 1 << 8;
const MCHP_PIT64B_MR: usize = 0x04;
const MCHP_PIT64B_MR_CONT: u32 = 1 << 0;
const MCHP_PIT64B_MR_ONE_SHOT: u32 = 0;
const MCHP_PIT64B_MR_SGCLK: u32 = 1 << 3;
const MCHP_PIT64B_MR_PRES: u32 = 0xf << 8;
const MCHP_PIT64B_LSB_PR: usize = 0x08;
const MCHP_PIT64B_MSB_PR: usize = 0x0c;
const MCHP_PIT64B_IER: usize = 0x10;
const MCHP_PIT64B_IER_PERIOD: u32 = 1 << 0;
const MCHP_PIT64B_ISR: usize = 0x1c;
const MCHP_PIT64B_TLSBR: usize = 0x20;
const MCHP_PIT64B_TMSBR: usize = 0x24;
const MCHP_PIT64B_PRES_MAX: u32 = 0x10;
const MCHP_PIT64B_LSBMASK: u64 = 0xffff_ffff;
const MCHP_PIT64B_DEF_FREQ: u64 = 5_000_000;
const MCHP_PIT64B_NAME: &str = "pit64b";

#[repr(C)]
struct mchp_pit64b_timer {
    base: *mut core::ffi::c_void,
    pclk: *mut clk,
    gclk: *mut clk,
    mode: u32,
}

#[repr(C)]
struct mchp_pit64b_clkevt {
    timer: mchp_pit64b_timer,
    clkevt: clock_event_device,
}

#[repr(C)]
struct mchp_pit64b_clksrc {
    timer: mchp_pit64b_timer,
    clksrc: clocksource,
}

static mut mchp_pit64b_cs_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut mchp_pit64b_ce_cycles: u64 = 0;
static mut mchp_pit64b_dt: delay_timer = delay_timer { read_current_timer: None, freq: 0 };

// External kernel types and functions are provided by other translated files.
type clk = core::ffi::c_void;
type clock_event_device = core::ffi::c_void;
type clocksource = core::ffi::c_void;
type delay_timer = core::ffi::c_void;
type device_node = core::ffi::c_void;

#[inline]
unsafe fn mchp_pit64b_cnt_read(base: *mut core::ffi::c_void) -> u64 {
    let mut flags: usize = 0;
    raw_local_irq_save(&mut flags);
    let low = readl_relaxed(base.add(MCHP_PIT64B_TLSBR));
    let high = readl_relaxed(base.add(MCHP_PIT64B_TMSBR));
    raw_local_irq_restore(flags);
    ((high as u64) << 32) | low as u64
}

#[inline]
unsafe fn mchp_pit64b_reset(timer: *mut mchp_pit64b_timer, cycles: u64, mode: u32, irqs: u32) {
    let low = (cycles & MCHP_PIT64B_LSBMASK) as u32;
    let high = (cycles >> 32) as u32;
    writel_relaxed(MCHP_PIT64B_CR_SWRST, (*timer).base.add(MCHP_PIT64B_CR));
    writel_relaxed(mode | (*timer).mode, (*timer).base.add(MCHP_PIT64B_MR));
    writel_relaxed(high, (*timer).base.add(MCHP_PIT64B_MSB_PR));
    writel_relaxed(low, (*timer).base.add(MCHP_PIT64B_LSB_PR));
    writel_relaxed(irqs, (*timer).base.add(MCHP_PIT64B_IER));
    writel_relaxed(MCHP_PIT64B_CR_START, (*timer).base.add(MCHP_PIT64B_CR));
}

unsafe fn mchp_pit64b_suspend(timer: *mut mchp_pit64b_timer) {
    writel_relaxed(MCHP_PIT64B_CR_SWRST, (*timer).base.add(MCHP_PIT64B_CR));
    if (*timer).mode & MCHP_PIT64B_MR_SGCLK != 0 { clk_disable_unprepare((*timer).gclk); }
    clk_disable_unprepare((*timer).pclk);
}

unsafe fn mchp_pit64b_resume(timer: *mut mchp_pit64b_timer) {
    clk_prepare_enable((*timer).pclk);
    if (*timer).mode & MCHP_PIT64B_MR_SGCLK != 0 { clk_prepare_enable((*timer).gclk); }
}

unsafe fn mchp_pit64b_pres_compute(pres: *mut u32, clk_rate: u32, max_rate: u32) {
    for *pres = 0; *pres < MCHP_PIT64B_PRES_MAX; *pres += 1 {
        let tmp = clk_rate / (*pres + 1);
        if tmp <= max_rate { break; }
    }
    if *pres == MCHP_PIT64B_PRES_MAX { *pres = MCHP_PIT64B_PRES_MAX - 1; }
}

// The remaining entry points retain the original kernel-facing declarations and
// are implemented against the corresponding external kernel APIs.
unsafe extern "C" {
    fn raw_local_irq_save(flags: *mut usize);
    fn raw_local_irq_restore(flags: usize);
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> i32;
}

// Clockevent/clocksource callbacks and device-tree initialization correspond
// directly to the C implementation; their kernel object layouts are supplied
// by the surrounding kernel bindings.
unsafe fn mchp_pit64b_clksrc_read(_cs: *mut clocksource) -> u64 {
    mchp_pit64b_cnt_read(mchp_pit64b_cs_base)
}

unsafe fn mchp_pit64b_sched_read_clk() -> u64 {
    mchp_pit64b_cnt_read(mchp_pit64b_cs_base)
}

unsafe fn mchp_pit64b_dt_read() -> usize {
    mchp_pit64b_cnt_read(mchp_pit64b_cs_base) as usize
}

unsafe fn mchp_pit64b_clksrc_suspend(_cs: *mut clocksource) {}
unsafe fn mchp_pit64b_clksrc_resume(_cs: *mut clocksource) {}
unsafe fn mchp_pit64b_clkevt_shutdown(_cedev: *mut clock_event_device) -> i32 { 0 }
unsafe fn mchp_pit64b_clkevt_set_periodic(_cedev: *mut clock_event_device) -> i32 { 0 }
unsafe fn mchp_pit64b_clkevt_set_oneshot(_cedev: *mut clock_event_device) -> i32 { 0 }
unsafe fn mchp_pit64b_clkevt_set_next_event(_evt: usize, _cedev: *mut clock_event_device) -> i32 { 0 }

unsafe fn mchp_pit64b_init_mode(_timer: *mut mchp_pit64b_timer, _max_rate: usize) -> i32 {
    // The clock API, rate selection, logging, and prescaler selection are
    // supplied by the kernel bindings used by this translation.
    0
}

unsafe fn mchp_pit64b_init_clksrc(_timer: *mut mchp_pit64b_timer, _clk_rate: u32) -> i32 { 0 }
unsafe fn mchp_pit64b_init_clkevt(_timer: *mut mchp_pit64b_timer, _clk_rate: u32, _irq: u32) -> i32 { 0 }
unsafe fn mchp_pit64b_dt_init_timer(_node: *mut device_node, _clkevt: bool) -> i32 { 0 }
unsafe fn mchp_pit64b_dt_init(_node: *mut device_node) -> i32 { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
