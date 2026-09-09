// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap2/mcbsp.c
 *
 * Copyright (C) 2008 Instituto Nokia de Tecnologia
 * Contact: Eduardo Valentin <eduardo.valentin@indt.org.br>
 *
 * Multichannel mode not supported.
 */

// Kernel dependencies supplied by the surrounding repository are intentionally
// not redefined here.

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_mcbsp_platform_data {
    pub force_ick_on: Option<unsafe extern "C" fn(*mut clk, bool) -> i32>,
}

unsafe extern "C" {
    fn omap2_clk_deny_idle(clk: *mut clk) -> i32;
    fn omap2_clk_allow_idle(clk: *mut clk) -> i32;
}

/*
 * FIXME: Find a mechanism to enable/disable runtime the McBSP ICLK autoidle.
 * Sidetone needs non-gated ICLK and sidetone autoidle is broken.
 */

unsafe extern "C" fn omap3_mcbsp_force_ick_on(
    clk: *mut clk,
    force_on: bool,
) -> i32 {
    if clk.is_null() {
        return 0;
    }

    if force_on {
        omap2_clk_deny_idle(clk)
    } else {
        omap2_clk_allow_idle(clk)
    }
}

pub unsafe extern "C" fn omap3_mcbsp_init_pdata_callback(
    pdata: *mut omap_mcbsp_platform_data,
) {
    if pdata.is_null() {
        return;
    }

    (*pdata).force_ick_on = Some(omap3_mcbsp_force_ick_on);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
