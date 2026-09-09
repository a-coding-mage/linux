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

// Dependencies supplied by the surrounding PowerPC boot environment:
// "ops.h", "stdio.h", and "44x.h".

// Original declaration: BSS_STACK(4096);
// This build-time macro supplies the boot BSS stack.

const OPENBIOS_MAC_BASE: usize = 0xfffffe0c;
const OPENBIOS_MAC_OFFSET: usize = 0xc;

unsafe extern "C" {
    static mut _end: u8;

    fn simple_alloc_init(
        base: *mut u8,
        size: usize,
        align: usize,
        boundary: usize,
    );
    fn ebony_init(mac_base: *mut u8, mac_address: *mut u8);
}

#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    let end_of_ram: usize = 0x8000000;
    let end = core::ptr::addr_of_mut!(_end);
    let avail_ram = end_of_ram - (end as usize);

    simple_alloc_init(end, avail_ram, 32, 64);
    ebony_init(
        OPENBIOS_MAC_BASE as *mut u8,
        (OPENBIOS_MAC_BASE + OPENBIOS_MAC_OFFSET) as *mut u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
