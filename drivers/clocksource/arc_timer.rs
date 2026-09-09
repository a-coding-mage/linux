// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016-17 Synopsys, Inc. (www.synopsys.com)
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* ARC700 has two 32bit independent prog Timers: TIMER0 and TIMER1, Each can be
 * programmed to go from @count to @limit and optionally interrupt.
 * We've designated TIMER0 for clockevents and TIMER1 for clocksource
 *
 * ARCv2 based HS38 cores have RTC (in-core) and GFRC (inside ARConnect/MCIP)
 * which are suitable for UP and SMP based clocksources respectively
 */

static mut arc_timer_freq: c_ulong = 0;

unsafe fn arc_get_timer_clk(node: *mut device_node) -> c_int {
    let clk: *mut clk = of_clk_get(node, 0);
    if is_err(clk) {
        pr_err!("timer missing clk\n");
        return ptr_err(clk);
    }

    let ret = clk_prepare_enable(clk);
    if ret != 0 {
        pr_err!("Couldn't enable parent clk\n");
        return ret;
    }

    arc_timer_freq = clk_get_rate(clk);
    0
}

/* Clock Source Device */

/* Preserved from CONFIG_ARC_TIMERS_64BIT conditional compilation. */
#[cfg(CONFIG_ARC_TIMERS_64BIT)]
unsafe fn arc_read_gfrc(_cs: *mut clocksource) -> u64 {
    let mut flags: c_ulong = 0;
    let mut l: u32;
    let mut h: u32;

    local_irq_save(&mut flags);
    __mcip_cmd(CMD_GFRC_READ_LO, 0);
    l = read_aux_reg(ARC_REG_MCIP_READBACK);
    __mcip_cmd(CMD_GFRC_READ_HI, 0);
    h = read_aux_reg(ARC_REG_MCIP_READBACK);
    local_irq_restore(flags);
    ((h as u64) << 32) | l as u64
}

#[cfg(CONFIG_ARC_TIMERS_64BIT)]
unsafe fn arc_gfrc_clock_read() -> u64 { arc_read_gfrc(core::ptr::null_mut()) }

#[cfg(CONFIG_ARC_TIMERS_64BIT)]
static mut arc_counter_gfrc: clocksource = clocksource {
    name: "ARConnect GFRC",
    rating: 400,
    read: Some(arc_read_gfrc),
    mask: CLOCKSOURCE_MASK!(64),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

#[cfg(CONFIG_ARC_TIMERS_64BIT)]
unsafe fn arc_cs_setup_gfrc(node: *mut device_node) -> c_int {
    let mut mp: mcip_bcr = core::mem::zeroed();
    READ_BCR!(ARC_REG_MCIP_BCR, mp);
    if !mp.gfrc {
        pr_warn!("Global-64-bit-Ctr clocksource not detected\n");
        return -ENXIO;
    }
    let ret = arc_get_timer_clk(node);
    if ret != 0 { return ret; }
    sched_clock_register(Some(arc_gfrc_clock_read), 64, arc_timer_freq);
    clocksource_register_hz(&mut arc_counter_gfrc, arc_timer_freq)
}
// TIMER_OF_DECLARE(arc_gfrc, "snps,archs-timer-gfrc", arc_cs_setup_gfrc);

const AUX_RTC_CTRL: c_uint = 0x103;
const AUX_RTC_LOW: c_uint = 0x104;
const AUX_RTC_HIGH: c_uint = 0x105;

#[cfg(CONFIG_ARC_TIMERS_64BIT)]
unsafe fn arc_read_rtc(_cs: *mut clocksource) -> u64 {
    let mut status: c_ulong;
    let (mut l, mut h): (u32, u32);
    loop {
        l = read_aux_reg(AUX_RTC_LOW);
        h = read_aux_reg(AUX_RTC_HIGH);
        status = read_aux_reg(AUX_RTC_CTRL) as c_ulong;
        if (status & BIT!(31)) != 0 { break; }
    }
    ((h as u64) << 32) | l as u64
}

#[cfg(CONFIG_ARC_TIMERS_64BIT)]
unsafe fn arc_rtc_clock_read() -> u64 { arc_read_rtc(core::ptr::null_mut()) }

#[cfg(CONFIG_ARC_TIMERS_64BIT)]
static mut arc_counter_rtc: clocksource = clocksource {
    name: "ARCv2 RTC", rating: 350, read: Some(arc_read_rtc),
    mask: CLOCKSOURCE_MASK!(64), flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

#[cfg(CONFIG_ARC_TIMERS_64BIT)]
unsafe fn arc_cs_setup_rtc(node: *mut device_node) -> c_int {
    let mut timer: bcr_timer = core::mem::zeroed();
    let ret;
    READ_BCR!(ARC_REG_TIMERS_BCR, timer);
    if !timer.rtc { pr_warn!("Local-64-bit-Ctr clocksource not detected\n"); return -ENXIO; }
    /* Local to CPU hence not usable in SMP */
    if IS_ENABLED!(CONFIG_SMP) { pr_warn!("Local-64-bit-Ctr not usable in SMP\n"); return -EINVAL; }
    ret = arc_get_timer_clk(node);
    if ret != 0 { return ret; }
    write_aux_reg(AUX_RTC_CTRL, 1);
    sched_clock_register(Some(arc_rtc_clock_read), 64, arc_timer_freq);
    clocksource_register_hz(&mut arc_counter_rtc, arc_timer_freq)
}
// TIMER_OF_DECLARE(arc_rtc, "snps,archs-timer-rtc", arc_cs_setup_rtc);

unsafe fn arc_read_timer1(_cs: *mut clocksource) -> u64 { read_aux_reg(ARC_REG_TIMER1_CNT) as u64 }
unsafe fn arc_timer1_clock_read() -> u64 { arc_read_timer1(core::ptr::null_mut()) }

static mut arc_counter_timer1: clocksource = clocksource {
    name: "ARC Timer1", rating: 300, read: Some(arc_read_timer1),
    mask: CLOCKSOURCE_MASK!(32), flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe fn arc_cs_setup_timer1(node: *mut device_node) -> c_int {
    if IS_ENABLED!(CONFIG_SMP) { return -EINVAL; }
    let ret = arc_get_timer_clk(node);
    if ret != 0 { return ret; }
    write_aux_reg(ARC_REG_TIMER1_LIMIT, ARC_TIMERN_MAX);
    write_aux_reg(ARC_REG_TIMER1_CNT, 0);
    write_aux_reg(ARC_REG_TIMER1_CTRL, ARC_TIMER_CTRL_NH);
    sched_clock_register(Some(arc_timer1_clock_read), 32, arc_timer_freq);
    clocksource_register_hz(&mut arc_counter_timer1, arc_timer_freq)
}

static mut arc_timer_irq: c_int = 0;

unsafe fn arc_timer_event_setup(cycles: c_uint) {
    write_aux_reg(ARC_REG_TIMER0_LIMIT, cycles);
    write_aux_reg(ARC_REG_TIMER0_CNT, 0);
    write_aux_reg(ARC_REG_TIMER0_CTRL, ARC_TIMER_CTRL_IE | ARC_TIMER_CTRL_NH);
}

unsafe fn arc_clkevent_set_next_event(delta: c_ulong, _dev: *mut clock_event_device) -> c_int {
    arc_timer_event_setup(delta as c_uint); 0
}

unsafe fn arc_clkevent_set_periodic(_dev: *mut clock_event_device) -> c_int {
    arc_timer_event_setup(arc_timer_freq as c_uint / HZ); 0
}

static mut arc_clockevent_device: clock_event_device = clock_event_device {
    name: "ARC Timer0", features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC,
    rating: 300, set_next_event: Some(arc_clkevent_set_next_event),
    set_state_periodic: Some(arc_clkevent_set_periodic),
};

unsafe fn timer_irq_handler(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let evt = this_cpu_ptr(&mut arc_clockevent_device);
    let irq_reenable = clockevent_state_periodic(evt);
    write_aux_reg(ARC_REG_TIMER0_CTRL, irq_reenable | ARC_TIMER_CTRL_NH);
    ((*evt).event_handler.unwrap())(evt);
    IRQ_HANDLED
}

unsafe fn arc_timer_starting_cpu(_cpu: c_uint) -> c_int {
    let evt = this_cpu_ptr(&mut arc_clockevent_device);
    (*evt).cpumask = cpumask_of(smp_processor_id());
    clockevents_config_and_register(evt, arc_timer_freq, 0, ARC_TIMERN_MAX);
    enable_percpu_irq(arc_timer_irq, 0); 0
}

unsafe fn arc_timer_dying_cpu(_cpu: c_uint) -> c_int { disable_percpu_irq(arc_timer_irq); 0 }

unsafe fn arc_clockevent_setup(node: *mut device_node) -> c_int {
    let evt = this_cpu_ptr(&mut arc_clockevent_device);
    arc_timer_irq = irq_of_parse_and_map(node, 0);
    if arc_timer_irq <= 0 { pr_err!("clockevent: missing irq\n"); return -EINVAL; }
    let ret = arc_get_timer_clk(node);
    if ret != 0 { return ret; }
    let ret = request_percpu_irq(arc_timer_irq, Some(timer_irq_handler), "Timer0 (per-cpu-tick)", evt);
    if ret != 0 { pr_err!("clockevent: unable to request irq\n"); return ret; }
    let ret = cpuhp_setup_state(CPUHP_AP_ARC_TIMER_STARTING, "clockevents/arc/timer:starting", Some(arc_timer_starting_cpu), Some(arc_timer_dying_cpu));
    if ret != 0 { pr_err!("Failed to setup hotplug state\n"); return ret; }
    0
}

unsafe fn arc_of_timer_init(np: *mut device_node) -> c_int {
    static mut init_count: c_int = 0;
    if init_count == 0 { init_count = 1; arc_clockevent_setup(np) } else { arc_cs_setup_timer1(np) }
}
// TIMER_OF_DECLARE(arc_clkevt, "snps,arc-timer", arc_of_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
