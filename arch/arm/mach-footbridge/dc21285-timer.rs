// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/arm/mach-footbridge/dc21285-timer.c
 *
 *  Copyright (C) 1998 Russell King.
 *  Copyright (C) 1998 Phil Blundell
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn cksrc_dc21285_read(cs: *mut clocksource) -> u64 {
    (*cs).mask.wrapping_sub(core::ptr::read_volatile(CSR_TIMER2_VALUE))
}

unsafe fn cksrc_dc21285_enable(cs: *mut clocksource) -> i32 {
    core::ptr::write_volatile(CSR_TIMER2_LOAD, (*cs).mask);
    core::ptr::write_volatile(CSR_TIMER2_CLR, 0);
    core::ptr::write_volatile(
        CSR_TIMER2_CNTL,
        TIMER_CNTL_ENABLE | TIMER_CNTL_DIV16,
    );
    0
}

unsafe fn cksrc_dc21285_disable(_cs: *mut clocksource) {
    core::ptr::write_volatile(CSR_TIMER2_CNTL, 0);
}

static mut cksrc_dc21285: clocksource = clocksource {
    name: "dc21285_timer2",
    rating: 200,
    read: Some(cksrc_dc21285_read),
    enable: Some(cksrc_dc21285_enable),
    disable: Some(cksrc_dc21285_disable),
    mask: CLOCKSOURCE_MASK(24),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe fn ckevt_dc21285_set_next_event(
    delta: c_ulong,
    _c: *mut clock_event_device,
) -> i32 {
    core::ptr::write_volatile(CSR_TIMER1_CLR, 0);
    core::ptr::write_volatile(CSR_TIMER1_LOAD, delta);
    core::ptr::write_volatile(
        CSR_TIMER1_CNTL,
        TIMER_CNTL_ENABLE | TIMER_CNTL_DIV16,
    );
    0
}

unsafe fn ckevt_dc21285_shutdown(_c: *mut clock_event_device) -> i32 {
    core::ptr::write_volatile(CSR_TIMER1_CNTL, 0);
    0
}

unsafe fn ckevt_dc21285_set_periodic(_c: *mut clock_event_device) -> i32 {
    core::ptr::write_volatile(CSR_TIMER1_CLR, 0);
    core::ptr::write_volatile(
        CSR_TIMER1_LOAD,
        (mem_fclk_21285.wrapping_add(8 * HZ)) / (16 * HZ),
    );
    core::ptr::write_volatile(
        CSR_TIMER1_CNTL,
        TIMER_CNTL_ENABLE | TIMER_CNTL_AUTORELOAD | TIMER_CNTL_DIV16,
    );
    0
}

static mut ckevt_dc21285: clock_event_device = clock_event_device {
    name: "dc21285_timer1",
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    rating: 200,
    irq: IRQ_TIMER1,
    set_next_event: Some(ckevt_dc21285_set_next_event),
    set_state_shutdown: Some(ckevt_dc21285_shutdown),
    set_state_periodic: Some(ckevt_dc21285_set_periodic),
    set_state_oneshot: Some(ckevt_dc21285_shutdown),
    tick_resume: Some(ckevt_dc21285_set_periodic),
};

unsafe fn timer1_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let ce = dev_id as *mut clock_event_device;

    core::ptr::write_volatile(CSR_TIMER1_CLR, 0);

    /* Stop the timer if in one-shot mode */
    if clockevent_state_oneshot(ce) {
        core::ptr::write_volatile(CSR_TIMER1_CNTL, 0);
    }

    ((*ce).event_handler)(ce);

    IRQ_HANDLED
}

/*
 * Set up timer interrupt.
 */
unsafe fn footbridge_timer_init() {
    let ce = &mut ckevt_dc21285 as *mut clock_event_device;
    let rate: c_uint = DIV_ROUND_CLOSEST(mem_fclk_21285, 16);

    clocksource_register_hz(&mut cksrc_dc21285, rate);

    if request_irq(
        (*ce).irq,
        timer1_interrupt,
        IRQF_TIMER | IRQF_IRQPOLL,
        "dc21285_timer1",
        &mut ckevt_dc21285 as *mut _ as *mut core::ffi::c_void,
    ) != 0 {
        pr_err("Failed to request irq %d (dc21285_timer1)", (*ce).irq);
    }

    (*ce).cpumask = cpumask_of(smp_processor_id());
    clockevents_config_and_register(ce, rate, 0x4, 0xffffff);
}

unsafe fn footbridge_read_sched_clock() -> u64 {
    !core::ptr::read_volatile(CSR_TIMER3_VALUE)
}

unsafe fn footbridge_sched_clock() {
    let rate: c_uint = DIV_ROUND_CLOSEST(mem_fclk_21285, 16);

    core::ptr::write_volatile(CSR_TIMER3_LOAD, 0);
    core::ptr::write_volatile(CSR_TIMER3_CLR, 0);
    core::ptr::write_volatile(
        CSR_TIMER3_CNTL,
        TIMER_CNTL_ENABLE | TIMER_CNTL_DIV16,
    );

    sched_clock_register(footbridge_read_sched_clock, 24, rate);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
