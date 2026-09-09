// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 * pit.c -- Freescale ColdFire PIT timer. Currently this type of
 *           hardware timer only exists in the Freescale ColdFire
 *           5270/5271, 5282 and 5208 CPUs. No doubt newer ColdFire
 *           family members will probably use it too.
 *
 * Copyright (C) 1999-2008, Greg Ungerer (gerg@snapgear.com)
 * Copyright (C) 2001-2004, SnapGear Inc. (www.snapgear.com)
 */

/***************************************************************************/

// Linux and ColdFire dependencies supplied by other translation units.

/* By default use timer1 as the system clock timer. */
const FREQ: u32 = (MCF_CLK / 2) / 64;

#[inline]
const fn ta(a: u32) -> u32 {
    MCFPIT_BASE1 + a
}

const PIT_CYCLES_PER_JIFFY: u32 = FREQ / HZ;

static mut pit_cnt: u32 = 0;

/*
 * Initialize the PIT timer.
 *
 * This is also called after resume to bring the PIT into operation again.
 */
unsafe fn cf_pit_set_periodic(_evt: *mut clock_event_device) -> i32 {
    mcf_write16(MCFPIT_PCSR_DISABLE, ta(MCFPIT_PCSR));
    mcf_write16(PIT_CYCLES_PER_JIFFY as u16, ta(MCFPIT_PMR));
    mcf_write16(
        MCFPIT_PCSR_EN | MCFPIT_PCSR_PIE | MCFPIT_PCSR_OVW |
            MCFPIT_PCSR_RLD | MCFPIT_PCSR_CLK64,
        ta(MCFPIT_PCSR),
    );
    0
}

unsafe fn cf_pit_set_oneshot(_evt: *mut clock_event_device) -> i32 {
    mcf_write16(MCFPIT_PCSR_DISABLE, ta(MCFPIT_PCSR));
    mcf_write16(
        MCFPIT_PCSR_EN | MCFPIT_PCSR_PIE | MCFPIT_PCSR_OVW | MCFPIT_PCSR_CLK64,
        ta(MCFPIT_PCSR),
    );
    0
}

unsafe fn cf_pit_shutdown(_evt: *mut clock_event_device) -> i32 {
    mcf_write16(MCFPIT_PCSR_DISABLE, ta(MCFPIT_PCSR));
    0
}

/* Program the next event in oneshot mode. Delta is given in PIT ticks. */
unsafe fn cf_pit_next_event(delta: c_ulong, _evt: *mut clock_event_device) -> i32 {
    mcf_write16(delta as u16, ta(MCFPIT_PMR));
    0
}

pub static mut cf_pit_clockevent: clock_event_device = clock_event_device {
    name: "pit",
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    set_state_shutdown: Some(cf_pit_shutdown),
    set_state_periodic: Some(cf_pit_set_periodic),
    set_state_oneshot: Some(cf_pit_set_oneshot),
    set_next_event: Some(cf_pit_next_event),
    shift: 32,
    irq: MCF_IRQ_PIT1,
    ..clock_event_device::default()
};

/***************************************************************************/

unsafe fn pit_tick(_irq: i32, _dummy: *mut c_void) -> irqreturn_t {
    let evt: *mut clock_event_device = &raw mut cf_pit_clockevent;
    let pcsr: u16;

    /* Reset the ColdFire timer */
    pcsr = mcf_read16(ta(MCFPIT_PCSR));
    mcf_write16(pcsr | MCFPIT_PCSR_PIF, ta(MCFPIT_PCSR));

    pit_cnt = pit_cnt.wrapping_add(PIT_CYCLES_PER_JIFFY);
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

/***************************************************************************/

unsafe fn pit_read_clk(_cs: *mut clocksource) -> u64 {
    let mut flags: c_ulong = 0;
    let cycles: u32;
    let pcntr: u16;

    local_irq_save(&mut flags);
    pcntr = mcf_read16(ta(MCFPIT_PCNTR));
    cycles = pit_cnt;
    local_irq_restore(flags);

    (cycles + PIT_CYCLES_PER_JIFFY - pcntr as u32) as u64
}

/***************************************************************************/

static mut pit_clk: clocksource = clocksource {
    name: "pit",
    rating: 100,
    read: Some(pit_read_clk),
    mask: CLOCKSOURCE_MASK(32),
    ..clocksource::default()
};

/***************************************************************************/

pub unsafe fn hw_timer_init() {
    let ret: i32;

    cf_pit_clockevent.cpumask = cpumask_of(smp_processor_id());
    cf_pit_clockevent.mult = div_sc(FREQ, NSEC_PER_SEC, 32);
    cf_pit_clockevent.max_delta_ns =
        clockevent_delta2ns(0xFFFF, &mut cf_pit_clockevent);
    cf_pit_clockevent.max_delta_ticks = 0xFFFF;
    cf_pit_clockevent.min_delta_ns =
        clockevent_delta2ns(0x3f, &mut cf_pit_clockevent);
    cf_pit_clockevent.min_delta_ticks = 0x3f;
    clockevents_register_device(&mut cf_pit_clockevent);

    ret = request_irq(MCF_IRQ_PIT1, Some(pit_tick), IRQF_TIMER, "timer", core::ptr::null_mut());
    if ret != 0 {
        pr_err!(
            "Failed to request irq {} (timer): %pe\\n",
            MCF_IRQ_PIT1,
            ERR_PTR(ret)
        );
    }

    clocksource_register_hz(&mut pit_clk, FREQ);
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
