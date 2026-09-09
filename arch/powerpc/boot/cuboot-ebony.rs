// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Ebony
 *
 * Author: David Gibson <david@gibson.dropbear.id.au>
 *
 * Copyright 2007 David Gibson, IBM Corporatio.
 *   Based on cuboot-83xx.c, which is:
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding PowerPC boot code:
// ops.h, stdio, 44x.h, cuboot.h
// TARGET_4xx and TARGET_44x are defined for ppcboot.h.

extern "C" {
    fn ebony_init(enetaddr: *mut u8, enet1addr: *mut u8);
}

// bd_t is supplied by ppcboot.h.
static mut bd: bd_t = unsafe { core::mem::zeroed() };

pub unsafe extern "C" fn platform_init(
    r3: core::ffi::c_ulong,
    r4: core::ffi::c_ulong,
    r5: core::ffi::c_ulong,
    r6: core::ffi::c_ulong,
    r7: core::ffi::c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);

    // CUBOOT_INIT();
    ebony_init(&mut bd.bi_enetaddr as *mut _, &mut bd.bi_enet1addr as *mut _);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
