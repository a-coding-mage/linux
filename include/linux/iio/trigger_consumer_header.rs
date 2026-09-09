/* SPDX-License-Identifier: GPL-2.0-only */
/* The industrial I/O core, trigger consumer functions
 *
 * Copyright (c) 2008-2011 Jonathan Cameron
 */

// C dependencies supplied by the surrounding kernel translation:
// `irqreturn_t`.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_trigger {
    _private: [u8; 0],
}

/**
 * struct iio_poll_func - poll function pair
 *
 * @indio_dev:            data specific to device (passed into poll func)
 * @h:                    the function that is actually run on trigger
 * @thread:               threaded interrupt part
 * @type:                 the type of interrupt (basically if oneshot)
 * @name:                 name used to identify the trigger consumer.
 * @irq:                  the corresponding irq as allocated from the
 *                        trigger pool
 * @timestamp:            some devices need a timestamp grabbed as soon
 *                        as possible after the trigger - hence handler
 *                        passes it via here.
 **/
#[repr(C)]
pub struct iio_poll_func {
    pub indio_dev: *mut iio_dev,
    pub h: Option<unsafe extern "C" fn(irq: i32, p: *mut c_void) -> irqreturn_t>,
    pub thread: Option<unsafe extern "C" fn(irq: i32, p: *mut c_void) -> irqreturn_t>,
    pub type_: i32,
    pub name: *mut c_char,
    pub irq: i32,
    pub timestamp: i64,
}

// C attribute: __printf(5, 6)
unsafe extern "C" {
    pub fn iio_alloc_pollfunc(
        h: Option<unsafe extern "C" fn(irq: i32, p: *mut c_void) -> irqreturn_t>,
        thread: Option<unsafe extern "C" fn(irq: i32, p: *mut c_void) -> irqreturn_t>,
        type_: i32,
        indio_dev: *mut iio_dev,
        fmt: *const c_char,
        ...,
    ) -> *mut iio_poll_func;

    pub fn iio_dealloc_pollfunc(pf: *mut iio_poll_func);
    pub fn iio_pollfunc_store_time(irq: i32, p: *mut c_void) -> irqreturn_t;
    pub fn iio_trigger_notify_done(trig: *mut iio_trigger);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
