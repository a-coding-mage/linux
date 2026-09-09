/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Header file for Xtables timer target module.
 *
 * Copyright (C) 2004, 2010 Nokia Corporation
 * Written by Timo Teras <ext-timo.teras@nokia.com>
 *
 * Converted to x_tables and forward-ported to 2.6.34
 * by Luciano Coelho <luciano.coelho@nokia.com>
 *
 * Contact: Luciano Coelho <luciano.coelho@nokia.com>
 */

// Dependency corresponding to <linux/types.h>.

pub const MAX_IDLETIMER_LABEL_SIZE: usize = 28;
pub const XT_IDLETIMER_ALARM: u8 = 0x01;

// Opaque type supplied by the kernel timer implementation.
pub struct idletimer_tg;

#[repr(C)]
pub struct idletimer_tg_info {
    pub timeout: u32,
    pub label: [core::ffi::c_char; MAX_IDLETIMER_LABEL_SIZE],
    // For kernel module internal use only. The C declaration requests 8-byte alignment.
    pub timer: *mut idletimer_tg,
}

#[repr(C)]
pub struct idletimer_tg_info_v1 {
    pub timeout: u32,
    pub label: [core::ffi::c_char; MAX_IDLETIMER_LABEL_SIZE],
    pub send_nl_msg: u8, // Unused: for compatibility with Android.
    pub timer_type: u8,
    // For kernel module internal use only. The C declaration requests 8-byte alignment.
    pub timer: *mut idletimer_tg,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
