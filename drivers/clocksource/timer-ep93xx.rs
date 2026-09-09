// SPDX-License-Identifier: GPL-2.0
/*
 * Cirrus Logic EP93xx timer driver.
 * Copyright (C) 2021 Nikita Shubin <nikita.shubin@maquefel.me>
 *
 * Based on a rewrite of arch/arm/mach-ep93xx/timer.c:
 */

// Linux kernel dependencies supplied by other translation units.

const EP93XX_TIMER1_LOAD: usize = 0x00;
const EP93XX_TIMER1_VALUE: usize = 0x04;
const EP93XX_TIMER1_CONTROL: usize = 0x08;
const EP93XX_TIMER123_CONTROL_ENABLE: u32 = 1 << 7;
const EP93XX_TIMER123_CONTROL_MODE: u32 = 1 << 6;
const EP93XX_TIMER123_CONTROL_CLKSEL: u32 = 1 << 3;
const EP93XX_TIMER1_CLEAR: usize = 0x0c;
const EP93XX_TIMER2_LOAD: usize = 0x20;
const EP93XX_TIMER2_VALUE: usize = 0x24;
const EP93XX_TIMER2_CONTROL: usize = 0x28;
const EP93XX_TIMER2_CLEAR: usize = 0x2c;
/*
 * This read-only register contains the low word of the time stamp debug timer
 * ( Timer4). When this register is read, the high byte of the Timer4 counter is
 * saved in the Timer4ValueHigh register.
 */
const EP93XX_TIMER4_VALUE_LOW: usize = 0x60;
const EP93XX_TIMER4_VALUE_HIGH: usize = 0x64;
const EP93XX_TIMER4_VALUE_HIGH_ENABLE: u32 = 1 << 8;
const EP93XX_TIMER3_LOAD: usize = 0x80;
const EP93XX_TIMER3_VALUE: usize = 0x84;
const EP93XX_TIMER3_CONTROL: usize = 0x88;
const EP93XX_TIMER3_CLEAR: usize = 0x8c;

const EP93XX_TIMER123_RATE: u32 = 508469;
const EP93XX_TIMER4_RATE: u32 = 983040;

#[repr(C)]
pub struct ep93xx_tcu {
    pub base: *mut core::ffi::c_void,
}

static mut ep93xx_tcu: *mut ep93xx_tcu = core::ptr::null_mut();

extern "C" {
    fn lo_hi_readq(addr: *mut core::ffi::c_void) -> u64;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn kzalloc_obj<T>() -> *mut T;
    fn of_iomap(np: *mut device_node, index: u32) -> *mut core::ffi::c_void;
    fn irq_of_parse_and_map(np: *mut device_node, index: u32) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        flags: usize,
        name: *const core::ffi::c_char,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
    fn clocksource_mmio_init(
        addr: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        rating: u32,
        frequency: u32,
        bits: u32,
        read: unsafe extern "C" fn(*mut clocksource) -> u64,
    ) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn clockevents_config_and_register(
        dev: *mut clock_event_device,
        freq: u32,
        min_delta: u32,
        max_delta: u32,
    );
}

#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clocksource;
#[repr(C)] pub struct clock_event_device {
    pub name: *const core::ffi::c_char,
    pub features: u32,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub rating: i32,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}
pub type irqreturn_t = i32;

const CLOCK_EVT_FEAT_ONESHOT: u32 = 1;
const IRQF_TIMER: usize = 0x00000002;
const IRQF_IRQPOLL: usize = 0x00001000;
const IRQ_HANDLED: irqreturn_t = 1;
const ENOMEM: i32 = 12;
const ENXIO: i32 = 6;
const EINVAL: i32 = 22;
const UINT_MAX: u32 = u32::MAX;

unsafe extern "C" fn ep93xx_clocksource_read(_c: *mut clocksource) -> u64 {
    let tcu = ep93xx_tcu;
    lo_hi_readq((*tcu).base.add(EP93XX_TIMER4_VALUE_LOW)) & ((1u64 << 40) - 1)
}

unsafe extern "C" fn ep93xx_read_sched_clock() -> u64 {
    ep93xx_clocksource_read(core::ptr::null_mut())
}

unsafe extern "C" fn ep93xx_clkevt_set_next_event(
    next: usize,
    _evt: *mut clock_event_device,
) -> i32 {
    let tcu = ep93xx_tcu;
    /* Default mode: periodic, off, 508 kHz */
    let tmode = EP93XX_TIMER123_CONTROL_MODE | EP93XX_TIMER123_CONTROL_CLKSEL;
    writel(tmode, (*tcu).base.add(EP93XX_TIMER3_CONTROL));
    writel(next as u32, (*tcu).base.add(EP93XX_TIMER3_LOAD));
    writel(tmode | EP93XX_TIMER123_CONTROL_ENABLE, (*tcu).base.add(EP93XX_TIMER3_CONTROL));
    0
}

unsafe extern "C" fn ep93xx_clkevt_shutdown(_evt: *mut clock_event_device) -> i32 {
    let tcu = ep93xx_tcu;
    /* Disable timer */
    writel(0, (*tcu).base.add(EP93XX_TIMER3_CONTROL));
    0
}

static mut ep93xx_clockevent: clock_event_device = clock_event_device {
    name: b"timer1\0".as_ptr() as *const core::ffi::c_char,
    features: CLOCK_EVT_FEAT_ONESHOT,
    set_state_shutdown: Some(ep93xx_clkevt_shutdown),
    set_state_oneshot: Some(ep93xx_clkevt_shutdown),
    tick_resume: Some(ep93xx_clkevt_shutdown),
    set_next_event: Some(ep93xx_clkevt_set_next_event),
    rating: 300,
    event_handler: None,
};

unsafe extern "C" fn ep93xx_timer_interrupt(
    _irq: i32,
    dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
    let tcu = ep93xx_tcu;
    let evt = dev_id as *mut clock_event_device;
    writel(1, (*tcu).base.add(EP93XX_TIMER3_CLEAR));
    if let Some(handler) = (*evt).event_handler {
        handler(evt);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn ep93xx_timer_of_init(np: *mut device_node) -> i32 {
    let irq: i32;
    let flags: usize = IRQF_TIMER | IRQF_IRQPOLL;
    let tcu = kzalloc_obj::<ep93xx_tcu>();
    if tcu.is_null() { return -ENOMEM; }
    (*tcu).base = of_iomap(np, 0);
    if (*tcu).base.is_null() {
        kfree(tcu as *mut core::ffi::c_void);
        return -ENXIO;
    }
    ep93xx_tcu = tcu;
    irq = irq_of_parse_and_map(np, 0);
    if irq == 0 {
        kfree(tcu as *mut core::ffi::c_void);
        return -EINVAL;
    }
    writel(EP93XX_TIMER4_VALUE_HIGH_ENABLE, (*tcu).base.add(EP93XX_TIMER4_VALUE_HIGH));
    clocksource_mmio_init(core::ptr::null_mut(), b"timer4\0".as_ptr() as _, EP93XX_TIMER4_RATE, 200, 40, ep93xx_clocksource_read);
    sched_clock_register(ep93xx_read_sched_clock, 40, EP93XX_TIMER4_RATE);
    if request_irq(irq, ep93xx_timer_interrupt, flags, b"ep93xx timer\0".as_ptr() as _, &mut ep93xx_clockevent as *mut _ as _) != 0 {}
    clockevents_config_and_register(&mut ep93xx_clockevent, EP93XX_TIMER123_RATE, 1, UINT_MAX);
    0
}

// TIMER_OF_DECLARE(ep93xx_timer, "cirrus,ep9301-timer", ep93xx_timer_of_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
