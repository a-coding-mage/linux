/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  arch/arm/include/asm/hardware/vic.h
 *
 *  Copyright (c) ARM Limited 2003.  All rights reserved.
 */

// Dependency equivalent to <linux/types.h>: u32 is represented by Rust's u32.

extern "C" {
    pub fn vic_init(
        base: *mut core::ffi::c_void,
        irq_start: u32,
        vic_sources: u32,
        resume_sources: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
