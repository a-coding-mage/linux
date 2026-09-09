// SPDX-License-Identifier: GPL-2.0-only
/*
 * Utility to set the DAVINCI MUX register from a table in mux.h
 *
 * Author: Vladimir Barinov, MontaVista Software, Inc. <source@mvista.com>
 *
 * Based on linux/arch/arm/plat-omap/mux.c:
 * Copyright (C) 2003 - 2005 Nokia Corporation
 *
 * Written by Tony Lindgren
 *
 * 2007 (c) MontaVista Software, Inc.
 *
 * Copyright (C) 2008 Texas Instruments.
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies: linux/io.h, linux/module.h, linux/spinlock.h, mux.h, common.h

use core::ffi::{c_char, c_void};

extern "C" {
    static mut davinci_soc_info: DavinciSocInfo;
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn __raw_readl(addr: *mut c_void) -> u32;
    fn __raw_writel(value: u32, addr: *mut c_void);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: c_ulong);
    fn dump_stack();
    fn pr_err(format: *const c_char, ...);
    fn pr_warn(format: *const c_char, ...);
    fn warn_on(condition: bool) -> bool;
}

type c_ulong = usize;

#[repr(C)]
pub struct Spinlock {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct MuxConfig {
    pub name: *const c_char,
    pub mux_reg: usize,
    pub mask: u32,
    pub mask_offset: u32,
    pub mode: u32,
    pub mux_reg_name: *const c_char,
    pub debug: bool,
}

#[repr(C)]
pub struct DavinciSocInfo {
    pub pinmux_pins: *const MuxConfig,
    pub pinmux_pins_num: usize,
    pub pinmux_base: usize,
}

static mut pinmux_base: *mut c_void = core::ptr::null_mut();

/*
 * Sets the DAVINCI MUX register based on the table
 */
pub unsafe fn davinci_cfg_reg(index: usize) -> i32 {
    static mut mux_spin_lock: Spinlock = Spinlock { _opaque: [] };
    let soc_info: *mut DavinciSocInfo = &raw mut davinci_soc_info;
    let mut flags: c_ulong = 0;
    let mut reg_orig: u32 = 0;
    let mut reg: u32 = 0;
    let mut mask: u32;
    let mut warn: u32 = 0;

    if warn_on((*soc_info).pinmux_pins.is_null()) {
        return -19; // -ENODEV
    }

    if pinmux_base.is_null() {
        pinmux_base = ioremap((*soc_info).pinmux_base, 0x1000); // SZ_4K
        if warn_on(pinmux_base.is_null()) {
            return -12; // -ENOMEM
        }
    }

    if index >= (*soc_info).pinmux_pins_num {
        pr_err(b"Invalid pin mux index: %lu (%lu)\n\0".as_ptr() as *const c_char,
               index, (*soc_info).pinmux_pins_num);
        dump_stack();
        return -19; // -ENODEV
    }

    let cfg: *const MuxConfig = (*soc_info).pinmux_pins.add(index);

    if (*cfg).name.is_null() {
        pr_err(b"No entry for the specified index\n\0".as_ptr() as *const c_char);
        return -19; // -ENODEV
    }

    /* Update the mux register in question */
    if (*cfg).mask != 0 {
        let mut tmp1: u32;
        let mut tmp2: u32;

        spin_lock_irqsave(&raw mut mux_spin_lock, &raw mut flags);
        reg_orig = __raw_readl(pinmux_base.add((*cfg).mux_reg));

        mask = (*cfg).mask << (*cfg).mask_offset;
        tmp1 = reg_orig & mask;
        reg = reg_orig & !mask;

        tmp2 = (*cfg).mode << (*cfg).mask_offset;
        reg |= tmp2;

        if tmp1 != tmp2 {
            warn = 1;
        }

        __raw_writel(reg, pinmux_base.add((*cfg).mux_reg));
        spin_unlock_irqrestore(&raw mut mux_spin_lock, flags);
    }

    if warn != 0 {
        // CONFIG_DAVINCI_MUX_WARNINGS controls this block.
        // pr_warn("initialized %s\n", cfg->name);
    }

    // CONFIG_DAVINCI_MUX_DEBUG controls this block.
    // if (*cfg).debug || warn != 0 {
    //     pr_warn("Setting register %s\n", (*cfg).name);
    //     pr_warn("   %s (0x%08x) = 0x%08x -> 0x%08x\n",
    //             (*cfg).mux_reg_name, (*cfg).mux_reg, reg_orig, reg);
    // }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
