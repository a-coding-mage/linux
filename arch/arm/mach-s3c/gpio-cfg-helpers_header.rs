/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * Samsung Platform - GPIO pin configuration helper definitions
 */

/* This is meant for core cpu support, machine or other driver files
 * should not be including this header.
 */

/* As a note, all gpio configuration functions are entered exclusively, either
 * with the relevant lock held or the system prevented from doing anything else
 * by disabling interrupts.
 */

/// Direct wrapper for the GPIO configuration callback.
#[inline]
pub unsafe fn samsung_gpio_do_setcfg(
    chip: *mut samsung_gpio_chip,
    off: ::core::ffi::c_uint,
    config: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    ((*(*chip).config).set_config)(chip, off, config)
}

/// Direct wrapper for the GPIO pull configuration callback.
#[inline]
pub unsafe fn samsung_gpio_do_setpull(
    chip: *mut samsung_gpio_chip,
    off: ::core::ffi::c_uint,
    pull: samsung_gpio_pull_t,
) -> ::core::ffi::c_int {
    ((*(*chip).config).set_pull)(chip, off, pull)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
