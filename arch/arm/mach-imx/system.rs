// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 1999 ARM Limited
 * Copyright (C) 2000 Deep Blue Solutions Ltd
 * Copyright 2006-2007 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 * Copyright 2009 Ilya Yanok, Emcraft Systems Ltd, yanok@emcraft.com
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub enum reboot_mode {}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_get_sys(dev_id: *const c_char, con_id: *const c_char) -> *mut clk;
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn imx_writew(value: c_int, addr: *mut c_void);
    fn mdelay(msecs: c_uint);
    fn soft_restart(addr: c_uint) -> !;
    fn of_find_compatible_node(
        from: *mut device_node,
        r#type: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void;
    fn of_node_put(node: *mut device_node);
    fn iounmap(addr: *mut c_void);
    fn readl_relaxed(addr: *mut c_void) -> c_uint;
    fn writel_relaxed(value: c_uint, addr: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn is_err(ptr: *const c_void) -> bool;
}

const L2X0_CTRL: usize = 0x100;
const L2X0_CTRL_EN: c_uint = 1;
const L310_PREFETCH_CTRL: usize = 0xF60;
const L310_PREFETCH_CTRL_DBL_LINEFILL: c_uint = 1 << 30;
const L310_PREFETCH_CTRL_INSTR_PREFETCH: c_uint = 1 << 29;
const L310_PREFETCH_CTRL_DATA_PREFETCH: c_uint = 1 << 28;
const L310_PREFETCH_CTRL_OFFSET_MASK: c_uint = 0xF;

static mut wdog_base: *mut c_void = core::ptr::null_mut();
static mut wdog_clk: *mut clk = core::ptr::null_mut();
static mut wcr_enable: c_int = 1 << 2;

/*
 * Reset the system. It is called by machine_restart().
 */
pub unsafe extern "C" fn mxc_restart(mode: reboot_mode, cmd: *const c_char) {
    let _ = mode;
    let _ = cmd;

    if wdog_base.is_null() {
        soft_restart(0);
    }

    if !is_err(wdog_clk.cast()) {
        clk_enable(wdog_clk);
    }

    /* Assert SRS signal */
    imx_writew(wcr_enable, wdog_base);
    /*
     * Due to imx6q errata ERR004346 (WDOG: WDOG SRS bit requires to be
     * written twice), we add another two writes to ensure there must be at
     * least two writes happen in the same one 32kHz clock period.  We save
     * the target check here, since the writes shouldn't be a huge burden
     * for other platforms.
     */
    imx_writew(wcr_enable, wdog_base);
    imx_writew(wcr_enable, wdog_base);

    /* wait for reset to assert... */
    mdelay(500);

    pr_err(c"%s: Watchdog reset failed to assert reset\n", c"mxc_restart\0".as_ptr().cast());

    /* delay to allow the serial port to show the message */
    mdelay(50);

    /* we'll take a jump through zero as a poor second */
    soft_restart(0);
}

pub unsafe extern "C" fn mxc_arch_reset_init(base: *mut c_void) {
    wdog_base = base;

    wdog_clk = clk_get_sys(c"imx2-wdt.0\0".as_ptr().cast(), core::ptr::null());
    if is_err(wdog_clk.cast()) {
        pr_warn(c"%s: failed to get wdog clock\n", c"mxc_arch_reset_init\0".as_ptr().cast());
    } else {
        clk_prepare(wdog_clk);
    }
}

#[cfg(CONFIG_CACHE_L2X0)]
pub unsafe extern "C" fn imx_init_l2cache() {
    let mut l2x0_base: *mut c_void;
    let np: *mut device_node;
    let mut val: c_uint;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        c"arm,pl310-cache".as_ptr().cast(),
    );
    if np.is_null() {
        return;
    }

    l2x0_base = of_iomap(np, 0);
    if l2x0_base.is_null() {
        of_node_put(np);
        return;
    }

    if (readl_relaxed(l2x0_base.add(L2X0_CTRL)) & L2X0_CTRL_EN) == 0 {
        /* Configure the L2 PREFETCH and POWER registers */
        val = readl_relaxed(l2x0_base.add(L310_PREFETCH_CTRL));
        val |= L310_PREFETCH_CTRL_DBL_LINEFILL
            | L310_PREFETCH_CTRL_INSTR_PREFETCH
            | L310_PREFETCH_CTRL_DATA_PREFETCH;

        /* Set perfetch offset to improve performance */
        val &= !L310_PREFETCH_CTRL_OFFSET_MASK;
        val |= 15;

        writel_relaxed(val, l2x0_base.add(L310_PREFETCH_CTRL));
    }

    iounmap(l2x0_base);
    of_node_put(np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
