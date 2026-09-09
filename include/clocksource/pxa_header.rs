/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * PXA clocksource, clockevents, and OST interrupt handlers.
 *
 * Copyright (C) 2014 Robert Jarzmik
 */

// External C declaration corresponding to the original header declaration.
unsafe extern "C" {
    pub fn pxa_timer_nodt_init(
        irq: core::ffi::c_int,
        base: *mut core::ffi::c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
