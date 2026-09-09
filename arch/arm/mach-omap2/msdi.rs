// SPDX-License-Identifier: GPL-2.0-only
/*
 * MSDI IP block reset
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 *
 * XXX What about pad muxing?
 */

// Dependencies supplied by the surrounding kernel translation.

const MSDI_CON_OFFSET: u32 = 0x0c;

const MSDI_CON_POW_MASK: u16 = 1 << 11;
const MSDI_CON_CLKD_MASK: u16 = 0x3f << 0;
const MSDI_CON_CLKD_SHIFT: u32 = 0;

const MSDI_TARGET_RESET_CLKD: u16 = 0x3ff;

#[repr(C)]
pub struct OmapHwmodClassSysc {
    pub syss_offs: u32,
}

#[repr(C)]
pub struct OmapHwmodClass {
    pub sysc: *mut OmapHwmodClassSysc,
}

#[repr(C)]
pub struct OmapHwmod {
    pub class: *mut OmapHwmodClass,
    pub name: *const core::ffi::c_char,
}

unsafe extern "C" {
    fn omap_hwmod_softreset(oh: *mut OmapHwmod);
    fn omap_hwmod_write(v: u16, oh: *mut OmapHwmod, reg: u32);
    fn omap_hwmod_read(oh: *mut OmapHwmod, reg: u32) -> u16;
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
}

// Supplied by the OMAP hwmod dependencies.
unsafe extern "C" {
    static SYSS_RESETDONE_MASK: u16;
    static MAX_MODULE_SOFTRESET_WAIT: i32;
}

/// Reset the MSDI IP block.
///
/// The MSDI IP block on OMAP2420 has to have both the POW and CLKD fields set
/// inside its CON register for a reset to complete successfully. This is not
/// documented in the TRM. For CLKD, the value resulting in the lowest possible
/// clock rate is used, to attempt to avoid disturbing any cards.
pub unsafe fn omap_msdi_reset(oh: *mut OmapHwmod) -> i32 {
    let mut v: u16 = 0;
    let mut c: i32 = 0;

    // Write to the SOFTRESET bit.
    omap_hwmod_softreset(oh);

    // Enable the MSDI core and internal clock.
    v |= MSDI_CON_POW_MASK;
    v |= MSDI_TARGET_RESET_CLKD << MSDI_CON_CLKD_SHIFT;
    omap_hwmod_write(v, oh, MSDI_CON_OFFSET);

    // Poll on RESETDONE bit.
    while c < MAX_MODULE_SOFTRESET_WAIT
        && (omap_hwmod_read(
            oh,
            (*(*oh).class).sysc.as_ref().unwrap().syss_offs,
        ) & SYSS_RESETDONE_MASK)
            == 0
    {
        c += 1;
    }

    if c == MAX_MODULE_SOFTRESET_WAIT {
        pr_warn(
            c"%s: %s: softreset failed (waited %d usec)\n",
            c"omap_msdi_reset\0".as_ptr(),
            (*oh).name,
            MAX_MODULE_SOFTRESET_WAIT,
        );
    } else {
        pr_debug(
            c"%s: %s: softreset in %d usec\n",
            c"omap_msdi_reset\0".as_ptr(),
            (*oh).name,
            c,
        );
    }

    // Disable the MSDI internal clock.
    v &= !MSDI_CON_CLKD_MASK;
    omap_hwmod_write(v, oh, MSDI_CON_OFFSET);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
