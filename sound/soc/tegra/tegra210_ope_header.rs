/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra210_ope.h - Definitions for Tegra210 OPE driver
 *
 * Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.
 *
 */

/* C header dependencies translated as external Rust dependencies:
 * linux/regmap.h, sound/soc.h, and tegra210_peq.h.
 */

/*
 * OPE_RX registers are with respect to XBAR.
 * The data comes from XBAR to OPE
 */
pub const TEGRA210_OPE_RX_STATUS: u32 = 0xc;
pub const TEGRA210_OPE_RX_INT_STATUS: u32 = 0x10;
pub const TEGRA210_OPE_RX_INT_MASK: u32 = 0x14;
pub const TEGRA210_OPE_RX_INT_SET: u32 = 0x18;
pub const TEGRA210_OPE_RX_INT_CLEAR: u32 = 0x1c;
pub const TEGRA210_OPE_RX_CIF_CTRL: u32 = 0x20;

/*
 * OPE_TX registers are with respect to XBAR.
 * The data goes out from OPE to XBAR
 */
pub const TEGRA210_OPE_TX_STATUS: u32 = 0x4c;
pub const TEGRA210_OPE_TX_INT_STATUS: u32 = 0x50;
pub const TEGRA210_OPE_TX_INT_MASK: u32 = 0x54;
pub const TEGRA210_OPE_TX_INT_SET: u32 = 0x58;
pub const TEGRA210_OPE_TX_INT_CLEAR: u32 = 0x5c;
pub const TEGRA210_OPE_TX_CIF_CTRL: u32 = 0x60;

/* OPE Gloabal registers */
pub const TEGRA210_OPE_ENABLE: u32 = 0x80;
pub const TEGRA210_OPE_SOFT_RESET: u32 = 0x84;
pub const TEGRA210_OPE_CG: u32 = 0x88;
pub const TEGRA210_OPE_STATUS: u32 = 0x8c;
pub const TEGRA210_OPE_INT_STATUS: u32 = 0x90;
pub const TEGRA210_OPE_DIR: u32 = 0x94;

/* Fields for TEGRA210_OPE_ENABLE */
pub const TEGRA210_OPE_EN_SHIFT: u32 = 0;
pub const TEGRA210_OPE_EN: u32 = 1u32 << TEGRA210_OPE_EN_SHIFT;

/* Fields for TEGRA210_OPE_SOFT_RESET */
pub const TEGRA210_OPE_SOFT_RESET_SHIFT: u32 = 0;
pub const TEGRA210_OPE_SOFT_RESET_EN: u32 = 1u32 << TEGRA210_OPE_SOFT_RESET_SHIFT;

pub const TEGRA210_OPE_DIR_SHIFT: u32 = 0;

#[repr(C)]
pub struct tegra210_ope {
    pub regmap: *mut regmap,
    pub peq_regmap: *mut regmap,
    pub mbdrc_regmap: *mut regmap,
    pub peq_biquad_gains: [u32; TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH],
    pub peq_biquad_shifts: [u32; TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH],
    pub data_dir: ::core::ffi::c_uint,
}

/* Extension of soc_bytes structure defined in sound/soc.h */
#[repr(C)]
pub struct tegra_soc_bytes {
    pub soc: soc_bytes,
    pub shift: u32, /* Used as offset for AHUB RAM related programing */
}

/* Utility structures for using mixer control of type snd_soc_bytes */
#[macro_export]
macro_rules! TEGRA_SOC_BYTES_EXT {
    (
        $xname:expr,
        $xbase:expr,
        $xregs:expr,
        $xshift:expr,
        $xmask:expr,
        $xhandler_get:expr,
        $xhandler_put:expr,
        $xinfo:expr
    ) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: $xinfo,
            get: $xhandler_get,
            put: $xhandler_put,
            private_value: (&tegra_soc_bytes {
                soc: soc_bytes {
                    base: $xbase,
                    num_regs: $xregs,
                    mask: $xmask,
                },
                shift: $xshift,
            } as *const tegra_soc_bytes) as ::core::ffi::c_ulong,
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
