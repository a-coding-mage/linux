/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/hardware/cache-feroceon-l2.h
 *
 * Copyright (C) 2008 Marvell Semiconductor
 */

// The C __init annotation is a build/linkage attribute with no direct
// file-local Rust equivalent.
unsafe extern "C" {
    pub fn feroceon_l2_init(l2_wt_override: ::core::ffi::c_int);
    pub fn feroceon_of_init() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
