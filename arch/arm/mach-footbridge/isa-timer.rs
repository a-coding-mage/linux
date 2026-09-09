// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/arm/mach-footbridge/isa-timer.c
 *
 *  Copyright (C) 1998 Russell King.
 *  Copyright (C) 1998 Phil Blundell
 */

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the surrounding kernel sources.
#[repr(C)]
pub struct clock_event_device {
    pub irq: c_int,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

pub type irqreturn_t = c_int;

pub const IRQ_HANDLED: irqreturn_t = 1;
pub const IRQF_TIMER: c_int = 0x0000_0020;
pub const IRQF_IRQPOLL: c_int = 0x0000_1000;

unsafe extern "C" {
    pub static mut i8253_clockevent: clock_event_device;

    pub fn clocksource_i8253_init();
    pub fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_int,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    pub fn clockevent_i8253_init(periodic: bool);
    pub fn pr_err(fmt: *const c_char, ...);
}

unsafe extern "C" fn pit_timer_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let ce = dev_id as *mut clock_event_device;
    if let Some(event_handler) = (*ce).event_handler {
        event_handler(ce);
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn isa_timer_init() {
    clocksource_i8253_init();

    if request_irq(
        i8253_clockevent.irq,
        pit_timer_interrupt,
        IRQF_TIMER | IRQF_IRQPOLL,
        b"pit\0".as_ptr() as *const c_char,
        &raw mut i8253_clockevent as *mut c_void,
    ) != 0
    {
        pr_err(
            b"Failed to request irq %d(pit)\n\0".as_ptr() as *const c_char,
            i8253_clockevent.irq,
        );
    }
    clockevent_i8253_init(false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
