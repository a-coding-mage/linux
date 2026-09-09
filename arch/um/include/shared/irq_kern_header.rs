/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2001, 2002 Jeff Dike (jdike@karaya.com)
 */

// Dependencies supplied by the surrounding kernel/UML translation.

pub const UM_IRQ_ALLOC: i32 = -1;

unsafe extern "C" {
    pub fn um_request_irq(
        irq: i32,
        fd: i32,
        irq_type: um_irq_type,
        handler: irq_handler_t,
        irqflags: core::ffi::c_ulong,
        devname: *const core::ffi::c_char,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
}

/* CONFIG_UML_TIME_TRAVEL_SUPPORT selects the external declaration below. */
#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
unsafe extern "C" {
    pub fn um_request_irq_tt(
        irq: i32,
        fd: i32,
        irq_type: um_irq_type,
        handler: irq_handler_t,
        irqflags: core::ffi::c_ulong,
        devname: *const core::ffi::c_char,
        dev_id: *mut core::ffi::c_void,
        timetravel_handler: Option<
            unsafe extern "C" fn(
                i32,
                i32,
                *mut core::ffi::c_void,
                *mut time_travel_event,
            ),
        >,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
pub unsafe fn um_request_irq_tt(
    irq: i32,
    fd: i32,
    irq_type: um_irq_type,
    handler: irq_handler_t,
    irqflags: core::ffi::c_ulong,
    devname: *const core::ffi::c_char,
    dev_id: *mut core::ffi::c_void,
    _timetravel_handler: Option<
        unsafe extern "C" fn(
            i32,
            i32,
            *mut core::ffi::c_void,
            *mut time_travel_event,
        ),
    >,
) -> i32 {
    um_request_irq(irq, fd, irq_type, handler, irqflags, devname, dev_id)
}

pub unsafe fn um_irq_timetravel_handler_used() -> bool {
    time_travel_mode == TT_MODE_EXTERNAL
}

unsafe extern "C" {
    pub fn um_free_irq(irq: i32, dev_id: *mut core::ffi::c_void);
    pub fn free_irqs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
