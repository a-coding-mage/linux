// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Paul Mundt
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int};

extern "C" {
    fn gpio_request(gpio: c_int, label: *const c_char) -> c_int;
}

// GPIO_FN_* constants are supplied by cpu/sh7785.h and mach/highlander.h.

pub unsafe fn highlander_plat_pinmux_setup() {
    /* SCIF0 */
    gpio_request(GPIO_FN_SCIF0_CTS, core::ptr::null());
    gpio_request(GPIO_FN_SCIF0_RTS, core::ptr::null());
    gpio_request(GPIO_FN_SCIF0_SCK, core::ptr::null());
    gpio_request(GPIO_FN_SCIF0_RXD, core::ptr::null());
    gpio_request(GPIO_FN_SCIF0_TXD, core::ptr::null());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
