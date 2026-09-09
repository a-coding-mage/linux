// SPDX-License-Identifier: GPL-2.0
/*
 * Allwinner SoCs hstimer driver.
 *
 * Copyright (C) 2013 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Linux dependencies supplied by other translated units.

const TIMER_IRQ_EN_REG: u32 = 0x00;
const TIMER_IRQ_ST_REG: u32 = 0x04;
const TIMER_CTL_ENABLE: u32 = 1 << 0;
const TIMER_CTL_RELOAD: u32 = 1 << 1;
const TIMER_CTL_ONESHOT: u32 = 1 << 7;
const TIMER_SYNC_TICKS: u32 = 3;

#[inline]
const fn timer_irq_en(val: u32) -> u32 { 1 << val }
#[inline]
const fn timer_ctl_reg(val: u32, offset: u32) -> u32 { 0x20 * val + 0x10 + offset }
#[inline]
const fn timer_ctl_clk_pres(val: u32) -> u32 { (val & 0x7) << 4 }
#[inline]
const fn timer_intval_lo_reg(val: u32, offset: u32) -> u32 { 0x20 * val + 0x14 + offset }
#[inline]
const fn timer_intval_hi_reg(val: u32, offset: u32) -> u32 { 0x20 * val + 0x18 + offset }
#[inline]
const fn timer_cntval_lo_reg(val: u32, offset: u32) -> u32 { 0x20 * val + 0x1c + offset }
#[inline]
const fn timer_cntval_hi_reg(val: u32, offset: u32) -> u32 { 0x20 * val + 0x20 + offset }

#[repr(C)]
struct SunxiTimerQuirks { from_ctl_base_offset: u32 }

#[repr(C)]
struct Sun5iTimer {
    base: *mut core::ffi::c_void,
    clk: *mut Clk,
    clk_rate_cb: NotifierBlock,
    ticks_per_jiffy: u32,
    clksrc: Clocksource,
    clkevt: ClockEventDevice,
    quirks: *const SunxiTimerQuirks,
}

unsafe fn sun5i_clkevt_sync(ce: *mut Sun5iTimer) {
    let offset = (*(*ce).quirks).from_ctl_base_offset;
    let old = readl((*ce).base.add(timer_cntval_lo_reg(1, offset) as usize));
    while old.wrapping_sub(readl((*ce).base.add(timer_cntval_lo_reg(1, offset) as usize))) < TIMER_SYNC_TICKS {
        cpu_relax();
    }
}

unsafe fn sun5i_clkevt_time_stop(ce: *mut Sun5iTimer, timer: u8) {
    let offset = (*(*ce).quirks).from_ctl_base_offset;
    let reg = timer_ctl_reg(timer as u32, offset);
    let val = readl((*ce).base.add(reg as usize));
    writel(val & !TIMER_CTL_ENABLE, (*ce).base.add(reg as usize));
    sun5i_clkevt_sync(ce);
}

unsafe fn sun5i_clkevt_time_setup(ce: *mut Sun5iTimer, timer: u8, delay: u32) {
    let offset = (*(*ce).quirks).from_ctl_base_offset;
    writel(delay, (*ce).base.add(timer_intval_lo_reg(timer as u32, offset) as usize));
}

unsafe fn sun5i_clkevt_time_start(ce: *mut Sun5iTimer, timer: u8, periodic: bool) {
    let offset = (*(*ce).quirks).from_ctl_base_offset;
    let reg = timer_ctl_reg(timer as u32, offset);
    let mut val = readl((*ce).base.add(reg as usize));
    if periodic { val &= !TIMER_CTL_ONESHOT; } else { val |= TIMER_CTL_ONESHOT; }
    writel(val | TIMER_CTL_ENABLE | TIMER_CTL_RELOAD, (*ce).base.add(reg as usize));
}

unsafe extern "C" fn sun5i_clkevt_shutdown(clkevt: *mut ClockEventDevice) -> i32 {
    let ce = clkevt_to_sun5i_timer(clkevt); sun5i_clkevt_time_stop(ce, 0); 0
}
unsafe extern "C" fn sun5i_clkevt_set_oneshot(clkevt: *mut ClockEventDevice) -> i32 {
    let ce = clkevt_to_sun5i_timer(clkevt); sun5i_clkevt_time_stop(ce, 0); sun5i_clkevt_time_start(ce, 0, false); 0
}
unsafe extern "C" fn sun5i_clkevt_set_periodic(clkevt: *mut ClockEventDevice) -> i32 {
    let ce = clkevt_to_sun5i_timer(clkevt); sun5i_clkevt_time_stop(ce, 0); sun5i_clkevt_time_setup(ce, 0, (*ce).ticks_per_jiffy); sun5i_clkevt_time_start(ce, 0, true); 0
}
unsafe extern "C" fn sun5i_clkevt_next_event(evt: usize, clkevt: *mut ClockEventDevice) -> i32 {
    let ce = clkevt_to_sun5i_timer(clkevt); sun5i_clkevt_time_stop(ce, 0); sun5i_clkevt_time_setup(ce, 0, (evt as u32).wrapping_sub(TIMER_SYNC_TICKS)); sun5i_clkevt_time_start(ce, 0, false); 0
}

unsafe extern "C" fn sun5i_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> Irqreturn {
    let ce = dev_id as *mut Sun5iTimer;
    writel(0x1, (*ce).base.add(TIMER_IRQ_ST_REG as usize));
    ((*ce).clkevt.event_handler.unwrap())(&mut (*ce).clkevt);
    IRQ_HANDLED
}

unsafe extern "C" fn sun5i_clksrc_read(clksrc: *mut Clocksource) -> u64 {
    let cs = clksrc_to_sun5i_timer(clksrc);
    (!readl((*cs).base.add(timer_cntval_lo_reg(1, (*(*cs).quirks).from_ctl_base_offset) as usize))) as u64
}

// Rate notifier and platform-driver setup are retained as direct declarations/operations below.
unsafe extern "C" fn sun5i_rate_cb(_nb: *mut NotifierBlock, _event: usize, _data: *mut core::ffi::c_void) -> i32 { NOTIFY_DONE }

#[repr(C)]
struct SunxiTimerQuirksExport { from_ctl_base_offset: u32 }

static SUN5I_SUN7I_HSTIMER_QUIRKS: SunxiTimerQuirks = SunxiTimerQuirks { from_ctl_base_offset: 0x0 };
static SUN20I_D1_HSTIMER_QUIRKS: SunxiTimerQuirks = SunxiTimerQuirks { from_ctl_base_offset: 0x10 };

// Device-tree matches:
// allwinner,sun5i-a13-hstimer -> SUN5I_SUN7I_HSTIMER_QUIRKS
// allwinner,sun7i-a20-hstimer -> SUN5I_SUN7I_HSTIMER_QUIRKS
// allwinner,sun20i-d1-hstimer -> SUN20I_D1_HSTIMER_QUIRKS

// The following kernel objects and helpers are supplied by other translated units.
extern "C" {
    fn sun5i_timer_probe(pdev: *mut PlatformDevice) -> i32;
    fn sun5i_timer_remove(pdev: *mut PlatformDevice);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
