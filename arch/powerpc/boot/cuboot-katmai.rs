// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Katmai
 *
 * Author: Hugh Blemings <hugh@au.ibm.com>
 *
 * Copyright 2007 Hugh Blemings, IBM Corporation.
 *   Based on cuboot-ebony.c which is:
 * Copyright 2007 David Gibson, IBM Corporation.
 *   Based on cuboot-83xx.c, which is:
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding PowerPC boot environment:
// ops.h, stdio.h, reg.h, dcr.h, 4xx.h, 44x.h, cuboot.h, and ppcboot.h

// Build-time conditions from the C source.
// #define TARGET_4xx
// #define TARGET_44x

extern "C" {
    static mut bd: bd_t;
    static mut platform_ops: platform_ops_t;
    static _dtb_start: core::ffi::c_void;

    fn ibm440spe_fixup_clocks(sysclk: core::ffi::c_ulong, uartclk: core::ffi::c_ulong, flags: core::ffi::c_ulong);
    fn ibm440spe_fixup_memsize();
    fn dt_fixup_mac_address(index: core::ffi::c_ulong, address: *const u8);
    fn ibm4xx_fixup_ebc_ranges(path: *const u8);
    fn fdt_init(dtb: *const core::ffi::c_void);
    fn serial_console_init();
}

// BSS_STACK(4096);

unsafe fn katmai_fixups() {
    let sysclk: core::ffi::c_ulong = 33333000;

    /* 440SP Clock logic is all but identical to 440GX
     * so we just use that code for now at least
     */
    ibm440spe_fixup_clocks(sysclk, 6 * 1843200, 0);

    ibm440spe_fixup_memsize();

    dt_fixup_mac_address(0, (*core::ptr::addr_of!(bd)).bi_enetaddr.as_ptr());

    ibm4xx_fixup_ebc_ranges(b"/plb/opb/ebc\0".as_ptr());
}

pub unsafe fn platform_init(
    _r3: core::ffi::c_ulong,
    _r4: core::ffi::c_ulong,
    _r5: core::ffi::c_ulong,
    _r6: core::ffi::c_ulong,
    _r7: core::ffi::c_ulong,
) {
    // CUBOOT_INIT();

    (*core::ptr::addr_of_mut!(platform_ops)).fixups = Some(katmai_fixups);
    fdt_init(core::ptr::addr_of!(_dtb_start));
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
