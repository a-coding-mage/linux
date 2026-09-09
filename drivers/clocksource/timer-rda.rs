// SPDX-License-Identifier: GPL-2.0+
/*
 * RDA8810PL SoC timer driver
 *
 * Copyright RDA Microelectronics Company Limited
 * Copyright (c) 2017 Andreas Färber
 * Copyright (c) 2018 Manivannan Sadhasivam
 *
 * RDA8810PL has two independent timers: OSTIMER (56 bit) and HWTIMER (64 bit).
 * Each timer provides optional interrupt support. In this driver, OSTIMER is
 * used for clockevents and HWTIMER is used for clocksource.
 */

// C dependencies: linux/init.h, linux/interrupt.h, linux/sched_clock.h,
// and timer-of.h are supplied by the surrounding kernel Rust bindings.

const RDA_OSTIMER_LOADVAL_L: usize = 0x000;
const RDA_OSTIMER_CTRL: usize = 0x004;
const RDA_HWTIMER_LOCKVAL_L: usize = 0x024;
const RDA_HWTIMER_LOCKVAL_H: usize = 0x028;
const RDA_TIMER_IRQ_MASK_SET: usize = 0x02c;
const RDA_TIMER_IRQ_MASK_CLR: usize = 0x030;
const RDA_TIMER_IRQ_CLR: usize = 0x034;

const RDA_OSTIMER_CTRL_ENABLE: u32 = 1 << 24;
const RDA_OSTIMER_CTRL_REPEAT: u32 = 1 << 28;
const RDA_OSTIMER_CTRL_LOAD: u32 = 1 << 30;

const RDA_TIMER_IRQ_MASK_OSTIMER: u32 = 1 << 0;
const RDA_TIMER_IRQ_CLR_OSTIMER: u32 = 1 << 0;

unsafe fn rda_ostimer_start(base: *mut core::ffi::c_void, periodic: bool, cycles: u64) -> i32 {
    let load_l: u32 = cycles as u32;
    let mut ctrl: u32 = ((cycles >> 32) & 0xffffff) as u32;
    ctrl |= RDA_OSTIMER_CTRL_LOAD | RDA_OSTIMER_CTRL_ENABLE;
    if periodic {
        ctrl |= RDA_OSTIMER_CTRL_REPEAT;
    }

    // Enable ostimer interrupt first
    writel_relaxed(RDA_TIMER_IRQ_MASK_OSTIMER, base.add(RDA_TIMER_IRQ_MASK_SET));

    // Write low 32 bits first, high 24 bits are with ctrl
    writel_relaxed(load_l, base.add(RDA_OSTIMER_LOADVAL_L));
    writel_relaxed(ctrl, base.add(RDA_OSTIMER_CTRL));

    0
}

unsafe fn rda_ostimer_stop(base: *mut core::ffi::c_void) -> i32 {
    // Disable ostimer interrupt first
    writel_relaxed(RDA_TIMER_IRQ_MASK_OSTIMER, base.add(RDA_TIMER_IRQ_MASK_CLR));
    writel_relaxed(0, base.add(RDA_OSTIMER_CTRL));
    0
}

unsafe fn rda_ostimer_set_state_shutdown(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt);
    rda_ostimer_stop(timer_of_base(to));
    0
}

unsafe fn rda_ostimer_set_state_oneshot(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt);
    rda_ostimer_stop(timer_of_base(to));
    0
}

unsafe fn rda_ostimer_set_state_periodic(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt);
    rda_ostimer_stop(timer_of_base(to));

    let cycles_per_jiffy: usize = (((NSEC_PER_SEC as u64 / HZ as u64)
        * (*evt).mult as u64) >> (*evt).shift) as usize;
    rda_ostimer_start(timer_of_base(to), true, cycles_per_jiffy as u64);
    0
}

unsafe fn rda_ostimer_tick_resume(_evt: *mut clock_event_device) -> i32 {
    0
}

unsafe fn rda_ostimer_set_next_event(evt: usize, ev: *mut clock_event_device) -> i32 {
    let to = to_timer_of(ev);
    rda_ostimer_start(timer_of_base(to), false, evt as u64);
    0
}

unsafe fn rda_ostimer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    let to = to_timer_of(evt);

    // clear timer int
    writel_relaxed(RDA_TIMER_IRQ_CLR_OSTIMER, timer_of_base(to).add(RDA_TIMER_IRQ_CLR));

    if let Some(handler) = (*evt).event_handler {
        handler(evt);
    }

    IRQ_HANDLED
}

static mut rda_ostimer_of: timer_of = timer_of {
    flags: TIMER_OF_IRQ | TIMER_OF_BASE,
    clkevt: clock_event_device {
        name: "rda-ostimer",
        rating: 250,
        features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ,
        set_state_shutdown: Some(rda_ostimer_set_state_shutdown),
        set_state_oneshot: Some(rda_ostimer_set_state_oneshot),
        set_state_periodic: Some(rda_ostimer_set_state_periodic),
        tick_resume: Some(rda_ostimer_tick_resume),
        set_next_event: Some(rda_ostimer_set_next_event),
        ..clock_event_device::ZERO
    },
    of_base: timer_of_resource { name: "rda-timer", index: 0 },
    of_irq: timer_of_irq { name: "ostimer", handler: Some(rda_ostimer_interrupt), flags: IRQF_TIMER },
};

unsafe fn rda_hwtimer_clocksource_read() -> u64 {
    let base = timer_of_base(&mut rda_ostimer_of);
    let (mut lo, mut hi): (u32, u32);
    loop {
        // Always read low 32 bits first
        lo = readl_relaxed(base.add(RDA_HWTIMER_LOCKVAL_L));
        hi = readl_relaxed(base.add(RDA_HWTIMER_LOCKVAL_H));
        if hi == readl_relaxed(base.add(RDA_HWTIMER_LOCKVAL_H)) {
            break;
        }
    }
    ((hi as u64) << 32) | lo as u64
}

unsafe fn rda_hwtimer_read(_cs: *mut clocksource) -> u64 {
    rda_hwtimer_clocksource_read()
}

static mut rda_hwtimer_clocksource: clocksource = clocksource {
    name: "rda-timer",
    rating: 400,
    read: Some(rda_hwtimer_read),
    mask: u64::MAX,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    ..clocksource::ZERO
};

unsafe fn rda_timer_init(np: *mut device_node) -> i32 {
    let rate: usize = 2000000;
    let ret = timer_of_init(np, &mut rda_ostimer_of);
    if ret != 0 {
        return ret;
    }

    clocksource_register_hz(&mut rda_hwtimer_clocksource, rate);
    sched_clock_register(Some(rda_hwtimer_clocksource_read), 64, rate);
    clockevents_config_and_register(&mut rda_ostimer_of.clkevt, rate, 0x2, u32::MAX);
    0
}

// TIMER_OF_DECLARE(rda8810pl, "rda,8810pl-timer", rda_timer_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
