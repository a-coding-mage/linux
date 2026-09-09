// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies supplied by the surrounding Linux/Rust environment.

const RTTM_DATA: usize = 0x0;
const RTTM_CNT: usize = 0x4;
const RTTM_CTRL: usize = 0x8;
const RTTM_INT: usize = 0xc;

const RTTM_CTRL_ENABLE: u32 = 1 << 28;
const RTTM_INT_PENDING: u32 = 1 << 16;
const RTTM_INT_ENABLE: u32 = 1 << 20;

/*
 * The Otto platform provides multiple 28 bit timers/counters with the following
 * operating logic. If enabled the timer counts up. Per timer one can set a
 * maximum counter value as an end marker. If end marker is reached the timer
 * fires an interrupt. If the timer "overflows" by reaching the end marker or
 * by adding 1 to 0x0fffffff the counter is reset to 0. When this happens and
 * the timer is in operating mode COUNTER it stops. In mode TIMER it will
 * continue to count up.
 */
const RTTM_CTRL_COUNTER: u32 = 0;
const RTTM_CTRL_TIMER: u32 = 1 << 24;
const RTTM_BIT_COUNT: u32 = 28;
const RTTM_MIN_DELTA: u32 = 8;
const RTTM_MAX_DELTA: u32 = (1 << 28) - 1;
const RTTM_MAX_DIVISOR: u32 = (1 << 16) - 1;
const RTTM_TICKS_PER_SEC: u32 = 3_125_000;

#[repr(C)]
pub struct RttmCs {
    pub to: timer_of,
    pub cs: clocksource,
}

#[inline]
unsafe fn rttm_get_counter(base: *mut core::ffi::c_void) -> u32 {
    core::ptr::read_volatile((base as *mut u8).add(RTTM_CNT) as *const u32)
}

#[inline]
unsafe fn rttm_set_period(base: *mut core::ffi::c_void, period: u32) {
    core::ptr::write_volatile((base as *mut u8).add(RTTM_DATA) as *mut u32, period);
}

#[inline]
unsafe fn rttm_disable_timer(base: *mut core::ffi::c_void) {
    rttm_set_reg(base, RTTM_CTRL, 0);
}

#[inline]
unsafe fn rttm_enable_timer(base: *mut core::ffi::c_void, mode: u32, divisor: u32) {
    rttm_set_reg(base, RTTM_CTRL, RTTM_CTRL_ENABLE | mode | divisor);
}

#[inline]
unsafe fn rttm_ack_irq(base: *mut core::ffi::c_void) {
    let value = rttm_get_reg(base, RTTM_INT);
    rttm_set_reg(base, RTTM_INT, value | RTTM_INT_PENDING);
}

#[inline]
unsafe fn rttm_enable_irq(base: *mut core::ffi::c_void) { rttm_set_reg(base, RTTM_INT, RTTM_INT_ENABLE); }
#[inline]
unsafe fn rttm_disable_irq(base: *mut core::ffi::c_void) { rttm_set_reg(base, RTTM_INT, 0); }

#[inline]
unsafe fn rttm_get_reg(base: *mut core::ffi::c_void, offset: usize) -> u32 {
    core::ptr::read_volatile((base as *mut u8).add(offset) as *const u32)
}
#[inline]
unsafe fn rttm_set_reg(base: *mut core::ffi::c_void, offset: usize, value: u32) {
    core::ptr::write_volatile((base as *mut u8).add(offset) as *mut u32, value);
}

unsafe fn rttm_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let clkevt = dev_id as *mut clock_event_device;
    let to = to_timer_of(clkevt);
    rttm_ack_irq((*to).of_base.base);
    ((*clkevt).event_handler)(clkevt);
    IRQ_HANDLED
}

unsafe fn rttm_bounce_timer(base: *mut core::ffi::c_void, mode: u32) {
    rttm_disable_timer(base);
    rttm_enable_timer(base, mode, RTTM_MAX_DIVISOR);
}
unsafe fn rttm_stop_timer(base: *mut core::ffi::c_void) { rttm_disable_timer(base); rttm_ack_irq(base); }
unsafe fn rttm_start_timer(to: *mut timer_of, mode: u32) {
    rttm_enable_timer((*to).of_base.base, mode, (*to).of_clk.rate / RTTM_TICKS_PER_SEC);
}

unsafe fn rttm_next_event(delta: usize, clkevt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clkevt);
    rttm_bounce_timer((*to).of_base.base, RTTM_CTRL_COUNTER);
    rttm_disable_timer((*to).of_base.base);
    rttm_set_period((*to).of_base.base, delta as u32);
    rttm_start_timer(to, RTTM_CTRL_COUNTER); 0
}
unsafe fn rttm_state_oneshot(clkevt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clkevt);
    rttm_bounce_timer((*to).of_base.base, RTTM_CTRL_COUNTER);
    rttm_disable_timer((*to).of_base.base);
    rttm_set_period((*to).of_base.base, RTTM_TICKS_PER_SEC / HZ);
    rttm_start_timer(to, RTTM_CTRL_COUNTER); 0
}
unsafe fn rttm_state_periodic(clkevt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clkevt);
    rttm_bounce_timer((*to).of_base.base, RTTM_CTRL_TIMER);
    rttm_disable_timer((*to).of_base.base);
    rttm_set_period((*to).of_base.base, RTTM_TICKS_PER_SEC / HZ);
    rttm_start_timer(to, RTTM_CTRL_TIMER); 0
}
unsafe fn rttm_state_shutdown(clkevt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clkevt); rttm_stop_timer((*to).of_base.base); 0
}
unsafe fn rttm_setup_timer(base: *mut core::ffi::c_void) { rttm_stop_timer(base); rttm_set_period(base, 0); }

unsafe fn rttm_read_clocksource(cs: *mut clocksource) -> u64 {
    let rcs = container_of!(cs, RttmCs, cs); rttm_get_counter((*rcs).to.of_base.base) as u64
}

unsafe fn rttm_enable_clocksource(cs: *mut clocksource) -> i32 {
    let rcs = container_of!(cs, RttmCs, cs);
    rttm_disable_irq((*rcs).to.of_base.base);
    rttm_setup_timer((*rcs).to.of_base.base);
    rttm_enable_timer((*rcs).to.of_base.base, RTTM_CTRL_TIMER, (*rcs).to.of_clk.rate / RTTM_TICKS_PER_SEC);
    0
}

unsafe fn rttm_read_clock() -> u64 { rttm_get_counter(RTTM_CS.to.of_base.base) as u64 }

// Remaining platform registration and per-CPU initialization are supplied by
// the kernel framework declarations used by this implementation.
extern "C" {
    static mut RTTM_CS: RttmCs;
    fn to_timer_of(clkevt: *mut clock_event_device) -> *mut timer_of;
    static mut RTTM_TO: timer_of;
}

unsafe fn rttm_cpu_starting(cpu: u32) -> i32 {
    let to = per_cpu_ptr(&mut RTTM_TO, cpu);
    (*to).clkevt.cpumask = cpumask_of(cpu);
    irq_force_affinity((*to).of_irq.irq, (*to).clkevt.cpumask);
    clockevents_config_and_register(&mut (*to).clkevt, RTTM_TICKS_PER_SEC,
                                     RTTM_MIN_DELTA, RTTM_MAX_DELTA);
    rttm_enable_irq((*to).of_base.base);
    0
}

unsafe fn rttm_probe(np: *mut device_node) -> i32 {
    let mut cpu: u32 = 0;
    let clkidx = num_possible_cpus();
    for_each_possible_cpu!(cpu) {
        let to = per_cpu_ptr(&mut RTTM_TO, cpu);
        (*to).of_irq.index = cpu;
        (*to).of_base.index = cpu;
        if timer_of_init(np, to) != 0 {
            pr_err!("setup of timer {} failed", cpu);
            let mut cpu_rollback: u32 = 0;
            for_each_possible_cpu!(cpu_rollback) {
                if cpu_rollback == cpu { break; }
                timer_of_cleanup(per_cpu_ptr(&mut RTTM_TO, cpu_rollback));
            }
            return -22;
        }
        rttm_setup_timer((*to).of_base.base);
    }
    let to = &mut RTTM_CS.to;
    to.of_base.index = clkidx;
    timer_of_init(np, to);
    if !(*to).of_base.base.is_null() && (*to).of_clk.rate != 0 {
        rttm_enable_clocksource(&mut RTTM_CS.cs);
        clocksource_register_hz(&mut RTTM_CS.cs, RTTM_TICKS_PER_SEC);
        sched_clock_register(rttm_read_clock, RTTM_BIT_COUNT, RTTM_TICKS_PER_SEC);
    } else {
        pr_err!(" setup of timer {} as clocksource failed", clkidx);
    }
    cpuhp_setup_state(CPUHP_AP_REALTEK_TIMER_STARTING,
                      "timer/realtek:online", rttm_cpu_starting, None)
}

// Equivalent of TIMER_OF_DECLARE(otto_timer, "realtek,otto-timer", rttm_probe).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
