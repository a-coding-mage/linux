/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8192-afe-common.h  --  Mediatek 8192 audio driver definitions
 *
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Shane Chien <shane.chien@mediatek.com>
 */

/* Bindings for declarations supplied by:
 * <linux/list.h>, <linux/regmap.h>, <sound/soc.h>,
 * "../common/mtk-base-afe.h", and "mt8192-reg.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub enum clk {}
pub enum regmap {}
pub enum mtk_base_afe {}
pub enum device {}

pub const MT8192_MEMIF_DL1: c_int = 0;
pub const MT8192_MEMIF_DL12: c_int = 1;
pub const MT8192_MEMIF_DL2: c_int = 2;
pub const MT8192_MEMIF_DL3: c_int = 3;
pub const MT8192_MEMIF_DL4: c_int = 4;
pub const MT8192_MEMIF_DL5: c_int = 5;
pub const MT8192_MEMIF_DL6: c_int = 6;
pub const MT8192_MEMIF_DL7: c_int = 7;
pub const MT8192_MEMIF_DL8: c_int = 8;
pub const MT8192_MEMIF_DL9: c_int = 9;
pub const MT8192_MEMIF_DAI: c_int = 10;
pub const MT8192_MEMIF_DAI2: c_int = 11;
pub const MT8192_MEMIF_MOD_DAI: c_int = 12;
pub const MT8192_MEMIF_VUL12: c_int = 13;
pub const MT8192_MEMIF_VUL2: c_int = 14;
pub const MT8192_MEMIF_VUL3: c_int = 15;
pub const MT8192_MEMIF_VUL4: c_int = 16;
pub const MT8192_MEMIF_VUL5: c_int = 17;
pub const MT8192_MEMIF_VUL6: c_int = 18;
pub const MT8192_MEMIF_AWB: c_int = 19;
pub const MT8192_MEMIF_AWB2: c_int = 20;
pub const MT8192_MEMIF_HDMI: c_int = 21;
pub const MT8192_MEMIF_NUM: c_int = 22;
pub const MT8192_DAI_ADDA: c_int = MT8192_MEMIF_NUM;
pub const MT8192_DAI_ADDA_CH34: c_int = 23;
pub const MT8192_DAI_AP_DMIC: c_int = 24;
pub const MT8192_DAI_AP_DMIC_CH34: c_int = 25;
pub const MT8192_DAI_VOW: c_int = 26;
pub const MT8192_DAI_CONNSYS_I2S: c_int = 27;
pub const MT8192_DAI_I2S_0: c_int = 28;
pub const MT8192_DAI_I2S_1: c_int = 29;
pub const MT8192_DAI_I2S_2: c_int = 30;
pub const MT8192_DAI_I2S_3: c_int = 31;
pub const MT8192_DAI_I2S_5: c_int = 32;
pub const MT8192_DAI_I2S_6: c_int = 33;
pub const MT8192_DAI_I2S_7: c_int = 34;
pub const MT8192_DAI_I2S_8: c_int = 35;
pub const MT8192_DAI_I2S_9: c_int = 36;
pub const MT8192_DAI_HW_GAIN_1: c_int = 37;
pub const MT8192_DAI_HW_GAIN_2: c_int = 38;
pub const MT8192_DAI_SRC_1: c_int = 39;
pub const MT8192_DAI_SRC_2: c_int = 40;
pub const MT8192_DAI_PCM_1: c_int = 41;
pub const MT8192_DAI_PCM_2: c_int = 42;
pub const MT8192_DAI_TDM: c_int = 43;
pub const MT8192_DAI_NUM: c_int = 44;

pub const MT8192_IRQ_0: c_int = 0;
pub const MT8192_IRQ_1: c_int = 1;
pub const MT8192_IRQ_2: c_int = 2;
pub const MT8192_IRQ_3: c_int = 3;
pub const MT8192_IRQ_4: c_int = 4;
pub const MT8192_IRQ_5: c_int = 5;
pub const MT8192_IRQ_6: c_int = 6;
pub const MT8192_IRQ_7: c_int = 7;
pub const MT8192_IRQ_8: c_int = 8;
pub const MT8192_IRQ_9: c_int = 9;
pub const MT8192_IRQ_10: c_int = 10;
pub const MT8192_IRQ_11: c_int = 11;
pub const MT8192_IRQ_12: c_int = 12;
pub const MT8192_IRQ_13: c_int = 13;
pub const MT8192_IRQ_14: c_int = 14;
pub const MT8192_IRQ_15: c_int = 15;
pub const MT8192_IRQ_16: c_int = 16;
pub const MT8192_IRQ_17: c_int = 17;
pub const MT8192_IRQ_18: c_int = 18;
pub const MT8192_IRQ_19: c_int = 19;
pub const MT8192_IRQ_20: c_int = 20;
pub const MT8192_IRQ_21: c_int = 21;
pub const MT8192_IRQ_22: c_int = 22;
pub const MT8192_IRQ_23: c_int = 23;
pub const MT8192_IRQ_24: c_int = 24;
pub const MT8192_IRQ_25: c_int = 25;
pub const MT8192_IRQ_26: c_int = 26;
pub const MT8192_IRQ_31: c_int = 27; /* used only for TDM */
pub const MT8192_IRQ_NUM: c_int = 28;

pub const MTKAIF_PROTOCOL_1: c_int = 0;
pub const MTKAIF_PROTOCOL_2: c_int = 1;
pub const MTKAIF_PROTOCOL_2_CLK_P2: c_int = 2;

pub const MTK_AFE_ADDA_DL_GAIN_MUTE: c_int = 0;
pub const MTK_AFE_ADDA_DL_GAIN_NORMAL: c_int = 0xf74f;
/* SA suggest apply -0.3db to audio/speech path */

/* MCLK */
pub const MT8192_I2S0_MCK: c_int = 0;
pub const MT8192_I2S1_MCK: c_int = 1;
pub const MT8192_I2S2_MCK: c_int = 2;
pub const MT8192_I2S3_MCK: c_int = 3;
pub const MT8192_I2S4_MCK: c_int = 4;
pub const MT8192_I2S4_BCK: c_int = 5;
pub const MT8192_I2S5_MCK: c_int = 6;
pub const MT8192_I2S6_MCK: c_int = 7;
pub const MT8192_I2S7_MCK: c_int = 8;
pub const MT8192_I2S8_MCK: c_int = 9;
pub const MT8192_I2S9_MCK: c_int = 10;
pub const MT8192_MCK_NUM: c_int = 11;

#[repr(C)]
pub struct mt8192_afe_private {
    pub clk: *mut *mut clk,
    pub topckgen: *mut regmap,
    pub apmixedsys: *mut regmap,
    pub infracfg: *mut regmap,
    pub stf_positive_gain_db: c_int,
    pub pm_runtime_bypass_reg_ctl: c_int,

    /* dai */
    pub dai_on: [bool; MT8192_DAI_NUM as usize],
    pub dai_priv: [*mut c_void; MT8192_DAI_NUM as usize],

    /* adda */
    pub mtkaif_protocol: c_int,
    pub mtkaif_chosen_phase: [c_int; 4],
    pub mtkaif_phase_cycle: [c_int; 4],
    pub mtkaif_calibration_num_phase: c_int,
    pub mtkaif_dmic: c_int,
    pub mtkaif_dmic_ch34: c_int,
    pub mtkaif_adda6_only: c_int,

    /* mck */
    pub mck_rate: [c_int; MT8192_MCK_NUM as usize],
}

unsafe extern "C" {
    pub fn mt8192_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8192_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8192_dai_hw_gain_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8192_dai_src_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8192_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8192_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int;

    pub fn mt8192_dai_i2s_set_share(
        afe: *mut mtk_base_afe,
        main_i2s_name: *const c_char,
        secondary_i2s_name: *const c_char,
    ) -> c_int;

    pub fn mt8192_general_rate_transform(dev: *mut device, rate: c_uint) -> c_uint;
    pub fn mt8192_rate_transform(dev: *mut device, rate: c_uint, aud_blk: c_int) -> c_uint;

    pub fn mt8192_dai_set_priv(
        afe: *mut mtk_base_afe,
        id: c_int,
        priv_size: c_int,
        priv_data: *const c_void,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
