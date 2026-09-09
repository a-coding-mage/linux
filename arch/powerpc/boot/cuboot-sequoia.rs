// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Sequoia
 *
 * Valentine Barshak <vbarshak@ru.mvista.com>
 * Copyright 2007 MontaVista Software, Inc
 *
 * Based on Ebony code by David Gibson <david@gibson.dropbear.id.au>
 * Copyright IBM Corporation, 2007
 *
 * Based on Bamboo code by Josh Boyer <jwboyer@linux.vnet.ibm.com>
 * Copyright IBM Corporation, 2007
 */

// C dependencies: stdarg.h, stddef.h, types.h, elf.h, string.h, stdio.h,
// page.h, ops.h, dcr.h, 4xx.h, 44x.h, cuboot.h, and ppcboot.h.
// Build-time target conditions retained from the original source.

const TARGET_4XX: bool = true;
const TARGET_44X: bool = true;

extern "C" {
    static mut bd: bd_t;

    fn ibm440ep_fixup_clocks(sysclk: core::ffi::c_ulong, uartclk: core::ffi::c_ulong, plbclk: core::ffi::c_ulong);
    fn ibm4xx_fixup_ebc_ranges(path: *const core::ffi::c_char);
    fn ibm4xx_denali_fixup_memsize();
    fn dt_fixup_mac_address_by_alias(alias: *const core::ffi::c_char, address: *const u8);
    fn ibm44x_dbcr_reset();
    fn fdt_init(dtb_start: *const u8);
    fn serial_console_init();

    static mut platform_ops: platform_ops;
    static _dtb_start: u8;
}

unsafe fn sequoia_fixups() {
    let sysclk: core::ffi::c_ulong = 33333333;

    ibm440ep_fixup_clocks(sysclk, 11059200, 50000000);
    ibm4xx_fixup_ebc_ranges(b"/plb/opb/ebc\0".as_ptr() as *const c_char);
    ibm4xx_denali_fixup_memsize();
    dt_fixup_mac_address_by_alias(b"ethernet0\0".as_ptr() as *const core::ffi::c_char, core::ptr::addr_of!(bd.bi_enetaddr));
    dt_fixup_mac_address_by_alias(b"ethernet1\0".as_ptr() as *const core::ffi::c_char, core::ptr::addr_of!(bd.bi_enet1addr));
}

pub unsafe fn platform_init(
    r3: core::ffi::c_ulong,
    r4: core::ffi::c_ulong,
    r5: core::ffi::c_ulong,
    r6: core::ffi::c_ulong,
    r7: core::ffi::c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);
    CUBOOT_INIT!();
    platform_ops.fixups = Some(sequoia_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    fdt_init(core::ptr::addr_of!(_dtb_start));
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
