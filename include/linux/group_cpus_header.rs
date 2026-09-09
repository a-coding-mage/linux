/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Thomas Gleixner.
 * Copyright (C) 2016-2017 Christoph Hellwig.
 */

// Translated from the C header guard and dependencies:
// #ifndef __LINUX_GROUP_CPUS_H
// #include <linux/kernel.h>
// #include <linux/cpu.h>

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    pub fn group_cpus_evenly(
        numgrps: ::core::ffi::c_uint,
        nummasks: *mut ::core::ffi::c_uint,
    ) -> *mut cpumask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
