/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2003 H. Peter Anvin - All Rights Reserved
 *
 * Public interface to the RAID6 P/Q calculation and recovery library.
 */

// C dependency: <linux/types.h>

pub const RAID6_MIN_DISKS: i32 = 4;

unsafe extern "C" {
    pub fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void);
    pub fn raid6_xor_syndrome(
        disks: i32,
        start: i32,
        stop: i32,
        bytes: usize,
        ptrs: *mut *mut core::ffi::c_void,
    );
    pub fn raid6_can_xor_syndrome() -> bool;

    pub fn raid6_recov_2data(
        disks: i32,
        bytes: usize,
        faila: i32,
        failb: i32,
        ptrs: *mut *mut core::ffi::c_void,
    );
    pub fn raid6_recov_datap(
        disks: i32,
        bytes: usize,
        faila: i32,
        ptrs: *mut *mut core::ffi::c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
