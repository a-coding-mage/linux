// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Clocksource driver for Loongson-1 SoC
 *
 * Copyright (c) 2023 Keguang Zhang <keguang.zhang@gmail.com>
 */

// External Linux/kernel declarations and macros are supplied by the surrounding tree.

const PWM_CNTR: usize = 0x0;
const PWM_HRC: usize = 0x4;
const PWM_LRC: usize = 0x8;
const PWM_CTRL: usize = 0xc;

const INT_LRC_EN: u32 = 1 << 11;
const INT_HRC_EN: u32 = 1 << 10;
const CNTR_RST: u32 = 1 << 7;
const INT_SR: u32 = 1 << 6;
const INT_EN: u32 = 1 << 5;
const PWM_SINGLE: u32 = 1 << 4;
const PWM_OE: u32 = 1 << 3;
const CNT_EN: u32 = 1 << 0;

const CNTR_WIDTH: u32 = 24;

static mut ls1x_timer_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK!();

#[repr(C)]
struct ls1x_clocksource {
    reg_base: *mut core::ffi::c_void,
    ticks_per_jiffy: usize,
    clksrc: clocksource,
}

#[inline]
unsafe fn to_ls1x_clksrc(c: *mut clocksource) -> *mut ls1x_clocksource {
    container_of!(c, ls1x_clocksource, clksrc)
}

#[inline]
unsafe fn ls1x_pwmtimer_set_period(period: u32, to: *mut timer_of) {
    writel(period, timer_of_base(to).add(PWM_LRC));
    writel(period, timer_of_base(to).add(PWM_HRC));
}

#[inline]
unsafe fn ls1x_pwmtimer_clear(to: *mut timer_of) {
    writel(0, timer_of_base(to).add(PWM_CNTR));
}

#[inline]
unsafe fn ls1x_pwmtimer_start(to: *mut timer_of) {
    writel(INT_EN | PWM_OE | CNT_EN, timer_of_base(to).add(PWM_CTRL));
}

#[inline]
unsafe fn ls1x_pwmtimer_stop(to: *mut timer_of) {
    writel(0, timer_of_base(to).add(PWM_CTRL));
}

#[inline]
unsafe fn ls1x_pwmtimer_irq_ack(to: *mut timer_of) {
    let mut val: u32 = readl(timer_of_base(to).add(PWM_CTRL));
    val |= INT_SR;
    writel(val, timer_of_base(to).add(PWM_CTRL));
}

unsafe extern "C" fn ls1x_clockevent_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let clkevt = dev_id as *mut clock_event_device;
    let to = to_timer_of(clkevt);

    ls1x_pwmtimer_irq_ack(to);
    ls1x_pwmtimer_clear(to);
    ls1x_pwmtimer_start(to);
    ((*clkevt).event_handler.unwrap())(clkevt);
    IRQ_HANDLED
}

unsafe extern "C" fn ls1x_clockevent_set_state_periodic(clkevt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clkevt);
    raw_spin_lock(&mut ls1x_timer_lock);
    ls1x_pwmtimer_set_period(timer_of_period(to), to);
    ls1x_pwmtimer_clear(to);
    ls1x_pwmtimer_start(to);
    raw_spin_unlock(&mut ls1x_timer_lock);
    0
}

unsafe extern "C" fn ls1x_clockevent_tick_resume(clkevt: *mut clock_event_device) -> i32 {
    raw_spin_lock(&mut ls1x_timer_lock);
    ls1x_pwmtimer_start(to_timer_of(clkevt));
    raw_spin_unlock(&mut ls1x_timer_lock);
    0
}

unsafe extern "C" fn ls1x_clockevent_set_state_shutdown(clkevt: *mut clock_event_device) -> i32 {
    raw_spin_lock(&mut ls1x_timer_lock);
    ls1x_pwmtimer_stop(to_timer_of(clkevt));
    raw_spin_unlock(&mut ls1x_timer_lock);
    0
}

unsafe extern "C" fn ls1x_clockevent_set_next(evt: usize, clkevt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clkevt);
    raw_spin_lock(&mut ls1x_timer_lock);
    ls1x_pwmtimer_set_period(evt as u32, to);
    ls1x_pwmtimer_clear(to);
    ls1x_pwmtimer_start(to);
    raw_spin_unlock(&mut ls1x_timer_lock);
    0
}

static mut ls1x_to: timer_of = timer_of {
    flags: TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    clkevt: clock_event_device {
        name: b"ls1x-pwmtimer\0".as_ptr() as *const i8,
        features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
        rating: 300,
        set_next_event: Some(ls1x_clockevent_set_next),
        set_state_periodic: Some(ls1x_clockevent_set_state_periodic),
        set_state_oneshot: Some(ls1x_clockevent_set_state_shutdown),
        set_state_shutdown: Some(ls1x_clockevent_set_state_shutdown),
        tick_resume: Some(ls1x_clockevent_tick_resume),
        ..unsafe { core::mem::zeroed() }
    },
    of_irq: timer_of_irq {
        handler: Some(ls1x_clockevent_isr),
        flags: IRQF_TIMER,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

/*
 * Since the PWM timer overflows every two ticks, its not very useful
 * to just read by itself. So use jiffies to emulate a free
 * running counter:
 */
unsafe extern "C" fn ls1x_clocksource_read(cs: *mut clocksource) -> u64 {
    let ls1x_cs = to_ls1x_clksrc(cs);
    let mut flags: usize = 0;
    let mut count: i32;
    let mut jifs: u32;
    static mut old_count: i32 = 0;
    static mut old_jifs: u32 = 0;

    raw_spin_lock_irqsave(&mut ls1x_timer_lock, &mut flags);
    /* See the C implementation: jiffies is read before latching the timer count. */
    jifs = jiffies;
    count = readl((*ls1x_cs).reg_base.add(PWM_CNTR)) as i32;

    if count < old_count && jifs == old_jifs {
        count = old_count;
    }
    old_count = count;
    old_jifs = jifs;
    raw_spin_unlock_irqrestore(&mut ls1x_timer_lock, flags);
    (jifs as u64).wrapping_mul((*ls1x_cs).ticks_per_jiffy as u64).wrapping_add(count as u64)
}

static mut ls1x_clocksource: ls1x_clocksource = ls1x_clocksource {
    clksrc: clocksource {
        name: b"ls1x-pwmtimer\0".as_ptr() as *const i8,
        rating: 300,
        read: Some(ls1x_clocksource_read),
        mask: CLOCKSOURCE_MASK(CNTR_WIDTH),
        flags: CLOCK_SOURCE_IS_CONTINUOUS,
        ..unsafe { core::mem::zeroed() }
    },
    reg_base: core::ptr::null_mut(),
    ticks_per_jiffy: 0,
};

unsafe extern "C" fn ls1x_pwm_clocksource_init(np: *mut device_node) -> i32 {
    let to = &mut ls1x_to;
    let ret = timer_of_init(np, to);
    if ret != 0 { return ret; }
    clockevents_config_and_register(&mut to.clkevt, timer_of_rate(to), 0x1, GENMASK(CNTR_WIDTH - 1, 0));
    ls1x_clocksource.reg_base = timer_of_base(to);
    ls1x_clocksource.ticks_per_jiffy = timer_of_period(to) as usize;
    clocksource_register_hz(&mut ls1x_clocksource.clksrc, timer_of_rate(to))
}

TIMER_OF_DECLARE!(ls1x_pwm_clocksource, "loongson,ls1b-pwmtimer", ls1x_pwm_clocksource_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
