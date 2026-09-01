/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8955.h  --  WM8904 ASoC driver
 *
 * Copyright 2009 Wolfson Microelectronics, plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */


pub const WM8955_CLK_MCLK: u32 = 1;

/*
 * Register values.
 */
pub const WM8955_LOUT1_VOLUME: u32 = 0x02;
pub const WM8955_ROUT1_VOLUME: u32 = 0x03;
pub const WM8955_DAC_CONTROL: u32 = 0x05;
pub const WM8955_AUDIO_INTERFACE: u32 = 0x07;
pub const WM8955_SAMPLE_RATE: u32 = 0x08;
pub const WM8955_LEFT_DAC_VOLUME: u32 = 0x0A;
pub const WM8955_RIGHT_DAC_VOLUME: u32 = 0x0B;
pub const WM8955_BASS_CONTROL: u32 = 0x0C;
pub const WM8955_TREBLE_CONTROL: u32 = 0x0D;
pub const WM8955_RESET: u32 = 0x0F;
pub const WM8955_ADDITIONAL_CONTROL_1: u32 = 0x17;
pub const WM8955_ADDITIONAL_CONTROL_2: u32 = 0x18;
pub const WM8955_POWER_MANAGEMENT_1: u32 = 0x19;
pub const WM8955_POWER_MANAGEMENT_2: u32 = 0x1A;
pub const WM8955_ADDITIONAL_CONTROL_3: u32 = 0x1B;
pub const WM8955_LEFT_OUT_MIX_1: u32 = 0x22;
pub const WM8955_LEFT_OUT_MIX_2: u32 = 0x23;
pub const WM8955_RIGHT_OUT_MIX_1: u32 = 0x24;
pub const WM8955_RIGHT_OUT_MIX_2: u32 = 0x25;
pub const WM8955_MONO_OUT_MIX_1: u32 = 0x26;
pub const WM8955_MONO_OUT_MIX_2: u32 = 0x27;
pub const WM8955_LOUT2_VOLUME: u32 = 0x28;
pub const WM8955_ROUT2_VOLUME: u32 = 0x29;
pub const WM8955_MONOOUT_VOLUME: u32 = 0x2A;
pub const WM8955_CLOCKING_PLL: u32 = 0x2B;
pub const WM8955_PLL_CONTROL_1: u32 = 0x2C;
pub const WM8955_PLL_CONTROL_2: u32 = 0x2D;
pub const WM8955_PLL_CONTROL_3: u32 = 0x2E;
pub const WM8955_PLL_CONTROL_4: u32 = 0x3B;

pub const WM8955_REGISTER_COUNT: u32 = 29;
pub const WM8955_MAX_REGISTER: u32 = 0x3B;

/*
 * Field Definitions.
 */

/*
 * R2 (0x02) - LOUT1 volume
 */
pub const WM8955_LO1VU: u32 = 0x0100;  /* LO1VU */
pub const WM8955_LO1VU_MASK: u32 = 0x0100;  /* LO1VU */
pub const WM8955_LO1VU_SHIFT: u32 = 8;  /* LO1VU */
pub const WM8955_LO1VU_WIDTH: u32 = 1;  /* LO1VU */
pub const WM8955_LO1ZC: u32 = 0x0080;  /* LO1ZC */
pub const WM8955_LO1ZC_MASK: u32 = 0x0080;  /* LO1ZC */
pub const WM8955_LO1ZC_SHIFT: u32 = 7;  /* LO1ZC */
pub const WM8955_LO1ZC_WIDTH: u32 = 1;  /* LO1ZC */
pub const WM8955_LOUTVOL_MASK: u32 = 0x007F;  /* LOUTVOL - [6:0] */
pub const WM8955_LOUTVOL_SHIFT: u32 = 0;  /* LOUTVOL - [6:0] */
pub const WM8955_LOUTVOL_WIDTH: u32 = 7;  /* LOUTVOL - [6:0] */

/*
 * R3 (0x03) - ROUT1 volume
 */
pub const WM8955_RO1VU: u32 = 0x0100;  /* RO1VU */
pub const WM8955_RO1VU_MASK: u32 = 0x0100;  /* RO1VU */
pub const WM8955_RO1VU_SHIFT: u32 = 8;  /* RO1VU */
pub const WM8955_RO1VU_WIDTH: u32 = 1;  /* RO1VU */
pub const WM8955_RO1ZC: u32 = 0x0080;  /* RO1ZC */
pub const WM8955_RO1ZC_MASK: u32 = 0x0080;  /* RO1ZC */
pub const WM8955_RO1ZC_SHIFT: u32 = 7;  /* RO1ZC */
pub const WM8955_RO1ZC_WIDTH: u32 = 1;  /* RO1ZC */
pub const WM8955_ROUTVOL_MASK: u32 = 0x007F;  /* ROUTVOL - [6:0] */
pub const WM8955_ROUTVOL_SHIFT: u32 = 0;  /* ROUTVOL - [6:0] */
pub const WM8955_ROUTVOL_WIDTH: u32 = 7;  /* ROUTVOL - [6:0] */

/*
 * R5 (0x05) - DAC Control
 */
pub const WM8955_DAT: u32 = 0x0080;  /* DAT */
pub const WM8955_DAT_MASK: u32 = 0x0080;  /* DAT */
pub const WM8955_DAT_SHIFT: u32 = 7;  /* DAT */
pub const WM8955_DAT_WIDTH: u32 = 1;  /* DAT */
pub const WM8955_DACMU: u32 = 0x0008;  /* DACMU */
pub const WM8955_DACMU_MASK: u32 = 0x0008;  /* DACMU */
pub const WM8955_DACMU_SHIFT: u32 = 3;  /* DACMU */
pub const WM8955_DACMU_WIDTH: u32 = 1;  /* DACMU */
pub const WM8955_DEEMPH_MASK: u32 = 0x0006;  /* DEEMPH - [2:1] */
pub const WM8955_DEEMPH_SHIFT: u32 = 1;  /* DEEMPH - [2:1] */
pub const WM8955_DEEMPH_WIDTH: u32 = 2;  /* DEEMPH - [2:1] */

/*
 * R7 (0x07) - Audio Interface
 */
pub const WM8955_BCLKINV: u32 = 0x0080;  /* BCLKINV */
pub const WM8955_BCLKINV_MASK: u32 = 0x0080;  /* BCLKINV */
pub const WM8955_BCLKINV_SHIFT: u32 = 7;  /* BCLKINV */
pub const WM8955_BCLKINV_WIDTH: u32 = 1;  /* BCLKINV */
pub const WM8955_MS: u32 = 0x0040;  /* MS */
pub const WM8955_MS_MASK: u32 = 0x0040;  /* MS */
pub const WM8955_MS_SHIFT: u32 = 6;  /* MS */
pub const WM8955_MS_WIDTH: u32 = 1;  /* MS */
pub const WM8955_LRSWAP: u32 = 0x0020;  /* LRSWAP */
pub const WM8955_LRSWAP_MASK: u32 = 0x0020;  /* LRSWAP */
pub const WM8955_LRSWAP_SHIFT: u32 = 5;  /* LRSWAP */
pub const WM8955_LRSWAP_WIDTH: u32 = 1;  /* LRSWAP */
pub const WM8955_LRP: u32 = 0x0010;  /* LRP */
pub const WM8955_LRP_MASK: u32 = 0x0010;  /* LRP */
pub const WM8955_LRP_SHIFT: u32 = 4;  /* LRP */
pub const WM8955_LRP_WIDTH: u32 = 1;  /* LRP */
pub const WM8955_WL_MASK: u32 = 0x000C;  /* WL - [3:2] */
pub const WM8955_WL_SHIFT: u32 = 2;  /* WL - [3:2] */
pub const WM8955_WL_WIDTH: u32 = 2;  /* WL - [3:2] */
pub const WM8955_FORMAT_MASK: u32 = 0x0003;  /* FORMAT - [1:0] */
pub const WM8955_FORMAT_SHIFT: u32 = 0;  /* FORMAT - [1:0] */
pub const WM8955_FORMAT_WIDTH: u32 = 2;  /* FORMAT - [1:0] */

/*
 * R8 (0x08) - Sample Rate
 */
pub const WM8955_BCLKDIV2: u32 = 0x0080;  /* BCLKDIV2 */
pub const WM8955_BCLKDIV2_MASK: u32 = 0x0080;  /* BCLKDIV2 */
pub const WM8955_BCLKDIV2_SHIFT: u32 = 7;  /* BCLKDIV2 */
pub const WM8955_BCLKDIV2_WIDTH: u32 = 1;  /* BCLKDIV2 */
pub const WM8955_MCLKDIV2: u32 = 0x0040;  /* MCLKDIV2 */
pub const WM8955_MCLKDIV2_MASK: u32 = 0x0040;  /* MCLKDIV2 */
pub const WM8955_MCLKDIV2_SHIFT: u32 = 6;  /* MCLKDIV2 */
pub const WM8955_MCLKDIV2_WIDTH: u32 = 1;  /* MCLKDIV2 */
pub const WM8955_SR_MASK: u32 = 0x003E;  /* SR - [5:1] */
pub const WM8955_SR_SHIFT: u32 = 1;  /* SR - [5:1] */
pub const WM8955_SR_WIDTH: u32 = 5;  /* SR - [5:1] */
pub const WM8955_USB: u32 = 0x0001;  /* USB */
pub const WM8955_USB_MASK: u32 = 0x0001;  /* USB */
pub const WM8955_USB_SHIFT: u32 = 0;  /* USB */
pub const WM8955_USB_WIDTH: u32 = 1;  /* USB */

/*
 * R10 (0x0A) - Left DAC volume
 */
pub const WM8955_LDVU: u32 = 0x0100;  /* LDVU */
pub const WM8955_LDVU_MASK: u32 = 0x0100;  /* LDVU */
pub const WM8955_LDVU_SHIFT: u32 = 8;  /* LDVU */
pub const WM8955_LDVU_WIDTH: u32 = 1;  /* LDVU */
pub const WM8955_LDACVOL_MASK: u32 = 0x00FF;  /* LDACVOL - [7:0] */
pub const WM8955_LDACVOL_SHIFT: u32 = 0;  /* LDACVOL - [7:0] */
pub const WM8955_LDACVOL_WIDTH: u32 = 8;  /* LDACVOL - [7:0] */

/*
 * R11 (0x0B) - Right DAC volume
 */
pub const WM8955_RDVU: u32 = 0x0100;  /* RDVU */
pub const WM8955_RDVU_MASK: u32 = 0x0100;  /* RDVU */
pub const WM8955_RDVU_SHIFT: u32 = 8;  /* RDVU */
pub const WM8955_RDVU_WIDTH: u32 = 1;  /* RDVU */
pub const WM8955_RDACVOL_MASK: u32 = 0x00FF;  /* RDACVOL - [7:0] */
pub const WM8955_RDACVOL_SHIFT: u32 = 0;  /* RDACVOL - [7:0] */
pub const WM8955_RDACVOL_WIDTH: u32 = 8;  /* RDACVOL - [7:0] */

/*
 * R12 (0x0C) - Bass control
 */
pub const WM8955_BB: u32 = 0x0080;  /* BB */
pub const WM8955_BB_MASK: u32 = 0x0080;  /* BB */
pub const WM8955_BB_SHIFT: u32 = 7;  /* BB */
pub const WM8955_BB_WIDTH: u32 = 1;  /* BB */
pub const WM8955_BC: u32 = 0x0040;  /* BC */
pub const WM8955_BC_MASK: u32 = 0x0040;  /* BC */
pub const WM8955_BC_SHIFT: u32 = 6;  /* BC */
pub const WM8955_BC_WIDTH: u32 = 1;  /* BC */
pub const WM8955_BASS_MASK: u32 = 0x000F;  /* BASS - [3:0] */
pub const WM8955_BASS_SHIFT: u32 = 0;  /* BASS - [3:0] */
pub const WM8955_BASS_WIDTH: u32 = 4;  /* BASS - [3:0] */

/*
 * R13 (0x0D) - Treble control
 */
pub const WM8955_TC: u32 = 0x0040;  /* TC */
pub const WM8955_TC_MASK: u32 = 0x0040;  /* TC */
pub const WM8955_TC_SHIFT: u32 = 6;  /* TC */
pub const WM8955_TC_WIDTH: u32 = 1;  /* TC */
pub const WM8955_TRBL_MASK: u32 = 0x000F;  /* TRBL - [3:0] */
pub const WM8955_TRBL_SHIFT: u32 = 0;  /* TRBL - [3:0] */
pub const WM8955_TRBL_WIDTH: u32 = 4;  /* TRBL - [3:0] */

/*
 * R15 (0x0F) - Reset
 */
pub const WM8955_RESET_MASK: u32 = 0x01FF;  /* RESET - [8:0] */
pub const WM8955_RESET_SHIFT: u32 = 0;  /* RESET - [8:0] */
pub const WM8955_RESET_WIDTH: u32 = 9;  /* RESET - [8:0] */

/*
 * R23 (0x17) - Additional control (1)
 */
pub const WM8955_TSDEN: u32 = 0x0100;  /* TSDEN */
pub const WM8955_TSDEN_MASK: u32 = 0x0100;  /* TSDEN */
pub const WM8955_TSDEN_SHIFT: u32 = 8;  /* TSDEN */
pub const WM8955_TSDEN_WIDTH: u32 = 1;  /* TSDEN */
pub const WM8955_VSEL_MASK: u32 = 0x00C0;  /* VSEL - [7:6] */
pub const WM8955_VSEL_SHIFT: u32 = 6;  /* VSEL - [7:6] */
pub const WM8955_VSEL_WIDTH: u32 = 2;  /* VSEL - [7:6] */
pub const WM8955_DMONOMIX_MASK: u32 = 0x0030;  /* DMONOMIX - [5:4] */
pub const WM8955_DMONOMIX_SHIFT: u32 = 4;  /* DMONOMIX - [5:4] */
pub const WM8955_DMONOMIX_WIDTH: u32 = 2;  /* DMONOMIX - [5:4] */
pub const WM8955_DACINV: u32 = 0x0002;  /* DACINV */
pub const WM8955_DACINV_MASK: u32 = 0x0002;  /* DACINV */
pub const WM8955_DACINV_SHIFT: u32 = 1;  /* DACINV */
pub const WM8955_DACINV_WIDTH: u32 = 1;  /* DACINV */
pub const WM8955_TOEN: u32 = 0x0001;  /* TOEN */
pub const WM8955_TOEN_MASK: u32 = 0x0001;  /* TOEN */
pub const WM8955_TOEN_SHIFT: u32 = 0;  /* TOEN */
pub const WM8955_TOEN_WIDTH: u32 = 1;  /* TOEN */

/*
 * R24 (0x18) - Additional control (2)
 */
pub const WM8955_OUT3SW_MASK: u32 = 0x0180;  /* OUT3SW - [8:7] */
pub const WM8955_OUT3SW_SHIFT: u32 = 7;  /* OUT3SW - [8:7] */
pub const WM8955_OUT3SW_WIDTH: u32 = 2;  /* OUT3SW - [8:7] */
pub const WM8955_ROUT2INV: u32 = 0x0010;  /* ROUT2INV */
pub const WM8955_ROUT2INV_MASK: u32 = 0x0010;  /* ROUT2INV */
pub const WM8955_ROUT2INV_SHIFT: u32 = 4;  /* ROUT2INV */
pub const WM8955_ROUT2INV_WIDTH: u32 = 1;  /* ROUT2INV */
pub const WM8955_DACOSR: u32 = 0x0001;  /* DACOSR */
pub const WM8955_DACOSR_MASK: u32 = 0x0001;  /* DACOSR */
pub const WM8955_DACOSR_SHIFT: u32 = 0;  /* DACOSR */
pub const WM8955_DACOSR_WIDTH: u32 = 1;  /* DACOSR */

/*
 * R25 (0x19) - Power Management (1)
 */
pub const WM8955_VMIDSEL_MASK: u32 = 0x0180;  /* VMIDSEL - [8:7] */
pub const WM8955_VMIDSEL_SHIFT: u32 = 7;  /* VMIDSEL - [8:7] */
pub const WM8955_VMIDSEL_WIDTH: u32 = 2;  /* VMIDSEL - [8:7] */
pub const WM8955_VREF: u32 = 0x0040;  /* VREF */
pub const WM8955_VREF_MASK: u32 = 0x0040;  /* VREF */
pub const WM8955_VREF_SHIFT: u32 = 6;  /* VREF */
pub const WM8955_VREF_WIDTH: u32 = 1;  /* VREF */
pub const WM8955_DIGENB: u32 = 0x0001;  /* DIGENB */
pub const WM8955_DIGENB_MASK: u32 = 0x0001;  /* DIGENB */
pub const WM8955_DIGENB_SHIFT: u32 = 0;  /* DIGENB */
pub const WM8955_DIGENB_WIDTH: u32 = 1;  /* DIGENB */

/*
 * R26 (0x1A) - Power Management (2)
 */
pub const WM8955_DACL: u32 = 0x0100;  /* DACL */
pub const WM8955_DACL_MASK: u32 = 0x0100;  /* DACL */
pub const WM8955_DACL_SHIFT: u32 = 8;  /* DACL */
pub const WM8955_DACL_WIDTH: u32 = 1;  /* DACL */
pub const WM8955_DACR: u32 = 0x0080;  /* DACR */
pub const WM8955_DACR_MASK: u32 = 0x0080;  /* DACR */
pub const WM8955_DACR_SHIFT: u32 = 7;  /* DACR */
pub const WM8955_DACR_WIDTH: u32 = 1;  /* DACR */
pub const WM8955_LOUT1: u32 = 0x0040;  /* LOUT1 */
pub const WM8955_LOUT1_MASK: u32 = 0x0040;  /* LOUT1 */
pub const WM8955_LOUT1_SHIFT: u32 = 6;  /* LOUT1 */
pub const WM8955_LOUT1_WIDTH: u32 = 1;  /* LOUT1 */
pub const WM8955_ROUT1: u32 = 0x0020;  /* ROUT1 */
pub const WM8955_ROUT1_MASK: u32 = 0x0020;  /* ROUT1 */
pub const WM8955_ROUT1_SHIFT: u32 = 5;  /* ROUT1 */
pub const WM8955_ROUT1_WIDTH: u32 = 1;  /* ROUT1 */
pub const WM8955_LOUT2: u32 = 0x0010;  /* LOUT2 */
pub const WM8955_LOUT2_MASK: u32 = 0x0010;  /* LOUT2 */
pub const WM8955_LOUT2_SHIFT: u32 = 4;  /* LOUT2 */
pub const WM8955_LOUT2_WIDTH: u32 = 1;  /* LOUT2 */
pub const WM8955_ROUT2: u32 = 0x0008;  /* ROUT2 */
pub const WM8955_ROUT2_MASK: u32 = 0x0008;  /* ROUT2 */
pub const WM8955_ROUT2_SHIFT: u32 = 3;  /* ROUT2 */
pub const WM8955_ROUT2_WIDTH: u32 = 1;  /* ROUT2 */
pub const WM8955_MONO: u32 = 0x0004;  /* MONO */
pub const WM8955_MONO_MASK: u32 = 0x0004;  /* MONO */
pub const WM8955_MONO_SHIFT: u32 = 2;  /* MONO */
pub const WM8955_MONO_WIDTH: u32 = 1;  /* MONO */
pub const WM8955_OUT3: u32 = 0x0002;  /* OUT3 */
pub const WM8955_OUT3_MASK: u32 = 0x0002;  /* OUT3 */
pub const WM8955_OUT3_SHIFT: u32 = 1;  /* OUT3 */
pub const WM8955_OUT3_WIDTH: u32 = 1;  /* OUT3 */

/*
 * R27 (0x1B) - Additional Control (3)
 */
pub const WM8955_VROI: u32 = 0x0040;  /* VROI */
pub const WM8955_VROI_MASK: u32 = 0x0040;  /* VROI */
pub const WM8955_VROI_SHIFT: u32 = 6;  /* VROI */
pub const WM8955_VROI_WIDTH: u32 = 1;  /* VROI */

/*
 * R34 (0x22) - Left out Mix (1)
 */
pub const WM8955_LD2LO: u32 = 0x0100;  /* LD2LO */
pub const WM8955_LD2LO_MASK: u32 = 0x0100;  /* LD2LO */
pub const WM8955_LD2LO_SHIFT: u32 = 8;  /* LD2LO */
pub const WM8955_LD2LO_WIDTH: u32 = 1;  /* LD2LO */
pub const WM8955_LI2LO: u32 = 0x0080;  /* LI2LO */
pub const WM8955_LI2LO_MASK: u32 = 0x0080;  /* LI2LO */
pub const WM8955_LI2LO_SHIFT: u32 = 7;  /* LI2LO */
pub const WM8955_LI2LO_WIDTH: u32 = 1;  /* LI2LO */
pub const WM8955_LI2LOVOL_MASK: u32 = 0x0070;  /* LI2LOVOL - [6:4] */
pub const WM8955_LI2LOVOL_SHIFT: u32 = 4;  /* LI2LOVOL - [6:4] */
pub const WM8955_LI2LOVOL_WIDTH: u32 = 3;  /* LI2LOVOL - [6:4] */

/*
 * R35 (0x23) - Left out Mix (2)
 */
pub const WM8955_RD2LO: u32 = 0x0100;  /* RD2LO */
pub const WM8955_RD2LO_MASK: u32 = 0x0100;  /* RD2LO */
pub const WM8955_RD2LO_SHIFT: u32 = 8;  /* RD2LO */
pub const WM8955_RD2LO_WIDTH: u32 = 1;  /* RD2LO */
pub const WM8955_RI2LO: u32 = 0x0080;  /* RI2LO */
pub const WM8955_RI2LO_MASK: u32 = 0x0080;  /* RI2LO */
pub const WM8955_RI2LO_SHIFT: u32 = 7;  /* RI2LO */
pub const WM8955_RI2LO_WIDTH: u32 = 1;  /* RI2LO */
pub const WM8955_RI2LOVOL_MASK: u32 = 0x0070;  /* RI2LOVOL - [6:4] */
pub const WM8955_RI2LOVOL_SHIFT: u32 = 4;  /* RI2LOVOL - [6:4] */
pub const WM8955_RI2LOVOL_WIDTH: u32 = 3;  /* RI2LOVOL - [6:4] */

/*
 * R36 (0x24) - Right out Mix (1)
 */
pub const WM8955_LD2RO: u32 = 0x0100;  /* LD2RO */
pub const WM8955_LD2RO_MASK: u32 = 0x0100;  /* LD2RO */
pub const WM8955_LD2RO_SHIFT: u32 = 8;  /* LD2RO */
pub const WM8955_LD2RO_WIDTH: u32 = 1;  /* LD2RO */
pub const WM8955_LI2RO: u32 = 0x0080;  /* LI2RO */
pub const WM8955_LI2RO_MASK: u32 = 0x0080;  /* LI2RO */
pub const WM8955_LI2RO_SHIFT: u32 = 7;  /* LI2RO */
pub const WM8955_LI2RO_WIDTH: u32 = 1;  /* LI2RO */
pub const WM8955_LI2ROVOL_MASK: u32 = 0x0070;  /* LI2ROVOL - [6:4] */
pub const WM8955_LI2ROVOL_SHIFT: u32 = 4;  /* LI2ROVOL - [6:4] */
pub const WM8955_LI2ROVOL_WIDTH: u32 = 3;  /* LI2ROVOL - [6:4] */

/*
 * R37 (0x25) - Right Out Mix (2)
 */
pub const WM8955_RD2RO: u32 = 0x0100;  /* RD2RO */
pub const WM8955_RD2RO_MASK: u32 = 0x0100;  /* RD2RO */
pub const WM8955_RD2RO_SHIFT: u32 = 8;  /* RD2RO */
pub const WM8955_RD2RO_WIDTH: u32 = 1;  /* RD2RO */
pub const WM8955_RI2RO: u32 = 0x0080;  /* RI2RO */
pub const WM8955_RI2RO_MASK: u32 = 0x0080;  /* RI2RO */
pub const WM8955_RI2RO_SHIFT: u32 = 7;  /* RI2RO */
pub const WM8955_RI2RO_WIDTH: u32 = 1;  /* RI2RO */
pub const WM8955_RI2ROVOL_MASK: u32 = 0x0070;  /* RI2ROVOL - [6:4] */
pub const WM8955_RI2ROVOL_SHIFT: u32 = 4;  /* RI2ROVOL - [6:4] */
pub const WM8955_RI2ROVOL_WIDTH: u32 = 3;  /* RI2ROVOL - [6:4] */

/*
 * R38 (0x26) - Mono out Mix (1)
 */
pub const WM8955_LD2MO: u32 = 0x0100;  /* LD2MO */
pub const WM8955_LD2MO_MASK: u32 = 0x0100;  /* LD2MO */
pub const WM8955_LD2MO_SHIFT: u32 = 8;  /* LD2MO */
pub const WM8955_LD2MO_WIDTH: u32 = 1;  /* LD2MO */
pub const WM8955_LI2MO: u32 = 0x0080;  /* LI2MO */
pub const WM8955_LI2MO_MASK: u32 = 0x0080;  /* LI2MO */
pub const WM8955_LI2MO_SHIFT: u32 = 7;  /* LI2MO */
pub const WM8955_LI2MO_WIDTH: u32 = 1;  /* LI2MO */
pub const WM8955_LI2MOVOL_MASK: u32 = 0x0070;  /* LI2MOVOL - [6:4] */
pub const WM8955_LI2MOVOL_SHIFT: u32 = 4;  /* LI2MOVOL - [6:4] */
pub const WM8955_LI2MOVOL_WIDTH: u32 = 3;  /* LI2MOVOL - [6:4] */
pub const WM8955_DMEN: u32 = 0x0001;  /* DMEN */
pub const WM8955_DMEN_MASK: u32 = 0x0001;  /* DMEN */
pub const WM8955_DMEN_SHIFT: u32 = 0;  /* DMEN */
pub const WM8955_DMEN_WIDTH: u32 = 1;  /* DMEN */

/*
 * R39 (0x27) - Mono out Mix (2)
 */
pub const WM8955_RD2MO: u32 = 0x0100;  /* RD2MO */
pub const WM8955_RD2MO_MASK: u32 = 0x0100;  /* RD2MO */
pub const WM8955_RD2MO_SHIFT: u32 = 8;  /* RD2MO */
pub const WM8955_RD2MO_WIDTH: u32 = 1;  /* RD2MO */
pub const WM8955_RI2MO: u32 = 0x0080;  /* RI2MO */
pub const WM8955_RI2MO_MASK: u32 = 0x0080;  /* RI2MO */
pub const WM8955_RI2MO_SHIFT: u32 = 7;  /* RI2MO */
pub const WM8955_RI2MO_WIDTH: u32 = 1;  /* RI2MO */
pub const WM8955_RI2MOVOL_MASK: u32 = 0x0070;  /* RI2MOVOL - [6:4] */
pub const WM8955_RI2MOVOL_SHIFT: u32 = 4;  /* RI2MOVOL - [6:4] */
pub const WM8955_RI2MOVOL_WIDTH: u32 = 3;  /* RI2MOVOL - [6:4] */

/*
 * R40 (0x28) - LOUT2 volume
 */
pub const WM8955_LO2VU: u32 = 0x0100;  /* LO2VU */
pub const WM8955_LO2VU_MASK: u32 = 0x0100;  /* LO2VU */
pub const WM8955_LO2VU_SHIFT: u32 = 8;  /* LO2VU */
pub const WM8955_LO2VU_WIDTH: u32 = 1;  /* LO2VU */
pub const WM8955_LO2ZC: u32 = 0x0080;  /* LO2ZC */
pub const WM8955_LO2ZC_MASK: u32 = 0x0080;  /* LO2ZC */
pub const WM8955_LO2ZC_SHIFT: u32 = 7;  /* LO2ZC */
pub const WM8955_LO2ZC_WIDTH: u32 = 1;  /* LO2ZC */
pub const WM8955_LOUT2VOL_MASK: u32 = 0x007F;  /* LOUT2VOL - [6:0] */
pub const WM8955_LOUT2VOL_SHIFT: u32 = 0;  /* LOUT2VOL - [6:0] */
pub const WM8955_LOUT2VOL_WIDTH: u32 = 7;  /* LOUT2VOL - [6:0] */

/*
 * R41 (0x29) - ROUT2 volume
 */
pub const WM8955_RO2VU: u32 = 0x0100;  /* RO2VU */
pub const WM8955_RO2VU_MASK: u32 = 0x0100;  /* RO2VU */
pub const WM8955_RO2VU_SHIFT: u32 = 8;  /* RO2VU */
pub const WM8955_RO2VU_WIDTH: u32 = 1;  /* RO2VU */
pub const WM8955_RO2ZC: u32 = 0x0080;  /* RO2ZC */
pub const WM8955_RO2ZC_MASK: u32 = 0x0080;  /* RO2ZC */
pub const WM8955_RO2ZC_SHIFT: u32 = 7;  /* RO2ZC */
pub const WM8955_RO2ZC_WIDTH: u32 = 1;  /* RO2ZC */
pub const WM8955_ROUT2VOL_MASK: u32 = 0x007F;  /* ROUT2VOL - [6:0] */
pub const WM8955_ROUT2VOL_SHIFT: u32 = 0;  /* ROUT2VOL - [6:0] */
pub const WM8955_ROUT2VOL_WIDTH: u32 = 7;  /* ROUT2VOL - [6:0] */

/*
 * R42 (0x2A) - MONOOUT volume
 */
pub const WM8955_MOZC: u32 = 0x0080;  /* MOZC */
pub const WM8955_MOZC_MASK: u32 = 0x0080;  /* MOZC */
pub const WM8955_MOZC_SHIFT: u32 = 7;  /* MOZC */
pub const WM8955_MOZC_WIDTH: u32 = 1;  /* MOZC */
pub const WM8955_MOUTVOL_MASK: u32 = 0x007F;  /* MOUTVOL - [6:0] */
pub const WM8955_MOUTVOL_SHIFT: u32 = 0;  /* MOUTVOL - [6:0] */
pub const WM8955_MOUTVOL_WIDTH: u32 = 7;  /* MOUTVOL - [6:0] */

/*
 * R43 (0x2B) - Clocking / PLL
 */
pub const WM8955_MCLKSEL: u32 = 0x0100;  /* MCLKSEL */
pub const WM8955_MCLKSEL_MASK: u32 = 0x0100;  /* MCLKSEL */
pub const WM8955_MCLKSEL_SHIFT: u32 = 8;  /* MCLKSEL */
pub const WM8955_MCLKSEL_WIDTH: u32 = 1;  /* MCLKSEL */
pub const WM8955_PLLOUTDIV2: u32 = 0x0020;  /* PLLOUTDIV2 */
pub const WM8955_PLLOUTDIV2_MASK: u32 = 0x0020;  /* PLLOUTDIV2 */
pub const WM8955_PLLOUTDIV2_SHIFT: u32 = 5;  /* PLLOUTDIV2 */
pub const WM8955_PLLOUTDIV2_WIDTH: u32 = 1;  /* PLLOUTDIV2 */
pub const WM8955_PLL_RB: u32 = 0x0010;  /* PLL_RB */
pub const WM8955_PLL_RB_MASK: u32 = 0x0010;  /* PLL_RB */
pub const WM8955_PLL_RB_SHIFT: u32 = 4;  /* PLL_RB */
pub const WM8955_PLL_RB_WIDTH: u32 = 1;  /* PLL_RB */
pub const WM8955_PLLEN: u32 = 0x0008;  /* PLLEN */
pub const WM8955_PLLEN_MASK: u32 = 0x0008;  /* PLLEN */
pub const WM8955_PLLEN_SHIFT: u32 = 3;  /* PLLEN */
pub const WM8955_PLLEN_WIDTH: u32 = 1;  /* PLLEN */

/*
 * R44 (0x2C) - PLL Control 1
 */
pub const WM8955_N_MASK: u32 = 0x01E0;  /* N - [8:5] */
pub const WM8955_N_SHIFT: u32 = 5;  /* N - [8:5] */
pub const WM8955_N_WIDTH: u32 = 4;  /* N - [8:5] */
pub const WM8955_K_21_18_MASK: u32 = 0x000F;  /* K(21:18) - [3:0] */
pub const WM8955_K_21_18_SHIFT: u32 = 0;  /* K(21:18) - [3:0] */
pub const WM8955_K_21_18_WIDTH: u32 = 4;  /* K(21:18) - [3:0] */

/*
 * R45 (0x2D) - PLL Control 2
 */
pub const WM8955_K_17_9_MASK: u32 = 0x01FF;  /* K(17:9) - [8:0] */
pub const WM8955_K_17_9_SHIFT: u32 = 0;  /* K(17:9) - [8:0] */
pub const WM8955_K_17_9_WIDTH: u32 = 9;  /* K(17:9) - [8:0] */

/*
 * R46 (0x2E) - PLL Control 3
 */
pub const WM8955_K_8_0_MASK: u32 = 0x01FF;  /* K(8:0) - [8:0] */
pub const WM8955_K_8_0_SHIFT: u32 = 0;  /* K(8:0) - [8:0] */
pub const WM8955_K_8_0_WIDTH: u32 = 9;  /* K(8:0) - [8:0] */

/*
 * R59 (0x3B) - PLL Control 4
 */
pub const WM8955_KEN: u32 = 0x0080;  /* KEN */
pub const WM8955_KEN_MASK: u32 = 0x0080;  /* KEN */
pub const WM8955_KEN_SHIFT: u32 = 7;  /* KEN */
pub const WM8955_KEN_WIDTH: u32 = 1;  /* KEN */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
