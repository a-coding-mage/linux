/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * wm8737.c  --  WM8523 ALSA SoC Audio driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/*
 * Register values.
 */
pub const WM8737_LEFT_PGA_VOLUME: u16 = 0x00;
pub const WM8737_RIGHT_PGA_VOLUME: u16 = 0x01;
pub const WM8737_AUDIO_PATH_L: u16 = 0x02;
pub const WM8737_AUDIO_PATH_R: u16 = 0x03;
pub const WM8737_3D_ENHANCE: u16 = 0x04;
pub const WM8737_ADC_CONTROL: u16 = 0x05;
pub const WM8737_POWER_MANAGEMENT: u16 = 0x06;
pub const WM8737_AUDIO_FORMAT: u16 = 0x07;
pub const WM8737_CLOCKING: u16 = 0x08;
pub const WM8737_MIC_PREAMP_CONTROL: u16 = 0x09;
pub const WM8737_MISC_BIAS_CONTROL: u16 = 0x0A;
pub const WM8737_NOISE_GATE: u16 = 0x0B;
pub const WM8737_ALC1: u16 = 0x0C;
pub const WM8737_ALC2: u16 = 0x0D;
pub const WM8737_ALC3: u16 = 0x0E;
pub const WM8737_RESET: u16 = 0x0F;

pub const WM8737_REGISTER_COUNT: u16 = 16;
pub const WM8737_MAX_REGISTER: u16 = 0x0F;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - Left PGA volume
 */
pub const WM8737_LVU: u16 = 0x0100; /* LVU */
pub const WM8737_LVU_MASK: u16 = 0x0100; /* LVU */
pub const WM8737_LVU_SHIFT: u16 = 8; /* LVU */
pub const WM8737_LVU_WIDTH: u16 = 1; /* LVU */
pub const WM8737_LINVOL_MASK: u16 = 0x00FF; /* LINVOL - [7:0] */
pub const WM8737_LINVOL_SHIFT: u16 = 0; /* LINVOL - [7:0] */
pub const WM8737_LINVOL_WIDTH: u16 = 8; /* LINVOL - [7:0] */

/*
 * R1 (0x01) - Right PGA volume
 */
pub const WM8737_RVU: u16 = 0x0100; /* RVU */
pub const WM8737_RVU_MASK: u16 = 0x0100; /* RVU */
pub const WM8737_RVU_SHIFT: u16 = 8; /* RVU */
pub const WM8737_RVU_WIDTH: u16 = 1; /* RVU */
pub const WM8737_RINVOL_MASK: u16 = 0x00FF; /* RINVOL - [7:0] */
pub const WM8737_RINVOL_SHIFT: u16 = 0; /* RINVOL - [7:0] */
pub const WM8737_RINVOL_WIDTH: u16 = 8; /* RINVOL - [7:0] */

/*
 * R2 (0x02) - AUDIO path L
 */
pub const WM8737_LINSEL_MASK: u16 = 0x0180; /* LINSEL - [8:7] */
pub const WM8737_LINSEL_SHIFT: u16 = 7; /* LINSEL - [8:7] */
pub const WM8737_LINSEL_WIDTH: u16 = 2; /* LINSEL - [8:7] */
pub const WM8737_LMICBOOST_MASK: u16 = 0x0060; /* LMICBOOST - [6:5] */
pub const WM8737_LMICBOOST_SHIFT: u16 = 5; /* LMICBOOST - [6:5] */
pub const WM8737_LMICBOOST_WIDTH: u16 = 2; /* LMICBOOST - [6:5] */
pub const WM8737_LMBE: u16 = 0x0010; /* LMBE */
pub const WM8737_LMBE_MASK: u16 = 0x0010; /* LMBE */
pub const WM8737_LMBE_SHIFT: u16 = 4; /* LMBE */
pub const WM8737_LMBE_WIDTH: u16 = 1; /* LMBE */
pub const WM8737_LMZC: u16 = 0x0008; /* LMZC */
pub const WM8737_LMZC_MASK: u16 = 0x0008; /* LMZC */
pub const WM8737_LMZC_SHIFT: u16 = 3; /* LMZC */
pub const WM8737_LMZC_WIDTH: u16 = 1; /* LMZC */
pub const WM8737_LPZC: u16 = 0x0004; /* LPZC */
pub const WM8737_LPZC_MASK: u16 = 0x0004; /* LPZC */
pub const WM8737_LPZC_SHIFT: u16 = 2; /* LPZC */
pub const WM8737_LPZC_WIDTH: u16 = 1; /* LPZC */
pub const WM8737_LZCTO_MASK: u16 = 0x0003; /* LZCTO - [1:0] */
pub const WM8737_LZCTO_SHIFT: u16 = 0; /* LZCTO - [1:0] */
pub const WM8737_LZCTO_WIDTH: u16 = 2; /* LZCTO - [1:0] */

/*
 * R3 (0x03) - AUDIO path R
 */
pub const WM8737_RINSEL_MASK: u16 = 0x0180; /* RINSEL - [8:7] */
pub const WM8737_RINSEL_SHIFT: u16 = 7; /* RINSEL - [8:7] */
pub const WM8737_RINSEL_WIDTH: u16 = 2; /* RINSEL - [8:7] */
pub const WM8737_RMICBOOST_MASK: u16 = 0x0060; /* RMICBOOST - [6:5] */
pub const WM8737_RMICBOOST_SHIFT: u16 = 5; /* RMICBOOST - [6:5] */
pub const WM8737_RMICBOOST_WIDTH: u16 = 2; /* RMICBOOST - [6:5] */
pub const WM8737_RMBE: u16 = 0x0010; /* RMBE */
pub const WM8737_RMBE_MASK: u16 = 0x0010; /* RMBE */
pub const WM8737_RMBE_SHIFT: u16 = 4; /* RMBE */
pub const WM8737_RMBE_WIDTH: u16 = 1; /* RMBE */
pub const WM8737_RMZC: u16 = 0x0008; /* RMZC */
pub const WM8737_RMZC_MASK: u16 = 0x0008; /* RMZC */
pub const WM8737_RMZC_SHIFT: u16 = 3; /* RMZC */
pub const WM8737_RMZC_WIDTH: u16 = 1; /* RMZC */
pub const WM8737_RPZC: u16 = 0x0004; /* RPZC */
pub const WM8737_RPZC_MASK: u16 = 0x0004; /* RPZC */
pub const WM8737_RPZC_SHIFT: u16 = 2; /* RPZC */
pub const WM8737_RPZC_WIDTH: u16 = 1; /* RPZC */
pub const WM8737_RZCTO_MASK: u16 = 0x0003; /* RZCTO - [1:0] */
pub const WM8737_RZCTO_SHIFT: u16 = 0; /* RZCTO - [1:0] */
pub const WM8737_RZCTO_WIDTH: u16 = 2; /* RZCTO - [1:0] */

/*
 * R4 (0x04) - 3D Enhance
 */
pub const WM8737_DIV2: u16 = 0x0080; /* DIV2 */
pub const WM8737_DIV2_MASK: u16 = 0x0080; /* DIV2 */
pub const WM8737_DIV2_SHIFT: u16 = 7; /* DIV2 */
pub const WM8737_DIV2_WIDTH: u16 = 1; /* DIV2 */
pub const WM8737_3DLC: u16 = 0x0040; /* 3DLC */
pub const WM8737_3DLC_MASK: u16 = 0x0040; /* 3DLC */
pub const WM8737_3DLC_SHIFT: u16 = 6; /* 3DLC */
pub const WM8737_3DLC_WIDTH: u16 = 1; /* 3DLC */
pub const WM8737_3DUC: u16 = 0x0020; /* 3DUC */
pub const WM8737_3DUC_MASK: u16 = 0x0020; /* 3DUC */
pub const WM8737_3DUC_SHIFT: u16 = 5; /* 3DUC */
pub const WM8737_3DUC_WIDTH: u16 = 1; /* 3DUC */
pub const WM8737_3DDEPTH_MASK: u16 = 0x001E; /* 3DDEPTH - [4:1] */
pub const WM8737_3DDEPTH_SHIFT: u16 = 1; /* 3DDEPTH - [4:1] */
pub const WM8737_3DDEPTH_WIDTH: u16 = 4; /* 3DDEPTH - [4:1] */
pub const WM8737_3DE: u16 = 0x0001; /* 3DE */
pub const WM8737_3DE_MASK: u16 = 0x0001; /* 3DE */
pub const WM8737_3DE_SHIFT: u16 = 0; /* 3DE */
pub const WM8737_3DE_WIDTH: u16 = 1; /* 3DE */

/*
 * R5 (0x05) - ADC Control
 */
pub const WM8737_MONOMIX_MASK: u16 = 0x0180; /* MONOMIX - [8:7] */
pub const WM8737_MONOMIX_SHIFT: u16 = 7; /* MONOMIX - [8:7] */
pub const WM8737_MONOMIX_WIDTH: u16 = 2; /* MONOMIX - [8:7] */
pub const WM8737_POLARITY_MASK: u16 = 0x0060; /* POLARITY - [6:5] */
pub const WM8737_POLARITY_SHIFT: u16 = 5; /* POLARITY - [6:5] */
pub const WM8737_POLARITY_WIDTH: u16 = 2; /* POLARITY - [6:5] */
pub const WM8737_HPOR: u16 = 0x0010; /* HPOR */
pub const WM8737_HPOR_MASK: u16 = 0x0010; /* HPOR */
pub const WM8737_HPOR_SHIFT: u16 = 4; /* HPOR */
pub const WM8737_HPOR_WIDTH: u16 = 1; /* HPOR */
pub const WM8737_LP: u16 = 0x0004; /* LP */
pub const WM8737_LP_MASK: u16 = 0x0004; /* LP */
pub const WM8737_LP_SHIFT: u16 = 2; /* LP */
pub const WM8737_LP_WIDTH: u16 = 1; /* LP */
pub const WM8737_MONOUT: u16 = 0x0002; /* MONOUT */
pub const WM8737_MONOUT_MASK: u16 = 0x0002; /* MONOUT */
pub const WM8737_MONOUT_SHIFT: u16 = 1; /* MONOUT */
pub const WM8737_MONOUT_WIDTH: u16 = 1; /* MONOUT */
pub const WM8737_ADCHPD: u16 = 0x0001; /* ADCHPD */
pub const WM8737_ADCHPD_MASK: u16 = 0x0001; /* ADCHPD */
pub const WM8737_ADCHPD_SHIFT: u16 = 0; /* ADCHPD */
pub const WM8737_ADCHPD_WIDTH: u16 = 1; /* ADCHPD */

/*
 * R6 (0x06) - Power Management
 */
pub const WM8737_VMID: u16 = 0x0100; /* VMID */
pub const WM8737_VMID_MASK: u16 = 0x0100; /* VMID */
pub const WM8737_VMID_SHIFT: u16 = 8; /* VMID */
pub const WM8737_VMID_WIDTH: u16 = 1; /* VMID */
pub const WM8737_VREF: u16 = 0x0080; /* VREF */
pub const WM8737_VREF_MASK: u16 = 0x0080; /* VREF */
pub const WM8737_VREF_SHIFT: u16 = 7; /* VREF */
pub const WM8737_VREF_WIDTH: u16 = 1; /* VREF */
pub const WM8737_AI: u16 = 0x0040; /* AI */
pub const WM8737_AI_MASK: u16 = 0x0040; /* AI */
pub const WM8737_AI_SHIFT: u16 = 6; /* AI */
pub const WM8737_AI_WIDTH: u16 = 1; /* AI */
pub const WM8737_PGL: u16 = 0x0020; /* PGL */
pub const WM8737_PGL_MASK: u16 = 0x0020; /* PGL */
pub const WM8737_PGL_SHIFT: u16 = 5; /* PGL */
pub const WM8737_PGL_WIDTH: u16 = 1; /* PGL */
pub const WM8737_PGR: u16 = 0x0010; /* PGR */
pub const WM8737_PGR_MASK: u16 = 0x0010; /* PGR */
pub const WM8737_PGR_SHIFT: u16 = 4; /* PGR */
pub const WM8737_PGR_WIDTH: u16 = 1; /* PGR */
pub const WM8737_ADL: u16 = 0x0008; /* ADL */
pub const WM8737_ADL_MASK: u16 = 0x0008; /* ADL */
pub const WM8737_ADL_SHIFT: u16 = 3; /* ADL */
pub const WM8737_ADL_WIDTH: u16 = 1; /* ADL */
pub const WM8737_ADR: u16 = 0x0004; /* ADR */
pub const WM8737_ADR_MASK: u16 = 0x0004; /* ADR */
pub const WM8737_ADR_SHIFT: u16 = 2; /* ADR */
pub const WM8737_ADR_WIDTH: u16 = 1; /* ADR */
pub const WM8737_MICBIAS_MASK: u16 = 0x0003; /* MICBIAS - [1:0] */
pub const WM8737_MICBIAS_SHIFT: u16 = 0; /* MICBIAS - [1:0] */
pub const WM8737_MICBIAS_WIDTH: u16 = 2; /* MICBIAS - [1:0] */

/*
 * R7 (0x07) - Audio Format
 */
pub const WM8737_SDODIS: u16 = 0x0080; /* SDODIS */
pub const WM8737_SDODIS_MASK: u16 = 0x0080; /* SDODIS */
pub const WM8737_SDODIS_SHIFT: u16 = 7; /* SDODIS */
pub const WM8737_SDODIS_WIDTH: u16 = 1; /* SDODIS */
pub const WM8737_MS: u16 = 0x0040; /* MS */
pub const WM8737_MS_MASK: u16 = 0x0040; /* MS */
pub const WM8737_MS_SHIFT: u16 = 6; /* MS */
pub const WM8737_MS_WIDTH: u16 = 1; /* MS */
pub const WM8737_LRP: u16 = 0x0010; /* LRP */
pub const WM8737_LRP_MASK: u16 = 0x0010; /* LRP */
pub const WM8737_LRP_SHIFT: u16 = 4; /* LRP */
pub const WM8737_LRP_WIDTH: u16 = 1; /* LRP */
pub const WM8737_WL_MASK: u16 = 0x000C; /* WL - [3:2] */
pub const WM8737_WL_SHIFT: u16 = 2; /* WL - [3:2] */
pub const WM8737_WL_WIDTH: u16 = 2; /* WL - [3:2] */
pub const WM8737_FORMAT_MASK: u16 = 0x0003; /* FORMAT - [1:0] */
pub const WM8737_FORMAT_SHIFT: u16 = 0; /* FORMAT - [1:0] */
pub const WM8737_FORMAT_WIDTH: u16 = 2; /* FORMAT - [1:0] */

/*
 * R8 (0x08) - Clocking
 */
pub const WM8737_AUTODETECT: u16 = 0x0080; /* AUTODETECT */
pub const WM8737_AUTODETECT_MASK: u16 = 0x0080; /* AUTODETECT */
pub const WM8737_AUTODETECT_SHIFT: u16 = 7; /* AUTODETECT */
pub const WM8737_AUTODETECT_WIDTH: u16 = 1; /* AUTODETECT */
pub const WM8737_CLKDIV2: u16 = 0x0040; /* CLKDIV2 */
pub const WM8737_CLKDIV2_MASK: u16 = 0x0040; /* CLKDIV2 */
pub const WM8737_CLKDIV2_SHIFT: u16 = 6; /* CLKDIV2 */
pub const WM8737_CLKDIV2_WIDTH: u16 = 1; /* CLKDIV2 */
pub const WM8737_SR_MASK: u16 = 0x003E; /* SR - [5:1] */
pub const WM8737_SR_SHIFT: u16 = 1; /* SR - [5:1] */
pub const WM8737_SR_WIDTH: u16 = 5; /* SR - [5:1] */
pub const WM8737_USB_MODE: u16 = 0x0001; /* USB MODE */
pub const WM8737_USB_MODE_MASK: u16 = 0x0001; /* USB MODE */
pub const WM8737_USB_MODE_SHIFT: u16 = 0; /* USB MODE */
pub const WM8737_USB_MODE_WIDTH: u16 = 1; /* USB MODE */

/*
 * R9 (0x09) - MIC Preamp Control
 */
pub const WM8737_RBYPEN: u16 = 0x0008; /* RBYPEN */
pub const WM8737_RBYPEN_MASK: u16 = 0x0008; /* RBYPEN */
pub const WM8737_RBYPEN_SHIFT: u16 = 3; /* RBYPEN */
pub const WM8737_RBYPEN_WIDTH: u16 = 1; /* RBYPEN */
pub const WM8737_LBYPEN: u16 = 0x0004; /* LBYPEN */
pub const WM8737_LBYPEN_MASK: u16 = 0x0004; /* LBYPEN */
pub const WM8737_LBYPEN_SHIFT: u16 = 2; /* LBYPEN */
pub const WM8737_LBYPEN_WIDTH: u16 = 1; /* LBYPEN */
pub const WM8737_MBCTRL_MASK: u16 = 0x0003; /* MBCTRL - [1:0] */
pub const WM8737_MBCTRL_SHIFT: u16 = 0; /* MBCTRL - [1:0] */
pub const WM8737_MBCTRL_WIDTH: u16 = 2; /* MBCTRL - [1:0] */

/*
 * R10 (0x0A) - Misc Bias Control
 */
pub const WM8737_VMIDSEL_MASK: u16 = 0x000C; /* VMIDSEL - [3:2] */
pub const WM8737_VMIDSEL_SHIFT: u16 = 2; /* VMIDSEL - [3:2] */
pub const WM8737_VMIDSEL_WIDTH: u16 = 2; /* VMIDSEL - [3:2] */
pub const WM8737_LINPUT1_DC_BIAS_ENABLE: u16 = 0x0002; /* LINPUT1 DC BIAS ENABLE */
pub const WM8737_LINPUT1_DC_BIAS_ENABLE_MASK: u16 = 0x0002; /* LINPUT1 DC BIAS ENABLE */
pub const WM8737_LINPUT1_DC_BIAS_ENABLE_SHIFT: u16 = 1; /* LINPUT1 DC BIAS ENABLE */
pub const WM8737_LINPUT1_DC_BIAS_ENABLE_WIDTH: u16 = 1; /* LINPUT1 DC BIAS ENABLE */
pub const WM8737_RINPUT1_DC_BIAS_ENABLE: u16 = 0x0001; /* RINPUT1 DC BIAS ENABLE */
pub const WM8737_RINPUT1_DC_BIAS_ENABLE_MASK: u16 = 0x0001; /* RINPUT1 DC BIAS ENABLE */
pub const WM8737_RINPUT1_DC_BIAS_ENABLE_SHIFT: u16 = 0; /* RINPUT1 DC BIAS ENABLE */
pub const WM8737_RINPUT1_DC_BIAS_ENABLE_WIDTH: u16 = 1; /* RINPUT1 DC BIAS ENABLE */

/*
 * R11 (0x0B) - Noise Gate
 */
pub const WM8737_NGTH_MASK: u16 = 0x001C; /* NGTH - [4:2] */
pub const WM8737_NGTH_SHIFT: u16 = 2; /* NGTH - [4:2] */
pub const WM8737_NGTH_WIDTH: u16 = 3; /* NGTH - [4:2] */
pub const WM8737_NGAT: u16 = 0x0001; /* NGAT */
pub const WM8737_NGAT_MASK: u16 = 0x0001; /* NGAT */
pub const WM8737_NGAT_SHIFT: u16 = 0; /* NGAT */
pub const WM8737_NGAT_WIDTH: u16 = 1; /* NGAT */

/*
 * R12 (0x0C) - ALC1
 */
pub const WM8737_ALCSEL_MASK: u16 = 0x0180; /* ALCSEL - [8:7] */
pub const WM8737_ALCSEL_SHIFT: u16 = 7; /* ALCSEL - [8:7] */
pub const WM8737_ALCSEL_WIDTH: u16 = 2; /* ALCSEL - [8:7] */
pub const WM8737_MAX_GAIN_MASK: u16 = 0x0070; /* MAX GAIN - [6:4] */
pub const WM8737_MAX_GAIN_SHIFT: u16 = 4; /* MAX GAIN - [6:4] */
pub const WM8737_MAX_GAIN_WIDTH: u16 = 3; /* MAX GAIN - [6:4] */
pub const WM8737_ALCL_MASK: u16 = 0x000F; /* ALCL - [3:0] */
pub const WM8737_ALCL_SHIFT: u16 = 0; /* ALCL - [3:0] */
pub const WM8737_ALCL_WIDTH: u16 = 4; /* ALCL - [3:0] */

/*
 * R13 (0x0D) - ALC2
 */
pub const WM8737_ALCZCE: u16 = 0x0010; /* ALCZCE */
pub const WM8737_ALCZCE_MASK: u16 = 0x0010; /* ALCZCE */
pub const WM8737_ALCZCE_SHIFT: u16 = 4; /* ALCZCE */
pub const WM8737_ALCZCE_WIDTH: u16 = 1; /* ALCZCE */
pub const WM8737_HLD_MASK: u16 = 0x000F; /* HLD - [3:0] */
pub const WM8737_HLD_SHIFT: u16 = 0; /* HLD - [3:0] */
pub const WM8737_HLD_WIDTH: u16 = 4; /* HLD - [3:0] */

/*
 * R14 (0x0E) - ALC3
 */
pub const WM8737_DCY_MASK: u16 = 0x00F0; /* DCY - [7:4] */
pub const WM8737_DCY_SHIFT: u16 = 4; /* DCY - [7:4] */
pub const WM8737_DCY_WIDTH: u16 = 4; /* DCY - [7:4] */
pub const WM8737_ATK_MASK: u16 = 0x000F; /* ATK - [3:0] */
pub const WM8737_ATK_SHIFT: u16 = 0; /* ATK - [3:0] */
pub const WM8737_ATK_WIDTH: u16 = 4; /* ATK - [3:0] */

/*
 * R15 (0x0F) - Reset
 */
pub const WM8737_RESET_MASK: u16 = 0x01FF; /* RESET - [8:0] */
pub const WM8737_RESET_SHIFT: u16 = 0; /* RESET - [8:0] */
pub const WM8737_RESET_WIDTH: u16 = 9; /* RESET - [8:0] */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
