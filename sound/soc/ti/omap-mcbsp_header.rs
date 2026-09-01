/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap-mcbsp.h
 *
 * Copyright (C) 2008 Nokia Corporation
 *
 * Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
 *          Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

/* Dependency intent from C header: #include <sound/dmaengine_pcm.h> */

use core::ffi::c_int;

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

/* Source clocks for McBSP sample rate generator */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum omap_mcbsp_clksrg_clk {
    OMAP_MCBSP_SYSCLK_CLKS_FCLK = 0, /* Internal FCLK */
    OMAP_MCBSP_SYSCLK_CLKS_EXT = 1,  /* External CLKS pin */
    OMAP_MCBSP_SYSCLK_CLK = 2,       /* Internal ICLK */
    OMAP_MCBSP_SYSCLK_CLKX_EXT = 3,  /* External CLKX pin */
    OMAP_MCBSP_SYSCLK_CLKR_EXT = 4,  /* External CLKR pin */
}

/* McBSP dividers */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum omap_mcbsp_div {
    OMAP_MCBSP_CLKGDV = 0, /* Sample rate generator divider */
}

unsafe extern "C" {
    pub fn omap_mcbsp_st_add_controls(
        rtd: *mut snd_soc_pcm_runtime,
        port_id: c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
