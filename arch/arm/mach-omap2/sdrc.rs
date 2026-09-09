// SPDX-License-Identifier: GPL-2.0-only
/*
 * SMS/SDRC (SDRAM controller) common code for OMAP2/3
 *
 * Copyright (C) 2005, 2008 Texas Instruments Inc.
 * Copyright (C) 2005, 2008 Nokia Corporation
 *
 * Tony Lindgren <tony@atomide.com>
 * Paul Walmsley
 * Richard Woodruff <r-woodruff2@ti.com>
 */

// Dependencies supplied by the surrounding translation unit:
// common.h, clock.h, and sdrc.h

use core::ffi::c_void;

extern "C" {
    fn sms_read_reg(reg: u32) -> u32;
    fn sms_write_reg(value: u32, reg: u32);
    fn sdrc_read_reg(reg: u32) -> u32;
    fn sdrc_write_reg(value: u32, reg: u32);
}

// `struct omap_sdrc_params` is declared by sdrc.h.
#[repr(C)]
pub struct omap_sdrc_params {
    _private: [u8; 0],
}

// Register constants are supplied by sdrc.h.
extern "C" {
    static SMS_SYSCONFIG: u32;
    static SDRC_SYSCONFIG: u32;
    static SDRC_POWER: u32;
}

static mut sdrc_init_params_cs0: *mut omap_sdrc_params = core::ptr::null_mut();
static mut sdrc_init_params_cs1: *mut omap_sdrc_params = core::ptr::null_mut();

pub static mut omap2_sdrc_base: *mut c_void = core::ptr::null_mut();
pub static mut omap2_sms_base: *mut c_void = core::ptr::null_mut();

#[repr(C)]
struct omap2_sms_regs {
    sms_sysconfig: u32,
}

static mut sms_context: omap2_sms_regs = omap2_sms_regs { sms_sysconfig: 0 };

/* SDRC_POWER register bits */
const SDRC_POWER_EXTCLKDIS_SHIFT: u32 = 3;
const SDRC_POWER_PWDENA_SHIFT: u32 = 2;
const SDRC_POWER_PAGEPOLICY_SHIFT: u32 = 0;

/**
 * omap2_sms_save_context - Save SMS registers
 *
 * Save SMS registers that need to be restored after off mode.
 */
unsafe fn omap2_sms_save_context() {
    sms_context.sms_sysconfig = sms_read_reg(SMS_SYSCONFIG);
}

/**
 * omap2_sms_restore_context - Restore SMS registers
 *
 * Restore SMS registers that need to be Restored after off mode.
 */
pub unsafe fn omap2_sms_restore_context() {
    sms_write_reg(sms_context.sms_sysconfig, SMS_SYSCONFIG);
}

pub unsafe fn omap2_set_globals_sdrc(sdrc: *mut c_void, sms: *mut c_void) {
    omap2_sdrc_base = sdrc;
    omap2_sms_base = sms;
}

/**
 * omap2_sdrc_init - initialize SMS, SDRC devices on boot
 * @sdrc_cs[01]: pointers to a null-terminated list of struct omap_sdrc_params
 *  Support for 2 chip selects timings
 *
 * Turn on smart idle modes for SDRAM scheduler and controller.
 * Program a known-good configuration for the SDRC to deal with buggy
 * bootloaders.
 */
pub unsafe fn omap2_sdrc_init(
    sdrc_cs0: *mut omap_sdrc_params,
    sdrc_cs1: *mut omap_sdrc_params,
) {
    let mut l: u32;

    l = sms_read_reg(SMS_SYSCONFIG);
    l &= !(0x3u32 << 3);
    l |= 0x2u32 << 3;
    sms_write_reg(l, SMS_SYSCONFIG);

    l = sdrc_read_reg(SDRC_SYSCONFIG);
    l &= !(0x3u32 << 3);
    l |= 0x2u32 << 3;
    sdrc_write_reg(l, SDRC_SYSCONFIG);

    sdrc_init_params_cs0 = sdrc_cs0;
    sdrc_init_params_cs1 = sdrc_cs1;

    /* XXX Enable SRFRONIDLEREQ here also? */
    /*
     * PWDENA should not be set due to 34xx erratum 1.150 - PWDENA
     * can cause random memory corruption
     */
    l = (1u32 << SDRC_POWER_EXTCLKDIS_SHIFT)
        | (1u32 << SDRC_POWER_PAGEPOLICY_SHIFT);
    sdrc_write_reg(l, SDRC_POWER);
    omap2_sms_save_context();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
