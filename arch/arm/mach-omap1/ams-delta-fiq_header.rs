/* SPDX-License-Identifier: GPL-2.0 */

/*
 * arch/arm/mach-omap1/ams-delta-fiq.h
 *
 * Taken from the original Amstrad modifications to fiq.h
 *
 * Copyright (c) 2004 Amstrad Plc
 * Copyright (c) 2006 Matt Callow
 * Copyright (c) 2010 Janusz Krzysztofik
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependency supplied by "irqs.h".

/*
 * Interrupt number used for passing control from FIQ to IRQ.
 * IRQ12, described as reserved, has been selected.
 */
pub const INT_DEFERRED_FIQ: usize = INT_1510_RES12;

/*
 * Base address of an interrupt handler that the INT_DEFERRED_FIQ belongs to.
 * This preserves the original preprocessor condition as a Rust constant
 * expression; the referenced IRQ symbols are supplied by the dependency.
 */
pub const DEFERRED_FIQ_IH_BASE: usize = if INT_DEFERRED_FIQ < IH2_BASE {
    OMAP_IH1_BASE
} else {
    OMAP_IH2_BASE
};

#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut qwerty_fiqin_start: u8;
    pub static mut qwerty_fiqin_end: u8;

    // __init
    pub fn ams_delta_init_fiq(chip: *mut gpio_chip, pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
