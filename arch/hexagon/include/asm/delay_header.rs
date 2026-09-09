/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency intent: declarations from <asm/param.h> are supplied externally.

extern "C" {
    pub fn __delay(cycles: core::ffi::c_ulong);
    pub fn __udelay(usecs: core::ffi::c_ulong);
}

#[macro_export]
macro_rules! udelay {
    ($usecs:expr) => {
        unsafe { $crate::__udelay($usecs) }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
