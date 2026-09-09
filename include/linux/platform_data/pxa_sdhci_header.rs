/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/platform_data/pxa_sdhci.h
 *
 * Copyright 2010 Marvell
 *	Zhangfei Gao <zhangfei.gao@marvell.com>
 *
 * PXA Platform - SDHCI platform data definitions
 */

// C header guard: _PXA_SDHCI_H_

/* pxa specific flag */
/* Require clock free running */
pub const PXA_FLAG_ENABLE_CLOCK_GATING: u32 = 1 << 0;
/* card always wired to host, like on-chip emmc */
pub const PXA_FLAG_CARD_PERMANENT: u32 = 1 << 1;
/* Board design supports 8-bit data on SD/SDIO BUS */
pub const PXA_FLAG_SD_8_BIT_CAPABLE_SLOT: u32 = 1 << 2;

/*
 * struct pxa_sdhci_platdata() - Platform device data for PXA SDHCI
 * @flags: flags for platform requirement
 * @clk_delay_cycles:
 *	mmp2: each step is roughly 100ps, 5bits width
 *	pxa910: each step is 1ns, 4bits width
 * @clk_delay_sel: select clk_delay, used on pxa910
 *	0: choose feedback clk
 *	1: choose feedback clk + delay value
 *	2: choose internal clk
 * @clk_delay_enable: enable clk_delay or not, used on pxa910
 * @max_speed: the maximum speed supported
 * @host_caps: Standard MMC host capabilities bit field.
 * @quirks: quirks of platfrom
 * @quirks2: quirks2 of platfrom
 * @pm_caps: pm_caps of platfrom
 */
#[repr(C)]
pub struct sdhci_pxa_platdata {
    pub flags: u32,
    pub clk_delay_cycles: u32,
    pub clk_delay_sel: u32,
    pub clk_delay_enable: bool,
    pub max_speed: u32,
    pub host_caps: u32,
    pub host_caps2: u32,
    pub quirks: u32,
    pub quirks2: u32,
    pub pm_caps: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
