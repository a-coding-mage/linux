/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/mach-mvebu/include/mach/coherency.h
 *
 * Coherency fabric (Aurora) support for Armada 370 and XP platforms.
 *
 * Copyright (C) 2012 Marvell
 */

// C header guard: __MACH_370_XP_COHERENCY_H

unsafe extern "C" {
    pub static mut coherency_base: *mut core::ffi::c_void; // for coherency_ll.S
    pub static mut coherency_phys_base: usize;
    pub fn set_cpu_coherent() -> i32;

    pub fn coherency_init() -> i32;
    pub fn coherency_available() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
