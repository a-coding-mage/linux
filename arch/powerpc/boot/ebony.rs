// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2007 David Gibson, IBM Corporation.
 *
 * Based on earlier code:
 *   Copyright (C) Paul Mackerras 1997.
 *
 *   Matt Porter <mporter@kernel.crashing.org>
 *   Copyright 2002-2005 MontaVista Software Inc.
 *
 *   Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 *   Copyright (c) 2003, 2004 Zultys Technologies
 */

// C dependencies: stdarg.h, stddef.h, types.h, elf.h, string.h, stdio.h,
// page.h, ops.h, reg.h, io.h, dcr.h, 4xx.h, and 44x.h.

static mut ebony_mac0: *mut u8 = core::ptr::null_mut();
static mut ebony_mac1: *mut u8 = core::ptr::null_mut();

const EBONY_FPGA_PATH: &str = "/plb/opb/ebc/fpga";
const EBONY_FPGA_FLASH_SEL: u32 = 0x01;
const EBONY_SMALL_FLASH_PATH: &str = "/plb/opb/ebc/small-flash";

unsafe fn ebony_flashsel_fixup() {
    let mut devp: *mut core::ffi::c_void;
    let mut reg: [u32; 3] = [0x0, 0x0, 0x80000];
    let mut fpga: *mut u8 = core::ptr::null_mut();
    let mut fpga_reg0: u8 = 0x0;

    devp = finddevice(EBONY_FPGA_PATH.as_ptr() as *const i8);
    if devp.is_null() {
        fatal(b"Couldn't locate FPGA node %s\n\r\0".as_ptr() as *const i8, EBONY_FPGA_PATH.as_ptr() as *const i8);
    }

    if getprop(devp, b"virtual-reg\0".as_ptr() as *const i8, &mut fpga as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<*mut u8>()) != core::mem::size_of::<*mut u8>() {
        fatal(b"%s has missing or invalid virtual-reg property\n\r\0".as_ptr() as *const i8, EBONY_FPGA_PATH.as_ptr() as *const i8);
    }

    fpga_reg0 = in_8(fpga);

    devp = finddevice(EBONY_SMALL_FLASH_PATH.as_ptr() as *const i8);
    if devp.is_null() {
        fatal(b"Couldn't locate small flash node %s\n\r\0".as_ptr() as *const i8, EBONY_SMALL_FLASH_PATH.as_ptr() as *const i8);
    }

    if getprop(devp, b"reg\0".as_ptr() as *const i8, reg.as_mut_ptr() as *mut core::ffi::c_void, core::mem::size_of_val(&reg)) != core::mem::size_of_val(&reg) {
        fatal(b"%s has reg property of unexpected size\n\r\0".as_ptr() as *const i8, EBONY_SMALL_FLASH_PATH.as_ptr() as *const i8);
    }

    /* Invert address bit 14 (IBM-endian) if FLASH_SEL fpga bit is set */
    if (fpga_reg0 as u32) & EBONY_FPGA_FLASH_SEL != 0 {
        reg[1] ^= 0x80000;
    }

    setprop(devp, b"reg\0".as_ptr() as *const i8, reg.as_mut_ptr() as *mut core::ffi::c_void, core::mem::size_of_val(&reg));
}

unsafe fn ebony_fixups() {
    // FIXME: sysclk should be derived by reading the FPGA registers
    let sysclk: u64 = 33000000;

    ibm440gp_fixup_clocks(sysclk, 6 * 1843200);
    ibm4xx_sdram_fixup_memsize();
    dt_fixup_mac_address_by_alias(b"ethernet0\0".as_ptr() as *const i8, ebony_mac0);
    dt_fixup_mac_address_by_alias(b"ethernet1\0".as_ptr() as *const i8, ebony_mac1);
    ibm4xx_fixup_ebc_ranges(b"/plb/opb/ebc\0".as_ptr() as *const i8);
    ebony_flashsel_fixup();
}

pub unsafe fn ebony_init(mac0: *mut core::ffi::c_void, mac1: *mut core::ffi::c_void) {
    platform_ops.fixups = Some(ebony_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    ebony_mac0 = mac0 as *mut u8;
    ebony_mac1 = mac1 as *mut u8;
    fdt_init(_dtb_start);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
