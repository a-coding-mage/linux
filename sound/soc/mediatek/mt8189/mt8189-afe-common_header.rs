/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8189-afe-common.h  --  Mediatek 8189 audio driver definitions
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

use core::ffi::{c_int, c_uint, c_void};

/* C includes translated as external dependencies:
 * <linux/regmap.h>
 * <sound/soc.h>
 * "mt8189-reg.h"
 * "../common/mtk-base-afe.h"
 */

pub const MTK_AFE_RATE_8K: c_int = 0;
pub const MTK_AFE_RATE_11K: c_int = 1;
pub const MTK_AFE_RATE_12K: c_int = 2;
pub const MTK_AFE_RATE_384K: c_int = 3;
pub const MTK_AFE_RATE_16K: c_int = 4;
pub const MTK_AFE_RATE_22K: c_int = 5;
pub const MTK_AFE_RATE_24K: c_int = 6;
pub const MTK_AFE_RATE_352K: c_int = 7;
pub const MTK_AFE_RATE_32K: c_int = 8;
pub const MTK_AFE_RATE_44K: c_int = 9;
pub const MTK_AFE_RATE_48K: c_int = 10;
pub const MTK_AFE_RATE_88K: c_int = 11;
pub const MTK_AFE_RATE_96K: c_int = 12;
pub const MTK_AFE_RATE_176K: c_int = 13;
pub const MTK_AFE_RATE_192K: c_int = 14;
pub const MTK_AFE_RATE_260K: c_int = 15;

/* HW IPM 2.0 */
pub const MTK_AFE_IPM2P0_RATE_8K: c_int = 0x0;
pub const MTK_AFE_IPM2P0_RATE_11K: c_int = 0x1;
pub const MTK_AFE_IPM2P0_RATE_12K: c_int = 0x2;
pub const MTK_AFE_IPM2P0_RATE_16K: c_int = 0x4;
pub const MTK_AFE_IPM2P0_RATE_22K: c_int = 0x5;
pub const MTK_AFE_IPM2P0_RATE_24K: c_int = 0x6;
pub const MTK_AFE_IPM2P0_RATE_32K: c_int = 0x8;
pub const MTK_AFE_IPM2P0_RATE_44K: c_int = 0x9;
pub const MTK_AFE_IPM2P0_RATE_48K: c_int = 0xa;
pub const MTK_AFE_IPM2P0_RATE_88K: c_int = 0xd;
pub const MTK_AFE_IPM2P0_RATE_96K: c_int = 0xe;
pub const MTK_AFE_IPM2P0_RATE_176K: c_int = 0x11;
pub const MTK_AFE_IPM2P0_RATE_192K: c_int = 0x12;
pub const MTK_AFE_IPM2P0_RATE_352K: c_int = 0x15;
pub const MTK_AFE_IPM2P0_RATE_384K: c_int = 0x16;

pub const MTK_AFE_DAI_MEMIF_RATE_8K: c_int = 0;
pub const MTK_AFE_DAI_MEMIF_RATE_16K: c_int = 1;
pub const MTK_AFE_DAI_MEMIF_RATE_32K: c_int = 2;
pub const MTK_AFE_DAI_MEMIF_RATE_48K: c_int = 3;

pub const MTK_AFE_PCM_RATE_8K: c_int = 0;
pub const MTK_AFE_PCM_RATE_16K: c_int = 1;
pub const MTK_AFE_PCM_RATE_32K: c_int = 2;
pub const MTK_AFE_PCM_RATE_48K: c_int = 3;

pub const MTKAIF_PROTOCOL_1: c_int = 0;
pub const MTKAIF_PROTOCOL_2: c_int = 1;
pub const MTKAIF_PROTOCOL_2_CLK_P2: c_int = 2;

pub const MT8189_MEMIF_DL0: c_int = 0;
pub const MT8189_MEMIF_DL1: c_int = 1;
pub const MT8189_MEMIF_DL2: c_int = 2;
pub const MT8189_MEMIF_DL3: c_int = 3;
pub const MT8189_MEMIF_DL4: c_int = 4;
pub const MT8189_MEMIF_DL5: c_int = 5;
pub const MT8189_MEMIF_DL6: c_int = 6;
pub const MT8189_MEMIF_DL7: c_int = 7;
pub const MT8189_MEMIF_DL8: c_int = 8;
pub const MT8189_MEMIF_DL23: c_int = 9;
pub const MT8189_MEMIF_DL24: c_int = 10;
pub const MT8189_MEMIF_DL25: c_int = 11;
pub const MT8189_MEMIF_DL_24CH: c_int = 12;
pub const MT8189_MEMIF_VUL0: c_int = 13;
pub const MT8189_MEMIF_VUL1: c_int = 14;
pub const MT8189_MEMIF_VUL2: c_int = 15;
pub const MT8189_MEMIF_VUL3: c_int = 16;
pub const MT8189_MEMIF_VUL4: c_int = 17;
pub const MT8189_MEMIF_VUL5: c_int = 18;
pub const MT8189_MEMIF_VUL6: c_int = 19;
pub const MT8189_MEMIF_VUL7: c_int = 20;
pub const MT8189_MEMIF_VUL8: c_int = 21;
pub const MT8189_MEMIF_VUL9: c_int = 22;
pub const MT8189_MEMIF_VUL10: c_int = 23;
pub const MT8189_MEMIF_VUL24: c_int = 24;
pub const MT8189_MEMIF_VUL25: c_int = 25;
pub const MT8189_MEMIF_VUL_CM0: c_int = 26;
pub const MT8189_MEMIF_VUL_CM1: c_int = 27;
pub const MT8189_MEMIF_ETDM_IN0: c_int = 28;
pub const MT8189_MEMIF_ETDM_IN1: c_int = 29;
pub const MT8189_MEMIF_HDMI: c_int = 30;
pub const MT8189_MEMIF_NUM: c_int = 31;
pub const MT8189_DAI_ADDA: c_int = MT8189_MEMIF_NUM;
pub const MT8189_DAI_ADDA_CH34: c_int = 32;
pub const MT8189_DAI_ADDA_CH56: c_int = 33;
pub const MT8189_DAI_AP_DMIC: c_int = 34;
pub const MT8189_DAI_AP_DMIC_CH34: c_int = 35;
pub const MT8189_DAI_I2S_IN0: c_int = 36;
pub const MT8189_DAI_I2S_IN1: c_int = 37;
pub const MT8189_DAI_I2S_OUT0: c_int = 38;
pub const MT8189_DAI_I2S_OUT1: c_int = 39;
pub const MT8189_DAI_I2S_OUT4: c_int = 40;
pub const MT8189_DAI_PCM_0: c_int = 41;
pub const MT8189_DAI_TDM: c_int = 42;
pub const MT8189_DAI_TDM_DPTX: c_int = 43;
pub const MT8189_DAI_NUM: c_int = 44;

/* update irq ID (= enum) from AFE_IRQ_MCU_STATUS */
pub const MT8189_IRQ_0: c_int = 0;
pub const MT8189_IRQ_1: c_int = 1;
pub const MT8189_IRQ_2: c_int = 2;
pub const MT8189_IRQ_3: c_int = 3;
pub const MT8189_IRQ_4: c_int = 4;
pub const MT8189_IRQ_5: c_int = 5;
pub const MT8189_IRQ_6: c_int = 6;
pub const MT8189_IRQ_7: c_int = 7;
pub const MT8189_IRQ_8: c_int = 8;
pub const MT8189_IRQ_9: c_int = 9;
pub const MT8189_IRQ_10: c_int = 10;
pub const MT8189_IRQ_11: c_int = 11;
pub const MT8189_IRQ_12: c_int = 12;
pub const MT8189_IRQ_13: c_int = 13;
pub const MT8189_IRQ_14: c_int = 14;
pub const MT8189_IRQ_15: c_int = 15;
pub const MT8189_IRQ_16: c_int = 16;
pub const MT8189_IRQ_17: c_int = 17;
pub const MT8189_IRQ_18: c_int = 18;
pub const MT8189_IRQ_19: c_int = 19;
pub const MT8189_IRQ_20: c_int = 20;
pub const MT8189_IRQ_21: c_int = 21;
pub const MT8189_IRQ_22: c_int = 22;
pub const MT8189_IRQ_23: c_int = 23;
pub const MT8189_IRQ_24: c_int = 24;
pub const MT8189_IRQ_25: c_int = 25;
pub const MT8189_IRQ_26: c_int = 26;
pub const MT8189_IRQ_31: c_int = 27;
pub const MT8189_IRQ_NUM: c_int = 28;

/* update irq ID (= enum) from AFE_IRQ_MCU_STATUS */
pub const MT8189_CUS_IRQ_TDM: c_int = 0; /* used only for TDM */
pub const MT8189_CUS_IRQ_NUM: c_int = 1;

/* AUDIO_ENGEN_CON0 */
pub const MT8189_AUDIO_26M_EN_ON: c_int = 0;
pub const MT8189_AUDIO_F3P25M_EN_ON: c_int = 1;
pub const MT8189_AUDIO_APLL1_EN_ON: c_int = 2;
pub const MT8189_AUDIO_APLL2_EN_ON: c_int = 3;
pub const MT8189_AUDIO_F26M_EN_RST: c_int = 4;
pub const MT8189_MULTI_USER_RST: c_int = 5;
pub const MT8189_MULTI_USER_BYPASS: c_int = 6;
/* AUDIO_TOP_CON4 */
pub const MT8189_CG_AUDIO_HOPPING_CK: c_int = 7;
pub const MT8189_CG_AUDIO_F26M_CK: c_int = 8;
pub const MT8189_CG_APLL1_CK: c_int = 9;
pub const MT8189_CG_APLL2_CK: c_int = 10;
pub const MT8189_PDN_APLL_TUNER2: c_int = 11;
pub const MT8189_PDN_APLL_TUNER1: c_int = 12;
pub const MT8189_AUDIO_CG_NUM: c_int = 13;

/* MCLK */
pub const MT8189_I2SIN0_MCK: c_int = 0;
pub const MT8189_I2SIN1_MCK: c_int = 1;
pub const MT8189_I2SOUT0_MCK: c_int = 2;
pub const MT8189_I2SOUT1_MCK: c_int = 3;
pub const MT8189_FMI2S_MCK: c_int = 4;
pub const MT8189_TDMOUT_MCK: c_int = 5;
pub const MT8189_TDMOUT_BCK: c_int = 6;
pub const MT8189_MCK_NUM: c_int = 7;

pub const CM0: c_int = 0;
pub const CM1: c_int = 1;
pub const CM_NUM: c_int = 2;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mt8189_afe_private {
    pub clk: *mut *mut clk,
    pub pmic_regmap: *mut regmap,

    /* dai */
    pub dai_priv: [*mut c_void; MT8189_DAI_NUM as usize],

    /* adda */
    pub mtkaif_protocol: c_int,
    pub mtkaif_chosen_phase: [c_int; 4],
    pub mtkaif_phase_cycle: [c_int; 4],
    pub mtkaif_calibration_num_phase: c_int,
    pub mtkaif_dmic: c_int,
    pub mtkaif_dmic_ch34: c_int,

    /* add for vs1 voter */
    pub is_adda_dl_on: bool,
    pub is_adda_ul_on: bool,
    /* adda dl vol idx is at maximum */
    pub is_adda_dl_max_vol: bool,
    /* current vote status of vs1 */
    pub is_mt6363_vote: bool,

    /* mck */
    pub mck_rate: [c_int; MT8189_MCK_NUM as usize],

    /* channel merge */
    pub cm_rate: [c_uint; CM_NUM as usize],
    pub cm_channels: c_uint,
}

unsafe extern "C" {
    pub fn mt8189_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int;
    pub fn mt8189_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
