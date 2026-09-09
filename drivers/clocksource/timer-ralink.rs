// SPDX-License-Identifier: GPL-2.0
/*
 * Ralink System Tick Counter driver present on RT3352 and MT7620 SoCs.
 *
 * Copyright (C) 2013 by John Crispin <john@phrozen.org>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const SYSTICK_FREQ: u32 = 50 * 1000;

const SYSTICK_CONFIG: usize = 0x00;
const SYSTICK_COMPARE: usize = 0x04;
const SYSTICK_COUNT: usize = 0x08;

/* route systick irq to mips irq 7 instead of the r4k-timer */
const CFG_EXT_STK_EN: u32 = 0x2;
/* enable the counter */
const CFG_CNT_EN: u32 = 0x1;

#[repr(C)]
struct ClockEventDevice {
    name: *const core::ffi::c_char,
    irq: i32,
    rating: i32,
    features: u32,
    set_next_event: Option<unsafe extern "C" fn(usize, *mut ClockEventDevice) -> i32>,
    set_state_shutdown: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>,
    set_state_oneshot: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>,
    event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)>,
    mult: u32,
    shift: u32,
    max_delta_ns: u64,
    max_delta_ticks: u32,
    min_delta_ns: u64,
    min_delta_ticks: u32,
}

#[repr(C)]
struct SystickDevice {
    membase: *mut u8,
    dev: ClockEventDevice,
    irq_requested: i32,
    freq_scale: i32,
}

unsafe extern "C" {
    fn ioread32(addr: *mut u8) -> u32;
    fn iowrite32(value: u32, addr: *mut u8);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
                   flags: u32, name: *const core::ffi::c_char, dev_id: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: i32, dev_id: *mut core::ffi::c_void);
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn irq_of_parse_and_map(np: *mut DeviceNode, index: i32) -> i32;
    fn irq_dispose_mapping(irq: i32);
    fn clockevents_calc_mult_shift(dev: *mut ClockEventDevice, freq: u32, maxsec: u32);
    fn clockevent_delta2ns(delta: u32, dev: *const ClockEventDevice) -> u64;
    fn clocksource_mmio_init(base: *mut u8, name: *const core::ffi::c_char,
                             freq: u32, rating: u32, bits: u32, read: *const core::ffi::c_void) -> i32;
    fn clockevents_register_device(dev: *mut ClockEventDevice);
}

#[repr(C)]
struct DeviceNode {
    name: *const core::ffi::c_char,
}

const IRQ_HANDLED: i32 = 1;
const IRQF_PERCPU: u32 = 0;
const IRQF_TIMER: u32 = 0;

unsafe extern "C" fn systick_next_event(delta: usize, evt: *mut ClockEventDevice) -> i32 {
    let sdev = (evt as *mut u8).sub(core::mem::offset_of!(SystickDevice, dev)) as *mut SystickDevice;
    let mut count = ioread32((*sdev).membase.add(SYSTICK_COUNT));
    count = (count.wrapping_add(delta as u32)) % SYSTICK_FREQ;
    iowrite32(count, (*sdev).membase.add(SYSTICK_COMPARE));
    0
}

unsafe extern "C" fn systick_event_handler(_dev: *mut ClockEventDevice) {
    /* noting to do here */
}

unsafe extern "C" fn systick_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let dev = dev_id as *mut ClockEventDevice;
    if let Some(handler) = (*dev).event_handler {
        handler(dev);
    }
    IRQ_HANDLED
}

static mut SYSTICK: SystickDevice = SystickDevice {
    membase: core::ptr::null_mut(),
    dev: ClockEventDevice {
        name: core::ptr::null(), irq: 0, rating: 310, features: 1,
        set_next_event: Some(systick_next_event),
        set_state_shutdown: Some(systick_shutdown),
        set_state_oneshot: Some(systick_set_oneshot),
        event_handler: Some(systick_event_handler),
        mult: 0, shift: 0, max_delta_ns: 0, max_delta_ticks: 0,
        min_delta_ns: 0, min_delta_ticks: 0,
    },
    irq_requested: 0,
    freq_scale: 0,
};

unsafe extern "C" fn systick_shutdown(evt: *mut ClockEventDevice) -> i32 {
    let sdev = (evt as *mut u8).sub(core::mem::offset_of!(SystickDevice, dev)) as *mut SystickDevice;
    if (*sdev).irq_requested != 0 {
        free_irq(SYSTICK.dev.irq, &raw mut SYSTICK.dev as *mut _ as *mut core::ffi::c_void);
    }
    (*sdev).irq_requested = 0;
    iowrite32(0, SYSTICK.membase.add(SYSTICK_CONFIG));
    0
}

unsafe extern "C" fn systick_set_oneshot(evt: *mut ClockEventDevice) -> i32 {
    let name = SYSTICK.dev.name;
    let sdev = (evt as *mut u8).sub(core::mem::offset_of!(SystickDevice, dev)) as *mut SystickDevice;
    let irq = SYSTICK.dev.irq;
    if (*sdev).irq_requested == 0 {
        if request_irq(irq, systick_interrupt, IRQF_PERCPU | IRQF_TIMER, name,
                       &raw mut SYSTICK.dev as *mut _ as *mut core::ffi::c_void) != 0 {
            // pr_err("Failed to request irq %d (%s)\n", irq, name);
        }
    }
    (*sdev).irq_requested = 1;
    iowrite32(CFG_EXT_STK_EN | CFG_CNT_EN, SYSTICK.membase.add(SYSTICK_CONFIG));
    0
}

unsafe extern "C" fn ralink_systick_init(np: *mut DeviceNode) -> i32 {
    let mut ret: i32;
    SYSTICK.membase = of_iomap(np, 0);
    if SYSTICK.membase.is_null() { return -6; }
    SYSTICK.dev.name = (*np).name;
    clockevents_calc_mult_shift(&raw mut SYSTICK.dev, SYSTICK_FREQ, 60);
    SYSTICK.dev.max_delta_ns = clockevent_delta2ns(0x7fff, &SYSTICK.dev);
    SYSTICK.dev.max_delta_ticks = 0x7fff;
    SYSTICK.dev.min_delta_ns = clockevent_delta2ns(0x3, &SYSTICK.dev);
    SYSTICK.dev.min_delta_ticks = 0x3;
    SYSTICK.dev.irq = irq_of_parse_and_map(np, 0);
    if SYSTICK.dev.irq == 0 { ret = -22; goto err_iounmap; }
    ret = clocksource_mmio_init(SYSTICK.membase.add(SYSTICK_COUNT), np.as_ref().unwrap().name,
                                SYSTICK_FREQ, 301, 16, core::ptr::null());
    if ret != 0 { irq_dispose_mapping(SYSTICK.dev.irq); goto err_iounmap; }
    clockevents_register_device(&raw mut SYSTICK.dev);
    return 0;
err_iounmap:
    iounmap(SYSTICK.membase);
    ret
}

// TIMER_OF_DECLARE(systick, "ralink,cevt-systick", ralink_systick_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
