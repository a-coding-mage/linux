// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1999, 2000, 05, 06 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */
// Linux and MIPS header dependencies are supplied by the surrounding crate.

const NSEC_PER_CYCLE: u64 = 800;
const CYCLES_PER_SEC: u64 = NSEC_PER_SEC / NSEC_PER_CYCLE;

unsafe fn rt_next_event(delta: c_ulong, _evt: *mut clock_event_device) -> c_int {
    let cpu: c_uint = smp_processor_id();
    let slice: c_int = cputoslice(cpu);
    let mut cnt: c_ulong;

    cnt = LOCAL_HUB_L(PI_RT_COUNT);
    cnt = cnt.wrapping_add(delta);
    LOCAL_HUB_S(PI_RT_COMPARE_A + PI_COUNT_OFFSET * slice, cnt);

    if LOCAL_HUB_L(PI_RT_COUNT) >= cnt { -ETIME } else { 0 }
}

// DEFINE_PER_CPU(struct clock_event_device, hub_rt_clockevent);
// DEFINE_PER_CPU(char [11], hub_rt_name);
extern "C" {
    static mut hub_rt_clockevent: clock_event_device;
    static mut hub_rt_name: [c_char; 11];
}

unsafe fn hub_rt_counter_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let _ = irq;
    let _ = dev_id;
    let cpu: c_uint = smp_processor_id();
    let cd: *mut clock_event_device = &mut hub_rt_clockevent;
    let slice: c_int = cputoslice(cpu);

    /*
     * Ack
     */
    LOCAL_HUB_S(PI_RT_PEND_A + PI_COUNT_OFFSET * slice, 0);
    ((*cd).event_handler)(cd);

    IRQ_HANDLED
}

unsafe fn hub_rt_clock_event_init() {
    let cpu: c_uint = smp_processor_id();
    let cd: *mut clock_event_device = &mut hub_rt_clockevent;
    let name: *mut c_char = hub_rt_name.as_mut_ptr();

    sprintf(name, b"hub-rt %d\0".as_ptr() as *const c_char, cpu);
    (*cd).name = name;
    (*cd).features = CLOCK_EVT_FEAT_ONESHOT;
    clockevent_set_clock(cd, CYCLES_PER_SEC as _);
    (*cd).max_delta_ns = clockevent_delta2ns(0xfffffffffffff, cd);
    (*cd).max_delta_ticks = 0xfffffffffffff;
    (*cd).min_delta_ns = clockevent_delta2ns(0x300, cd);
    (*cd).min_delta_ticks = 0x300;
    (*cd).rating = 200;
    (*cd).irq = IP27_RT_TIMER_IRQ;
    (*cd).cpumask = cpumask_of(cpu);
    (*cd).set_next_event = Some(rt_next_event);
    clockevents_register_device(cd);

    enable_percpu_irq(IP27_RT_TIMER_IRQ, IRQ_TYPE_NONE);
}

unsafe fn hub_rt_clock_event_global_init() {
    irq_set_handler(IP27_RT_TIMER_IRQ, handle_percpu_devid_irq);
    irq_set_percpu_devid(IP27_RT_TIMER_IRQ);
    WARN_ON(request_percpu_irq(
        IP27_RT_TIMER_IRQ,
        hub_rt_counter_handler,
        b"hub-rt\0".as_ptr() as *const c_char,
        &mut hub_rt_clockevent as *mut _ as *mut c_void,
    ));
}

unsafe fn hub_rt_read(_cs: *mut clocksource) -> u64 {
    REMOTE_HUB_L(cputonasid(0), PI_RT_COUNT)
}

#[no_mangle]
pub static mut hub_rt_clocksource: clocksource = clocksource {
    name: b"HUB-RT\0".as_ptr() as *const c_char,
    rating: 200,
    read: Some(hub_rt_read),
    mask: CLOCKSOURCE_MASK(52),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe fn hub_rt_read_sched_clock() -> u64 {
    REMOTE_HUB_L(cputonasid(0), PI_RT_COUNT)
}

unsafe fn hub_rt_clocksource_init() {
    let cs: *mut clocksource = &mut hub_rt_clocksource;

    clocksource_register_hz(cs, CYCLES_PER_SEC as _);
    sched_clock_register(Some(hub_rt_read_sched_clock), 52, CYCLES_PER_SEC as _);
}

#[no_mangle]
pub unsafe fn plat_time_init() {
    hub_rt_clocksource_init();
    hub_rt_clock_event_global_init();
    hub_rt_clock_event_init();
}

#[no_mangle]
pub unsafe fn hub_rtc_init(nasid: nasid_t) {
    /*
     * We only need to initialize the current node.
     * If this is not the current node then it is a cpuless
     * node and timeouts will not happen there.
     */
    if get_nasid() == nasid {
        LOCAL_HUB_S(PI_RT_EN_A, 1);
        LOCAL_HUB_S(PI_RT_EN_B, 1);
        LOCAL_HUB_S(PI_PROF_EN_A, 0);
        LOCAL_HUB_S(PI_PROF_EN_B, 0);
        LOCAL_HUB_S(PI_RT_COUNT, 0);
        LOCAL_HUB_S(PI_RT_PEND_A, 0);
        LOCAL_HUB_S(PI_RT_PEND_B, 0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
