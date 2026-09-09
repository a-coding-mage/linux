/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/exec.h
 *
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency intent from C: <linux/sched.h>

pub unsafe extern "C" {
    pub fn arch_align_stack(sp: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
