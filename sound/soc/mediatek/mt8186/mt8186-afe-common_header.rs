/* SPDX-License-Identifier: GPL-2.0
 *
 * mt8186-afe-common.h  --  Mediatek 8186 audio driver definitions
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
 */

/* C includes translated as external dependencies:
 * <sound/soc.h>
 * <linux/list.h>
 * <linux/regmap.h>
 * "mt8186-reg.h"
 * "../common/mtk-base-afe.h"
 */

pub const MT8186_MEMIF_DL1: i32 = 0;
pub const MT8186_MEMIF_DL12: i32 = 1;
pub const MT8186_MEMIF_DL2: i32 = 2;
pub const MT8186_MEMIF_DL3: i32 = 3;
pub const MT8186_MEMIF_DL4: i32 = 4;
pub const MT8186_MEMIF_DL5: i32 = 5;
pub const MT8186_MEMIF_DL6: i32 = 6;
pub const MT8186_MEMIF_DL7: i32 = 7;
pub const MT8186_MEMIF_DL8: i32 = 8;
pub const MT8186_MEMIF_VUL12: i32 = 9;
pub const MT8186_MEMIF_VUL2: i32 = 10;
pub const MT8186_MEMIF_VUL3: i32 = 11;
pub const MT8186_MEMIF_VUL4: i32 = 12;
pub const MT8186_MEMIF_VUL5: i32 = 13;
pub const MT8186_MEMIF_VUL6: i32 = 14;
pub const MT8186_MEMIF_AWB: i32 = 15;
pub const MT8186_MEMIF_AWB2: i32 = 16;
pub const MT8186_MEMIF_NUM: i32 = 17;
pub const MT8186_DAI_ADDA: i32 = MT8186_MEMIF_NUM;
pub const MT8186_DAI_AP_DMIC: i32 = 18;
pub const MT8186_DAI_CONNSYS_I2S: i32 = 19;
pub const MT8186_DAI_I2S_0: i32 = 20;
pub const MT8186_DAI_I2S_1: i32 = 21;
pub const MT8186_DAI_I2S_2: i32 = 22;
pub const MT8186_DAI_I2S_3: i32 = 23;
pub const MT8186_DAI_HW_GAIN_1: i32 = 24;
pub const MT8186_DAI_HW_GAIN_2: i32 = 25;
pub const MT8186_DAI_SRC_1: i32 = 26;
pub const MT8186_DAI_SRC_2: i32 = 27;
pub const MT8186_DAI_PCM: i32 = 28;
pub const MT8186_DAI_TDM_IN: i32 = 29;
pub const MT8186_DAI_HOSTLESS_LPBK: i32 = 30;
pub const MT8186_DAI_HOSTLESS_FM: i32 = 31;
pub const MT8186_DAI_HOSTLESS_HW_GAIN_AAUDIO: i32 = 32;
pub const MT8186_DAI_HOSTLESS_SRC_AAUDIO: i32 = 33;
pub const MT8186_DAI_HOSTLESS_SRC_1: i32 = 34;
pub const MT8186_DAI_HOSTLESS_SRC_BARGEIN: i32 = 35;
pub const MT8186_DAI_HOSTLESS_UL1: i32 = 36;
pub const MT8186_DAI_HOSTLESS_UL2: i32 = 37;
pub const MT8186_DAI_HOSTLESS_UL3: i32 = 38;
pub const MT8186_DAI_HOSTLESS_UL5: i32 = 39;
pub const MT8186_DAI_HOSTLESS_UL6: i32 = 40;
pub const MT8186_DAI_NUM: i32 = 41;

pub const MT8186_RECORD_MEMIF: i32 = MT8186_MEMIF_VUL12;
pub const MT8186_ECHO_REF_MEMIF: i32 = MT8186_MEMIF_AWB;
pub const MT8186_PRIMARY_MEMIF: i32 = MT8186_MEMIF_DL1;
pub const MT8186_FAST_MEMIF: i32 = MT8186_MEMIF_DL2;
pub const MT8186_DEEP_MEMIF: i32 = MT8186_MEMIF_DL3;
pub const MT8186_VOIP_MEMIF: i32 = MT8186_MEMIF_DL12;
pub const MT8186_MMAP_DL_MEMIF: i32 = MT8186_MEMIF_DL5;
pub const MT8186_MMAP_UL_MEMIF: i32 = MT8186_MEMIF_VUL5;
pub const MT8186_BARGEIN_MEMIF: i32 = MT8186_MEMIF_AWB;

pub const MT8186_IRQ_0: i32 = 0;
pub const MT8186_IRQ_1: i32 = 1;
pub const MT8186_IRQ_2: i32 = 2;
pub const MT8186_IRQ_3: i32 = 3;
pub const MT8186_IRQ_4: i32 = 4;
pub const MT8186_IRQ_5: i32 = 5;
pub const MT8186_IRQ_6: i32 = 6;
pub const MT8186_IRQ_7: i32 = 7;
pub const MT8186_IRQ_8: i32 = 8;
pub const MT8186_IRQ_9: i32 = 9;
pub const MT8186_IRQ_10: i32 = 10;
pub const MT8186_IRQ_11: i32 = 11;
pub const MT8186_IRQ_12: i32 = 12;
pub const MT8186_IRQ_13: i32 = 13;
pub const MT8186_IRQ_14: i32 = 14;
pub const MT8186_IRQ_15: i32 = 15;
pub const MT8186_IRQ_16: i32 = 16;
pub const MT8186_IRQ_17: i32 = 17;
pub const MT8186_IRQ_18: i32 = 18;
pub const MT8186_IRQ_19: i32 = 19;
pub const MT8186_IRQ_20: i32 = 20;
pub const MT8186_IRQ_21: i32 = 21;
pub const MT8186_IRQ_22: i32 = 22;
pub const MT8186_IRQ_23: i32 = 23;
pub const MT8186_IRQ_24: i32 = 24;
pub const MT8186_IRQ_25: i32 = 25;
pub const MT8186_IRQ_26: i32 = 26;
pub const MT8186_IRQ_NUM: i32 = 27;

pub const MT8186_AFE_IRQ_DIR_MCU: i32 = 0;
pub const MT8186_AFE_IRQ_DIR_DSP: i32 = 1;
pub const MT8186_AFE_IRQ_DIR_BOTH: i32 = 2;

pub const MTKAIF_PROTOCOL_1: i32 = 0;
pub const MTKAIF_PROTOCOL_2: i32 = 1;
pub const MTKAIF_PROTOCOL_2_CLK_P2: i32 = 2;

pub const MTK_AFE_ADDA_DL_GAIN_MUTE: i32 = 0;
pub const MTK_AFE_ADDA_DL_GAIN_NORMAL: i32 = 0xf74f;
/* SA suggest apply -0.3db to audio/speech path */

pub const MTK_SPK_I2S_0_STR: &[u8; 14] = b"MTK_SPK_I2S_0\0";
pub const MTK_SPK_I2S_1_STR: &[u8; 14] = b"MTK_SPK_I2S_1\0";
pub const MTK_SPK_I2S_2_STR: &[u8; 14] = b"MTK_SPK_I2S_2\0";
pub const MTK_SPK_I2S_3_STR: &[u8; 14] = b"MTK_SPK_I2S_3\0";

/* MCLK */
pub const MT8186_I2S0_MCK: i32 = 0;
pub const MT8186_I2S1_MCK: i32 = 1;
pub const MT8186_I2S2_MCK: i32 = 2;
pub const MT8186_I2S4_MCK: i32 = 3;
pub const MT8186_TDM_MCK: i32 = 4;
pub const MT8186_MCK_NUM: i32 = 5;

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_irq_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk_lookup {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mt8186_afe_private {
    pub clk: *mut *mut clk,
    pub lookup: *mut *mut clk_lookup,
    pub topckgen: *mut regmap,
    pub apmixedsys: *mut regmap,
    pub infracfg: *mut regmap,
    pub irq_cnt: [i32; MT8186_MEMIF_NUM as usize],
    pub stf_positive_gain_db: i32,
    pub pm_runtime_bypass_reg_ctl: i32,
    pub sgen_mode: i32,
    pub sgen_rate: i32,
    pub sgen_amplitude: i32,

    /* xrun assert */
    pub xrun_assert: [i32; MT8186_MEMIF_NUM as usize],

    /* dai */
    pub dai_on: [bool; MT8186_DAI_NUM as usize],
    pub dai_priv: [*mut ::core::ffi::c_void; MT8186_DAI_NUM as usize],

    /* adda */
    pub mtkaif_calibration_ok: bool,
    pub mtkaif_protocol: i32,
    pub mtkaif_chosen_phase: [i32; 4],
    pub mtkaif_phase_cycle: [i32; 4],
    pub mtkaif_calibration_num_phase: i32,
    pub mtkaif_dmic: i32,
    pub mtkaif_looback0: i32,
    pub mtkaif_looback1: i32,

    /* mck */
    pub mck_rate: [i32; MT8186_MCK_NUM as usize],
}

unsafe extern "C" {
    pub fn mt8186_dai_adda_register(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_dai_i2s_register(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_dai_tdm_register(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_dai_hw_gain_register(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_dai_src_register(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_dai_pcm_register(afe: *mut mtk_base_afe) -> i32;
    pub fn mt8186_dai_hostless_register(afe: *mut mtk_base_afe) -> i32;

    pub fn mt8186_add_misc_control(component: *mut snd_soc_component) -> i32;

    pub fn mt8186_general_rate_transform(dev: *mut device, rate: u32) -> u32;
    pub fn mt8186_rate_transform(dev: *mut device, rate: u32, aud_blk: i32) -> u32;
    pub fn mt8186_tdm_relatch_rate_transform(dev: *mut device, rate: u32) -> u32;

    pub fn mt8186_dai_i2s_set_share(
        afe: *mut mtk_base_afe,
        main_i2s_name: *const ::core::ffi::c_char,
        secondary_i2s_name: *const ::core::ffi::c_char,
    ) -> i32;

    pub fn mt8186_dai_set_priv(
        afe: *mut mtk_base_afe,
        id: i32,
        priv_size: i32,
        priv_data: *const ::core::ffi::c_void,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
