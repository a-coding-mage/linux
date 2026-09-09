/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <linux/interrupt.h> are supplied by
// the surrounding translation unit.

extern "C" {
    pub fn iio_triggered_event_setup(
        indio_dev: *mut iio_dev,
        h: Option<unsafe extern "C" fn(irq: ::core::ffi::c_int, p: *mut ::core::ffi::c_void) -> irqreturn_t>,
        thread: Option<unsafe extern "C" fn(irq: ::core::ffi::c_int, p: *mut ::core::ffi::c_void) -> irqreturn_t>,
    );

    pub fn iio_triggered_event_cleanup(indio_dev: *mut iio_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
