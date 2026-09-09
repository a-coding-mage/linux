// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Maxime Coquelin 2015
 * Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
 *
 * Inspired by time-efm32.c from Uwe Kleine-Koenig
 */

// Linux kernel dependencies supplied by the surrounding translation.
// #include <linux/kernel.h>
// #include <linux/clocksource.h>
// #include <linux/clockchips.h>
// #include <linux/delay.h>
// #include <linux/irq.h>
// #include <linux/interrupt.h>
// #include <linux/of.h>
// #include <linux/of_address.h>
// #include <linux/of_irq.h>
// #include <linux/clk.h>
// #include <linux/reset.h>
// #include <linux/sched_clock.h>
// #include <linux/slab.h>
// #include "timer-of.h"

const TIM_CR1: usize = 0x00;
const TIM_DIER: usize = 0x0c;
const TIM_SR: usize = 0x10;
const TIM_EGR: usize = 0x14;
const TIM_CNT: usize = 0x24;
const TIM_PSC: usize = 0x28;
const TIM_ARR: usize = 0x2c;
const TIM_CCR1: usize = 0x34;

const TIM_CR1_CEN: u32 = 1 << 0;
const TIM_CR1_UDIS: u32 = 1 << 1;
const TIM_CR1_OPM: u32 = 1 << 3;
const TIM_CR1_ARPE: u32 = 1 << 7;
const TIM_DIER_UIE: u32 = 1 << 0;
const TIM_DIER_CC1IE: u32 = 1 << 1;
const TIM_SR_UIF: u32 = 1 << 0;
const TIM_EGR_UG: u32 = 1 << 0;
const TIM_PSC_MAX: i32 = u16::MAX as i32;
const TIM_PSC_CLKRATE: i32 = 10000;

#[repr(C)]
struct Stm32TimerPrivate {
    bits: i32,
}

unsafe fn stm32_timer_of_bits_set(to: *mut TimerOf, bits: i32) {
    let pd = (*to).private_data as *mut Stm32TimerPrivate;
    (*pd).bits = bits;
}

unsafe fn stm32_timer_of_bits_get(to: *mut TimerOf) -> i32 {
    let pd = (*to).private_data as *mut Stm32TimerPrivate;
    (*pd).bits
}

static mut STM32_TIMER_CNT: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn stm32_read_sched_clock() -> u64 {
    readl_relaxed(STM32_TIMER_CNT)
}

static mut STM32_TIMER_DELAY: DelayTimer = DelayTimer::zeroed();

unsafe fn stm32_read_delay() -> usize {
    readl_relaxed(STM32_TIMER_CNT) as usize
}

unsafe fn stm32_clock_event_disable(to: *mut TimerOf) {
    writel_relaxed(0, timer_of_base(to).add(TIM_DIER));
}

unsafe fn stm32_timer_start(to: *mut TimerOf) {
    writel_relaxed(TIM_CR1_UDIS | TIM_CR1_CEN, timer_of_base(to).add(TIM_CR1));
}

unsafe fn stm32_clock_event_shutdown(clkevt: *mut ClockEventDevice) -> i32 {
    let to = to_timer_of(clkevt);
    stm32_clock_event_disable(to);
    0
}

unsafe fn stm32_clock_event_set_next_event(evt: usize, clkevt: *mut ClockEventDevice) -> i32 {
    let to = to_timer_of(clkevt);
    let next = readl_relaxed(timer_of_base(to).add(TIM_CNT)).wrapping_add(evt as u32);
    writel_relaxed(next, timer_of_base(to).add(TIM_CCR1));
    let now = readl_relaxed(timer_of_base(to).add(TIM_CNT));
    if next.wrapping_sub(now) > evt as u32 { return -ETIME; }
    writel_relaxed(TIM_DIER_CC1IE, timer_of_base(to).add(TIM_DIER));
    0
}

unsafe fn stm32_clock_event_set_periodic(clkevt: *mut ClockEventDevice) -> i32 {
    let to = to_timer_of(clkevt);
    stm32_timer_start(to);
    stm32_clock_event_set_next_event(timer_of_period(to), clkevt)
}

unsafe fn stm32_clock_event_set_oneshot(clkevt: *mut ClockEventDevice) -> i32 {
    stm32_timer_start(to_timer_of(clkevt));
    0
}

unsafe fn stm32_clock_event_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let clkevt = dev_id as *mut ClockEventDevice;
    let to = to_timer_of(clkevt);
    writel_relaxed(0, timer_of_base(to).add(TIM_SR));
    if clockevent_state_periodic(clkevt) { stm32_clock_event_set_periodic(clkevt); }
    else { stm32_clock_event_shutdown(clkevt); }
    ((*clkevt).event_handler)(clkevt);
    IRQ_HANDLED
}

unsafe fn stm32_timer_set_width(to: *mut TimerOf) {
    writel_relaxed(u32::MAX, timer_of_base(to).add(TIM_ARR));
    let width = readl_relaxed(timer_of_base(to).add(TIM_ARR));
    stm32_timer_of_bits_set(to, if width == u32::MAX { 32 } else { 16 });
}

unsafe fn stm32_timer_set_prescaler(to: *mut TimerOf) {
    let mut prescaler: i32 = 1;
    if stm32_timer_of_bits_get(to) != 32 {
        prescaler = div_round_closest(timer_of_rate(to), TIM_PSC_CLKRATE);
        prescaler = if prescaler < TIM_PSC_MAX { prescaler } else { TIM_PSC_MAX };
    }
    writel_relaxed((prescaler - 1) as u32, timer_of_base(to).add(TIM_PSC));
    writel_relaxed(TIM_EGR_UG, timer_of_base(to).add(TIM_EGR));
    writel_relaxed(0, timer_of_base(to).add(TIM_SR));
    (*to).of_clk.rate = div_round_closest((*to).of_clk.rate, prescaler);
    (*to).of_clk.period = div_round_up((*to).of_clk.rate, HZ);
}

unsafe fn stm32_clocksource_init(to: *mut TimerOf) -> i32 {
    let bits = stm32_timer_of_bits_get(to) as u32;
    let name = (*(*to).np).full_name;
    if bits == 32 && STM32_TIMER_CNT.is_null() {
        stm32_timer_start(to);
        STM32_TIMER_CNT = timer_of_base(to).add(TIM_CNT);
        sched_clock_register(stm32_read_sched_clock, bits, timer_of_rate(to));
        pr_info("%s: STM32 sched_clock registered\n", name);
        STM32_TIMER_DELAY.read_current_timer = Some(stm32_read_delay);
        STM32_TIMER_DELAY.freq = timer_of_rate(to);
        register_current_timer_delay(&mut STM32_TIMER_DELAY);
        pr_info("%s: STM32 delay timer registered\n", name);
    }
    clocksource_mmio_init(timer_of_base(to).add(TIM_CNT), name, timer_of_rate(to),
        if bits == 32 { 250 } else { 100 }, bits, clocksource_mmio_readl_up)
}

unsafe fn stm32_clockevent_init(to: *mut TimerOf) {
    let bits = stm32_timer_of_bits_get(to) as u32;
    (*to).clkevt.name = (*(*to).np).full_name;
    (*to).clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    (*to).clkevt.set_state_shutdown = Some(stm32_clock_event_shutdown);
    (*to).clkevt.set_state_periodic = Some(stm32_clock_event_set_periodic);
    (*to).clkevt.set_state_oneshot = Some(stm32_clock_event_set_oneshot);
    (*to).clkevt.tick_resume = Some(stm32_clock_event_shutdown);
    (*to).clkevt.set_next_event = Some(stm32_clock_event_set_next_event);
    (*to).clkevt.rating = if bits == 32 { 250 } else { 100 };
    clockevents_config_and_register(&mut (*to).clkevt, timer_of_rate(to), 0x1, (1u32 << bits) - 1);
    pr_info("%pOF: STM32 clockevent driver initialized (%d bits)\n", (*to).np, bits);
}

unsafe fn stm32_timer_init(node: *mut DeviceNode) -> i32 {
    let mut rstc: *mut ResetControl;
    let to = kzalloc_timer_of();
    if to.is_null() { return -ENOMEM; }
    (*to).flags = TIMER_OF_IRQ | TIMER_OF_CLOCK | TIMER_OF_BASE;
    (*to).of_irq.handler = Some(stm32_clock_event_handler);
    let mut ret = timer_of_init(node, to);
    if ret != 0 { kfree(to); return ret; }
    (*to).private_data = kzalloc_private();
    if (*to).private_data.is_null() { ret = -ENOMEM; timer_of_cleanup(to); kfree(to); return ret; }
    rstc = of_reset_control_get(node, core::ptr::null());
    if !is_err(rstc) { reset_control_assert(rstc); reset_control_deassert(rstc); }
    stm32_timer_set_width(to);
    stm32_timer_set_prescaler(to);
    ret = stm32_clocksource_init(to);
    if ret != 0 { timer_of_cleanup(to); kfree(to); return ret; }
    stm32_clockevent_init(to);
    0
}

timer_of_declare!(stm32, "st,stm32-timer", stm32_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
