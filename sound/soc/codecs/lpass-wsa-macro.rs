// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = ::core::primitive::u8;
type u16 = ::core::primitive::u16;
type u32 = ::core::primitive::u32;
type bool_ = bool;
type kernel_ulong_t = c_ulong;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 >> (31 - h)) & (!0u32 << l)
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 1;
const SND_SOC_NOPM: c_uint = !0u32;
const SNDRV_PCM_STREAM_PLAYBACK: c_uint = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_uint = 1;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_POST_PMD: c_int = 0x4;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_3LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 3;
const LPASS_MACRO_FLAG_HAS_NPL_CLOCK: kernel_ulong_t = 1;

const LPASS_CODEC_VERSION_1_0: c_int = 0x0100;
const LPASS_CODEC_VERSION_1_1: c_int = 0x0101;
const LPASS_CODEC_VERSION_1_2: c_int = 0x0102;
const LPASS_CODEC_VERSION_2_0: c_int = 0x0200;
const LPASS_CODEC_VERSION_2_1: c_int = 0x0201;
const LPASS_CODEC_VERSION_2_5: c_int = 0x0205;
const LPASS_CODEC_VERSION_2_6: c_int = 0x0206;
const LPASS_CODEC_VERSION_2_7: c_int = 0x0207;
const LPASS_CODEC_VERSION_2_8: c_int = 0x0208;
const LPASS_CODEC_VERSION_2_9: c_int = 0x0209;
const WSA_MACRO_SPKR_MODE_1: c_int = 1;

const CDC_WSA_CLK_RST_CTRL_MCLK_CONTROL: u32 = 0x0000;
const CDC_WSA_MCLK_EN_MASK: u32 = BIT(0);
const CDC_WSA_MCLK_ENABLE: u32 = BIT(0);
const CDC_WSA_MCLK_DISABLE: u32 = 0;
const CDC_WSA_CLK_RST_CTRL_FS_CNT_CONTROL: u32 = 0x0004;
const CDC_WSA_FS_CNT_EN_MASK: u32 = BIT(0);
const CDC_WSA_FS_CNT_ENABLE: u32 = BIT(0);
const CDC_WSA_FS_CNT_DISABLE: u32 = 0;
const CDC_WSA_CLK_RST_CTRL_SWR_CONTROL: u32 = 0x0008;
const CDC_WSA_SWR_CLK_EN_MASK: u32 = BIT(0);
const CDC_WSA_SWR_CLK_ENABLE: u32 = BIT(0);
const CDC_WSA_SWR_RST_EN_MASK: u32 = BIT(1);
const CDC_WSA_SWR_RST_ENABLE: u32 = BIT(1);
const CDC_WSA_SWR_RST_DISABLE: u32 = 0;
const CDC_WSA_TOP_TOP_CFG0: u32 = 0x0080;
const CDC_WSA_TOP_TOP_CFG1: u32 = 0x0084;
const CDC_WSA_TOP_FREQ_MCLK: u32 = 0x0088;
const CDC_WSA_TOP_DEBUG_BUS_SEL: u32 = 0x008C;
const CDC_WSA_TOP_DEBUG_EN0: u32 = 0x0090;
const CDC_WSA_TOP_DEBUG_EN1: u32 = 0x0094;
const CDC_WSA_TOP_DEBUG_DSM_LB: u32 = 0x0098;
const CDC_WSA_TOP_RX_I2S_CTL: u32 = 0x009C;
const CDC_WSA_TOP_TX_I2S_CTL: u32 = 0x00A0;
const CDC_WSA_TOP_I2S_CLK: u32 = 0x00A4;
const CDC_WSA_TOP_I2S_RESET: u32 = 0x00A8;
const CDC_WSA_RX_INP_MUX_RX_INT0_CFG0: u32 = 0x0100;
const CDC_WSA_RX_INP_MUX_RX_INT0_CFG1: u32 = 0x0104;
const CDC_WSA_RX_INP_MUX_RX_INT1_CFG0: u32 = 0x0108;
const CDC_WSA_RX_INP_MUX_RX_INT1_CFG1: u32 = 0x010C;
const CDC_WSA_RX_INP_MUX_RX_MIX_CFG0: u32 = 0x0110;
const CDC_WSA_RX_MIX_TX1_SEL_MASK: u32 = GENMASK(5, 3);
const CDC_WSA_RX_MIX_TX1_SEL_SHFT: u32 = 3;
const CDC_WSA_RX_MIX_TX0_SEL_MASK: u32 = GENMASK(2, 0);
const CDC_WSA_RX_INP_MUX_RX_EC_CFG0: u32 = 0x0114;
const CDC_WSA_RX_INP_MUX_SOFTCLIP_CFG0: u32 = 0x0118;
const CDC_WSA_TX0_SPKR_PROT_PATH_CTL: u32 = 0x0244;
const CDC_WSA_TX_SPKR_PROT_RESET_MASK: u32 = BIT(5);
const CDC_WSA_TX_SPKR_PROT_RESET: u32 = BIT(5);
const CDC_WSA_TX_SPKR_PROT_NO_RESET: u32 = 0;
const CDC_WSA_TX_SPKR_PROT_CLK_EN_MASK: u32 = BIT(4);
const CDC_WSA_TX_SPKR_PROT_CLK_ENABLE: u32 = BIT(4);
const CDC_WSA_TX_SPKR_PROT_CLK_DISABLE: u32 = 0;
const CDC_WSA_TX_SPKR_PROT_PCM_RATE_MASK: u32 = GENMASK(3, 0);
const CDC_WSA_TX_SPKR_PROT_PCM_RATE_8K: u32 = 0;
const CDC_WSA_TX_SPKR_PROT_PCM_RATE_16K: u32 = 1;
const CDC_WSA_TX_SPKR_PROT_PCM_RATE_24K: u32 = 2;
const CDC_WSA_TX_SPKR_PROT_PCM_RATE_32K: u32 = 3;
const CDC_WSA_TX_SPKR_PROT_PCM_RATE_48K: u32 = 4;
const CDC_WSA_TX0_SPKR_PROT_PATH_CFG0: u32 = 0x0248;
const CDC_WSA_TX1_SPKR_PROT_PATH_CTL: u32 = 0x0264;
const CDC_WSA_TX1_SPKR_PROT_PATH_CFG0: u32 = 0x0268;
const CDC_WSA_TX2_SPKR_PROT_PATH_CTL: u32 = 0x0284;
const CDC_WSA_TX2_SPKR_PROT_PATH_CFG0: u32 = 0x0288;
const CDC_WSA_TX3_SPKR_PROT_PATH_CTL: u32 = 0x02A4;
const CDC_WSA_TX3_SPKR_PROT_PATH_CFG0: u32 = 0x02A8;
const CDC_WSA_INTR_CTRL_CFG: u32 = 0x0340;
const CDC_WSA_INTR_CTRL_CLR_COMMIT: u32 = 0x0344;
const CDC_WSA_INTR_CTRL_PIN1_MASK0: u32 = 0x0360;
const CDC_WSA_INTR_CTRL_PIN1_STATUS0: u32 = 0x0368;
const CDC_WSA_INTR_CTRL_PIN1_CLEAR0: u32 = 0x0370;
const CDC_WSA_INTR_CTRL_PIN2_MASK0: u32 = 0x0380;
const CDC_WSA_INTR_CTRL_PIN2_STATUS0: u32 = 0x0388;
const CDC_WSA_INTR_CTRL_PIN2_CLEAR0: u32 = 0x0390;
const CDC_WSA_INTR_CTRL_LEVEL0: u32 = 0x03C0;
const CDC_WSA_INTR_CTRL_BYPASS0: u32 = 0x03C8;
const CDC_WSA_INTR_CTRL_SET0: u32 = 0x03D0;
const CDC_WSA_RX0_RX_PATH_CTL: u32 = 0x0400;
const CDC_WSA_RX_PATH_CLK_EN_MASK: u32 = BIT(5);
const CDC_WSA_RX_PATH_CLK_ENABLE: u32 = BIT(5);
const CDC_WSA_RX_PATH_CLK_DISABLE: u32 = 0;
const CDC_WSA_RX_PATH_PGA_MUTE_EN_MASK: u32 = BIT(4);
const CDC_WSA_RX_PATH_PGA_MUTE_ENABLE: u32 = BIT(4);
const CDC_WSA_RX_PATH_PGA_MUTE_DISABLE: u32 = 0;
const CDC_WSA_RX0_RX_PATH_CFG0: u32 = 0x0404;
const CDC_WSA_RX_PATH_COMP_EN_MASK: u32 = BIT(1);
const CDC_WSA_RX_PATH_COMP_ENABLE: u32 = BIT(1);
const CDC_WSA_RX_PATH_HD2_EN_MASK: u32 = BIT(2);
const CDC_WSA_RX_PATH_HD2_ENABLE: u32 = BIT(2);
const CDC_WSA_RX_PATH_SPKR_RATE_MASK: u32 = BIT(3);
const CDC_WSA_RX_PATH_SPKR_RATE_FS_2P4_3P072: u32 = BIT(3);
const CDC_WSA_RX0_RX_PATH_CFG1: u32 = 0x0408;
const CDC_WSA_RX_PATH_SMART_BST_EN_MASK: u32 = BIT(0);
const CDC_WSA_RX_PATH_SMART_BST_ENABLE: u32 = BIT(0);
const CDC_WSA_RX_PATH_SMART_BST_DISABLE: u32 = 0;
const CDC_WSA_RX0_RX_PATH_CFG2: u32 = 0x040C;
const CDC_WSA_RX0_RX_PATH_CFG3: u32 = 0x0410;
const CDC_WSA_RX_DC_DCOEFF_MASK: u32 = GENMASK(1, 0);
const CDC_WSA_RX0_RX_VOL_CTL: u32 = 0x0414;
const CDC_WSA_RX0_RX_PATH_MIX_CTL: u32 = 0x0418;
const CDC_WSA_RX_PATH_MIX_CLK_EN_MASK: u32 = BIT(5);
const CDC_WSA_RX_PATH_MIX_CLK_ENABLE: u32 = BIT(5);
const CDC_WSA_RX_PATH_MIX_CLK_DISABLE: u32 = 0;
const CDC_WSA_RX0_RX_PATH_MIX_CFG: u32 = 0x041C;
const CDC_WSA_RX0_RX_VOL_MIX_CTL: u32 = 0x0420;
const CDC_WSA_RX0_RX_PATH_SEC0: u32 = 0x0424;
const CDC_WSA_RX0_RX_PATH_SEC1: u32 = 0x0428;
const CDC_WSA_RX_PGA_HALF_DB_MASK: u32 = BIT(0);
const CDC_WSA_RX_PGA_HALF_DB_ENABLE: u32 = BIT(0);
const CDC_WSA_RX_PGA_HALF_DB_DISABLE: u32 = 0;
const CDC_WSA_RX0_RX_PATH_SEC2: u32 = 0x042C;
const CDC_WSA_RX0_RX_PATH_SEC3: u32 = 0x0430;
const CDC_WSA_RX_PATH_HD2_SCALE_MASK: u32 = GENMASK(1, 0);
const CDC_WSA_RX_PATH_HD2_ALPHA_MASK: u32 = GENMASK(5, 2);
const CDC_WSA_RX0_RX_PATH_SEC5: u32 = 0x0438;
const CDC_WSA_RX0_RX_PATH_SEC6: u32 = 0x043C;
const CDC_WSA_RX0_RX_PATH_SEC7: u32 = 0x0440;
const CDC_WSA_RX0_RX_PATH_MIX_SEC0: u32 = 0x0444;
const CDC_WSA_RX0_RX_PATH_MIX_SEC1: u32 = 0x0448;
const CDC_WSA_RX0_RX_PATH_DSMDEM_CTL: u32 = 0x044C;
const CDC_WSA_RX_DSMDEM_CLK_EN_MASK: u32 = BIT(0);
const CDC_WSA_RX_DSMDEM_CLK_ENABLE: u32 = BIT(0);
const CDC_WSA_RX1_RX_PATH_CTL: u32 = 0x0480;
const CDC_WSA_RX1_RX_PATH_CFG0: u32 = 0x0484;
const CDC_WSA_RX1_RX_PATH_CFG1: u32 = 0x0488;
const CDC_WSA_RX1_RX_PATH_CFG2: u32 = 0x048C;
const CDC_WSA_RX1_RX_PATH_CFG3: u32 = 0x0490;
const CDC_WSA_RX1_RX_VOL_CTL: u32 = 0x0494;
const CDC_WSA_RX1_RX_PATH_MIX_CTL: u32 = 0x0498;
const CDC_WSA_RX1_RX_PATH_MIX_CFG: u32 = 0x049C;
const CDC_WSA_RX1_RX_VOL_MIX_CTL: u32 = 0x04A0;
const CDC_WSA_RX1_RX_PATH_SEC0: u32 = 0x04A4;
const CDC_WSA_RX1_RX_PATH_SEC1: u32 = 0x04A8;
const CDC_WSA_RX1_RX_PATH_SEC2: u32 = 0x04AC;
const CDC_WSA_RX1_RX_PATH_SEC3: u32 = 0x04B0;
const CDC_WSA_RX1_RX_PATH_SEC5: u32 = 0x04B8;
const CDC_WSA_RX1_RX_PATH_SEC6: u32 = 0x04BC;
const CDC_WSA_RX1_RX_PATH_SEC7: u32 = 0x04C0;
const CDC_WSA_RX1_RX_PATH_MIX_SEC0: u32 = 0x04C4;
const CDC_WSA_RX1_RX_PATH_MIX_SEC1: u32 = 0x04C8;
const CDC_WSA_RX1_RX_PATH_DSMDEM_CTL: u32 = 0x04CC;
const CDC_WSA_BOOST0_BOOST_PATH_CTL: u32 = 0x0500;
const CDC_WSA_BOOST_PATH_CLK_EN_MASK: u32 = BIT(4);
const CDC_WSA_BOOST_PATH_CLK_ENABLE: u32 = BIT(4);
const CDC_WSA_BOOST_PATH_CLK_DISABLE: u32 = 0;
const CDC_WSA_BOOST0_BOOST_CTL: u32 = 0x0504;
const CDC_WSA_BOOST0_BOOST_CFG1: u32 = 0x0508;
const CDC_WSA_BOOST0_BOOST_CFG2: u32 = 0x050C;
const CDC_WSA_BOOST1_BOOST_PATH_CTL: u32 = 0x0540;
const CDC_WSA_BOOST1_BOOST_CTL: u32 = 0x0544;
const CDC_WSA_BOOST1_BOOST_CFG1: u32 = 0x0548;
const CDC_WSA_BOOST1_BOOST_CFG2: u32 = 0x054C;
const CDC_WSA_COMPANDER0_CTL0: u32 = 0x0580;
const CDC_WSA_COMPANDER_CLK_EN_MASK: u32 = BIT(0);
const CDC_WSA_COMPANDER_CLK_ENABLE: u32 = BIT(0);
const CDC_WSA_COMPANDER_SOFT_RST_MASK: u32 = BIT(1);
const CDC_WSA_COMPANDER_SOFT_RST_ENABLE: u32 = BIT(1);
const CDC_WSA_COMPANDER_HALT_MASK: u32 = BIT(2);
const CDC_WSA_COMPANDER_HALT: u32 = BIT(2);
const CDC_WSA_COMPANDER0_CTL1: u32 = 0x0584;
const CDC_WSA_COMPANDER0_CTL2: u32 = 0x0588;
const CDC_WSA_COMPANDER0_CTL3: u32 = 0x058C;
const CDC_WSA_COMPANDER0_CTL4: u32 = 0x0590;
const CDC_WSA_COMPANDER0_CTL5: u32 = 0x0594;
const CDC_WSA_COMPANDER0_CTL6: u32 = 0x0598;
const CDC_WSA_COMPANDER0_CTL7: u32 = 0x059C;
/* CDC_WSA_COMPANDER1_CTLx and CDC_WSA_SOFTCLIPx differ per LPASS codec versions */
const CDC_WSA_EC_HQ0_EC_REF_HQ_PATH_CTL: u32 = 0x0680;
const CDC_WSA_EC_HQ_EC_CLK_EN_MASK: u32 = BIT(0);
const CDC_WSA_EC_HQ_EC_CLK_ENABLE: u32 = BIT(0);
const CDC_WSA_EC_HQ0_EC_REF_HQ_CFG0: u32 = 0x0684;
const CDC_WSA_EC_HQ_EC_REF_PCM_RATE_MASK: u32 = GENMASK(4, 1);
const CDC_WSA_EC_HQ_EC_REF_PCM_RATE_48K: u32 = BIT(3);
const CDC_WSA_EC_HQ1_EC_REF_HQ_PATH_CTL: u32 = 0x06C0;
const CDC_WSA_EC_HQ1_EC_REF_HQ_CFG0: u32 = 0x06C4;
const CDC_WSA_SPLINE_ASRC0_CLK_RST_CTL: u32 = 0x0700;
const CDC_WSA_SPLINE_ASRC0_CTL0: u32 = 0x0704;
const CDC_WSA_SPLINE_ASRC0_CTL1: u32 = 0x0708;
const CDC_WSA_SPLINE_ASRC0_FIFO_CTL: u32 = 0x070C;
const CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_LSB: u32 = 0x0710;
const CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_MSB: u32 = 0x0714;
const CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_LSB: u32 = 0x0718;
const CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_MSB: u32 = 0x071C;
const CDC_WSA_SPLINE_ASRC0_STATUS_FIFO: u32 = 0x0720;
const CDC_WSA_SPLINE_ASRC1_CLK_RST_CTL: u32 = 0x0740;
const CDC_WSA_SPLINE_ASRC1_CTL0: u32 = 0x0744;
const CDC_WSA_SPLINE_ASRC1_CTL1: u32 = 0x0748;
const CDC_WSA_SPLINE_ASRC1_FIFO_CTL: u32 = 0x074C;
const CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_LSB: u32 = 0x0750;
const CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_MSB: u32 = 0x0754;
const CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_LSB: u32 = 0x0758;
const CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_MSB: u32 = 0x075C;
const CDC_WSA_SPLINE_ASRC1_STATUS_FIFO: u32 = 0x0760;
const WSA_MAX_OFFSET: u32 = 0x0760;

/* LPASS codec version <=2.4 register offsets */
const CDC_WSA_COMPANDER1_CTL0: u32 = 0x05C0;
const CDC_WSA_COMPANDER1_CTL1: u32 = 0x05C4;
const CDC_WSA_COMPANDER1_CTL2: u32 = 0x05C8;
const CDC_WSA_COMPANDER1_CTL3: u32 = 0x05CC;
const CDC_WSA_COMPANDER1_CTL4: u32 = 0x05D0;
const CDC_WSA_COMPANDER1_CTL5: u32 = 0x05D4;
const CDC_WSA_COMPANDER1_CTL6: u32 = 0x05D8;
const CDC_WSA_COMPANDER1_CTL7: u32 = 0x05DC;
const CDC_WSA_SOFTCLIP0_CRC: u32 = 0x0600;
const CDC_WSA_SOFTCLIP_CLK_EN_MASK: u32 = BIT(0);
const CDC_WSA_SOFTCLIP_CLK_ENABLE: u32 = BIT(0);
const CDC_WSA_SOFTCLIP0_SOFTCLIP_CTRL: u32 = 0x0604;
const CDC_WSA_SOFTCLIP_EN_MASK: u32 = BIT(0);
const CDC_WSA_SOFTCLIP_ENABLE: u32 = BIT(0);
const CDC_WSA_SOFTCLIP1_CRC: u32 = 0x0640;
const CDC_WSA_SOFTCLIP1_SOFTCLIP_CTRL: u32 = 0x0644;

/* LPASS codec version >=2.5 register offsets */
const CDC_WSA_TOP_FS_UNGATE: u32 = 0x00AC;
const CDC_WSA_TOP_GRP_SEL: u32 = 0x00B0;
const CDC_WSA_TOP_FS_UNGATE2: u32 = 0x00DC;
const CDC_2_5_WSA_COMPANDER0_CTL8: u32 = 0x05A0;
const CDC_2_5_WSA_COMPANDER0_CTL9: u32 = 0x05A4;
const CDC_2_5_WSA_COMPANDER0_CTL10: u32 = 0x05A8;
const CDC_2_5_WSA_COMPANDER0_CTL11: u32 = 0x05AC;
const CDC_2_5_WSA_COMPANDER0_CTL12: u32 = 0x05B0;
const CDC_2_5_WSA_COMPANDER0_CTL13: u32 = 0x05B4;
const CDC_2_5_WSA_COMPANDER0_CTL14: u32 = 0x05B8;
const CDC_2_5_WSA_COMPANDER0_CTL15: u32 = 0x05BC;
const CDC_2_5_WSA_COMPANDER0_CTL16: u32 = 0x05C0;
const CDC_2_5_WSA_COMPANDER0_CTL17: u32 = 0x05C4;
const CDC_2_5_WSA_COMPANDER0_CTL18: u32 = 0x05C8;
const CDC_2_5_WSA_COMPANDER0_CTL19: u32 = 0x05CC;
const CDC_2_5_WSA_COMPANDER1_CTL0: u32 = 0x05E0;
const CDC_2_5_WSA_COMPANDER1_CTL1: u32 = 0x05E4;
const CDC_2_5_WSA_COMPANDER1_CTL2: u32 = 0x05E8;
const CDC_2_5_WSA_COMPANDER1_CTL3: u32 = 0x05EC;
const CDC_2_5_WSA_COMPANDER1_CTL4: u32 = 0x05F0;
const CDC_2_5_WSA_COMPANDER1_CTL5: u32 = 0x05F4;
const CDC_2_5_WSA_COMPANDER1_CTL6: u32 = 0x05F8;
const CDC_2_5_WSA_COMPANDER1_CTL7: u32 = 0x05FC;
const CDC_2_5_WSA_COMPANDER1_CTL8: u32 = 0x0600;
const CDC_2_5_WSA_COMPANDER1_CTL9: u32 = 0x0604;
const CDC_2_5_WSA_COMPANDER1_CTL10: u32 = 0x0608;
const CDC_2_5_WSA_COMPANDER1_CTL11: u32 = 0x060C;
const CDC_2_5_WSA_COMPANDER1_CTL12: u32 = 0x0610;
const CDC_2_5_WSA_COMPANDER1_CTL13: u32 = 0x0614;
const CDC_2_5_WSA_COMPANDER1_CTL14: u32 = 0x0618;
const CDC_2_5_WSA_COMPANDER1_CTL15: u32 = 0x061C;
const CDC_2_5_WSA_COMPANDER1_CTL16: u32 = 0x0620;
const CDC_2_5_WSA_COMPANDER1_CTL17: u32 = 0x0624;
const CDC_2_5_WSA_COMPANDER1_CTL18: u32 = 0x0628;
const CDC_2_5_WSA_COMPANDER1_CTL19: u32 = 0x062C;
const CDC_2_5_WSA_SOFTCLIP0_CRC: u32 = 0x0640;
const CDC_2_5_WSA_SOFTCLIP0_SOFTCLIP_CTRL: u32 = 0x0644;
const CDC_2_5_WSA_SOFTCLIP1_CRC: u32 = 0x0660;
const CDC_2_5_WSA_SOFTCLIP1_SOFTCLIP_CTRL: u32 = 0x0664;

const WSA_MACRO_RX_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const WSA_MACRO_RX_MIX_RATES: c_uint = SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const WSA_MACRO_RX_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE;
const WSA_MACRO_ECHO_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000;
const WSA_MACRO_ECHO_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_3LE;

const NUM_INTERPOLATORS: usize = 2;
const WSA_NUM_CLKS_MAX: usize = 5;
const WSA_MACRO_MCLK_FREQ: c_uint = 19200000;
const WSA_MACRO_MUX_CFG_OFFSET: u32 = 0x8;
const WSA_MACRO_MUX_CFG1_OFFSET: u32 = 0x4;
const WSA_MACRO_RX_PATH_OFFSET: u32 = 0x80;
const WSA_MACRO_RX_PATH_CFG3_OFFSET: u32 = 0x10;
const WSA_MACRO_RX_PATH_DSMDEM_OFFSET: u32 = 0x4C;
const WSA_MACRO_FS_RATE_MASK: u32 = 0x0F;
const WSA_MACRO_EC_MIX_TX0_MASK: u32 = 0x03;
const WSA_MACRO_EC_MIX_TX1_MASK: u32 = 0x18;
const WSA_MACRO_MAX_DMA_CH_PER_PORT: u16 = 0x2;

const WSA_MACRO_GAIN_OFFSET_M1P5_DB: usize = 0;
const WSA_MACRO_GAIN_OFFSET_0_DB: usize = 1;
const WSA_MACRO_RX0: usize = 0;
const WSA_MACRO_RX1: usize = 1;
const WSA_MACRO_RX_MIX0: usize = 2;
const WSA_MACRO_RX_MIX1: usize = 3;
const WSA_MACRO_RX_MAX: usize = 4;
const WSA_MACRO_TX0: usize = 0;
const WSA_MACRO_TX1: usize = 1;
const WSA_MACRO_TX_MAX: usize = 2;
const WSA_MACRO_EC0_MUX: usize = 0;
const WSA_MACRO_EC1_MUX: usize = 1;
const WSA_MACRO_EC_MUX_MAX: usize = 2;
const WSA_MACRO_COMP1: usize = 0; /* SPK_L */
const WSA_MACRO_COMP2: usize = 1; /* SPK_R */
const WSA_MACRO_COMP_MAX: usize = 2;
const WSA_MACRO_SOFTCLIP0: usize = 0; /* RX0 */
const WSA_MACRO_SOFTCLIP1: usize = 1; /* RX1 */
const WSA_MACRO_SOFTCLIP_MAX: usize = 2;
const INTn_1_INP_SEL_ZERO: u8 = 0;
const INTn_1_INP_SEL_RX0: u8 = 1;
const INTn_2_INP_SEL_ZERO: u8 = 0;
const INTn_2_INP_SEL_RX0: u8 = 1;
const WSA_MACRO_AIF1_PB: usize = 0;
const WSA_MACRO_AIF_MIX1_PB: usize = 1;
const WSA_MACRO_AIF_VI: usize = 2;
const WSA_MACRO_AIF_ECHO: usize = 3;
const WSA_MACRO_MAX_DAIS: usize = 4;

#[repr(C)]
struct device { _private: [u8; 0] }
#[repr(C)]
struct platform_device { dev: device }
#[repr(C)]
struct regmap { _private: [u8; 0] }
#[repr(C)]
struct clk { _private: [u8; 0] }
#[repr(C)]
struct clk_hw { init: *const clk_init_data }
#[repr(C)]
struct snd_soc_component { dev: *mut device }
#[repr(C)]
struct snd_soc_dai { id: c_int, component: *mut snd_soc_component }
#[repr(C)]
struct snd_pcm_substream { stream: c_uint }
#[repr(C)]
struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_widget { dapm: *mut snd_soc_dapm_context, shift: c_uint, name: *const c_char }
#[repr(C)]
struct snd_kcontrol { private_value: c_ulong }
#[repr(C)]
struct soc_mixer_control { shift: c_int }
#[repr(C)]
struct snd_soc_dapm_update { _private: [u8; 0] }
#[repr(C)]
struct soc_enum { reg: c_uint, shift_l: c_uint, items: c_uint, texts: *const *const c_char }
#[repr(C)]
struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    get_channel_map: Option<unsafe extern "C" fn(*const snd_soc_dai, *mut c_uint, *mut c_uint, *mut c_uint, *mut c_uint) -> c_int>,
}
#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    rates: c_uint,
    formats: c_uint,
    rate_max: c_uint,
    rate_min: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
struct reg_default { reg: c_uint, def: c_uint }
#[repr(C)]
struct regmap_config {
    name: *const c_char,
    reg_bits: c_uint,
    val_bits: c_uint,
    reg_stride: c_uint,
    cache_type: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_int,
    max_register: c_uint,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}
#[repr(C)]
struct clk_ops {
    prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
}
#[repr(C)]
struct clk_init_data {
    name: *const c_char,
    ops: *const clk_ops,
    flags: c_uint,
    parent_names: *const *const c_char,
    num_parents: u8,
}
#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget_desc,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
}
#[repr(C)]
struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)]
struct of_device_id { compatible: *const c_char, data: *const c_void }
#[repr(C)]
struct platform_driver { driver: driver_inner, probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int> }
#[repr(C)]
struct driver_inner { name: *const c_char, of_match_table: *const of_device_id, pm: *const dev_pm_ops }
#[repr(C)]
struct snd_ctl_elem_value { value: snd_ctl_elem_value_union }
#[repr(C)]
union snd_ctl_elem_value_union { integer: snd_ctl_elem_value_integer, enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer { value: [c_long; 4] }
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_enumerated { item: [c_uint; 4] }
type c_long = isize;

#[repr(C)]
struct interp_sample_rate { sample_rate: c_int, rate_val: c_int }

static int_prim_sample_rate_val: [interp_sample_rate; 9] = [
    interp_sample_rate { sample_rate: 8000, rate_val: 0x0 }, /* 8K */
    interp_sample_rate { sample_rate: 16000, rate_val: 0x1 }, /* 16K */
    interp_sample_rate { sample_rate: 24000, rate_val: -EINVAL }, /* 24K */
    interp_sample_rate { sample_rate: 32000, rate_val: 0x3 }, /* 32K */
    interp_sample_rate { sample_rate: 48000, rate_val: 0x4 }, /* 48K */
    interp_sample_rate { sample_rate: 96000, rate_val: 0x5 }, /* 96K */
    interp_sample_rate { sample_rate: 192000, rate_val: 0x6 }, /* 192K */
    interp_sample_rate { sample_rate: 384000, rate_val: 0x7 }, /* 384K */
    interp_sample_rate { sample_rate: 44100, rate_val: 0x8 }, /* 44.1K */
];

static int_mix_sample_rate_val: [interp_sample_rate; 3] = [
    interp_sample_rate { sample_rate: 48000, rate_val: 0x4 }, /* 48K */
    interp_sample_rate { sample_rate: 96000, rate_val: 0x5 }, /* 96K */
    interp_sample_rate { sample_rate: 192000, rate_val: 0x6 }, /* 192K */
];

/**
 * struct wsa_reg_layout - Register layout differences
 * @rx_intx_1_mix_inp0_sel_mask: register mask for RX_INTX_1_MIX_INP0_SEL_MASK
 * @rx_intx_1_mix_inp1_sel_mask: register mask for RX_INTX_1_MIX_INP1_SEL_MASK
 * @rx_intx_1_mix_inp2_sel_mask: register mask for RX_INTX_1_MIX_INP2_SEL_MASK
 * @rx_intx_2_sel_mask: register mask for RX_INTX_2_SEL_MASK
 * @compander1_reg_offset: offset between compander registers (compander1 - compander0)
 * @softclip0_reg_base: base address of softclip0 register
 * @softclip1_reg_offset: offset between compander registers (softclip1 - softclip0)
 */
#[repr(C)]
struct wsa_reg_layout {
    rx_intx_1_mix_inp0_sel_mask: c_uint,
    rx_intx_1_mix_inp1_sel_mask: c_uint,
    rx_intx_1_mix_inp2_sel_mask: c_uint,
    rx_intx_2_sel_mask: c_uint,
    compander1_reg_offset: c_uint,
    softclip0_reg_base: c_uint,
    softclip1_reg_offset: c_uint,
}

#[repr(C)]
struct wsa_macro {
    dev: *mut device,
    comp_enabled: [c_int; WSA_MACRO_COMP_MAX],
    ec_hq: [c_int; WSA_MACRO_RX1 + 1],
    prim_int_users: [u16; WSA_MACRO_RX1 + 1],
    wsa_mclk_users: u16,
    codec_version: c_int,
    reg_layout: *const wsa_reg_layout,
    active_ch_mask: [c_ulong; WSA_MACRO_MAX_DAIS],
    active_ch_cnt: [c_ulong; WSA_MACRO_MAX_DAIS],
    rx_port_value: [c_int; WSA_MACRO_RX_MAX],
    ear_spkr_gain: c_int,
    spkr_gain_offset: c_int,
    spkr_mode: c_int,
    pcm_rate_vi: u32,
    is_softclip_on: [c_int; WSA_MACRO_SOFTCLIP_MAX],
    softclip_clk_users: [c_int; WSA_MACRO_SOFTCLIP_MAX],
    regmap: *mut regmap,
    mclk: *mut clk,
    npl: *mut clk,
    macro_: *mut clk,
    dcodec: *mut clk,
    fsgen: *mut clk,
    hw: clk_hw,
}

unsafe fn to_wsa_macro(hw: *mut clk_hw) -> *mut wsa_macro {
    (hw as *mut u8).sub(core::mem::offset_of!(wsa_macro, hw)) as *mut wsa_macro
}

static wsa_codec_v2_1: wsa_reg_layout = wsa_reg_layout {
    rx_intx_1_mix_inp0_sel_mask: GENMASK(2, 0),
    rx_intx_1_mix_inp1_sel_mask: GENMASK(5, 3),
    rx_intx_1_mix_inp2_sel_mask: GENMASK(5, 3),
    rx_intx_2_sel_mask: GENMASK(2, 0),
    compander1_reg_offset: 0x40,
    softclip0_reg_base: 0x600,
    softclip1_reg_offset: 0x40,
};

static wsa_codec_v2_5: wsa_reg_layout = wsa_reg_layout {
    rx_intx_1_mix_inp0_sel_mask: GENMASK(3, 0),
    rx_intx_1_mix_inp1_sel_mask: GENMASK(7, 4),
    rx_intx_1_mix_inp2_sel_mask: GENMASK(7, 4),
    rx_intx_2_sel_mask: GENMASK(3, 0),
    compander1_reg_offset: 0x60,
    softclip0_reg_base: 0x640,
    softclip1_reg_offset: 0x20,
};

static digital_gain: [c_int; 3] = [-8400, 100, -8400];

static rx_text_v2_1: [&[u8]; 7] = [b"ZERO\0", b"RX0\0", b"RX1\0", b"RX_MIX0\0", b"RX_MIX1\0", b"DEC0\0", b"DEC1\0"];
static rx_text_v2_5: [&[u8]; 12] = [b"ZERO\0", b"RX0\0", b"RX1\0", b"RX_MIX0\0", b"RX_MIX1\0", b"RX4\0", b"RX5\0", b"RX6\0", b"RX7\0", b"RX8\0", b"DEC0\0", b"DEC1\0"];
static rx_mix_text_v2_1: [&[u8]; 5] = [b"ZERO\0", b"RX0\0", b"RX1\0", b"RX_MIX0\0", b"RX_MIX1\0"];
static rx_mix_text_v2_5: [&[u8]; 10] = [b"ZERO\0", b"RX0\0", b"RX1\0", b"RX_MIX0\0", b"RX_MIX1\0", b"RX4\0", b"RX5\0", b"RX6\0", b"RX7\0", b"RX8\0"];
static rx_mix_ec_text: [&[u8]; 3] = [b"ZERO\0", b"RX_MIX_TX0\0", b"RX_MIX_TX1\0"];
/* Order must match WSA_MACRO_MAX_DAIS enum (offset by 1) */
static rx_mux_text: [&[u8]; 3] = [b"ZERO\0", b"AIF1_PB\0", b"AIF_MIX1_PB\0"];
static rx_sidetone_mix_text: [&[u8]; 2] = [b"ZERO\0", b"SRC0\0"];
static wsa_macro_ear_spkr_pa_gain_text: [&[u8]; 8] = [b"G_DEFAULT\0", b"G_0_DB\0", b"G_1_DB\0", b"G_2_DB\0", b"G_3_DB\0", b"G_4_DB\0", b"G_5_DB\0", b"G_6_DB\0"];

/* SOC_ENUM_* and SND_SOC_DAPM_* macros expand to external ALSA structures.
 * Their concrete Rust layout is a dependency of the surrounding kernel bindings;
 * these local declarations preserve the source-level items and names.
 */
static wsa_macro_ear_spkr_pa_gain_enum: soc_enum = soc_enum { reg: SND_SOC_NOPM, shift_l: 0, items: 8, texts: ptr::null() };
static rx_mux_enum: soc_enum = soc_enum { reg: SND_SOC_NOPM, shift_l: 0, items: 3, texts: ptr::null() };
static rx0_prim_inp0_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG0, shift_l: 0, items: 7, texts: ptr::null() };
static rx0_prim_inp1_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG0, shift_l: 3, items: 7, texts: ptr::null() };
static rx0_prim_inp2_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG1, shift_l: 3, items: 7, texts: ptr::null() };
static rx0_mix_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG1, shift_l: 0, items: 5, texts: ptr::null() };
static rx0_prim_inp0_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG0, shift_l: 0, items: 12, texts: ptr::null() };
static rx0_prim_inp1_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG0, shift_l: 4, items: 12, texts: ptr::null() };
static rx0_prim_inp2_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG1, shift_l: 4, items: 12, texts: ptr::null() };
static rx0_mix_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG1, shift_l: 0, items: 10, texts: ptr::null() };
static rx0_sidetone_mix_enum: soc_enum = soc_enum { reg: SND_SOC_NOPM, shift_l: 0, items: 2, texts: ptr::null() };
static rx1_prim_inp0_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG0, shift_l: 0, items: 7, texts: ptr::null() };
static rx1_prim_inp1_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG0, shift_l: 3, items: 7, texts: ptr::null() };
static rx1_prim_inp2_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG1, shift_l: 3, items: 7, texts: ptr::null() };
static rx1_mix_chain_enum_v2_1: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG1, shift_l: 0, items: 5, texts: ptr::null() };
static rx1_prim_inp0_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG0, shift_l: 0, items: 12, texts: ptr::null() };
static rx1_prim_inp1_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG0, shift_l: 4, items: 12, texts: ptr::null() };
static rx1_prim_inp2_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG1, shift_l: 4, items: 12, texts: ptr::null() };
static rx1_mix_chain_enum_v2_5: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG1, shift_l: 0, items: 10, texts: ptr::null() };
static rx_mix_ec0_enum: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_MIX_CFG0, shift_l: 0, items: 3, texts: ptr::null() };
static rx_mix_ec1_enum: soc_enum = soc_enum { reg: CDC_WSA_RX_INP_MUX_RX_MIX_CFG0, shift_l: 3, items: 3, texts: ptr::null() };

static wsa_defaults: [reg_default; 104] = [
    reg_default { reg: CDC_WSA_CLK_RST_CTRL_MCLK_CONTROL, def: 0x00 }, reg_default { reg: CDC_WSA_CLK_RST_CTRL_FS_CNT_CONTROL, def: 0x00 },
    reg_default { reg: CDC_WSA_CLK_RST_CTRL_SWR_CONTROL, def: 0x00 }, reg_default { reg: CDC_WSA_TOP_TOP_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_TOP_TOP_CFG1, def: 0x00 }, reg_default { reg: CDC_WSA_TOP_FREQ_MCLK, def: 0x00 },
    reg_default { reg: CDC_WSA_TOP_DEBUG_BUS_SEL, def: 0x00 }, reg_default { reg: CDC_WSA_TOP_DEBUG_EN0, def: 0x00 },
    reg_default { reg: CDC_WSA_TOP_DEBUG_EN1, def: 0x00 }, reg_default { reg: CDC_WSA_TOP_DEBUG_DSM_LB, def: 0x88 },
    reg_default { reg: CDC_WSA_TOP_RX_I2S_CTL, def: 0x0C }, reg_default { reg: CDC_WSA_TOP_TX_I2S_CTL, def: 0x0C },
    reg_default { reg: CDC_WSA_TOP_I2S_CLK, def: 0x02 }, reg_default { reg: CDC_WSA_TOP_I2S_RESET, def: 0x00 },
    reg_default { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG0, def: 0x00 }, reg_default { reg: CDC_WSA_RX_INP_MUX_RX_INT0_CFG1, def: 0x00 },
    reg_default { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG0, def: 0x00 }, reg_default { reg: CDC_WSA_RX_INP_MUX_RX_INT1_CFG1, def: 0x00 },
    reg_default { reg: CDC_WSA_RX_INP_MUX_RX_MIX_CFG0, def: 0x00 }, reg_default { reg: CDC_WSA_RX_INP_MUX_RX_EC_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_RX_INP_MUX_SOFTCLIP_CFG0, def: 0x00 }, reg_default { reg: CDC_WSA_INTR_CTRL_CFG, def: 0x00 },
    reg_default { reg: CDC_WSA_INTR_CTRL_CLR_COMMIT, def: 0x00 }, reg_default { reg: CDC_WSA_INTR_CTRL_PIN1_MASK0, def: 0xFF },
    reg_default { reg: CDC_WSA_INTR_CTRL_PIN1_STATUS0, def: 0x00 }, reg_default { reg: CDC_WSA_INTR_CTRL_PIN1_CLEAR0, def: 0x00 },
    reg_default { reg: CDC_WSA_INTR_CTRL_PIN2_MASK0, def: 0xFF }, reg_default { reg: CDC_WSA_INTR_CTRL_PIN2_STATUS0, def: 0x00 },
    reg_default { reg: CDC_WSA_INTR_CTRL_PIN2_CLEAR0, def: 0x00 }, reg_default { reg: CDC_WSA_INTR_CTRL_LEVEL0, def: 0x00 },
    reg_default { reg: CDC_WSA_INTR_CTRL_BYPASS0, def: 0x00 }, reg_default { reg: CDC_WSA_INTR_CTRL_SET0, def: 0x00 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_CTL, def: 0x04 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_CFG1, def: 0x64 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_CFG2, def: 0x8F },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_CFG3, def: 0x00 }, reg_default { reg: CDC_WSA_RX0_RX_VOL_CTL, def: 0x00 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_MIX_CTL, def: 0x04 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_MIX_CFG, def: 0x7E },
    reg_default { reg: CDC_WSA_RX0_RX_VOL_MIX_CTL, def: 0x00 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_SEC0, def: 0x04 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_SEC1, def: 0x08 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_SEC2, def: 0x00 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_SEC3, def: 0x00 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_SEC5, def: 0x00 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_SEC6, def: 0x00 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_SEC7, def: 0x00 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_MIX_SEC0, def: 0x08 }, reg_default { reg: CDC_WSA_RX0_RX_PATH_MIX_SEC1, def: 0x00 },
    reg_default { reg: CDC_WSA_RX0_RX_PATH_DSMDEM_CTL, def: 0x00 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_CFG1, def: 0x64 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_CFG2, def: 0x8F },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_CFG3, def: 0x00 }, reg_default { reg: CDC_WSA_RX1_RX_VOL_CTL, def: 0x00 },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_MIX_CTL, def: 0x04 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_MIX_CFG, def: 0x7E },
    reg_default { reg: CDC_WSA_RX1_RX_VOL_MIX_CTL, def: 0x00 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_SEC0, def: 0x04 },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_SEC1, def: 0x08 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_SEC2, def: 0x00 },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_SEC3, def: 0x00 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_SEC5, def: 0x00 },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_SEC6, def: 0x00 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_SEC7, def: 0x00 },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_MIX_SEC0, def: 0x08 }, reg_default { reg: CDC_WSA_RX1_RX_PATH_MIX_SEC1, def: 0x00 },
    reg_default { reg: CDC_WSA_RX1_RX_PATH_DSMDEM_CTL, def: 0x00 }, reg_default { reg: CDC_WSA_BOOST0_BOOST_PATH_CTL, def: 0x00 },
    reg_default { reg: CDC_WSA_BOOST0_BOOST_CTL, def: 0xD0 }, reg_default { reg: CDC_WSA_BOOST0_BOOST_CFG1, def: 0x89 },
    reg_default { reg: CDC_WSA_BOOST0_BOOST_CFG2, def: 0x04 }, reg_default { reg: CDC_WSA_BOOST1_BOOST_PATH_CTL, def: 0x00 },
    reg_default { reg: CDC_WSA_BOOST1_BOOST_CTL, def: 0xD0 }, reg_default { reg: CDC_WSA_BOOST1_BOOST_CFG1, def: 0x89 },
    reg_default { reg: CDC_WSA_BOOST1_BOOST_CFG2, def: 0x04 }, reg_default { reg: CDC_WSA_COMPANDER0_CTL0, def: 0x60 },
    reg_default { reg: CDC_WSA_COMPANDER0_CTL1, def: 0xDB }, reg_default { reg: CDC_WSA_COMPANDER0_CTL2, def: 0xFF },
    reg_default { reg: CDC_WSA_COMPANDER0_CTL3, def: 0x35 }, reg_default { reg: CDC_WSA_COMPANDER0_CTL4, def: 0xFF },
    reg_default { reg: CDC_WSA_COMPANDER0_CTL5, def: 0x00 }, reg_default { reg: CDC_WSA_COMPANDER0_CTL6, def: 0x01 },
    reg_default { reg: CDC_WSA_COMPANDER0_CTL7, def: 0x28 }, reg_default { reg: CDC_WSA_EC_HQ0_EC_REF_HQ_PATH_CTL, def: 0x00 },
    reg_default { reg: CDC_WSA_EC_HQ0_EC_REF_HQ_CFG0, def: 0x01 }, reg_default { reg: CDC_WSA_EC_HQ1_EC_REF_HQ_PATH_CTL, def: 0x00 },
    reg_default { reg: CDC_WSA_EC_HQ1_EC_REF_HQ_CFG0, def: 0x01 }, reg_default { reg: CDC_WSA_SPLINE_ASRC0_CLK_RST_CTL, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC0_CTL0, def: 0x00 }, reg_default { reg: CDC_WSA_SPLINE_ASRC0_CTL1, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC0_FIFO_CTL, def: 0xA8 }, reg_default { reg: CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_LSB, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_MSB, def: 0x00 }, reg_default { reg: CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_LSB, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_MSB, def: 0x00 }, reg_default { reg: CDC_WSA_SPLINE_ASRC0_STATUS_FIFO, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC1_CLK_RST_CTL, def: 0x00 }, reg_default { reg: CDC_WSA_SPLINE_ASRC1_CTL0, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC1_CTL1, def: 0x00 }, reg_default { reg: CDC_WSA_SPLINE_ASRC1_FIFO_CTL, def: 0xA8 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_LSB, def: 0x00 }, reg_default { reg: CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_MSB, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_LSB, def: 0x00 }, reg_default { reg: CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_MSB, def: 0x00 },
    reg_default { reg: CDC_WSA_SPLINE_ASRC1_STATUS_FIFO, def: 0x00 },
];

static wsa_defaults_v2_1: [reg_default; 20] = [
    reg_default { reg: CDC_WSA_TX0_SPKR_PROT_PATH_CTL, def: 0x02 }, reg_default { reg: CDC_WSA_TX0_SPKR_PROT_PATH_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_TX1_SPKR_PROT_PATH_CTL, def: 0x02 }, reg_default { reg: CDC_WSA_TX1_SPKR_PROT_PATH_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_TX2_SPKR_PROT_PATH_CTL, def: 0x02 }, reg_default { reg: CDC_WSA_TX2_SPKR_PROT_PATH_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_TX3_SPKR_PROT_PATH_CTL, def: 0x02 }, reg_default { reg: CDC_WSA_TX3_SPKR_PROT_PATH_CFG0, def: 0x00 },
    reg_default { reg: CDC_WSA_COMPANDER1_CTL0, def: 0x60 }, reg_default { reg: CDC_WSA_COMPANDER1_CTL1, def: 0xDB },
    reg_default { reg: CDC_WSA_COMPANDER1_CTL2, def: 0xFF }, reg_default { reg: CDC_WSA_COMPANDER1_CTL3, def: 0x35 },
    reg_default { reg: CDC_WSA_COMPANDER1_CTL4, def: 0xFF }, reg_default { reg: CDC_WSA_COMPANDER1_CTL5, def: 0x00 },
    reg_default { reg: CDC_WSA_COMPANDER1_CTL6, def: 0x01 }, reg_default { reg: CDC_WSA_COMPANDER1_CTL7, def: 0x28 },
    reg_default { reg: CDC_WSA_SOFTCLIP0_CRC, def: 0x00 }, reg_default { reg: CDC_WSA_SOFTCLIP0_SOFTCLIP_CTRL, def: 0x38 },
    reg_default { reg: CDC_WSA_SOFTCLIP1_CRC, def: 0x00 }, reg_default { reg: CDC_WSA_SOFTCLIP1_SOFTCLIP_CTRL, def: 0x38 },
];

static wsa_defaults_v2_5: [reg_default; 46] = [
    reg_default { reg: CDC_WSA_TOP_FS_UNGATE, def: 0xFF }, reg_default { reg: CDC_WSA_TOP_GRP_SEL, def: 0x08 },
    reg_default { reg: CDC_WSA_TOP_FS_UNGATE2, def: 0x1F }, reg_default { reg: CDC_WSA_TX0_SPKR_PROT_PATH_CTL, def: 0x04 },
    reg_default { reg: CDC_WSA_TX0_SPKR_PROT_PATH_CFG0, def: 0x02 }, reg_default { reg: CDC_WSA_TX1_SPKR_PROT_PATH_CTL, def: 0x04 },
    reg_default { reg: CDC_WSA_TX1_SPKR_PROT_PATH_CFG0, def: 0x02 }, reg_default { reg: CDC_WSA_TX2_SPKR_PROT_PATH_CTL, def: 0x04 },
    reg_default { reg: CDC_WSA_TX2_SPKR_PROT_PATH_CFG0, def: 0x02 }, reg_default { reg: CDC_WSA_TX3_SPKR_PROT_PATH_CTL, def: 0x04 },
    reg_default { reg: CDC_WSA_TX3_SPKR_PROT_PATH_CFG0, def: 0x02 }, reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL8, def: 0x00 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL9, def: 0x00 }, reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL10, def: 0x06 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL11, def: 0x12 }, reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL12, def: 0x1E },
    reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL13, def: 0x24 }, reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL14, def: 0x24 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL15, def: 0x24 }, reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL16, def: 0x00 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL17, def: 0x24 }, reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL18, def: 0x2A },
    reg_default { reg: CDC_2_5_WSA_COMPANDER0_CTL19, def: 0x16 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL0, def: 0x60 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL1, def: 0xDB }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL2, def: 0xFF },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL3, def: 0x35 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL4, def: 0xFF },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL5, def: 0x00 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL6, def: 0x01 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL7, def: 0x28 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL8, def: 0x00 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL9, def: 0x00 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL10, def: 0x06 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL11, def: 0x12 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL12, def: 0x1E },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL13, def: 0x24 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL14, def: 0x24 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL15, def: 0x24 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL16, def: 0x00 },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL17, def: 0x24 }, reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL18, def: 0x2A },
    reg_default { reg: CDC_2_5_WSA_COMPANDER1_CTL19, def: 0x16 }, reg_default { reg: CDC_2_5_WSA_SOFTCLIP0_CRC, def: 0x00 },
    reg_default { reg: CDC_2_5_WSA_SOFTCLIP0_SOFTCLIP_CTRL, def: 0x38 }, reg_default { reg: CDC_2_5_WSA_SOFTCLIP1_CRC, def: 0x00 },
    reg_default { reg: CDC_2_5_WSA_SOFTCLIP1_SOFTCLIP_CTRL, def: 0x38 },
];

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_read_field(component: *mut snd_soc_component, reg: c_uint, mask: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_int) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(comp: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_init_regmap(comp: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget_desc, num: c_uint) -> c_int;
    fn snd_soc_dapm_kcontrol_to_widget(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, mux: c_uint, e: *mut soc_enum, update: *mut snd_soc_dapm_update) -> c_int;
    fn snd_soc_dapm_mixer_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, enable: c_uint, update: *mut snd_soc_dapm_update) -> c_int;
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, name: *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn pm_clk_suspend(dev: *mut device) -> c_int;
    fn pm_clk_resume(dev: *mut device) -> c_int;
    fn devm_pm_clk_create(dev: *mut device) -> c_int;
    fn of_pm_clk_add_clks(dev: *mut device) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get_optional(dev: *mut device, name: *const c_char) -> *mut clk;
    fn devm_clk_get(dev: *mut device, name: *const c_char) -> *mut clk;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut c_void, config: *mut regmap_config) -> *mut regmap;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, drv: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, num: c_int) -> c_int;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const c_void, data: *mut clk_hw) -> c_int;
    fn dev_of_node(dev: *mut device) -> *mut c_void;
    fn of_property_read_string(np: *mut c_void, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn lpass_macro_get_codec_version() -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    static of_clk_hw_simple_get: c_void;
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool { (ptr as isize) < 0 && (ptr as isize) > -4096 }
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int { ptr as isize as c_int }
fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool { event == SND_SOC_DAPM_PRE_PMU || event == SND_SOC_DAPM_POST_PMU }
fn SND_SOC_DAPM_EVENT_OFF(event: c_int) -> bool { event == SND_SOC_DAPM_POST_PMD }
unsafe fn test_bit(bit: usize, addr: *const c_ulong) -> bool { ((*addr) & (1usize << bit) as c_ulong) != 0 }
unsafe fn set_bit(bit: usize, addr: *mut c_ulong) { *addr |= (1usize << bit) as c_ulong; }
unsafe fn clear_bit(bit: usize, addr: *mut c_ulong) { *addr &= !((1usize << bit) as c_ulong); }

unsafe extern "C" fn wsa_is_wronly_register(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg, CDC_WSA_INTR_CTRL_CLR_COMMIT | CDC_WSA_INTR_CTRL_PIN1_CLEAR0 | CDC_WSA_INTR_CTRL_PIN2_CLEAR0)
}

unsafe extern "C" fn wsa_is_rw_register_v2_1(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg,
        CDC_WSA_COMPANDER1_CTL0 | CDC_WSA_COMPANDER1_CTL1 | CDC_WSA_COMPANDER1_CTL2 |
        CDC_WSA_COMPANDER1_CTL3 | CDC_WSA_COMPANDER1_CTL4 | CDC_WSA_COMPANDER1_CTL5 |
        CDC_WSA_COMPANDER1_CTL7 | CDC_WSA_SOFTCLIP0_CRC | CDC_WSA_SOFTCLIP0_SOFTCLIP_CTRL |
        CDC_WSA_SOFTCLIP1_CRC | CDC_WSA_SOFTCLIP1_SOFTCLIP_CTRL)
}

unsafe extern "C" fn wsa_is_rw_register_v2_5(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg,
        CDC_WSA_TOP_FS_UNGATE | CDC_WSA_TOP_GRP_SEL | CDC_WSA_TOP_FS_UNGATE2 |
        CDC_2_5_WSA_COMPANDER0_CTL8 | CDC_2_5_WSA_COMPANDER0_CTL9 | CDC_2_5_WSA_COMPANDER0_CTL10 |
        CDC_2_5_WSA_COMPANDER0_CTL11 | CDC_2_5_WSA_COMPANDER0_CTL12 | CDC_2_5_WSA_COMPANDER0_CTL13 |
        CDC_2_5_WSA_COMPANDER0_CTL14 | CDC_2_5_WSA_COMPANDER0_CTL15 | CDC_2_5_WSA_COMPANDER0_CTL16 |
        CDC_2_5_WSA_COMPANDER0_CTL17 | CDC_2_5_WSA_COMPANDER0_CTL18 | CDC_2_5_WSA_COMPANDER0_CTL19 |
        CDC_2_5_WSA_COMPANDER1_CTL0 | CDC_2_5_WSA_COMPANDER1_CTL1 | CDC_2_5_WSA_COMPANDER1_CTL2 |
        CDC_2_5_WSA_COMPANDER1_CTL3 | CDC_2_5_WSA_COMPANDER1_CTL4 | CDC_2_5_WSA_COMPANDER1_CTL5 |
        CDC_2_5_WSA_COMPANDER1_CTL7 | CDC_2_5_WSA_COMPANDER1_CTL8 | CDC_2_5_WSA_COMPANDER1_CTL9 |
        CDC_2_5_WSA_COMPANDER1_CTL10 | CDC_2_5_WSA_COMPANDER1_CTL11 | CDC_2_5_WSA_COMPANDER1_CTL12 |
        CDC_2_5_WSA_COMPANDER1_CTL13 | CDC_2_5_WSA_COMPANDER1_CTL14 | CDC_2_5_WSA_COMPANDER1_CTL15 |
        CDC_2_5_WSA_COMPANDER1_CTL16 | CDC_2_5_WSA_COMPANDER1_CTL17 | CDC_2_5_WSA_COMPANDER1_CTL18 |
        CDC_2_5_WSA_COMPANDER1_CTL19 | CDC_2_5_WSA_SOFTCLIP0_CRC | CDC_2_5_WSA_SOFTCLIP0_SOFTCLIP_CTRL |
        CDC_2_5_WSA_SOFTCLIP1_CRC | CDC_2_5_WSA_SOFTCLIP1_SOFTCLIP_CTRL)
}

unsafe extern "C" fn wsa_is_rw_register(dev: *mut device, reg: c_uint) -> bool {
    let wsa = dev_get_drvdata(dev) as *mut wsa_macro;
    if matches!(reg,
        CDC_WSA_CLK_RST_CTRL_MCLK_CONTROL | CDC_WSA_CLK_RST_CTRL_FS_CNT_CONTROL | CDC_WSA_CLK_RST_CTRL_SWR_CONTROL |
        CDC_WSA_TOP_TOP_CFG0 | CDC_WSA_TOP_TOP_CFG1 | CDC_WSA_TOP_FREQ_MCLK | CDC_WSA_TOP_DEBUG_BUS_SEL |
        CDC_WSA_TOP_DEBUG_EN0 | CDC_WSA_TOP_DEBUG_EN1 | CDC_WSA_TOP_DEBUG_DSM_LB | CDC_WSA_TOP_RX_I2S_CTL |
        CDC_WSA_TOP_TX_I2S_CTL | CDC_WSA_TOP_I2S_CLK | CDC_WSA_TOP_I2S_RESET |
        CDC_WSA_RX_INP_MUX_RX_INT0_CFG0 | CDC_WSA_RX_INP_MUX_RX_INT0_CFG1 | CDC_WSA_RX_INP_MUX_RX_INT1_CFG0 |
        CDC_WSA_RX_INP_MUX_RX_INT1_CFG1 | CDC_WSA_RX_INP_MUX_RX_MIX_CFG0 | CDC_WSA_RX_INP_MUX_RX_EC_CFG0 |
        CDC_WSA_RX_INP_MUX_SOFTCLIP_CFG0 | CDC_WSA_TX0_SPKR_PROT_PATH_CTL | CDC_WSA_TX0_SPKR_PROT_PATH_CFG0 |
        CDC_WSA_TX1_SPKR_PROT_PATH_CTL | CDC_WSA_TX1_SPKR_PROT_PATH_CFG0 | CDC_WSA_TX2_SPKR_PROT_PATH_CTL |
        CDC_WSA_TX2_SPKR_PROT_PATH_CFG0 | CDC_WSA_TX3_SPKR_PROT_PATH_CTL | CDC_WSA_TX3_SPKR_PROT_PATH_CFG0 |
        CDC_WSA_INTR_CTRL_CFG | CDC_WSA_INTR_CTRL_PIN1_MASK0 | CDC_WSA_INTR_CTRL_PIN2_MASK0 | CDC_WSA_INTR_CTRL_LEVEL0 |
        CDC_WSA_INTR_CTRL_BYPASS0 | CDC_WSA_INTR_CTRL_SET0 | CDC_WSA_RX0_RX_PATH_CTL | CDC_WSA_RX0_RX_PATH_CFG0 |
        CDC_WSA_RX0_RX_PATH_CFG1 | CDC_WSA_RX0_RX_PATH_CFG2 | CDC_WSA_RX0_RX_PATH_CFG3 | CDC_WSA_RX0_RX_VOL_CTL |
        CDC_WSA_RX0_RX_PATH_MIX_CTL | CDC_WSA_RX0_RX_PATH_MIX_CFG | CDC_WSA_RX0_RX_VOL_MIX_CTL | CDC_WSA_RX0_RX_PATH_SEC0 |
        CDC_WSA_RX0_RX_PATH_SEC1 | CDC_WSA_RX0_RX_PATH_SEC2 | CDC_WSA_RX0_RX_PATH_SEC3 | CDC_WSA_RX0_RX_PATH_SEC5 |
        CDC_WSA_RX0_RX_PATH_SEC6 | CDC_WSA_RX0_RX_PATH_SEC7 | CDC_WSA_RX0_RX_PATH_MIX_SEC0 | CDC_WSA_RX0_RX_PATH_MIX_SEC1 |
        CDC_WSA_RX0_RX_PATH_DSMDEM_CTL | CDC_WSA_RX1_RX_PATH_CTL | CDC_WSA_RX1_RX_PATH_CFG0 | CDC_WSA_RX1_RX_PATH_CFG1 |
        CDC_WSA_RX1_RX_PATH_CFG2 | CDC_WSA_RX1_RX_PATH_CFG3 | CDC_WSA_RX1_RX_VOL_CTL | CDC_WSA_RX1_RX_PATH_MIX_CTL |
        CDC_WSA_RX1_RX_PATH_MIX_CFG | CDC_WSA_RX1_RX_VOL_MIX_CTL | CDC_WSA_RX1_RX_PATH_SEC0 | CDC_WSA_RX1_RX_PATH_SEC1 |
        CDC_WSA_RX1_RX_PATH_SEC2 | CDC_WSA_RX1_RX_PATH_SEC3 | CDC_WSA_RX1_RX_PATH_SEC5 | CDC_WSA_RX1_RX_PATH_SEC6 |
        CDC_WSA_RX1_RX_PATH_SEC7 | CDC_WSA_RX1_RX_PATH_MIX_SEC0 | CDC_WSA_RX1_RX_PATH_MIX_SEC1 | CDC_WSA_RX1_RX_PATH_DSMDEM_CTL |
        CDC_WSA_BOOST0_BOOST_PATH_CTL | CDC_WSA_BOOST0_BOOST_CTL | CDC_WSA_BOOST0_BOOST_CFG1 | CDC_WSA_BOOST0_BOOST_CFG2 |
        CDC_WSA_BOOST1_BOOST_PATH_CTL | CDC_WSA_BOOST1_BOOST_CTL | CDC_WSA_BOOST1_BOOST_CFG1 | CDC_WSA_BOOST1_BOOST_CFG2 |
        CDC_WSA_COMPANDER0_CTL0 | CDC_WSA_COMPANDER0_CTL1 | CDC_WSA_COMPANDER0_CTL2 | CDC_WSA_COMPANDER0_CTL3 |
        CDC_WSA_COMPANDER0_CTL4 | CDC_WSA_COMPANDER0_CTL5 | CDC_WSA_COMPANDER0_CTL7 | CDC_WSA_EC_HQ0_EC_REF_HQ_PATH_CTL |
        CDC_WSA_EC_HQ0_EC_REF_HQ_CFG0 | CDC_WSA_EC_HQ1_EC_REF_HQ_PATH_CTL | CDC_WSA_EC_HQ1_EC_REF_HQ_CFG0 |
        CDC_WSA_SPLINE_ASRC0_CLK_RST_CTL | CDC_WSA_SPLINE_ASRC0_CTL0 | CDC_WSA_SPLINE_ASRC0_CTL1 | CDC_WSA_SPLINE_ASRC0_FIFO_CTL |
        CDC_WSA_SPLINE_ASRC1_CLK_RST_CTL | CDC_WSA_SPLINE_ASRC1_CTL0 | CDC_WSA_SPLINE_ASRC1_CTL1 | CDC_WSA_SPLINE_ASRC1_FIFO_CTL) {
        return true;
    }
    if (*wsa).codec_version >= LPASS_CODEC_VERSION_2_5 { wsa_is_rw_register_v2_5(dev, reg) } else { wsa_is_rw_register_v2_1(dev, reg) }
}

unsafe extern "C" fn wsa_is_writeable_register(dev: *mut device, reg: c_uint) -> bool {
    let ret = wsa_is_rw_register(dev, reg);
    if !ret { return wsa_is_wronly_register(dev, reg); }
    ret
}

unsafe extern "C" fn wsa_is_readable_register_v2_1(dev: *mut device, reg: c_uint) -> bool {
    if reg == CDC_WSA_COMPANDER1_CTL6 { return true; }
    wsa_is_rw_register(dev, reg)
}

unsafe extern "C" fn wsa_is_readable_register_v2_5(dev: *mut device, reg: c_uint) -> bool {
    if reg == CDC_2_5_WSA_COMPANDER1_CTL6 { return true; }
    wsa_is_rw_register(dev, reg)
}

unsafe extern "C" fn wsa_is_readable_register(dev: *mut device, reg: c_uint) -> bool {
    let wsa = dev_get_drvdata(dev) as *mut wsa_macro;
    if matches!(reg,
        CDC_WSA_INTR_CTRL_CLR_COMMIT | CDC_WSA_INTR_CTRL_PIN1_CLEAR0 | CDC_WSA_INTR_CTRL_PIN2_CLEAR0 |
        CDC_WSA_INTR_CTRL_PIN1_STATUS0 | CDC_WSA_INTR_CTRL_PIN2_STATUS0 | CDC_WSA_COMPANDER0_CTL6 |
        CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_LSB | CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_MSB |
        CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_LSB | CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_MSB |
        CDC_WSA_SPLINE_ASRC0_STATUS_FIFO | CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_LSB |
        CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_MSB | CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_LSB |
        CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_MSB | CDC_WSA_SPLINE_ASRC1_STATUS_FIFO) {
        return true;
    }
    if (*wsa).codec_version >= LPASS_CODEC_VERSION_2_5 { wsa_is_readable_register_v2_5(dev, reg) } else { wsa_is_readable_register_v2_1(dev, reg) }
}

unsafe extern "C" fn wsa_is_volatile_register_v2_1(_dev: *mut device, reg: c_uint) -> bool { reg == CDC_WSA_COMPANDER1_CTL6 }
unsafe extern "C" fn wsa_is_volatile_register_v2_5(_dev: *mut device, reg: c_uint) -> bool { reg == CDC_2_5_WSA_COMPANDER1_CTL6 }

unsafe extern "C" fn wsa_is_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    let wsa = dev_get_drvdata(dev) as *mut wsa_macro;
    /* Update volatile list for rx/tx macros */
    if matches!(reg,
        CDC_WSA_INTR_CTRL_PIN1_STATUS0 | CDC_WSA_INTR_CTRL_PIN2_STATUS0 | CDC_WSA_COMPANDER0_CTL6 |
        CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_LSB | CDC_WSA_SPLINE_ASRC0_STATUS_FMIN_CNTR_MSB |
        CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_LSB | CDC_WSA_SPLINE_ASRC0_STATUS_FMAX_CNTR_MSB |
        CDC_WSA_SPLINE_ASRC0_STATUS_FIFO | CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_LSB |
        CDC_WSA_SPLINE_ASRC1_STATUS_FMIN_CNTR_MSB | CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_LSB |
        CDC_WSA_SPLINE_ASRC1_STATUS_FMAX_CNTR_MSB | CDC_WSA_SPLINE_ASRC1_STATUS_FIFO) {
        return true;
    }
    if (*wsa).codec_version >= LPASS_CODEC_VERSION_2_5 { wsa_is_volatile_register_v2_5(dev, reg) } else { wsa_is_volatile_register_v2_1(dev, reg) }
}

static wsa_regmap_config: regmap_config = regmap_config {
    name: b"wsa_macro\0".as_ptr() as *const c_char,
    reg_bits: 16,
    val_bits: 32, /* 8 but with 32 bit read/write */
    reg_stride: 4,
    cache_type: REGCACHE_FLAT,
    reg_defaults: ptr::null(),
    num_reg_defaults: 0,
    max_register: WSA_MAX_OFFSET,
    writeable_reg: Some(wsa_is_writeable_register),
    volatile_reg: Some(wsa_is_volatile_register),
    readable_reg: Some(wsa_is_readable_register),
};

/**
 * wsa_macro_set_spkr_mode - Configures speaker compander and smartboost
 * settings based on speaker mode.
 *
 * @component: codec instance
 * @mode: Indicates speaker configuration mode.
 *
 * Returns 0 on success or -EINVAL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn wsa_macro_set_spkr_mode(component: *mut snd_soc_component, mode: c_int) -> c_int {
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*wsa).spkr_mode = mode;
    match mode {
        WSA_MACRO_SPKR_MODE_1 => {
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER0_CTL3, 0x80, 0x00);
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER1_CTL3, 0x80, 0x00);
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER0_CTL7, 0x01, 0x00);
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER1_CTL7, 0x01, 0x00);
            snd_soc_component_update_bits(component, CDC_WSA_BOOST0_BOOST_CTL, 0x7C, 0x44);
            snd_soc_component_update_bits(component, CDC_WSA_BOOST1_BOOST_CTL, 0x7C, 0x44);
        }
        _ => {
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER0_CTL3, 0x80, 0x80);
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER1_CTL3, 0x80, 0x80);
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER0_CTL7, 0x01, 0x01);
            snd_soc_component_update_bits(component, CDC_WSA_COMPANDER1_CTL7, 0x01, 0x01);
            snd_soc_component_update_bits(component, CDC_WSA_BOOST0_BOOST_CTL, 0x7C, 0x58);
            snd_soc_component_update_bits(component, CDC_WSA_BOOST1_BOOST_CTL, 0x7C, 0x58);
        }
    }
    0
}
/* EXPORT_SYMBOL(wsa_macro_set_spkr_mode); */

unsafe fn wsa_macro_set_prim_interpolator_rate(dai: *mut snd_soc_dai, int_prim_fs_rate_reg_val: u8, _sample_rate: u32) -> c_int {
    let component = (*dai).component;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let mut port = 0usize;
    while port < WSA_MACRO_RX_MAX {
        if test_bit(port, &(*wsa).active_ch_mask[(*dai).id as usize]) {
            let int_1_mix1_inp = port as u8;
            let mut int_mux_cfg0 = CDC_WSA_RX_INP_MUX_RX_INT0_CFG0;
            let mut j = 0usize;
            while j < NUM_INTERPOLATORS {
                let int_mux_cfg1 = int_mux_cfg0 + WSA_MACRO_MUX_CFG1_OFFSET;
                let inp0_sel = snd_soc_component_read_field(component, int_mux_cfg0, (*(*wsa).reg_layout).rx_intx_1_mix_inp0_sel_mask) as u8;
                let inp1_sel = snd_soc_component_read_field(component, int_mux_cfg0, (*(*wsa).reg_layout).rx_intx_1_mix_inp1_sel_mask) as u8;
                let inp2_sel = snd_soc_component_read_field(component, int_mux_cfg1, (*(*wsa).reg_layout).rx_intx_1_mix_inp2_sel_mask) as u8;
                if inp0_sel == int_1_mix1_inp + INTn_1_INP_SEL_RX0 ||
                   inp1_sel == int_1_mix1_inp + INTn_1_INP_SEL_RX0 ||
                   inp2_sel == int_1_mix1_inp + INTn_1_INP_SEL_RX0 {
                    let int_fs_reg = CDC_WSA_RX0_RX_PATH_CTL + WSA_MACRO_RX_PATH_OFFSET * j as u32;
                    /* sample_rate is in Hz */
                    snd_soc_component_update_bits(component, int_fs_reg, WSA_MACRO_FS_RATE_MASK, int_prim_fs_rate_reg_val as u32);
                }
                int_mux_cfg0 += WSA_MACRO_MUX_CFG_OFFSET;
                j += 1;
            }
        }
        port += 1;
    }
    0
}

unsafe fn wsa_macro_set_mix_interpolator_rate(dai: *mut snd_soc_dai, int_mix_fs_rate_reg_val: u8, _sample_rate: u32) -> c_int {
    let component = (*dai).component;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let mut port = 0usize;
    while port < WSA_MACRO_RX_MAX {
        if test_bit(port, &(*wsa).active_ch_mask[(*dai).id as usize]) {
            let int_2_inp = port as u8;
            let mut int_mux_cfg1 = CDC_WSA_RX_INP_MUX_RX_INT0_CFG1;
            let mut j = 0usize;
            while j < NUM_INTERPOLATORS {
                let int_mux_cfg1_val = snd_soc_component_read_field(component, int_mux_cfg1, (*(*wsa).reg_layout).rx_intx_2_sel_mask) as u8;
                if int_mux_cfg1_val == int_2_inp + INTn_2_INP_SEL_RX0 {
                    let int_fs_reg = CDC_WSA_RX0_RX_PATH_MIX_CTL + WSA_MACRO_RX_PATH_OFFSET * j as u32;
                    snd_soc_component_update_bits(component, int_fs_reg, WSA_MACRO_FS_RATE_MASK, int_mix_fs_rate_reg_val as u32);
                }
                int_mux_cfg1 += WSA_MACRO_MUX_CFG_OFFSET;
                j += 1;
            }
        }
        port += 1;
    }
    0
}

unsafe fn wsa_macro_set_interpolator_rate(dai: *mut snd_soc_dai, sample_rate: u32) -> c_int {
    let mut rate_val = 0;
    let mut i = 0usize;
    while i < int_mix_sample_rate_val.len() {
        if sample_rate as c_int == int_mix_sample_rate_val[i].sample_rate {
            rate_val = int_mix_sample_rate_val[i].rate_val;
            break;
        }
        i += 1;
    }
    if i != int_mix_sample_rate_val.len() && rate_val >= 0 {
        let ret = wsa_macro_set_mix_interpolator_rate(dai, rate_val as u8, sample_rate);
        if ret < 0 { return ret; }
    }
    i = 0;
    while i < int_prim_sample_rate_val.len() {
        if sample_rate as c_int == int_prim_sample_rate_val[i].sample_rate {
            rate_val = int_prim_sample_rate_val[i].rate_val;
            break;
        }
        i += 1;
    }
    if i == int_prim_sample_rate_val.len() || rate_val < 0 { return -EINVAL; }
    wsa_macro_set_prim_interpolator_rate(dai, rate_val as u8, sample_rate)
}

unsafe extern "C" fn wsa_macro_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    match (*substream).stream {
        SNDRV_PCM_STREAM_PLAYBACK => {
            let ret = wsa_macro_set_interpolator_rate(dai, params_rate(params));
            if ret != 0 {
                dev_err((*component).dev, b"%s: cannot set sample rate: %u\n\0".as_ptr() as *const c_char, b"wsa_macro_hw_params\0".as_ptr(), params_rate(params));
                return ret;
            }
        }
        SNDRV_PCM_STREAM_CAPTURE => {
            if (*dai).id as usize == WSA_MACRO_AIF_VI {
                (*wsa).pcm_rate_vi = params_rate(params);
            }
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn wsa_macro_get_channel_map(dai: *const snd_soc_dai, tx_num: *mut c_uint, tx_slot: *mut c_uint, rx_num: *mut c_uint, rx_slot: *mut c_uint) -> c_int {
    let component = (*dai).component;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let mut val: u16;
    let mut mask: u16 = 0;
    let mut cnt: u16 = 0;
    match (*dai).id as usize {
        WSA_MACRO_AIF_VI => {
            *tx_slot = (*wsa).active_ch_mask[(*dai).id as usize] as c_uint;
            *tx_num = (*wsa).active_ch_cnt[(*dai).id as usize] as c_uint;
        }
        WSA_MACRO_AIF1_PB | WSA_MACRO_AIF_MIX1_PB => {
            let mut temp = 0usize;
            while temp < WSA_MACRO_RX_MAX {
                if test_bit(temp, &(*wsa).active_ch_mask[(*dai).id as usize]) {
                    mask |= 1 << temp;
                    cnt += 1;
                    if cnt == WSA_MACRO_MAX_DMA_CH_PER_PORT { break; }
                }
                temp += 1;
            }
            if (mask & 0x0C) != 0 { mask >>= 0x2; }
            *rx_slot = mask as c_uint;
            *rx_num = cnt as c_uint;
        }
        WSA_MACRO_AIF_ECHO => {
            val = snd_soc_component_read(component, CDC_WSA_RX_INP_MUX_RX_MIX_CFG0) as u16;
            if (val as u32 & WSA_MACRO_EC_MIX_TX1_MASK) != 0 { mask |= 0x2; cnt += 1; }
            if (val as u32 & WSA_MACRO_EC_MIX_TX0_MASK) != 0 { mask |= 0x1; cnt += 1; }
            *tx_slot = mask as c_uint;
            *tx_num = cnt as c_uint;
        }
        _ => dev_err((*component).dev, b"%s: Invalid AIF\n\0".as_ptr() as *const c_char, b"wsa_macro_get_channel_map\0".as_ptr()),
    }
    0
}

static wsa_macro_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { hw_params: Some(wsa_macro_hw_params), get_channel_map: Some(wsa_macro_get_channel_map) };

static mut wsa_macro_dai: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver { name: b"wsa_macro_rx1\0".as_ptr() as *const c_char, id: WSA_MACRO_AIF1_PB as c_int, playback: snd_soc_pcm_stream { stream_name: b"WSA_AIF1 Playback\0".as_ptr() as *const c_char, rates: WSA_MACRO_RX_RATES, formats: WSA_MACRO_RX_FORMATS, rate_max: 384000, rate_min: 8000, channels_min: 1, channels_max: 2 }, capture: snd_soc_pcm_stream { stream_name: ptr::null(), rates: 0, formats: 0, rate_max: 0, rate_min: 0, channels_min: 0, channels_max: 0 }, ops: &wsa_macro_dai_ops },
    snd_soc_dai_driver { name: b"wsa_macro_rx_mix\0".as_ptr() as *const c_char, id: WSA_MACRO_AIF_MIX1_PB as c_int, playback: snd_soc_pcm_stream { stream_name: b"WSA_AIF_MIX1 Playback\0".as_ptr() as *const c_char, rates: WSA_MACRO_RX_MIX_RATES, formats: WSA_MACRO_RX_FORMATS, rate_max: 192000, rate_min: 48000, channels_min: 1, channels_max: 2 }, capture: snd_soc_pcm_stream { stream_name: ptr::null(), rates: 0, formats: 0, rate_max: 0, rate_min: 0, channels_min: 0, channels_max: 0 }, ops: &wsa_macro_dai_ops },
    snd_soc_dai_driver { name: b"wsa_macro_vifeedback\0".as_ptr() as *const c_char, id: WSA_MACRO_AIF_VI as c_int, playback: snd_soc_pcm_stream { stream_name: ptr::null(), rates: 0, formats: 0, rate_max: 0, rate_min: 0, channels_min: 0, channels_max: 0 }, capture: snd_soc_pcm_stream { stream_name: b"WSA_AIF_VI Capture\0".as_ptr() as *const c_char, rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_48000, formats: WSA_MACRO_RX_FORMATS, rate_max: 48000, rate_min: 8000, channels_min: 1, channels_max: 4 }, ops: &wsa_macro_dai_ops },
    snd_soc_dai_driver { name: b"wsa_macro_echo\0".as_ptr() as *const c_char, id: WSA_MACRO_AIF_ECHO as c_int, playback: snd_soc_pcm_stream { stream_name: ptr::null(), rates: 0, formats: 0, rate_max: 0, rate_min: 0, channels_min: 0, channels_max: 0 }, capture: snd_soc_pcm_stream { stream_name: b"WSA_AIF_ECHO Capture\0".as_ptr() as *const c_char, rates: WSA_MACRO_ECHO_RATES, formats: WSA_MACRO_ECHO_FORMATS, rate_max: 48000, rate_min: 8000, channels_min: 1, channels_max: 2 }, ops: &wsa_macro_dai_ops },
];

unsafe fn wsa_macro_mclk_enable(wsa: *mut wsa_macro, mclk_enable: bool) {
    let regmap = (*wsa).regmap;
    if mclk_enable {
        if (*wsa).wsa_mclk_users == 0 {
            regcache_mark_dirty(regmap);
            regcache_sync(regmap);
            /* 9.6MHz MCLK, set value 0x00 if other frequency */
            regmap_update_bits(regmap, CDC_WSA_TOP_FREQ_MCLK, 0x01, 0x01);
            regmap_update_bits(regmap, CDC_WSA_CLK_RST_CTRL_MCLK_CONTROL, CDC_WSA_MCLK_EN_MASK, CDC_WSA_MCLK_ENABLE);
            regmap_update_bits(regmap, CDC_WSA_CLK_RST_CTRL_FS_CNT_CONTROL, CDC_WSA_FS_CNT_EN_MASK, CDC_WSA_FS_CNT_ENABLE);
        }
        (*wsa).wsa_mclk_users = (*wsa).wsa_mclk_users.wrapping_add(1);
    } else {
        if (*wsa).wsa_mclk_users == 0 {
            dev_err((*wsa).dev, b"clock already disabled\n\0".as_ptr() as *const c_char);
            (*wsa).wsa_mclk_users = 0;
            return;
        }
        (*wsa).wsa_mclk_users -= 1;
        if (*wsa).wsa_mclk_users == 0 {
            regmap_update_bits(regmap, CDC_WSA_CLK_RST_CTRL_FS_CNT_CONTROL, CDC_WSA_FS_CNT_EN_MASK, CDC_WSA_FS_CNT_DISABLE);
            regmap_update_bits(regmap, CDC_WSA_CLK_RST_CTRL_MCLK_CONTROL, CDC_WSA_MCLK_EN_MASK, CDC_WSA_MCLK_DISABLE);
        }
    }
}

unsafe fn wsa_macro_enable_disable_vi_sense(component: *mut snd_soc_component, enable: bool, tx_reg0: u32, tx_reg1: u32, val: u32) {
    if enable {
        /* Enable V&I sensing */
        snd_soc_component_update_bits(component, tx_reg0, CDC_WSA_TX_SPKR_PROT_RESET_MASK, CDC_WSA_TX_SPKR_PROT_RESET);
        snd_soc_component_update_bits(component, tx_reg1, CDC_WSA_TX_SPKR_PROT_RESET_MASK, CDC_WSA_TX_SPKR_PROT_RESET);
        snd_soc_component_update_bits(component, tx_reg0, CDC_WSA_TX_SPKR_PROT_PCM_RATE_MASK, val);
        snd_soc_component_update_bits(component, tx_reg1, CDC_WSA_TX_SPKR_PROT_PCM_RATE_MASK, val);
        snd_soc_component_update_bits(component, tx_reg0, CDC_WSA_TX_SPKR_PROT_CLK_EN_MASK, CDC_WSA_TX_SPKR_PROT_CLK_ENABLE);
        snd_soc_component_update_bits(component, tx_reg1, CDC_WSA_TX_SPKR_PROT_CLK_EN_MASK, CDC_WSA_TX_SPKR_PROT_CLK_ENABLE);
        snd_soc_component_update_bits(component, tx_reg0, CDC_WSA_TX_SPKR_PROT_RESET_MASK, CDC_WSA_TX_SPKR_PROT_NO_RESET);
        snd_soc_component_update_bits(component, tx_reg1, CDC_WSA_TX_SPKR_PROT_RESET_MASK, CDC_WSA_TX_SPKR_PROT_NO_RESET);
    } else {
        snd_soc_component_update_bits(component, tx_reg0, CDC_WSA_TX_SPKR_PROT_RESET_MASK, CDC_WSA_TX_SPKR_PROT_RESET);
        snd_soc_component_update_bits(component, tx_reg1, CDC_WSA_TX_SPKR_PROT_RESET_MASK, CDC_WSA_TX_SPKR_PROT_RESET);
        snd_soc_component_update_bits(component, tx_reg0, CDC_WSA_TX_SPKR_PROT_CLK_EN_MASK, CDC_WSA_TX_SPKR_PROT_CLK_DISABLE);
        snd_soc_component_update_bits(component, tx_reg1, CDC_WSA_TX_SPKR_PROT_CLK_EN_MASK, CDC_WSA_TX_SPKR_PROT_CLK_DISABLE);
    }
}

unsafe fn wsa_macro_enable_disable_vi_feedback(component: *mut snd_soc_component, enable: bool, rate: u32) {
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    if test_bit(WSA_MACRO_TX0, &(*wsa).active_ch_mask[WSA_MACRO_AIF_VI]) {
        wsa_macro_enable_disable_vi_sense(component, enable, CDC_WSA_TX0_SPKR_PROT_PATH_CTL, CDC_WSA_TX1_SPKR_PROT_PATH_CTL, rate);
    }
    if test_bit(WSA_MACRO_TX1, &(*wsa).active_ch_mask[WSA_MACRO_AIF_VI]) {
        wsa_macro_enable_disable_vi_sense(component, enable, CDC_WSA_TX2_SPKR_PROT_PATH_CTL, CDC_WSA_TX3_SPKR_PROT_PATH_CTL, rate);
    }
}

unsafe extern "C" fn wsa_macro_mclk_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    wsa_macro_mclk_enable(wsa, event == SND_SOC_DAPM_PRE_PMU);
    0
}

unsafe extern "C" fn wsa_macro_enable_vi_feedback(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let rate_val = match (*wsa).pcm_rate_vi {
        8000 => CDC_WSA_TX_SPKR_PROT_PCM_RATE_8K,
        16000 => CDC_WSA_TX_SPKR_PROT_PCM_RATE_16K,
        24000 => CDC_WSA_TX_SPKR_PROT_PCM_RATE_24K,
        32000 => CDC_WSA_TX_SPKR_PROT_PCM_RATE_32K,
        48000 => CDC_WSA_TX_SPKR_PROT_PCM_RATE_48K,
        _ => CDC_WSA_TX_SPKR_PROT_PCM_RATE_8K,
    };
    match event {
        SND_SOC_DAPM_POST_PMU => wsa_macro_enable_disable_vi_feedback(component, true, rate_val),
        SND_SOC_DAPM_POST_PMD => wsa_macro_enable_disable_vi_feedback(component, false, rate_val),
        _ => {}
    }
    0
}

unsafe fn wsa_macro_hd2_control(component: *mut snd_soc_component, reg: u16, event: c_int) {
    let mut hd2_scale_reg: u16 = 0;
    let mut hd2_enable_reg: u16 = 0;
    if reg as u32 == CDC_WSA_RX0_RX_PATH_CTL {
        hd2_scale_reg = CDC_WSA_RX0_RX_PATH_SEC3 as u16;
        hd2_enable_reg = CDC_WSA_RX0_RX_PATH_CFG0 as u16;
    }
    if reg as u32 == CDC_WSA_RX1_RX_PATH_CTL {
        hd2_scale_reg = CDC_WSA_RX1_RX_PATH_SEC3 as u16;
        hd2_enable_reg = CDC_WSA_RX1_RX_PATH_CFG0 as u16;
    }
    if hd2_enable_reg != 0 && SND_SOC_DAPM_EVENT_ON(event) {
        snd_soc_component_update_bits(component, hd2_scale_reg as u32, CDC_WSA_RX_PATH_HD2_ALPHA_MASK, 0x10);
        snd_soc_component_update_bits(component, hd2_scale_reg as u32, CDC_WSA_RX_PATH_HD2_SCALE_MASK, 0x1);
        snd_soc_component_update_bits(component, hd2_enable_reg as u32, CDC_WSA_RX_PATH_HD2_EN_MASK, CDC_WSA_RX_PATH_HD2_ENABLE);
    }
    if hd2_enable_reg != 0 && SND_SOC_DAPM_EVENT_OFF(event) {
        snd_soc_component_update_bits(component, hd2_enable_reg as u32, CDC_WSA_RX_PATH_HD2_EN_MASK, 0);
        snd_soc_component_update_bits(component, hd2_scale_reg as u32, CDC_WSA_RX_PATH_HD2_SCALE_MASK, 0);
        snd_soc_component_update_bits(component, hd2_scale_reg as u32, CDC_WSA_RX_PATH_HD2_ALPHA_MASK, 0);
    }
}

unsafe fn wsa_macro_config_compander(component: *mut snd_soc_component, comp: c_int, event: c_int) -> c_int {
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    if (*wsa).comp_enabled[comp as usize] == 0 { return 0; }
    let comp_ctl0_reg = CDC_WSA_COMPANDER0_CTL0 + comp as u32 * (*(*wsa).reg_layout).compander1_reg_offset;
    let rx_path_cfg0_reg = CDC_WSA_RX0_RX_PATH_CFG0 + comp as u32 * WSA_MACRO_RX_PATH_OFFSET;
    if SND_SOC_DAPM_EVENT_ON(event) {
        /* Enable Compander Clock */
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_CLK_EN_MASK, CDC_WSA_COMPANDER_CLK_ENABLE);
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_SOFT_RST_MASK, CDC_WSA_COMPANDER_SOFT_RST_ENABLE);
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_SOFT_RST_MASK, 0);
        snd_soc_component_update_bits(component, rx_path_cfg0_reg, CDC_WSA_RX_PATH_COMP_EN_MASK, CDC_WSA_RX_PATH_COMP_ENABLE);
    }
    if SND_SOC_DAPM_EVENT_OFF(event) {
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_HALT_MASK, CDC_WSA_COMPANDER_HALT);
        snd_soc_component_update_bits(component, rx_path_cfg0_reg, CDC_WSA_RX_PATH_COMP_EN_MASK, 0);
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_SOFT_RST_MASK, CDC_WSA_COMPANDER_SOFT_RST_ENABLE);
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_SOFT_RST_MASK, 0);
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_CLK_EN_MASK, 0);
        snd_soc_component_update_bits(component, comp_ctl0_reg, CDC_WSA_COMPANDER_HALT_MASK, 0);
    }
    0
}

unsafe fn wsa_macro_enable_softclip_clk(component: *mut snd_soc_component, wsa: *mut wsa_macro, path: c_int, enable: bool) {
    let softclip_clk_reg = (*(*wsa).reg_layout).softclip0_reg_base + path as u32 * (*(*wsa).reg_layout).softclip1_reg_offset;
    let softclip_mux_mask = 1u32 << path;
    let softclip_mux_value = 1u32 << path;
    if enable {
        if (*wsa).softclip_clk_users[path as usize] == 0 {
            snd_soc_component_update_bits(component, softclip_clk_reg, CDC_WSA_SOFTCLIP_CLK_EN_MASK, CDC_WSA_SOFTCLIP_CLK_ENABLE);
            snd_soc_component_update_bits(component, CDC_WSA_RX_INP_MUX_SOFTCLIP_CFG0, softclip_mux_mask, softclip_mux_value);
        }
        (*wsa).softclip_clk_users[path as usize] += 1;
    } else {
        (*wsa).softclip_clk_users[path as usize] -= 1;
        if (*wsa).softclip_clk_users[path as usize] == 0 {
            snd_soc_component_update_bits(component, softclip_clk_reg, CDC_WSA_SOFTCLIP_CLK_EN_MASK, 0);
            snd_soc_component_update_bits(component, CDC_WSA_RX_INP_MUX_SOFTCLIP_CFG0, softclip_mux_mask, 0x00);
        }
    }
}

unsafe fn wsa_macro_config_softclip(component: *mut snd_soc_component, path: c_int, event: c_int) -> c_int {
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let mut softclip_path = 0;
    if path == WSA_MACRO_COMP1 as c_int { softclip_path = WSA_MACRO_SOFTCLIP0 as c_int; }
    else if path == WSA_MACRO_COMP2 as c_int { softclip_path = WSA_MACRO_SOFTCLIP1 as c_int; }
    if (*wsa).is_softclip_on[softclip_path as usize] == 0 { return 0; }
    let softclip_ctrl_reg = CDC_WSA_SOFTCLIP0_SOFTCLIP_CTRL + softclip_path as u32 * (*(*wsa).reg_layout).softclip1_reg_offset;
    if SND_SOC_DAPM_EVENT_ON(event) {
        /* Enable Softclip clock and mux */
        wsa_macro_enable_softclip_clk(component, wsa, softclip_path, true);
        /* Enable Softclip control */
        snd_soc_component_update_bits(component, softclip_ctrl_reg, CDC_WSA_SOFTCLIP_EN_MASK, CDC_WSA_SOFTCLIP_ENABLE);
    }
    if SND_SOC_DAPM_EVENT_OFF(event) {
        snd_soc_component_update_bits(component, softclip_ctrl_reg, CDC_WSA_SOFTCLIP_EN_MASK, 0);
        wsa_macro_enable_softclip_clk(component, wsa, softclip_path, false);
    }
    0
}

unsafe fn wsa_macro_interp_get_primary_reg(reg: u16, ind: *mut u16) -> c_int {
    let mut prim_int_reg: u16 = 0;
    match reg as u32 {
        CDC_WSA_RX0_RX_PATH_CTL | CDC_WSA_RX0_RX_PATH_MIX_CTL => { prim_int_reg = CDC_WSA_RX0_RX_PATH_CTL as u16; *ind = 0; }
        CDC_WSA_RX1_RX_PATH_CTL | CDC_WSA_RX1_RX_PATH_MIX_CTL => { prim_int_reg = CDC_WSA_RX1_RX_PATH_CTL as u16; *ind = 1; }
        _ => {}
    }
    prim_int_reg as c_int
}

unsafe fn wsa_macro_enable_prim_interpolator(component: *mut snd_soc_component, reg: u16, event: c_int) -> c_int {
    let mut ind: u16 = 0;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let prim_int_reg = wsa_macro_interp_get_primary_reg(reg, &mut ind) as u16;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            (*wsa).prim_int_users[ind as usize] += 1;
            if (*wsa).prim_int_users[ind as usize] == 1 {
                snd_soc_component_update_bits(component, prim_int_reg as u32 + WSA_MACRO_RX_PATH_CFG3_OFFSET, CDC_WSA_RX_DC_DCOEFF_MASK, 0x3);
                snd_soc_component_update_bits(component, prim_int_reg as u32, CDC_WSA_RX_PATH_PGA_MUTE_EN_MASK, CDC_WSA_RX_PATH_PGA_MUTE_ENABLE);
                wsa_macro_hd2_control(component, prim_int_reg, event);
                snd_soc_component_update_bits(component, prim_int_reg as u32 + WSA_MACRO_RX_PATH_DSMDEM_OFFSET, CDC_WSA_RX_DSMDEM_CLK_EN_MASK, CDC_WSA_RX_DSMDEM_CLK_ENABLE);
            }
            if reg != prim_int_reg && (snd_soc_component_read(component, prim_int_reg as u32) & 0x10) != 0 {
                snd_soc_component_update_bits(component, reg as u32, 0x10, 0x10);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            (*wsa).prim_int_users[ind as usize] -= 1;
            if (*wsa).prim_int_users[ind as usize] == 0 {
                snd_soc_component_update_bits(component, prim_int_reg as u32 + WSA_MACRO_RX_PATH_DSMDEM_OFFSET, CDC_WSA_RX_DSMDEM_CLK_EN_MASK, 0);
                wsa_macro_hd2_control(component, prim_int_reg, event);
            }
        }
        _ => {}
    }
    0
}

unsafe fn wsa_macro_config_ear_spkr_gain(component: *mut snd_soc_component, wsa: *mut wsa_macro, event: c_int, gain_reg: c_int) -> c_int {
    let comp_gain_offset = match (*wsa).spkr_mode {
        /* Compander gain in WSA_MACRO_SPKR_MODE1 case is 12 dB */
        WSA_MACRO_SPKR_MODE_1 => -12,
        /* Default case compander gain is 15 dB */
        _ => -15,
    };
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* Apply ear spkr gain only if compander is enabled */
            if (*wsa).comp_enabled[WSA_MACRO_COMP1] != 0 && gain_reg as u32 == CDC_WSA_RX0_RX_VOL_CTL && (*wsa).ear_spkr_gain != 0 {
                /* For example, val is -8(-12+5-1) for 4dB of gain */
                let val = comp_gain_offset + (*wsa).ear_spkr_gain - 1;
                snd_soc_component_write(component, gain_reg as u32, val);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            /*
             * Reset RX0 volume to 0 dB if compander is enabled and
             * ear_spkr_gain is non-zero.
             */
            if (*wsa).comp_enabled[WSA_MACRO_COMP1] != 0 && gain_reg as u32 == CDC_WSA_RX0_RX_VOL_CTL && (*wsa).ear_spkr_gain != 0 {
                snd_soc_component_write(component, gain_reg as u32, 0x0);
            }
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn wsa_macro_enable_interpolator(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut gain_reg: u16 = 0;
    let mut reg: u16 = 0;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    if (*w).shift as usize == WSA_MACRO_COMP1 { reg = CDC_WSA_RX0_RX_PATH_CTL as u16; gain_reg = CDC_WSA_RX0_RX_VOL_CTL as u16; }
    else if (*w).shift as usize == WSA_MACRO_COMP2 { reg = CDC_WSA_RX1_RX_PATH_CTL as u16; gain_reg = CDC_WSA_RX1_RX_VOL_CTL as u16; }
    match event {
        SND_SOC_DAPM_PRE_PMU => wsa_macro_enable_prim_interpolator(component, reg, event),
        SND_SOC_DAPM_POST_PMU => {
            wsa_macro_config_compander(component, (*w).shift as c_int, event);
            wsa_macro_config_softclip(component, (*w).shift as c_int, event);
            if (*wsa).spkr_gain_offset == WSA_MACRO_GAIN_OFFSET_M1P5_DB as c_int && ((*wsa).comp_enabled[WSA_MACRO_COMP1] != 0 || (*wsa).comp_enabled[WSA_MACRO_COMP2] != 0) {
                for r in [CDC_WSA_RX0_RX_PATH_SEC1, CDC_WSA_RX0_RX_PATH_MIX_SEC0, CDC_WSA_RX1_RX_PATH_SEC1, CDC_WSA_RX1_RX_PATH_MIX_SEC0] {
                    snd_soc_component_update_bits(component, r, CDC_WSA_RX_PGA_HALF_DB_MASK, CDC_WSA_RX_PGA_HALF_DB_ENABLE);
                }
            }
            wsa_macro_config_ear_spkr_gain(component, wsa, event, gain_reg as c_int)
        }
        SND_SOC_DAPM_POST_PMD => {
            wsa_macro_config_compander(component, (*w).shift as c_int, event);
            wsa_macro_config_softclip(component, (*w).shift as c_int, event);
            wsa_macro_enable_prim_interpolator(component, reg, event);
            if (*wsa).spkr_gain_offset == WSA_MACRO_GAIN_OFFSET_M1P5_DB as c_int && ((*wsa).comp_enabled[WSA_MACRO_COMP1] != 0 || (*wsa).comp_enabled[WSA_MACRO_COMP2] != 0) {
                for r in [CDC_WSA_RX0_RX_PATH_SEC1, CDC_WSA_RX0_RX_PATH_MIX_SEC0, CDC_WSA_RX1_RX_PATH_SEC1, CDC_WSA_RX1_RX_PATH_MIX_SEC0] {
                    snd_soc_component_update_bits(component, r, CDC_WSA_RX_PGA_HALF_DB_MASK, CDC_WSA_RX_PGA_HALF_DB_DISABLE);
                }
            }
            wsa_macro_config_ear_spkr_gain(component, wsa, event, gain_reg as c_int)
        }
        _ => 0,
    };
    0
}

unsafe extern "C" fn wsa_macro_spk_boost_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut boost_path_ctl: u16 = 0;
    let mut boost_path_cfg1: u16 = 0;
    let mut reg: u16 = 0;
    let mut reg_mix: u16 = 0;
    if snd_soc_dapm_widget_name_cmp(w, b"WSA_RX INT0 CHAIN\0".as_ptr() as *const c_char) == 0 {
        boost_path_ctl = CDC_WSA_BOOST0_BOOST_PATH_CTL as u16;
        boost_path_cfg1 = CDC_WSA_RX0_RX_PATH_CFG1 as u16;
        reg = CDC_WSA_RX0_RX_PATH_CTL as u16;
        reg_mix = CDC_WSA_RX0_RX_PATH_MIX_CTL as u16;
    } else if snd_soc_dapm_widget_name_cmp(w, b"WSA_RX INT1 CHAIN\0".as_ptr() as *const c_char) == 0 {
        boost_path_ctl = CDC_WSA_BOOST1_BOOST_PATH_CTL as u16;
        boost_path_cfg1 = CDC_WSA_RX1_RX_PATH_CFG1 as u16;
        reg = CDC_WSA_RX1_RX_PATH_CTL as u16;
        reg_mix = CDC_WSA_RX1_RX_PATH_MIX_CTL as u16;
    } else {
        dev_warn((*component).dev, b"Incorrect widget name in the driver\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, boost_path_cfg1 as u32, CDC_WSA_RX_PATH_SMART_BST_EN_MASK, CDC_WSA_RX_PATH_SMART_BST_ENABLE);
            snd_soc_component_update_bits(component, boost_path_ctl as u32, CDC_WSA_BOOST_PATH_CLK_EN_MASK, CDC_WSA_BOOST_PATH_CLK_ENABLE);
            if (snd_soc_component_read(component, reg_mix as u32) & 0x10) != 0 {
                snd_soc_component_update_bits(component, reg_mix as u32, 0x10, 0x00);
            }
        }
        SND_SOC_DAPM_POST_PMU => { snd_soc_component_update_bits(component, reg as u32, 0x10, 0x00); }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, boost_path_ctl as u32, CDC_WSA_BOOST_PATH_CLK_EN_MASK, CDC_WSA_BOOST_PATH_CLK_DISABLE);
            snd_soc_component_update_bits(component, boost_path_cfg1 as u32, CDC_WSA_RX_PATH_SMART_BST_EN_MASK, CDC_WSA_RX_PATH_SMART_BST_DISABLE);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn wsa_macro_enable_echo(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let mut val = snd_soc_component_read(component, CDC_WSA_RX_INP_MUX_RX_MIX_CFG0) as u16;
    let ec_tx: u16;
    match (*w).shift as usize {
        WSA_MACRO_EC0_MUX => { val &= CDC_WSA_RX_MIX_TX0_SEL_MASK as u16; ec_tx = val.wrapping_sub(1); }
        WSA_MACRO_EC1_MUX => { val &= CDC_WSA_RX_MIX_TX1_SEL_MASK as u16; ec_tx = (val >> CDC_WSA_RX_MIX_TX1_SEL_SHFT).wrapping_sub(1); }
        _ => {
            dev_err((*component).dev, b"%s: Invalid shift %u\n\0".as_ptr() as *const c_char, b"wsa_macro_enable_echo\0".as_ptr(), (*w).shift);
            return -EINVAL;
        }
    }
    if (*wsa).ec_hq[ec_tx as usize] != 0 {
        let mut ec_hq_reg = CDC_WSA_EC_HQ0_EC_REF_HQ_PATH_CTL + 0x40 * ec_tx as u32;
        snd_soc_component_update_bits(component, ec_hq_reg, CDC_WSA_EC_HQ_EC_CLK_EN_MASK, CDC_WSA_EC_HQ_EC_CLK_ENABLE);
        ec_hq_reg = CDC_WSA_EC_HQ0_EC_REF_HQ_CFG0 + 0x40 * ec_tx as u32;
        /* default set to 48k */
        snd_soc_component_update_bits(component, ec_hq_reg, CDC_WSA_EC_HQ_EC_REF_PCM_RATE_MASK, CDC_WSA_EC_HQ_EC_REF_PCM_RATE_48K);
    }
    0
}

unsafe extern "C" fn wsa_macro_get_ec_hq(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let ec_tx = (*( (*kcontrol).private_value as *mut soc_mixer_control)).shift;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*ucontrol).value.integer.value[0] = (*wsa).ec_hq[ec_tx as usize] as c_long;
    0
}

unsafe extern "C" fn wsa_macro_set_ec_hq(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let ec_tx = (*((*kcontrol).private_value as *mut soc_mixer_control)).shift;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*wsa).ec_hq[ec_tx as usize] = value;
    0
}

unsafe extern "C" fn wsa_macro_get_compander(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let comp = (*((*kcontrol).private_value as *mut soc_mixer_control)).shift;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*ucontrol).value.integer.value[0] = (*wsa).comp_enabled[comp as usize] as c_long;
    0
}

unsafe extern "C" fn wsa_macro_set_compander(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let comp = (*((*kcontrol).private_value as *mut soc_mixer_control)).shift;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*wsa).comp_enabled[comp as usize] = value;
    0
}

unsafe extern "C" fn wsa_macro_ear_spkr_pa_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*ucontrol).value.enumerated.item[0] = (*wsa).ear_spkr_gain as c_uint;
    0
}

unsafe extern "C" fn wsa_macro_ear_spkr_pa_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*wsa).ear_spkr_gain = (*ucontrol).value.enumerated.item[0] as c_int;
    0
}

unsafe extern "C" fn wsa_macro_rx_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    (*ucontrol).value.enumerated.item[0] = (*wsa).rx_port_value[(*widget).shift as usize] as c_uint;
    0
}

unsafe extern "C" fn wsa_macro_rx_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let update: *mut snd_soc_dapm_update = ptr::null_mut();
    let rx_port_value = (*ucontrol).value.enumerated.item[0];
    let bit_input = (*widget).shift as usize;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let aif_rst = (*wsa).rx_port_value[(*widget).shift as usize] as u32;
    if rx_port_value == 0 {
        if aif_rst == 0 { return 0; }
        if aif_rst >= WSA_MACRO_RX_MAX as u32 {
            dev_err((*component).dev, b"%s: Invalid AIF reset\n\0".as_ptr() as *const c_char, b"wsa_macro_rx_mux_put\0".as_ptr());
            return 0;
        }
    }
    (*wsa).rx_port_value[(*widget).shift as usize] = rx_port_value as c_int;
    match rx_port_value {
        0 => {
            /*
             * active_ch_cnt and active_ch_mask use DAI IDs (WSA_MACRO_MAX_DAIS).
             * active_ch_cnt == 0 was tested in if() above.
             */
            let dai_id = (aif_rst - 1) as usize;
            if (*wsa).active_ch_cnt[dai_id] != 0 {
                clear_bit(bit_input, &mut (*wsa).active_ch_mask[dai_id]);
                (*wsa).active_ch_cnt[dai_id] -= 1;
            }
        }
        1 | 2 => {
            /* active_ch_cnt and active_ch_mask use DAI IDs (WSA_MACRO_MAX_DAIS). */
            let dai_id = (rx_port_value - 1) as usize;
            set_bit(bit_input, &mut (*wsa).active_ch_mask[dai_id]);
            (*wsa).active_ch_cnt[dai_id] += 1;
        }
        _ => {
            dev_err((*component).dev, b"%s: Invalid AIF_ID for WSA RX MUX %d\n\0".as_ptr() as *const c_char, b"wsa_macro_rx_mux_put\0".as_ptr(), rx_port_value);
            return -EINVAL;
        }
    }
    snd_soc_dapm_mux_update_power((*widget).dapm, kcontrol, rx_port_value, e, update);
    0
}

unsafe extern "C" fn wsa_macro_soft_clip_enable_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let path = (*((*kcontrol).private_value as *mut soc_mixer_control)).shift;
    (*ucontrol).value.integer.value[0] = (*wsa).is_softclip_on[path as usize] as c_long;
    0
}

unsafe extern "C" fn wsa_macro_soft_clip_enable_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let path = (*((*kcontrol).private_value as *mut soc_mixer_control)).shift;
    (*wsa).is_softclip_on[path as usize] = (*ucontrol).value.integer.value[0] as c_int;
    0
}

unsafe extern "C" fn wsa_macro_vi_feed_mixer_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let mixer = (*kcontrol).private_value as *mut soc_mixer_control;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let spk_tx_id = (*mixer).shift as usize;
    let dai_id = (*widget).shift as usize;
    (*ucontrol).value.integer.value[0] = if test_bit(spk_tx_id, &(*wsa).active_ch_mask[dai_id]) { 1 } else { 0 };
    0
}

unsafe extern "C" fn wsa_macro_vi_feed_mixer_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let mixer = (*kcontrol).private_value as *mut soc_mixer_control;
    let wsa = snd_soc_component_get_drvdata(component) as *mut wsa_macro;
    let enable = (*ucontrol).value.integer.value[0] as u32;
    let spk_tx_id = (*mixer).shift as usize;
    let dai_id = (*widget).shift as usize;
    if enable != 0 {
        if spk_tx_id == WSA_MACRO_TX0 && !test_bit(WSA_MACRO_TX0, &(*wsa).active_ch_mask[dai_id]) {
            set_bit(WSA_MACRO_TX0, &mut (*wsa).active_ch_mask[dai_id]);
            (*wsa).active_ch_cnt[dai_id] += 1;
        }
        if spk_tx_id == WSA_MACRO_TX1 && !test_bit(WSA_MACRO_TX1, &(*wsa).active_ch_mask[dai_id]) {
            set_bit(WSA_MACRO_TX1, &mut (*wsa).active_ch_mask[dai_id]);
            (*wsa).active_ch_cnt[dai_id] += 1;
        }
    } else {
        if spk_tx_id == WSA_MACRO_TX0 && test_bit(WSA_MACRO_TX0, &(*wsa).active_ch_mask[dai_id]) {
            clear_bit(WSA_MACRO_TX0, &mut (*wsa).active_ch_mask[dai_id]);
            (*wsa).active_ch_cnt[dai_id] -= 1;
        }
        if spk_tx_id == WSA_MACRO_TX1 && test_bit(WSA_MACRO_TX1, &(*wsa).active_ch_mask[dai_id]) {
            clear_bit(WSA_MACRO_TX1, &mut (*wsa).active_ch_mask[dai_id]);
            (*wsa).active_ch_cnt[dai_id] -= 1;
        }
    }
    snd_soc_dapm_mixer_update_power((*widget).dapm, kcontrol, enable, ptr::null_mut());
    0
}

/* The following arrays are macro-generated ALSA control/widget declarations in C:
 * rx*_mux*, wsa_macro_snd_controls, rx_mux, aif_vi_mixer,
 * wsa_macro_dapm_widgets, wsa_macro_dapm_widgets_v2_1,
 * wsa_macro_dapm_widgets_v2_5. They are preserved as zero-sized opaque arrays
 * because this isolated file does not define the target Rust constructors for
 * SOC_* or SND_SOC_DAPM_* macros.
 */
static wsa_macro_snd_controls: [snd_kcontrol_new; 0] = [];
static rx_mux: [snd_kcontrol_new; 0] = [];
static aif_vi_mixer: [snd_kcontrol_new; 0] = [];
static wsa_macro_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];
static wsa_macro_dapm_widgets_v2_1: [snd_soc_dapm_widget_desc; 0] = [];
static wsa_macro_dapm_widgets_v2_5: [snd_soc_dapm_widget_desc; 0] = [];
static wsa_audio_map: [snd_soc_dapm_route; 0] = [];

unsafe fn wsa_swrm_clock(wsa: *mut wsa_macro, enable: bool) -> c_int {
    let regmap = (*wsa).regmap;
    let mut ret: c_int;
    if enable {
        ret = pm_runtime_resume_and_get((*wsa).dev);
        if ret < 0 { return ret; }
        wsa_macro_mclk_enable(wsa, true);
        regmap_update_bits(regmap, CDC_WSA_CLK_RST_CTRL_SWR_CONTROL, CDC_WSA_SWR_CLK_EN_MASK, CDC_WSA_SWR_CLK_ENABLE);
    } else {
        regmap_update_bits(regmap, CDC_WSA_CLK_RST_CTRL_SWR_CONTROL, CDC_WSA_SWR_CLK_EN_MASK, 0);
        wsa_macro_mclk_enable(wsa, false);
        ret = pm_runtime_put_autosuspend((*wsa).dev);
        if ret < 0 { dev_warn((*wsa).dev, b"runtime PM put failed: %d\n\0".as_ptr() as *const c_char, ret); }
    }
    0
}

unsafe extern "C" fn wsa_macro_component_probe(comp: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(comp);
    let wsa = snd_soc_component_get_drvdata(comp) as *mut wsa_macro;
    let widgets: *const snd_soc_dapm_widget_desc;
    let num_widgets: c_uint;
    snd_soc_component_init_regmap(comp, (*wsa).regmap);
    (*wsa).spkr_gain_offset = WSA_MACRO_GAIN_OFFSET_M1P5_DB as c_int;
    /* set SPKR rate to FS_2P4_3P072 */
    snd_soc_component_update_bits(comp, CDC_WSA_RX0_RX_PATH_CFG1, CDC_WSA_RX_PATH_SPKR_RATE_MASK, CDC_WSA_RX_PATH_SPKR_RATE_FS_2P4_3P072);
    snd_soc_component_update_bits(comp, CDC_WSA_RX1_RX_PATH_CFG1, CDC_WSA_RX_PATH_SPKR_RATE_MASK, CDC_WSA_RX_PATH_SPKR_RATE_FS_2P4_3P072);
    wsa_macro_set_spkr_mode(comp, WSA_MACRO_SPKR_MODE_1);
    match (*wsa).codec_version {
        LPASS_CODEC_VERSION_1_0 | LPASS_CODEC_VERSION_1_1 | LPASS_CODEC_VERSION_1_2 | LPASS_CODEC_VERSION_2_0 | LPASS_CODEC_VERSION_2_1 => {
            widgets = wsa_macro_dapm_widgets_v2_1.as_ptr();
            num_widgets = wsa_macro_dapm_widgets_v2_1.len() as c_uint;
        }
        LPASS_CODEC_VERSION_2_5 | LPASS_CODEC_VERSION_2_6 | LPASS_CODEC_VERSION_2_7 | LPASS_CODEC_VERSION_2_8 | LPASS_CODEC_VERSION_2_9 => {
            widgets = wsa_macro_dapm_widgets_v2_5.as_ptr();
            num_widgets = wsa_macro_dapm_widgets_v2_5.len() as c_uint;
        }
        _ => return -EINVAL,
    }
    snd_soc_dapm_new_controls(dapm, widgets, num_widgets)
}

unsafe extern "C" fn swclk_gate_enable(hw: *mut clk_hw) -> c_int { wsa_swrm_clock(to_wsa_macro(hw), true) }
unsafe extern "C" fn swclk_gate_disable(hw: *mut clk_hw) { wsa_swrm_clock(to_wsa_macro(hw), false); }
unsafe extern "C" fn swclk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    let wsa = to_wsa_macro(hw);
    let mut val: c_int = 0;
    regmap_read((*wsa).regmap, CDC_WSA_CLK_RST_CTRL_SWR_CONTROL, &mut val);
    val & BIT(0) as c_int
}
unsafe extern "C" fn swclk_recalc_rate(_hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong { parent_rate / 2 }

static swclk_gate_ops: clk_ops = clk_ops {
    prepare: Some(swclk_gate_enable),
    unprepare: Some(swclk_gate_disable),
    is_enabled: Some(swclk_gate_is_enabled),
    recalc_rate: Some(swclk_recalc_rate),
};

unsafe fn wsa_macro_register_mclk_output(wsa: *mut wsa_macro) -> c_int {
    let dev = (*wsa).dev;
    let parent_clk_name: *const c_char = if !(*wsa).npl.is_null() { __clk_get_name((*wsa).npl) } else { __clk_get_name((*wsa).mclk) };
    let mut init = clk_init_data { name: b"mclk\0".as_ptr() as *const c_char, ops: &swclk_gate_ops, flags: 0, parent_names: &parent_clk_name, num_parents: 1 };
    of_property_read_string(dev_of_node(dev), b"clock-output-names\0".as_ptr() as *const c_char, &mut init.name);
    (*wsa).hw.init = &init;
    let hw = &mut (*wsa).hw as *mut clk_hw;
    let ret = devm_clk_hw_register((*wsa).dev, hw);
    if ret != 0 { return ret; }
    devm_of_clk_add_hw_provider(dev, &of_clk_hw_simple_get as *const c_void, hw)
}

static wsa_macro_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    name: b"WSA MACRO\0".as_ptr() as *const c_char,
    probe: Some(wsa_macro_component_probe),
    controls: wsa_macro_snd_controls.as_ptr(),
    num_controls: wsa_macro_snd_controls.len() as c_uint,
    dapm_widgets: wsa_macro_dapm_widgets.as_ptr(),
    num_dapm_widgets: wsa_macro_dapm_widgets.len() as c_uint,
    dapm_routes: wsa_audio_map.as_ptr(),
    num_dapm_routes: wsa_audio_map.len() as c_uint,
};

unsafe extern "C" fn wsa_macro_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let flags = device_get_match_data(dev) as kernel_ulong_t;
    let wsa = devm_kzalloc(dev, size_of::<wsa_macro>(), GFP_KERNEL) as *mut wsa_macro;
    if wsa.is_null() { return -ENOMEM; }
    (*wsa).macro_ = devm_clk_get_optional(dev, b"macro\0".as_ptr() as *const c_char);
    if IS_ERR((*wsa).macro_) { return dev_err_probe(dev, PTR_ERR((*wsa).macro_), b"unable to get macro clock\n\0".as_ptr() as *const c_char); }
    (*wsa).dcodec = devm_clk_get_optional(dev, b"dcodec\0".as_ptr() as *const c_char);
    if IS_ERR((*wsa).dcodec) { return dev_err_probe(dev, PTR_ERR((*wsa).dcodec), b"unable to get dcodec clock\n\0".as_ptr() as *const c_char); }
    (*wsa).mclk = devm_clk_get(dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*wsa).mclk) { return dev_err_probe(dev, PTR_ERR((*wsa).mclk), b"unable to get mclk clock\n\0".as_ptr() as *const c_char); }
    if (flags & LPASS_MACRO_FLAG_HAS_NPL_CLOCK) != 0 {
        (*wsa).npl = devm_clk_get(dev, b"npl\0".as_ptr() as *const c_char);
        if IS_ERR((*wsa).npl) { return dev_err_probe(dev, PTR_ERR((*wsa).npl), b"unable to get npl clock\n\0".as_ptr() as *const c_char); }
    }
    (*wsa).fsgen = devm_clk_get(dev, b"fsgen\0".as_ptr() as *const c_char);
    if IS_ERR((*wsa).fsgen) { return dev_err_probe(dev, PTR_ERR((*wsa).fsgen), b"unable to get fsgen clock\n\0".as_ptr() as *const c_char); }
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    (*wsa).codec_version = lpass_macro_get_codec_version();
    let mut def_count: c_int;
    let reg_defaults: *mut reg_default;
    match (*wsa).codec_version {
        LPASS_CODEC_VERSION_1_0 | LPASS_CODEC_VERSION_1_1 | LPASS_CODEC_VERSION_1_2 | LPASS_CODEC_VERSION_2_0 | LPASS_CODEC_VERSION_2_1 => {
            (*wsa).reg_layout = &wsa_codec_v2_1;
            def_count = (wsa_defaults.len() + wsa_defaults_v2_1.len()) as c_int;
            reg_defaults = kmalloc(size_of::<reg_default>() * def_count as usize, GFP_KERNEL) as *mut reg_default;
            if reg_defaults.is_null() { return -ENOMEM; }
            memcpy(reg_defaults as *mut c_void, wsa_defaults.as_ptr() as *const c_void, size_of::<reg_default>() * wsa_defaults.len());
            memcpy(reg_defaults.add(wsa_defaults.len()) as *mut c_void, wsa_defaults_v2_1.as_ptr() as *const c_void, size_of::<reg_default>() * wsa_defaults_v2_1.len());
        }
        LPASS_CODEC_VERSION_2_5 | LPASS_CODEC_VERSION_2_6 | LPASS_CODEC_VERSION_2_7 | LPASS_CODEC_VERSION_2_8 | LPASS_CODEC_VERSION_2_9 => {
            (*wsa).reg_layout = &wsa_codec_v2_5;
            def_count = (wsa_defaults.len() + wsa_defaults_v2_5.len()) as c_int;
            reg_defaults = kmalloc(size_of::<reg_default>() * def_count as usize, GFP_KERNEL) as *mut reg_default;
            if reg_defaults.is_null() { return -ENOMEM; }
            memcpy(reg_defaults as *mut c_void, wsa_defaults.as_ptr() as *const c_void, size_of::<reg_default>() * wsa_defaults.len());
            memcpy(reg_defaults.add(wsa_defaults.len()) as *mut c_void, wsa_defaults_v2_5.as_ptr() as *const c_void, size_of::<reg_default>() * wsa_defaults_v2_5.len());
        }
        _ => {
            dev_err(dev, b"Unsupported Codec version (%d)\n\0".as_ptr() as *const c_char, (*wsa).codec_version);
            return -EINVAL;
        }
    }
    let reg_config = kmemdup(&wsa_regmap_config as *const _ as *const c_void, size_of::<regmap_config>(), GFP_KERNEL) as *mut regmap_config;
    if reg_config.is_null() { return -ENOMEM; }
    (*reg_config).reg_defaults = reg_defaults;
    (*reg_config).num_reg_defaults = def_count;
    (*wsa).regmap = devm_regmap_init_mmio(dev, base, reg_config);
    if IS_ERR((*wsa).regmap) { return PTR_ERR((*wsa).regmap); }
    dev_set_drvdata(dev, wsa as *mut c_void);
    (*wsa).dev = dev;
    let mut ret = clk_set_rate((*wsa).mclk, WSA_MACRO_MCLK_FREQ);
    if ret != 0 { return ret; }
    ret = clk_set_rate((*wsa).npl, WSA_MACRO_MCLK_FREQ);
    if ret != 0 { return ret; }
    ret = devm_pm_clk_create(dev);
    if ret != 0 { return ret; }
    ret = of_pm_clk_add_clks(dev);
    if ret < 0 { return ret; }
    pm_runtime_set_autosuspend_delay(dev, 100);
    pm_runtime_use_autosuspend(dev);
    ret = devm_pm_runtime_enable(dev);
    if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 { return ret; }
    /* reset swr ip */
    regmap_update_bits((*wsa).regmap, CDC_WSA_CLK_RST_CTRL_SWR_CONTROL, CDC_WSA_SWR_RST_EN_MASK, CDC_WSA_SWR_RST_ENABLE);
    regmap_update_bits((*wsa).regmap, CDC_WSA_CLK_RST_CTRL_SWR_CONTROL, CDC_WSA_SWR_CLK_EN_MASK, CDC_WSA_SWR_CLK_ENABLE);
    /* Bring out of reset */
    regmap_update_bits((*wsa).regmap, CDC_WSA_CLK_RST_CTRL_SWR_CONTROL, CDC_WSA_SWR_RST_EN_MASK, CDC_WSA_SWR_RST_DISABLE);
    ret = devm_snd_soc_register_component(dev, &wsa_macro_component_drv, wsa_macro_dai.as_mut_ptr(), wsa_macro_dai.len() as c_int);
    if ret != 0 { return wsa_macro_probe_err_rpm_put(dev, ret); }
    ret = wsa_macro_register_mclk_output(wsa);
    if ret != 0 { return wsa_macro_probe_err_rpm_put(dev, ret); }
    ret = pm_runtime_put_autosuspend(dev);
    if ret < 0 { dev_warn(dev, b"runtime PM put failed after probe: %d\n\0".as_ptr() as *const c_char, ret); }
    0
}

unsafe fn wsa_macro_probe_err_rpm_put(dev: *mut device, ret: c_int) -> c_int {
    if pm_runtime_put_sync_suspend(dev) < 0 {
        dev_warn(dev, b"runtime PM sync suspend failed in probe unwind\n\0".as_ptr() as *const c_char);
    }
    ret
}

unsafe extern "C" fn wsa_macro_runtime_suspend(dev: *mut device) -> c_int {
    let wsa = dev_get_drvdata(dev) as *mut wsa_macro;
    regcache_cache_only((*wsa).regmap, true);
    let ret = pm_clk_suspend(dev);
    if ret != 0 {
        regcache_cache_only((*wsa).regmap, false);
        return ret;
    }
    regcache_mark_dirty((*wsa).regmap);
    0
}

unsafe extern "C" fn wsa_macro_runtime_resume(dev: *mut device) -> c_int {
    let wsa = dev_get_drvdata(dev) as *mut wsa_macro;
    let mut ret = pm_clk_resume(dev);
    if ret != 0 {
        regcache_cache_only((*wsa).regmap, true);
        regcache_mark_dirty((*wsa).regmap);
        return ret;
    }
    regcache_cache_only((*wsa).regmap, false);
    ret = regcache_sync((*wsa).regmap);
    if ret != 0 {
        regcache_cache_only((*wsa).regmap, true);
        regcache_mark_dirty((*wsa).regmap);
        let sret = pm_clk_suspend(dev);
        if sret != 0 {
            dev_err(dev, b"failed to suspend clocks after regcache sync failure: %d\n\0".as_ptr() as *const c_char, sret);
        }
        return ret;
    }
    0
}

static wsa_macro_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };
static wsa_macro_dt_match: [of_device_id; 6] = [
    of_device_id { compatible: b"qcom,sc7280-lpass-wsa-macro\0".as_ptr() as *const c_char, data: LPASS_MACRO_FLAG_HAS_NPL_CLOCK as *const c_void },
    of_device_id { compatible: b"qcom,sm8250-lpass-wsa-macro\0".as_ptr() as *const c_char, data: LPASS_MACRO_FLAG_HAS_NPL_CLOCK as *const c_void },
    of_device_id { compatible: b"qcom,sm8450-lpass-wsa-macro\0".as_ptr() as *const c_char, data: LPASS_MACRO_FLAG_HAS_NPL_CLOCK as *const c_void },
    of_device_id { compatible: b"qcom,sm8550-lpass-wsa-macro\0".as_ptr() as *const c_char, data: ptr::null() },
    of_device_id { compatible: b"qcom,sc8280xp-lpass-wsa-macro\0".as_ptr() as *const c_char, data: LPASS_MACRO_FLAG_HAS_NPL_CLOCK as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, wsa_macro_dt_match); */

static mut wsa_macro_driver: platform_driver = platform_driver {
    driver: driver_inner {
        name: b"wsa_macro\0".as_ptr() as *const c_char,
        of_match_table: wsa_macro_dt_match.as_ptr(),
        pm: &wsa_macro_pm_ops,
    },
    probe: Some(wsa_macro_probe),
};

/* module_platform_driver(wsa_macro_driver);
 * MODULE_DESCRIPTION("WSA macro driver");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
