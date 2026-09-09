/*
 * arch/xtensa/kernel/time.c
 *
 * Timer and clock support.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 Tensilica Inc.
 *
 * Chris Zankel <chris@zankel.net>
 */

// Kernel and Xtensa declarations supplied by the surrounding build.

pub static mut ccount_freq: c_ulong = 0; /* ccount Hz */

unsafe extern "C" {
    fn get_ccount() -> c_ulong;
    fn set_linux_timer(value: c_ulong);
    fn get_linux_timer() -> c_ulong;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn disable_irq_nosync(irq: c_int);
    fn enable_irq(irq: c_int);
    fn irq_create_mapping(domain: *mut irq_domain, irq: c_uint) -> c_uint;
    fn cpumask_of(cpu: c_uint) -> *const cpumask;
    fn clockevents_config_and_register(
        evt: *mut clock_event_device,
        freq: c_ulong,
        min_delta: c_ulong,
        max_delta: c_ulong,
    );
    fn of_clk_init(data: *const c_void);
    fn platform_calibrate_ccount();
    fn clocksource_register_hz(cs: *mut clocksource, hz: c_ulong) -> c_int;
    fn request_irq(
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: c_uint, freq: c_ulong);
    fn timer_probe();
}

#[repr(C)]
pub struct clocksource {
    pub name: *const c_char,
    pub rating: c_int,
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u64,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct clock_event_device {
    pub features: c_ulong,
    pub rating: c_int,
    pub set_next_event: Option<unsafe extern "C" fn(c_ulong, *mut clock_event_device) -> c_int>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub name: *mut c_char,
    pub cpumask: *const cpumask,
    pub irq: c_uint,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

#[repr(C)]
pub struct ccount_timer {
    pub evt: clock_event_device,
    pub irq_enabled: c_int,
    pub name: [c_char; 24],
}

#[repr(C)]
pub struct irq_domain;
#[repr(C)]
pub struct cpumask;

pub type irqreturn_t = c_int;

unsafe extern "C" fn ccount_read(_cs: *mut clocksource) -> u64 {
    get_ccount() as u64
}

unsafe extern "C" fn ccount_sched_clock_read() -> u64 {
    get_ccount() as u64
}

static mut ccount_clocksource: clocksource = clocksource {
    name: b"ccount\0".as_ptr() as *const c_char,
    rating: 200,
    read: Some(ccount_read),
    mask: 0xffff_ffff,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

static mut ccount_timer: ccount_timer = ccount_timer {
    evt: clock_event_device {
        features: CLOCK_EVT_FEAT_ONESHOT,
        rating: 300,
        set_next_event: Some(ccount_timer_set_next_event),
        set_state_shutdown: Some(ccount_timer_shutdown),
        set_state_oneshot: Some(ccount_timer_set_oneshot),
        tick_resume: Some(ccount_timer_set_oneshot),
        name: core::ptr::null_mut(),
        cpumask: core::ptr::null(),
        irq: 0,
        event_handler: None,
    },
    irq_enabled: 0,
    name: [0; 24],
};

unsafe extern "C" fn ccount_timer_set_next_event(delta: c_ulong, _dev: *mut clock_event_device) -> c_int {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    let next = get_ccount().wrapping_add(delta);
    set_linux_timer(next);
    let ret = if next.wrapping_sub(get_ccount()) > delta { -ETIME } else { 0 };
    local_irq_restore(flags);
    ret
}

/*
 * There is no way to disable the timer interrupt at the device level,
 * only at the intenable register itself. Since enable_irq/disable_irq
 * calls are nested, we need to make sure that these calls are
 * balanced.
 */
unsafe extern "C" fn ccount_timer_shutdown(evt: *mut clock_event_device) -> c_int {
    let timer = &mut ccount_timer;
    if timer.irq_enabled != 0 {
        disable_irq_nosync((*evt).irq as c_int);
        timer.irq_enabled = 0;
    }
    0
}

unsafe extern "C" fn ccount_timer_set_oneshot(evt: *mut clock_event_device) -> c_int {
    let timer = &mut ccount_timer;
    if timer.irq_enabled == 0 {
        enable_irq((*evt).irq as c_int);
        timer.irq_enabled = 1;
    }
    0
}

unsafe extern "C" fn timer_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let evt = &mut ccount_timer.evt;
    set_linux_timer(get_linux_timer());
    if let Some(handler) = evt.event_handler {
        handler(evt);
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn local_timer_setup(cpu: c_uint) {
    let timer = &mut ccount_timer;
    let clockevent = &mut timer.evt;
    timer.irq_enabled = 1;
    // snprintf(timer->name, sizeof(timer->name), "ccount_clockevent_%u", cpu);
    clockevent.name = timer.name.as_mut_ptr();
    clockevent.cpumask = cpumask_of(cpu);
    clockevent.irq = irq_create_mapping(core::ptr::null_mut(), LINUX_TIMER_INT);
    if clockevent.irq == 0 {
        return;
    }
    clockevents_config_and_register(clockevent, ccount_freq, 0xf, 0xffff_ffff);
}

pub unsafe extern "C" fn time_init() {
    let mut irq: c_int;
    of_clk_init(core::ptr::null());
    ccount_freq = CONFIG_XTENSA_CPU_CLOCK as c_ulong * 1_000_000;
    clocksource_register_hz(&mut ccount_clocksource, ccount_freq);
    local_timer_setup(0);
    irq = ccount_timer.evt.irq as c_int;
    if request_irq(irq as c_uint, timer_interrupt, IRQF_TIMER, b"timer\0".as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
        // pr_err("Failed to request irq %d (timer)\n", irq);
    }
    sched_clock_register(ccount_sched_clock_read, 32, ccount_freq);
    timer_probe();
}

#[cfg(not(CONFIG_GENERIC_CALIBRATE_DELAY))]
pub unsafe extern "C" fn calibrate_delay() {
    loops_per_jiffy = ccount_freq / HZ as c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
