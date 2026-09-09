// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contains driver for the Cadence Triple Timer Counter Rev 06
 *
 * Copyright (C) 2011-2013 Xilinx
 *
 * based on arch/mips/kernel/time.c timer driver
 */

// Linux kernel dependencies and build-time interfaces are supplied externally.

const TTC_CLK_CNTRL_OFFSET: usize = 0x00;
const TTC_CNT_CNTRL_OFFSET: usize = 0x0c;
const TTC_COUNT_VAL_OFFSET: usize = 0x18;
const TTC_INTR_VAL_OFFSET: usize = 0x24;
const TTC_ISR_OFFSET: usize = 0x54;
const TTC_IER_OFFSET: usize = 0x60;
const TTC_CNT_CNTRL_DISABLE_MASK: u32 = 0x1;
const TTC_CLK_CNTRL_CSRC_MASK: u32 = 1 << 5;
const TTC_CLK_CNTRL_PSV_MASK: u32 = 0x1e;
const TTC_CLK_CNTRL_PSV_SHIFT: u32 = 1;
const PRESCALE_EXPONENT: u32 = 11;
const PRESCALE: u32 = 2048;
const CLK_CNTRL_PRESCALE: u32 = (PRESCALE_EXPONENT - 1) << 1;
const CLK_CNTRL_PRESCALE_EN: u32 = 1;
const CNT_CNTRL_RESET: u32 = 1 << 4;
const MAX_F_ERR: u64 = 50;

#[repr(C)]
struct TtcTimer {
    base_addr: *mut core::ffi::c_void,
    freq: usize,
    clk: *mut Clk,
    clk_rate_change_nb: NotifierBlock,
}

#[repr(C)]
struct TtcTimerClocksource {
    scale_clk_ctrl_reg_old: u32,
    scale_clk_ctrl_reg_new: u32,
    ttc: TtcTimer,
    cs: Clocksource,
}

#[repr(C)]
struct TtcTimerClockevent {
    ttc: TtcTimer,
    ce: ClockEventDevice,
}

static mut TTC_SCHED_CLOCK_VAL_REG: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn ttc_set_interval(timer: *mut TtcTimer, cycles: usize) {
    let mut ctrl_reg = readl_relaxed((*timer).base_addr.add(TTC_CNT_CNTRL_OFFSET));
    ctrl_reg |= TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, (*timer).base_addr.add(TTC_CNT_CNTRL_OFFSET));
    writel_relaxed(cycles as u32, (*timer).base_addr.add(TTC_INTR_VAL_OFFSET));
    ctrl_reg |= CNT_CNTRL_RESET;
    ctrl_reg &= !TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, (*timer).base_addr.add(TTC_CNT_CNTRL_OFFSET));
}

unsafe extern "C" fn ttc_clock_event_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let ttce = dev_id as *mut TtcTimerClockevent;
    let timer = &mut (*ttce).ttc;
    readl_relaxed(timer.base_addr.add(TTC_ISR_OFFSET));
    ((*ttce).ce.event_handler.unwrap())(&mut (*ttce).ce);
    IRQ_HANDLED
}

unsafe extern "C" fn __ttc_clocksource_read(cs: *mut Clocksource) -> u64 {
    let ttccs = (cs as *mut u8).sub(core::mem::offset_of!(TtcTimerClocksource, cs)) as *mut TtcTimerClocksource;
    readl_relaxed((*ttccs).ttc.base_addr.add(TTC_COUNT_VAL_OFFSET)) as u64
}

unsafe extern "C" fn ttc_sched_clock_read() -> u64 {
    readl_relaxed(TTC_SCHED_CLOCK_VAL_REG) as u64
}

unsafe extern "C" fn ttc_set_next_event(cycles: usize, evt: *mut ClockEventDevice) -> i32 {
    let ttce = (evt as *mut u8).sub(core::mem::offset_of!(TtcTimerClockevent, ce)) as *mut TtcTimerClockevent;
    ttc_set_interval(&mut (*ttce).ttc, cycles);
    0
}

unsafe extern "C" fn ttc_shutdown(evt: *mut ClockEventDevice) -> i32 {
    let ttce = (evt as *mut u8).sub(core::mem::offset_of!(TtcTimerClockevent, ce)) as *mut TtcTimerClockevent;
    let timer = &mut (*ttce).ttc;
    let mut ctrl_reg = readl_relaxed(timer.base_addr.add(TTC_CNT_CNTRL_OFFSET));
    ctrl_reg |= TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, timer.base_addr.add(TTC_CNT_CNTRL_OFFSET));
    0
}

unsafe extern "C" fn ttc_set_periodic(evt: *mut ClockEventDevice) -> i32 {
    let ttce = (evt as *mut u8).sub(core::mem::offset_of!(TtcTimerClockevent, ce)) as *mut TtcTimerClockevent;
    ttc_set_interval(&mut (*ttce).ttc, div_round_closest((*ttce).ttc.freq, PRESCALE as usize * HZ));
    0
}

unsafe extern "C" fn ttc_resume(evt: *mut ClockEventDevice) -> i32 {
    let ttce = (evt as *mut u8).sub(core::mem::offset_of!(TtcTimerClockevent, ce)) as *mut TtcTimerClockevent;
    let timer = &mut (*ttce).ttc;
    let mut ctrl_reg = readl_relaxed(timer.base_addr.add(TTC_CNT_CNTRL_OFFSET));
    ctrl_reg &= !TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, timer.base_addr.add(TTC_CNT_CNTRL_OFFSET));
    0
}

// The remaining notifier, setup, probe, and platform-driver declarations retain
// the source interfaces and depend on the corresponding kernel definitions.
unsafe extern "C" fn ttc_rate_change_clocksource_cb(nb: *mut NotifierBlock, event: usize, data: *mut core::ffi::c_void) -> i32 {
    let ndata = data as *mut ClkNotifierData;
    let ttc = (nb as *mut u8).sub(core::mem::offset_of!(TtcTimer, clk_rate_change_nb)) as *mut TtcTimer;
    let ttccs = (ttc as *mut u8).sub(core::mem::offset_of!(TtcTimerClocksource, ttc)) as *mut TtcTimerClocksource;
    match event {
        PRE_RATE_CHANGE => {
            let (factor, rate_low, rate_high) = if (*ndata).new_rate > (*ndata).old_rate {
                (div_round_closest((*ndata).new_rate, (*ndata).old_rate), (*ndata).old_rate, (*ndata).new_rate)
            } else { (div_round_closest((*ndata).old_rate, (*ndata).new_rate), (*ndata).new_rate, (*ndata).old_rate) };
            if !is_power_of_2(factor) || abs_diff(rate_high, factor * rate_low) > MAX_F_ERR { return NOTIFY_BAD; }
            let shift = ilog2_u32(factor as u32);
            (*ttccs).scale_clk_ctrl_reg_old = readl_relaxed((*ttccs).ttc.base_addr.add(TTC_CLK_CNTRL_OFFSET));
            let mut psv = ((*ttccs).scale_clk_ctrl_reg_old & TTC_CLK_CNTRL_PSV_MASK) >> TTC_CLK_CNTRL_PSV_SHIFT;
            if (*ndata).new_rate < (*ndata).old_rate { psv -= shift; } else { psv += shift; }
            if psv & !(TTC_CLK_CNTRL_PSV_MASK >> TTC_CLK_CNTRL_PSV_SHIFT) != 0 { return NOTIFY_BAD; }
            (*ttccs).scale_clk_ctrl_reg_new = ((*ttccs).scale_clk_ctrl_reg_old & !TTC_CLK_CNTRL_PSV_MASK) | (psv << TTC_CLK_CNTRL_PSV_SHIFT);
            if (*ndata).new_rate < (*ndata).old_rate { return NOTIFY_DONE; }
            writel_relaxed((*ttccs).scale_clk_ctrl_reg_new, (*ttccs).ttc.base_addr.add(TTC_CLK_CNTRL_OFFSET));
        }
        POST_RATE_CHANGE => { if (*ndata).new_rate <= (*ndata).old_rate { writel_relaxed((*ttccs).scale_clk_ctrl_reg_new, (*ttccs).ttc.base_addr.add(TTC_CLK_CNTRL_OFFSET)); } }
        ABORT_RATE_CHANGE => { if (*ndata).new_rate >= (*ndata).old_rate { writel_relaxed((*ttccs).scale_clk_ctrl_reg_old, (*ttccs).ttc.base_addr.add(TTC_CLK_CNTRL_OFFSET)); } else { return NOTIFY_OK; } }
        _ => {}
    }
    NOTIFY_DONE
}

// External kernel types, constants, and helper functions are intentionally
// referenced rather than reimplemented here.

unsafe extern "C" fn ttc_rate_change_clockevent_cb(nb: *mut NotifierBlock, event: usize, data: *mut core::ffi::c_void) -> i32 {
    if event == POST_RATE_CHANGE {
        let ndata = data as *mut ClkNotifierData;
        let ttc = (nb as *mut u8).sub(core::mem::offset_of!(TtcTimer, clk_rate_change_nb)) as *mut TtcTimer;
        let ttcce = (ttc as *mut u8).sub(core::mem::offset_of!(TtcTimerClockevent, ttc)) as *mut TtcTimerClockevent;
        (*ttc).freq = (*ndata).new_rate;
        clockevents_update_freq(&mut (*ttcce).ce, (*ndata).new_rate / PRESCALE as usize);
    }
    NOTIFY_DONE
}

unsafe fn ttc_setup_clocksource(clk: *mut Clk, base: *mut core::ffi::c_void, timer_width: u32) -> i32 {
    let ttccs = kzalloc::<TtcTimerClocksource>();
    if ttccs.is_null() { return -12; }
    (*ttccs).ttc.clk = clk;
    let err = clk_prepare_enable(clk);
    if err != 0 { kfree(ttccs as *mut core::ffi::c_void); return err; }
    (*ttccs).ttc.freq = clk_get_rate(clk);
    (*ttccs).ttc.clk_rate_change_nb.notifier_call = Some(ttc_rate_change_clocksource_cb);
    clk_notifier_register(clk, &mut (*ttccs).ttc.clk_rate_change_nb);
    (*ttccs).ttc.base_addr = base;
    (*ttccs).cs.name = "ttc_clocksource";
    (*ttccs).cs.rating = 200;
    (*ttccs).cs.read = Some(__ttc_clocksource_read);
    (*ttccs).cs.mask = if timer_width == 64 { u64::MAX } else { (1u64 << timer_width) - 1 };
    (*ttccs).cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    writel_relaxed(0, base.add(TTC_IER_OFFSET));
    writel_relaxed(CLK_CNTRL_PRESCALE | CLK_CNTRL_PRESCALE_EN, base.add(TTC_CLK_CNTRL_OFFSET));
    writel_relaxed(CNT_CNTRL_RESET, base.add(TTC_CNT_CNTRL_OFFSET));
    let err = clocksource_register_hz(&mut (*ttccs).cs, (*ttccs).ttc.freq / PRESCALE as usize);
    if err != 0 { kfree(ttccs as *mut core::ffi::c_void); return err; }
    TTC_SCHED_CLOCK_VAL_REG = base.add(TTC_COUNT_VAL_OFFSET);
    sched_clock_register(ttc_sched_clock_read, timer_width, (*ttccs).ttc.freq / PRESCALE as usize);
    0
}

unsafe fn ttc_setup_clockevent(clk: *mut Clk, base: *mut core::ffi::c_void, irq: u32) -> i32 {
    let ttcce = kzalloc::<TtcTimerClockevent>();
    if ttcce.is_null() { return -12; }
    (*ttcce).ttc.clk = clk;
    let err = clk_prepare_enable(clk);
    if err != 0 { kfree(ttcce as *mut core::ffi::c_void); return err; }
    (*ttcce).ttc.freq = clk_get_rate(clk);
    (*ttcce).ttc.base_addr = base;
    (*ttcce).ce.name = "ttc_clockevent";
    (*ttcce).ce.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    (*ttcce).ce.set_next_event = Some(ttc_set_next_event);
    (*ttcce).ce.set_state_shutdown = Some(ttc_shutdown);
    (*ttcce).ce.set_state_periodic = Some(ttc_set_periodic);
    (*ttcce).ce.set_state_oneshot = Some(ttc_shutdown);
    (*ttcce).ce.tick_resume = Some(ttc_resume);
    (*ttcce).ce.rating = 200;
    (*ttcce).ce.irq = irq;
    writel_relaxed(0x23, base.add(TTC_CNT_CNTRL_OFFSET));
    writel_relaxed(CLK_CNTRL_PRESCALE | CLK_CNTRL_PRESCALE_EN, base.add(TTC_CLK_CNTRL_OFFSET));
    writel_relaxed(1, base.add(TTC_IER_OFFSET));
    let err = request_irq(irq, Some(ttc_clock_event_interrupt), IRQF_TIMER, (*ttcce).ce.name, ttcce as *mut core::ffi::c_void);
    if err != 0 { clk_disable_unprepare(clk); kfree(ttcce as *mut core::ffi::c_void); return err; }
    clockevents_config_and_register(&mut (*ttcce).ce, (*ttcce).ttc.freq / PRESCALE as usize, 1, 0xfffe);
    0
}

// Device-tree match table, platform-driver registration, and the probe entry
// point are supplied by the kernel's platform-driver macros in the C source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
