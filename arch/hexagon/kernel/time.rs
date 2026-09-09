// SPDX-License-Identifier: GPL-2.0-only
/*
 * Time related functions for Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Linux and Hexagon dependencies are supplied by the surrounding kernel.

const TIMER_ENABLE: u32 = 1 << 0;

/*
 * For the clocksource we need:
 *     pcycle frequency (600MHz)
 * For the loops_per_jiffy we need:
 *     thread/cpu frequency (100MHz)
 * And for the timer, we need:
 *     sleep clock rate
 */

static mut pcycle_freq_mhz: cycles_t = 0;
static mut thread_freq_mhz: cycles_t = 0;
static mut sleep_clk_freq: cycles_t = 0;

/*
 * 8x50 HDD Specs 5-8.  Simulator co-sim not fixed until
 * release 1.1, and then it's "adjustable" and probably not defaulted.
 */
const RTOS_TIMER_INT: i32 = 3;
const RTOS_TIMER_REGS_ADDR: usize = 0xAB000000;

static mut rtos_timer_resources: [struct_resource; 1] = [struct_resource {
    start: RTOS_TIMER_REGS_ADDR,
    end: RTOS_TIMER_REGS_ADDR + PAGE_SIZE - 1,
    flags: IORESOURCE_MEM,
}];

static mut rtos_timer_device: platform_device = platform_device {
    name: "rtos_timer",
    id: -1,
    num_resources: 1,
    resource: unsafe { &mut rtos_timer_resources[0] },
};

/*  A lot of this stuff should move into a platform specific section.  */
#[repr(C)]
struct adsp_hw_timer_struct {
    match_: u32, // Match value
    count: u32,
    enable: u32, // [1] - CLR_ON_MATCH_EN, [0] - EN
    clear: u32,  // one-shot register that clears the count
}

/*  Look for "TCX0" for related constants.  */
static mut rtos_timer: *mut adsp_hw_timer_struct = core::ptr::null_mut();

unsafe extern "C" fn timer_get_cycles(cs: *mut clocksource) -> u64 {
    __vmgettime() as u64
}

static mut hexagon_clocksource: clocksource = clocksource {
    name: "pcycles",
    rating: 250,
    read: Some(timer_get_cycles),
    mask: CLOCKSOURCE_MASK_64,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe extern "C" fn set_next_event(delta: c_ulong, evt: *mut clock_event_device) -> i32 {
    /* Assuming the timer will be disabled when we enter here. */
    iowrite32(1, core::ptr::addr_of_mut!((*rtos_timer).clear));
    iowrite32(0, core::ptr::addr_of_mut!((*rtos_timer).clear));
    iowrite32(delta as u32, core::ptr::addr_of_mut!((*rtos_timer).match_));
    iowrite32(TIMER_ENABLE, core::ptr::addr_of_mut!((*rtos_timer).enable));
    0
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn broadcast(mask: *const cpumask) {
    send_ipi(mask, IPI_TIMER);
}

/* XXX Implement set_state_shutdown() */
static mut hexagon_clockevent_dev: clock_event_device = clock_event_device {
    name: "clockevent",
    features: CLOCK_EVT_FEAT_ONESHOT,
    rating: 400,
    irq: RTOS_TIMER_INT,
    set_next_event: Some(set_next_event),
    #[cfg(CONFIG_SMP)]
    broadcast: Some(broadcast),
};

#[cfg(CONFIG_SMP)]
static mut clock_events: per_cpu<clock_event_device> = DEFINE_PER_CPU!();

#[cfg(CONFIG_SMP)]
unsafe fn setup_percpu_clockdev() {
    let cpu = smp_processor_id();
    let ce_dev = core::ptr::addr_of_mut!(hexagon_clockevent_dev);
    let dummy_clock_dev = per_cpu_ptr(core::ptr::addr_of_mut!(clock_events), cpu);
    memcpy(dummy_clock_dev, ce_dev, core::mem::size_of::<clock_event_device>());
    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*dummy_clock_dev).list));
    (*dummy_clock_dev).features = CLOCK_EVT_FEAT_DUMMY;
    (*dummy_clock_dev).cpumask = cpumask_of(cpu);
    clockevents_register_device(dummy_clock_dev);
}

#[cfg(CONFIG_SMP)]
unsafe fn ipi_timer() {
    let cpu = smp_processor_id();
    let ce_dev = per_cpu_ptr(core::ptr::addr_of_mut!(clock_events), cpu);
    ((*ce_dev).event_handler)(ce_dev);
}

unsafe extern "C" fn timer_interrupt(irq: i32, devid: *mut c_void) -> irqreturn_t {
    let ce_dev = core::ptr::addr_of_mut!(hexagon_clockevent_dev);
    iowrite32(0, core::ptr::addr_of_mut!((*rtos_timer).enable));
    ((*ce_dev).event_handler)(ce_dev);
    IRQ_HANDLED
}

/*
 * time_init_deferred - called by start_kernel to set up timer/clock source
 *
 * Install the IRQ handler for the clock, setup timers.
 * This is done late, as that way, we can use ioremap().
 *
 * This runs just before the delay loop is calibrated, and
 * is used for delay calibration.
 */
unsafe extern "C" fn time_init_deferred() {
    let mut resource: *mut struct_resource = core::ptr::null_mut();
    let ce_dev = core::ptr::addr_of_mut!(hexagon_clockevent_dev);
    let flag = IRQF_TIMER | IRQF_TRIGGER_RISING;
    (*ce_dev).cpumask = cpu_all_mask;
    resource = rtos_timer_device.resource;
    /* ioremap here means this has to run later, after paging init */
    rtos_timer = ioremap((*resource).start, resource_size(resource));
    if rtos_timer.is_null() {
        release_mem_region((*resource).start, resource_size(resource));
    }
    clocksource_register_khz(core::ptr::addr_of_mut!(hexagon_clocksource), pcycle_freq_mhz * 1000);
    /* Note: the sim generic RTOS clock is apparently really 18750Hz */
    /* Last arg is some guaranteed seconds for which the conversion will work without overflow. */
    clockevents_calc_mult_shift(ce_dev, sleep_clk_freq, 4);
    (*ce_dev).max_delta_ns = clockevent_delta2ns(0x7fffffff, ce_dev);
    (*ce_dev).max_delta_ticks = 0x7fffffff;
    (*ce_dev).min_delta_ns = clockevent_delta2ns(0xf, ce_dev);
    (*ce_dev).min_delta_ticks = 0xf;
    #[cfg(CONFIG_SMP)]
    setup_percpu_clockdev();
    clockevents_register_device(ce_dev);
    if request_irq((*ce_dev).irq, Some(timer_interrupt), flag, "rtos_timer", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to register rtos_timer interrupt\n");
    }
}

unsafe extern "C" fn time_init() {
    late_time_init = Some(time_init_deferred);
}

unsafe extern "C" fn __delay(cycles: c_ulong) {
    let start = __vmgettime() as u64;
    while ((__vmgettime() as u64).wrapping_sub(start)) < cycles as u64 {
        cpu_relax();
    }
}

/* This could become parametric or perhaps even computed at run-time,
 * but for now we take the observed simulator jitter.
 */
static mut fudgefactor: i64 = 350;

unsafe extern "C" fn __udelay(usecs: c_ulong) {
    let start = __vmgettime() as u64;
    let finish = (pcycle_freq_mhz * usecs as u64) as i64 - fudgefactor;
    while ((__vmgettime() as u64).wrapping_sub(start)) < finish as u64 {
        cpu_relax(); /* not sure how this improves readability */
    }
}

unsafe extern "C" fn delay_read_timer(timer_val: *mut c_ulong) -> bool {
    *timer_val = __vmgettime() as c_ulong;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
