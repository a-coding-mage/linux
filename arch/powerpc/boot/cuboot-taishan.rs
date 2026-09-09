// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Taishan
 *
 * Author: Hugh Blemings <hugh@au.ibm.com>
 *
 * Copyright 2007 Hugh Blemings, IBM Corporation.
 *   Based on cuboot-ebony.c which is:
 * Copyright 2007 David Gibson, IBM Corporation.
 *   Based on cuboot-83xx.c, which is:
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding translation unit/build.

static mut bd: bd_t = unsafe { core::mem::zeroed() };

// BSS_STACK(4096);

unsafe fn taishan_fixups() {
    /* FIXME: sysclk should be derived by reading the FPGA
       registers */
    let sysclk: c_ulong = 33000000;

    ibm440gx_fixup_clocks(sysclk, 6 * 1843200, 25000000);

    ibm4xx_sdram_fixup_memsize();

    dt_fixup_mac_address_by_alias(
        "ethernet0\0".as_ptr() as *const c_char,
        (*core::ptr::addr_of!(bd)).bi_enetaddr.as_ptr(),
    );
    dt_fixup_mac_address_by_alias(
        "ethernet1\0".as_ptr() as *const c_char,
        (*core::ptr::addr_of!(bd)).bi_enet1addr.as_ptr(),
    );

    ibm4xx_fixup_ebc_ranges("/plb/opb/ebc\0".as_ptr() as *const c_char);
}

pub unsafe fn platform_init(
    r3: c_ulong,
    r4: c_ulong,
    r5: c_ulong,
    r6: c_ulong,
    r7: c_ulong,
) {
    // CUBOOT_INIT();

    platform_ops.fixups = Some(taishan_fixups);
    fdt_init(_dtb_start);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
