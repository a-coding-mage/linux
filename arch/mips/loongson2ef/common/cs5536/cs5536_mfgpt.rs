// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CS5536 General timer functions
 *
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Yanhua, yanh@lemote.com
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu zhangjin, wuzhangjin@gmail.com
 *
 * Reference: AMD Geode(TM) CS5536 Companion Device Data Book
 */

// Linux and architecture headers from the original source provide the
// declarations, constants, and macros referenced below.

static mut mfgpt_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK();

static mut mfgpt_base: u32 = 0;

/*
 * Initialize the MFGPT timer.
 *
 * This is also called after resume to bring the MFGPT into operation again.
 */

/* disable counter */
pub unsafe extern "C" fn disable_mfgpt0_counter() {
    outw(inw(MFGPT0_SETUP) & 0x7fff, MFGPT0_SETUP);
}

/* enable counter, comparator2 to event mode, 14.318MHz clock */
pub unsafe extern "C" fn enable_mfgpt0_counter() {
    outw(0xe310, MFGPT0_SETUP);
}

unsafe extern "C" fn mfgpt_timer_set_periodic(
    _evt: *mut clock_event_device,
) -> i32 {
    raw_spin_lock(&mut mfgpt_lock);

    outw(COMPARE, MFGPT0_CMP2); // set comparator2
    outw(0, MFGPT0_CNT); // set counter to 0
    enable_mfgpt0_counter();

    raw_spin_unlock(&mut mfgpt_lock);
    0
}

unsafe extern "C" fn mfgpt_timer_shutdown(evt: *mut clock_event_device) -> i32 {
    if clockevent_state_periodic(evt) || clockevent_state_oneshot(evt) {
        raw_spin_lock(&mut mfgpt_lock);
        disable_mfgpt0_counter();
        raw_spin_unlock(&mut mfgpt_lock);
    }

    0
}

static mut mfgpt_clockevent: clock_event_device = clock_event_device {
    name: "mfgpt",
    features: CLOCK_EVT_FEAT_PERIODIC,
    // The oneshot mode have very high deviation, don't use it!
    set_state_shutdown: Some(mfgpt_timer_shutdown),
    set_state_periodic: Some(mfgpt_timer_set_periodic),
    irq: CS5536_MFGPT_INTR,
};

unsafe extern "C" fn timer_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut basehi: u32 = 0;

    /*
     * get MFGPT base address
     *
     * NOTE: do not remove me, it's need for the value of mfgpt_base is
     * variable
     */
    _rdmsr(DIVIL_MSR_REG(DIVIL_LBAR_MFGPT), &mut basehi, &mut mfgpt_base);

    /* ack */
    outw(inw(MFGPT0_SETUP) | 0x4000, MFGPT0_SETUP);

    if let Some(handler) = mfgpt_clockevent.event_handler {
        handler(&mut mfgpt_clockevent);
    }

    IRQ_HANDLED
}

/*
 * Initialize the conversion factor and the min/max deltas of the clock event
 * structure and register the clock event source with the framework.
 */
pub unsafe extern "C" fn setup_mfgpt0_timer() {
    let mut basehi: u32 = 0;
    let cd: *mut clock_event_device = &mut mfgpt_clockevent;
    let cpu: u32 = smp_processor_id();

    (*cd).cpumask = cpumask_of(cpu);
    clockevent_set_clock(cd, MFGPT_TICK_RATE);
    (*cd).max_delta_ns = clockevent_delta2ns(0xffff, cd);
    (*cd).max_delta_ticks = 0xffff;
    (*cd).min_delta_ns = clockevent_delta2ns(0xf, cd);
    (*cd).min_delta_ticks = 0xf;

    /* Enable MFGPT0 Comparator 2 Output to the Interrupt Mapper */
    _wrmsr(DIVIL_MSR_REG(MFGPT_IRQ), 0, 0x100);

    /* Enable Interrupt Gate 5 */
    _wrmsr(DIVIL_MSR_REG(PIC_ZSEL_LOW), 0, 0x50000);

    /* get MFGPT base address */
    _rdmsr(DIVIL_MSR_REG(DIVIL_LBAR_MFGPT), &mut basehi, &mut mfgpt_base);

    clockevents_register_device(cd);

    if request_irq(
        CS5536_MFGPT_INTR,
        Some(timer_interrupt),
        IRQF_NOBALANCING | IRQF_TIMER,
        "timer",
        core::ptr::null_mut(),
    ) != 0 {
        pr_err("Failed to register timer interrupt\n");
    }
}

/*
 * Since the MFGPT overflows every tick, its not very useful
 * to just read by itself. So use jiffies to emulate a free
 * running counter:
 */
unsafe extern "C" fn mfgpt_read(_cs: *mut clocksource) -> u64 {
    let mut flags: unsigned_long = 0;
    let mut count: i32;
    let jifs: u32;
    static mut old_count: i32 = 0;
    static mut old_jifs: u32 = 0;

    raw_spin_lock_irqsave(&mut mfgpt_lock, &mut flags);
    /* See the original source for the seqlock and volatile-jiffies rationale. */
    jifs = jiffies;
    count = inw(MFGPT0_CNT) as i32;

    if count < old_count && jifs == old_jifs {
        count = old_count;
    }

    old_count = count;
    old_jifs = jifs;

    raw_spin_unlock_irqrestore(&mut mfgpt_lock, flags);

    (jifs as u64).wrapping_mul(COMPARE as u64).wrapping_add(count as u64)
}

static mut clocksource_mfgpt: clocksource = clocksource {
    name: "mfgpt",
    rating: 120, // Functional for real use, but not desired
    read: Some(mfgpt_read),
    mask: CLOCKSOURCE_MASK(32),
};

pub unsafe extern "C" fn init_mfgpt_clocksource() -> i32 {
    if num_possible_cpus() > 1 { // MFGPT does not scale!
        return 0;
    }

    clocksource_register_hz(&mut clocksource_mfgpt, MFGPT_TICK_RATE)
}

// arch_initcall(init_mfgpt_clocksource);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
