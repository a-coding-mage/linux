// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/arch/arm/plat-omap/ocpi.c
 *
 * Minimal OCP bus support for omap16xx
 *
 * Copyright (C) 2003 - 2005 Nokia Corporation
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Written by Tony Lindgren <tony@atomide.com>
 *
 * Modified for clock framework by Paul Mundt <paul.mundt@nokia.com>.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_void};

const OCPI_BASE: usize = 0xfffec320;
const OCPI_FAULT: usize = OCPI_BASE + 0x00;
const OCPI_CMD_FAULT: usize = OCPI_BASE + 0x04;
const OCPI_SINT0: usize = OCPI_BASE + 0x08;
const OCPI_TABORT: usize = OCPI_BASE + 0x0c;
const OCPI_SINT1: usize = OCPI_BASE + 0x10;
const OCPI_PROT: usize = OCPI_BASE + 0x14;
const OCPI_SEC: usize = OCPI_BASE + 0x18;

/* USB OHCI OCPI access error registers */
const HOSTUEADDR: usize = 0xfffba0e0;
const HOSTUESTATUS: usize = 0xfffba0e4;

const ENODEV: c_int = 19;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn cpu_is_omap16xx() -> bool;
    fn omap_readl(addr: usize) -> u32;
    fn omap_writel(value: u32, addr: usize);
    fn clk_get(dev: *mut c_void, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *mut clk) -> bool;
    fn PTR_ERR(ptr: *mut clk) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
}

static mut ocpi_ck: *mut clk = core::ptr::null_mut();

/*
 * Enables device access to OMAP buses via the OCPI bridge
 */
#[no_mangle]
pub unsafe extern "C" fn ocpi_enable() -> c_int {
    let mut val: u32;

    if !cpu_is_omap16xx() {
        return -ENODEV;
    }

    /* Enable access for OHCI in OCPI */
    val = omap_readl(OCPI_PROT);
    val &= !0xff;
    /* val &= (1 << 0);    Allow access only to EMIFS */
    omap_writel(val, OCPI_PROT);

    val = omap_readl(OCPI_SEC);
    val &= !0xff;
    omap_writel(val, OCPI_SEC);

    0
}

unsafe fn omap_ocpi_init() -> c_int {
    if !cpu_is_omap16xx() {
        return -ENODEV;
    }

    ocpi_ck = clk_get(core::ptr::null_mut(), b"l3_ocpi_ck\0".as_ptr() as *const c_char);
    if IS_ERR(ocpi_ck) {
        return PTR_ERR(ocpi_ck);
    }

    clk_prepare_enable(ocpi_ck);
    ocpi_enable();
    // pr_info("OMAP OCPI interconnect driver loaded\n");

    0
}

unsafe fn omap_ocpi_exit() {
    /* REVISIT: Disable OCPI */

    if !cpu_is_omap16xx() {
        return;
    }

    clk_disable_unprepare(ocpi_ck);
    clk_put(ocpi_ck);
}

// MODULE_AUTHOR("Tony Lindgren <tony@atomide.com>");
// MODULE_DESCRIPTION("OMAP OCPI bus controller module");
// MODULE_LICENSE("GPL");
// module_init(omap_ocpi_init);
// module_exit(omap_ocpi_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
