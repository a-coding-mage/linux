/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/mach/time.h
 *
 * Copyright (C) 2004 MontaVista Software, Inc.
 */

// The C header guard is omitted; Rust modules provide equivalent scoping.

// `timespec64` is supplied by an external dependency.
pub type clock_access_fn = Option<unsafe extern "C" fn(*mut timespec64)>;

unsafe extern "C" {
    pub fn register_persistent_clock(read_persistent: clock_access_fn) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
