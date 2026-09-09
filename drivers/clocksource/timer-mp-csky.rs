// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Linux and architecture dependencies are supplied by the surrounding kernel
// translation.

const PTIM_CCVR: &str = "cr<3, 14>";
const PTIM_CTLR: &str = "cr<0, 14>";
const PTIM_LVR: &str = "cr<6, 14>";
const PTIM_TSR: &str = "cr<1, 14>";

static mut CSKY_MPTIMER_IRQ: i32 = 0;

unsafe fn csky_mptimer_set_next_event(
    delta: libc::c_ulong,
    _ce: *mut clock_event_device,
) -> i32 {
    mtcr(PTIM_LVR, delta);
    0
}

unsafe fn csky_mptimer_shutdown(_ce: *mut clock_event_device) -> i32 {
    mtcr(PTIM_CTLR, 0);
    0
}

unsafe fn csky_mptimer_oneshot(_ce: *mut clock_event_device) -> i32 {
    mtcr(PTIM_CTLR, 1);
    0
}

unsafe fn csky_mptimer_oneshot_stopped(_ce: *mut clock_event_device) -> i32 {
    mtcr(PTIM_CTLR, 0);
    0
}

// DEFINE_PER_CPU(struct timer_of, csky_to)
static mut CSKY_TO: timer_of = timer_of {
    flags: TIMER_OF_CLOCK,
    clkevt: clock_event_device {
        rating: 300,
        features: CLOCK_EVT_FEAT_PERCPU | CLOCK_EVT_FEAT_ONESHOT,
        set_state_shutdown: Some(csky_mptimer_shutdown),
        set_state_oneshot: Some(csky_mptimer_oneshot),
        set_state_oneshot_stopped: Some(csky_mptimer_oneshot_stopped),
        set_next_event: Some(csky_mptimer_set_next_event),
        ..clock_event_device::zeroed()
    },
    ..timer_of::zeroed()
};

unsafe fn csky_timer_interrupt(_irq: i32, _dev: *mut core::ffi::c_void) -> irqreturn_t {
    let to: *mut timer_of = this_cpu_ptr(&raw mut CSKY_TO);

    mtcr(PTIM_TSR, 0);
    ((*to).clkevt.event_handler)(&mut (*to).clkevt);

    IRQ_HANDLED
}

// clock event for percpu
unsafe fn csky_mptimer_starting_cpu(cpu: u32) -> i32 {
    let to: *mut timer_of = per_cpu_ptr(&raw mut CSKY_TO, cpu);

    (*to).clkevt.cpumask = cpumask_of(cpu);
    enable_percpu_irq(CSKY_MPTIMER_IRQ, 0);
    clockevents_config_and_register(
        &mut (*to).clkevt,
        timer_of_rate(to),
        2,
        libc::ULONG_MAX,
    );
    0
}

unsafe fn csky_mptimer_dying_cpu(_cpu: u32) -> i32 {
    disable_percpu_irq(CSKY_MPTIMER_IRQ);
    0
}

// clock source
unsafe fn sched_clock_read() -> u64 {
    mfcr(PTIM_CCVR) as u64
}

unsafe fn clksrc_read(_c: *mut clocksource) -> u64 {
    mfcr(PTIM_CCVR) as u64
}

#[no_mangle]
static mut CSKY_CLOCKSOURCE: clocksource = clocksource {
    name: "csky",
    rating: 400,
    mask: CLOCKSOURCE_MASK(32),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    read: Some(clksrc_read),
    ..clocksource::zeroed()
};

unsafe fn csky_mptimer_init(np: *mut device_node) -> i32 {
    let mut ret: i32;
    let mut cpu: i32;
    let mut cpu_rollback: i32;
    let mut to: *mut timer_of = core::ptr::null_mut();

    // Csky_mptimer has private registers and IRQs for each core.  The
    // registers are accessed by mfcr/mtcr instructions rather than MMIO.
    CSKY_MPTIMER_IRQ = irq_of_parse_and_map(np, 0);
    if CSKY_MPTIMER_IRQ <= 0 {
        return -EINVAL;
    }

    ret = request_percpu_irq(
        CSKY_MPTIMER_IRQ,
        Some(csky_timer_interrupt),
        "csky_mp_timer",
        &raw mut CSKY_TO as *mut _,
    );
    if ret != 0 {
        return -EINVAL;
    }

    for_each_possible_cpu!(cpu) {
        to = per_cpu_ptr(&raw mut CSKY_TO, cpu as u32);
        ret = timer_of_init(np, to);
        if ret != 0 {
            for_each_possible_cpu!(cpu_rollback) {
                if cpu_rollback == cpu {
                    break;
                }
                to = per_cpu_ptr(&raw mut CSKY_TO, cpu_rollback as u32);
                timer_of_cleanup(to);
            }
            return -EINVAL;
        }
    }

    clocksource_register_hz(&raw mut CSKY_CLOCKSOURCE, timer_of_rate(to));
    sched_clock_register(Some(sched_clock_read), 32, timer_of_rate(to));

    ret = cpuhp_setup_state(
        CPUHP_AP_CSKY_TIMER_STARTING,
        "clockevents/csky/timer:starting",
        Some(csky_mptimer_starting_cpu),
        Some(csky_mptimer_dying_cpu),
    );
    if ret != 0 {
        return -EINVAL;
    }
    0
}

// TIMER_OF_DECLARE(csky_mptimer, "csky,mptimer", csky_mptimer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
