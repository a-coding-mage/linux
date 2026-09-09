// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Yosemite
 *
 * Author: Josh Boyer <jwboyer@linux.vnet.ibm.com>
 *
 * Copyright 2008 IBM Corporation
 */

// C dependencies: "ops.h", "stdio.h", "4xx.h", "44x.h", "cuboot.h",
// and "ppcboot.h" (with TARGET_4xx and TARGET_44x defined) provide the
// declarations and macros referenced below.

const TARGET_4XX: () = ();
const TARGET_44X: () = ();

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe fn yosemite_fixups() {
    let sysclk: ::core::ffi::c_ulong = 66666666;

    ibm440ep_fixup_clocks(sysclk, 11059200, 50000000);
    ibm4xx_sdram_fixup_memsize();
    ibm4xx_quiesce_eth(0xef600e00 as *mut u32, 0xef600f00 as *mut u32);
    dt_fixup_mac_address_by_alias(
        b"ethernet0\0".as_ptr() as *const ::core::ffi::c_char,
        (*::core::ptr::addr_of!(bd)).bi_enetaddr,
    );
    dt_fixup_mac_address_by_alias(
        b"ethernet1\0".as_ptr() as *const ::core::ffi::c_char,
        (*::core::ptr::addr_of!(bd)).bi_enet1addr,
    );
}

pub unsafe fn platform_init(
    r3: ::core::ffi::c_ulong,
    r4: ::core::ffi::c_ulong,
    r5: ::core::ffi::c_ulong,
    r6: ::core::ffi::c_ulong,
    r7: ::core::ffi::c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);
    CUBOOT_INIT!();
    platform_ops.fixups = Some(yosemite_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    fdt_init(_dtb_start);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
