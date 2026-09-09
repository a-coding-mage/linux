// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Cirrus Logic CLPS711X clocksource driver
 *
 *  Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
 */

// Linux kernel dependencies supplied by other translation units.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clock_event_device {
    pub name: *const c_char,
    pub rating: c_int,
    pub features: c_uint,
    pub cpumask: *const cpumask,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

pub type u64_ = u64;
pub type irqreturn_t = c_int;

pub const IRQ_HANDLED: irqreturn_t = 1;
pub const IRQF_TIMER: c_ulong = 0x0000_0020;
pub const CLOCK_EVT_FEAT_PERIODIC: c_uint = 0x0000_0010;
pub const CLOCK_EVT_FEAT_C3STOP: c_uint = 0x0000_0080;
pub const HZ: c_ulong = 100;
pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;

extern "C" {
    fn readw(addr: *mut c_void) -> u16;
    fn writew(value: u16, addr: *mut c_void);
    fn clk_get_rate(clock: *mut clk) -> c_ulong;
    fn clocksource_mmio_init(
        addr: *mut c_void,
        name: *const c_char,
        rating: c_ulong,
        shift: c_int,
        bits: c_int,
        read: Option<unsafe extern "C" fn(*mut c_void) -> u64>,
    ) -> c_int;
    fn sched_clock_register(read: Option<unsafe extern "C" fn() -> u64>, bits: c_int, rate: c_ulong);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn cpumask_of(cpu: c_uint) -> *const cpumask;
    fn clockevents_config_and_register(
        device: *mut clock_event_device,
        freq: c_ulong,
        min_delta: c_ulong,
        max_delta: c_ulong,
    );
    fn request_irq(
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn irq_of_parse_and_map(np: *mut device_node, index: c_uint) -> c_uint;
    fn of_clk_get(np: *mut device_node, index: c_uint) -> *mut clk;
    fn of_iomap(np: *mut device_node, index: c_uint) -> *mut c_void;
    fn of_alias_get_id(np: *mut device_node, stem: *const c_char) -> c_int;
    fn iounmap(addr: *mut c_void);
    fn ptr_err(ptr: *mut clk) -> c_int;
}

const CLPS711X_CLKSRC_CLOCKSOURCE: c_int = 0;
const CLPS711X_CLKSRC_CLOCKEVENT: c_int = 1;

static mut tcd: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn clps711x_sched_clock_read() -> u64 {
    !(readw(tcd) as u64)
}

unsafe extern "C" fn clps711x_clksrc_init(clock: *mut clk, base: *mut c_void) {
    let rate: c_ulong = clk_get_rate(clock);

    tcd = base;

    clocksource_mmio_init(
        tcd,
        b"clps711x-clocksource\0".as_ptr() as *const c_char,
        rate,
        300,
        16,
        Some(clocksource_mmio_readw_down),
    );

    sched_clock_register(Some(clps711x_sched_clock_read), 16, rate);
}

unsafe extern "C" fn clps711x_timer_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;

    if let Some(event_handler) = (*evt).event_handler {
        event_handler(evt);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn _clps711x_clkevt_init(
    clock: *mut clk,
    base: *mut c_void,
    irq: c_uint,
) -> c_int {
    let clkevt = kzalloc(core::mem::size_of::<clock_event_device>(), 0) as *mut clock_event_device;
    if clkevt.is_null() {
        return -ENOMEM;
    }

    let rate: c_ulong = clk_get_rate(clock);

    // Set Timer prescaler
    writew(((rate + HZ / 2) / HZ) as u16, base);

    (*clkevt).name = b"clps711x-clockevent\0".as_ptr() as *const c_char;
    (*clkevt).rating = 300;
    (*clkevt).features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_C3STOP;
    (*clkevt).cpumask = cpumask_of(0);
    clockevents_config_and_register(clkevt, HZ, 0, 0);

    request_irq(
        irq,
        Some(clps711x_timer_interrupt),
        IRQF_TIMER,
        b"clps711x-timer\0".as_ptr() as *const c_char,
        clkevt as *mut c_void,
    )
}

unsafe extern "C" fn clps711x_timer_init(np: *mut device_node) -> c_int {
    let irq: c_uint = irq_of_parse_and_map(np, 0);
    let clock: *mut clk = of_clk_get(np, 0);
    let base: *mut c_void = of_iomap(np, 0);
    let mut ret: c_int = 0;

    if base.is_null() {
        return -ENOMEM;
    }
    if irq == 0 {
        ret = -EINVAL;
        iounmap(base);
        return ret;
    }
    // IS_ERR(clock) is supplied by the kernel error-pointer machinery.
    if (clock as usize) >= (usize::MAX - 4095) {
        ret = ptr_err(clock);
        iounmap(base);
        return ret;
    }

    match of_alias_get_id(np, b"timer\0".as_ptr() as *const c_char) {
        CLPS711X_CLKSRC_CLOCKSOURCE => {
            clps711x_clksrc_init(clock, base);
            0
        }
        CLPS711X_CLKSRC_CLOCKEVENT => {
            ret = _clps711x_clkevt_init(clock, base, irq);
            iounmap(base);
            ret
        }
        _ => {
            ret = -EINVAL;
            iounmap(base);
            ret
        }
    }
}

extern "C" {
    fn clocksource_mmio_readw_down(addr: *mut c_void) -> u64;
}

// TIMER_OF_DECLARE(clps711x, "cirrus,ep7209-timer", clps711x_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
