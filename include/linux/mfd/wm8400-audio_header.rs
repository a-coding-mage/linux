/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm8400 private definitions for audio
 *
 * Copyright 2008 Wolfson Microelectronics plc
 */

// C header guard omitted in Rust: #ifndef __LINUX_MFD_WM8400_AUDIO_H
// C header guard omitted in Rust: #define __LINUX_MFD_WM8400_AUDIO_H

// Dependency retained from C header: #include <linux/mfd/wm8400-audio.h>

/*
 * R2 (0x02) - Power Management (1)
 */
pub const WM8400_CODEC_ENA: u16 = 0x8000u16;  /* CODEC_ENA */
pub const WM8400_CODEC_ENA_MASK: u16 = 0x8000u16;  /* CODEC_ENA */
pub const WM8400_CODEC_ENA_SHIFT: u16 = 15u16;  /* CODEC_ENA */
pub const WM8400_CODEC_ENA_WIDTH: u16 = 1u16;  /* CODEC_ENA */
pub const WM8400_SYSCLK_ENA: u16 = 0x4000u16;  /* SYSCLK_ENA */
pub const WM8400_SYSCLK_ENA_MASK: u16 = 0x4000u16;  /* SYSCLK_ENA */
pub const WM8400_SYSCLK_ENA_SHIFT: u16 = 14u16;  /* SYSCLK_ENA */
pub const WM8400_SYSCLK_ENA_WIDTH: u16 = 1u16;  /* SYSCLK_ENA */
pub const WM8400_SPK_MIX_ENA: u16 = 0x2000u16;  /* SPK_MIX_ENA */
pub const WM8400_SPK_MIX_ENA_MASK: u16 = 0x2000u16;  /* SPK_MIX_ENA */
pub const WM8400_SPK_MIX_ENA_SHIFT: u16 = 13u16;  /* SPK_MIX_ENA */
pub const WM8400_SPK_MIX_ENA_WIDTH: u16 = 1u16;  /* SPK_MIX_ENA */
pub const WM8400_SPK_ENA: u16 = 0x1000u16;  /* SPK_ENA */
pub const WM8400_SPK_ENA_MASK: u16 = 0x1000u16;  /* SPK_ENA */
pub const WM8400_SPK_ENA_SHIFT: u16 = 12u16;  /* SPK_ENA */
pub const WM8400_SPK_ENA_WIDTH: u16 = 1u16;  /* SPK_ENA */
pub const WM8400_OUT3_ENA: u16 = 0x0800u16;  /* OUT3_ENA */
pub const WM8400_OUT3_ENA_MASK: u16 = 0x0800u16;  /* OUT3_ENA */
pub const WM8400_OUT3_ENA_SHIFT: u16 = 11u16;  /* OUT3_ENA */
pub const WM8400_OUT3_ENA_WIDTH: u16 = 1u16;  /* OUT3_ENA */
pub const WM8400_OUT4_ENA: u16 = 0x0400u16;  /* OUT4_ENA */
pub const WM8400_OUT4_ENA_MASK: u16 = 0x0400u16;  /* OUT4_ENA */
pub const WM8400_OUT4_ENA_SHIFT: u16 = 10u16;  /* OUT4_ENA */
pub const WM8400_OUT4_ENA_WIDTH: u16 = 1u16;  /* OUT4_ENA */
pub const WM8400_LOUT_ENA: u16 = 0x0200u16;  /* LOUT_ENA */
pub const WM8400_LOUT_ENA_MASK: u16 = 0x0200u16;  /* LOUT_ENA */
pub const WM8400_LOUT_ENA_SHIFT: u16 = 9u16;  /* LOUT_ENA */
pub const WM8400_LOUT_ENA_WIDTH: u16 = 1u16;  /* LOUT_ENA */
pub const WM8400_ROUT_ENA: u16 = 0x0100u16;  /* ROUT_ENA */
pub const WM8400_ROUT_ENA_MASK: u16 = 0x0100u16;  /* ROUT_ENA */
pub const WM8400_ROUT_ENA_SHIFT: u16 = 8u16;  /* ROUT_ENA */
pub const WM8400_ROUT_ENA_WIDTH: u16 = 1u16;  /* ROUT_ENA */
pub const WM8400_MIC1BIAS_ENA: u16 = 0x0010u16;  /* MIC1BIAS_ENA */
pub const WM8400_MIC1BIAS_ENA_MASK: u16 = 0x0010u16;  /* MIC1BIAS_ENA */
pub const WM8400_MIC1BIAS_ENA_SHIFT: u16 = 4u16;  /* MIC1BIAS_ENA */
pub const WM8400_MIC1BIAS_ENA_WIDTH: u16 = 1u16;  /* MIC1BIAS_ENA */
pub const WM8400_VMID_MODE_MASK: u16 = 0x0006u16;  /* VMID_MODE - [2:1] */
pub const WM8400_VMID_MODE_SHIFT: u16 = 1u16;  /* VMID_MODE - [2:1] */
pub const WM8400_VMID_MODE_WIDTH: u16 = 2u16;  /* VMID_MODE - [2:1] */
pub const WM8400_VREF_ENA: u16 = 0x0001u16;  /* VREF_ENA */
pub const WM8400_VREF_ENA_MASK: u16 = 0x0001u16;  /* VREF_ENA */
pub const WM8400_VREF_ENA_SHIFT: u16 = 0u16;  /* VREF_ENA */
pub const WM8400_VREF_ENA_WIDTH: u16 = 1u16;  /* VREF_ENA */

/*
 * R3 (0x03) - Power Management (2)
 */
pub const WM8400_FLL_ENA: u16 = 0x8000u16;  /* FLL_ENA */
pub const WM8400_FLL_ENA_MASK: u16 = 0x8000u16;  /* FLL_ENA */
pub const WM8400_FLL_ENA_SHIFT: u16 = 15u16;  /* FLL_ENA */
pub const WM8400_FLL_ENA_WIDTH: u16 = 1u16;  /* FLL_ENA */
pub const WM8400_TSHUT_ENA: u16 = 0x4000u16;  /* TSHUT_ENA */
pub const WM8400_TSHUT_ENA_MASK: u16 = 0x4000u16;  /* TSHUT_ENA */
pub const WM8400_TSHUT_ENA_SHIFT: u16 = 14u16;  /* TSHUT_ENA */
pub const WM8400_TSHUT_ENA_WIDTH: u16 = 1u16;  /* TSHUT_ENA */
pub const WM8400_TSHUT_OPDIS: u16 = 0x2000u16;  /* TSHUT_OPDIS */
pub const WM8400_TSHUT_OPDIS_MASK: u16 = 0x2000u16;  /* TSHUT_OPDIS */
pub const WM8400_TSHUT_OPDIS_SHIFT: u16 = 13u16;  /* TSHUT_OPDIS */
pub const WM8400_TSHUT_OPDIS_WIDTH: u16 = 1u16;  /* TSHUT_OPDIS */
pub const WM8400_OPCLK_ENA: u16 = 0x0800u16;  /* OPCLK_ENA */
pub const WM8400_OPCLK_ENA_MASK: u16 = 0x0800u16;  /* OPCLK_ENA */
pub const WM8400_OPCLK_ENA_SHIFT: u16 = 11u16;  /* OPCLK_ENA */
pub const WM8400_OPCLK_ENA_WIDTH: u16 = 1u16;  /* OPCLK_ENA */
pub const WM8400_AINL_ENA: u16 = 0x0200u16;  /* AINL_ENA */
pub const WM8400_AINL_ENA_MASK: u16 = 0x0200u16;  /* AINL_ENA */
pub const WM8400_AINL_ENA_SHIFT: u16 = 9u16;  /* AINL_ENA */
pub const WM8400_AINL_ENA_WIDTH: u16 = 1u16;  /* AINL_ENA */
pub const WM8400_AINR_ENA: u16 = 0x0100u16;  /* AINR_ENA */
pub const WM8400_AINR_ENA_MASK: u16 = 0x0100u16;  /* AINR_ENA */
pub const WM8400_AINR_ENA_SHIFT: u16 = 8u16;  /* AINR_ENA */
pub const WM8400_AINR_ENA_WIDTH: u16 = 1u16;  /* AINR_ENA */
pub const WM8400_LIN34_ENA: u16 = 0x0080u16;  /* LIN34_ENA */
pub const WM8400_LIN34_ENA_MASK: u16 = 0x0080u16;  /* LIN34_ENA */
pub const WM8400_LIN34_ENA_SHIFT: u16 = 7u16;  /* LIN34_ENA */
pub const WM8400_LIN34_ENA_WIDTH: u16 = 1u16;  /* LIN34_ENA */
pub const WM8400_LIN12_ENA: u16 = 0x0040u16;  /* LIN12_ENA */
pub const WM8400_LIN12_ENA_MASK: u16 = 0x0040u16;  /* LIN12_ENA */
pub const WM8400_LIN12_ENA_SHIFT: u16 = 6u16;  /* LIN12_ENA */
pub const WM8400_LIN12_ENA_WIDTH: u16 = 1u16;  /* LIN12_ENA */
pub const WM8400_RIN34_ENA: u16 = 0x0020u16;  /* RIN34_ENA */
pub const WM8400_RIN34_ENA_MASK: u16 = 0x0020u16;  /* RIN34_ENA */
pub const WM8400_RIN34_ENA_SHIFT: u16 = 5u16;  /* RIN34_ENA */
pub const WM8400_RIN34_ENA_WIDTH: u16 = 1u16;  /* RIN34_ENA */
pub const WM8400_RIN12_ENA: u16 = 0x0010u16;  /* RIN12_ENA */
pub const WM8400_RIN12_ENA_MASK: u16 = 0x0010u16;  /* RIN12_ENA */
pub const WM8400_RIN12_ENA_SHIFT: u16 = 4u16;  /* RIN12_ENA */
pub const WM8400_RIN12_ENA_WIDTH: u16 = 1u16;  /* RIN12_ENA */
pub const WM8400_ADCL_ENA: u16 = 0x0002u16;  /* ADCL_ENA */
pub const WM8400_ADCL_ENA_MASK: u16 = 0x0002u16;  /* ADCL_ENA */
pub const WM8400_ADCL_ENA_SHIFT: u16 = 1u16;  /* ADCL_ENA */
pub const WM8400_ADCL_ENA_WIDTH: u16 = 1u16;  /* ADCL_ENA */
pub const WM8400_ADCR_ENA: u16 = 0x0001u16;  /* ADCR_ENA */
pub const WM8400_ADCR_ENA_MASK: u16 = 0x0001u16;  /* ADCR_ENA */
pub const WM8400_ADCR_ENA_SHIFT: u16 = 0u16;  /* ADCR_ENA */
pub const WM8400_ADCR_ENA_WIDTH: u16 = 1u16;  /* ADCR_ENA */

/*
 * R4 (0x04) - Power Management (3)
 */
pub const WM8400_LON_ENA: u16 = 0x2000u16;  /* LON_ENA */
pub const WM8400_LON_ENA_MASK: u16 = 0x2000u16;  /* LON_ENA */
pub const WM8400_LON_ENA_SHIFT: u16 = 13u16;  /* LON_ENA */
pub const WM8400_LON_ENA_WIDTH: u16 = 1u16;  /* LON_ENA */
pub const WM8400_LOP_ENA: u16 = 0x1000u16;  /* LOP_ENA */
pub const WM8400_LOP_ENA_MASK: u16 = 0x1000u16;  /* LOP_ENA */
pub const WM8400_LOP_ENA_SHIFT: u16 = 12u16;  /* LOP_ENA */
pub const WM8400_LOP_ENA_WIDTH: u16 = 1u16;  /* LOP_ENA */
pub const WM8400_RON_ENA: u16 = 0x0800u16;  /* RON_ENA */
pub const WM8400_RON_ENA_MASK: u16 = 0x0800u16;  /* RON_ENA */
pub const WM8400_RON_ENA_SHIFT: u16 = 11u16;  /* RON_ENA */
pub const WM8400_RON_ENA_WIDTH: u16 = 1u16;  /* RON_ENA */
pub const WM8400_ROP_ENA: u16 = 0x0400u16;  /* ROP_ENA */
pub const WM8400_ROP_ENA_MASK: u16 = 0x0400u16;  /* ROP_ENA */
pub const WM8400_ROP_ENA_SHIFT: u16 = 10u16;  /* ROP_ENA */
pub const WM8400_ROP_ENA_WIDTH: u16 = 1u16;  /* ROP_ENA */
pub const WM8400_LOPGA_ENA: u16 = 0x0080u16;  /* LOPGA_ENA */
pub const WM8400_LOPGA_ENA_MASK: u16 = 0x0080u16;  /* LOPGA_ENA */
pub const WM8400_LOPGA_ENA_SHIFT: u16 = 7u16;  /* LOPGA_ENA */
pub const WM8400_LOPGA_ENA_WIDTH: u16 = 1u16;  /* LOPGA_ENA */
pub const WM8400_ROPGA_ENA: u16 = 0x0040u16;  /* ROPGA_ENA */
pub const WM8400_ROPGA_ENA_MASK: u16 = 0x0040u16;  /* ROPGA_ENA */
pub const WM8400_ROPGA_ENA_SHIFT: u16 = 6u16;  /* ROPGA_ENA */
pub const WM8400_ROPGA_ENA_WIDTH: u16 = 1u16;  /* ROPGA_ENA */
pub const WM8400_LOMIX_ENA: u16 = 0x0020u16;  /* LOMIX_ENA */
pub const WM8400_LOMIX_ENA_MASK: u16 = 0x0020u16;  /* LOMIX_ENA */
pub const WM8400_LOMIX_ENA_SHIFT: u16 = 5u16;  /* LOMIX_ENA */
pub const WM8400_LOMIX_ENA_WIDTH: u16 = 1u16;  /* LOMIX_ENA */
pub const WM8400_ROMIX_ENA: u16 = 0x0010u16;  /* ROMIX_ENA */
pub const WM8400_ROMIX_ENA_MASK: u16 = 0x0010u16;  /* ROMIX_ENA */
pub const WM8400_ROMIX_ENA_SHIFT: u16 = 4u16;  /* ROMIX_ENA */
pub const WM8400_ROMIX_ENA_WIDTH: u16 = 1u16;  /* ROMIX_ENA */
pub const WM8400_DACL_ENA: u16 = 0x0002u16;  /* DACL_ENA */
pub const WM8400_DACL_ENA_MASK: u16 = 0x0002u16;  /* DACL_ENA */
pub const WM8400_DACL_ENA_SHIFT: u16 = 1u16;  /* DACL_ENA */
pub const WM8400_DACL_ENA_WIDTH: u16 = 1u16;  /* DACL_ENA */
pub const WM8400_DACR_ENA: u16 = 0x0001u16;  /* DACR_ENA */
pub const WM8400_DACR_ENA_MASK: u16 = 0x0001u16;  /* DACR_ENA */
pub const WM8400_DACR_ENA_SHIFT: u16 = 0u16;  /* DACR_ENA */
pub const WM8400_DACR_ENA_WIDTH: u16 = 1u16;  /* DACR_ENA */

/*
 * R5 (0x05) - Audio Interface (1)
 */
pub const WM8400_AIFADCL_SRC: u16 = 0x8000u16;  /* AIFADCL_SRC */
pub const WM8400_AIFADCL_SRC_MASK: u16 = 0x8000u16;  /* AIFADCL_SRC */
pub const WM8400_AIFADCL_SRC_SHIFT: u16 = 15u16;  /* AIFADCL_SRC */
pub const WM8400_AIFADCL_SRC_WIDTH: u16 = 1u16;  /* AIFADCL_SRC */
pub const WM8400_AIFADCR_SRC: u16 = 0x4000u16;  /* AIFADCR_SRC */
pub const WM8400_AIFADCR_SRC_MASK: u16 = 0x4000u16;  /* AIFADCR_SRC */
pub const WM8400_AIFADCR_SRC_SHIFT: u16 = 14u16;  /* AIFADCR_SRC */
pub const WM8400_AIFADCR_SRC_WIDTH: u16 = 1u16;  /* AIFADCR_SRC */
pub const WM8400_AIFADC_TDM: u16 = 0x2000u16;  /* AIFADC_TDM */
pub const WM8400_AIFADC_TDM_MASK: u16 = 0x2000u16;  /* AIFADC_TDM */
pub const WM8400_AIFADC_TDM_SHIFT: u16 = 13u16;  /* AIFADC_TDM */
pub const WM8400_AIFADC_TDM_WIDTH: u16 = 1u16;  /* AIFADC_TDM */
pub const WM8400_AIFADC_TDM_CHAN: u16 = 0x1000u16;  /* AIFADC_TDM_CHAN */
pub const WM8400_AIFADC_TDM_CHAN_MASK: u16 = 0x1000u16;  /* AIFADC_TDM_CHAN */
pub const WM8400_AIFADC_TDM_CHAN_SHIFT: u16 = 12u16;  /* AIFADC_TDM_CHAN */
pub const WM8400_AIFADC_TDM_CHAN_WIDTH: u16 = 1u16;  /* AIFADC_TDM_CHAN */
pub const WM8400_AIF_BCLK_INV: u16 = 0x0100u16;  /* AIF_BCLK_INV */
pub const WM8400_AIF_BCLK_INV_MASK: u16 = 0x0100u16;  /* AIF_BCLK_INV */
pub const WM8400_AIF_BCLK_INV_SHIFT: u16 = 8u16;  /* AIF_BCLK_INV */
pub const WM8400_AIF_BCLK_INV_WIDTH: u16 = 1u16;  /* AIF_BCLK_INV */
pub const WM8400_AIF_LRCLK_INV: u16 = 0x0080u16;  /* AIF_LRCLK_INV */
pub const WM8400_AIF_LRCLK_INV_MASK: u16 = 0x0080u16;  /* AIF_LRCLK_INV */
pub const WM8400_AIF_LRCLK_INV_SHIFT: u16 = 7u16;  /* AIF_LRCLK_INV */
pub const WM8400_AIF_LRCLK_INV_WIDTH: u16 = 1u16;  /* AIF_LRCLK_INV */
pub const WM8400_AIF_WL_MASK: u16 = 0x0060u16;  /* AIF_WL - [6:5] */
pub const WM8400_AIF_WL_SHIFT: u16 = 5u16;  /* AIF_WL - [6:5] */
pub const WM8400_AIF_WL_WIDTH: u16 = 2u16;  /* AIF_WL - [6:5] */
pub const WM8400_AIF_WL_16BITS: u16 = (0u16 << 5u16);
pub const WM8400_AIF_WL_20BITS: u16 = (1u16 << 5u16);
pub const WM8400_AIF_WL_24BITS: u16 = (2u16 << 5u16);
pub const WM8400_AIF_WL_32BITS: u16 = (3u16 << 5u16);
pub const WM8400_AIF_FMT_MASK: u16 = 0x0018u16;  /* AIF_FMT - [4:3] */
pub const WM8400_AIF_FMT_SHIFT: u16 = 3u16;  /* AIF_FMT - [4:3] */
pub const WM8400_AIF_FMT_WIDTH: u16 = 2u16;  /* AIF_FMT - [4:3] */
pub const WM8400_AIF_FMT_RIGHTJ: u16 = (0u16 << 3u16);
pub const WM8400_AIF_FMT_LEFTJ: u16 = (1u16 << 3u16);
pub const WM8400_AIF_FMT_I2S: u16 = (2u16 << 3u16);
pub const WM8400_AIF_FMT_DSP: u16 = (3u16 << 3u16);

/*
 * R6 (0x06) - Audio Interface (2)
 */
pub const WM8400_DACL_SRC: u16 = 0x8000u16;  /* DACL_SRC */
pub const WM8400_DACL_SRC_MASK: u16 = 0x8000u16;  /* DACL_SRC */
pub const WM8400_DACL_SRC_SHIFT: u16 = 15u16;  /* DACL_SRC */
pub const WM8400_DACL_SRC_WIDTH: u16 = 1u16;  /* DACL_SRC */
pub const WM8400_DACR_SRC: u16 = 0x4000u16;  /* DACR_SRC */
pub const WM8400_DACR_SRC_MASK: u16 = 0x4000u16;  /* DACR_SRC */
pub const WM8400_DACR_SRC_SHIFT: u16 = 14u16;  /* DACR_SRC */
pub const WM8400_DACR_SRC_WIDTH: u16 = 1u16;  /* DACR_SRC */
pub const WM8400_AIFDAC_TDM: u16 = 0x2000u16;  /* AIFDAC_TDM */
pub const WM8400_AIFDAC_TDM_MASK: u16 = 0x2000u16;  /* AIFDAC_TDM */
pub const WM8400_AIFDAC_TDM_SHIFT: u16 = 13u16;  /* AIFDAC_TDM */
pub const WM8400_AIFDAC_TDM_WIDTH: u16 = 1u16;  /* AIFDAC_TDM */
pub const WM8400_AIFDAC_TDM_CHAN: u16 = 0x1000u16;  /* AIFDAC_TDM_CHAN */
pub const WM8400_AIFDAC_TDM_CHAN_MASK: u16 = 0x1000u16;  /* AIFDAC_TDM_CHAN */
pub const WM8400_AIFDAC_TDM_CHAN_SHIFT: u16 = 12u16;  /* AIFDAC_TDM_CHAN */
pub const WM8400_AIFDAC_TDM_CHAN_WIDTH: u16 = 1u16;  /* AIFDAC_TDM_CHAN */
pub const WM8400_DAC_BOOST_MASK: u16 = 0x0C00u16;  /* DAC_BOOST - [11:10] */
pub const WM8400_DAC_BOOST_SHIFT: u16 = 10u16;  /* DAC_BOOST - [11:10] */
pub const WM8400_DAC_BOOST_WIDTH: u16 = 2u16;  /* DAC_BOOST - [11:10] */
pub const WM8400_DAC_COMP: u16 = 0x0010u16;  /* DAC_COMP */
pub const WM8400_DAC_COMP_MASK: u16 = 0x0010u16;  /* DAC_COMP */
pub const WM8400_DAC_COMP_SHIFT: u16 = 4u16;  /* DAC_COMP */
pub const WM8400_DAC_COMP_WIDTH: u16 = 1u16;  /* DAC_COMP */
pub const WM8400_DAC_COMPMODE: u16 = 0x0008u16;  /* DAC_COMPMODE */
pub const WM8400_DAC_COMPMODE_MASK: u16 = 0x0008u16;  /* DAC_COMPMODE */
pub const WM8400_DAC_COMPMODE_SHIFT: u16 = 3u16;  /* DAC_COMPMODE */
pub const WM8400_DAC_COMPMODE_WIDTH: u16 = 1u16;  /* DAC_COMPMODE */
pub const WM8400_ADC_COMP: u16 = 0x0004u16;  /* ADC_COMP */
pub const WM8400_ADC_COMP_MASK: u16 = 0x0004u16;  /* ADC_COMP */
pub const WM8400_ADC_COMP_SHIFT: u16 = 2u16;  /* ADC_COMP */
pub const WM8400_ADC_COMP_WIDTH: u16 = 1u16;  /* ADC_COMP */
pub const WM8400_ADC_COMPMODE: u16 = 0x0002u16;  /* ADC_COMPMODE */
pub const WM8400_ADC_COMPMODE_MASK: u16 = 0x0002u16;  /* ADC_COMPMODE */
pub const WM8400_ADC_COMPMODE_SHIFT: u16 = 1u16;  /* ADC_COMPMODE */
pub const WM8400_ADC_COMPMODE_WIDTH: u16 = 1u16;  /* ADC_COMPMODE */
pub const WM8400_LOOPBACK: u16 = 0x0001u16;  /* LOOPBACK */
pub const WM8400_LOOPBACK_MASK: u16 = 0x0001u16;  /* LOOPBACK */
pub const WM8400_LOOPBACK_SHIFT: u16 = 0u16;  /* LOOPBACK */
pub const WM8400_LOOPBACK_WIDTH: u16 = 1u16;  /* LOOPBACK */

/*
 * R7 (0x07) - Clocking (1)
 */
pub const WM8400_TOCLK_RATE: u16 = 0x8000u16;  /* TOCLK_RATE */
pub const WM8400_TOCLK_RATE_MASK: u16 = 0x8000u16;  /* TOCLK_RATE */
pub const WM8400_TOCLK_RATE_SHIFT: u16 = 15u16;  /* TOCLK_RATE */
pub const WM8400_TOCLK_RATE_WIDTH: u16 = 1u16;  /* TOCLK_RATE */
pub const WM8400_TOCLK_ENA: u16 = 0x4000u16;  /* TOCLK_ENA */
pub const WM8400_TOCLK_ENA_MASK: u16 = 0x4000u16;  /* TOCLK_ENA */
pub const WM8400_TOCLK_ENA_SHIFT: u16 = 14u16;  /* TOCLK_ENA */
pub const WM8400_TOCLK_ENA_WIDTH: u16 = 1u16;  /* TOCLK_ENA */
pub const WM8400_OPCLKDIV_MASK: u16 = 0x1E00u16;  /* OPCLKDIV - [12:9] */
pub const WM8400_OPCLKDIV_SHIFT: u16 = 9u16;  /* OPCLKDIV - [12:9] */
pub const WM8400_OPCLKDIV_WIDTH: u16 = 4u16;  /* OPCLKDIV - [12:9] */
pub const WM8400_DCLKDIV_MASK: u16 = 0x01C0u16;  /* DCLKDIV - [8:6] */
pub const WM8400_DCLKDIV_SHIFT: u16 = 6u16;  /* DCLKDIV - [8:6] */
pub const WM8400_DCLKDIV_WIDTH: u16 = 3u16;  /* DCLKDIV - [8:6] */
pub const WM8400_BCLK_DIV_MASK: u16 = 0x001Eu16;  /* BCLK_DIV - [4:1] */
pub const WM8400_BCLK_DIV_SHIFT: u16 = 1u16;  /* BCLK_DIV - [4:1] */
pub const WM8400_BCLK_DIV_WIDTH: u16 = 4u16;  /* BCLK_DIV - [4:1] */

/*
 * R8 (0x08) - Clocking (2)
 */
pub const WM8400_MCLK_SRC: u16 = 0x8000u16;  /* MCLK_SRC */
pub const WM8400_MCLK_SRC_MASK: u16 = 0x8000u16;  /* MCLK_SRC */
pub const WM8400_MCLK_SRC_SHIFT: u16 = 15u16;  /* MCLK_SRC */
pub const WM8400_MCLK_SRC_WIDTH: u16 = 1u16;  /* MCLK_SRC */
pub const WM8400_SYSCLK_SRC: u16 = 0x4000u16;  /* SYSCLK_SRC */
pub const WM8400_SYSCLK_SRC_MASK: u16 = 0x4000u16;  /* SYSCLK_SRC */
pub const WM8400_SYSCLK_SRC_SHIFT: u16 = 14u16;  /* SYSCLK_SRC */
pub const WM8400_SYSCLK_SRC_WIDTH: u16 = 1u16;  /* SYSCLK_SRC */
pub const WM8400_CLK_FORCE: u16 = 0x2000u16;  /* CLK_FORCE */
pub const WM8400_CLK_FORCE_MASK: u16 = 0x2000u16;  /* CLK_FORCE */
pub const WM8400_CLK_FORCE_SHIFT: u16 = 13u16;  /* CLK_FORCE */
pub const WM8400_CLK_FORCE_WIDTH: u16 = 1u16;  /* CLK_FORCE */
pub const WM8400_MCLK_DIV_MASK: u16 = 0x1800u16;  /* MCLK_DIV - [12:11] */
pub const WM8400_MCLK_DIV_SHIFT: u16 = 11u16;  /* MCLK_DIV - [12:11] */
pub const WM8400_MCLK_DIV_WIDTH: u16 = 2u16;  /* MCLK_DIV - [12:11] */
pub const WM8400_MCLK_INV: u16 = 0x0400u16;  /* MCLK_INV */
pub const WM8400_MCLK_INV_MASK: u16 = 0x0400u16;  /* MCLK_INV */
pub const WM8400_MCLK_INV_SHIFT: u16 = 10u16;  /* MCLK_INV */
pub const WM8400_MCLK_INV_WIDTH: u16 = 1u16;  /* MCLK_INV */
pub const WM8400_ADC_CLKDIV_MASK: u16 = 0x00E0u16;  /* ADC_CLKDIV - [7:5] */
pub const WM8400_ADC_CLKDIV_SHIFT: u16 = 5u16;  /* ADC_CLKDIV - [7:5] */
pub const WM8400_ADC_CLKDIV_WIDTH: u16 = 3u16;  /* ADC_CLKDIV - [7:5] */
pub const WM8400_DAC_CLKDIV_MASK: u16 = 0x001Cu16;  /* DAC_CLKDIV - [4:2] */
pub const WM8400_DAC_CLKDIV_SHIFT: u16 = 2u16;  /* DAC_CLKDIV - [4:2] */
pub const WM8400_DAC_CLKDIV_WIDTH: u16 = 3u16;  /* DAC_CLKDIV - [4:2] */

/*
 * R9 (0x09) - Audio Interface (3)
 */
pub const WM8400_AIF_MSTR1: u16 = 0x8000u16;  /* AIF_MSTR1 */
pub const WM8400_AIF_MSTR1_MASK: u16 = 0x8000u16;  /* AIF_MSTR1 */
pub const WM8400_AIF_MSTR1_SHIFT: u16 = 15u16;  /* AIF_MSTR1 */
pub const WM8400_AIF_MSTR1_WIDTH: u16 = 1u16;  /* AIF_MSTR1 */
pub const WM8400_AIF_MSTR2: u16 = 0x4000u16;  /* AIF_MSTR2 */
pub const WM8400_AIF_MSTR2_MASK: u16 = 0x4000u16;  /* AIF_MSTR2 */
pub const WM8400_AIF_MSTR2_SHIFT: u16 = 14u16;  /* AIF_MSTR2 */
pub const WM8400_AIF_MSTR2_WIDTH: u16 = 1u16;  /* AIF_MSTR2 */
pub const WM8400_AIF_SEL: u16 = 0x2000u16;  /* AIF_SEL */
pub const WM8400_AIF_SEL_MASK: u16 = 0x2000u16;  /* AIF_SEL */
pub const WM8400_AIF_SEL_SHIFT: u16 = 13u16;  /* AIF_SEL */
pub const WM8400_AIF_SEL_WIDTH: u16 = 1u16;  /* AIF_SEL */
pub const WM8400_ADCLRC_DIR: u16 = 0x0800u16;  /* ADCLRC_DIR */
pub const WM8400_ADCLRC_DIR_MASK: u16 = 0x0800u16;  /* ADCLRC_DIR */
pub const WM8400_ADCLRC_DIR_SHIFT: u16 = 11u16;  /* ADCLRC_DIR */
pub const WM8400_ADCLRC_DIR_WIDTH: u16 = 1u16;  /* ADCLRC_DIR */
pub const WM8400_ADCLRC_RATE_MASK: u16 = 0x07FFu16;  /* ADCLRC_RATE - [10:0] */
pub const WM8400_ADCLRC_RATE_SHIFT: u16 = 0u16;  /* ADCLRC_RATE - [10:0] */
pub const WM8400_ADCLRC_RATE_WIDTH: u16 = 11u16;  /* ADCLRC_RATE - [10:0] */

/*
 * R10 (0x0A) - Audio Interface (4)
 */
pub const WM8400_ALRCGPIO1: u16 = 0x8000u16;  /* ALRCGPIO1 */
pub const WM8400_ALRCGPIO1_MASK: u16 = 0x8000u16;  /* ALRCGPIO1 */
pub const WM8400_ALRCGPIO1_SHIFT: u16 = 15u16;  /* ALRCGPIO1 */
pub const WM8400_ALRCGPIO1_WIDTH: u16 = 1u16;  /* ALRCGPIO1 */
pub const WM8400_ALRCBGPIO6: u16 = 0x4000u16;  /* ALRCBGPIO6 */
pub const WM8400_ALRCBGPIO6_MASK: u16 = 0x4000u16;  /* ALRCBGPIO6 */
pub const WM8400_ALRCBGPIO6_SHIFT: u16 = 14u16;  /* ALRCBGPIO6 */
pub const WM8400_ALRCBGPIO6_WIDTH: u16 = 1u16;  /* ALRCBGPIO6 */
pub const WM8400_AIF_TRIS: u16 = 0x2000u16;  /* AIF_TRIS */
pub const WM8400_AIF_TRIS_MASK: u16 = 0x2000u16;  /* AIF_TRIS */
pub const WM8400_AIF_TRIS_SHIFT: u16 = 13u16;  /* AIF_TRIS */
pub const WM8400_AIF_TRIS_WIDTH: u16 = 1u16;  /* AIF_TRIS */
pub const WM8400_DACLRC_DIR: u16 = 0x0800u16;  /* DACLRC_DIR */
pub const WM8400_DACLRC_DIR_MASK: u16 = 0x0800u16;  /* DACLRC_DIR */
pub const WM8400_DACLRC_DIR_SHIFT: u16 = 11u16;  /* DACLRC_DIR */
pub const WM8400_DACLRC_DIR_WIDTH: u16 = 1u16;  /* DACLRC_DIR */
pub const WM8400_DACLRC_RATE_MASK: u16 = 0x07FFu16;  /* DACLRC_RATE - [10:0] */
pub const WM8400_DACLRC_RATE_SHIFT: u16 = 0u16;  /* DACLRC_RATE - [10:0] */
pub const WM8400_DACLRC_RATE_WIDTH: u16 = 11u16;  /* DACLRC_RATE - [10:0] */

/*
 * R11 (0x0B) - DAC CTRL
 */
pub const WM8400_DAC_SDMCLK_RATE: u16 = 0x2000u16;  /* DAC_SDMCLK_RATE */
pub const WM8400_DAC_SDMCLK_RATE_MASK: u16 = 0x2000u16;  /* DAC_SDMCLK_RATE */
pub const WM8400_DAC_SDMCLK_RATE_SHIFT: u16 = 13u16;  /* DAC_SDMCLK_RATE */
pub const WM8400_DAC_SDMCLK_RATE_WIDTH: u16 = 1u16;  /* DAC_SDMCLK_RATE */
pub const WM8400_AIF_LRCLKRATE: u16 = 0x0400u16;  /* AIF_LRCLKRATE */
pub const WM8400_AIF_LRCLKRATE_MASK: u16 = 0x0400u16;  /* AIF_LRCLKRATE */
pub const WM8400_AIF_LRCLKRATE_SHIFT: u16 = 10u16;  /* AIF_LRCLKRATE */
pub const WM8400_AIF_LRCLKRATE_WIDTH: u16 = 1u16;  /* AIF_LRCLKRATE */
pub const WM8400_DAC_MONO: u16 = 0x0200u16;  /* DAC_MONO */
pub const WM8400_DAC_MONO_MASK: u16 = 0x0200u16;  /* DAC_MONO */
pub const WM8400_DAC_MONO_SHIFT: u16 = 9u16;  /* DAC_MONO */
pub const WM8400_DAC_MONO_WIDTH: u16 = 1u16;  /* DAC_MONO */
pub const WM8400_DAC_SB_FILT: u16 = 0x0100u16;  /* DAC_SB_FILT */
pub const WM8400_DAC_SB_FILT_MASK: u16 = 0x0100u16;  /* DAC_SB_FILT */
pub const WM8400_DAC_SB_FILT_SHIFT: u16 = 8u16;  /* DAC_SB_FILT */
pub const WM8400_DAC_SB_FILT_WIDTH: u16 = 1u16;  /* DAC_SB_FILT */
pub const WM8400_DAC_MUTERATE: u16 = 0x0080u16;  /* DAC_MUTERATE */
pub const WM8400_DAC_MUTERATE_MASK: u16 = 0x0080u16;  /* DAC_MUTERATE */
pub const WM8400_DAC_MUTERATE_SHIFT: u16 = 7u16;  /* DAC_MUTERATE */
pub const WM8400_DAC_MUTERATE_WIDTH: u16 = 1u16;  /* DAC_MUTERATE */
pub const WM8400_DAC_MUTEMODE: u16 = 0x0040u16;  /* DAC_MUTEMODE */
pub const WM8400_DAC_MUTEMODE_MASK: u16 = 0x0040u16;  /* DAC_MUTEMODE */
pub const WM8400_DAC_MUTEMODE_SHIFT: u16 = 6u16;  /* DAC_MUTEMODE */
pub const WM8400_DAC_MUTEMODE_WIDTH: u16 = 1u16;  /* DAC_MUTEMODE */
pub const WM8400_DEEMP_MASK: u16 = 0x0030u16;  /* DEEMP - [5:4] */
pub const WM8400_DEEMP_SHIFT: u16 = 4u16;  /* DEEMP - [5:4] */
pub const WM8400_DEEMP_WIDTH: u16 = 2u16;  /* DEEMP - [5:4] */
pub const WM8400_DAC_MUTE: u16 = 0x0004u16;  /* DAC_MUTE */
pub const WM8400_DAC_MUTE_MASK: u16 = 0x0004u16;  /* DAC_MUTE */
pub const WM8400_DAC_MUTE_SHIFT: u16 = 2u16;  /* DAC_MUTE */
pub const WM8400_DAC_MUTE_WIDTH: u16 = 1u16;  /* DAC_MUTE */
pub const WM8400_DACL_DATINV: u16 = 0x0002u16;  /* DACL_DATINV */
pub const WM8400_DACL_DATINV_MASK: u16 = 0x0002u16;  /* DACL_DATINV */
pub const WM8400_DACL_DATINV_SHIFT: u16 = 1u16;  /* DACL_DATINV */
pub const WM8400_DACL_DATINV_WIDTH: u16 = 1u16;  /* DACL_DATINV */
pub const WM8400_DACR_DATINV: u16 = 0x0001u16;  /* DACR_DATINV */
pub const WM8400_DACR_DATINV_MASK: u16 = 0x0001u16;  /* DACR_DATINV */
pub const WM8400_DACR_DATINV_SHIFT: u16 = 0u16;  /* DACR_DATINV */
pub const WM8400_DACR_DATINV_WIDTH: u16 = 1u16;  /* DACR_DATINV */

/*
 * R12 (0x0C) - Left DAC Digital Volume
 */
pub const WM8400_DAC_VU: u16 = 0x0100u16;  /* DAC_VU */
pub const WM8400_DAC_VU_MASK: u16 = 0x0100u16;  /* DAC_VU */
pub const WM8400_DAC_VU_SHIFT: u16 = 8u16;  /* DAC_VU */
pub const WM8400_DAC_VU_WIDTH: u16 = 1u16;  /* DAC_VU */
pub const WM8400_DACL_VOL_MASK: u16 = 0x00FFu16;  /* DACL_VOL - [7:0] */
pub const WM8400_DACL_VOL_SHIFT: u16 = 0u16;  /* DACL_VOL - [7:0] */
pub const WM8400_DACL_VOL_WIDTH: u16 = 8u16;  /* DACL_VOL - [7:0] */

/*
 * R13 (0x0D) - Right DAC Digital Volume
 */
// Duplicate C macro retained: #define WM8400_DAC_VU                           0x0100  /* DAC_VU */
// Duplicate C macro retained: #define WM8400_DAC_VU_MASK                      0x0100  /* DAC_VU */
// Duplicate C macro retained: #define WM8400_DAC_VU_SHIFT                          8  /* DAC_VU */
// Duplicate C macro retained: #define WM8400_DAC_VU_WIDTH                          1  /* DAC_VU */
pub const WM8400_DACR_VOL_MASK: u16 = 0x00FFu16;  /* DACR_VOL - [7:0] */
pub const WM8400_DACR_VOL_SHIFT: u16 = 0u16;  /* DACR_VOL - [7:0] */
pub const WM8400_DACR_VOL_WIDTH: u16 = 8u16;  /* DACR_VOL - [7:0] */

/*
 * R14 (0x0E) - Digital Side Tone
 */
pub const WM8400_ADCL_DAC_SVOL_MASK: u16 = 0x1E00u16;  /*   ADCL_DAC_SVOL - [12:9] */
pub const WM8400_ADCL_DAC_SVOL_SHIFT: u16 = 9u16;  /*   ADCL_DAC_SVOL - [12:9] */
pub const WM8400_ADCL_DAC_SVOL_WIDTH: u16 = 4u16;  /*   ADCL_DAC_SVOL - [12:9] */
pub const WM8400_ADCR_DAC_SVOL_MASK: u16 = 0x01E0u16;  /* ADCR_DAC_SVOL - [8:5] */
pub const WM8400_ADCR_DAC_SVOL_SHIFT: u16 = 5u16;  /* ADCR_DAC_SVOL - [8:5] */
pub const WM8400_ADCR_DAC_SVOL_WIDTH: u16 = 4u16;  /* ADCR_DAC_SVOL - [8:5] */
pub const WM8400_ADC_TO_DACL_MASK: u16 = 0x000Cu16;  /* ADC_TO_DACL - [3:2] */
pub const WM8400_ADC_TO_DACL_SHIFT: u16 = 2u16;  /* ADC_TO_DACL - [3:2] */
pub const WM8400_ADC_TO_DACL_WIDTH: u16 = 2u16;  /* ADC_TO_DACL - [3:2] */
pub const WM8400_ADC_TO_DACR_MASK: u16 = 0x0003u16;  /* ADC_TO_DACR - [1:0] */
pub const WM8400_ADC_TO_DACR_SHIFT: u16 = 0u16;  /* ADC_TO_DACR - [1:0] */
pub const WM8400_ADC_TO_DACR_WIDTH: u16 = 2u16;  /* ADC_TO_DACR - [1:0] */

/*
 * R15 (0x0F) - ADC CTRL
 */
pub const WM8400_ADC_HPF_ENA: u16 = 0x0100u16;  /* ADC_HPF_ENA */
pub const WM8400_ADC_HPF_ENA_MASK: u16 = 0x0100u16;  /* ADC_HPF_ENA */
pub const WM8400_ADC_HPF_ENA_SHIFT: u16 = 8u16;  /* ADC_HPF_ENA */
pub const WM8400_ADC_HPF_ENA_WIDTH: u16 = 1u16;  /* ADC_HPF_ENA */
pub const WM8400_ADC_HPF_CUT_MASK: u16 = 0x0060u16;  /* ADC_HPF_CUT - [6:5] */
pub const WM8400_ADC_HPF_CUT_SHIFT: u16 = 5u16;  /* ADC_HPF_CUT - [6:5] */
pub const WM8400_ADC_HPF_CUT_WIDTH: u16 = 2u16;  /* ADC_HPF_CUT - [6:5] */
pub const WM8400_ADCL_DATINV: u16 = 0x0002u16;  /* ADCL_DATINV */
pub const WM8400_ADCL_DATINV_MASK: u16 = 0x0002u16;  /* ADCL_DATINV */
pub const WM8400_ADCL_DATINV_SHIFT: u16 = 1u16;  /* ADCL_DATINV */
pub const WM8400_ADCL_DATINV_WIDTH: u16 = 1u16;  /* ADCL_DATINV */
pub const WM8400_ADCR_DATINV: u16 = 0x0001u16;  /* ADCR_DATINV */
pub const WM8400_ADCR_DATINV_MASK: u16 = 0x0001u16;  /* ADCR_DATINV */
pub const WM8400_ADCR_DATINV_SHIFT: u16 = 0u16;  /* ADCR_DATINV */
pub const WM8400_ADCR_DATINV_WIDTH: u16 = 1u16;  /* ADCR_DATINV */

/*
 * R16 (0x10) - Left ADC Digital Volume
 */
pub const WM8400_ADC_VU: u16 = 0x0100u16;  /* ADC_VU */
pub const WM8400_ADC_VU_MASK: u16 = 0x0100u16;  /* ADC_VU */
pub const WM8400_ADC_VU_SHIFT: u16 = 8u16;  /* ADC_VU */
pub const WM8400_ADC_VU_WIDTH: u16 = 1u16;  /* ADC_VU */
pub const WM8400_ADCL_VOL_MASK: u16 = 0x00FFu16;  /* ADCL_VOL - [7:0] */
pub const WM8400_ADCL_VOL_SHIFT: u16 = 0u16;  /* ADCL_VOL - [7:0] */
pub const WM8400_ADCL_VOL_WIDTH: u16 = 8u16;  /* ADCL_VOL - [7:0] */

/*
 * R17 (0x11) - Right ADC Digital Volume
 */
// Duplicate C macro retained: #define WM8400_ADC_VU                           0x0100  /* ADC_VU */
// Duplicate C macro retained: #define WM8400_ADC_VU_MASK                      0x0100  /* ADC_VU */
// Duplicate C macro retained: #define WM8400_ADC_VU_SHIFT                          8  /* ADC_VU */
// Duplicate C macro retained: #define WM8400_ADC_VU_WIDTH                          1  /* ADC_VU */
pub const WM8400_ADCR_VOL_MASK: u16 = 0x00FFu16;  /* ADCR_VOL - [7:0] */
pub const WM8400_ADCR_VOL_SHIFT: u16 = 0u16;  /* ADCR_VOL - [7:0] */
pub const WM8400_ADCR_VOL_WIDTH: u16 = 8u16;  /* ADCR_VOL - [7:0] */

/*
 * R24 (0x18) - Left Line Input 1&2 Volume
 */
pub const WM8400_IPVU: u16 = 0x0100u16;  /* IPVU */
pub const WM8400_IPVU_MASK: u16 = 0x0100u16;  /* IPVU */
pub const WM8400_IPVU_SHIFT: u16 = 8u16;  /* IPVU */
pub const WM8400_IPVU_WIDTH: u16 = 1u16;  /* IPVU */
pub const WM8400_LI12MUTE: u16 = 0x0080u16;  /* LI12MUTE */
pub const WM8400_LI12MUTE_MASK: u16 = 0x0080u16;  /* LI12MUTE */
pub const WM8400_LI12MUTE_SHIFT: u16 = 7u16;  /* LI12MUTE */
pub const WM8400_LI12MUTE_WIDTH: u16 = 1u16;  /* LI12MUTE */
pub const WM8400_LI12ZC: u16 = 0x0040u16;  /* LI12ZC */
pub const WM8400_LI12ZC_MASK: u16 = 0x0040u16;  /* LI12ZC */
pub const WM8400_LI12ZC_SHIFT: u16 = 6u16;  /* LI12ZC */
pub const WM8400_LI12ZC_WIDTH: u16 = 1u16;  /* LI12ZC */
pub const WM8400_LIN12VOL_MASK: u16 = 0x001Fu16;  /* LIN12VOL - [4:0] */
pub const WM8400_LIN12VOL_SHIFT: u16 = 0u16;  /* LIN12VOL - [4:0] */
pub const WM8400_LIN12VOL_WIDTH: u16 = 5u16;  /* LIN12VOL - [4:0] */

/*
 * R25 (0x19) - Left Line Input 3&4 Volume
 */
// Duplicate C macro retained: #define WM8400_IPVU                             0x0100  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_MASK                        0x0100  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_SHIFT                            8  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_WIDTH                            1  /* IPVU */
pub const WM8400_LI34MUTE: u16 = 0x0080u16;  /* LI34MUTE */
pub const WM8400_LI34MUTE_MASK: u16 = 0x0080u16;  /* LI34MUTE */
pub const WM8400_LI34MUTE_SHIFT: u16 = 7u16;  /* LI34MUTE */
pub const WM8400_LI34MUTE_WIDTH: u16 = 1u16;  /* LI34MUTE */
pub const WM8400_LI34ZC: u16 = 0x0040u16;  /* LI34ZC */
pub const WM8400_LI34ZC_MASK: u16 = 0x0040u16;  /* LI34ZC */
pub const WM8400_LI34ZC_SHIFT: u16 = 6u16;  /* LI34ZC */
pub const WM8400_LI34ZC_WIDTH: u16 = 1u16;  /* LI34ZC */
pub const WM8400_LIN34VOL_MASK: u16 = 0x001Fu16;  /* LIN34VOL - [4:0] */
pub const WM8400_LIN34VOL_SHIFT: u16 = 0u16;  /* LIN34VOL - [4:0] */
pub const WM8400_LIN34VOL_WIDTH: u16 = 5u16;  /* LIN34VOL - [4:0] */

/*
 * R26 (0x1A) - Right Line Input 1&2 Volume
 */
// Duplicate C macro retained: #define WM8400_IPVU                             0x0100  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_MASK                        0x0100  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_SHIFT                            8  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_WIDTH                            1  /* IPVU */
pub const WM8400_RI12MUTE: u16 = 0x0080u16;  /* RI12MUTE */
pub const WM8400_RI12MUTE_MASK: u16 = 0x0080u16;  /* RI12MUTE */
pub const WM8400_RI12MUTE_SHIFT: u16 = 7u16;  /* RI12MUTE */
pub const WM8400_RI12MUTE_WIDTH: u16 = 1u16;  /* RI12MUTE */
pub const WM8400_RI12ZC: u16 = 0x0040u16;  /* RI12ZC */
pub const WM8400_RI12ZC_MASK: u16 = 0x0040u16;  /* RI12ZC */
pub const WM8400_RI12ZC_SHIFT: u16 = 6u16;  /* RI12ZC */
pub const WM8400_RI12ZC_WIDTH: u16 = 1u16;  /* RI12ZC */
pub const WM8400_RIN12VOL_MASK: u16 = 0x001Fu16;  /* RIN12VOL - [4:0] */
pub const WM8400_RIN12VOL_SHIFT: u16 = 0u16;  /* RIN12VOL - [4:0] */
pub const WM8400_RIN12VOL_WIDTH: u16 = 5u16;  /* RIN12VOL - [4:0] */

/*
 * R27 (0x1B) - Right Line Input 3&4 Volume
 */
// Duplicate C macro retained: #define WM8400_IPVU                             0x0100  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_MASK                        0x0100  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_SHIFT                            8  /* IPVU */
// Duplicate C macro retained: #define WM8400_IPVU_WIDTH                            1  /* IPVU */
pub const WM8400_RI34MUTE: u16 = 0x0080u16;  /* RI34MUTE */
pub const WM8400_RI34MUTE_MASK: u16 = 0x0080u16;  /* RI34MUTE */
pub const WM8400_RI34MUTE_SHIFT: u16 = 7u16;  /* RI34MUTE */
pub const WM8400_RI34MUTE_WIDTH: u16 = 1u16;  /* RI34MUTE */
pub const WM8400_RI34ZC: u16 = 0x0040u16;  /* RI34ZC */
pub const WM8400_RI34ZC_MASK: u16 = 0x0040u16;  /* RI34ZC */
pub const WM8400_RI34ZC_SHIFT: u16 = 6u16;  /* RI34ZC */
pub const WM8400_RI34ZC_WIDTH: u16 = 1u16;  /* RI34ZC */
pub const WM8400_RIN34VOL_MASK: u16 = 0x001Fu16;  /* RIN34VOL - [4:0] */
pub const WM8400_RIN34VOL_SHIFT: u16 = 0u16;  /* RIN34VOL - [4:0] */
pub const WM8400_RIN34VOL_WIDTH: u16 = 5u16;  /* RIN34VOL - [4:0] */

/*
 * R28 (0x1C) - Left Output Volume
 */
pub const WM8400_OPVU: u16 = 0x0100u16;  /* OPVU */
pub const WM8400_OPVU_MASK: u16 = 0x0100u16;  /* OPVU */
pub const WM8400_OPVU_SHIFT: u16 = 8u16;  /* OPVU */
pub const WM8400_OPVU_WIDTH: u16 = 1u16;  /* OPVU */
pub const WM8400_LOZC: u16 = 0x0080u16;  /* LOZC */
pub const WM8400_LOZC_MASK: u16 = 0x0080u16;  /* LOZC */
pub const WM8400_LOZC_SHIFT: u16 = 7u16;  /* LOZC */
pub const WM8400_LOZC_WIDTH: u16 = 1u16;  /* LOZC */
pub const WM8400_LOUTVOL_MASK: u16 = 0x007Fu16;  /* LOUTVOL - [6:0] */
pub const WM8400_LOUTVOL_SHIFT: u16 = 0u16;  /* LOUTVOL - [6:0] */
pub const WM8400_LOUTVOL_WIDTH: u16 = 7u16;  /* LOUTVOL - [6:0] */

/*
 * R29 (0x1D) - Right Output Volume
 */
// Duplicate C macro retained: #define WM8400_OPVU                             0x0100  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_MASK                        0x0100  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_SHIFT                            8  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_WIDTH                            1  /* OPVU */
pub const WM8400_ROZC: u16 = 0x0080u16;  /* ROZC */
pub const WM8400_ROZC_MASK: u16 = 0x0080u16;  /* ROZC */
pub const WM8400_ROZC_SHIFT: u16 = 7u16;  /* ROZC */
pub const WM8400_ROZC_WIDTH: u16 = 1u16;  /* ROZC */
pub const WM8400_ROUTVOL_MASK: u16 = 0x007Fu16;  /* ROUTVOL - [6:0] */
pub const WM8400_ROUTVOL_SHIFT: u16 = 0u16;  /* ROUTVOL - [6:0] */
pub const WM8400_ROUTVOL_WIDTH: u16 = 7u16;  /* ROUTVOL - [6:0] */

/*
 * R30 (0x1E) - Line Outputs Volume
 */
pub const WM8400_LONMUTE: u16 = 0x0040u16;  /* LONMUTE */
pub const WM8400_LONMUTE_MASK: u16 = 0x0040u16;  /* LONMUTE */
pub const WM8400_LONMUTE_SHIFT: u16 = 6u16;  /* LONMUTE */
pub const WM8400_LONMUTE_WIDTH: u16 = 1u16;  /* LONMUTE */
pub const WM8400_LOPMUTE: u16 = 0x0020u16;  /* LOPMUTE */
pub const WM8400_LOPMUTE_MASK: u16 = 0x0020u16;  /* LOPMUTE */
pub const WM8400_LOPMUTE_SHIFT: u16 = 5u16;  /* LOPMUTE */
pub const WM8400_LOPMUTE_WIDTH: u16 = 1u16;  /* LOPMUTE */
pub const WM8400_LOATTN: u16 = 0x0010u16;  /* LOATTN */
pub const WM8400_LOATTN_MASK: u16 = 0x0010u16;  /* LOATTN */
pub const WM8400_LOATTN_SHIFT: u16 = 4u16;  /* LOATTN */
pub const WM8400_LOATTN_WIDTH: u16 = 1u16;  /* LOATTN */
pub const WM8400_RONMUTE: u16 = 0x0004u16;  /* RONMUTE */
pub const WM8400_RONMUTE_MASK: u16 = 0x0004u16;  /* RONMUTE */
pub const WM8400_RONMUTE_SHIFT: u16 = 2u16;  /* RONMUTE */
pub const WM8400_RONMUTE_WIDTH: u16 = 1u16;  /* RONMUTE */
pub const WM8400_ROPMUTE: u16 = 0x0002u16;  /* ROPMUTE */
pub const WM8400_ROPMUTE_MASK: u16 = 0x0002u16;  /* ROPMUTE */
pub const WM8400_ROPMUTE_SHIFT: u16 = 1u16;  /* ROPMUTE */
pub const WM8400_ROPMUTE_WIDTH: u16 = 1u16;  /* ROPMUTE */
pub const WM8400_ROATTN: u16 = 0x0001u16;  /* ROATTN */
pub const WM8400_ROATTN_MASK: u16 = 0x0001u16;  /* ROATTN */
pub const WM8400_ROATTN_SHIFT: u16 = 0u16;  /* ROATTN */
pub const WM8400_ROATTN_WIDTH: u16 = 1u16;  /* ROATTN */

/*
 * R31 (0x1F) - Out3/4 Volume
 */
pub const WM8400_OUT3MUTE: u16 = 0x0020u16;  /* OUT3MUTE */
pub const WM8400_OUT3MUTE_MASK: u16 = 0x0020u16;  /* OUT3MUTE */
pub const WM8400_OUT3MUTE_SHIFT: u16 = 5u16;  /* OUT3MUTE */
pub const WM8400_OUT3MUTE_WIDTH: u16 = 1u16;  /* OUT3MUTE */
pub const WM8400_OUT3ATTN: u16 = 0x0010u16;  /* OUT3ATTN */
pub const WM8400_OUT3ATTN_MASK: u16 = 0x0010u16;  /* OUT3ATTN */
pub const WM8400_OUT3ATTN_SHIFT: u16 = 4u16;  /* OUT3ATTN */
pub const WM8400_OUT3ATTN_WIDTH: u16 = 1u16;  /* OUT3ATTN */
pub const WM8400_OUT4MUTE: u16 = 0x0002u16;  /* OUT4MUTE */
pub const WM8400_OUT4MUTE_MASK: u16 = 0x0002u16;  /* OUT4MUTE */
pub const WM8400_OUT4MUTE_SHIFT: u16 = 1u16;  /* OUT4MUTE */
pub const WM8400_OUT4MUTE_WIDTH: u16 = 1u16;  /* OUT4MUTE */
pub const WM8400_OUT4ATTN: u16 = 0x0001u16;  /* OUT4ATTN */
pub const WM8400_OUT4ATTN_MASK: u16 = 0x0001u16;  /* OUT4ATTN */
pub const WM8400_OUT4ATTN_SHIFT: u16 = 0u16;  /* OUT4ATTN */
pub const WM8400_OUT4ATTN_WIDTH: u16 = 1u16;  /* OUT4ATTN */

/*
 * R32 (0x20) - Left OPGA Volume
 */
// Duplicate C macro retained: #define WM8400_OPVU                             0x0100  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_MASK                        0x0100  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_SHIFT                            8  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_WIDTH                            1  /* OPVU */
pub const WM8400_LOPGAZC: u16 = 0x0080u16;  /* LOPGAZC */
pub const WM8400_LOPGAZC_MASK: u16 = 0x0080u16;  /* LOPGAZC */
pub const WM8400_LOPGAZC_SHIFT: u16 = 7u16;  /* LOPGAZC */
pub const WM8400_LOPGAZC_WIDTH: u16 = 1u16;  /* LOPGAZC */
pub const WM8400_LOPGAVOL_MASK: u16 = 0x007Fu16;  /* LOPGAVOL - [6:0] */
pub const WM8400_LOPGAVOL_SHIFT: u16 = 0u16;  /* LOPGAVOL - [6:0] */
pub const WM8400_LOPGAVOL_WIDTH: u16 = 7u16;  /* LOPGAVOL - [6:0] */

/*
 * R33 (0x21) - Right OPGA Volume
 */
// Duplicate C macro retained: #define WM8400_OPVU                             0x0100  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_MASK                        0x0100  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_SHIFT                            8  /* OPVU */
// Duplicate C macro retained: #define WM8400_OPVU_WIDTH                            1  /* OPVU */
pub const WM8400_ROPGAZC: u16 = 0x0080u16;  /* ROPGAZC */
pub const WM8400_ROPGAZC_MASK: u16 = 0x0080u16;  /* ROPGAZC */
pub const WM8400_ROPGAZC_SHIFT: u16 = 7u16;  /* ROPGAZC */
pub const WM8400_ROPGAZC_WIDTH: u16 = 1u16;  /* ROPGAZC */
pub const WM8400_ROPGAVOL_MASK: u16 = 0x007Fu16;  /* ROPGAVOL - [6:0] */
pub const WM8400_ROPGAVOL_SHIFT: u16 = 0u16;  /* ROPGAVOL - [6:0] */
pub const WM8400_ROPGAVOL_WIDTH: u16 = 7u16;  /* ROPGAVOL - [6:0] */

/*
 * R34 (0x22) - Speaker Volume
 */
pub const WM8400_SPKATTN_MASK: u16 = 0x0003u16;  /* SPKATTN - [1:0] */
pub const WM8400_SPKATTN_SHIFT: u16 = 0u16;  /* SPKATTN - [1:0] */
pub const WM8400_SPKATTN_WIDTH: u16 = 2u16;  /* SPKATTN - [1:0] */

/*
 * R35 (0x23) - ClassD1
 */
pub const WM8400_CDMODE: u16 = 0x0100u16;  /* CDMODE */
pub const WM8400_CDMODE_MASK: u16 = 0x0100u16;  /* CDMODE */
pub const WM8400_CDMODE_SHIFT: u16 = 8u16;  /* CDMODE */
pub const WM8400_CDMODE_WIDTH: u16 = 1u16;  /* CDMODE */
pub const WM8400_CLASSD_CLK_SEL: u16 = 0x0080u16;  /* CLASSD_CLK_SEL */
pub const WM8400_CLASSD_CLK_SEL_MASK: u16 = 0x0080u16;  /* CLASSD_CLK_SEL */
pub const WM8400_CLASSD_CLK_SEL_SHIFT: u16 = 7u16;  /* CLASSD_CLK_SEL */
pub const WM8400_CLASSD_CLK_SEL_WIDTH: u16 = 1u16;  /* CLASSD_CLK_SEL */
pub const WM8400_CD_SRCTRL: u16 = 0x0040u16;  /* CD_SRCTRL */
pub const WM8400_CD_SRCTRL_MASK: u16 = 0x0040u16;  /* CD_SRCTRL */
pub const WM8400_CD_SRCTRL_SHIFT: u16 = 6u16;  /* CD_SRCTRL */
pub const WM8400_CD_SRCTRL_WIDTH: u16 = 1u16;  /* CD_SRCTRL */
pub const WM8400_SPKNOPOP: u16 = 0x0020u16;  /* SPKNOPOP */
pub const WM8400_SPKNOPOP_MASK: u16 = 0x0020u16;  /* SPKNOPOP */
pub const WM8400_SPKNOPOP_SHIFT: u16 = 5u16;  /* SPKNOPOP */
pub const WM8400_SPKNOPOP_WIDTH: u16 = 1u16;  /* SPKNOPOP */
pub const WM8400_DBLERATE: u16 = 0x0010u16;  /* DBLERATE */
pub const WM8400_DBLERATE_MASK: u16 = 0x0010u16;  /* DBLERATE */
pub const WM8400_DBLERATE_SHIFT: u16 = 4u16;  /* DBLERATE */
pub const WM8400_DBLERATE_WIDTH: u16 = 1u16;  /* DBLERATE */
pub const WM8400_LOOPTEST: u16 = 0x0008u16;  /* LOOPTEST */
pub const WM8400_LOOPTEST_MASK: u16 = 0x0008u16;  /* LOOPTEST */
pub const WM8400_LOOPTEST_SHIFT: u16 = 3u16;  /* LOOPTEST */
pub const WM8400_LOOPTEST_WIDTH: u16 = 1u16;  /* LOOPTEST */
pub const WM8400_HALFABBIAS: u16 = 0x0004u16;  /* HALFABBIAS */
pub const WM8400_HALFABBIAS_MASK: u16 = 0x0004u16;  /* HALFABBIAS */
pub const WM8400_HALFABBIAS_SHIFT: u16 = 2u16;  /* HALFABBIAS */
pub const WM8400_HALFABBIAS_WIDTH: u16 = 1u16;  /* HALFABBIAS */
pub const WM8400_TRIDEL_MASK: u16 = 0x0003u16;  /* TRIDEL - [1:0] */
pub const WM8400_TRIDEL_SHIFT: u16 = 0u16;  /* TRIDEL - [1:0] */
pub const WM8400_TRIDEL_WIDTH: u16 = 2u16;  /* TRIDEL - [1:0] */

/*
 * R37 (0x25) - ClassD3
 */
pub const WM8400_DCGAIN_MASK: u16 = 0x0038u16;  /* DCGAIN - [5:3] */
pub const WM8400_DCGAIN_SHIFT: u16 = 3u16;  /* DCGAIN - [5:3] */
pub const WM8400_DCGAIN_WIDTH: u16 = 3u16;  /* DCGAIN - [5:3] */
pub const WM8400_ACGAIN_MASK: u16 = 0x0007u16;  /* ACGAIN - [2:0] */
pub const WM8400_ACGAIN_SHIFT: u16 = 0u16;  /* ACGAIN - [2:0] */
pub const WM8400_ACGAIN_WIDTH: u16 = 3u16;  /* ACGAIN - [2:0] */

/*
 * R39 (0x27) - Input Mixer1
 */
pub const WM8400_AINLMODE_MASK: u16 = 0x000Cu16;  /* AINLMODE - [3:2] */
pub const WM8400_AINLMODE_SHIFT: u16 = 2u16;  /* AINLMODE - [3:2] */
pub const WM8400_AINLMODE_WIDTH: u16 = 2u16;  /* AINLMODE - [3:2] */
pub const WM8400_AINRMODE_MASK: u16 = 0x0003u16;  /* AINRMODE - [1:0] */
pub const WM8400_AINRMODE_SHIFT: u16 = 0u16;  /* AINRMODE - [1:0] */
pub const WM8400_AINRMODE_WIDTH: u16 = 2u16;  /* AINRMODE - [1:0] */

/*
 * R40 (0x28) - Input Mixer2
 */
pub const WM8400_LMP4: u16 = 0x0080u16;  /* LMP4 */
pub const WM8400_LMP4_MASK: u16 = 0x0080u16;  /* LMP4 */
pub const WM8400_LMP4_SHIFT: u16 = 7u16;  /* LMP4 */
pub const WM8400_LMP4_WIDTH: u16 = 1u16;  /* LMP4 */
pub const WM8400_LMN3: u16 = 0x0040u16;  /* LMN3 */
pub const WM8400_LMN3_MASK: u16 = 0x0040u16;  /* LMN3 */
pub const WM8400_LMN3_SHIFT: u16 = 6u16;  /* LMN3 */
pub const WM8400_LMN3_WIDTH: u16 = 1u16;  /* LMN3 */
pub const WM8400_LMP2: u16 = 0x0020u16;  /* LMP2 */
pub const WM8400_LMP2_MASK: u16 = 0x0020u16;  /* LMP2 */
pub const WM8400_LMP2_SHIFT: u16 = 5u16;  /* LMP2 */
pub const WM8400_LMP2_WIDTH: u16 = 1u16;  /* LMP2 */
pub const WM8400_LMN1: u16 = 0x0010u16;  /* LMN1 */
pub const WM8400_LMN1_MASK: u16 = 0x0010u16;  /* LMN1 */
pub const WM8400_LMN1_SHIFT: u16 = 4u16;  /* LMN1 */
pub const WM8400_LMN1_WIDTH: u16 = 1u16;  /* LMN1 */
pub const WM8400_RMP4: u16 = 0x0008u16;  /* RMP4 */
pub const WM8400_RMP4_MASK: u16 = 0x0008u16;  /* RMP4 */
pub const WM8400_RMP4_SHIFT: u16 = 3u16;  /* RMP4 */
pub const WM8400_RMP4_WIDTH: u16 = 1u16;  /* RMP4 */
pub const WM8400_RMN3: u16 = 0x0004u16;  /* RMN3 */
pub const WM8400_RMN3_MASK: u16 = 0x0004u16;  /* RMN3 */
pub const WM8400_RMN3_SHIFT: u16 = 2u16;  /* RMN3 */
pub const WM8400_RMN3_WIDTH: u16 = 1u16;  /* RMN3 */
pub const WM8400_RMP2: u16 = 0x0002u16;  /* RMP2 */
pub const WM8400_RMP2_MASK: u16 = 0x0002u16;  /* RMP2 */
pub const WM8400_RMP2_SHIFT: u16 = 1u16;  /* RMP2 */
pub const WM8400_RMP2_WIDTH: u16 = 1u16;  /* RMP2 */
pub const WM8400_RMN1: u16 = 0x0001u16;  /* RMN1 */
pub const WM8400_RMN1_MASK: u16 = 0x0001u16;  /* RMN1 */
pub const WM8400_RMN1_SHIFT: u16 = 0u16;  /* RMN1 */
pub const WM8400_RMN1_WIDTH: u16 = 1u16;  /* RMN1 */

/*
 * R41 (0x29) - Input Mixer3
 */
pub const WM8400_L34MNB: u16 = 0x0100u16;  /* L34MNB */
pub const WM8400_L34MNB_MASK: u16 = 0x0100u16;  /* L34MNB */
pub const WM8400_L34MNB_SHIFT: u16 = 8u16;  /* L34MNB */
pub const WM8400_L34MNB_WIDTH: u16 = 1u16;  /* L34MNB */
pub const WM8400_L34MNBST: u16 = 0x0080u16;  /* L34MNBST */
pub const WM8400_L34MNBST_MASK: u16 = 0x0080u16;  /* L34MNBST */
pub const WM8400_L34MNBST_SHIFT: u16 = 7u16;  /* L34MNBST */
pub const WM8400_L34MNBST_WIDTH: u16 = 1u16;  /* L34MNBST */
pub const WM8400_L12MNB: u16 = 0x0020u16;  /* L12MNB */
pub const WM8400_L12MNB_MASK: u16 = 0x0020u16;  /* L12MNB */
pub const WM8400_L12MNB_SHIFT: u16 = 5u16;  /* L12MNB */
pub const WM8400_L12MNB_WIDTH: u16 = 1u16;  /* L12MNB */
pub const WM8400_L12MNBST: u16 = 0x0010u16;  /* L12MNBST */
pub const WM8400_L12MNBST_MASK: u16 = 0x0010u16;  /* L12MNBST */
pub const WM8400_L12MNBST_SHIFT: u16 = 4u16;  /* L12MNBST */
pub const WM8400_L12MNBST_WIDTH: u16 = 1u16;  /* L12MNBST */
pub const WM8400_LDBVOL_MASK: u16 = 0x0007u16;  /* LDBVOL - [2:0] */
pub const WM8400_LDBVOL_SHIFT: u16 = 0u16;  /* LDBVOL - [2:0] */
pub const WM8400_LDBVOL_WIDTH: u16 = 3u16;  /* LDBVOL - [2:0] */

/*
 * R42 (0x2A) - Input Mixer4
 */
pub const WM8400_R34MNB: u16 = 0x0100u16;  /* R34MNB */
pub const WM8400_R34MNB_MASK: u16 = 0x0100u16;  /* R34MNB */
pub const WM8400_R34MNB_SHIFT: u16 = 8u16;  /* R34MNB */
pub const WM8400_R34MNB_WIDTH: u16 = 1u16;  /* R34MNB */
pub const WM8400_R34MNBST: u16 = 0x0080u16;  /* R34MNBST */
pub const WM8400_R34MNBST_MASK: u16 = 0x0080u16;  /* R34MNBST */
pub const WM8400_R34MNBST_SHIFT: u16 = 7u16;  /* R34MNBST */
pub const WM8400_R34MNBST_WIDTH: u16 = 1u16;  /* R34MNBST */
pub const WM8400_R12MNB: u16 = 0x0020u16;  /* R12MNB */
pub const WM8400_R12MNB_MASK: u16 = 0x0020u16;  /* R12MNB */
pub const WM8400_R12MNB_SHIFT: u16 = 5u16;  /* R12MNB */
pub const WM8400_R12MNB_WIDTH: u16 = 1u16;  /* R12MNB */
pub const WM8400_R12MNBST: u16 = 0x0010u16;  /* R12MNBST */
pub const WM8400_R12MNBST_MASK: u16 = 0x0010u16;  /* R12MNBST */
pub const WM8400_R12MNBST_SHIFT: u16 = 4u16;  /* R12MNBST */
pub const WM8400_R12MNBST_WIDTH: u16 = 1u16;  /* R12MNBST */
pub const WM8400_RDBVOL_MASK: u16 = 0x0007u16;  /* RDBVOL - [2:0] */
pub const WM8400_RDBVOL_SHIFT: u16 = 0u16;  /* RDBVOL - [2:0] */
pub const WM8400_RDBVOL_WIDTH: u16 = 3u16;  /* RDBVOL - [2:0] */

/*
 * R43 (0x2B) - Input Mixer5
 */
pub const WM8400_LI2BVOL_MASK: u16 = 0x01C0u16;  /* LI2BVOL - [8:6] */
pub const WM8400_LI2BVOL_SHIFT: u16 = 6u16;  /* LI2BVOL - [8:6] */
pub const WM8400_LI2BVOL_WIDTH: u16 = 3u16;  /* LI2BVOL - [8:6] */
pub const WM8400_LR4BVOL_MASK: u16 = 0x0038u16;  /* LR4BVOL - [5:3] */
pub const WM8400_LR4BVOL_SHIFT: u16 = 3u16;  /* LR4BVOL - [5:3] */
pub const WM8400_LR4BVOL_WIDTH: u16 = 3u16;  /* LR4BVOL - [5:3] */
pub const WM8400_LL4BVOL_MASK: u16 = 0x0007u16;  /* LL4BVOL - [2:0] */
pub const WM8400_LL4BVOL_SHIFT: u16 = 0u16;  /* LL4BVOL - [2:0] */
pub const WM8400_LL4BVOL_WIDTH: u16 = 3u16;  /* LL4BVOL - [2:0] */

/*
 * R44 (0x2C) - Input Mixer6
 */
pub const WM8400_RI2BVOL_MASK: u16 = 0x01C0u16;  /* RI2BVOL - [8:6] */
pub const WM8400_RI2BVOL_SHIFT: u16 = 6u16;  /* RI2BVOL - [8:6] */
pub const WM8400_RI2BVOL_WIDTH: u16 = 3u16;  /* RI2BVOL - [8:6] */
pub const WM8400_RL4BVOL_MASK: u16 = 0x0038u16;  /* RL4BVOL - [5:3] */
pub const WM8400_RL4BVOL_SHIFT: u16 = 3u16;  /* RL4BVOL - [5:3] */
pub const WM8400_RL4BVOL_WIDTH: u16 = 3u16;  /* RL4BVOL - [5:3] */
pub const WM8400_RR4BVOL_MASK: u16 = 0x0007u16;  /* RR4BVOL - [2:0] */
pub const WM8400_RR4BVOL_SHIFT: u16 = 0u16;  /* RR4BVOL - [2:0] */
pub const WM8400_RR4BVOL_WIDTH: u16 = 3u16;  /* RR4BVOL - [2:0] */

/*
 * R45 (0x2D) - Output Mixer1
 */
pub const WM8400_LRBLO: u16 = 0x0080u16;  /* LRBLO */
pub const WM8400_LRBLO_MASK: u16 = 0x0080u16;  /* LRBLO */
pub const WM8400_LRBLO_SHIFT: u16 = 7u16;  /* LRBLO */
pub const WM8400_LRBLO_WIDTH: u16 = 1u16;  /* LRBLO */
pub const WM8400_LLBLO: u16 = 0x0040u16;  /* LLBLO */
pub const WM8400_LLBLO_MASK: u16 = 0x0040u16;  /* LLBLO */
pub const WM8400_LLBLO_SHIFT: u16 = 6u16;  /* LLBLO */
pub const WM8400_LLBLO_WIDTH: u16 = 1u16;  /* LLBLO */
pub const WM8400_LRI3LO: u16 = 0x0020u16;  /* LRI3LO */
pub const WM8400_LRI3LO_MASK: u16 = 0x0020u16;  /* LRI3LO */
pub const WM8400_LRI3LO_SHIFT: u16 = 5u16;  /* LRI3LO */
pub const WM8400_LRI3LO_WIDTH: u16 = 1u16;  /* LRI3LO */
pub const WM8400_LLI3LO: u16 = 0x0010u16;  /* LLI3LO */
pub const WM8400_LLI3LO_MASK: u16 = 0x0010u16;  /* LLI3LO */
pub const WM8400_LLI3LO_SHIFT: u16 = 4u16;  /* LLI3LO */
pub const WM8400_LLI3LO_WIDTH: u16 = 1u16;  /* LLI3LO */
pub const WM8400_LR12LO: u16 = 0x0008u16;  /* LR12LO */
pub const WM8400_LR12LO_MASK: u16 = 0x0008u16;  /* LR12LO */
pub const WM8400_LR12LO_SHIFT: u16 = 3u16;  /* LR12LO */
pub const WM8400_LR12LO_WIDTH: u16 = 1u16;  /* LR12LO */
pub const WM8400_LL12LO: u16 = 0x0004u16;  /* LL12LO */
pub const WM8400_LL12LO_MASK: u16 = 0x0004u16;  /* LL12LO */
pub const WM8400_LL12LO_SHIFT: u16 = 2u16;  /* LL12LO */
pub const WM8400_LL12LO_WIDTH: u16 = 1u16;  /* LL12LO */
pub const WM8400_LDLO: u16 = 0x0001u16;  /* LDLO */
pub const WM8400_LDLO_MASK: u16 = 0x0001u16;  /* LDLO */
pub const WM8400_LDLO_SHIFT: u16 = 0u16;  /* LDLO */
pub const WM8400_LDLO_WIDTH: u16 = 1u16;  /* LDLO */

/*
 * R46 (0x2E) - Output Mixer2
 */
pub const WM8400_RLBRO: u16 = 0x0080u16;  /* RLBRO */
pub const WM8400_RLBRO_MASK: u16 = 0x0080u16;  /* RLBRO */
pub const WM8400_RLBRO_SHIFT: u16 = 7u16;  /* RLBRO */
pub const WM8400_RLBRO_WIDTH: u16 = 1u16;  /* RLBRO */
pub const WM8400_RRBRO: u16 = 0x0040u16;  /* RRBRO */
pub const WM8400_RRBRO_MASK: u16 = 0x0040u16;  /* RRBRO */
pub const WM8400_RRBRO_SHIFT: u16 = 6u16;  /* RRBRO */
pub const WM8400_RRBRO_WIDTH: u16 = 1u16;  /* RRBRO */
pub const WM8400_RLI3RO: u16 = 0x0020u16;  /* RLI3RO */
pub const WM8400_RLI3RO_MASK: u16 = 0x0020u16;  /* RLI3RO */
pub const WM8400_RLI3RO_SHIFT: u16 = 5u16;  /* RLI3RO */
pub const WM8400_RLI3RO_WIDTH: u16 = 1u16;  /* RLI3RO */
pub const WM8400_RRI3RO: u16 = 0x0010u16;  /* RRI3RO */
pub const WM8400_RRI3RO_MASK: u16 = 0x0010u16;  /* RRI3RO */
pub const WM8400_RRI3RO_SHIFT: u16 = 4u16;  /* RRI3RO */
pub const WM8400_RRI3RO_WIDTH: u16 = 1u16;  /* RRI3RO */
pub const WM8400_RL12RO: u16 = 0x0008u16;  /* RL12RO */
pub const WM8400_RL12RO_MASK: u16 = 0x0008u16;  /* RL12RO */
pub const WM8400_RL12RO_SHIFT: u16 = 3u16;  /* RL12RO */
pub const WM8400_RL12RO_WIDTH: u16 = 1u16;  /* RL12RO */
pub const WM8400_RR12RO: u16 = 0x0004u16;  /* RR12RO */
pub const WM8400_RR12RO_MASK: u16 = 0x0004u16;  /* RR12RO */
pub const WM8400_RR12RO_SHIFT: u16 = 2u16;  /* RR12RO */
pub const WM8400_RR12RO_WIDTH: u16 = 1u16;  /* RR12RO */
pub const WM8400_RDRO: u16 = 0x0001u16;  /* RDRO */
pub const WM8400_RDRO_MASK: u16 = 0x0001u16;  /* RDRO */
pub const WM8400_RDRO_SHIFT: u16 = 0u16;  /* RDRO */
pub const WM8400_RDRO_WIDTH: u16 = 1u16;  /* RDRO */

/*
 * R47 (0x2F) - Output Mixer3
 */
pub const WM8400_LLI3LOVOL_MASK: u16 = 0x01C0u16;  /* LLI3LOVOL - [8:6] */
pub const WM8400_LLI3LOVOL_SHIFT: u16 = 6u16;  /* LLI3LOVOL - [8:6] */
pub const WM8400_LLI3LOVOL_WIDTH: u16 = 3u16;  /* LLI3LOVOL - [8:6] */
pub const WM8400_LR12LOVOL_MASK: u16 = 0x0038u16;  /* LR12LOVOL - [5:3] */
pub const WM8400_LR12LOVOL_SHIFT: u16 = 3u16;  /* LR12LOVOL - [5:3] */
pub const WM8400_LR12LOVOL_WIDTH: u16 = 3u16;  /* LR12LOVOL - [5:3] */
pub const WM8400_LL12LOVOL_MASK: u16 = 0x0007u16;  /* LL12LOVOL - [2:0] */
pub const WM8400_LL12LOVOL_SHIFT: u16 = 0u16;  /* LL12LOVOL - [2:0] */
pub const WM8400_LL12LOVOL_WIDTH: u16 = 3u16;  /* LL12LOVOL - [2:0] */

/*
 * R48 (0x30) - Output Mixer4
 */
pub const WM8400_RRI3ROVOL_MASK: u16 = 0x01C0u16;  /* RRI3ROVOL - [8:6] */
pub const WM8400_RRI3ROVOL_SHIFT: u16 = 6u16;  /* RRI3ROVOL - [8:6] */
pub const WM8400_RRI3ROVOL_WIDTH: u16 = 3u16;  /* RRI3ROVOL - [8:6] */
pub const WM8400_RL12ROVOL_MASK: u16 = 0x0038u16;  /* RL12ROVOL - [5:3] */
pub const WM8400_RL12ROVOL_SHIFT: u16 = 3u16;  /* RL12ROVOL - [5:3] */
pub const WM8400_RL12ROVOL_WIDTH: u16 = 3u16;  /* RL12ROVOL - [5:3] */
pub const WM8400_RR12ROVOL_MASK: u16 = 0x0007u16;  /* RR12ROVOL - [2:0] */
pub const WM8400_RR12ROVOL_SHIFT: u16 = 0u16;  /* RR12ROVOL - [2:0] */
pub const WM8400_RR12ROVOL_WIDTH: u16 = 3u16;  /* RR12ROVOL - [2:0] */

/*
 * R49 (0x31) - Output Mixer5
 */
pub const WM8400_LRI3LOVOL_MASK: u16 = 0x01C0u16;  /* LRI3LOVOL - [8:6] */
pub const WM8400_LRI3LOVOL_SHIFT: u16 = 6u16;  /* LRI3LOVOL - [8:6] */
pub const WM8400_LRI3LOVOL_WIDTH: u16 = 3u16;  /* LRI3LOVOL - [8:6] */
pub const WM8400_LRBLOVOL_MASK: u16 = 0x0038u16;  /* LRBLOVOL - [5:3] */
pub const WM8400_LRBLOVOL_SHIFT: u16 = 3u16;  /* LRBLOVOL - [5:3] */
pub const WM8400_LRBLOVOL_WIDTH: u16 = 3u16;  /* LRBLOVOL - [5:3] */
pub const WM8400_LLBLOVOL_MASK: u16 = 0x0007u16;  /* LLBLOVOL - [2:0] */
pub const WM8400_LLBLOVOL_SHIFT: u16 = 0u16;  /* LLBLOVOL - [2:0] */
pub const WM8400_LLBLOVOL_WIDTH: u16 = 3u16;  /* LLBLOVOL - [2:0] */

/*
 * R50 (0x32) - Output Mixer6
 */
pub const WM8400_RLI3ROVOL_MASK: u16 = 0x01C0u16;  /* RLI3ROVOL - [8:6] */
pub const WM8400_RLI3ROVOL_SHIFT: u16 = 6u16;  /* RLI3ROVOL - [8:6] */
pub const WM8400_RLI3ROVOL_WIDTH: u16 = 3u16;  /* RLI3ROVOL - [8:6] */
pub const WM8400_RLBROVOL_MASK: u16 = 0x0038u16;  /* RLBROVOL - [5:3] */
pub const WM8400_RLBROVOL_SHIFT: u16 = 3u16;  /* RLBROVOL - [5:3] */
pub const WM8400_RLBROVOL_WIDTH: u16 = 3u16;  /* RLBROVOL - [5:3] */
pub const WM8400_RRBROVOL_MASK: u16 = 0x0007u16;  /* RRBROVOL - [2:0] */
pub const WM8400_RRBROVOL_SHIFT: u16 = 0u16;  /* RRBROVOL - [2:0] */
pub const WM8400_RRBROVOL_WIDTH: u16 = 3u16;  /* RRBROVOL - [2:0] */

/*
 * R51 (0x33) - Out3/4 Mixer
 */
pub const WM8400_VSEL_MASK: u16 = 0x0180u16;  /* VSEL - [8:7] */
pub const WM8400_VSEL_SHIFT: u16 = 7u16;  /* VSEL - [8:7] */
pub const WM8400_VSEL_WIDTH: u16 = 2u16;  /* VSEL - [8:7] */
pub const WM8400_LI4O3: u16 = 0x0020u16;  /* LI4O3 */
pub const WM8400_LI4O3_MASK: u16 = 0x0020u16;  /* LI4O3 */
pub const WM8400_LI4O3_SHIFT: u16 = 5u16;  /* LI4O3 */
pub const WM8400_LI4O3_WIDTH: u16 = 1u16;  /* LI4O3 */
pub const WM8400_LPGAO3: u16 = 0x0010u16;  /* LPGAO3 */
pub const WM8400_LPGAO3_MASK: u16 = 0x0010u16;  /* LPGAO3 */
pub const WM8400_LPGAO3_SHIFT: u16 = 4u16;  /* LPGAO3 */
pub const WM8400_LPGAO3_WIDTH: u16 = 1u16;  /* LPGAO3 */
pub const WM8400_RI4O4: u16 = 0x0002u16;  /* RI4O4 */
pub const WM8400_RI4O4_MASK: u16 = 0x0002u16;  /* RI4O4 */
pub const WM8400_RI4O4_SHIFT: u16 = 1u16;  /* RI4O4 */
pub const WM8400_RI4O4_WIDTH: u16 = 1u16;  /* RI4O4 */
pub const WM8400_RPGAO4: u16 = 0x0001u16;  /* RPGAO4 */
pub const WM8400_RPGAO4_MASK: u16 = 0x0001u16;  /* RPGAO4 */
pub const WM8400_RPGAO4_SHIFT: u16 = 0u16;  /* RPGAO4 */
pub const WM8400_RPGAO4_WIDTH: u16 = 1u16;  /* RPGAO4 */

/*
 * R52 (0x34) - Line Mixer1
 */
pub const WM8400_LLOPGALON: u16 = 0x0040u16;  /* LLOPGALON */
pub const WM8400_LLOPGALON_MASK: u16 = 0x0040u16;  /* LLOPGALON */
pub const WM8400_LLOPGALON_SHIFT: u16 = 6u16;  /* LLOPGALON */
pub const WM8400_LLOPGALON_WIDTH: u16 = 1u16;  /* LLOPGALON */
pub const WM8400_LROPGALON: u16 = 0x0020u16;  /* LROPGALON */
pub const WM8400_LROPGALON_MASK: u16 = 0x0020u16;  /* LROPGALON */
pub const WM8400_LROPGALON_SHIFT: u16 = 5u16;  /* LROPGALON */
pub const WM8400_LROPGALON_WIDTH: u16 = 1u16;  /* LROPGALON */
pub const WM8400_LOPLON: u16 = 0x0010u16;  /* LOPLON */
pub const WM8400_LOPLON_MASK: u16 = 0x0010u16;  /* LOPLON */
pub const WM8400_LOPLON_SHIFT: u16 = 4u16;  /* LOPLON */
pub const WM8400_LOPLON_WIDTH: u16 = 1u16;  /* LOPLON */
pub const WM8400_LR12LOP: u16 = 0x0004u16;  /* LR12LOP */
pub const WM8400_LR12LOP_MASK: u16 = 0x0004u16;  /* LR12LOP */
pub const WM8400_LR12LOP_SHIFT: u16 = 2u16;  /* LR12LOP */
pub const WM8400_LR12LOP_WIDTH: u16 = 1u16;  /* LR12LOP */
pub const WM8400_LL12LOP: u16 = 0x0002u16;  /* LL12LOP */
pub const WM8400_LL12LOP_MASK: u16 = 0x0002u16;  /* LL12LOP */
pub const WM8400_LL12LOP_SHIFT: u16 = 1u16;  /* LL12LOP */
pub const WM8400_LL12LOP_WIDTH: u16 = 1u16;  /* LL12LOP */
pub const WM8400_LLOPGALOP: u16 = 0x0001u16;  /* LLOPGALOP */
pub const WM8400_LLOPGALOP_MASK: u16 = 0x0001u16;  /* LLOPGALOP */
pub const WM8400_LLOPGALOP_SHIFT: u16 = 0u16;  /* LLOPGALOP */
pub const WM8400_LLOPGALOP_WIDTH: u16 = 1u16;  /* LLOPGALOP */

/*
 * R53 (0x35) - Line Mixer2
 */
pub const WM8400_RROPGARON: u16 = 0x0040u16;  /* RROPGARON */
pub const WM8400_RROPGARON_MASK: u16 = 0x0040u16;  /* RROPGARON */
pub const WM8400_RROPGARON_SHIFT: u16 = 6u16;  /* RROPGARON */
pub const WM8400_RROPGARON_WIDTH: u16 = 1u16;  /* RROPGARON */
pub const WM8400_RLOPGARON: u16 = 0x0020u16;  /* RLOPGARON */
pub const WM8400_RLOPGARON_MASK: u16 = 0x0020u16;  /* RLOPGARON */
pub const WM8400_RLOPGARON_SHIFT: u16 = 5u16;  /* RLOPGARON */
pub const WM8400_RLOPGARON_WIDTH: u16 = 1u16;  /* RLOPGARON */
pub const WM8400_ROPRON: u16 = 0x0010u16;  /* ROPRON */
pub const WM8400_ROPRON_MASK: u16 = 0x0010u16;  /* ROPRON */
pub const WM8400_ROPRON_SHIFT: u16 = 4u16;  /* ROPRON */
pub const WM8400_ROPRON_WIDTH: u16 = 1u16;  /* ROPRON */
pub const WM8400_RL12ROP: u16 = 0x0004u16;  /* RL12ROP */
pub const WM8400_RL12ROP_MASK: u16 = 0x0004u16;  /* RL12ROP */
pub const WM8400_RL12ROP_SHIFT: u16 = 2u16;  /* RL12ROP */
pub const WM8400_RL12ROP_WIDTH: u16 = 1u16;  /* RL12ROP */
pub const WM8400_RR12ROP: u16 = 0x0002u16;  /* RR12ROP */
pub const WM8400_RR12ROP_MASK: u16 = 0x0002u16;  /* RR12ROP */
pub const WM8400_RR12ROP_SHIFT: u16 = 1u16;  /* RR12ROP */
pub const WM8400_RR12ROP_WIDTH: u16 = 1u16;  /* RR12ROP */
pub const WM8400_RROPGAROP: u16 = 0x0001u16;  /* RROPGAROP */
pub const WM8400_RROPGAROP_MASK: u16 = 0x0001u16;  /* RROPGAROP */
pub const WM8400_RROPGAROP_SHIFT: u16 = 0u16;  /* RROPGAROP */
pub const WM8400_RROPGAROP_WIDTH: u16 = 1u16;  /* RROPGAROP */

/*
 * R54 (0x36) - Speaker Mixer
 */
pub const WM8400_LB2SPK: u16 = 0x0080u16;  /* LB2SPK */
pub const WM8400_LB2SPK_MASK: u16 = 0x0080u16;  /* LB2SPK */
pub const WM8400_LB2SPK_SHIFT: u16 = 7u16;  /* LB2SPK */
pub const WM8400_LB2SPK_WIDTH: u16 = 1u16;  /* LB2SPK */
pub const WM8400_RB2SPK: u16 = 0x0040u16;  /* RB2SPK */
pub const WM8400_RB2SPK_MASK: u16 = 0x0040u16;  /* RB2SPK */
pub const WM8400_RB2SPK_SHIFT: u16 = 6u16;  /* RB2SPK */
pub const WM8400_RB2SPK_WIDTH: u16 = 1u16;  /* RB2SPK */
pub const WM8400_LI2SPK: u16 = 0x0020u16;  /* LI2SPK */
pub const WM8400_LI2SPK_MASK: u16 = 0x0020u16;  /* LI2SPK */
pub const WM8400_LI2SPK_SHIFT: u16 = 5u16;  /* LI2SPK */
pub const WM8400_LI2SPK_WIDTH: u16 = 1u16;  /* LI2SPK */
pub const WM8400_RI2SPK: u16 = 0x0010u16;  /* RI2SPK */
pub const WM8400_RI2SPK_MASK: u16 = 0x0010u16;  /* RI2SPK */
pub const WM8400_RI2SPK_SHIFT: u16 = 4u16;  /* RI2SPK */
pub const WM8400_RI2SPK_WIDTH: u16 = 1u16;  /* RI2SPK */
pub const WM8400_LOPGASPK: u16 = 0x0008u16;  /* LOPGASPK */
pub const WM8400_LOPGASPK_MASK: u16 = 0x0008u16;  /* LOPGASPK */
pub const WM8400_LOPGASPK_SHIFT: u16 = 3u16;  /* LOPGASPK */
pub const WM8400_LOPGASPK_WIDTH: u16 = 1u16;  /* LOPGASPK */
pub const WM8400_ROPGASPK: u16 = 0x0004u16;  /* ROPGASPK */
pub const WM8400_ROPGASPK_MASK: u16 = 0x0004u16;  /* ROPGASPK */
pub const WM8400_ROPGASPK_SHIFT: u16 = 2u16;  /* ROPGASPK */
pub const WM8400_ROPGASPK_WIDTH: u16 = 1u16;  /* ROPGASPK */
pub const WM8400_LDSPK: u16 = 0x0002u16;  /* LDSPK */
pub const WM8400_LDSPK_MASK: u16 = 0x0002u16;  /* LDSPK */
pub const WM8400_LDSPK_SHIFT: u16 = 1u16;  /* LDSPK */
pub const WM8400_LDSPK_WIDTH: u16 = 1u16;  /* LDSPK */
pub const WM8400_RDSPK: u16 = 0x0001u16;  /* RDSPK */
pub const WM8400_RDSPK_MASK: u16 = 0x0001u16;  /* RDSPK */
pub const WM8400_RDSPK_SHIFT: u16 = 0u16;  /* RDSPK */
pub const WM8400_RDSPK_WIDTH: u16 = 1u16;  /* RDSPK */

/*
 * R55 (0x37) - Additional Control
 */
pub const WM8400_VROI: u16 = 0x0001u16;  /* VROI */
pub const WM8400_VROI_MASK: u16 = 0x0001u16;  /* VROI */
pub const WM8400_VROI_SHIFT: u16 = 0u16;  /* VROI */
pub const WM8400_VROI_WIDTH: u16 = 1u16;  /* VROI */

/*
 * R56 (0x38) - AntiPOP1
 */
pub const WM8400_DIS_LLINE: u16 = 0x0020u16;  /* DIS_LLINE */
pub const WM8400_DIS_LLINE_MASK: u16 = 0x0020u16;  /* DIS_LLINE */
pub const WM8400_DIS_LLINE_SHIFT: u16 = 5u16;  /* DIS_LLINE */
pub const WM8400_DIS_LLINE_WIDTH: u16 = 1u16;  /* DIS_LLINE */
pub const WM8400_DIS_RLINE: u16 = 0x0010u16;  /* DIS_RLINE */
pub const WM8400_DIS_RLINE_MASK: u16 = 0x0010u16;  /* DIS_RLINE */
pub const WM8400_DIS_RLINE_SHIFT: u16 = 4u16;  /* DIS_RLINE */
pub const WM8400_DIS_RLINE_WIDTH: u16 = 1u16;  /* DIS_RLINE */
pub const WM8400_DIS_OUT3: u16 = 0x0008u16;  /* DIS_OUT3 */
pub const WM8400_DIS_OUT3_MASK: u16 = 0x0008u16;  /* DIS_OUT3 */
pub const WM8400_DIS_OUT3_SHIFT: u16 = 3u16;  /* DIS_OUT3 */
pub const WM8400_DIS_OUT3_WIDTH: u16 = 1u16;  /* DIS_OUT3 */
pub const WM8400_DIS_OUT4: u16 = 0x0004u16;  /* DIS_OUT4 */
pub const WM8400_DIS_OUT4_MASK: u16 = 0x0004u16;  /* DIS_OUT4 */
pub const WM8400_DIS_OUT4_SHIFT: u16 = 2u16;  /* DIS_OUT4 */
pub const WM8400_DIS_OUT4_WIDTH: u16 = 1u16;  /* DIS_OUT4 */
pub const WM8400_DIS_LOUT: u16 = 0x0002u16;  /* DIS_LOUT */
pub const WM8400_DIS_LOUT_MASK: u16 = 0x0002u16;  /* DIS_LOUT */
pub const WM8400_DIS_LOUT_SHIFT: u16 = 1u16;  /* DIS_LOUT */
pub const WM8400_DIS_LOUT_WIDTH: u16 = 1u16;  /* DIS_LOUT */
pub const WM8400_DIS_ROUT: u16 = 0x0001u16;  /* DIS_ROUT */
pub const WM8400_DIS_ROUT_MASK: u16 = 0x0001u16;  /* DIS_ROUT */
pub const WM8400_DIS_ROUT_SHIFT: u16 = 0u16;  /* DIS_ROUT */
pub const WM8400_DIS_ROUT_WIDTH: u16 = 1u16;  /* DIS_ROUT */

/*
 * R57 (0x39) - AntiPOP2
 */
pub const WM8400_SOFTST: u16 = 0x0040u16;  /* SOFTST */
pub const WM8400_SOFTST_MASK: u16 = 0x0040u16;  /* SOFTST */
pub const WM8400_SOFTST_SHIFT: u16 = 6u16;  /* SOFTST */
pub const WM8400_SOFTST_WIDTH: u16 = 1u16;  /* SOFTST */
pub const WM8400_BUFIOEN: u16 = 0x0008u16;  /* BUFIOEN */
pub const WM8400_BUFIOEN_MASK: u16 = 0x0008u16;  /* BUFIOEN */
pub const WM8400_BUFIOEN_SHIFT: u16 = 3u16;  /* BUFIOEN */
pub const WM8400_BUFIOEN_WIDTH: u16 = 1u16;  /* BUFIOEN */
pub const WM8400_BUFDCOPEN: u16 = 0x0004u16;  /* BUFDCOPEN */
pub const WM8400_BUFDCOPEN_MASK: u16 = 0x0004u16;  /* BUFDCOPEN */
pub const WM8400_BUFDCOPEN_SHIFT: u16 = 2u16;  /* BUFDCOPEN */
pub const WM8400_BUFDCOPEN_WIDTH: u16 = 1u16;  /* BUFDCOPEN */
pub const WM8400_POBCTRL: u16 = 0x0002u16;  /* POBCTRL */
pub const WM8400_POBCTRL_MASK: u16 = 0x0002u16;  /* POBCTRL */
pub const WM8400_POBCTRL_SHIFT: u16 = 1u16;  /* POBCTRL */
pub const WM8400_POBCTRL_WIDTH: u16 = 1u16;  /* POBCTRL */
pub const WM8400_VMIDTOG: u16 = 0x0001u16;  /* VMIDTOG */
pub const WM8400_VMIDTOG_MASK: u16 = 0x0001u16;  /* VMIDTOG */
pub const WM8400_VMIDTOG_SHIFT: u16 = 0u16;  /* VMIDTOG */
pub const WM8400_VMIDTOG_WIDTH: u16 = 1u16;  /* VMIDTOG */

/*
 * R58 (0x3A) - MICBIAS
 */
pub const WM8400_MCDSCTH_MASK: u16 = 0x00C0u16;  /* MCDSCTH - [7:6] */
pub const WM8400_MCDSCTH_SHIFT: u16 = 6u16;  /* MCDSCTH - [7:6] */
pub const WM8400_MCDSCTH_WIDTH: u16 = 2u16;  /* MCDSCTH - [7:6] */
pub const WM8400_MCDTHR_MASK: u16 = 0x0038u16;  /* MCDTHR - [5:3] */
pub const WM8400_MCDTHR_SHIFT: u16 = 3u16;  /* MCDTHR - [5:3] */
pub const WM8400_MCDTHR_WIDTH: u16 = 3u16;  /* MCDTHR - [5:3] */
pub const WM8400_MCD: u16 = 0x0004u16;  /* MCD */
pub const WM8400_MCD_MASK: u16 = 0x0004u16;  /* MCD */
pub const WM8400_MCD_SHIFT: u16 = 2u16;  /* MCD */
pub const WM8400_MCD_WIDTH: u16 = 1u16;  /* MCD */
pub const WM8400_MBSEL: u16 = 0x0001u16;  /* MBSEL */
pub const WM8400_MBSEL_MASK: u16 = 0x0001u16;  /* MBSEL */
pub const WM8400_MBSEL_SHIFT: u16 = 0u16;  /* MBSEL */
pub const WM8400_MBSEL_WIDTH: u16 = 1u16;  /* MBSEL */

/*
 * R60 (0x3C) - FLL Control 1
 */
pub const WM8400_FLL_REF_FREQ: u16 = 0x1000u16;  /* FLL_REF_FREQ */
pub const WM8400_FLL_REF_FREQ_MASK: u16 = 0x1000u16;  /* FLL_REF_FREQ */
pub const WM8400_FLL_REF_FREQ_SHIFT: u16 = 12u16;  /* FLL_REF_FREQ */
pub const WM8400_FLL_REF_FREQ_WIDTH: u16 = 1u16;  /* FLL_REF_FREQ */
pub const WM8400_FLL_CLK_SRC_MASK: u16 = 0x0C00u16;  /* FLL_CLK_SRC - [11:10] */
pub const WM8400_FLL_CLK_SRC_SHIFT: u16 = 10u16;  /* FLL_CLK_SRC - [11:10] */
pub const WM8400_FLL_CLK_SRC_WIDTH: u16 = 2u16;  /* FLL_CLK_SRC - [11:10] */
pub const WM8400_FLL_FRAC: u16 = 0x0200u16;  /* FLL_FRAC */
pub const WM8400_FLL_FRAC_MASK: u16 = 0x0200u16;  /* FLL_FRAC */
pub const WM8400_FLL_FRAC_SHIFT: u16 = 9u16;  /* FLL_FRAC */
pub const WM8400_FLL_FRAC_WIDTH: u16 = 1u16;  /* FLL_FRAC */
pub const WM8400_FLL_OSC_ENA: u16 = 0x0100u16;  /* FLL_OSC_ENA */
pub const WM8400_FLL_OSC_ENA_MASK: u16 = 0x0100u16;  /* FLL_OSC_ENA */
pub const WM8400_FLL_OSC_ENA_SHIFT: u16 = 8u16;  /* FLL_OSC_ENA */
pub const WM8400_FLL_OSC_ENA_WIDTH: u16 = 1u16;  /* FLL_OSC_ENA */
pub const WM8400_FLL_CTRL_RATE_MASK: u16 = 0x00E0u16;  /* FLL_CTRL_RATE - [7:5] */
pub const WM8400_FLL_CTRL_RATE_SHIFT: u16 = 5u16;  /* FLL_CTRL_RATE - [7:5] */
pub const WM8400_FLL_CTRL_RATE_WIDTH: u16 = 3u16;  /* FLL_CTRL_RATE - [7:5] */
pub const WM8400_FLL_FRATIO_MASK: u16 = 0x001Fu16;  /* FLL_FRATIO - [4:0] */
pub const WM8400_FLL_FRATIO_SHIFT: u16 = 0u16;  /* FLL_FRATIO - [4:0] */
pub const WM8400_FLL_FRATIO_WIDTH: u16 = 5u16;  /* FLL_FRATIO - [4:0] */

/*
 * R61 (0x3D) - FLL Control 2
 */
pub const WM8400_FLL_K_MASK: u16 = 0xFFFFu16;  /* FLL_K - [15:0] */
pub const WM8400_FLL_K_SHIFT: u16 = 0u16;  /* FLL_K - [15:0] */
pub const WM8400_FLL_K_WIDTH: u16 = 16u16;  /* FLL_K - [15:0] */

/*
 * R62 (0x3E) - FLL Control 3
 */
pub const WM8400_FLL_N_MASK: u16 = 0x03FFu16;  /* FLL_N - [9:0] */
pub const WM8400_FLL_N_SHIFT: u16 = 0u16;  /* FLL_N - [9:0] */
pub const WM8400_FLL_N_WIDTH: u16 = 10u16;  /* FLL_N - [9:0] */

/*
 * R63 (0x3F) - FLL Control 4
 */
pub const WM8400_FLL_TRK_GAIN_MASK: u16 = 0x0078u16;  /* FLL_TRK_GAIN - [6:3] */
pub const WM8400_FLL_TRK_GAIN_SHIFT: u16 = 3u16;  /* FLL_TRK_GAIN - [6:3] */
pub const WM8400_FLL_TRK_GAIN_WIDTH: u16 = 4u16;  /* FLL_TRK_GAIN - [6:3] */
pub const WM8400_FLL_OUTDIV_MASK: u16 = 0x0007u16;  /* FLL_OUTDIV - [2:0] */
pub const WM8400_FLL_OUTDIV_SHIFT: u16 = 0u16;  /* FLL_OUTDIV - [2:0] */
pub const WM8400_FLL_OUTDIV_WIDTH: u16 = 3u16;  /* FLL_OUTDIV - [2:0] */

struct wm8400;
void wm8400_reset_codec_reg_cache(struct wm8400 *wm8400);

// C header guard omitted in Rust: #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
