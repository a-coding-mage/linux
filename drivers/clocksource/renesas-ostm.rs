// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Timer Support - OSTM
 *
 * Copyright (C) 2017 Renesas Electronics America, Inc.
 * Copyright (C) 2017 Chris Brandt
 */

// Linux kernel dependencies supplied by other translation units.

/*
 * The OSTM contains independent channels.
 * The first OSTM channel probed will be set up as a free running
 * clocksource. Additionally we will use this clocksource for the system
 * schedule timer sched_clock().
 *
 * The second (or more) channel probed will be set up as an interrupt
 * driven clock event.
 */

static mut system_clock: *mut core::ffi::c_void = core::ptr::null_mut(); // For sched_clock()

/* OSTM REGISTERS */
const OSTM_CMP: usize = 0x000; // RW,32
const OSTM_CNT: usize = 0x004; // R,32
const OSTM_TE: usize = 0x010; // R,8
const OSTM_TS: usize = 0x014; // W,8
const OSTM_TT: usize = 0x018; // W,8
const OSTM_CTL: usize = 0x020; // RW,8

const TE: u8 = 0x01;
const TS: u8 = 0x01;
const TT: u8 = 0x01;
const CTL_PERIODIC: u8 = 0x00;
const CTL_ONESHOT: u8 = 0x02;
const CTL_FREERUN: u8 = 0x02;

unsafe fn ostm_timer_stop(to: *mut timer_of) {
    if readb(timer_of_base(to).add(OSTM_TE)) & TE != 0 {
        writeb(TT, timer_of_base(to).add(OSTM_TT));

        /*
         * Read back the register simply to confirm the write operation
         * has completed since I/O writes can sometimes get queued by
         * the bus architecture.
         */
        while readb(timer_of_base(to).add(OSTM_TE)) & TE != 0 {}
    }
}

unsafe fn ostm_init_clksrc(to: *mut timer_of) -> i32 {
    ostm_timer_stop(to);

    writel(0, timer_of_base(to).add(OSTM_CMP));
    writeb(CTL_FREERUN, timer_of_base(to).add(OSTM_CTL));
    writeb(TS, timer_of_base(to).add(OSTM_TS));

    clocksource_mmio_init(timer_of_base(to).add(OSTM_CNT), (*(*to).np).full_name, timer_of_rate(to), 300, 32, clocksource_mmio_readl_up)
}

unsafe fn ostm_read_sched_clock() -> u64 {
    readl(system_clock) as u64
}

unsafe fn ostm_init_sched_clock(to: *mut timer_of) {
    system_clock = timer_of_base(to).add(OSTM_CNT);
    sched_clock_register(ostm_read_sched_clock, 32, timer_of_rate(to));
}

unsafe fn ostm_clock_event_next(delta: usize, ced: *mut clock_event_device) -> i32 {
    let to = to_timer_of(ced);

    ostm_timer_stop(to);
    writel(delta as u32, timer_of_base(to).add(OSTM_CMP));
    writeb(CTL_ONESHOT, timer_of_base(to).add(OSTM_CTL));
    writeb(TS, timer_of_base(to).add(OSTM_TS));

    0
}

unsafe fn ostm_shutdown(ced: *mut clock_event_device) -> i32 {
    let to = to_timer_of(ced);
    ostm_timer_stop(to);
    0
}

unsafe fn ostm_set_periodic(ced: *mut clock_event_device) -> i32 {
    let to = to_timer_of(ced);

    if clockevent_state_oneshot(ced) || clockevent_state_periodic(ced) {
        ostm_timer_stop(to);
    }
    writel(timer_of_period(to).wrapping_sub(1), timer_of_base(to).add(OSTM_CMP));
    writeb(CTL_PERIODIC, timer_of_base(to).add(OSTM_CTL));
    writeb(TS, timer_of_base(to).add(OSTM_TS));
    0
}

unsafe fn ostm_set_oneshot(ced: *mut clock_event_device) -> i32 {
    ostm_timer_stop(to_timer_of(ced));
    0
}

unsafe fn ostm_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let ced = dev_id as *mut clock_event_device;
    if clockevent_state_oneshot(ced) {
        ostm_timer_stop(to_timer_of(ced));
    }
    /* notify clockevent layer */
    if !(*ced).event_handler.is_null() {
        ((*ced).event_handler)(ced);
    }
    IRQ_HANDLED
}

unsafe fn ostm_init_clkevt(to: *mut timer_of) -> i32 {
    let ced = &mut (*to).clkevt as *mut clock_event_device;
    (*ced).features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC;
    (*ced).set_state_shutdown = Some(ostm_shutdown);
    (*ced).set_state_periodic = Some(ostm_set_periodic);
    (*ced).set_state_oneshot = Some(ostm_set_oneshot);
    (*ced).set_next_event = Some(ostm_clock_event_next);
    (*ced).shift = 32;
    (*ced).rating = 300;
    (*ced).cpumask = cpumask_of(0);
    clockevents_config_and_register(ced, timer_of_rate(to), 0xf, 0xffffffff);
    0
}

unsafe fn ostm_init(np: *mut device_node) -> i32 {
    let mut rstc: *mut reset_control;
    let to = kzalloc_obj::<timer_of>();
    if to.is_null() { return -ENOMEM; }

    rstc = of_reset_control_get_optional_exclusive(np, core::ptr::null());
    if IS_ERR(rstc) {
        let ret = PTR_ERR(rstc);
        kfree(to);
        return ret;
    }
    reset_control_deassert(rstc);

    (*to).flags = TIMER_OF_BASE | TIMER_OF_CLOCK;
    if !system_clock.is_null() {
        /* clock sources don't use interrupts, clock events do */
        (*to).flags |= TIMER_OF_IRQ;
        (*to).of_irq.flags = IRQF_TIMER | IRQF_IRQPOLL;
        (*to).of_irq.handler = Some(ostm_timer_interrupt);
    }

    let mut ret = timer_of_init(np, to);
    if ret != 0 { reset_control_assert(rstc); reset_control_put(rstc); kfree(to); return ret; }

    if system_clock.is_null() {
        ret = ostm_init_clksrc(to);
        if ret != 0 { timer_of_cleanup(to); reset_control_assert(rstc); reset_control_put(rstc); kfree(to); return ret; }
        ostm_init_sched_clock(to);
        pr_info!("%pOF: used for clocksource\n", np);
    } else {
        ret = ostm_init_clkevt(to);
        if ret != 0 { timer_of_cleanup(to); reset_control_assert(rstc); reset_control_put(rstc); kfree(to); return ret; }
        pr_info!("%pOF: used for clock events\n", np);
    }
    of_node_set_flag(np, OF_POPULATED);
    0
}

TIMER_OF_DECLARE!(ostm, "renesas,ostm", ostm_init);

unsafe fn ostm_probe(pdev: *mut platform_device) -> i32 {
    ostm_init((*pdev).dev.of_node)
}

static ostm_of_table: [of_device_id; 2] = [
    of_device_id { compatible: "renesas,ostm", ..Default::default() },
    of_device_id { ..Default::default() },
];

static mut ostm_device_driver: platform_driver = platform_driver {
    driver: driver {
        name: "renesas_ostm",
        of_match_table: of_match_ptr!(ostm_of_table),
        suppress_bind_attrs: true,
    },
};

builtin_platform_driver_probe!(ostm_device_driver, ostm_probe);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
