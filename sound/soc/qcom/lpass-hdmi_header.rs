// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 The Linux Foundation. All rights reserved.
 *
 * lpass_hdmi.h - Definitions for the QTi LPASS HDMI
 */

// C dependency: <linux/regmap.h>

pub const LPASS_HDMITX_LEGACY_DISABLE: u32 = 0x0;
pub const LPASS_HDMITX_LEGACY_ENABLE: u32 = 0x1;
pub const LPASS_DP_AUDIO_BITWIDTH16: u32 = 0x0;
pub const LPASS_DP_AUDIO_BITWIDTH24: u32 = 0xb;
pub const LPASS_DATA_FORMAT_SHIFT: u32 = 0x1;
pub const LPASS_FREQ_BIT_SHIFT: u32 = 24;
pub const LPASS_DATA_FORMAT_LINEAR: u32 = 0x0;
pub const LPASS_DATA_FORMAT_NON_LINEAR: u32 = 0x1;
pub const LPASS_SAMPLING_FREQ32: u32 = 0x3;
pub const LPASS_SAMPLING_FREQ44: u32 = 0x0;
pub const LPASS_SAMPLING_FREQ48: u32 = 0x2;
pub const LPASS_TX_CTL_RESET: u32 = 0x1;
pub const LPASS_TX_CTL_CLEAR: u32 = 0x0;
pub const LPASS_SSTREAM_ENABLE: u32 = 1;
pub const LPASS_SSTREAM_DISABLE: u32 = 0;
pub const LPASS_LAYOUT_SP_DEFAULT: u32 = 0xf;
pub const LPASS_SSTREAM_DEFAULT_ENABLE: u32 = 1;
pub const LPASS_SSTREAM_DEFAULT_DISABLE: u32 = 0;
pub const LPASS_MUTE_ENABLE: u32 = 1;
pub const LPASS_MUTE_DISABLE: u32 = 0;
pub const LPASS_META_DEFAULT_VAL: u32 = 0;
pub const HW_MODE: u32 = 1;
pub const SW_MODE: u32 = 0;
pub const LEGACY_LPASS_LPAIF: u32 = 1;
pub const LEGACY_LPASS_HDMI: u32 = 0;
pub const REPLACE_VBIT: u32 = 0x1;
pub const LINEAR_PCM_DATA: u32 = 0x0;
pub const NON_LINEAR_PCM_DATA: u32 = 0x1;
pub const HDMITX_PARITY_CALC_EN: u32 = 0x1;
pub const HDMITX_PARITY_CALC_DIS: u32 = 0x0;
pub const LPASS_DATA_FORMAT_MASK: u32 = 0x2;
pub const LPASS_WORDLENGTH_MASK: u32 = 0xf;
pub const LPASS_FREQ_BIT_MASK: u32 = 0x0f00_0000;

macro_rules! LPASS_HDMI_TX_CTL_ADDR {
    ($v:expr) => {
        (*$v).hdmi_tx_ctl_addr
    };
}

macro_rules! LPASS_HDMI_TX_LEGACY_ADDR {
    ($v:expr) => {
        (*$v).hdmi_legacy_addr
    };
}

macro_rules! LPASS_HDMI_TX_VBIT_CTL_ADDR {
    ($v:expr) => {
        (*$v).hdmi_vbit_addr
    };
}

macro_rules! LPASS_HDMI_TX_PARITY_ADDR {
    ($v:expr) => {
        (*$v).hdmi_parity_addr
    };
}

macro_rules! LPASS_HDMI_TX_DP_ADDR {
    ($v:expr) => {
        (*$v).hdmi_DP_addr
    };
}

macro_rules! LPASS_HDMI_TX_SSTREAM_ADDR {
    ($v:expr) => {
        (*$v).hdmi_sstream_addr
    };
}

macro_rules! LPASS_HDMI_TX_CH_LSB_ADDR {
    ($v:expr, $port:expr) => {
        (*$v).hdmi_ch_lsb_addr + (*$v).ch_stride * ($port)
    };
}

macro_rules! LPASS_HDMI_TX_CH_MSB_ADDR {
    ($v:expr, $port:expr) => {
        (*$v).hdmi_ch_msb_addr + (*$v).ch_stride * ($port)
    };
}

macro_rules! LPASS_HDMI_TX_DMA_ADDR {
    ($v:expr, $port:expr) => {
        (*$v).hdmi_dmactl_addr + (*$v).hdmi_dma_stride * ($port)
    };
}

#[repr(C)]
pub struct lpass_sstream_ctl {
    pub sstream_en: *mut regmap_field,
    pub dma_sel: *mut regmap_field,
    pub auto_bbit_en: *mut regmap_field,
    pub layout: *mut regmap_field,
    pub layout_sp: *mut regmap_field,
    pub set_sp_on_en: *mut regmap_field,
    pub dp_audio: *mut regmap_field,
    pub dp_staffing_en: *mut regmap_field,
    pub dp_sp_b_hw_en: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_dp_metadata_ctl {
    pub mute: *mut regmap_field,
    pub as_sdp_cc: *mut regmap_field,
    pub as_sdp_ct: *mut regmap_field,
    pub aif_db4: *mut regmap_field,
    pub frequency: *mut regmap_field,
    pub mst_index: *mut regmap_field,
    pub dptx_index: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_hdmi_tx_ctl {
    pub soft_reset: *mut regmap_field,
    pub force_reset: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_hdmitx_dmactl {
    pub use_hw_chs: *mut regmap_field,
    pub use_hw_usr: *mut regmap_field,
    pub hw_chs_sel: *mut regmap_field,
    pub hw_usr_sel: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_vbit_ctrl {
    pub replace_vbit: *mut regmap_field,
    pub vbit_stream: *mut regmap_field,
}

unsafe extern "C" {
    pub static asoc_qcom_lpass_hdmi_dai_ops: snd_soc_dai_ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
