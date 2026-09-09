// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC time.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    fn timer_interrupt(regs: *mut pt_regs) -> irqreturn_t;
}

/* Test the timer ticks to count, used in sync routine */
#[inline]
unsafe fn openrisc_timer_set(count: c_ulong) {
    mtspr(SPR_TTCR, count);
}

/* Set the timer to trigger in delta cycles */
#[inline]
unsafe fn openrisc_timer_set_next(delta: c_ulong) {
    let mut c: u32;

    /* Read 32-bit counter value, add delta, mask off the low 28 bits.
     * We're guaranteed delta won't be bigger than 28 bits because the
     * generic timekeeping code ensures that for us.
     */
    c = mfspr(SPR_TTCR);
    c = c.wrapping_add(delta as u32);
    c &= SPR_TTMR_TP;

    /* Set counter and enable interrupt.
     * Keep timer in continuous mode always.
     */
    mtspr(SPR_TTMR, SPR_TTMR_CR | SPR_TTMR_IE | c);
}

unsafe fn openrisc_timer_set_next_event(
    delta: c_ulong,
    dev: *mut clock_event_device,
) -> i32 {
    openrisc_timer_set_next(delta);
    0
}

/* This is the clock event device based on the OR1K tick timer.
 * As the timer is being used as a continuous clock-source (required for HR
 * timers) we cannot enable the PERIODIC feature.  The tick timer can run using
 * one-shot events, so no problem.
 */
// DEFINE_PER_CPU(struct clock_event_device, clockevent_openrisc_timer);
static mut clockevent_openrisc_timer: PerCpu<clock_event_device>;

unsafe fn openrisc_clockevent_init() {
    let cpu: c_uint = smp_processor_id();
    let evt: *mut clock_event_device =
        &mut per_cpu(clockevent_openrisc_timer, cpu);
    let cpuinfo: *mut cpuinfo_or1k = &mut cpuinfo_or1k[cpu as usize];

    mtspr(SPR_TTMR, SPR_TTMR_CR);

    #[cfg(CONFIG_SMP)]
    {
        (*evt).broadcast = tick_broadcast;
    }
    (*evt).name = "openrisc_timer_clockevent";
    (*evt).features = CLOCK_EVT_FEAT_ONESHOT;
    (*evt).rating = 300;
    (*evt).set_next_event = openrisc_timer_set_next_event;

    (*evt).cpumask = cpumask_of(cpu);

    /* We only have 28 bits */
    clockevents_config_and_register(evt, (*cpuinfo).clock_frequency,
                                    100, 0x0fffffff);
}

#[inline]
unsafe fn timer_ack() {
    /* Clear the IP bit and disable further interrupts */
    /* This can be done very simply... we just need to keep the timer
       running, so just maintain the CR bits while clearing the rest
       of the register
     */
    mtspr(SPR_TTMR, SPR_TTMR_CR);
}

/*
 * The timer interrupt is mostly handled in generic code nowadays... this
 * function just acknowledges the interrupt and fires the event handler that
 * has been set on the clockevent device by the generic time management code.
 *
 * This function needs to be called by the timer exception handler and that's
 * all the exception handler needs to do.
 */
unsafe fn timer_interrupt_impl(regs: *mut pt_regs) -> irqreturn_t {
    let old_regs: *mut pt_regs = set_irq_regs(regs);
    let cpu: c_uint = smp_processor_id();
    let evt: *mut clock_event_device =
        &mut per_cpu(clockevent_openrisc_timer, cpu);

    timer_ack();

    /*
     * update_process_times() expects us to have called irq_enter().
     */
    irq_enter();
    ((*evt).event_handler)(evt);
    irq_exit();

    set_irq_regs(old_regs);

    IRQ_HANDLED
}

/*
 * Clocksource: Based on OpenRISC timer/counter
 *
 * This sets up the OpenRISC Tick Timer as a clock source.  The tick timer
 * is 32 bits wide and runs at the CPU clock frequency.
 */
unsafe fn openrisc_timer_read(cs: *mut clocksource) -> u64 {
    mfspr(SPR_TTCR) as u64
}

static mut openrisc_timer: clocksource = clocksource {
    name: "openrisc_timer",
    rating: 200,
    read: openrisc_timer_read,
    mask: CLOCKSOURCE_MASK(32),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe fn openrisc_timer_init() -> i32 {
    let cpuinfo: *mut cpuinfo_or1k = &mut cpuinfo_or1k[smp_processor_id() as usize];

    if clocksource_register_hz(&mut openrisc_timer, (*cpuinfo).clock_frequency) != 0 {
        panic!("failed to register clocksource");
    }

    /* Enable the incrementer: 'continuous' mode with interrupt disabled */
    mtspr(SPR_TTMR, SPR_TTMR_CR);

    0
}

unsafe fn time_init() {
    let upr: u32;

    upr = mfspr(SPR_UPR);
    if (upr & SPR_UPR_TTP) == 0 {
        panic!("Linux not supported on devices without tick timer");
    }

    openrisc_timer_init();
    openrisc_clockevent_init();

    of_clk_init(core::ptr::null_mut());
    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
