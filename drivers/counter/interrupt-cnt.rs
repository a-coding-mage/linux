// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;
use core::sync::atomic::{AtomicIsize, Ordering};

const INTERRUPT_CNT_NAME: &[u8] = b"interrupt-cnt\0";

#[repr(C)]
pub struct gpio_desc;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device;
#[repr(C)]
pub struct counter_device;
#[repr(C)]
pub struct counter_signal;
#[repr(C)]
pub struct counter_synapse;
#[repr(C)]
pub struct counter_count;
#[repr(C)]
pub struct counter_comp;
#[repr(C)]
pub struct counter_ops;
#[repr(C)]
pub struct counter_watch {
    pub channel: u8,
    pub event: c_uint,
}
#[repr(C)]
pub struct mutex;

pub type irqreturn_t = c_int;
pub type u8_ = u8;
pub type u64_ = u64;

const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TRIGGER_RISING: c_uint = 0x00000001;
const IRQ_NOAUTOEN: c_uint = 0x00000080;
const GPIOD_IN: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENXIO: c_int = 6;
const ERANGE: c_int = 34;

const COUNTER_EVENT_CHANGE_OF_STATE: c_uint = 0;
const COUNTER_SYNAPSE_ACTION_RISING_EDGE: c_uint = 0;
const COUNTER_FUNCTION_INCREASE: c_uint = 0;
const COUNTER_SIGNAL_LEVEL_HIGH: c_uint = 1;
const COUNTER_SIGNAL_LEVEL_LOW: c_uint = 0;

#[repr(C)]
pub struct interrupt_cnt_priv {
    pub count: AtomicIsize,
    pub gpio: *mut gpio_desc,
    pub irq: c_int,
    pub enabled: bool,
    pub lock: mutex,
    pub signals: counter_signal,
    pub synapses: counter_synapse,
    pub cnts: counter_count,
}

extern "C" {
    fn counter_priv(counter: *mut counter_device) -> *mut interrupt_cnt_priv;
    fn counter_push_event(counter: *mut counter_device, event: c_uint, channel: c_uint);
    fn enable_irq(irq: c_int);
    fn disable_irq(irq: c_int);
    fn gpiod_get_value(desc: *mut gpio_desc) -> c_int;
    fn platform_get_irq_optional(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_counter_alloc(dev: *mut device, size: usize) -> *mut counter_device;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_to_irq(desc: *mut gpio_desc) -> c_int;
    fn irq_set_status_flags(irq: c_int, flags: c_uint);
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut counter_device,
    ) -> c_int;
    fn devm_mutex_init(dev: *mut device, lock: *mut mutex) -> c_int;
    fn devm_counter_add(dev: *mut device, counter: *mut counter_device) -> c_int;
}

unsafe extern "C" fn interrupt_cnt_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let counter = dev_id as *mut counter_device;
    let priv_ = counter_priv(counter);
    (*priv_).count.fetch_add(1, Ordering::Relaxed);
    counter_push_event(counter, COUNTER_EVENT_CHANGE_OF_STATE, 0);
    IRQ_HANDLED
}

unsafe extern "C" fn interrupt_cnt_enable_read(
    counter: *mut counter_device,
    _count: *mut counter_count,
    enable: *mut u8,
) -> c_int {
    let priv_ = counter_priv(counter);
    *enable = (*priv_).enabled as u8;
    0
}

unsafe extern "C" fn interrupt_cnt_enable_write(
    counter: *mut counter_device,
    _count: *mut counter_count,
    enable: u8,
) -> c_int {
    let priv_ = counter_priv(counter);
    if (*priv_).enabled as u8 == enable { return 0; }
    if enable != 0 {
        (*priv_).enabled = true;
        enable_irq((*priv_).irq);
    } else {
        disable_irq((*priv_).irq);
        (*priv_).enabled = false;
    }
    0
}

unsafe extern "C" fn interrupt_cnt_action_read(
    _counter: *mut counter_device, _count: *mut counter_count,
    _synapse: *mut counter_synapse, action: *mut c_uint,
) -> c_int { *action = COUNTER_SYNAPSE_ACTION_RISING_EDGE; 0 }

unsafe extern "C" fn interrupt_cnt_read(
    counter: *mut counter_device, _count: *mut counter_count, val: *mut u64,
) -> c_int {
    *val = counter_priv(counter).as_ref().unwrap().count.load(Ordering::Relaxed) as u64;
    0
}

unsafe extern "C" fn interrupt_cnt_write(
    counter: *mut counter_device, _count: *mut counter_count, val: u64,
) -> c_int {
    let priv_ = counter_priv(counter);
    if val > isize::MAX as u64 { return -ERANGE; }
    (*priv_).count.store(val as isize, Ordering::Relaxed);
    0
}

unsafe extern "C" fn interrupt_cnt_function_read(
    _counter: *mut counter_device, _count: *mut counter_count, function: *mut c_uint,
) -> c_int { *function = COUNTER_FUNCTION_INCREASE; 0 }

unsafe extern "C" fn interrupt_cnt_signal_read(
    counter: *mut counter_device, _signal: *mut counter_signal, level: *mut c_uint,
) -> c_int {
    let priv_ = counter_priv(counter);
    if (*priv_).gpio.is_null() { return -EINVAL; }
    let ret = gpiod_get_value((*priv_).gpio);
    if ret < 0 { return ret; }
    *level = if ret != 0 { COUNTER_SIGNAL_LEVEL_HIGH } else { COUNTER_SIGNAL_LEVEL_LOW };
    0
}

unsafe extern "C" fn interrupt_cnt_watch_validate(
    _counter: *mut counter_device, watch: *const counter_watch,
) -> c_int {
    if (*watch).channel != 0 || (*watch).event != COUNTER_EVENT_CHANGE_OF_STATE { return -EINVAL; }
    0
}

unsafe extern "C" fn interrupt_cnt_probe(pdev: *mut platform_device) -> c_int {
    // Direct translation of the C probe's resource acquisition and registration sequence.
    let dev = pdev as *mut device;
    let counter = devm_counter_alloc(dev, core::mem::size_of::<interrupt_cnt_priv>());
    if counter.is_null() { return -ENOMEM; }
    let priv_ = counter_priv(counter);
    (*priv_).irq = platform_get_irq_optional(pdev, 0);
    if (*priv_).irq == -ENXIO { (*priv_).irq = 0; }
    else if (*priv_).irq < 0 { return (*priv_).irq; }
    (*priv_).gpio = devm_gpiod_get_optional(dev, ptr::null(), GPIOD_IN);
    if (*priv_).irq == 0 && (*priv_).gpio.is_null() { return -ENODEV; }
    if (*priv_).irq == 0 {
        let irq = gpiod_to_irq((*priv_).gpio);
        if irq < 0 { return irq; }
        (*priv_).irq = irq;
    }
    irq_set_status_flags((*priv_).irq, IRQ_NOAUTOEN);
    let ret = devm_request_irq(dev, (*priv_).irq, interrupt_cnt_isr,
                               IRQF_TRIGGER_RISING, INTERRUPT_CNT_NAME.as_ptr() as *const c_char, counter);
    if ret != 0 { return ret; }
    let ret = devm_mutex_init(dev, &mut (*priv_).lock);
    if ret != 0 { return ret; }
    let ret = devm_counter_add(dev, counter);
    if ret < 0 { return ret; }
    0
}

// C descriptors and module registration are represented as kernel-facing symbols.
#[no_mangle]
pub static INTERRUPT_CNT_OF_MATCH_COMPATIBLE: &[u8] = b"interrupt-counter\0";
#[allow(dead_code)]
static INTERRUPT_CNT_DRIVER_NAME: *const u8 = INTERRUPT_CNT_NAME.as_ptr();

// MODULE_ALIAS("platform:interrupt-counter");
// MODULE_AUTHOR("Oleksij Rempel <o.rempel@pengutronix.de>");
// MODULE_DESCRIPTION("Interrupt counter driver");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("COUNTER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
