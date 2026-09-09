// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) Copyright 2009 Intel Corporation
 * Author: Jacob Pan (jacob.jun.pan@intel.com)
 *
 * Shared with ARM platforms, Jamie Iles, Picochip 2011
 *
 * Support for the Synopsys DesignWare APB Timers.
 */

const APBT_MIN_PERIOD: u32 = 4;
const APBT_MIN_DELTA_USEC: u32 = 200;

const APBTMR_N_LOAD_COUNT: usize = 0x00;
const APBTMR_N_CURRENT_VALUE: usize = 0x04;
const APBTMR_N_CONTROL: usize = 0x08;
const APBTMR_N_EOI: usize = 0x0c;
const APBTMR_N_INT_STATUS: usize = 0x10;

const APBTMRS_INT_STATUS: usize = 0xa0;
const APBTMRS_EOI: usize = 0xa4;
const APBTMRS_RAW_INT_STATUS: usize = 0xa8;
const APBTMRS_COMP_VERSION: usize = 0xac;

const APBTMR_CONTROL_ENABLE: u32 = 1 << 0;
/* 1: periodic, 0:free running. */
const APBTMR_CONTROL_MODE_PERIODIC: u32 = 1 << 1;
const APBTMR_CONTROL_INT: u32 = 1 << 2;

#[inline]
unsafe fn ced_to_dw_apb_ced(evt: *mut clock_event_device) -> *mut dw_apb_clock_event_device {
    container_of!(evt, dw_apb_clock_event_device, ced)
}

#[inline]
unsafe fn clocksource_to_dw_apb_clocksource(cs: *mut clocksource) -> *mut dw_apb_clocksource {
    container_of!(cs, dw_apb_clocksource, cs)
}

#[inline]
unsafe fn apbt_readl(timer: *mut dw_apb_timer, offs: usize) -> u32 {
    readl((*timer).base.add(offs) as *const core::ffi::c_void)
}

#[inline]
unsafe fn apbt_writel(timer: *mut dw_apb_timer, val: u32, offs: usize) {
    writel(val, (*timer).base.add(offs));
}

#[inline]
unsafe fn apbt_readl_relaxed(timer: *mut dw_apb_timer, offs: usize) -> u32 {
    readl_relaxed((*timer).base.add(offs) as *const core::ffi::c_void)
}

#[inline]
unsafe fn apbt_writel_relaxed(timer: *mut dw_apb_timer, val: u32, offs: usize) {
    writel_relaxed(val, (*timer).base.add(offs));
}

unsafe fn apbt_eoi(timer: *mut dw_apb_timer) {
    apbt_readl_relaxed(timer, APBTMR_N_EOI);
}

unsafe extern "C" fn dw_apb_clockevent_irq(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = data as *mut clock_event_device;
    let dw_ced = ced_to_dw_apb_ced(evt);

    if (*evt).event_handler.is_none() {
        pr_info!("Spurious APBT timer interrupt {}\n", irq);
        return IRQ_NONE;
    }

    if let Some(eoi) = (*dw_ced).eoi {
        eoi(&mut (*dw_ced).timer);
    }

    ((*evt).event_handler.unwrap())(evt);
    IRQ_HANDLED
}

unsafe fn apbt_enable_int(timer: *mut dw_apb_timer) {
    let mut ctrl = apbt_readl(timer, APBTMR_N_CONTROL);
    /* clear pending intr */
    apbt_readl(timer, APBTMR_N_EOI);
    ctrl &= !APBTMR_CONTROL_INT;
    apbt_writel(timer, ctrl, APBTMR_N_CONTROL);
}

unsafe extern "C" fn apbt_shutdown(evt: *mut clock_event_device) -> i32 {
    let dw_ced = ced_to_dw_apb_ced(evt);
    pr_debug!("{} CPU {} state=shutdown\n", __func__, cpumask_first((*evt).cpumask));
    let mut ctrl = apbt_readl(&mut (*dw_ced).timer, APBTMR_N_CONTROL);
    ctrl &= !APBTMR_CONTROL_ENABLE;
    apbt_writel(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    0
}

unsafe extern "C" fn apbt_set_oneshot(evt: *mut clock_event_device) -> i32 {
    let dw_ced = ced_to_dw_apb_ced(evt);
    pr_debug!("{} CPU {} state=oneshot\n", __func__, cpumask_first((*evt).cpumask));
    let mut ctrl = apbt_readl(&mut (*dw_ced).timer, APBTMR_N_CONTROL);
    /* set free running mode, this mode will let timer reload max
     * timeout which will give time (3min on 25MHz clock) to rearm
     * the next event, therefore emulate the one-shot mode. */
    ctrl &= !APBTMR_CONTROL_ENABLE;
    ctrl &= !APBTMR_CONTROL_MODE_PERIODIC;
    apbt_writel(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    /* write again to set free running mode */
    apbt_writel(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    /* DW APB p. 46, load counter with all 1s before starting free
     * running mode. */
    apbt_writel(&mut (*dw_ced).timer, !0, APBTMR_N_LOAD_COUNT);
    ctrl &= !APBTMR_CONTROL_INT;
    ctrl |= APBTMR_CONTROL_ENABLE;
    apbt_writel(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    0
}

unsafe extern "C" fn apbt_set_periodic(evt: *mut clock_event_device) -> i32 {
    let dw_ced = ced_to_dw_apb_ced(evt);
    let period = div_round_up((*dw_ced).timer.freq, HZ);
    pr_debug!("{} CPU {} state=periodic\n", __func__, cpumask_first((*evt).cpumask));
    let mut ctrl = apbt_readl(&mut (*dw_ced).timer, APBTMR_N_CONTROL);
    ctrl |= APBTMR_CONTROL_MODE_PERIODIC;
    apbt_writel(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    /* DW APB p. 46, have to disable timer before load counter,
     * may cause sync problem. */
    ctrl &= !APBTMR_CONTROL_ENABLE;
    apbt_writel(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    udelay(1);
    pr_debug!("Setting clock period {} for HZ {}\n", period, HZ);
    apbt_writel(&mut (*dw_ced).timer, period as u32, APBTMR_N_LOAD_COUNT);
    ctrl |= APBTMR_CONTROL_ENABLE;
    apbt_writel(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    0
}

unsafe extern "C" fn apbt_resume(evt: *mut clock_event_device) -> i32 {
    let dw_ced = ced_to_dw_apb_ced(evt);
    pr_debug!("{} CPU {} state=resume\n", __func__, cpumask_first((*evt).cpumask));
    apbt_enable_int(&mut (*dw_ced).timer);
    0
}

unsafe extern "C" fn apbt_next_event(delta: usize, evt: *mut clock_event_device) -> i32 {
    let dw_ced = ced_to_dw_apb_ced(evt);
    let mut ctrl = apbt_readl_relaxed(&mut (*dw_ced).timer, APBTMR_N_CONTROL);
    ctrl &= !APBTMR_CONTROL_ENABLE;
    apbt_writel_relaxed(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    apbt_writel_relaxed(&mut (*dw_ced).timer, delta as u32, APBTMR_N_LOAD_COUNT);
    ctrl |= APBTMR_CONTROL_ENABLE;
    apbt_writel_relaxed(&mut (*dw_ced).timer, ctrl, APBTMR_N_CONTROL);
    0
}

pub unsafe fn dw_apb_clockevent_init(cpu: i32, name: *const i8, rating: u32,
    base: *mut core::ffi::c_void, irq: i32, freq: usize) -> *mut dw_apb_clock_event_device {
    let dw_ced = kzalloc_obj::<dw_apb_clock_event_device>();
    if dw_ced.is_null() { return core::ptr::null_mut(); }
    (*dw_ced).timer.base = base as *mut u8;
    (*dw_ced).timer.irq = irq;
    (*dw_ced).timer.freq = freq;
    clockevents_calc_mult_shift(&mut (*dw_ced).ced, freq, APBT_MIN_PERIOD);
    (*dw_ced).ced.max_delta_ns = clockevent_delta2ns(0x7fffffff, &(*dw_ced).ced);
    (*dw_ced).ced.max_delta_ticks = 0x7fffffff;
    (*dw_ced).ced.min_delta_ns = clockevent_delta2ns(5000, &(*dw_ced).ced);
    (*dw_ced).ced.min_delta_ticks = 5000;
    (*dw_ced).ced.cpumask = if cpu < 0 { cpu_possible_mask } else { cpumask_of(cpu) };
    (*dw_ced).ced.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ;
    (*dw_ced).ced.set_state_shutdown = Some(apbt_shutdown);
    (*dw_ced).ced.set_state_periodic = Some(apbt_set_periodic);
    (*dw_ced).ced.set_state_oneshot = Some(apbt_set_oneshot);
    (*dw_ced).ced.set_state_oneshot_stopped = Some(apbt_shutdown);
    (*dw_ced).ced.tick_resume = Some(apbt_resume);
    (*dw_ced).ced.set_next_event = Some(apbt_next_event);
    (*dw_ced).ced.irq = irq;
    (*dw_ced).ced.rating = rating;
    (*dw_ced).ced.name = name;
    (*dw_ced).eoi = Some(apbt_eoi);
    let err = request_irq(irq, Some(dw_apb_clockevent_irq), IRQF_TIMER | IRQF_IRQPOLL | IRQF_NOBALANCING,
        name, &mut (*dw_ced).ced as *mut _ as *mut core::ffi::c_void);
    if err != 0 {
        pr_err!("failed to request timer irq\n");
        kfree(dw_ced as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    dw_ced
}

pub unsafe fn dw_apb_clockevent_register(dw_ced: *mut dw_apb_clock_event_device) {
    apbt_writel(&mut (*dw_ced).timer, 0, APBTMR_N_CONTROL);
    clockevents_register_device(&mut (*dw_ced).ced);
    apbt_enable_int(&mut (*dw_ced).timer);
}

pub unsafe fn dw_apb_clocksource_start(dw_cs: *mut dw_apb_clocksource) {
    let mut ctrl = apbt_readl(&mut (*dw_cs).timer, APBTMR_N_CONTROL);
    ctrl &= !APBTMR_CONTROL_ENABLE;
    apbt_writel(&mut (*dw_cs).timer, ctrl, APBTMR_N_CONTROL);
    apbt_writel(&mut (*dw_cs).timer, !0, APBTMR_N_LOAD_COUNT);
    ctrl &= !APBTMR_CONTROL_MODE_PERIODIC;
    ctrl |= APBTMR_CONTROL_ENABLE | APBTMR_CONTROL_INT;
    apbt_writel(&mut (*dw_cs).timer, ctrl, APBTMR_N_CONTROL);
    dw_apb_clocksource_read(dw_cs);
}

unsafe fn __apbt_read_clocksource(cs: *mut clocksource) -> u64 {
    let dw_cs = clocksource_to_dw_apb_clocksource(cs);
    (!apbt_readl_relaxed(&mut (*dw_cs).timer, APBTMR_N_CURRENT_VALUE)) as u64
}

unsafe fn apbt_restart_clocksource(cs: *mut clocksource) {
    dw_apb_clocksource_start(clocksource_to_dw_apb_clocksource(cs));
}

pub unsafe fn dw_apb_clocksource_init(rating: u32, name: *const i8, base: *mut core::ffi::c_void,
    freq: usize) -> *mut dw_apb_clocksource {
    let dw_cs = kzalloc_obj::<dw_apb_clocksource>();
    if dw_cs.is_null() { return core::ptr::null_mut(); }
    (*dw_cs).timer.base = base as *mut u8;
    (*dw_cs).timer.freq = freq;
    (*dw_cs).cs.name = name;
    (*dw_cs).cs.rating = rating;
    (*dw_cs).cs.read = Some(__apbt_read_clocksource);
    (*dw_cs).cs.mask = CLOCKSOURCE_MASK(32);
    (*dw_cs).cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    (*dw_cs).cs.resume = Some(apbt_restart_clocksource);
    dw_cs
}

pub unsafe fn dw_apb_clocksource_register(dw_cs: *mut dw_apb_clocksource) {
    clocksource_register_hz(&mut (*dw_cs).cs, (*dw_cs).timer.freq);
}

pub unsafe fn dw_apb_clocksource_read(dw_cs: *mut dw_apb_clocksource) -> u64 {
    (!apbt_readl(&mut (*dw_cs).timer, APBTMR_N_CURRENT_VALUE)) as u64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
