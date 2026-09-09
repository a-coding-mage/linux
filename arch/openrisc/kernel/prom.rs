// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC prom.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 *
 * Architecture specific procedures for creating, accessing and
 * interpreting the device tree.
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn early_init_dt_scan(params: *mut core::ffi::c_void, addr: usize);
    fn memblock_allow_resize();
    // `__pa` is provided by the architecture-specific page definitions.
    fn __pa(addr: *mut core::ffi::c_void) -> usize;
}

pub unsafe fn early_init_devtree(params: *mut core::ffi::c_void) {
    early_init_dt_scan(params, __pa(params));
    memblock_allow_resize();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
