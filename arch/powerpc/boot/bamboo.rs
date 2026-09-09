// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright IBM Corporation, 2007
 * Josh Boyer <jwboyer@linux.vnet.ibm.com>
 *
 * Based on ebony wrapper:
 * Copyright 2007 David Gibson, IBM Corporation.
 *
 * Clocking code based on code by:
 * Stefan Roese <sr@denx.de>
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding PowerPC boot environment.
#[repr(C)]
pub struct PlatformOps {
    pub fixups: Option<unsafe extern "C" fn()>,
    pub exit: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static mut platform_ops: PlatformOps;
    static _dtb_start: u8;

    fn ibm440ep_fixup_clocks(sysclk: u32, uartclk: u32, pci_clk: u32);
    fn ibm4xx_sdram_fixup_memsize();
    fn ibm4xx_quiesce_eth(base0: *mut u32, base1: *mut u32);
    fn dt_fixup_mac_address_by_alias(alias: *const u8, mac: *mut u8);
    fn ibm44x_dbcr_reset();
    fn fdt_init(dtb: *const u8);
    fn serial_console_init();
}

static mut bamboo_mac0: *mut u8 = core::ptr::null_mut();
static mut bamboo_mac1: *mut u8 = core::ptr::null_mut();

unsafe extern "C" fn bamboo_fixups() {
    let sysclk: u32 = 33333333;

    ibm440ep_fixup_clocks(sysclk, 11059200, 25000000);
    ibm4xx_sdram_fixup_memsize();
    ibm4xx_quiesce_eth(0xef600e00 as *mut u32, 0xef600f00 as *mut u32);
    dt_fixup_mac_address_by_alias(b"ethernet0\0".as_ptr(), bamboo_mac0);
    dt_fixup_mac_address_by_alias(b"ethernet1\0".as_ptr(), bamboo_mac1);
}

pub unsafe extern "C" fn bamboo_init(mac0: *mut c_void, mac1: *mut c_void) {
    platform_ops.fixups = Some(bamboo_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    bamboo_mac0 = mac0 as *mut u8;
    bamboo_mac1 = mac1 as *mut u8;
    fdt_init(core::ptr::addr_of!(_dtb_start));
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
