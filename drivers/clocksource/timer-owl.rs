// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Actions Semi Owl timer
 *
 * Copyright 2012 Actions Semi Inc.
 * Author: Actions Semi, Inc.
 *
 * Copyright (c) 2017 SUSE Linux GmbH
 * Author: Andreas Färber
 */

const OWL_Tx_CTL: usize = 0x0;
const OWL_Tx_CMP: usize = 0x4;
const OWL_Tx_VAL: usize = 0x8;

const OWL_Tx_CTL_PD: u32 = 1 << 0;
const OWL_Tx_CTL_INTEN: u32 = 1 << 1;
const OWL_Tx_CTL_EN: u32 = 1 << 2;

static mut owl_timer_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut owl_clksrc_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut owl_clkevt_base: *mut core::ffi::c_void = core::ptr::null_mut();

#[inline]
unsafe fn owl_timer_reset(base: *mut core::ffi::c_void) {
    writel(0, base.add(OWL_Tx_CTL));
    writel(0, base.add(OWL_Tx_VAL));
    writel(0, base.add(OWL_Tx_CMP));
}

#[inline]
unsafe fn owl_timer_set_enabled(base: *mut core::ffi::c_void, enabled: bool) {
    let mut ctl: u32 = readl(base.add(OWL_Tx_CTL));

    /* PD bit is cleared when set */
    ctl &= !OWL_Tx_CTL_PD;

    if enabled {
        ctl |= OWL_Tx_CTL_EN;
    } else {
        ctl &= !OWL_Tx_CTL_EN;
    }

    writel(ctl, base.add(OWL_Tx_CTL));
}

unsafe fn owl_timer_sched_read() -> u64 {
    readl(owl_clksrc_base.add(OWL_Tx_VAL)) as u64
}

unsafe fn owl_timer_set_state_shutdown(_evt: *mut clock_event_device) -> i32 {
    owl_timer_set_enabled(owl_clkevt_base, false);
    0
}

unsafe fn owl_timer_set_state_oneshot(_evt: *mut clock_event_device) -> i32 {
    owl_timer_reset(owl_clkevt_base);
    0
}

unsafe fn owl_timer_tick_resume(_evt: *mut clock_event_device) -> i32 {
    0
}

unsafe fn owl_timer_set_next_event(evt: usize, _ev: *mut clock_event_device) -> i32 {
    let base = owl_clkevt_base;

    owl_timer_set_enabled(base, false);
    writel(OWL_Tx_CTL_INTEN, base.add(OWL_Tx_CTL));
    writel(0, base.add(OWL_Tx_VAL));
    writel(evt as u32, base.add(OWL_Tx_CMP));
    owl_timer_set_enabled(base, true);

    0
}

static mut owl_clockevent: clock_event_device = clock_event_device {
    name: b"owl_tick\0".as_ptr() as *const i8,
    rating: 200,
    features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ,
    set_state_shutdown: Some(owl_timer_set_state_shutdown),
    set_state_oneshot: Some(owl_timer_set_state_oneshot),
    tick_resume: Some(owl_timer_tick_resume),
    set_next_event: Some(owl_timer_set_next_event),
};

unsafe fn owl_timer1_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;

    writel(OWL_Tx_CTL_PD, owl_clkevt_base.add(OWL_Tx_CTL));
    ((*evt).event_handler)(evt);

    IRQ_HANDLED
}

unsafe fn owl_timer_init(node: *mut device_node) -> i32 {
    let mut clk: *mut clk;
    let rate: usize;
    let timer1_irq: i32;
    let mut ret: i32;

    owl_timer_base = of_io_request_and_map(node, 0, b"owl-timer\0".as_ptr() as *const i8);
    if is_err(owl_timer_base) {
        pr_err(b"Can't map timer registers\n\0".as_ptr() as *const i8);
        return ptr_err(owl_timer_base);
    }

    owl_clksrc_base = owl_timer_base.add(0x08);
    owl_clkevt_base = owl_timer_base.add(0x14);

    timer1_irq = of_irq_get_byname(node, b"timer1\0".as_ptr() as *const i8);
    if timer1_irq <= 0 {
        pr_err(b"Can't parse timer1 IRQ\n\0".as_ptr() as *const i8);
        return -EINVAL;
    }

    clk = of_clk_get(node, 0);
    if is_err(clk as *mut core::ffi::c_void) {
        ret = ptr_err(clk as *mut core::ffi::c_void);
        pr_err(b"Failed to get clock for clocksource (%d)\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    rate = clk_get_rate(clk);

    owl_timer_reset(owl_clksrc_base);
    owl_timer_set_enabled(owl_clksrc_base, true);

    sched_clock_register(owl_timer_sched_read, 32, rate);
    ret = clocksource_mmio_init(owl_clksrc_base.add(OWL_Tx_VAL), (*node).name,
                                rate, 200, 32, clocksource_mmio_readl_up);
    if ret != 0 {
        pr_err(b"Failed to register clocksource (%d)\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    owl_timer_reset(owl_clkevt_base);

    ret = request_irq(timer1_irq, owl_timer1_interrupt, IRQF_TIMER,
                      b"owl-timer\0".as_ptr() as *const i8,
                      &mut owl_clockevent as *mut _ as *mut core::ffi::c_void);
    if ret != 0 {
        pr_err(b"failed to request irq %d\n\0".as_ptr() as *const i8, timer1_irq);
        return ret;
    }

    owl_clockevent.cpumask = cpumask_of(0);
    owl_clockevent.irq = timer1_irq;
    clockevents_config_and_register(&mut owl_clockevent, rate, 0xf, 0xffffffff);

    0
}

TIMER_OF_DECLARE!(owl_s500, b"actions,s500-timer\0", owl_timer_init);
TIMER_OF_DECLARE!(owl_s700, b"actions,s700-timer\0", owl_timer_init);
TIMER_OF_DECLARE!(owl_s900, b"actions,s900-timer\0", owl_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
