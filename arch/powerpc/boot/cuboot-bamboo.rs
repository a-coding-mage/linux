// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Bamboo
 *
 * Author: Josh Boyer <jwboyer@linux.vnet.ibm.com>
 *
 * Copyright 2007 IBM Corporation
 *
 * Based on cuboot-ebony.c
 */

// Dependencies supplied by the surrounding PowerPC boot environment:
// "ops.h", "stdio.h", "44x.h", "cuboot.h", and "ppcboot.h".
//
// #define TARGET_4xx
// #define TARGET_44x

use core::ffi::c_ulong;

static mut bd: bd_t = unsafe { core::mem::zeroed() };

extern "C" {
    fn bamboo_init(enetaddr: *mut u8, enet1addr: *mut u8);
}

pub unsafe fn platform_init(
    r3: c_ulong,
    r4: c_ulong,
    r5: c_ulong,
    r6: c_ulong,
    r7: c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);
    // CUBOOT_INIT();
    cuboot_init!();
    bamboo_init(
        core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(bd)).bi_enetaddr),
        core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(bd)).bi_enet1addr),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
