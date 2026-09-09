// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  GT641xx clockevent routines.
 *
 *  Copyright (C) 2007 Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::linux::clockchips::*;
use crate::linux::init::*;
use crate::linux::interrupt::*;
use crate::linux::spinlock::*;
use crate::linux::irq::*;
use crate::asm::gt64120::*;
use crate::asm::time::*;

static mut GT641XX_TIMER_LOCK: RawSpinLock = DEFINE_RAW_SPINLOCK!();
static mut gt641xx_base_clock: u32 = 0;

pub unsafe fn gt641xx_set_base_clock(clock: u32) {
    gt641xx_base_clock = clock;
}

pub unsafe fn gt641xx_timer0_state() -> i32 {
    if GT_READ(GT_TC0_OFS) != 0 {
        return 0;
    }

    GT_WRITE(GT_TC0_OFS, gt641xx_base_clock / HZ);
    GT_WRITE(GT_TC_CONTROL_OFS, GT_TC_CONTROL_ENTC0_MSK);

    1
}

unsafe fn gt641xx_timer0_set_next_event(
    delta: c_ulong,
    _evt: *mut clock_event_device,
) -> i32 {
    let mut ctrl: u32;

    raw_spin_lock(&raw mut GT641XX_TIMER_LOCK);

    ctrl = GT_READ(GT_TC_CONTROL_OFS);
    ctrl &= !(GT_TC_CONTROL_ENTC0_MSK | GT_TC_CONTROL_SELTC0_MSK);
    ctrl |= GT_TC_CONTROL_ENTC0_MSK;

    GT_WRITE(GT_TC0_OFS, delta);
    GT_WRITE(GT_TC_CONTROL_OFS, ctrl);

    raw_spin_unlock(&raw mut GT641XX_TIMER_LOCK);

    0
}

unsafe fn gt641xx_timer0_shutdown(_evt: *mut clock_event_device) -> i32 {
    let mut ctrl: u32;

    raw_spin_lock(&raw mut GT641XX_TIMER_LOCK);

    ctrl = GT_READ(GT_TC_CONTROL_OFS);
    ctrl &= !(GT_TC_CONTROL_ENTC0_MSK | GT_TC_CONTROL_SELTC0_MSK);
    GT_WRITE(GT_TC_CONTROL_OFS, ctrl);

    raw_spin_unlock(&raw mut GT641XX_TIMER_LOCK);
    0
}

unsafe fn gt641xx_timer0_set_oneshot(_evt: *mut clock_event_device) -> i32 {
    let mut ctrl: u32;

    raw_spin_lock(&raw mut GT641XX_TIMER_LOCK);

    ctrl = GT_READ(GT_TC_CONTROL_OFS);
    ctrl &= !GT_TC_CONTROL_SELTC0_MSK;
    ctrl |= GT_TC_CONTROL_ENTC0_MSK;
    GT_WRITE(GT_TC_CONTROL_OFS, ctrl);

    raw_spin_unlock(&raw mut GT641XX_TIMER_LOCK);
    0
}

unsafe fn gt641xx_timer0_set_periodic(_evt: *mut clock_event_device) -> i32 {
    let mut ctrl: u32;

    raw_spin_lock(&raw mut GT641XX_TIMER_LOCK);

    ctrl = GT_READ(GT_TC_CONTROL_OFS);
    ctrl |= GT_TC_CONTROL_ENTC0_MSK | GT_TC_CONTROL_SELTC0_MSK;
    GT_WRITE(GT_TC_CONTROL_OFS, ctrl);

    raw_spin_unlock(&raw mut GT641XX_TIMER_LOCK);
    0
}

unsafe extern "C" fn gt641xx_timer0_event_handler(_dev: *mut clock_event_device) {}

static mut gt641xx_timer0_clockevent: clock_event_device = clock_event_device {
    name: b"gt641xx-timer0\0".as_ptr() as *const c_char,
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    irq: GT641XX_TIMER0_IRQ,
    set_next_event: Some(gt641xx_timer0_set_next_event),
    set_state_shutdown: Some(gt641xx_timer0_shutdown),
    set_state_periodic: Some(gt641xx_timer0_set_periodic),
    set_state_oneshot: Some(gt641xx_timer0_set_oneshot),
    tick_resume: Some(gt641xx_timer0_shutdown),
    event_handler: Some(gt641xx_timer0_event_handler),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn gt641xx_timer0_interrupt(_irq: i32, _dev_id: *mut c_void) -> irqreturn_t {
    let cd: *mut clock_event_device = &raw mut gt641xx_timer0_clockevent;

    ((*cd).event_handler.unwrap())(cd);

    IRQ_HANDLED
}

unsafe fn gt641xx_timer0_clockevent_init() -> i32 {
    let cd: *mut clock_event_device;

    if gt641xx_base_clock == 0 {
        return 0;
    }

    GT_WRITE(GT_TC0_OFS, gt641xx_base_clock / HZ);

    cd = &raw mut gt641xx_timer0_clockevent;
    (*cd).rating = 200 + gt641xx_base_clock / 10000000;
    clockevent_set_clock(cd, gt641xx_base_clock);
    (*cd).max_delta_ns = clockevent_delta2ns(0x7fffffff, cd);
    (*cd).max_delta_ticks = 0x7fffffff;
    (*cd).min_delta_ns = clockevent_delta2ns(0x300, cd);
    (*cd).min_delta_ticks = 0x300;
    (*cd).cpumask = cpumask_of(0);

    clockevents_register_device(&raw mut gt641xx_timer0_clockevent);

    request_irq(
        GT641XX_TIMER0_IRQ,
        Some(gt641xx_timer0_interrupt),
        IRQF_PERCPU | IRQF_TIMER,
        b"gt641xx_timer0\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    )
}

arch_initcall!(gt641xx_timer0_clockevent_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
