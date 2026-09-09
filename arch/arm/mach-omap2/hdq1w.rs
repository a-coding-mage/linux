// SPDX-License-Identifier: GPL-2.0-only
/*
 * IP block integration code for the HDQ1W/1-wire IP block
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 *
 * Based on the I2C reset code in arch/arm/mach-omap2/i2c.c by
 *     Avinash.H.M <avinashhm@ti.com>
 */

use core::ffi::{c_char, c_int, c_uint};

// Declarations supplied by the corresponding kernel dependencies.
#[repr(C)]
pub struct omap_sysc {
    pub syss_offs: u16,
}

#[repr(C)]
pub struct omap_hwmod_class {
    pub sysc: *mut omap_sysc,
}

#[repr(C)]
pub struct omap_hwmod {
    pub class: *mut omap_hwmod_class,
    pub name: *const c_char,
}

extern "C" {
    fn omap_hwmod_softreset(oh: *mut omap_hwmod);
    fn omap_hwmod_read(oh: *mut omap_hwmod, offset: u16) -> c_uint;
    fn omap_hwmod_write(value: c_uint, oh: *mut omap_hwmod, offset: u16);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

extern "C" {
    static HDQ_CTRL_STATUS_OFFSET: u16;
    static HDQ_CTRL_STATUS_CLOCKENABLE_SHIFT: u32;
    static SYSS_RESETDONE_MASK: c_uint;
    static MAX_MODULE_SOFTRESET_WAIT: c_int;
}

/// omap_hdq1w_reset - reset the OMAP HDQ1W module
/// @oh: struct omap_hwmod *
///
/// OCP soft reset the HDQ1W IP block.  Section 20.6.1.4 "HDQ1W/1-Wire
/// Software Reset" of the OMAP34xx Technical Reference Manual Revision
/// ZR (SWPU223R) does not include the rather important fact that, for the
/// reset to succeed, the HDQ1W module's internal clock gate must be
/// programmed to allow the clock to propagate to the rest of the module.
/// In this sense, it's rather similar to the I2C custom reset function.
/// Returns 0.
pub unsafe extern "C" fn omap_hdq1w_reset(oh: *mut omap_hwmod) -> c_int {
    let mut v: c_uint;
    let mut c: c_int = 0;

    /* Write to the SOFTRESET bit */
    omap_hwmod_softreset(oh);

    /* Enable the module's internal clocks */
    v = omap_hwmod_read(oh, HDQ_CTRL_STATUS_OFFSET);
    v |= 1u32 << HDQ_CTRL_STATUS_CLOCKENABLE_SHIFT;
    omap_hwmod_write(v, oh, HDQ_CTRL_STATUS_OFFSET);

    /* Poll on RESETDONE bit */
    while (c < MAX_MODULE_SOFTRESET_WAIT
        && (omap_hwmod_read(
            oh,
            (*(*oh).class).sysc.as_ref().unwrap().syss_offs,
        ) & SYSS_RESETDONE_MASK) == 0)
    {
        c += 1;
    }

    if c == MAX_MODULE_SOFTRESET_WAIT {
        pr_warn(
            b"%s: %s: softreset failed (waited %d usec)\0".as_ptr() as *const c_char,
            b"omap_hdq1w_reset\0".as_ptr(),
            (*oh).name,
            MAX_MODULE_SOFTRESET_WAIT,
        );
    } else {
        pr_debug(
            b"%s: %s: softreset in %d usec\0".as_ptr() as *const c_char,
            b"omap_hdq1w_reset\0".as_ptr(),
            (*oh).name,
            c,
        );
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
