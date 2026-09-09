// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/arch/arm/mach-at91/at91rm9200_time.c
 *
 *  Copyright (C) 2003 SAN People
 *  Copyright (C) 2003 ATMEL
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static mut last_crtr: c_ulong = 0;
static mut irqmask: u32 = 0;
static mut clkevt: clock_event_device = clock_event_device {
    name: "at91_tick",
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    rating: 150,
    set_next_event: Some(clkevt32k_next_event),
    set_state_shutdown: Some(clkevt32k_shutdown),
    set_state_periodic: Some(clkevt32k_set_periodic),
    set_state_oneshot: Some(clkevt32k_set_oneshot),
    tick_resume: Some(clkevt32k_shutdown),
    ..clock_event_device::default()
};
static mut regmap_st: *mut regmap = core::ptr::null_mut();
static mut timer_latch: c_int = 0;

/*
 * The ST_CRTR is updated asynchronously to the master clock ... but
 * the updates as seen by the CPU don't seem to be strictly monotonic.
 * Waiting until we read the same value twice avoids glitching.
 */
unsafe fn read_CRTR() -> c_ulong {
    let mut x1: c_uint = 0;
    let mut x2: c_uint;

    regmap_read(regmap_st, AT91_ST_CRTR, &mut x1);
    loop {
        regmap_read(regmap_st, AT91_ST_CRTR, &mut x2);
        if x1 == x2 {
            break;
        }
        x1 = x2;
    }
    x1 as c_ulong
}

/* IRQ handler for the timer. */
unsafe fn at91rm9200_timer_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let mut sr: u32 = 0;

    regmap_read(regmap_st, AT91_ST_SR, &mut sr);
    sr &= irqmask;
    WARN_ON_ONCE(!irqs_disabled());

    /* simulate "oneshot" timer with alarm */
    if sr & AT91_ST_ALMS != 0 {
        ((*clkevt.event_handler.expect("event_handler"))(&mut clkevt));
        return IRQ_HANDLED;
    }

    /* periodic mode should handle delayed ticks */
    if sr & AT91_ST_PITS != 0 {
        let crtr = read_CRTR() as u32;
        while ((crtr.wrapping_sub(last_crtr as u32)) & AT91_ST_CRTV) >= timer_latch as u32 {
            last_crtr = last_crtr.wrapping_add(timer_latch as c_ulong);
            ((*clkevt.event_handler.expect("event_handler"))(&mut clkevt));
        }
        return IRQ_HANDLED;
    }

    /* this irq is shared ... */
    IRQ_NONE
}

unsafe fn read_clk32k(_cs: *mut clocksource) -> u64 { read_CRTR() as u64 }

static mut clk32k: clocksource = clocksource {
    name: "32k_counter",
    rating: 150,
    read: Some(read_clk32k),
    mask: CLOCKSOURCE_MASK(20),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    ..clocksource::default()
};

unsafe fn clkdev32k_disable_and_flush_irq() {
    let mut val: c_uint = 0;
    regmap_write(regmap_st, AT91_ST_IDR, AT91_ST_PITS | AT91_ST_ALMS);
    regmap_read(regmap_st, AT91_ST_SR, &mut val);
    last_crtr = read_CRTR();
}

unsafe fn clkevt32k_shutdown(_evt: *mut clock_event_device) -> c_int {
    clkdev32k_disable_and_flush_irq();
    irqmask = 0;
    regmap_write(regmap_st, AT91_ST_IER, irqmask);
    0
}

unsafe fn clkevt32k_set_oneshot(_dev: *mut clock_event_device) -> c_int {
    clkdev32k_disable_and_flush_irq();
    /* ALM for oneshot irqs, set by next_event() before 32 seconds have passed. */
    irqmask = AT91_ST_ALMS;
    regmap_write(regmap_st, AT91_ST_RTAR, last_crtr as u32);
    regmap_write(regmap_st, AT91_ST_IER, irqmask);
    0
}

unsafe fn clkevt32k_set_periodic(_dev: *mut clock_event_device) -> c_int {
    clkdev32k_disable_and_flush_irq();
    /* PIT for periodic irqs; fixed rate of 1/HZ */
    irqmask = AT91_ST_PITS;
    regmap_write(regmap_st, AT91_ST_PIMR, timer_latch as u32);
    regmap_write(regmap_st, AT91_ST_IER, irqmask);
    0
}

unsafe fn clkevt32k_next_event(delta: c_ulong, _dev: *mut clock_event_device) -> c_int {
    let mut alm: u32;
    let mut val: c_uint = 0;

    BUG_ON(delta < 2);
    /* The alarm IRQ uses absolute time (now+delta), not the relative
     * time (delta) in our calling convention.  Like all clockevents
     * using such "match" hardware, we have a race to defend against.
     *
     * Our defense here is to have set up the clockevent device so the
     * delta is at least two.  That way we never end up writing RTAR
     * with the value then held in CRTR ... which would mean the match
     * wouldn't trigger until 32 seconds later, after CRTR wraps.
     */
    alm = read_CRTR() as u32;
    /* Cancel any pending alarm; flush any pending IRQ */
    regmap_write(regmap_st, AT91_ST_RTAR, alm);
    regmap_read(regmap_st, AT91_ST_SR, &mut val);
    /* Schedule alarm by writing RTAR. */
    alm = alm.wrapping_add(delta as u32);
    regmap_write(regmap_st, AT91_ST_RTAR, alm);
    0
}

/* ST (system timer) module supports both clockevents and clocksource. */
unsafe fn atmel_st_timer_init(node: *mut device_node) -> c_int {
    let mut sclk: *mut clk;
    let (mut sclk_rate, mut val): (c_uint, c_uint) = (0, 0);
    let (mut irq, mut ret): (c_int, c_int);

    regmap_st = syscon_node_to_regmap(node);
    if IS_ERR(regmap_st) { pr_err!("Unable to get regmap\n"); return PTR_ERR(regmap_st); }
    regmap_write(regmap_st, AT91_ST_IDR, AT91_ST_PITS | AT91_ST_WDOVF | AT91_ST_RTTINC | AT91_ST_ALMS);
    regmap_read(regmap_st, AT91_ST_SR, &mut val);
    irq = irq_of_parse_and_map(node, 0);
    if irq == 0 { pr_err!("Unable to get IRQ from DT\n"); return -EINVAL; }
    ret = request_irq(irq, Some(at91rm9200_timer_interrupt), IRQF_SHARED | IRQF_TIMER | IRQF_IRQPOLL, "at91_tick", regmap_st as *mut c_void);
    if ret != 0 { pr_err!("Unable to setup IRQ\n"); return ret; }
    sclk = of_clk_get(node, 0);
    if IS_ERR(sclk) { pr_err!("Unable to get slow clock\n"); return PTR_ERR(sclk); }
    ret = clk_prepare_enable(sclk);
    if ret != 0 { pr_err!("Could not enable slow clock\n"); return ret; }
    sclk_rate = clk_get_rate(sclk);
    if sclk_rate == 0 { pr_err!("Invalid slow clock rate\n"); return -EINVAL; }
    timer_latch = ((sclk_rate + HZ / 2) / HZ) as c_int;
    /* The 32KiHz "Slow Clock" (tick every 30517.58 nanoseconds) is used
     * directly for the clocksource and all clockevents, after adjusting
     * its prescaler from the 1 Hz default.
     */
    regmap_write(regmap_st, AT91_ST_RTMR, 1);
    /* Setup timer clockevent, with minimum of two ticks (important!!) */
    clkevt.cpumask = cpumask_of(0);
    clockevents_config_and_register(&mut clkevt, sclk_rate, 2, AT91_ST_ALMV);
    /* register clocksource */
    clocksource_register_hz(&mut clk32k, sclk_rate)
}

TIMER_OF_DECLARE!(atmel_st_timer, "atmel,at91rm9200-st", atmel_st_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
