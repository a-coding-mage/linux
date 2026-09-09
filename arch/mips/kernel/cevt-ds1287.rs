// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  DS1287 clockevent driver
 *
 *  Copyright (C) 2008	Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

extern "C" {
    static mut rtc_lock: c_void;
    fn CMOS_READ(reg: u8) -> u8;
    fn CMOS_WRITE(value: u8, reg: u8);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn clockevent_set_clock(cd: *mut clock_event_device, hz: u32);
    fn clockevent_delta2ns(delta: u32, cd: *const clock_event_device) -> u64;
    fn cpumask_of(cpu: u32) -> *const c_void;
    fn clockevents_register_device(cd: *mut clock_event_device);
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const u8,
        dev_id: *mut c_void,
    ) -> i32;
}

type c_ulong = usize;
type irqreturn_t = i32;

const RTC_REG_A: u8 = 0x0a;
const RTC_REG_B: u8 = 0x0b;
const RTC_REG_C: u8 = 0x0c;
const RTC_PF: u8 = 0x40;
const RTC_PIE: u8 = 0x40;
const RTC_REF_CLCK_32KHZ: u8 = 0x20;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 0x0000_0002;
const IRQF_PERCPU: c_ulong = 0x0000_0080;
const IRQF_TIMER: c_ulong = 0x0000_0100;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: i32 = 22;

#[repr(C)]
pub struct clock_event_device {
    pub name: *const u8,
    pub features: u32,
    pub rating: i32,
    pub irq: i32,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
    pub max_delta_ns: u64,
    pub max_delta_ticks: u32,
    pub min_delta_ns: u64,
    pub min_delta_ticks: u32,
    pub cpumask: *const c_void,
}

pub unsafe extern "C" fn ds1287_timer_state() -> i32 {
    (CMOS_READ(RTC_REG_C) & RTC_PF != 0) as i32
}

pub unsafe extern "C" fn ds1287_set_base_clock(hz: u32) -> i32 {
    let rate: u8;

    match hz {
        128 => rate = 0x9,
        256 => rate = 0x8,
        1024 => rate = 0x6,
        _ => return -EINVAL,
    }

    CMOS_WRITE(RTC_REF_CLCK_32KHZ | rate, RTC_REG_A);

    0
}

unsafe extern "C" fn ds1287_set_next_event(
    _delta: usize,
    _evt: *mut clock_event_device,
) -> i32 {
    -EINVAL
}

unsafe extern "C" fn ds1287_shutdown(_evt: *mut clock_event_device) -> i32 {
    let val: u8;

    spin_lock(&raw mut rtc_lock);

    val = CMOS_READ(RTC_REG_B) & !RTC_PIE;
    CMOS_WRITE(val, RTC_REG_B);

    spin_unlock(&raw mut rtc_lock);
    0
}

unsafe extern "C" fn ds1287_set_periodic(_evt: *mut clock_event_device) -> i32 {
    let val: u8;

    spin_lock(&raw mut rtc_lock);

    val = CMOS_READ(RTC_REG_B) | RTC_PIE;
    CMOS_WRITE(val, RTC_REG_B);

    spin_unlock(&raw mut rtc_lock);
    0
}

unsafe extern "C" fn ds1287_event_handler(_dev: *mut clock_event_device) {}

static mut ds1287_clockevent: clock_event_device = clock_event_device {
    name: b"ds1287\0".as_ptr(),
    features: CLOCK_EVT_FEAT_PERIODIC,
    rating: 0,
    irq: 0,
    set_next_event: Some(ds1287_set_next_event),
    set_state_shutdown: Some(ds1287_shutdown),
    set_state_periodic: Some(ds1287_set_periodic),
    tick_resume: Some(ds1287_shutdown),
    event_handler: Some(ds1287_event_handler),
    max_delta_ns: 0,
    max_delta_ticks: 0,
    min_delta_ns: 0,
    min_delta_ticks: 0,
    cpumask: core::ptr::null(),
};

unsafe extern "C" fn ds1287_interrupt(_irq: i32, _dev_id: *mut c_void) -> irqreturn_t {
    let cd: *mut clock_event_device = &raw mut ds1287_clockevent;

    /* Ack the RTC interrupt. */
    CMOS_READ(RTC_REG_C);

    if let Some(event_handler) = (*cd).event_handler {
        event_handler(cd);
    }

    IRQ_HANDLED
}

pub unsafe extern "C" fn ds1287_clockevent_init(irq: i32) -> i32 {
    let flags: c_ulong = IRQF_PERCPU | IRQF_TIMER;
    let cd: *mut clock_event_device;

    cd = &raw mut ds1287_clockevent;
    (*cd).rating = 100;
    (*cd).irq = irq;
    clockevent_set_clock(cd, 32768);
    (*cd).max_delta_ns = clockevent_delta2ns(0x7fffffff, cd);
    (*cd).max_delta_ticks = 0x7fffffff;
    (*cd).min_delta_ns = clockevent_delta2ns(0x300, cd);
    (*cd).min_delta_ticks = 0x300;
    (*cd).cpumask = cpumask_of(0);

    clockevents_register_device(cd);

    request_irq(irq, ds1287_interrupt, flags, b"ds1287\0".as_ptr(), core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
