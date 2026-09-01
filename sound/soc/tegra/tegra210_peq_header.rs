/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra210_peq.h - Definitions for Tegra210 PEQ driver
 *
 * Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.
 *
 */

/* C header dependencies:
 * #include <linux/platform_device.h>
 * #include <linux/regmap.h>
 * #include <sound/soc.h>
 */

/* Register offsets from PEQ base */
pub const TEGRA210_PEQ_SOFT_RESET: u32 = 0x0;
pub const TEGRA210_PEQ_CG: u32 = 0x4;
pub const TEGRA210_PEQ_STATUS: u32 = 0x8;
pub const TEGRA210_PEQ_CFG: u32 = 0xc;
pub const TEGRA210_PEQ_CFG_RAM_CTRL: u32 = 0x10;
pub const TEGRA210_PEQ_CFG_RAM_DATA: u32 = 0x14;
pub const TEGRA210_PEQ_CFG_RAM_SHIFT_CTRL: u32 = 0x18;
pub const TEGRA210_PEQ_CFG_RAM_SHIFT_DATA: u32 = 0x1c;

/* Fields in TEGRA210_PEQ_CFG */
pub const TEGRA210_PEQ_CFG_BIQUAD_STAGES_SHIFT: u32 = 2;
pub const TEGRA210_PEQ_CFG_BIQUAD_STAGES_MASK: u32 =
    0xf << TEGRA210_PEQ_CFG_BIQUAD_STAGES_SHIFT;

pub const TEGRA210_PEQ_CFG_MODE_SHIFT: u32 = 0;
pub const TEGRA210_PEQ_CFG_MODE_MASK: u32 = 0x1 << TEGRA210_PEQ_CFG_MODE_SHIFT;

pub const TEGRA210_PEQ_RAM_CTRL_RW_READ: u32 = 0;
pub const TEGRA210_PEQ_RAM_CTRL_RW_WRITE: u32 = 1 << 14;
pub const TEGRA210_PEQ_RAM_CTRL_ADDR_INIT_EN: u32 = 1 << 13;
pub const TEGRA210_PEQ_RAM_CTRL_SEQ_ACCESS_EN: u32 = 1 << 12;
pub const TEGRA210_PEQ_RAM_CTRL_RAM_ADDR_MASK: u32 = 0x1ff;

/* PEQ register definition ends here */
pub const TEGRA210_PEQ_MAX_BIQUAD_STAGES: u32 = 12;

pub const TEGRA210_PEQ_MAX_CHANNELS: u32 = 8;

pub const TEGRA210_PEQ_BIQUAD_INIT_STAGE: u32 = 5;

pub const TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH: u32 =
    2 + TEGRA210_PEQ_MAX_BIQUAD_STAGES * 5;
pub const TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH: u32 =
    2 + TEGRA210_PEQ_MAX_BIQUAD_STAGES;

unsafe extern "C" {
    pub fn tegra210_peq_regmap_init(pdev: *mut platform_device) -> core::ffi::c_int;
    pub fn tegra210_peq_component_init(cmpnt: *mut snd_soc_component) -> core::ffi::c_int;
    pub fn tegra210_peq_restore(
        regmap: *mut regmap,
        biquad_gains: *mut u32,
        biquad_shifts: *mut u32,
    );
    pub fn tegra210_peq_save(
        regmap: *mut regmap,
        biquad_gains: *mut u32,
        biquad_shifts: *mut u32,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
