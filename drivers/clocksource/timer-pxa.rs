// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-pxa/time.c
 *
 * PXA clocksource, clockevents, and OST interrupt handlers.
 * Copyright (c) 2007 by Bill Gatliff <bgat@billgatliff.com>.
 *
 * Derived from Nicolas Pitre's PXA timer handler Copyright (c) 2001
 * by MontaVista Software, Inc.  (Nico, your code rocks!)
 */

const OSMR0: usize = 0x00;
const OSMR1: usize = 0x04;
const OSMR2: usize = 0x08;
const OSMR3: usize = 0x0C;
const OSCR: usize = 0x10;
const OSSR: usize = 0x14;
const OWER: usize = 0x18;
const OIER: usize = 0x1C;

const OSSR_M3: u32 = 1 << 3;
const OSSR_M2: u32 = 1 << 2;
const OSSR_M1: u32 = 1 << 1;
const OSSR_M0: u32 = 1 << 0;
const OIER_E0: u32 = 1 << 0;

/*
 * This is PXA's sched_clock implementation. This has a resolution
 * of at least 308 ns and a maximum value of 208 days.
 *
 * The return value is guaranteed to be monotonic in that range as
 * long as there is always less than 582 seconds between successive
 * calls to sched_clock() which should always be the case in practice.
 */

static mut timer_base: *mut core::ffi::c_void = core::ptr::null_mut();

#[inline]
unsafe fn timer_readl(reg: usize) -> u32 {
    readl_relaxed((timer_base as *mut u8).add(reg))
}

#[inline]
unsafe fn timer_writel(val: u32, reg: usize) {
    writel_relaxed(val, (timer_base as *mut u8).add(reg));
}

unsafe fn pxa_read_sched_clock() -> u64 {
    timer_readl(OSCR) as u64
}

const MIN_OSCR_DELTA: u32 = 16;

unsafe fn pxa_ost0_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let c = dev_id as *mut clock_event_device;

    /* Disarm the compare/match, signal the event. */
    timer_writel(timer_readl(OIER) & !OIER_E0, OIER);
    timer_writel(OSSR_M0, OSSR);
    ((*c).event_handler.unwrap())(c);

    IRQ_HANDLED
}

unsafe fn pxa_osmr0_set_next_event(delta: usize, _dev: *mut clock_event_device) -> i32 {
    timer_writel(timer_readl(OIER) | OIER_E0, OIER);
    let next = (timer_readl(OSCR) as usize).wrapping_add(delta);
    timer_writel(next as u32, OSMR0);
    let oscr = timer_readl(OSCR) as usize;

    if (next.wrapping_sub(oscr) as i32) <= MIN_OSCR_DELTA as i32 { -ETIME } else { 0 }
}

unsafe fn pxa_osmr0_shutdown(_evt: *mut clock_event_device) -> i32 {
    /* initializing, released, or preparing for suspend */
    timer_writel(timer_readl(OIER) & !OIER_E0, OIER);
    timer_writel(OSSR_M0, OSSR);
    0
}

#[cfg(CONFIG_PM)]
static mut osmr: [usize; 4] = [0; 4];
#[cfg(CONFIG_PM)]
static mut oier: usize = 0;
#[cfg(CONFIG_PM)]
static mut oscr: usize = 0;

#[cfg(CONFIG_PM)]
unsafe fn pxa_timer_suspend(_cedev: *mut clock_event_device) {
    osmr[0] = timer_readl(OSMR0) as usize;
    osmr[1] = timer_readl(OSMR1) as usize;
    osmr[2] = timer_readl(OSMR2) as usize;
    osmr[3] = timer_readl(OSMR3) as usize;
    oier = timer_readl(OIER) as usize;
    oscr = timer_readl(OSCR) as usize;
}

#[cfg(CONFIG_PM)]
unsafe fn pxa_timer_resume(_cedev: *mut clock_event_device) {
    /*
     * Ensure that we have at least MIN_OSCR_DELTA between match
     * register 0 and the OSCR, to guarantee that we will receive
     * the one-shot timer interrupt.  We adjust OSMR0 in preference
     * to OSCR to guarantee that OSCR is monotonically incrementing.
     */
    if osmr[0].wrapping_sub(oscr) < MIN_OSCR_DELTA as usize {
        osmr[0] = osmr[0].wrapping_add(MIN_OSCR_DELTA as usize);
    }

    timer_writel(osmr[0] as u32, OSMR0);
    timer_writel(osmr[1] as u32, OSMR1);
    timer_writel(osmr[2] as u32, OSMR2);
    timer_writel(osmr[3] as u32, OSMR3);
    timer_writel(oier as u32, OIER);
    timer_writel(oscr as u32, OSCR);
}

#[cfg(not(CONFIG_PM))]
const pxa_timer_suspend: Option<unsafe fn(*mut clock_event_device)> = None;
#[cfg(not(CONFIG_PM))]
const pxa_timer_resume: Option<unsafe fn(*mut clock_event_device)> = None;

static mut ckevt_pxa_osmr0: clock_event_device = clock_event_device {
    name: b"osmr0\0".as_ptr() as *const i8,
    features: CLOCK_EVT_FEAT_ONESHOT,
    rating: 200,
    set_next_event: Some(pxa_osmr0_set_next_event),
    set_state_shutdown: Some(pxa_osmr0_shutdown),
    set_state_oneshot: Some(pxa_osmr0_shutdown),
    suspend: Some(pxa_timer_suspend),
    resume: Some(pxa_timer_resume),
    ..clock_event_device::default()
};

unsafe fn pxa_timer_common_init(irq: i32, clock_tick_rate: u32) -> i32 {
    timer_writel(0, OIER);
    timer_writel(OSSR_M0 | OSSR_M1 | OSSR_M2 | OSSR_M3, OSSR);

    sched_clock_register(Some(pxa_read_sched_clock), 32, clock_tick_rate);
    ckevt_pxa_osmr0.cpumask = cpumask_of(0);

    let mut ret = request_irq(irq, Some(pxa_ost0_interrupt), IRQF_TIMER | IRQF_IRQPOLL,
                              b"ost0\0".as_ptr() as *const i8,
                              &mut ckevt_pxa_osmr0 as *mut _ as *mut core::ffi::c_void);
    if ret != 0 {
        pr_err!("Failed to setup irq\n");
        return ret;
    }

    ret = clocksource_mmio_init((timer_base as *mut u8).add(OSCR), b"oscr0\0".as_ptr() as *const i8,
                                 clock_tick_rate, 200, 32, Some(clocksource_mmio_readl_up));
    if ret != 0 {
        pr_err!("Failed to init clocksource\n");
        return ret;
    }

    clockevents_config_and_register(&mut ckevt_pxa_osmr0, clock_tick_rate,
                                    MIN_OSCR_DELTA * 2, 0x7fffffff);
    0
}

unsafe fn pxa_timer_dt_init(np: *mut device_node) -> i32 {
    /* timer registers are shared with watchdog timer */
    timer_base = of_iomap(np, 0);
    if timer_base.is_null() {
        pr_err!("%pOFn: unable to map resource\n", np);
        return -ENXIO;
    }

    let clk = of_clk_get(np, 0);
    if IS_ERR(clk) {
        pr_crit!("%pOFn: unable to get clk\n", np);
        return PTR_ERR(clk);
    }

    let ret = clk_prepare_enable(clk);
    if ret != 0 {
        pr_crit!("Failed to prepare clock\n");
        return ret;
    }

    /* we are only interested in OS-timer0 irq */
    let irq = irq_of_parse_and_map(np, 0);
    if irq <= 0 {
        pr_crit!("%pOFn: unable to parse OS-timer0 irq\n", np);
        return -EINVAL;
    }

    pxa_timer_common_init(irq, clk_get_rate(clk))
}

TIMER_OF_DECLARE!(pxa_timer, "marvell,pxa-timer", pxa_timer_dt_init);

/*
 * Legacy timer init for non device-tree boards.
 */
unsafe fn pxa_timer_nodt_init(irq: i32, base: *mut core::ffi::c_void) {
    timer_base = base;
    let clk = clk_get(core::ptr::null_mut(), b"OSTIMER0\0".as_ptr() as *const i8);
    if !clk.is_null() && !IS_ERR(clk) {
        clk_prepare_enable(clk);
        pxa_timer_common_init(irq, clk_get_rate(clk));
    } else {
        pr_crit!("%s: unable to get clk\n", b"pxa_timer_nodt_init\0".as_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
