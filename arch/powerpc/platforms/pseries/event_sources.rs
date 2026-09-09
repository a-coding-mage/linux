// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001 Dave Engebretsen IBM Corporation
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

pub type irq_handler_t = unsafe extern "C" fn(c_int, *mut c_void) -> c_int;

unsafe extern "C" {
    fn of_irq_get(np: *mut device_node, index: c_uint) -> c_int;
    fn request_irq(
        irq: c_int,
        handler: irq_handler_t,
        flags: c_uint,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn WARN(condition: bool, format: *const c_char, ...);
}

pub unsafe fn request_event_sources_irqs(
    np: *mut device_node,
    handler: irq_handler_t,
    name: *const c_char,
) {
    let mut i: c_int;
    let mut virq: c_int;
    let mut rc: c_int;

    i = 0;
    while i < 16 {
        virq = unsafe { of_irq_get(np, i as c_uint) };
        if virq < 0 {
            return;
        }
        if unsafe {
            WARN(
                virq == 0,
                b"event-sources: Unable to allocate interrupt number for %pOF\n\0"
                    .as_ptr() as *const c_char,
                np,
            );
            virq == 0
        } {
            continue;
        }

        rc = unsafe { request_irq(virq, handler, 0, name, core::ptr::null_mut()) };
        if unsafe {
            WARN(
                rc != 0,
                b"event-sources: Unable to request interrupt %d for %pOF\n\0".as_ptr()
                    as *const c_char,
                virq,
                np,
            );
            rc != 0
        } {
            return;
        }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
