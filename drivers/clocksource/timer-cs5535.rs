// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clock event driver for the CS5535/CS5536
 *
 * Copyright (C) 2006, Advanced Micro Devices, Inc.
 * Copyright (C) 2007  Andres Salomon <dilinger@debian.org>
 * Copyright (C) 2009  Andres Salomon <dilinger@collabora.co.uk>
 *
 * The MFGPTs are documented in AMD Geode CS5536 Companion Device Data Book.
 */

// Linux kernel dependencies supplied by other translation units.

const DRV_NAME: &str = "cs5535-clockevt";

static mut timer_irq: i32 = 0;

/*
 * We are using the 32.768kHz input clock - it's the only one that has the
 * ranges we find desirable.  The following table lists the suitable
 * divisors and the associated Hz, minimum interval and the maximum interval:
 *
 *  Divisor   Hz      Min Delta (s)  Max Delta (s)
 *   1        32768   .00048828125      2.000
 *   2        16384   .0009765625       4.000
 *   4         8192   .001953125        8.000
 *   8         4096   .00390625        16.000
 *  16        2048   .0078125         32.000
 *  32        1024   .015625          64.000
 *  64         512   .03125          128.000
 *  128         256   .0625           256.000
 *  256         128   .125            512.000
 */

#[repr(C)]
pub struct cs5535_mfgpt_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clock_event_device {
    pub name: *const u8,
    pub features: u32,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub rating: i32,
    pub owner: *mut (),
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

extern "C" {
    fn cs5535_mfgpt_write(timer: *mut cs5535_mfgpt_timer, reg: u32, value: u16);
    fn cs5535_mfgpt_read(timer: *mut cs5535_mfgpt_timer, reg: u32) -> u16;
    fn cs5535_mfgpt_alloc_timer(which: u32, domain: u32) -> *mut cs5535_mfgpt_timer;
    fn cs5535_mfgpt_setup_irq(timer: *mut cs5535_mfgpt_timer, cmp: u32, irq: *mut i32) -> i32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut ()), flags: usize, name: *const u8, dev_id: *mut ()) -> i32;
    fn cs5535_mfgpt_release_irq(timer: *mut cs5535_mfgpt_timer, cmp: u32, irq: *mut i32);
    fn cs5535_mfgpt_free_timer(timer: *mut cs5535_mfgpt_timer);
    fn clockevents_config_and_register(dev: *mut clock_event_device, hz: u32, min_delta: u32, max_delta: u32);
    fn clockevent_state_detached(dev: *const clock_event_device) -> bool;
    fn clockevent_state_shutdown(dev: *const clock_event_device) -> bool;
    fn clockevent_state_periodic(dev: *const clock_event_device) -> bool;
    fn printk(fmt: *const u8, ...);
}

const MFGPT_REG_SETUP: u32 = 0;
const MFGPT_REG_CMP2: u32 = 2;
const MFGPT_REG_COUNTER: u32 = 3;
const MFGPT_SETUP_CNTEN: u16 = 1;
const MFGPT_SETUP_CMP1: u16 = 1 << 4;
const MFGPT_SETUP_CMP2: u16 = 1 << 5;
const MFGPT_SETUP_SETUP: u16 = 1 << 6;
const MFGPT_TIMER_ANY: u32 = 0;
const MFGPT_DOMAIN_WORKING: u32 = 0;
const MFGPT_CMP2: u32 = 0;
const IRQ_NONE: i32 = 0;
const IRQ_HANDLED: i32 = 1;
const IRQF_NOBALANCING: usize = 0;
const IRQF_TIMER: usize = 0;
const IRQF_SHARED: usize = 0;
const HZ: u32 = 100;
const EIO: i32 = 5;
const ENODEV: i32 = 19;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 2;

const MFGPT_DIVISOR: u32 = 16;
const MFGPT_SCALE: u16 = 4;
const MFGPT_HZ: u32 = 32768 / MFGPT_DIVISOR;
const MFGPT_PERIODIC: u32 = MFGPT_HZ / HZ;

static mut cs5535_event_clock: *mut cs5535_mfgpt_timer = core::ptr::null_mut();

unsafe fn disable_timer(timer: *mut cs5535_mfgpt_timer) {
    /* avoid races by clearing CMP1 and CMP2 unconditionally */
    cs5535_mfgpt_write(timer, MFGPT_REG_SETUP,
        (!MFGPT_SETUP_CNTEN) | MFGPT_SETUP_CMP1 | MFGPT_SETUP_CMP2);
}

unsafe fn start_timer(timer: *mut cs5535_mfgpt_timer, delta: u16) {
    cs5535_mfgpt_write(timer, MFGPT_REG_CMP2, delta);
    cs5535_mfgpt_write(timer, MFGPT_REG_COUNTER, 0);
    cs5535_mfgpt_write(timer, MFGPT_REG_SETUP, MFGPT_SETUP_CNTEN | MFGPT_SETUP_CMP2);
}

unsafe extern "C" fn mfgpt_shutdown(_evt: *mut clock_event_device) -> i32 {
    disable_timer(cs5535_event_clock); 0
}

unsafe extern "C" fn mfgpt_set_periodic(_evt: *mut clock_event_device) -> i32 {
    disable_timer(cs5535_event_clock);
    start_timer(cs5535_event_clock, MFGPT_PERIODIC as u16); 0
}

unsafe extern "C" fn mfgpt_next_event(delta: usize, _evt: *mut clock_event_device) -> i32 {
    start_timer(cs5535_event_clock, delta as u16); 0
}

static mut cs5535_clockevent: clock_event_device = clock_event_device {
    name: DRV_NAME.as_ptr(), features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    set_state_shutdown: Some(mfgpt_shutdown), set_state_periodic: Some(mfgpt_set_periodic),
    set_state_oneshot: Some(mfgpt_shutdown), tick_resume: Some(mfgpt_shutdown),
    set_next_event: Some(mfgpt_next_event), rating: 250, owner: core::ptr::null_mut(),
    event_handler: None,
};

unsafe extern "C" fn mfgpt_tick(_irq: i32, _dev_id: *mut ()) -> i32 {
    let val = cs5535_mfgpt_read(cs5535_event_clock, MFGPT_REG_SETUP);
    if (val & (MFGPT_SETUP_SETUP | MFGPT_SETUP_CMP2 | MFGPT_SETUP_CMP1)) == 0 { return IRQ_NONE; }
    disable_timer(cs5535_event_clock);
    if clockevent_state_detached(&cs5535_clockevent) || clockevent_state_shutdown(&cs5535_clockevent) { return IRQ_HANDLED; }
    cs5535_mfgpt_write(cs5535_event_clock, MFGPT_REG_COUNTER, 0);
    if clockevent_state_periodic(&cs5535_clockevent) { cs5535_mfgpt_write(cs5535_event_clock, MFGPT_REG_SETUP, MFGPT_SETUP_CNTEN | MFGPT_SETUP_CMP2); }
    if let Some(handler) = cs5535_clockevent.event_handler { handler(&mut cs5535_clockevent); }
    IRQ_HANDLED
}

unsafe extern "C" fn cs5535_mfgpt_init() -> i32 {
    let flags = IRQF_NOBALANCING | IRQF_TIMER | IRQF_SHARED;
    let timer = cs5535_mfgpt_alloc_timer(MFGPT_TIMER_ANY, MFGPT_DOMAIN_WORKING);
    if timer.is_null() { return -ENODEV; }
    cs5535_event_clock = timer;
    if cs5535_mfgpt_setup_irq(timer, MFGPT_CMP2, &mut timer_irq) != 0 { goto_err_timer(); return -ENODEV; }
    if request_irq(timer_irq, mfgpt_tick, flags, DRV_NAME.as_ptr(), timer as *mut ()) != 0 { goto_err_irq(); return -EIO; }
    cs5535_mfgpt_write(cs5535_event_clock, MFGPT_REG_SETUP, MFGPT_SCALE | (3 << 8));
    clockevents_config_and_register(&mut cs5535_clockevent, MFGPT_HZ, 0xF, 0xFFFE);
    0
}

unsafe fn goto_err_irq() { cs5535_mfgpt_release_irq(cs5535_event_clock, MFGPT_CMP2, &mut timer_irq); cs5535_mfgpt_free_timer(cs5535_event_clock); }
unsafe fn goto_err_timer() { cs5535_mfgpt_free_timer(cs5535_event_clock); }

// module_param_hw_named(irq, timer_irq, int, irq, 0644)
// MODULE_PARM_DESC(irq, "Which IRQ to use for the clock source MFGPT ticks.");
// module_init(cs5535_mfgpt_init)
// MODULE_AUTHOR("Andres Salomon <dilinger@queued.net>");
// MODULE_DESCRIPTION("CS5535/CS5536 MFGPT clock event driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
