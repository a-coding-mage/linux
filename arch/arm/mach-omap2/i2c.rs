// SPDX-License-Identifier: GPL-2.0-only
/*
 * Helper module for board specific I2C bus registration
 *
 * Copyright (C) 2009 Nokia Corporation.
 */

// Dependencies supplied by the corresponding platform headers.

const I2C_EN: u32 = 1u32 << 15;
const OMAP2_I2C_CON_OFFSET: u16 = 0x24;
const OMAP4_I2C_CON_OFFSET: u16 = 0xA4;
const MAX_OMAP_I2C_HWMOD_NAME_LEN: usize = 16;

extern "C" {
    fn soc_is_omap24xx() -> bool;
    fn soc_is_omap34xx() -> bool;
    fn soc_is_am35xx() -> bool;
    fn omap_hwmod_read(oh: *mut omap_hwmod, reg: u16) -> u32;
    fn omap_hwmod_write(v: u32, oh: *mut omap_hwmod, reg: u16);
    fn omap_hwmod_softreset(oh: *mut omap_hwmod);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn udelay(usec: u32);
}

#[repr(C)]
pub struct omap_hwmod_sysc {
    pub syss_offs: u16,
}

#[repr(C)]
pub struct omap_hwmod_class {
    pub sysc: *mut omap_hwmod_sysc,
}

#[repr(C)]
pub struct omap_hwmod {
    pub r#class: *mut omap_hwmod_class,
    pub name: *const core::ffi::c_char,
}

const SYSS_RESETDONE_MASK: u32 = 1 << 0;
const MAX_MODULE_SOFTRESET_WAIT: i32 = 10000;

/**
 * omap_i2c_reset - reset the omap i2c module.
 * @oh: struct omap_hwmod *
 *
 * The i2c moudle in omap2, omap3 had a special sequence to reset. The
 * sequence is:
 * - Disable the I2C.
 * - Write to SOFTRESET bit.
 * - Enable the I2C.
 * - Poll on the RESETDONE bit.
 * The sequence is implemented in below function. This is called for 2420,
 * 2430 and omap3.
 */
pub unsafe fn omap_i2c_reset(oh: *mut omap_hwmod) -> i32 {
    let mut v: u32;
    let i2c_con: u16;
    let mut c: i32 = 0;

    if soc_is_omap24xx() || soc_is_omap34xx() || soc_is_am35xx() {
        i2c_con = OMAP2_I2C_CON_OFFSET;
    } else {
        i2c_con = OMAP4_I2C_CON_OFFSET;
    }

    /* Disable I2C */
    v = omap_hwmod_read(oh, i2c_con);
    v &= !I2C_EN;
    omap_hwmod_write(v, oh, i2c_con);

    /* Write to the SOFTRESET bit */
    omap_hwmod_softreset(oh);

    /* Enable I2C */
    v = omap_hwmod_read(oh, i2c_con);
    v |= I2C_EN;
    omap_hwmod_write(v, oh, i2c_con);

    /* Poll on RESETDONE bit */
    while (c < MAX_MODULE_SOFTRESET_WAIT
        && (omap_hwmod_read(oh, (*(*oh).r#class).sysc.as_ref().unwrap().syss_offs)
            & SYSS_RESETDONE_MASK) == 0)
    {
        udelay(1);
        c += 1;
    }

    if c == MAX_MODULE_SOFTRESET_WAIT {
        // Corresponds to: pr_warn("%s: %s: softreset failed (waited %d usec)\n", ...)
        pr_warn(core::ptr::null(), core::ptr::null(), (*oh).name, MAX_MODULE_SOFTRESET_WAIT);
    } else {
        // Corresponds to: pr_debug("%s: %s: softreset in %d usec\n", ...)
        pr_debug(core::ptr::null(), core::ptr::null(), (*oh).name, c);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
