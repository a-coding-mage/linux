/* SPDX-License-Identifier: GPL-2.0
 *
 * MediaTek 8365 audio driver common definitions
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

/* C header dependencies:
 * <linux/clk.h>, <linux/list.h>, <linux/regmap.h>, <sound/soc.h>,
 * <sound/asound.h>, "../common/mtk-base-afe.h", "mt8365-reg.h"
 */

use core::ffi::c_void;

pub const MT8365_AFE_MEMIF_DL1: u32 = 0;
pub const MT8365_AFE_MEMIF_DL2: u32 = 1;
pub const MT8365_AFE_MEMIF_TDM_OUT: u32 = 2;
/*
 * MT8365_AFE_MEMIF_SPDIF_OUT,
 */
pub const MT8365_AFE_MEMIF_AWB: u32 = 3;
pub const MT8365_AFE_MEMIF_VUL: u32 = 4;
pub const MT8365_AFE_MEMIF_VUL2: u32 = 5;
pub const MT8365_AFE_MEMIF_VUL3: u32 = 6;
pub const MT8365_AFE_MEMIF_TDM_IN: u32 = 7;
/*
 * MT8365_AFE_MEMIF_SPDIF_IN,
 */
pub const MT8365_AFE_MEMIF_NUM: u32 = 8;
pub const MT8365_AFE_BACKEND_BASE: u32 = MT8365_AFE_MEMIF_NUM;
pub const MT8365_AFE_IO_TDM_OUT: u32 = MT8365_AFE_BACKEND_BASE;
pub const MT8365_AFE_IO_TDM_IN: u32 = 9;
pub const MT8365_AFE_IO_I2S: u32 = 10;
pub const MT8365_AFE_IO_2ND_I2S: u32 = 11;
pub const MT8365_AFE_IO_PCM1: u32 = 12;
pub const MT8365_AFE_IO_VIRTUAL_DL_SRC: u32 = 13;
pub const MT8365_AFE_IO_VIRTUAL_TDM_OUT_SRC: u32 = 14;
pub const MT8365_AFE_IO_VIRTUAL_FM: u32 = 15;
pub const MT8365_AFE_IO_DMIC: u32 = 16;
pub const MT8365_AFE_IO_INT_ADDA: u32 = 17;
pub const MT8365_AFE_IO_GASRC1: u32 = 18;
pub const MT8365_AFE_IO_GASRC2: u32 = 19;
pub const MT8365_AFE_IO_TDM_ASRC: u32 = 20;
pub const MT8365_AFE_IO_HW_GAIN1: u32 = 21;
pub const MT8365_AFE_IO_HW_GAIN2: u32 = 22;
pub const MT8365_AFE_BACKEND_END: u32 = 23;
pub const MT8365_AFE_BACKEND_NUM: u32 = MT8365_AFE_BACKEND_END - MT8365_AFE_BACKEND_BASE;

pub const MT8365_AFE_IRQ1: u32 = 0;
pub const MT8365_AFE_IRQ2: u32 = 1;
pub const MT8365_AFE_IRQ3: u32 = 2;
pub const MT8365_AFE_IRQ4: u32 = 3;
pub const MT8365_AFE_IRQ5: u32 = 4;
pub const MT8365_AFE_IRQ6: u32 = 5;
pub const MT8365_AFE_IRQ7: u32 = 6;
pub const MT8365_AFE_IRQ8: u32 = 7;
pub const MT8365_AFE_IRQ9: u32 = 8;
pub const MT8365_AFE_IRQ10: u32 = 9;
pub const MT8365_AFE_IRQ_NUM: u32 = 10;

pub const MT8365_TOP_CG_AFE: u32 = 0;
pub const MT8365_TOP_CG_I2S_IN: u32 = 1;
pub const MT8365_TOP_CG_22M: u32 = 2;
pub const MT8365_TOP_CG_24M: u32 = 3;
pub const MT8365_TOP_CG_INTDIR_CK: u32 = 4;
pub const MT8365_TOP_CG_APLL2_TUNER: u32 = 5;
pub const MT8365_TOP_CG_APLL_TUNER: u32 = 6;
pub const MT8365_TOP_CG_SPDIF: u32 = 7;
pub const MT8365_TOP_CG_TDM_OUT: u32 = 8;
pub const MT8365_TOP_CG_TDM_IN: u32 = 9;
pub const MT8365_TOP_CG_ADC: u32 = 10;
pub const MT8365_TOP_CG_DAC: u32 = 11;
pub const MT8365_TOP_CG_DAC_PREDIS: u32 = 12;
pub const MT8365_TOP_CG_TML: u32 = 13;
pub const MT8365_TOP_CG_I2S1_BCLK: u32 = 14;
pub const MT8365_TOP_CG_I2S2_BCLK: u32 = 15;
pub const MT8365_TOP_CG_I2S3_BCLK: u32 = 16;
pub const MT8365_TOP_CG_I2S4_BCLK: u32 = 17;
pub const MT8365_TOP_CG_DMIC0_ADC: u32 = 18;
pub const MT8365_TOP_CG_DMIC1_ADC: u32 = 19;
pub const MT8365_TOP_CG_DMIC2_ADC: u32 = 20;
pub const MT8365_TOP_CG_DMIC3_ADC: u32 = 21;
pub const MT8365_TOP_CG_CONNSYS_I2S_ASRC: u32 = 22;
pub const MT8365_TOP_CG_GENERAL1_ASRC: u32 = 23;
pub const MT8365_TOP_CG_GENERAL2_ASRC: u32 = 24;
pub const MT8365_TOP_CG_TDM_ASRC: u32 = 25;
pub const MT8365_TOP_CG_NUM: u32 = 26;

pub const MT8365_CLK_TOP_AUD_SEL: u32 = 0;
pub const MT8365_CLK_AUD_I2S0_M: u32 = 1;
pub const MT8365_CLK_AUD_I2S1_M: u32 = 2;
pub const MT8365_CLK_AUD_I2S2_M: u32 = 3;
pub const MT8365_CLK_AUD_I2S3_M: u32 = 4;
pub const MT8365_CLK_ENGEN1: u32 = 5;
pub const MT8365_CLK_ENGEN2: u32 = 6;
pub const MT8365_CLK_AUD1: u32 = 7;
pub const MT8365_CLK_AUD2: u32 = 8;
pub const MT8365_CLK_I2S0_M_SEL: u32 = 9;
pub const MT8365_CLK_I2S1_M_SEL: u32 = 10;
pub const MT8365_CLK_I2S2_M_SEL: u32 = 11;
pub const MT8365_CLK_I2S3_M_SEL: u32 = 12;
pub const MT8365_CLK_CLK26M: u32 = 13;
pub const MT8365_CLK_NUM: u32 = 14;

pub const MT8365_AFE_APLL1: u32 = 0;
pub const MT8365_AFE_APLL2: u32 = 1;
pub const MT8365_AFE_APLL_NUM: u32 = 2;

pub const MT8365_AFE_1ST_I2S: u32 = 0;
pub const MT8365_AFE_2ND_I2S: u32 = 1;
pub const MT8365_AFE_I2S_SETS: u32 = 2;

pub const MT8365_AFE_I2S_SEPARATE_CLOCK: u32 = 0;
pub const MT8365_AFE_I2S_SHARED_CLOCK: u32 = 1;

pub const MT8365_AFE_TDM_OUT_I2S: u32 = 0;
pub const MT8365_AFE_TDM_OUT_TDM: u32 = 1;
pub const MT8365_AFE_TDM_OUT_I2S_32BITS: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mt8365_afe_tdm_ch_start {
    AFE_TDM_CH_START_O28_O29 = 0,
    AFE_TDM_CH_START_O30_O31 = 1,
    AFE_TDM_CH_START_O32_O33 = 2,
    AFE_TDM_CH_START_O34_O35 = 3,
    AFE_TDM_CH_ZERO = 4,
}

pub const MT8365_PCM_FORMAT_I2S: u32 = 0;
pub const MT8365_PCM_FORMAT_EIAJ: u32 = 1;
pub const MT8365_PCM_FORMAT_PCMA: u32 = 2;
pub const MT8365_PCM_FORMAT_PCMB: u32 = 3;

pub const MT8365_FS_8K: u32 = 0;
pub const MT8365_FS_11D025K: u32 = 1;
pub const MT8365_FS_12K: u32 = 2;
pub const MT8365_FS_384K: u32 = 3;
pub const MT8365_FS_16K: u32 = 4;
pub const MT8365_FS_22D05K: u32 = 5;
pub const MT8365_FS_24K: u32 = 6;
pub const MT8365_FS_130K: u32 = 7;
pub const MT8365_FS_32K: u32 = 8;
pub const MT8365_FS_44D1K: u32 = 9;
pub const MT8365_FS_48K: u32 = 10;
pub const MT8365_FS_88D2K: u32 = 11;
pub const MT8365_FS_96K: u32 = 12;
pub const MT8365_FS_176D4K: u32 = 13;
pub const MT8365_FS_192K: u32 = 14;

pub const FS_8000HZ: u32 = 0; /* 0000b */
pub const FS_11025HZ: u32 = 1; /* 0001b */
pub const FS_12000HZ: u32 = 2; /* 0010b */
pub const FS_384000HZ: u32 = 3; /* 0011b */
pub const FS_16000HZ: u32 = 4; /* 0100b */
pub const FS_22050HZ: u32 = 5; /* 0101b */
pub const FS_24000HZ: u32 = 6; /* 0110b */
pub const FS_130000HZ: u32 = 7; /* 0111b */
pub const FS_32000HZ: u32 = 8; /* 1000b */
pub const FS_44100HZ: u32 = 9; /* 1001b */
pub const FS_48000HZ: u32 = 10; /* 1010b */
pub const FS_88200HZ: u32 = 11; /* 1011b */
pub const FS_96000HZ: u32 = 12; /* 1100b */
pub const FS_176400HZ: u32 = 13; /* 1101b */
pub const FS_192000HZ: u32 = 14; /* 1110b */
pub const FS_260000HZ: u32 = 15; /* 1111b */

pub const MT8365_AFE_DEBUGFS_AFE: u32 = 0;
pub const MT8365_AFE_DEBUGFS_MEMIF: u32 = 1;
pub const MT8365_AFE_DEBUGFS_IRQ: u32 = 2;
pub const MT8365_AFE_DEBUGFS_CONN: u32 = 3;
pub const MT8365_AFE_DEBUGFS_DBG: u32 = 4;
pub const MT8365_AFE_DEBUGFS_NUM: u32 = 5;

pub const MT8365_AFE_IRQ_DIR_MCU: u32 = 0;
pub const MT8365_AFE_IRQ_DIR_DSP: u32 = 1;
pub const MT8365_AFE_IRQ_DIR_BOTH: u32 = 2;

/* MCLK */
pub const MT8365_I2S0_MCK: u32 = 0;
pub const MT8365_I2S3_MCK: u32 = 1;
pub const MT8365_MCK_NUM: u32 = 2;

/* SNDRV_PCM_STREAM_LAST is supplied by <sound/asound.h>. */
pub const SNDRV_PCM_STREAM_LAST: usize = 1;

#[repr(C)]
pub struct mt8365_fe_dai_data {
    pub use_sram: bool,
    pub sram_phy_addr: u32,
    pub sram_vir_addr: *mut c_void,
    pub sram_size: u32,
}

#[repr(C)]
pub struct mt8365_be_dai_data {
    pub prepared: [bool; SNDRV_PCM_STREAM_LAST + 1],
    pub fmt_mode: u32,
}

pub const MT8365_CLK_26M: u32 = 26000000;
pub const MT8365_CLK_24M: u32 = 24000000;
pub const MT8365_CLK_22M: u32 = 22000000;
pub const MT8365_CM_UPDATA_CNT_SET: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mt8365_cm_num {
    MT8365_CM1 = 0,
    MT8365_CM2 = 1,
    MT8365_CM_NUM = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mt8365_cm2_mux_in {
    MT8365_FROM_GASRC1 = 1,
    MT8365_FROM_GASRC2 = 2,
    MT8365_FROM_TDM_ASRC = 3,
    MT8365_CM_MUX_NUM = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cm2_mux_conn_in {
    GENERAL2_ASRC_OUT_LCH = 0,
    GENERAL2_ASRC_OUT_RCH = 1,
    TDM_IN_CH0 = 2,
    TDM_IN_CH1 = 3,
    TDM_IN_CH2 = 4,
    TDM_IN_CH3 = 5,
    TDM_IN_CH4 = 6,
    TDM_IN_CH5 = 7,
    TDM_IN_CH6 = 8,
    TDM_IN_CH7 = 9,
    GENERAL1_ASRC_OUT_LCH = 10,
    GENERAL1_ASRC_OUT_RCH = 11,
    TDM_OUT_ASRC_CH0 = 12,
    TDM_OUT_ASRC_CH1 = 13,
    TDM_OUT_ASRC_CH2 = 14,
    TDM_OUT_ASRC_CH3 = 15,
    TDM_OUT_ASRC_CH4 = 16,
    TDM_OUT_ASRC_CH5 = 17,
    TDM_OUT_ASRC_CH6 = 18,
    TDM_OUT_ASRC_CH7 = 19,
}

#[repr(C)]
pub struct mt8365_cm_ctrl_reg {
    pub con0: u32,
    pub con1: u32,
    pub con2: u32,
    pub con3: u32,
    pub con4: u32,
}

#[repr(C)]
pub struct mt8365_control_data {
    pub bypass_cm1: bool,
    pub bypass_cm2: bool,
    pub loopback_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dmic_input_mode {
    DMIC_MODE_3P25M = 0,
    DMIC_MODE_1P625M = 1,
    DMIC_MODE_812P5K = 2,
    DMIC_MODE_406P25K = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum iir_mode {
    IIR_MODE0 = 0,
    IIR_MODE1 = 1,
    IIR_MODE2 = 2,
    IIR_MODE3 = 3,
    IIR_MODE4 = 4,
    IIR_MODE5 = 5,
}

pub const MT8365_GASRC1: u32 = 0;
pub const MT8365_GASRC2: u32 = 1;
pub const MT8365_GASRC_NUM: u32 = 2;
pub const MT8365_TDM_ASRC1: u32 = MT8365_GASRC_NUM;
pub const MT8365_TDM_ASRC2: u32 = 3;
pub const MT8365_TDM_ASRC3: u32 = 4;
pub const MT8365_TDM_ASRC4: u32 = 5;
pub const MT8365_TDM_ASRC_NUM: u32 = 6;

#[repr(C)]
pub struct mt8365_gasrc_ctrl_reg {
    pub con0: u32,
    pub con2: u32,
    pub con3: u32,
    pub con4: u32,
    pub con5: u32,
    pub con6: u32,
    pub con9: u32,
    pub con10: u32,
    pub con12: u32,
    pub con13: u32,
}

#[repr(C)]
pub struct mt8365_gasrc_data {
    pub duplex: bool,
    pub tx_mode: bool,
    pub cali_on: bool,
    pub tdm_asrc_out_cm2: bool,
    pub iir_on: bool,
}

#[repr(C)]
pub struct mt8365_afe_private {
    pub clocks: [*mut c_void; MT8365_CLK_NUM as usize],
    pub topckgen: *mut c_void,
    pub fe_data: [mt8365_fe_dai_data; MT8365_AFE_MEMIF_NUM as usize],
    pub be_data: [mt8365_be_dai_data; MT8365_AFE_BACKEND_NUM as usize],
    pub ctrl_data: mt8365_control_data,
    pub gasrc_data: [mt8365_gasrc_data; MT8365_TDM_ASRC_NUM as usize],
    pub afe_on_ref_cnt: i32,
    pub top_cg_ref_cnt: [i32; MT8365_TOP_CG_NUM as usize],
    pub afe_sram_vir_addr: *mut c_void,
    pub afe_sram_phy_addr: u32,
    pub afe_sram_size: u32,
    /* locks */
    pub afe_ctrl_lock: c_void,
    pub afe_clk_mutex: c_void, /* Protect & sync APLL TUNER registers access*/
    /* CONFIG_DEBUG_FS: struct dentry *debugfs_dentry[MT8365_AFE_DEBUGFS_NUM]; */
    pub apll_tuner_ref_cnt: [i32; MT8365_AFE_APLL_NUM as usize],
    pub tdm_out_mode: u32,
    pub cm2_mux_input: u32,

    /* dai */
    pub dai_on: [bool; MT8365_AFE_BACKEND_END as usize],
    pub dai_priv: [*mut c_void; MT8365_AFE_BACKEND_END as usize],
}

#[inline]
pub fn rx_frequency_palette(fs: u32) -> u32 {
    /* *
     * A = (26M / fs) * 64
     * B = 8125 / A
     * return = DEC2HEX(B * 2^23)
     */
    match fs {
        FS_8000HZ => 0x050000,
        FS_11025HZ => 0x06E400,
        FS_12000HZ => 0x078000,
        FS_16000HZ => 0x0A0000,
        FS_22050HZ => 0x0DC800,
        FS_24000HZ => 0x0F0000,
        FS_32000HZ => 0x140000,
        FS_44100HZ => 0x1B9000,
        FS_48000HZ => 0x1E0000,
        FS_88200HZ => 0x372000,
        FS_96000HZ => 0x3C0000,
        FS_176400HZ => 0x6E4000,
        FS_192000HZ => 0x780000,
        _ => 0x0,
    }
}

#[inline]
pub fn AutoRstThHi(fs: u32) -> u32 {
    match fs {
        FS_8000HZ => 0x36000,
        FS_11025HZ => 0x27000,
        FS_12000HZ => 0x24000,
        FS_16000HZ => 0x1B000,
        FS_22050HZ => 0x14000,
        FS_24000HZ => 0x12000,
        FS_32000HZ => 0x0D800,
        FS_44100HZ => 0x09D00,
        FS_48000HZ => 0x08E00,
        FS_88200HZ => 0x04E00,
        FS_96000HZ => 0x04800,
        FS_176400HZ => 0x02700,
        FS_192000HZ => 0x02400,
        _ => 0x0,
    }
}

#[inline]
pub fn AutoRstThLo(fs: u32) -> u32 {
    match fs {
        FS_8000HZ => 0x30000,
        FS_11025HZ => 0x23000,
        FS_12000HZ => 0x20000,
        FS_16000HZ => 0x18000,
        FS_22050HZ => 0x11000,
        FS_24000HZ => 0x0FE00,
        FS_32000HZ => 0x0BE00,
        FS_44100HZ => 0x08A00,
        FS_48000HZ => 0x07F00,
        FS_88200HZ => 0x04500,
        FS_96000HZ => 0x04000,
        FS_176400HZ => 0x02300,
        FS_192000HZ => 0x02000,
        _ => 0x0,
    }
}

unsafe extern "C" {
    pub fn mt8365_afe_rate_supported(rate: u32, id: u32) -> bool;
    pub fn mt8365_afe_channel_supported(channel: u32, id: u32) -> bool;

    pub fn mt8365_dai_i2s_register(afe: *mut c_void) -> i32;
    pub fn mt8365_dai_set_priv(
        afe: *mut c_void,
        id: i32,
        priv_size: i32,
        priv_data: *const c_void,
    ) -> i32;

    pub fn mt8365_afe_fs_timing(rate: u32) -> i32;

    pub fn mt8365_afe_set_i2s_out_enable(afe: *mut c_void, enable: bool);
    pub fn mt8365_afe_set_i2s_out(afe: *mut c_void, rate: u32, bit_width: i32) -> i32;

    pub fn mt8365_dai_adda_register(afe: *mut c_void) -> i32;
    pub fn mt8365_dai_enable_adda_on(afe: *mut c_void) -> i32;
    pub fn mt8365_dai_disable_adda_on(afe: *mut c_void) -> i32;

    pub fn mt8365_dai_dmic_register(afe: *mut c_void) -> i32;

    pub fn mt8365_dai_pcm_register(afe: *mut c_void) -> i32;

    pub fn mt8365_dai_tdm_register(afe: *mut c_void) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
