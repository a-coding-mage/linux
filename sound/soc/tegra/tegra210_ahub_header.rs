// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra210_ahub.h - TEGRA210 AHUB
 *
 * Copyright (c) 2020-2025, NVIDIA CORPORATION.  All rights reserved.
 *
 */

/* Tegra210 specific */
pub const TEGRA210_XBAR_PART1_RX: u32 = 0x200;
pub const TEGRA210_XBAR_PART2_RX: u32 = 0x400;
pub const TEGRA210_XBAR_RX_STRIDE: u32 = 0x4;
pub const TEGRA210_XBAR_AUDIO_RX_COUNT: u32 = 90;
pub const TEGRA210_XBAR_REG_MASK_0: u32 = 0xf1f03ff;
pub const TEGRA210_XBAR_REG_MASK_1: u32 = 0x3f30031f;
pub const TEGRA210_XBAR_REG_MASK_2: u32 = 0xff1cf313;
pub const TEGRA210_XBAR_REG_MASK_3: u32 = 0x0;
pub const TEGRA210_XBAR_UPDATE_MAX_REG: usize = 3;

/* Tegra186 specific */
pub const TEGRA186_XBAR_PART3_RX: u32 = 0x600;
pub const TEGRA186_XBAR_AUDIO_RX_COUNT: u32 = 115;
pub const TEGRA186_XBAR_REG_MASK_0: u32 = 0xf3fffff;
pub const TEGRA186_XBAR_REG_MASK_1: u32 = 0x3f310f1f;
pub const TEGRA186_XBAR_REG_MASK_2: u32 = 0xff3cf311;
pub const TEGRA186_XBAR_REG_MASK_3: u32 = 0x3f0f00ff;
pub const TEGRA186_XBAR_UPDATE_MAX_REG: usize = 4;

/* Tegra264 specific */
pub const TEGRA264_XBAR_PART1_RX: u32 = 0x1000;
pub const TEGRA264_XBAR_PART2_RX: u32 = 0x2000;
pub const TEGRA264_XBAR_PART3_RX: u32 = 0x3000;
pub const TEGRA264_XBAR_PART4_RX: u32 = 0x4000;
pub const TEGRA264_XBAR_PART0_ADX6_RX1: u32 = 0x224;
pub const TEGRA264_XBAR_AUDIO_RX_COUNT: u32 = (TEGRA264_XBAR_PART0_ADX6_RX1 / 4) + 1;
pub const TEGRA264_XBAR_REG_MASK_0: u32 = 0xfffffff;
pub const TEGRA264_XBAR_REG_MASK_1: u32 = 0x3f013f1f;
pub const TEGRA264_XBAR_REG_MASK_2: u32 = 0xff3c0301;
pub const TEGRA264_XBAR_REG_MASK_3: u32 = 0x3f00ffff;
pub const TEGRA264_XBAR_REG_MASK_4: u32 = 0x7fff9f;
pub const TEGRA264_XBAR_UPDATE_MAX_REG: usize = 5;

pub const TEGRA264_AXBAR_ADMAIF_RX1: u32 = 0x0;
pub const TEGRA264_AXBAR_SFC4_RX1: u32 = 0x6c;
pub const TEGRA264_AXBAR_MIXER1_RX1: u32 = 0x80;
pub const TEGRA264_AXBAR_MIXER1_RX10: u32 = 0xa4;
pub const TEGRA264_AXBAR_DSPK1_RX1: u32 = 0xc0;
pub const TEGRA264_AXBAR_OPE1_RX1: u32 = 0x100;
pub const TEGRA264_AXBAR_MVC1_RX1: u32 = 0x110;
pub const TEGRA264_AXBAR_MVC2_RX1: u32 = 0x114;
pub const TEGRA264_AXBAR_AMX1_RX1: u32 = 0x120;
pub const TEGRA264_AXBAR_AMX3_RX4: u32 = 0x14c;
pub const TEGRA264_AXBAR_ADX1_RX1: u32 = 0x160;
pub const TEGRA264_AXBAR_ASRC1_RX7: u32 = 0x1a8;
pub const TEGRA264_AXBAR_ADMAIF_RX21: u32 = 0x1d0;
pub const TEGRA264_AXBAR_ADX6_RX1: u32 = 0x224;

pub const TEGRA_XBAR_UPDATE_MAX_REG: usize = TEGRA264_XBAR_UPDATE_MAX_REG;

pub const TEGRA264_MAX_REGISTER_ADDR: u32 = TEGRA264_XBAR_PART4_RX
    + (TEGRA210_XBAR_RX_STRIDE * (TEGRA264_XBAR_AUDIO_RX_COUNT - 1));

pub const TEGRA186_MAX_REGISTER_ADDR: u32 = TEGRA186_XBAR_PART3_RX
    + (TEGRA210_XBAR_RX_STRIDE * (TEGRA186_XBAR_AUDIO_RX_COUNT - 1));

pub const TEGRA210_MAX_REGISTER_ADDR: u32 = TEGRA210_XBAR_PART2_RX
    + (TEGRA210_XBAR_RX_STRIDE * (TEGRA210_XBAR_AUDIO_RX_COUNT - 1));

/* AXBAR register offsets */
pub const TEGRA186_AXBAR_PART_0_AMX1_RX1_0: u32 = 0x120;
pub const TEGRA186_AXBAR_PART_0_AMX3_RX4_0: u32 = 0x14c;
pub const TEGRA186_AXBAR_PART_0_ASRC1_RX7_0: u32 = 0x1a8;
pub const TEGRA186_AXBAR_PART_0_DSPK1_RX1_0: u32 = 0xc0;
pub const TEGRA186_AXBAR_PART_0_DSPK2_RX1_0: u32 = 0xc4;
pub const TEGRA186_AXBAR_PART_0_I2S6_RX1_0: u32 = 0x54;
pub const TEGRA186_AXBAR_PART_0_MVC1_RX1_0: u32 = 0x110;
pub const TEGRA186_AXBAR_PART_0_MVC2_RX1_0: u32 = 0x114;
pub const TEGRA210_AXBAR_PART_0_ADMAIF_RX10_0: u32 = 0x24;
pub const TEGRA210_AXBAR_PART_0_ADMAIF_RX1_0: u32 = 0x0;
pub const TEGRA210_AXBAR_PART_0_ADX1_RX1_0: u32 = 0x160;
pub const TEGRA210_AXBAR_PART_0_ADX2_RX1_0: u32 = 0x164;
pub const TEGRA210_AXBAR_PART_0_AFC1_RX1_0: u32 = 0xd0;
pub const TEGRA210_AXBAR_PART_0_AFC6_RX1_0: u32 = 0xe4;
pub const TEGRA210_AXBAR_PART_0_AMX1_RX1_0: u32 = 0x140;
pub const TEGRA210_AXBAR_PART_0_I2S1_RX1_0: u32 = 0x40;
pub const TEGRA210_AXBAR_PART_0_I2S5_RX1_0: u32 = 0x50;
pub const TEGRA210_AXBAR_PART_0_MIXER1_RX10_0: u32 = 0xa4;
pub const TEGRA210_AXBAR_PART_0_MIXER1_RX1_0: u32 = 0x80;
pub const TEGRA210_AXBAR_PART_0_MVC1_RX1_0: u32 = 0x120;
pub const TEGRA210_AXBAR_PART_0_MVC2_RX1_0: u32 = 0x124;
pub const TEGRA210_AXBAR_PART_0_OPE1_RX1_0: u32 = 0x100;
pub const TEGRA210_AXBAR_PART_0_OPE2_RX1_0: u32 = 0x104;
pub const TEGRA210_AXBAR_PART_0_SFC1_RX1_0: u32 = 0x60;
pub const TEGRA210_AXBAR_PART_0_SFC4_RX1_0: u32 = 0x6c;
pub const TEGRA210_AXBAR_PART_0_SPDIF1_RX1_0: u32 = 0xc0;
pub const TEGRA210_AXBAR_PART_0_SPDIF1_RX2_0: u32 = 0xc4;
pub const TEGRA210_AXBAR_PART_0_SPKPROT1_RX1_0: u32 = 0x110;

#[inline]
pub const fn MUX_REG(id: u32) -> u32 {
    TEGRA210_XBAR_RX_STRIDE * id
}

#[inline]
pub const fn MUX_VALUE(npart: u32, nbit: u32) -> u32 {
    1 + nbit + npart * 32
}

macro_rules! SOC_VALUE_ENUM_WIDE {
    ($xreg:expr, $shift:expr, $xmax:expr, $xtexts:expr, $xvalues:expr) => {
        soc_enum {
            reg: $xreg,
            shift_l: $shift,
            shift_r: $shift,
            items: $xmax,
            texts: $xtexts,
            values: $xvalues,
            mask: if $xmax != 0 {
                roundup_pow_of_two($xmax) - 1
            } else {
                0
            },
        }
    };
}

macro_rules! SOC_VALUE_ENUM_WIDE_DECL {
    ($name:ident, $xreg:expr, $shift:expr, $xtexts:expr, $xvalues:expr) => {
        static mut $name: soc_enum = SOC_VALUE_ENUM_WIDE!(
            $xreg,
            $shift,
            ARRAY_SIZE!($xtexts),
            $xtexts,
            $xvalues
        );
    };
}

macro_rules! MUX_ENUM_CTRL_DECL {
    ($ename:ident, $id:expr) => {
        paste::paste! {
            SOC_VALUE_ENUM_WIDE_DECL!(
                [<$ename _enum>],
                MUX_REG($id),
                0,
                tegra210_ahub_mux_texts,
                tegra210_ahub_mux_values
            );
            static [<$ename _control>]: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!(
                "Route",
                [<$ename _enum>],
                tegra_ahub_get_value_enum,
                tegra_ahub_put_value_enum
            );
        }
    };
}

macro_rules! MUX_ENUM_CTRL_DECL_186 {
    ($ename:ident, $id:expr) => {
        paste::paste! {
            SOC_VALUE_ENUM_WIDE_DECL!(
                [<$ename _enum>],
                MUX_REG($id),
                0,
                tegra186_ahub_mux_texts,
                tegra186_ahub_mux_values
            );
            static [<$ename _control>]: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!(
                "Route",
                [<$ename _enum>],
                tegra_ahub_get_value_enum,
                tegra_ahub_put_value_enum
            );
        }
    };
}

macro_rules! MUX_ENUM_CTRL_DECL_234 {
    ($ename:ident, $id:expr) => {
        MUX_ENUM_CTRL_DECL_186!($ename, $id);
    };
}

macro_rules! MUX_ENUM_CTRL_DECL_264 {
    ($ename:ident, $id:expr) => {
        paste::paste! {
            SOC_VALUE_ENUM_WIDE_DECL!(
                [<$ename _enum>],
                MUX_REG($id),
                0,
                tegra264_ahub_mux_texts,
                tegra264_ahub_mux_values
            );
            static [<$ename _control>]: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!(
                "Route",
                [<$ename _enum>],
                tegra_ahub_get_value_enum,
                tegra_ahub_put_value_enum
            );
        }
    };
}

macro_rules! WIDGETS {
    ($sname:expr, $ename:ident) => {
        SND_SOC_DAPM_AIF_IN!($sname " XBAR-RX", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_OUT!($sname " XBAR-TX", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        paste::paste! {
            SND_SOC_DAPM_MUX!($sname " Mux", SND_SOC_NOPM, 0, 0, &[<$ename _control>])
        }
    };
}

macro_rules! TX_WIDGETS {
    ($sname:expr) => {
        SND_SOC_DAPM_AIF_IN!($sname " XBAR-RX", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_OUT!($sname " XBAR-TX", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0)
    };
}

macro_rules! DAI {
    ($sname:ident) => {
        snd_soc_dai_driver {
            name: concat!("XBAR-", stringify!($sname)),
            playback: snd_soc_pcm_stream {
                stream_name: concat!(stringify!($sname), " XBAR-Playback"),
                channels_min: 1,
                channels_max: 32,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S24_LE
                    | SNDRV_PCM_FMTBIT_S32_LE,
            },
            capture: snd_soc_pcm_stream {
                stream_name: concat!(stringify!($sname), " XBAR-Capture"),
                channels_min: 1,
                channels_max: 32,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S24_LE
                    | SNDRV_PCM_FMTBIT_S32_LE,
            },
        }
    };
}

#[repr(C)]
pub struct tegra_ahub_soc_data {
    pub regmap_config: *const regmap_config,
    pub cmpnt_drv: *const snd_soc_component_driver,
    pub dai_drv: *mut snd_soc_dai_driver,
    pub mask: [u32; TEGRA_XBAR_UPDATE_MAX_REG],
    pub reg_count: u32,
    pub num_dais: u32,
    pub xbar_part_size: u32,
}

#[repr(C)]
pub struct tegra_ahub {
    pub soc_data: *const tegra_ahub_soc_data,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
