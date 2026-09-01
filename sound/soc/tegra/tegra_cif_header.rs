/* SPDX-License-Identifier: GPL-2.0-only
 * SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION. All rights reserved.
 *
 * tegra_cif.h - TEGRA Audio CIF Programming
 *
 */

// C dependency: <linux/regmap.h>

pub const TEGRA_ACIF_CTRL_FIFO_TH_SHIFT: ::core::ffi::c_uint = 24;
pub const TEGRA_ACIF_CTRL_AUDIO_CH_SHIFT: ::core::ffi::c_uint = 20;
pub const TEGRA_ACIF_CTRL_CLIENT_CH_SHIFT: ::core::ffi::c_uint = 16;
pub const TEGRA_ACIF_CTRL_AUDIO_BITS_SHIFT: ::core::ffi::c_uint = 12;
pub const TEGRA_ACIF_CTRL_CLIENT_BITS_SHIFT: ::core::ffi::c_uint = 8;
pub const TEGRA_ACIF_CTRL_EXPAND_SHIFT: ::core::ffi::c_uint = 6;
pub const TEGRA_ACIF_CTRL_STEREO_CONV_SHIFT: ::core::ffi::c_uint = 4;
pub const TEGRA_ACIF_CTRL_REPLICATE_SHIFT: ::core::ffi::c_uint = 3;
pub const TEGRA_ACIF_CTRL_TRUNCATE_SHIFT: ::core::ffi::c_uint = 1;
pub const TEGRA_ACIF_CTRL_MONO_CONV_SHIFT: ::core::ffi::c_uint = 0;

pub const TEGRA264_ACIF_CTRL_AUDIO_BITS_SHIFT: ::core::ffi::c_uint = 11;
pub const TEGRA264_ACIF_CTRL_CLIENT_CH_SHIFT: ::core::ffi::c_uint = 14;
pub const TEGRA264_ACIF_CTRL_AUDIO_CH_SHIFT: ::core::ffi::c_uint = 19;

/* AUDIO/CLIENT_BITS values */
pub const TEGRA_ACIF_BITS_8: ::core::ffi::c_uint = 1;
pub const TEGRA_ACIF_BITS_16: ::core::ffi::c_uint = 3;
pub const TEGRA_ACIF_BITS_24: ::core::ffi::c_uint = 5;
pub const TEGRA_ACIF_BITS_32: ::core::ffi::c_uint = 7;

pub const TEGRA_ACIF_UPDATE_MASK: ::core::ffi::c_uint = 0x3ffffffb;

#[repr(C)]
pub struct regmap {
    _data: [u8; 0],
}

unsafe extern "C" {
    pub fn regmap_update_bits(
        regmap: *mut regmap,
        reg: ::core::ffi::c_uint,
        mask: ::core::ffi::c_uint,
        val: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct tegra_cif_conf {
    pub threshold: ::core::ffi::c_uint,
    pub audio_ch: ::core::ffi::c_uint,
    pub client_ch: ::core::ffi::c_uint,
    pub audio_bits: ::core::ffi::c_uint,
    pub client_bits: ::core::ffi::c_uint,
    pub expand: ::core::ffi::c_uint,
    pub stereo_conv: ::core::ffi::c_uint,
    pub replicate: ::core::ffi::c_uint,
    pub truncate: ::core::ffi::c_uint,
    pub mono_conv: ::core::ffi::c_uint,
}

#[inline]
pub unsafe fn tegra_set_cif(
    regmap: *mut regmap,
    reg: ::core::ffi::c_uint,
    conf: *mut tegra_cif_conf,
) {
    let value: ::core::ffi::c_uint = ((*conf).threshold << TEGRA_ACIF_CTRL_FIFO_TH_SHIFT)
        | (((*conf).audio_ch.wrapping_sub(1)) << TEGRA_ACIF_CTRL_AUDIO_CH_SHIFT)
        | (((*conf).client_ch.wrapping_sub(1)) << TEGRA_ACIF_CTRL_CLIENT_CH_SHIFT)
        | ((*conf).audio_bits << TEGRA_ACIF_CTRL_AUDIO_BITS_SHIFT)
        | ((*conf).client_bits << TEGRA_ACIF_CTRL_CLIENT_BITS_SHIFT)
        | ((*conf).expand << TEGRA_ACIF_CTRL_EXPAND_SHIFT)
        | ((*conf).stereo_conv << TEGRA_ACIF_CTRL_STEREO_CONV_SHIFT)
        | ((*conf).replicate << TEGRA_ACIF_CTRL_REPLICATE_SHIFT)
        | ((*conf).truncate << TEGRA_ACIF_CTRL_TRUNCATE_SHIFT)
        | ((*conf).mono_conv << TEGRA_ACIF_CTRL_MONO_CONV_SHIFT);

    regmap_update_bits(regmap, reg, TEGRA_ACIF_UPDATE_MASK, value);
}

#[inline]
pub unsafe fn tegra264_set_cif(
    regmap: *mut regmap,
    reg: ::core::ffi::c_uint,
    conf: *mut tegra_cif_conf,
) {
    let value: ::core::ffi::c_uint = ((*conf).threshold << TEGRA_ACIF_CTRL_FIFO_TH_SHIFT)
        | (((*conf).audio_ch.wrapping_sub(1)) << TEGRA264_ACIF_CTRL_AUDIO_CH_SHIFT)
        | (((*conf).client_ch.wrapping_sub(1)) << TEGRA264_ACIF_CTRL_CLIENT_CH_SHIFT)
        | ((*conf).audio_bits << TEGRA264_ACIF_CTRL_AUDIO_BITS_SHIFT)
        | ((*conf).client_bits << TEGRA_ACIF_CTRL_CLIENT_BITS_SHIFT)
        | ((*conf).expand << TEGRA_ACIF_CTRL_EXPAND_SHIFT)
        | ((*conf).stereo_conv << TEGRA_ACIF_CTRL_STEREO_CONV_SHIFT)
        | ((*conf).replicate << TEGRA_ACIF_CTRL_REPLICATE_SHIFT)
        | ((*conf).truncate << TEGRA_ACIF_CTRL_TRUNCATE_SHIFT)
        | ((*conf).mono_conv << TEGRA_ACIF_CTRL_MONO_CONV_SHIFT);

    regmap_update_bits(regmap, reg, TEGRA_ACIF_UPDATE_MASK, value);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
