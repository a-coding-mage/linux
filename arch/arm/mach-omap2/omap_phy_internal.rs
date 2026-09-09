// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file configures the internal USB PHY in OMAP4430. Used
 * with TWL6030 transceiver and MUSB on OMAP4430.
 *
 * Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com
 * Author: Hema HK <hemahk@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const CONTROL_DEV_CONF: usize = 0x300;
const PHY_PD: u32 = 0x1;

extern "C" {
    fn cpu_is_omap44xx() -> bool;
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn iounmap(addr: *mut core::ffi::c_void);
}

// OMAP443X_SCM_BASE and SZ_1K are supplied by the platform headers.
extern "C" {
    static OMAP443X_SCM_BASE: usize;
    static SZ_1K: usize;
}

/**
 * omap4430_phy_power_down: disable MUSB PHY during early init
 *
 * OMAP4 MUSB PHY module is enabled by default on reset, but this will
 * prevent core retention if not disabled by SW. USB driver will
 * later on enable this, once and if the driver needs it.
 */
#[allow(non_snake_case)]
unsafe fn omap4430_phy_power_down() -> i32 {
    let mut ctrl_base: *mut core::ffi::c_void;

    if !cpu_is_omap44xx() {
        return 0;
    }

    ctrl_base = ioremap(OMAP443X_SCM_BASE, SZ_1K);
    if ctrl_base.is_null() {
        // pr_err("control module ioremap failed\n");
        return -12; // -ENOMEM
    }

    /* Power down the phy */
    writel_relaxed(
        PHY_PD,
        (ctrl_base as *mut u8).add(CONTROL_DEV_CONF) as *mut core::ffi::c_void,
    );

    iounmap(ctrl_base);

    0
}

// omap_early_initcall(omap4430_phy_power_down);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
