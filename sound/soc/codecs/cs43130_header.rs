/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC CS43130 codec driver
 *
 * Copyright 2017 Cirrus Logic, Inc.
 *
 * Author: Li Xu <li.xu@cirrus.com>
 */

/* Dependency intent from C header: #include <linux/math.h> */

/* CS43130 registers addresses */
/* all reg address is shifted by a byte for control byte to be LSB */
pub const CS43130_FIRSTREG: u32 = 0x010000;
pub const CS43130_LASTREG: u32 = 0x190000;
pub const CS43130_CHIP_ID: u32 = 0x00043130;
pub const CS4399_CHIP_ID: u32 = 0x00043990;
pub const CS43131_CHIP_ID: u32 = 0x00043131;
pub const CS43198_CHIP_ID: u32 = 0x00043198;
pub const CS43130_DEVID_AB: u32 = 0x010000; /* Device ID A & B [RO] */
pub const CS43130_DEVID_CD: u32 = 0x010001; /* Device ID C & D [RO] */
pub const CS43130_DEVID_E: u32 = 0x010002; /* Device ID E [RO] */
pub const CS43130_FAB_ID: u32 = 0x010003; /* Fab ID [RO] */
pub const CS43130_REV_ID: u32 = 0x010004; /* Revision ID [RO] */
pub const CS43130_SUBREV_ID: u32 = 0x010005; /* Subrevision ID */
pub const CS43130_SYS_CLK_CTL_1: u32 = 0x010006; /* System Clocking Ctl 1 */
pub const CS43130_SP_SRATE: u32 = 0x01000B; /* Serial Port Sample Rate */
pub const CS43130_SP_BITSIZE: u32 = 0x01000C; /* Serial Port Bit Size */
pub const CS43130_PAD_INT_CFG: u32 = 0x01000D; /* Pad Interface Config */
pub const CS43130_DXD1: u32 = 0x010010; /* DXD1 */
pub const CS43130_DXD7: u32 = 0x010025; /* DXD7 */
pub const CS43130_DXD19: u32 = 0x010026; /* DXD19 */
pub const CS43130_DXD17: u32 = 0x010027; /* DXD17 */
pub const CS43130_DXD18: u32 = 0x010028; /* DXD18 */
pub const CS43130_DXD12: u32 = 0x01002C; /* DXD12 */
pub const CS43130_DXD8: u32 = 0x01002E; /* DXD8 */
pub const CS43130_PWDN_CTL: u32 = 0x020000; /* Power Down Ctl */
pub const CS43130_DXD2: u32 = 0x020019; /* DXD2 */
pub const CS43130_CRYSTAL_SET: u32 = 0x020052; /* Crystal Setting */
pub const CS43130_PLL_SET_1: u32 = 0x030001; /* PLL Setting 1 */
pub const CS43130_PLL_SET_2: u32 = 0x030002; /* PLL Setting 2 */
pub const CS43130_PLL_SET_3: u32 = 0x030003; /* PLL Setting 3 */
pub const CS43130_PLL_SET_4: u32 = 0x030004; /* PLL Setting 4 */
pub const CS43130_PLL_SET_5: u32 = 0x030005; /* PLL Setting 5 */
pub const CS43130_PLL_SET_6: u32 = 0x030008; /* PLL Setting 6 */
pub const CS43130_PLL_SET_7: u32 = 0x03000A; /* PLL Setting 7 */
pub const CS43130_PLL_SET_8: u32 = 0x03001B; /* PLL Setting 8 */
pub const CS43130_PLL_SET_9: u32 = 0x040002; /* PLL Setting 9 */
pub const CS43130_PLL_SET_10: u32 = 0x040003; /* PLL Setting 10 */
pub const CS43130_CLKOUT_CTL: u32 = 0x040004; /* CLKOUT Ctl */
pub const CS43130_ASP_NUM_1: u32 = 0x040010; /* ASP Numerator 1 */
pub const CS43130_ASP_NUM_2: u32 = 0x040011; /* ASP Numerator 2 */
pub const CS43130_ASP_DEN_1: u32 = 0x040012; /* ASP Denominator 1 */
pub const CS43130_ASP_DEN_2: u32 = 0x040013; /* ASP Denominator 2 */
pub const CS43130_ASP_LRCK_HI_TIME_1: u32 = 0x040014; /* ASP LRCK High Time 1 */
pub const CS43130_ASP_LRCK_HI_TIME_2: u32 = 0x040015; /* ASP LRCK High Time 2 */
pub const CS43130_ASP_LRCK_PERIOD_1: u32 = 0x040016; /* ASP LRCK Period 1 */
pub const CS43130_ASP_LRCK_PERIOD_2: u32 = 0x040017; /* ASP LRCK Period 2 */
pub const CS43130_ASP_CLOCK_CONF: u32 = 0x040018; /* ASP Clock Config */
pub const CS43130_ASP_FRAME_CONF: u32 = 0x040019; /* ASP Frame Config */
pub const CS43130_XSP_NUM_1: u32 = 0x040020; /* XSP Numerator 1 */
pub const CS43130_XSP_NUM_2: u32 = 0x040021; /* XSP Numerator 2 */
pub const CS43130_XSP_DEN_1: u32 = 0x040022; /* XSP Denominator 1 */
pub const CS43130_XSP_DEN_2: u32 = 0x040023; /* XSP Denominator 2 */
pub const CS43130_XSP_LRCK_HI_TIME_1: u32 = 0x040024; /* XSP LRCK High Time 1 */
pub const CS43130_XSP_LRCK_HI_TIME_2: u32 = 0x040025; /* XSP LRCK High Time 2 */
pub const CS43130_XSP_LRCK_PERIOD_1: u32 = 0x040026; /* XSP LRCK Period 1 */
pub const CS43130_XSP_LRCK_PERIOD_2: u32 = 0x040027; /* XSP LRCK Period 2 */
pub const CS43130_XSP_CLOCK_CONF: u32 = 0x040028; /* XSP Clock Config */
pub const CS43130_XSP_FRAME_CONF: u32 = 0x040029; /* XSP Frame Config */
pub const CS43130_ASP_CH_1_LOC: u32 = 0x050000; /* ASP Chan 1 Location */
pub const CS43130_ASP_CH_2_LOC: u32 = 0x050001; /* ASP Chan 2 Location */
pub const CS43130_ASP_CH_1_SZ_EN: u32 = 0x05000A; /* ASP Chan 1 Size, Enable */
pub const CS43130_ASP_CH_2_SZ_EN: u32 = 0x05000B; /* ASP Chan 2 Size, Enable */
pub const CS43130_XSP_CH_1_LOC: u32 = 0x060000; /* XSP Chan 1 Location */
pub const CS43130_XSP_CH_2_LOC: u32 = 0x060001; /* XSP Chan 2 Location */
pub const CS43130_XSP_CH_1_SZ_EN: u32 = 0x06000A; /* XSP Chan 1 Size, Enable */
pub const CS43130_XSP_CH_2_SZ_EN: u32 = 0x06000B; /* XSP Chan 2 Size, Enable */
pub const CS43130_DSD_VOL_B: u32 = 0x070000; /* DSD Volume B */
pub const CS43130_DSD_VOL_A: u32 = 0x070001; /* DSD Volume A */
pub const CS43130_DSD_PATH_CTL_1: u32 = 0x070002; /* DSD Proc Path Sig Ctl 1 */
pub const CS43130_DSD_INT_CFG: u32 = 0x070003; /* DSD Interface Config */
pub const CS43130_DSD_PATH_CTL_2: u32 = 0x070004; /* DSD Proc Path Sig Ctl 2 */
pub const CS43130_DSD_PCM_MIX_CTL: u32 = 0x070005; /* DSD and PCM Mixing Ctl */
pub const CS43130_DSD_PATH_CTL_3: u32 = 0x070006; /* DSD Proc Path Sig Ctl 3 */
pub const CS43130_HP_OUT_CTL_1: u32 = 0x080000; /* HP Output Ctl 1 */
pub const CS43130_DXD16: u32 = 0x080024; /* DXD16 */
pub const CS43130_DXD13: u32 = 0x080032; /* DXD13 */
pub const CS43130_PCM_FILT_OPT: u32 = 0x090000; /* PCM Filter Option */
pub const CS43130_PCM_VOL_B: u32 = 0x090001; /* PCM Volume B */
pub const CS43130_PCM_VOL_A: u32 = 0x090002; /* PCM Volume A */
pub const CS43130_PCM_PATH_CTL_1: u32 = 0x090003; /* PCM Path Signal Ctl 1 */
pub const CS43130_PCM_PATH_CTL_2: u32 = 0x090004; /* PCM Path Signal Ctl 2 */
pub const CS43130_DXD6: u32 = 0x090097; /* DXD6 */
pub const CS43130_CLASS_H_CTL: u32 = 0x0B0000; /* Class H Ctl */
pub const CS43130_DXD15: u32 = 0x0B0005; /* DXD15 */
pub const CS43130_DXD14: u32 = 0x0B0006; /* DXD14 */
pub const CS43130_DXD3: u32 = 0x0C0002; /* DXD3 */
pub const CS43130_DXD10: u32 = 0x0C0003; /* DXD10 */
pub const CS43130_DXD11: u32 = 0x0C0005; /* DXD11 */
pub const CS43130_DXD9: u32 = 0x0C0006; /* DXD9 */
pub const CS43130_DXD4: u32 = 0x0C0009; /* DXD4 */
pub const CS43130_DXD5: u32 = 0x0C000E; /* DXD5 */
pub const CS43130_HP_DETECT: u32 = 0x0D0000; /* HP Detect */
pub const CS43130_HP_STATUS: u32 = 0x0D0001; /* HP Status [RO] */
pub const CS43130_HP_LOAD_1: u32 = 0x0E0000; /* HP Load 1 */
pub const CS43130_HP_MEAS_LOAD_1: u32 = 0x0E0003; /* HP Load Measurement 1 */
pub const CS43130_HP_MEAS_LOAD_2: u32 = 0x0E0004; /* HP Load Measurement 2 */
pub const CS43130_HP_DC_STAT_1: u32 = 0x0E000D; /* HP DC Load Status 0 [RO] */
pub const CS43130_HP_DC_STAT_2: u32 = 0x0E000E; /* HP DC Load Status 1 [RO] */
pub const CS43130_HP_AC_STAT_1: u32 = 0x0E0010; /* HP AC Load Status 0 [RO] */
pub const CS43130_HP_AC_STAT_2: u32 = 0x0E0011; /* HP AC Load Status 1 [RO] */
pub const CS43130_HP_LOAD_STAT: u32 = 0x0E001A; /* HP Load Status [RO] */
pub const CS43130_INT_STATUS_1: u32 = 0x0F0000; /* Interrupt Status 1 */
pub const CS43130_INT_STATUS_2: u32 = 0x0F0001; /* Interrupt Status 2 */
pub const CS43130_INT_STATUS_3: u32 = 0x0F0002; /* Interrupt Status 3 */
pub const CS43130_INT_STATUS_4: u32 = 0x0F0003; /* Interrupt Status 4 */
pub const CS43130_INT_STATUS_5: u32 = 0x0F0004; /* Interrupt Status 5 */
pub const CS43130_INT_MASK_1: u32 = 0x0F0010; /* Interrupt Mask 1 */
pub const CS43130_INT_MASK_2: u32 = 0x0F0011; /* Interrupt Mask 2 */
pub const CS43130_INT_MASK_3: u32 = 0x0F0012; /* Interrupt Mask 3 */
pub const CS43130_INT_MASK_4: u32 = 0x0F0013; /* Interrupt Mask 4 */
pub const CS43130_INT_MASK_5: u32 = 0x0F0014; /* Interrupt Mask 5 */

pub const CS43130_MCLK_SRC_SEL_MASK: u32 = 0x03;
pub const CS43130_MCLK_SRC_SEL_SHIFT: u32 = 0;
pub const CS43130_MCLK_INT_MASK: u32 = 0x04;
pub const CS43130_MCLK_INT_SHIFT: u32 = 2;
pub const CS43130_CH_BITSIZE_MASK: u32 = 0x03;
pub const CS43130_CH_EN_MASK: u32 = 0x04;
pub const CS43130_CH_EN_SHIFT: u32 = 2;
pub const CS43130_ASP_BITSIZE_MASK: u32 = 0x03;
pub const CS43130_XSP_BITSIZE_MASK: u32 = 0x0C;
pub const CS43130_XSP_BITSIZE_SHIFT: u32 = 2;
pub const CS43130_SP_BITSIZE_ASP_SHIFT: u32 = 0;
pub const CS43130_HP_DETECT_CTRL_SHIFT: u32 = 6;
pub const CS43130_HP_DETECT_CTRL_MASK: u32 = 0x03 << CS43130_HP_DETECT_CTRL_SHIFT;
pub const CS43130_HP_DETECT_INV_SHIFT: u32 = 5;
pub const CS43130_HP_DETECT_INV_MASK: u32 = 1 << CS43130_HP_DETECT_INV_SHIFT;

/* CS43130_INT_MASK_1 */
pub const CS43130_HP_PLUG_INT_SHIFT: u32 = 6;
pub const CS43130_HP_PLUG_INT: u32 = 1 << CS43130_HP_PLUG_INT_SHIFT;
pub const CS43130_HP_UNPLUG_INT_SHIFT: u32 = 5;
pub const CS43130_HP_UNPLUG_INT: u32 = 1 << CS43130_HP_UNPLUG_INT_SHIFT;
pub const CS43130_XTAL_RDY_INT_SHIFT: u32 = 4;
pub const CS43130_XTAL_RDY_INT_MASK: u32 = 0x10;
pub const CS43130_XTAL_RDY_INT: u32 = 1 << CS43130_XTAL_RDY_INT_SHIFT;
pub const CS43130_XTAL_ERR_INT_SHIFT: u32 = 3;
pub const CS43130_XTAL_ERR_INT: u32 = 1 << CS43130_XTAL_ERR_INT_SHIFT;
pub const CS43130_PLL_RDY_INT_MASK: u32 = 0x04;
pub const CS43130_PLL_RDY_INT_SHIFT: u32 = 2;
pub const CS43130_PLL_RDY_INT: u32 = 1 << CS43130_PLL_RDY_INT_SHIFT;

/* CS43130_INT_MASK_4 */
pub const CS43130_INT_MASK_ALL: u32 = 0xFF;
pub const CS43130_HPLOAD_NO_DC_INT_SHIFT: u32 = 7;
pub const CS43130_HPLOAD_NO_DC_INT: u32 = 1 << CS43130_HPLOAD_NO_DC_INT_SHIFT;
pub const CS43130_HPLOAD_UNPLUG_INT_SHIFT: u32 = 6;
pub const CS43130_HPLOAD_UNPLUG_INT: u32 = 1 << CS43130_HPLOAD_UNPLUG_INT_SHIFT;
pub const CS43130_HPLOAD_OOR_INT_SHIFT: u32 = 4;
pub const CS43130_HPLOAD_OOR_INT: u32 = 1 << CS43130_HPLOAD_OOR_INT_SHIFT;
pub const CS43130_HPLOAD_AC_INT_SHIFT: u32 = 3;
pub const CS43130_HPLOAD_AC_INT: u32 = 1 << CS43130_HPLOAD_AC_INT_SHIFT;
pub const CS43130_HPLOAD_DC_INT_SHIFT: u32 = 2;
pub const CS43130_HPLOAD_DC_INT: u32 = 1 << CS43130_HPLOAD_DC_INT_SHIFT;
pub const CS43130_HPLOAD_OFF_INT_SHIFT: u32 = 1;
pub const CS43130_HPLOAD_OFF_INT: u32 = 1 << CS43130_HPLOAD_OFF_INT_SHIFT;
pub const CS43130_HPLOAD_ON_INT: u32 = 1;

/* CS43130_HP_LOAD_1 */
pub const CS43130_HPLOAD_EN_SHIFT: u32 = 7;
pub const CS43130_HPLOAD_EN: u32 = 1 << CS43130_HPLOAD_EN_SHIFT;
pub const CS43130_HPLOAD_CHN_SEL_SHIFT: u32 = 4;
pub const CS43130_HPLOAD_CHN_SEL: u32 = 1 << CS43130_HPLOAD_CHN_SEL_SHIFT;
pub const CS43130_HPLOAD_AC_START_SHIFT: u32 = 1;
pub const CS43130_HPLOAD_AC_START: u32 = 1 << CS43130_HPLOAD_AC_START_SHIFT;
pub const CS43130_HPLOAD_DC_START: u32 = 1;

/* Reg CS43130_SP_BITSIZE */
pub const CS43130_SP_BIT_SIZE_8: u32 = 0x03;
pub const CS43130_SP_BIT_SIZE_16: u32 = 0x02;
pub const CS43130_SP_BIT_SIZE_24: u32 = 0x01;
pub const CS43130_SP_BIT_SIZE_32: u32 = 0x00;

/* Reg CS43130_SP_CH_SZ_EN */
pub const CS43130_CH_BIT_SIZE_8: u32 = 0x00;
pub const CS43130_CH_BIT_SIZE_16: u32 = 0x01;
pub const CS43130_CH_BIT_SIZE_24: u32 = 0x02;
pub const CS43130_CH_BIT_SIZE_32: u32 = 0x03;

/* PLL */
pub const CS43130_PLL_START_MASK: u32 = 0x01;
pub const CS43130_PLL_MODE_MASK: u32 = 0x02;
pub const CS43130_PLL_MODE_SHIFT: u32 = 1;
pub const CS43130_PLL_REF_PREDIV_MASK: u32 = 0x3;
pub const CS43130_SP_STP_MASK: u32 = 0x10;
pub const CS43130_SP_STP_SHIFT: u32 = 4;
pub const CS43130_SP_5050_MASK: u32 = 0x08;
pub const CS43130_SP_5050_SHIFT: u32 = 3;
pub const CS43130_SP_FSD_MASK: u32 = 0x07;
pub const CS43130_SP_MODE_MASK: u32 = 0x10;
pub const CS43130_SP_MODE_SHIFT: u32 = 4;
pub const CS43130_SP_SCPOL_OUT_MASK: u32 = 0x08;
pub const CS43130_SP_SCPOL_OUT_SHIFT: u32 = 3;
pub const CS43130_SP_SCPOL_IN_MASK: u32 = 0x04;
pub const CS43130_SP_SCPOL_IN_SHIFT: u32 = 2;
pub const CS43130_SP_LCPOL_OUT_MASK: u32 = 0x02;
pub const CS43130_SP_LCPOL_OUT_SHIFT: u32 = 1;
pub const CS43130_SP_LCPOL_IN_MASK: u32 = 0x01;
pub const CS43130_SP_LCPOL_IN_SHIFT: u32 = 0;

/* Reg CS43130_PWDN_CTL */
pub const CS43130_PDN_XSP_MASK: u32 = 0x80;
pub const CS43130_PDN_XSP_SHIFT: u32 = 7;
pub const CS43130_PDN_ASP_MASK: u32 = 0x40;
pub const CS43130_PDN_ASP_SHIFT: u32 = 6;
pub const CS43130_PDN_DSPIF_MASK: u32 = 0x20;
pub const CS43130_PDN_DSDIF_SHIFT: u32 = 5;
pub const CS43130_PDN_HP_MASK: u32 = 0x10;
pub const CS43130_PDN_HP_SHIFT: u32 = 4;
pub const CS43130_PDN_XTAL_MASK: u32 = 0x08;
pub const CS43130_PDN_XTAL_SHIFT: u32 = 3;
pub const CS43130_PDN_PLL_MASK: u32 = 0x04;
pub const CS43130_PDN_PLL_SHIFT: u32 = 2;
pub const CS43130_PDN_CLKOUT_MASK: u32 = 0x02;
pub const CS43130_PDN_CLKOUT_SHIFT: u32 = 1;

/* Reg CS43130_HP_OUT_CTL_1 */
pub const CS43130_HP_IN_EN_SHIFT: u32 = 3;
pub const CS43130_HP_IN_EN_MASK: u32 = 0x08;

/* Reg CS43130_PAD_INT_CFG */
pub const CS43130_ASP_3ST_MASK: u32 = 0x01;
pub const CS43130_XSP_3ST_MASK: u32 = 0x02;

/* Reg CS43130_PLL_SET_2 */
pub const CS43130_PLL_DIV_DATA_MASK: u32 = 0x000000FF;
pub const CS43130_PLL_DIV_FRAC_0_DATA_SHIFT: u32 = 0;

/* Reg CS43130_PLL_SET_3 */
pub const CS43130_PLL_DIV_FRAC_1_DATA_SHIFT: u32 = 8;

/* Reg CS43130_PLL_SET_4 */
pub const CS43130_PLL_DIV_FRAC_2_DATA_SHIFT: u32 = 16;

/* Reg CS43130_SP_DEN_1 */
pub const CS43130_SP_M_LSB_DATA_MASK: u32 = 0x00FF;
pub const CS43130_SP_M_LSB_DATA_SHIFT: u32 = 0;

/* Reg CS43130_SP_DEN_2 */
pub const CS43130_SP_M_MSB_DATA_MASK: u32 = 0xFF00;
pub const CS43130_SP_M_MSB_DATA_SHIFT: u32 = 8;

/* Reg CS43130_SP_NUM_1 */
pub const CS43130_SP_N_LSB_DATA_MASK: u32 = 0x00FF;
pub const CS43130_SP_N_LSB_DATA_SHIFT: u32 = 0;

/* Reg CS43130_SP_NUM_2 */
pub const CS43130_SP_N_MSB_DATA_MASK: u32 = 0xFF00;
pub const CS43130_SP_N_MSB_DATA_SHIFT: u32 = 8;

/* Reg CS43130_SP_LRCK_HI_TIME_1 */
pub const CS43130_SP_LCHI_DATA_MASK: u32 = 0x00FF;
pub const CS43130_SP_LCHI_LSB_DATA_SHIFT: u32 = 0;

/* Reg CS43130_SP_LRCK_HI_TIME_2 */
pub const CS43130_SP_LCHI_MSB_DATA_SHIFT: u32 = 8;

/* Reg CS43130_SP_LRCK_PERIOD_1 */
pub const CS43130_SP_LCPR_DATA_MASK: u32 = 0x00FF;
pub const CS43130_SP_LCPR_LSB_DATA_SHIFT: u32 = 0;

/* Reg CS43130_SP_LRCK_PERIOD_2 */
pub const CS43130_SP_LCPR_MSB_DATA_SHIFT: u32 = 8;

pub const CS43130_PCM_FORMATS: u32 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

pub const CS43130_DOP_FORMATS: u32 = SNDRV_PCM_FMTBIT_DSD_U16_LE
    | SNDRV_PCM_FMTBIT_DSD_U16_BE
    | SNDRV_PCM_FMTBIT_S24_LE;

/* Reg CS43130_CRYSTAL_SET */
pub const CS43130_XTAL_IBIAS_MASK: u32 = 0x07;

/* Reg CS43130_PATH_CTL_1 */
pub const CS43130_MUTE_MASK: u32 = 0x03;
pub const CS43130_MUTE_EN: u32 = 0x03;

/* Reg CS43130_DSD_INT_CFG */
pub const CS43130_DSD_MASTER: u32 = 0x04;

/* Reg CS43130_DSD_PATH_CTL_2 */
pub const CS43130_DSD_SRC_MASK: u32 = 0x60;
pub const CS43130_DSD_SRC_SHIFT: u32 = 5;
pub const CS43130_DSD_EN_SHIFT: u32 = 4;
pub const CS43130_DSD_SPEED_MASK: u32 = 0x04;
pub const CS43130_DSD_SPEED_SHIFT: u32 = 2;

/* Reg CS43130_DSD_PCM_MIX_CTL */
pub const CS43130_MIX_PCM_PREP_SHIFT: u32 = 1;
pub const CS43130_MIX_PCM_PREP_MASK: u32 = 0x02;
pub const CS43130_MIX_PCM_DSD_SHIFT: u32 = 0;
pub const CS43130_MIX_PCM_DSD_MASK: u32 = 0x01;

/* Reg CS43130_HP_MEAS_LOAD */
pub const CS43130_HP_MEAS_LOAD_MASK: u32 = 0x000000FF;
pub const CS43130_HP_MEAS_LOAD_1_SHIFT: u32 = 0;
pub const CS43130_HP_MEAS_LOAD_2_SHIFT: u32 = 8;

pub const CS43130_MCLK_22M: u32 = 22579200;
pub const CS43130_MCLK_24M: u32 = 24576000;

pub const CS43130_LINEOUT_LOAD: u32 = 5000;
pub const CS43130_JACK_LINEOUT: u32 = SND_JACK_MECHANICAL | SND_JACK_LINEOUT;
pub const CS43130_JACK_HEADPHONE: u32 = SND_JACK_MECHANICAL | SND_JACK_HEADPHONE;
pub const CS43130_JACK_MASK: u32 =
    SND_JACK_MECHANICAL | SND_JACK_LINEOUT | SND_JACK_HEADPHONE;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs43130_dsd_src {
    CS43130_DSD_SRC_DSD = 0,
    CS43130_DSD_SRC_ASP = 2,
    CS43130_DSD_SRC_XSP = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs43130_asp_rate {
    CS43130_ASP_SPRATE_32K = 0,
    CS43130_ASP_SPRATE_44_1K = 1,
    CS43130_ASP_SPRATE_48K = 2,
    CS43130_ASP_SPRATE_88_2K = 3,
    CS43130_ASP_SPRATE_96K = 4,
    CS43130_ASP_SPRATE_176_4K = 5,
    CS43130_ASP_SPRATE_192K = 6,
    CS43130_ASP_SPRATE_352_8K = 7,
    CS43130_ASP_SPRATE_384K = 8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs43130_mclk_src_sel {
    CS43130_MCLK_SRC_EXT = 0,
    CS43130_MCLK_SRC_PLL = 1,
    CS43130_MCLK_SRC_RCO = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs43130_mclk_int_freq {
    CS43130_MCLK_24P5 = 0,
    CS43130_MCLK_22P5 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs43130_xtal_ibias {
    CS43130_XTAL_UNUSED = -1,
    CS43130_XTAL_IBIAS_15UA = 2,
    CS43130_XTAL_IBIAS_12_5UA = 4,
    CS43130_XTAL_IBIAS_7_5UA = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs43130_dai_id {
    CS43130_ASP_PCM_DAI = 0,
    CS43130_ASP_DOP_DAI = 1,
    CS43130_XSP_DOP_DAI = 2,
    CS43130_XSP_DSD_DAI = 3,
    CS43130_DAI_ID_MAX = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs43130_clk_gen {
    pub mclk_int: ::core::ffi::c_uint,
    pub fs: ::core::ffi::c_int,
    pub v: u16_fract,
}

/* frm_size = 16 */
pub static cs43130_16_clk_gen: [cs43130_clk_gen; 18] = [
    cs43130_clk_gen { mclk_int: 22579200, fs: 32000, v: u16_fract { numerator: 10, denominator: 441 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 44100, v: u16_fract { numerator: 1, denominator: 32 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 48000, v: u16_fract { numerator: 5, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 88200, v: u16_fract { numerator: 1, denominator: 16 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 96000, v: u16_fract { numerator: 10, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 176400, v: u16_fract { numerator: 1, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 192000, v: u16_fract { numerator: 20, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 352800, v: u16_fract { numerator: 1, denominator: 4 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 384000, v: u16_fract { numerator: 40, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 32000, v: u16_fract { numerator: 1, denominator: 48 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 44100, v: u16_fract { numerator: 147, denominator: 5120 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 48000, v: u16_fract { numerator: 1, denominator: 32 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 88200, v: u16_fract { numerator: 147, denominator: 2560 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 96000, v: u16_fract { numerator: 1, denominator: 16 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 176400, v: u16_fract { numerator: 147, denominator: 1280 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 192000, v: u16_fract { numerator: 1, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 352800, v: u16_fract { numerator: 147, denominator: 640 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 384000, v: u16_fract { numerator: 1, denominator: 4 } },
];

/* frm_size = 32 */
pub static cs43130_32_clk_gen: [cs43130_clk_gen; 18] = [
    cs43130_clk_gen { mclk_int: 22579200, fs: 32000, v: u16_fract { numerator: 20, denominator: 441 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 44100, v: u16_fract { numerator: 1, denominator: 16 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 48000, v: u16_fract { numerator: 10, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 88200, v: u16_fract { numerator: 1, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 96000, v: u16_fract { numerator: 20, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 176400, v: u16_fract { numerator: 1, denominator: 4 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 192000, v: u16_fract { numerator: 40, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 352800, v: u16_fract { numerator: 1, denominator: 2 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 384000, v: u16_fract { numerator: 80, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 32000, v: u16_fract { numerator: 1, denominator: 24 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 44100, v: u16_fract { numerator: 147, denominator: 2560 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 48000, v: u16_fract { numerator: 1, denominator: 16 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 88200, v: u16_fract { numerator: 147, denominator: 1280 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 96000, v: u16_fract { numerator: 1, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 176400, v: u16_fract { numerator: 147, denominator: 640 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 192000, v: u16_fract { numerator: 1, denominator: 4 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 352800, v: u16_fract { numerator: 147, denominator: 320 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 384000, v: u16_fract { numerator: 1, denominator: 2 } },
];

/* frm_size = 48 */
pub static cs43130_48_clk_gen: [cs43130_clk_gen; 18] = [
    cs43130_clk_gen { mclk_int: 22579200, fs: 32000, v: u16_fract { numerator: 100, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 44100, v: u16_fract { numerator: 3, denominator: 32 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 48000, v: u16_fract { numerator: 5, denominator: 49 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 88200, v: u16_fract { numerator: 3, denominator: 16 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 96000, v: u16_fract { numerator: 10, denominator: 49 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 176400, v: u16_fract { numerator: 3, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 192000, v: u16_fract { numerator: 20, denominator: 49 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 352800, v: u16_fract { numerator: 3, denominator: 4 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 384000, v: u16_fract { numerator: 40, denominator: 49 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 32000, v: u16_fract { numerator: 1, denominator: 16 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 44100, v: u16_fract { numerator: 441, denominator: 5120 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 48000, v: u16_fract { numerator: 3, denominator: 32 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 88200, v: u16_fract { numerator: 441, denominator: 2560 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 96000, v: u16_fract { numerator: 3, denominator: 16 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 176400, v: u16_fract { numerator: 441, denominator: 1280 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 192000, v: u16_fract { numerator: 3, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 352800, v: u16_fract { numerator: 441, denominator: 640 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 384000, v: u16_fract { numerator: 3, denominator: 4 } },
];

/* frm_size = 64 */
pub static cs43130_64_clk_gen: [cs43130_clk_gen; 17] = [
    cs43130_clk_gen { mclk_int: 22579200, fs: 32000, v: u16_fract { numerator: 40, denominator: 441 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 44100, v: u16_fract { numerator: 1, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 48000, v: u16_fract { numerator: 20, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 88200, v: u16_fract { numerator: 1, denominator: 4 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 96000, v: u16_fract { numerator: 40, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 176400, v: u16_fract { numerator: 1, denominator: 2 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 192000, v: u16_fract { numerator: 80, denominator: 147 } },
    cs43130_clk_gen { mclk_int: 22579200, fs: 352800, v: u16_fract { numerator: 1, denominator: 1 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 32000, v: u16_fract { numerator: 1, denominator: 12 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 44100, v: u16_fract { numerator: 147, denominator: 1280 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 48000, v: u16_fract { numerator: 1, denominator: 8 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 88200, v: u16_fract { numerator: 147, denominator: 640 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 96000, v: u16_fract { numerator: 1, denominator: 4 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 176400, v: u16_fract { numerator: 147, denominator: 320 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 192000, v: u16_fract { numerator: 1, denominator: 2 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 352800, v: u16_fract { numerator: 147, denominator: 160 } },
    cs43130_clk_gen { mclk_int: 24576000, fs: 384000, v: u16_fract { numerator: 1, denominator: 1 } },
];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs43130_bitwidth_map {
    pub bitwidth: ::core::ffi::c_uint,
    pub sp_bit: u8,
    pub ch_bit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs43130_rate_map {
    pub fs: ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}

pub const HP_LEFT: u32 = 0;
pub const HP_RIGHT: u32 = 1;
pub const CS43130_AC_FREQ: usize = 10;
pub const CS43130_DC_THRESHOLD: usize = 2;

pub const CS43130_NUM_SUPPLIES: usize = 5;
pub static cs43130_supply_names: [*const ::core::ffi::c_char; CS43130_NUM_SUPPLIES] = [
    b"VA\0".as_ptr() as *const ::core::ffi::c_char,
    b"VP\0".as_ptr() as *const ::core::ffi::c_char,
    b"VCP\0".as_ptr() as *const ::core::ffi::c_char,
    b"VD\0".as_ptr() as *const ::core::ffi::c_char,
    b"VL\0".as_ptr() as *const ::core::ffi::c_char,
];

pub const CS43130_NUM_INT: usize = 5; /* number of interrupt status reg */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs43130_dai {
    pub sclk: ::core::ffi::c_uint,
    pub dai_format: ::core::ffi::c_uint,
    pub dai_mode: ::core::ffi::c_uint,
    pub dai_invert: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct cs43130_private {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; CS43130_NUM_SUPPLIES],
    pub reset_gpio: *mut gpio_desc,
    pub dev_id: ::core::ffi::c_uint, /* codec device ID */
    pub xtal_ibias: ::core::ffi::c_int,
    pub has_irq_line: bool,

    /* shared by both DAIs */
    pub clk_mutex: mutex,
    pub clk_req: ::core::ffi::c_int,
    pub pll_bypass: bool,
    pub xtal_rdy: completion,
    pub pll_rdy: completion,
    pub mclk: ::core::ffi::c_uint,
    pub mclk_int: ::core::ffi::c_uint,
    pub mclk_int_src: ::core::ffi::c_int,

    /* DAI specific */
    pub dais: [cs43130_dai; cs43130_dai_id::CS43130_DAI_ID_MAX as usize],

    /* HP load specific */
    pub dc_meas: bool,
    pub ac_meas: bool,
    pub hpload_done: bool,
    pub hpload_evt: completion,
    pub hpload_stat: ::core::ffi::c_uint,
    pub hpload_dc: [u16; 2],
    pub dc_threshold: [u16; CS43130_DC_THRESHOLD],
    pub ac_freq: [u16; CS43130_AC_FREQ],
    pub hpload_ac: [[u16; 2]; CS43130_AC_FREQ],
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub jack: snd_soc_jack,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
