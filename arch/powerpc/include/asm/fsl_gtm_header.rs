/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Freescale General-purpose Timers Module
 *
 * Copyright 2006 Freescale Semiconductor, Inc.
 *               Shlomi Gridish <gridish@freescale.com>
 *               Jerry Huang <Chang-Ming.Huang@freescale.com>
 * Copyright (c) MontaVista Software, Inc. 2008.
 *               Anton Vorontsov <avorontsov@ru.mvista.com>
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

#[repr(C)]
pub struct gtm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gtm_timer {
    pub irq: u32,

    pub gtm: *mut gtm,
    pub requested: bool,
    pub gtcfr: *mut u8,
    pub gtmdr: *mut u16,
    pub gtpsr: *mut u16,
    pub gtcnr: *mut u16,
    pub gtrfr: *mut u16,
    pub gtevr: *mut u16,
}

extern "C" {
    pub fn gtm_get_timer16() -> *mut gtm_timer;
    pub fn gtm_get_specific_timer16(gtm: *mut gtm, timer: u32) -> *mut gtm_timer;
    pub fn gtm_put_timer16(tmr: *mut gtm_timer);
    pub fn gtm_set_timer16(tmr: *mut gtm_timer, usec: core::ffi::c_ulong, reload: bool) -> i32;
    pub fn gtm_set_exact_timer16(tmr: *mut gtm_timer, usec: u16, reload: bool) -> i32;
    pub fn gtm_stop_timer16(tmr: *mut gtm_timer);
    pub fn gtm_ack_timer16(tmr: *mut gtm_timer, events: u16);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
