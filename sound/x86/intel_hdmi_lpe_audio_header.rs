// SPDX-License-Identifier: GPL-2.0-only
//
//   intel_hdmi_lpe_audio.h - Intel HDMI LPE audio driver
//
//  Copyright (C) 2016 Intel Corp
//  Authors:	Sailaja Bandarupalli <sailaja.bandarupalli@intel.com>
//		Ramesh Babu K V <ramesh.babu@intel.com>
//		Vaibhav Agarwal <vaibhav.agarwal@intel.com>
//		Jerome Anand <jerome.anand@intel.com>
//		Aravind Siddappaji <aravindx.siddappaji@intel.com>
//  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub const HAD_MIN_CHANNEL: u32 = 2;
pub const HAD_MAX_CHANNEL: u32 = 8;
pub const HAD_NUM_OF_RING_BUFS: u32 = 4;

// max 20bit address, aligned to 64
pub const HAD_MAX_BUFFER: u32 = ((1024 * 1024 - 1) & !0x3f);
pub const HAD_DEFAULT_BUFFER: u32 = (600 * 1024); // default prealloc size
pub const HAD_MAX_PERIODS: u32 = 256; // arbitrary, but should suffice
pub const HAD_MIN_PERIODS: u32 = 1;
pub const HAD_MAX_PERIOD_BYTES: u32 = ((HAD_MAX_BUFFER / HAD_MIN_PERIODS) & !0x3f);
pub const HAD_MIN_PERIOD_BYTES: u32 = 1024; // might be smaller
pub const HAD_FIFO_SIZE: u32 = 0; // fifo not being used
pub const MAX_SPEAKERS: u32 = 8;

pub const AUD_SAMPLE_RATE_32: u32 = 32000;
pub const AUD_SAMPLE_RATE_44_1: u32 = 44100;
pub const AUD_SAMPLE_RATE_48: u32 = 48000;
pub const AUD_SAMPLE_RATE_88_2: u32 = 88200;
pub const AUD_SAMPLE_RATE_96: u32 = 96000;
pub const AUD_SAMPLE_RATE_176_4: u32 = 176400;
pub const AUD_SAMPLE_RATE_192: u32 = 192000;

pub const HAD_MIN_RATE: u32 = AUD_SAMPLE_RATE_32;
pub const HAD_MAX_RATE: u32 = AUD_SAMPLE_RATE_192;

pub const DIS_SAMPLE_RATE_25_2: u32 = 25200;
pub const DIS_SAMPLE_RATE_27: u32 = 27000;
pub const DIS_SAMPLE_RATE_54: u32 = 54000;
pub const DIS_SAMPLE_RATE_74_25: u32 = 74250;
pub const DIS_SAMPLE_RATE_148_5: u32 = 148500;
pub const HAD_REG_WIDTH: u32 = 0x08;
pub const HAD_MAX_DIP_WORDS: u32 = 16;

// DP Link Rates
pub const DP_2_7_GHZ: u32 = 270000;
pub const DP_1_62_GHZ: u32 = 162000;

// Maud Values
pub const AUD_SAMPLE_RATE_32_DP_2_7_MAUD_VAL: u32 = 1988;
pub const AUD_SAMPLE_RATE_44_1_DP_2_7_MAUD_VAL: u32 = 2740;
pub const AUD_SAMPLE_RATE_48_DP_2_7_MAUD_VAL: u32 = 2982;
pub const AUD_SAMPLE_RATE_88_2_DP_2_7_MAUD_VAL: u32 = 5480;
pub const AUD_SAMPLE_RATE_96_DP_2_7_MAUD_VAL: u32 = 5965;
pub const AUD_SAMPLE_RATE_176_4_DP_2_7_MAUD_VAL: u32 = 10961;
pub const HAD_MAX_RATE_DP_2_7_MAUD_VAL: u32 = 11930;
pub const AUD_SAMPLE_RATE_32_DP_1_62_MAUD_VAL: u32 = 3314;
pub const AUD_SAMPLE_RATE_44_1_DP_1_62_MAUD_VAL: u32 = 4567;
pub const AUD_SAMPLE_RATE_48_DP_1_62_MAUD_VAL: u32 = 4971;
pub const AUD_SAMPLE_RATE_88_2_DP_1_62_MAUD_VAL: u32 = 9134;
pub const AUD_SAMPLE_RATE_96_DP_1_62_MAUD_VAL: u32 = 9942;
pub const AUD_SAMPLE_RATE_176_4_DP_1_62_MAUD_VAL: u32 = 18268;
pub const HAD_MAX_RATE_DP_1_62_MAUD_VAL: u32 = 19884;

// Naud Value
pub const DP_NAUD_VAL: u32 = 32768;

// HDMI Controller register offsets - audio domain common
// Base address for below regs = 0x65000
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum hdmi_ctrl_reg_offset_common {
    AUDIO_HDMI_CONFIG_A = 0x000,
    AUDIO_HDMI_CONFIG_B = 0x800,
    AUDIO_HDMI_CONFIG_C = 0x900,
}

// HDMI controller register offsets
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum hdmi_ctrl_reg_offset {
    AUD_CONFIG = 0x0,
    AUD_CH_STATUS_0 = 0x08,
    AUD_CH_STATUS_1 = 0x0C,
    AUD_HDMI_CTS = 0x10,
    AUD_N_ENABLE = 0x14,
    AUD_SAMPLE_RATE = 0x18,
    AUD_BUF_CONFIG = 0x20,
    AUD_BUF_CH_SWAP = 0x24,
    AUD_BUF_A_ADDR = 0x40,
    AUD_BUF_A_LENGTH = 0x44,
    AUD_BUF_B_ADDR = 0x48,
    AUD_BUF_B_LENGTH = 0x4c,
    AUD_BUF_C_ADDR = 0x50,
    AUD_BUF_C_LENGTH = 0x54,
    AUD_BUF_D_ADDR = 0x58,
    AUD_BUF_D_LENGTH = 0x5c,
    AUD_CNTL_ST = 0x60,
    AUD_HDMI_STATUS = 0x64, // v2
    AUD_HDMIW_INFOFR = 0x68, // v2
}

// Audio configuration
#[repr(C)]
pub union aud_cfg {
    pub regx: aud_cfg_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_cfg_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // aud_en:1, layout:1, fmt:2, num_ch:3, set:1, flat:1, val_bit:1, user_bit:1,
    // underrun:1, packet_mode:1, left_align:1, bogus_sample:1, dp_modei:1, rsvd:16
}

pub const AUD_CONFIG_VALID_BIT: u32 = (1 << 9);
pub const AUD_CONFIG_DP_MODE: u32 = (1 << 15);
pub const AUD_CONFIG_CH_MASK: u32 = 0x70;
pub const LAYOUT0: u32 = 0; // interleaved stereo
pub const LAYOUT1: u32 = 1; // for channels > 2

// Audio Channel Status 0 Attributes
#[repr(C)]
pub union aud_ch_status_0 {
    pub regx: aud_ch_status_0_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_ch_status_0_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // ch_status:1, lpcm_id:1, cp_info:1, format:3, mode:2, ctg_code:8, src_num:4,
    // ch_num:4, samp_freq:4, clk_acc:2, rsvd:2
}

// samp_freq values - Sampling rate as per IEC60958 Ver 3
pub const CH_STATUS_MAP_32KHZ: u32 = 0x3;
pub const CH_STATUS_MAP_44KHZ: u32 = 0x0;
pub const CH_STATUS_MAP_48KHZ: u32 = 0x2;
pub const CH_STATUS_MAP_88KHZ: u32 = 0x8;
pub const CH_STATUS_MAP_96KHZ: u32 = 0xA;
pub const CH_STATUS_MAP_176KHZ: u32 = 0xC;
pub const CH_STATUS_MAP_192KHZ: u32 = 0xE;

// Audio Channel Status 1 Attributes
#[repr(C)]
pub union aud_ch_status_1 {
    pub regx: aud_ch_status_1_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_ch_status_1_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // max_wrd_len:1, wrd_len:3, rsvd:28
}

pub const MAX_SMPL_WIDTH_20: u32 = 0x0;
pub const MAX_SMPL_WIDTH_24: u32 = 0x1;
pub const SMPL_WIDTH_16BITS: u32 = 0x1;
pub const SMPL_WIDTH_24BITS: u32 = 0x5;

// CTS register
#[repr(C)]
pub union aud_hdmi_cts {
    pub regx: aud_hdmi_cts_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_hdmi_cts_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // cts_val:24, en_cts_prog:1, rsvd:7
}

// N register
#[repr(C)]
pub union aud_hdmi_n_enable {
    pub regx: aud_hdmi_n_enable_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_hdmi_n_enable_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // n_val:24, en_n_prog:1, rsvd:7
}

// Audio Buffer configurations
#[repr(C)]
pub union aud_buf_config {
    pub regx: aud_buf_config_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_buf_config_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // audio_fifo_watermark:8, dma_fifo_watermark:3, rsvd0:5,
    // aud_delay:8, rsvd1:8
}

pub const FIFO_THRESHOLD: u32 = 0xFE;
pub const DMA_FIFO_THRESHOLD: u32 = 0x7;

// Audio Sample Swapping offset
#[repr(C)]
pub union aud_buf_ch_swap {
    pub regx: aud_buf_ch_swap_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_buf_ch_swap_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // first_0:3, second_0:3, first_1:3, second_1:3, first_2:3, second_2:3,
    // first_3:3, second_3:3, rsvd:8
}

pub const SWAP_LFE_CENTER: u32 = 0x00fac4c8; // octal 76543210

// Address for Audio Buffer
#[repr(C)]
pub union aud_buf_addr {
    pub regx: aud_buf_addr_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_buf_addr_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // valid:1, intr_en:1, rsvd:4, addr:26
}

pub const AUD_BUF_VALID: u32 = (1u32 << 0);
pub const AUD_BUF_INTR_EN: u32 = (1u32 << 1);

// Length of Audio Buffer
#[repr(C)]
pub union aud_buf_len {
    pub regx: aud_buf_len_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_buf_len_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // buf_len:20, rsvd:12
}

// Audio Control State Register offset
#[repr(C)]
pub union aud_ctrl_st {
    pub regx: aud_ctrl_st_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_ctrl_st_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // ram_addr:4, eld_ack:1, eld_addr:4, eld_buf_size:5, eld_valid:1,
    // cp_ready:1, dip_freq:2, dip_idx:3, dip_en_sta:4, rsvd:7
}

// Audio HDMI Widget Data Island Packet offset
#[repr(C)]
pub union aud_info_frame1 {
    pub regx: aud_info_frame1_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_info_frame1_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // pkt_type:8, ver_num:8, len:5, rsvd:11
}

pub const HDMI_INFO_FRAME_WORD1: u32 = 0x000a0184;
pub const DP_INFO_FRAME_WORD1: u32 = 0x00441b84;

// DIP frame 2
#[repr(C)]
pub union aud_info_frame2 {
    pub regx: aud_info_frame2_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_info_frame2_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // chksum:8, chnl_cnt:3, rsvd0:1, coding_type:4, smpl_size:2,
    // smpl_freq:3, rsvd1:3, format:8
}

// DIP frame 3
#[repr(C)]
pub union aud_info_frame3 {
    pub regx: aud_info_frame3_regx,
    pub regval: u32,
}

#[repr(C)]
pub struct aud_info_frame3_regx {
    // Bitfield layout (C bitfields cannot be directly represented in Rust):
    // chnl_alloc:8, rsvd0:3, lsv:4, dm_inh:1, rsvd1:16
}

pub const VALID_DIP_WORDS: u32 = 3;

// AUD_HDMI_STATUS bits
pub const HDMI_AUDIO_UNDERRUN: u32 = (1u32 << 31);
pub const HDMI_AUDIO_BUFFER_DONE: u32 = (1u32 << 29);

// AUD_HDMI_STATUS register mask
pub const AUD_HDMI_STATUS_MASK_UNDERRUN: u32 = 0xC0000000;
pub const AUD_HDMI_STATUS_MASK_SRDBG: u32 = 0x00000002;
pub const AUD_HDMI_STATUSG_MASK_FUNCRST: u32 = 0x00000001;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
