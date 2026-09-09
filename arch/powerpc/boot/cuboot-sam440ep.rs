// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Sam440ep based off bamboo.c code
 * original copyrights below
 *
 * Author: Josh Boyer <jwboyer@linux.vnet.ibm.com>
 *
 * Copyright 2007 IBM Corporation
 *
 * Based on cuboot-ebony.c
 *
 * Modified from cuboot-bamboo.c for sam440ep:
 * Copyright 2008 Giuseppe Coviello <gicoviello@gmail.com>
 */

// Dependencies supplied by the surrounding PowerPC boot sources:
// ops.h, stdio.h, 44x.h, 4xx.h, cuboot.h, and ppcboot.h.

// #define TARGET_4xx
// #define TARGET_44x

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn ibm440ep_fixup_clocks(sysclk: c_ulong, uartclk: c_ulong, tmrclk: c_ulong);
    fn ibm4xx_sdram_fixup_memsize();
    fn ibm4xx_quiesce_eth(base: *mut u32, base2: *mut u32);
    fn dt_fixup_mac_addresses(enetaddr: *mut u8, enet1addr: *mut u8);
    fn ibm44x_dbcr_reset();
    fn fdt_init(dtb: *const u8);
    fn serial_console_init();
}

unsafe fn sam440ep_fixups() {
    let sysclk: c_ulong = 66666666;

    ibm440ep_fixup_clocks(sysclk, 11059200, 25000000);
    ibm4xx_sdram_fixup_memsize();
    ibm4xx_quiesce_eth(0xef600e00 as *mut u32, 0xef600f00 as *mut u32);
    dt_fixup_mac_addresses(
        core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(bd)).bi_enetaddr) as *mut u8,
        core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(bd)).bi_enet1addr) as *mut u8,
    );
}

pub unsafe fn platform_init(
    r3: c_ulong,
    r4: c_ulong,
    r5: c_ulong,
    r6: c_ulong,
    r7: c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);
    // CUBOOT_INIT() — supplied as a build-time macro by cuboot.h.
    CUBOOT_INIT!();
    platform_ops.fixups = Some(sam440ep_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    fdt_init(_dtb_start);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
