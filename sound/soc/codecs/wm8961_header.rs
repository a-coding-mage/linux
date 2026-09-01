/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8961.h  --  WM8961 Soc Audio driver
 */


// C dependency: #include <sound/soc.h>

pub const WM8961_BCLK: u32 = 1;
pub const WM8961_LRCLK: u32 = 2;

pub const WM8961_BCLK_DIV_1: u32 = 0;
pub const WM8961_BCLK_DIV_1_5: u32 = 1;
pub const WM8961_BCLK_DIV_2: u32 = 2;
pub const WM8961_BCLK_DIV_3: u32 = 3;
pub const WM8961_BCLK_DIV_4: u32 = 4;
pub const WM8961_BCLK_DIV_5_5: u32 = 5;
pub const WM8961_BCLK_DIV_6: u32 = 6;
pub const WM8961_BCLK_DIV_8: u32 = 7;
pub const WM8961_BCLK_DIV_11: u32 = 8;
pub const WM8961_BCLK_DIV_12: u32 = 9;
pub const WM8961_BCLK_DIV_16: u32 = 10;
pub const WM8961_BCLK_DIV_24: u32 = 11;
pub const WM8961_BCLK_DIV_32: u32 = 13;


/*
 * Register values.
 */
pub const WM8961_LEFT_INPUT_VOLUME: u32 = 0x00;
pub const WM8961_RIGHT_INPUT_VOLUME: u32 = 0x01;
pub const WM8961_LOUT1_VOLUME: u32 = 0x02;
pub const WM8961_ROUT1_VOLUME: u32 = 0x03;
pub const WM8961_CLOCKING1: u32 = 0x04;
pub const WM8961_ADC_DAC_CONTROL_1: u32 = 0x05;
pub const WM8961_ADC_DAC_CONTROL_2: u32 = 0x06;
pub const WM8961_AUDIO_INTERFACE_0: u32 = 0x07;
pub const WM8961_CLOCKING2: u32 = 0x08;
pub const WM8961_AUDIO_INTERFACE_1: u32 = 0x09;
pub const WM8961_LEFT_DAC_VOLUME: u32 = 0x0A;
pub const WM8961_RIGHT_DAC_VOLUME: u32 = 0x0B;
pub const WM8961_AUDIO_INTERFACE_2: u32 = 0x0E;
pub const WM8961_SOFTWARE_RESET: u32 = 0x0F;
pub const WM8961_ALC1: u32 = 0x11;
pub const WM8961_ALC2: u32 = 0x12;
pub const WM8961_ALC3: u32 = 0x13;
pub const WM8961_NOISE_GATE: u32 = 0x14;
pub const WM8961_LEFT_ADC_VOLUME: u32 = 0x15;
pub const WM8961_RIGHT_ADC_VOLUME: u32 = 0x16;
pub const WM8961_ADDITIONAL_CONTROL_1: u32 = 0x17;
pub const WM8961_ADDITIONAL_CONTROL_2: u32 = 0x18;
pub const WM8961_PWR_MGMT_1: u32 = 0x19;
pub const WM8961_PWR_MGMT_2: u32 = 0x1A;
pub const WM8961_ADDITIONAL_CONTROL_3: u32 = 0x1B;
pub const WM8961_ANTI_POP: u32 = 0x1C;
pub const WM8961_CLOCKING_3: u32 = 0x1E;
pub const WM8961_ADCL_SIGNAL_PATH: u32 = 0x20;
pub const WM8961_ADCR_SIGNAL_PATH: u32 = 0x21;
pub const WM8961_LOUT2_VOLUME: u32 = 0x28;
pub const WM8961_ROUT2_VOLUME: u32 = 0x29;
pub const WM8961_PWR_MGMT_3: u32 = 0x2F;
pub const WM8961_ADDITIONAL_CONTROL_4: u32 = 0x30;
pub const WM8961_CLASS_D_CONTROL_1: u32 = 0x31;
pub const WM8961_CLASS_D_CONTROL_2: u32 = 0x33;
pub const WM8961_CLOCKING_4: u32 = 0x38;
pub const WM8961_DSP_SIDETONE_0: u32 = 0x39;
pub const WM8961_DSP_SIDETONE_1: u32 = 0x3A;
pub const WM8961_DC_SERVO_0: u32 = 0x3C;
pub const WM8961_DC_SERVO_1: u32 = 0x3D;
pub const WM8961_DC_SERVO_3: u32 = 0x3F;
pub const WM8961_DC_SERVO_5: u32 = 0x41;
pub const WM8961_ANALOGUE_PGA_BIAS: u32 = 0x44;
pub const WM8961_ANALOGUE_HP_0: u32 = 0x45;
pub const WM8961_ANALOGUE_HP_2: u32 = 0x47;
pub const WM8961_CHARGE_PUMP_1: u32 = 0x48;
pub const WM8961_CHARGE_PUMP_B: u32 = 0x52;
pub const WM8961_WRITE_SEQUENCER_1: u32 = 0x57;
pub const WM8961_WRITE_SEQUENCER_2: u32 = 0x58;
pub const WM8961_WRITE_SEQUENCER_3: u32 = 0x59;
pub const WM8961_WRITE_SEQUENCER_4: u32 = 0x5A;
pub const WM8961_WRITE_SEQUENCER_5: u32 = 0x5B;
pub const WM8961_WRITE_SEQUENCER_6: u32 = 0x5C;
pub const WM8961_WRITE_SEQUENCER_7: u32 = 0x5D;
pub const WM8961_GENERAL_TEST_1: u32 = 0xFC;


/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - Left Input volume
 */
pub const WM8961_IPVU: u32 = 0x0100; /* IPVU */
pub const WM8961_IPVU_MASK: u32 = 0x0100; /* IPVU */
pub const WM8961_IPVU_SHIFT: u32 = 8; /* IPVU */
pub const WM8961_IPVU_WIDTH: u32 = 1; /* IPVU */
pub const WM8961_LINMUTE: u32 = 0x0080; /* LINMUTE */
pub const WM8961_LINMUTE_MASK: u32 = 0x0080; /* LINMUTE */
pub const WM8961_LINMUTE_SHIFT: u32 = 7; /* LINMUTE */
pub const WM8961_LINMUTE_WIDTH: u32 = 1; /* LINMUTE */
pub const WM8961_LIZC: u32 = 0x0040; /* LIZC */
pub const WM8961_LIZC_MASK: u32 = 0x0040; /* LIZC */
pub const WM8961_LIZC_SHIFT: u32 = 6; /* LIZC */
pub const WM8961_LIZC_WIDTH: u32 = 1; /* LIZC */
pub const WM8961_LINVOL_MASK: u32 = 0x003F; /* LINVOL - [5:0] */
pub const WM8961_LINVOL_SHIFT: u32 = 0; /* LINVOL - [5:0] */
pub const WM8961_LINVOL_WIDTH: u32 = 6; /* LINVOL - [5:0] */

/*
 * R1 (0x01) - Right Input volume
 */
pub const WM8961_DEVICE_ID_MASK: u32 = 0xF000; /* DEVICE_ID - [15:12] */
pub const WM8961_DEVICE_ID_SHIFT: u32 = 12; /* DEVICE_ID - [15:12] */
pub const WM8961_DEVICE_ID_WIDTH: u32 = 4; /* DEVICE_ID - [15:12] */
pub const WM8961_CHIP_REV_MASK: u32 = 0x0E00; /* CHIP_REV - [11:9] */
pub const WM8961_CHIP_REV_SHIFT: u32 = 9; /* CHIP_REV - [11:9] */
pub const WM8961_CHIP_REV_WIDTH: u32 = 3; /* CHIP_REV - [11:9] */
// Duplicate C define: WM8961_IPVU = 0x0100; /* IPVU */
// Duplicate C define: WM8961_IPVU_MASK = 0x0100; /* IPVU */
// Duplicate C define: WM8961_IPVU_SHIFT = 8; /* IPVU */
// Duplicate C define: WM8961_IPVU_WIDTH = 1; /* IPVU */
pub const WM8961_RINMUTE: u32 = 0x0080; /* RINMUTE */
pub const WM8961_RINMUTE_MASK: u32 = 0x0080; /* RINMUTE */
pub const WM8961_RINMUTE_SHIFT: u32 = 7; /* RINMUTE */
pub const WM8961_RINMUTE_WIDTH: u32 = 1; /* RINMUTE */
pub const WM8961_RIZC: u32 = 0x0040; /* RIZC */
pub const WM8961_RIZC_MASK: u32 = 0x0040; /* RIZC */
pub const WM8961_RIZC_SHIFT: u32 = 6; /* RIZC */
pub const WM8961_RIZC_WIDTH: u32 = 1; /* RIZC */
pub const WM8961_RINVOL_MASK: u32 = 0x003F; /* RINVOL - [5:0] */
pub const WM8961_RINVOL_SHIFT: u32 = 0; /* RINVOL - [5:0] */
pub const WM8961_RINVOL_WIDTH: u32 = 6; /* RINVOL - [5:0] */

/*
 * R2 (0x02) - LOUT1 volume
 */
pub const WM8961_OUT1VU: u32 = 0x0100; /* OUT1VU */
pub const WM8961_OUT1VU_MASK: u32 = 0x0100; /* OUT1VU */
pub const WM8961_OUT1VU_SHIFT: u32 = 8; /* OUT1VU */
pub const WM8961_OUT1VU_WIDTH: u32 = 1; /* OUT1VU */
pub const WM8961_LO1ZC: u32 = 0x0080; /* LO1ZC */
pub const WM8961_LO1ZC_MASK: u32 = 0x0080; /* LO1ZC */
pub const WM8961_LO1ZC_SHIFT: u32 = 7; /* LO1ZC */
pub const WM8961_LO1ZC_WIDTH: u32 = 1; /* LO1ZC */
pub const WM8961_LOUT1VOL_MASK: u32 = 0x007F; /* LOUT1VOL - [6:0] */
pub const WM8961_LOUT1VOL_SHIFT: u32 = 0; /* LOUT1VOL - [6:0] */
pub const WM8961_LOUT1VOL_WIDTH: u32 = 7; /* LOUT1VOL - [6:0] */

/*
 * R3 (0x03) - ROUT1 volume
 */
// Duplicate C define: WM8961_OUT1VU = 0x0100; /* OUT1VU */
// Duplicate C define: WM8961_OUT1VU_MASK = 0x0100; /* OUT1VU */
// Duplicate C define: WM8961_OUT1VU_SHIFT = 8; /* OUT1VU */
// Duplicate C define: WM8961_OUT1VU_WIDTH = 1; /* OUT1VU */
pub const WM8961_RO1ZC: u32 = 0x0080; /* RO1ZC */
pub const WM8961_RO1ZC_MASK: u32 = 0x0080; /* RO1ZC */
pub const WM8961_RO1ZC_SHIFT: u32 = 7; /* RO1ZC */
pub const WM8961_RO1ZC_WIDTH: u32 = 1; /* RO1ZC */
pub const WM8961_ROUT1VOL_MASK: u32 = 0x007F; /* ROUT1VOL - [6:0] */
pub const WM8961_ROUT1VOL_SHIFT: u32 = 0; /* ROUT1VOL - [6:0] */
pub const WM8961_ROUT1VOL_WIDTH: u32 = 7; /* ROUT1VOL - [6:0] */

/*
 * R4 (0x04) - Clocking1
 */
pub const WM8961_ADCDIV_MASK: u32 = 0x01C0; /* ADCDIV - [8:6] */
pub const WM8961_ADCDIV_SHIFT: u32 = 6; /* ADCDIV - [8:6] */
pub const WM8961_ADCDIV_WIDTH: u32 = 3; /* ADCDIV - [8:6] */
pub const WM8961_DACDIV_MASK: u32 = 0x0038; /* DACDIV - [5:3] */
pub const WM8961_DACDIV_SHIFT: u32 = 3; /* DACDIV - [5:3] */
pub const WM8961_DACDIV_WIDTH: u32 = 3; /* DACDIV - [5:3] */
pub const WM8961_MCLKDIV: u32 = 0x0004; /* MCLKDIV */
pub const WM8961_MCLKDIV_MASK: u32 = 0x0004; /* MCLKDIV */
pub const WM8961_MCLKDIV_SHIFT: u32 = 2; /* MCLKDIV */
pub const WM8961_MCLKDIV_WIDTH: u32 = 1; /* MCLKDIV */

/*
 * R5 (0x05) - ADC & DAC Control 1
 */
pub const WM8961_ADCPOL_MASK: u32 = 0x0060; /* ADCPOL - [6:5] */
pub const WM8961_ADCPOL_SHIFT: u32 = 5; /* ADCPOL - [6:5] */
pub const WM8961_ADCPOL_WIDTH: u32 = 2; /* ADCPOL - [6:5] */
pub const WM8961_DACMU: u32 = 0x0008; /* DACMU */
pub const WM8961_DACMU_MASK: u32 = 0x0008; /* DACMU */
pub const WM8961_DACMU_SHIFT: u32 = 3; /* DACMU */
pub const WM8961_DACMU_WIDTH: u32 = 1; /* DACMU */
pub const WM8961_DEEMPH_MASK: u32 = 0x0006; /* DEEMPH - [2:1] */
pub const WM8961_DEEMPH_SHIFT: u32 = 1; /* DEEMPH - [2:1] */
pub const WM8961_DEEMPH_WIDTH: u32 = 2; /* DEEMPH - [2:1] */
pub const WM8961_ADCHPD: u32 = 0x0001; /* ADCHPD */
pub const WM8961_ADCHPD_MASK: u32 = 0x0001; /* ADCHPD */
pub const WM8961_ADCHPD_SHIFT: u32 = 0; /* ADCHPD */
pub const WM8961_ADCHPD_WIDTH: u32 = 1; /* ADCHPD */

/*
 * R6 (0x06) - ADC & DAC Control 2
 */
pub const WM8961_ADC_HPF_CUT_MASK: u32 = 0x0180; /* ADC_HPF_CUT - [8:7] */
pub const WM8961_ADC_HPF_CUT_SHIFT: u32 = 7; /* ADC_HPF_CUT - [8:7] */
pub const WM8961_ADC_HPF_CUT_WIDTH: u32 = 2; /* ADC_HPF_CUT - [8:7] */
pub const WM8961_DACPOL_MASK: u32 = 0x0060; /* DACPOL - [6:5] */
pub const WM8961_DACPOL_SHIFT: u32 = 5; /* DACPOL - [6:5] */
pub const WM8961_DACPOL_WIDTH: u32 = 2; /* DACPOL - [6:5] */
pub const WM8961_DACSMM: u32 = 0x0008; /* DACSMM */
pub const WM8961_DACSMM_MASK: u32 = 0x0008; /* DACSMM */
pub const WM8961_DACSMM_SHIFT: u32 = 3; /* DACSMM */
pub const WM8961_DACSMM_WIDTH: u32 = 1; /* DACSMM */
pub const WM8961_DACMR: u32 = 0x0004; /* DACMR */
pub const WM8961_DACMR_MASK: u32 = 0x0004; /* DACMR */
pub const WM8961_DACMR_SHIFT: u32 = 2; /* DACMR */
pub const WM8961_DACMR_WIDTH: u32 = 1; /* DACMR */
pub const WM8961_DACSLOPE: u32 = 0x0002; /* DACSLOPE */
pub const WM8961_DACSLOPE_MASK: u32 = 0x0002; /* DACSLOPE */
pub const WM8961_DACSLOPE_SHIFT: u32 = 1; /* DACSLOPE */
pub const WM8961_DACSLOPE_WIDTH: u32 = 1; /* DACSLOPE */
pub const WM8961_DAC_OSR128: u32 = 0x0001; /* DAC_OSR128 */
pub const WM8961_DAC_OSR128_MASK: u32 = 0x0001; /* DAC_OSR128 */
pub const WM8961_DAC_OSR128_SHIFT: u32 = 0; /* DAC_OSR128 */
pub const WM8961_DAC_OSR128_WIDTH: u32 = 1; /* DAC_OSR128 */

/*
 * R7 (0x07) - Audio Interface 0
 */
pub const WM8961_ALRSWAP: u32 = 0x0100; /* ALRSWAP */
pub const WM8961_ALRSWAP_MASK: u32 = 0x0100; /* ALRSWAP */
pub const WM8961_ALRSWAP_SHIFT: u32 = 8; /* ALRSWAP */
pub const WM8961_ALRSWAP_WIDTH: u32 = 1; /* ALRSWAP */
pub const WM8961_BCLKINV: u32 = 0x0080; /* BCLKINV */
pub const WM8961_BCLKINV_MASK: u32 = 0x0080; /* BCLKINV */
pub const WM8961_BCLKINV_SHIFT: u32 = 7; /* BCLKINV */
pub const WM8961_BCLKINV_WIDTH: u32 = 1; /* BCLKINV */
pub const WM8961_MS: u32 = 0x0040; /* MS */
pub const WM8961_MS_MASK: u32 = 0x0040; /* MS */
pub const WM8961_MS_SHIFT: u32 = 6; /* MS */
pub const WM8961_MS_WIDTH: u32 = 1; /* MS */
pub const WM8961_DLRSWAP: u32 = 0x0020; /* DLRSWAP */
pub const WM8961_DLRSWAP_MASK: u32 = 0x0020; /* DLRSWAP */
pub const WM8961_DLRSWAP_SHIFT: u32 = 5; /* DLRSWAP */
pub const WM8961_DLRSWAP_WIDTH: u32 = 1; /* DLRSWAP */
pub const WM8961_LRP: u32 = 0x0010; /* LRP */
pub const WM8961_LRP_MASK: u32 = 0x0010; /* LRP */
pub const WM8961_LRP_SHIFT: u32 = 4; /* LRP */
pub const WM8961_LRP_WIDTH: u32 = 1; /* LRP */
pub const WM8961_WL_MASK: u32 = 0x000C; /* WL - [3:2] */
pub const WM8961_WL_SHIFT: u32 = 2; /* WL - [3:2] */
pub const WM8961_WL_WIDTH: u32 = 2; /* WL - [3:2] */
pub const WM8961_FORMAT_MASK: u32 = 0x0003; /* FORMAT - [1:0] */
pub const WM8961_FORMAT_SHIFT: u32 = 0; /* FORMAT - [1:0] */
pub const WM8961_FORMAT_WIDTH: u32 = 2; /* FORMAT - [1:0] */

/*
 * R8 (0x08) - Clocking2
 */
pub const WM8961_DCLKDIV_MASK: u32 = 0x01C0; /* DCLKDIV - [8:6] */
pub const WM8961_DCLKDIV_SHIFT: u32 = 6; /* DCLKDIV - [8:6] */
pub const WM8961_DCLKDIV_WIDTH: u32 = 3; /* DCLKDIV - [8:6] */
pub const WM8961_CLK_SYS_ENA: u32 = 0x0020; /* CLK_SYS_ENA */
pub const WM8961_CLK_SYS_ENA_MASK: u32 = 0x0020; /* CLK_SYS_ENA */
pub const WM8961_CLK_SYS_ENA_SHIFT: u32 = 5; /* CLK_SYS_ENA */
pub const WM8961_CLK_SYS_ENA_WIDTH: u32 = 1; /* CLK_SYS_ENA */
pub const WM8961_CLK_DSP_ENA: u32 = 0x0010; /* CLK_DSP_ENA */
pub const WM8961_CLK_DSP_ENA_MASK: u32 = 0x0010; /* CLK_DSP_ENA */
pub const WM8961_CLK_DSP_ENA_SHIFT: u32 = 4; /* CLK_DSP_ENA */
pub const WM8961_CLK_DSP_ENA_WIDTH: u32 = 1; /* CLK_DSP_ENA */
pub const WM8961_BCLKDIV_MASK: u32 = 0x000F; /* BCLKDIV - [3:0] */
pub const WM8961_BCLKDIV_SHIFT: u32 = 0; /* BCLKDIV - [3:0] */
pub const WM8961_BCLKDIV_WIDTH: u32 = 4; /* BCLKDIV - [3:0] */

/*
 * R9 (0x09) - Audio Interface 1
 */
pub const WM8961_DACCOMP_MASK: u32 = 0x0018; /* DACCOMP - [4:3] */
pub const WM8961_DACCOMP_SHIFT: u32 = 3; /* DACCOMP - [4:3] */
pub const WM8961_DACCOMP_WIDTH: u32 = 2; /* DACCOMP - [4:3] */
pub const WM8961_ADCCOMP_MASK: u32 = 0x0006; /* ADCCOMP - [2:1] */
pub const WM8961_ADCCOMP_SHIFT: u32 = 1; /* ADCCOMP - [2:1] */
pub const WM8961_ADCCOMP_WIDTH: u32 = 2; /* ADCCOMP - [2:1] */
pub const WM8961_LOOPBACK: u32 = 0x0001; /* LOOPBACK */
pub const WM8961_LOOPBACK_MASK: u32 = 0x0001; /* LOOPBACK */
pub const WM8961_LOOPBACK_SHIFT: u32 = 0; /* LOOPBACK */
pub const WM8961_LOOPBACK_WIDTH: u32 = 1; /* LOOPBACK */

/*
 * R10 (0x0A) - Left DAC volume
 */
pub const WM8961_DACVU: u32 = 0x0100; /* DACVU */
pub const WM8961_DACVU_MASK: u32 = 0x0100; /* DACVU */
pub const WM8961_DACVU_SHIFT: u32 = 8; /* DACVU */
pub const WM8961_DACVU_WIDTH: u32 = 1; /* DACVU */
pub const WM8961_LDACVOL_MASK: u32 = 0x00FF; /* LDACVOL - [7:0] */
pub const WM8961_LDACVOL_SHIFT: u32 = 0; /* LDACVOL - [7:0] */
pub const WM8961_LDACVOL_WIDTH: u32 = 8; /* LDACVOL - [7:0] */

/*
 * R11 (0x0B) - Right DAC volume
 */
// Duplicate C define: WM8961_DACVU = 0x0100; /* DACVU */
// Duplicate C define: WM8961_DACVU_MASK = 0x0100; /* DACVU */
// Duplicate C define: WM8961_DACVU_SHIFT = 8; /* DACVU */
// Duplicate C define: WM8961_DACVU_WIDTH = 1; /* DACVU */
pub const WM8961_RDACVOL_MASK: u32 = 0x00FF; /* RDACVOL - [7:0] */
pub const WM8961_RDACVOL_SHIFT: u32 = 0; /* RDACVOL - [7:0] */
pub const WM8961_RDACVOL_WIDTH: u32 = 8; /* RDACVOL - [7:0] */

/*
 * R14 (0x0E) - Audio Interface 2
 */
pub const WM8961_LRCLK_RATE_MASK: u32 = 0x01FF; /* LRCLK_RATE - [8:0] */
pub const WM8961_LRCLK_RATE_SHIFT: u32 = 0; /* LRCLK_RATE - [8:0] */
pub const WM8961_LRCLK_RATE_WIDTH: u32 = 9; /* LRCLK_RATE - [8:0] */

/*
 * R15 (0x0F) - Software Reset
 */
pub const WM8961_SW_RST_DEV_ID1_MASK: u32 = 0xFFFF; /* SW_RST_DEV_ID1 - [15:0] */
pub const WM8961_SW_RST_DEV_ID1_SHIFT: u32 = 0; /* SW_RST_DEV_ID1 - [15:0] */
pub const WM8961_SW_RST_DEV_ID1_WIDTH: u32 = 16; /* SW_RST_DEV_ID1 - [15:0] */

/*
 * R17 (0x11) - ALC1
 */
pub const WM8961_ALCSEL_MASK: u32 = 0x0180; /* ALCSEL - [8:7] */
pub const WM8961_ALCSEL_SHIFT: u32 = 7; /* ALCSEL - [8:7] */
pub const WM8961_ALCSEL_WIDTH: u32 = 2; /* ALCSEL - [8:7] */
pub const WM8961_MAXGAIN_MASK: u32 = 0x0070; /* MAXGAIN - [6:4] */
pub const WM8961_MAXGAIN_SHIFT: u32 = 4; /* MAXGAIN - [6:4] */
pub const WM8961_MAXGAIN_WIDTH: u32 = 3; /* MAXGAIN - [6:4] */
pub const WM8961_ALCL_MASK: u32 = 0x000F; /* ALCL - [3:0] */
pub const WM8961_ALCL_SHIFT: u32 = 0; /* ALCL - [3:0] */
pub const WM8961_ALCL_WIDTH: u32 = 4; /* ALCL - [3:0] */

/*
 * R18 (0x12) - ALC2
 */
pub const WM8961_ALCZC: u32 = 0x0080; /* ALCZC */
pub const WM8961_ALCZC_MASK: u32 = 0x0080; /* ALCZC */
pub const WM8961_ALCZC_SHIFT: u32 = 7; /* ALCZC */
pub const WM8961_ALCZC_WIDTH: u32 = 1; /* ALCZC */
pub const WM8961_MINGAIN_MASK: u32 = 0x0070; /* MINGAIN - [6:4] */
pub const WM8961_MINGAIN_SHIFT: u32 = 4; /* MINGAIN - [6:4] */
pub const WM8961_MINGAIN_WIDTH: u32 = 3; /* MINGAIN - [6:4] */
pub const WM8961_HLD_MASK: u32 = 0x000F; /* HLD - [3:0] */
pub const WM8961_HLD_SHIFT: u32 = 0; /* HLD - [3:0] */
pub const WM8961_HLD_WIDTH: u32 = 4; /* HLD - [3:0] */

/*
 * R19 (0x13) - ALC3
 */
pub const WM8961_ALCMODE: u32 = 0x0100; /* ALCMODE */
pub const WM8961_ALCMODE_MASK: u32 = 0x0100; /* ALCMODE */
pub const WM8961_ALCMODE_SHIFT: u32 = 8; /* ALCMODE */
pub const WM8961_ALCMODE_WIDTH: u32 = 1; /* ALCMODE */
pub const WM8961_DCY_MASK: u32 = 0x00F0; /* DCY - [7:4] */
pub const WM8961_DCY_SHIFT: u32 = 4; /* DCY - [7:4] */
pub const WM8961_DCY_WIDTH: u32 = 4; /* DCY - [7:4] */
pub const WM8961_ATK_MASK: u32 = 0x000F; /* ATK - [3:0] */
pub const WM8961_ATK_SHIFT: u32 = 0; /* ATK - [3:0] */
pub const WM8961_ATK_WIDTH: u32 = 4; /* ATK - [3:0] */

/*
 * R20 (0x14) - Noise Gate
 */
pub const WM8961_NGTH_MASK: u32 = 0x00F8; /* NGTH - [7:3] */
pub const WM8961_NGTH_SHIFT: u32 = 3; /* NGTH - [7:3] */
pub const WM8961_NGTH_WIDTH: u32 = 5; /* NGTH - [7:3] */
pub const WM8961_NGG: u32 = 0x0002; /* NGG */
pub const WM8961_NGG_MASK: u32 = 0x0002; /* NGG */
pub const WM8961_NGG_SHIFT: u32 = 1; /* NGG */
pub const WM8961_NGG_WIDTH: u32 = 1; /* NGG */
pub const WM8961_NGAT: u32 = 0x0001; /* NGAT */
pub const WM8961_NGAT_MASK: u32 = 0x0001; /* NGAT */
pub const WM8961_NGAT_SHIFT: u32 = 0; /* NGAT */
pub const WM8961_NGAT_WIDTH: u32 = 1; /* NGAT */

/*
 * R21 (0x15) - Left ADC volume
 */
pub const WM8961_ADCVU: u32 = 0x0100; /* ADCVU */
pub const WM8961_ADCVU_MASK: u32 = 0x0100; /* ADCVU */
pub const WM8961_ADCVU_SHIFT: u32 = 8; /* ADCVU */
pub const WM8961_ADCVU_WIDTH: u32 = 1; /* ADCVU */
pub const WM8961_LADCVOL_MASK: u32 = 0x00FF; /* LADCVOL - [7:0] */
pub const WM8961_LADCVOL_SHIFT: u32 = 0; /* LADCVOL - [7:0] */
pub const WM8961_LADCVOL_WIDTH: u32 = 8; /* LADCVOL - [7:0] */

/*
 * R22 (0x16) - Right ADC volume
 */
// Duplicate C define: WM8961_ADCVU = 0x0100; /* ADCVU */
// Duplicate C define: WM8961_ADCVU_MASK = 0x0100; /* ADCVU */
// Duplicate C define: WM8961_ADCVU_SHIFT = 8; /* ADCVU */
// Duplicate C define: WM8961_ADCVU_WIDTH = 1; /* ADCVU */
pub const WM8961_RADCVOL_MASK: u32 = 0x00FF; /* RADCVOL - [7:0] */
pub const WM8961_RADCVOL_SHIFT: u32 = 0; /* RADCVOL - [7:0] */
pub const WM8961_RADCVOL_WIDTH: u32 = 8; /* RADCVOL - [7:0] */

/*
 * R23 (0x17) - Additional control(1)
 */
pub const WM8961_TSDEN: u32 = 0x0100; /* TSDEN */
pub const WM8961_TSDEN_MASK: u32 = 0x0100; /* TSDEN */
pub const WM8961_TSDEN_SHIFT: u32 = 8; /* TSDEN */
pub const WM8961_TSDEN_WIDTH: u32 = 1; /* TSDEN */
pub const WM8961_DMONOMIX: u32 = 0x0010; /* DMONOMIX */
pub const WM8961_DMONOMIX_MASK: u32 = 0x0010; /* DMONOMIX */
pub const WM8961_DMONOMIX_SHIFT: u32 = 4; /* DMONOMIX */
pub const WM8961_DMONOMIX_WIDTH: u32 = 1; /* DMONOMIX */
pub const WM8961_TOEN: u32 = 0x0001; /* TOEN */
pub const WM8961_TOEN_MASK: u32 = 0x0001; /* TOEN */
pub const WM8961_TOEN_SHIFT: u32 = 0; /* TOEN */
pub const WM8961_TOEN_WIDTH: u32 = 1; /* TOEN */

/*
 * R24 (0x18) - Additional control(2)
 */
pub const WM8961_TRIS: u32 = 0x0008; /* TRIS */
pub const WM8961_TRIS_MASK: u32 = 0x0008; /* TRIS */
pub const WM8961_TRIS_SHIFT: u32 = 3; /* TRIS */
pub const WM8961_TRIS_WIDTH: u32 = 1; /* TRIS */

/*
 * R25 (0x19) - Pwr Mgmt (1)
 */
pub const WM8961_VMIDSEL_MASK: u32 = 0x0180; /* VMIDSEL - [8:7] */
pub const WM8961_VMIDSEL_SHIFT: u32 = 7; /* VMIDSEL - [8:7] */
pub const WM8961_VMIDSEL_WIDTH: u32 = 2; /* VMIDSEL - [8:7] */
pub const WM8961_VREF: u32 = 0x0040; /* VREF */
pub const WM8961_VREF_MASK: u32 = 0x0040; /* VREF */
pub const WM8961_VREF_SHIFT: u32 = 6; /* VREF */
pub const WM8961_VREF_WIDTH: u32 = 1; /* VREF */
pub const WM8961_AINL: u32 = 0x0020; /* AINL */
pub const WM8961_AINL_MASK: u32 = 0x0020; /* AINL */
pub const WM8961_AINL_SHIFT: u32 = 5; /* AINL */
pub const WM8961_AINL_WIDTH: u32 = 1; /* AINL */
pub const WM8961_AINR: u32 = 0x0010; /* AINR */
pub const WM8961_AINR_MASK: u32 = 0x0010; /* AINR */
pub const WM8961_AINR_SHIFT: u32 = 4; /* AINR */
pub const WM8961_AINR_WIDTH: u32 = 1; /* AINR */
pub const WM8961_ADCL: u32 = 0x0008; /* ADCL */
pub const WM8961_ADCL_MASK: u32 = 0x0008; /* ADCL */
pub const WM8961_ADCL_SHIFT: u32 = 3; /* ADCL */
pub const WM8961_ADCL_WIDTH: u32 = 1; /* ADCL */
pub const WM8961_ADCR: u32 = 0x0004; /* ADCR */
pub const WM8961_ADCR_MASK: u32 = 0x0004; /* ADCR */
pub const WM8961_ADCR_SHIFT: u32 = 2; /* ADCR */
pub const WM8961_ADCR_WIDTH: u32 = 1; /* ADCR */
pub const WM8961_MICB: u32 = 0x0002; /* MICB */
pub const WM8961_MICB_MASK: u32 = 0x0002; /* MICB */
pub const WM8961_MICB_SHIFT: u32 = 1; /* MICB */
pub const WM8961_MICB_WIDTH: u32 = 1; /* MICB */

/*
 * R26 (0x1A) - Pwr Mgmt (2)
 */
pub const WM8961_DACL: u32 = 0x0100; /* DACL */
pub const WM8961_DACL_MASK: u32 = 0x0100; /* DACL */
pub const WM8961_DACL_SHIFT: u32 = 8; /* DACL */
pub const WM8961_DACL_WIDTH: u32 = 1; /* DACL */
pub const WM8961_DACR: u32 = 0x0080; /* DACR */
pub const WM8961_DACR_MASK: u32 = 0x0080; /* DACR */
pub const WM8961_DACR_SHIFT: u32 = 7; /* DACR */
pub const WM8961_DACR_WIDTH: u32 = 1; /* DACR */
pub const WM8961_LOUT1_PGA: u32 = 0x0040; /* LOUT1_PGA */
pub const WM8961_LOUT1_PGA_MASK: u32 = 0x0040; /* LOUT1_PGA */
pub const WM8961_LOUT1_PGA_SHIFT: u32 = 6; /* LOUT1_PGA */
pub const WM8961_LOUT1_PGA_WIDTH: u32 = 1; /* LOUT1_PGA */
pub const WM8961_ROUT1_PGA: u32 = 0x0020; /* ROUT1_PGA */
pub const WM8961_ROUT1_PGA_MASK: u32 = 0x0020; /* ROUT1_PGA */
pub const WM8961_ROUT1_PGA_SHIFT: u32 = 5; /* ROUT1_PGA */
pub const WM8961_ROUT1_PGA_WIDTH: u32 = 1; /* ROUT1_PGA */
pub const WM8961_SPKL_PGA: u32 = 0x0010; /* SPKL_PGA */
pub const WM8961_SPKL_PGA_MASK: u32 = 0x0010; /* SPKL_PGA */
pub const WM8961_SPKL_PGA_SHIFT: u32 = 4; /* SPKL_PGA */
pub const WM8961_SPKL_PGA_WIDTH: u32 = 1; /* SPKL_PGA */
pub const WM8961_SPKR_PGA: u32 = 0x0008; /* SPKR_PGA */
pub const WM8961_SPKR_PGA_MASK: u32 = 0x0008; /* SPKR_PGA */
pub const WM8961_SPKR_PGA_SHIFT: u32 = 3; /* SPKR_PGA */
pub const WM8961_SPKR_PGA_WIDTH: u32 = 1; /* SPKR_PGA */

/*
 * R27 (0x1B) - Additional Control (3)
 */
pub const WM8961_SAMPLE_RATE_MASK: u32 = 0x0007; /* SAMPLE_RATE - [2:0] */
pub const WM8961_SAMPLE_RATE_SHIFT: u32 = 0; /* SAMPLE_RATE - [2:0] */
pub const WM8961_SAMPLE_RATE_WIDTH: u32 = 3; /* SAMPLE_RATE - [2:0] */

/*
 * R28 (0x1C) - Anti-pop
 */
pub const WM8961_BUFDCOPEN: u32 = 0x0010; /* BUFDCOPEN */
pub const WM8961_BUFDCOPEN_MASK: u32 = 0x0010; /* BUFDCOPEN */
pub const WM8961_BUFDCOPEN_SHIFT: u32 = 4; /* BUFDCOPEN */
pub const WM8961_BUFDCOPEN_WIDTH: u32 = 1; /* BUFDCOPEN */
pub const WM8961_BUFIOEN: u32 = 0x0008; /* BUFIOEN */
pub const WM8961_BUFIOEN_MASK: u32 = 0x0008; /* BUFIOEN */
pub const WM8961_BUFIOEN_SHIFT: u32 = 3; /* BUFIOEN */
pub const WM8961_BUFIOEN_WIDTH: u32 = 1; /* BUFIOEN */
pub const WM8961_SOFT_ST: u32 = 0x0004; /* SOFT_ST */
pub const WM8961_SOFT_ST_MASK: u32 = 0x0004; /* SOFT_ST */
pub const WM8961_SOFT_ST_SHIFT: u32 = 2; /* SOFT_ST */
pub const WM8961_SOFT_ST_WIDTH: u32 = 1; /* SOFT_ST */

/*
 * R30 (0x1E) - Clocking 3
 */
pub const WM8961_CLK_TO_DIV_MASK: u32 = 0x0180; /* CLK_TO_DIV - [8:7] */
pub const WM8961_CLK_TO_DIV_SHIFT: u32 = 7; /* CLK_TO_DIV - [8:7] */
pub const WM8961_CLK_TO_DIV_WIDTH: u32 = 2; /* CLK_TO_DIV - [8:7] */
pub const WM8961_CLK_256K_DIV_MASK: u32 = 0x007E; /* CLK_256K_DIV - [6:1] */
pub const WM8961_CLK_256K_DIV_SHIFT: u32 = 1; /* CLK_256K_DIV - [6:1] */
pub const WM8961_CLK_256K_DIV_WIDTH: u32 = 6; /* CLK_256K_DIV - [6:1] */
pub const WM8961_MANUAL_MODE: u32 = 0x0001; /* MANUAL_MODE */
pub const WM8961_MANUAL_MODE_MASK: u32 = 0x0001; /* MANUAL_MODE */
pub const WM8961_MANUAL_MODE_SHIFT: u32 = 0; /* MANUAL_MODE */
pub const WM8961_MANUAL_MODE_WIDTH: u32 = 1; /* MANUAL_MODE */

/*
 * R32 (0x20) - ADCL signal path
 */
pub const WM8961_LMICBOOST_MASK: u32 = 0x0030; /* LMICBOOST - [5:4] */
pub const WM8961_LMICBOOST_SHIFT: u32 = 4; /* LMICBOOST - [5:4] */
pub const WM8961_LMICBOOST_WIDTH: u32 = 2; /* LMICBOOST - [5:4] */

/*
 * R33 (0x21) - ADCR signal path
 */
pub const WM8961_RMICBOOST_MASK: u32 = 0x0030; /* RMICBOOST - [5:4] */
pub const WM8961_RMICBOOST_SHIFT: u32 = 4; /* RMICBOOST - [5:4] */
pub const WM8961_RMICBOOST_WIDTH: u32 = 2; /* RMICBOOST - [5:4] */

/*
 * R40 (0x28) - LOUT2 volume
 */
pub const WM8961_SPKVU: u32 = 0x0100; /* SPKVU */
pub const WM8961_SPKVU_MASK: u32 = 0x0100; /* SPKVU */
pub const WM8961_SPKVU_SHIFT: u32 = 8; /* SPKVU */
pub const WM8961_SPKVU_WIDTH: u32 = 1; /* SPKVU */
pub const WM8961_SPKLZC: u32 = 0x0080; /* SPKLZC */
pub const WM8961_SPKLZC_MASK: u32 = 0x0080; /* SPKLZC */
pub const WM8961_SPKLZC_SHIFT: u32 = 7; /* SPKLZC */
pub const WM8961_SPKLZC_WIDTH: u32 = 1; /* SPKLZC */
pub const WM8961_SPKLVOL_MASK: u32 = 0x007F; /* SPKLVOL - [6:0] */
pub const WM8961_SPKLVOL_SHIFT: u32 = 0; /* SPKLVOL - [6:0] */
pub const WM8961_SPKLVOL_WIDTH: u32 = 7; /* SPKLVOL - [6:0] */

/*
 * R41 (0x29) - ROUT2 volume
 */
// Duplicate C define: WM8961_SPKVU = 0x0100; /* SPKVU */
// Duplicate C define: WM8961_SPKVU_MASK = 0x0100; /* SPKVU */
// Duplicate C define: WM8961_SPKVU_SHIFT = 8; /* SPKVU */
// Duplicate C define: WM8961_SPKVU_WIDTH = 1; /* SPKVU */
pub const WM8961_SPKRZC: u32 = 0x0080; /* SPKRZC */
pub const WM8961_SPKRZC_MASK: u32 = 0x0080; /* SPKRZC */
pub const WM8961_SPKRZC_SHIFT: u32 = 7; /* SPKRZC */
pub const WM8961_SPKRZC_WIDTH: u32 = 1; /* SPKRZC */
pub const WM8961_SPKRVOL_MASK: u32 = 0x007F; /* SPKRVOL - [6:0] */
pub const WM8961_SPKRVOL_SHIFT: u32 = 0; /* SPKRVOL - [6:0] */
pub const WM8961_SPKRVOL_WIDTH: u32 = 7; /* SPKRVOL - [6:0] */

/*
 * R47 (0x2F) - Pwr Mgmt (3)
 */
pub const WM8961_TEMP_SHUT: u32 = 0x0002; /* TEMP_SHUT */
pub const WM8961_TEMP_SHUT_MASK: u32 = 0x0002; /* TEMP_SHUT */
pub const WM8961_TEMP_SHUT_SHIFT: u32 = 1; /* TEMP_SHUT */
pub const WM8961_TEMP_SHUT_WIDTH: u32 = 1; /* TEMP_SHUT */
pub const WM8961_TEMP_WARN: u32 = 0x0001; /* TEMP_WARN */
pub const WM8961_TEMP_WARN_MASK: u32 = 0x0001; /* TEMP_WARN */
pub const WM8961_TEMP_WARN_SHIFT: u32 = 0; /* TEMP_WARN */
pub const WM8961_TEMP_WARN_WIDTH: u32 = 1; /* TEMP_WARN */

/*
 * R48 (0x30) - Additional Control (4)
 */
pub const WM8961_TSENSEN: u32 = 0x0002; /* TSENSEN */
pub const WM8961_TSENSEN_MASK: u32 = 0x0002; /* TSENSEN */
pub const WM8961_TSENSEN_SHIFT: u32 = 1; /* TSENSEN */
pub const WM8961_TSENSEN_WIDTH: u32 = 1; /* TSENSEN */
pub const WM8961_MBSEL: u32 = 0x0001; /* MBSEL */
pub const WM8961_MBSEL_MASK: u32 = 0x0001; /* MBSEL */
pub const WM8961_MBSEL_SHIFT: u32 = 0; /* MBSEL */
pub const WM8961_MBSEL_WIDTH: u32 = 1; /* MBSEL */

/*
 * R49 (0x31) - Class D Control 1
 */
pub const WM8961_SPKR_ENA: u32 = 0x0080; /* SPKR_ENA */
pub const WM8961_SPKR_ENA_MASK: u32 = 0x0080; /* SPKR_ENA */
pub const WM8961_SPKR_ENA_SHIFT: u32 = 7; /* SPKR_ENA */
pub const WM8961_SPKR_ENA_WIDTH: u32 = 1; /* SPKR_ENA */
pub const WM8961_SPKL_ENA: u32 = 0x0040; /* SPKL_ENA */
pub const WM8961_SPKL_ENA_MASK: u32 = 0x0040; /* SPKL_ENA */
pub const WM8961_SPKL_ENA_SHIFT: u32 = 6; /* SPKL_ENA */
pub const WM8961_SPKL_ENA_WIDTH: u32 = 1; /* SPKL_ENA */

/*
 * R51 (0x33) - Class D Control 2
 */
pub const WM8961_CLASSD_ACGAIN_MASK: u32 = 0x0007; /* CLASSD_ACGAIN - [2:0] */
pub const WM8961_CLASSD_ACGAIN_SHIFT: u32 = 0; /* CLASSD_ACGAIN - [2:0] */
pub const WM8961_CLASSD_ACGAIN_WIDTH: u32 = 3; /* CLASSD_ACGAIN - [2:0] */

/*
 * R56 (0x38) - Clocking 4
 */
pub const WM8961_CLK_DCS_DIV_MASK: u32 = 0x01E0; /* CLK_DCS_DIV - [8:5] */
pub const WM8961_CLK_DCS_DIV_SHIFT: u32 = 5; /* CLK_DCS_DIV - [8:5] */
pub const WM8961_CLK_DCS_DIV_WIDTH: u32 = 4; /* CLK_DCS_DIV - [8:5] */
pub const WM8961_CLK_SYS_RATE_MASK: u32 = 0x001E; /* CLK_SYS_RATE - [4:1] */
pub const WM8961_CLK_SYS_RATE_SHIFT: u32 = 1; /* CLK_SYS_RATE - [4:1] */
pub const WM8961_CLK_SYS_RATE_WIDTH: u32 = 4; /* CLK_SYS_RATE - [4:1] */

/*
 * R57 (0x39) - DSP Sidetone 0
 */
pub const WM8961_ADCR_DAC_SVOL_MASK: u32 = 0x00F0; /* ADCR_DAC_SVOL - [7:4] */
pub const WM8961_ADCR_DAC_SVOL_SHIFT: u32 = 4; /* ADCR_DAC_SVOL - [7:4] */
pub const WM8961_ADCR_DAC_SVOL_WIDTH: u32 = 4; /* ADCR_DAC_SVOL - [7:4] */
pub const WM8961_ADC_TO_DACR_MASK: u32 = 0x000C; /* ADC_TO_DACR - [3:2] */
pub const WM8961_ADC_TO_DACR_SHIFT: u32 = 2; /* ADC_TO_DACR - [3:2] */
pub const WM8961_ADC_TO_DACR_WIDTH: u32 = 2; /* ADC_TO_DACR - [3:2] */

/*
 * R58 (0x3A) - DSP Sidetone 1
 */
pub const WM8961_ADCL_DAC_SVOL_MASK: u32 = 0x00F0; /* ADCL_DAC_SVOL - [7:4] */
pub const WM8961_ADCL_DAC_SVOL_SHIFT: u32 = 4; /* ADCL_DAC_SVOL - [7:4] */
pub const WM8961_ADCL_DAC_SVOL_WIDTH: u32 = 4; /* ADCL_DAC_SVOL - [7:4] */
pub const WM8961_ADC_TO_DACL_MASK: u32 = 0x000C; /* ADC_TO_DACL - [3:2] */
pub const WM8961_ADC_TO_DACL_SHIFT: u32 = 2; /* ADC_TO_DACL - [3:2] */
pub const WM8961_ADC_TO_DACL_WIDTH: u32 = 2; /* ADC_TO_DACL - [3:2] */

/*
 * R60 (0x3C) - DC Servo 0
 */
pub const WM8961_DCS_ENA_CHAN_INL: u32 = 0x0080; /* DCS_ENA_CHAN_INL */
pub const WM8961_DCS_ENA_CHAN_INL_MASK: u32 = 0x0080; /* DCS_ENA_CHAN_INL */
pub const WM8961_DCS_ENA_CHAN_INL_SHIFT: u32 = 7; /* DCS_ENA_CHAN_INL */
pub const WM8961_DCS_ENA_CHAN_INL_WIDTH: u32 = 1; /* DCS_ENA_CHAN_INL */
pub const WM8961_DCS_TRIG_STARTUP_INL: u32 = 0x0040; /* DCS_TRIG_STARTUP_INL */
pub const WM8961_DCS_TRIG_STARTUP_INL_MASK: u32 = 0x0040; /* DCS_TRIG_STARTUP_INL */
pub const WM8961_DCS_TRIG_STARTUP_INL_SHIFT: u32 = 6; /* DCS_TRIG_STARTUP_INL */
pub const WM8961_DCS_TRIG_STARTUP_INL_WIDTH: u32 = 1; /* DCS_TRIG_STARTUP_INL */
pub const WM8961_DCS_TRIG_SERIES_INL: u32 = 0x0010; /* DCS_TRIG_SERIES_INL */
pub const WM8961_DCS_TRIG_SERIES_INL_MASK: u32 = 0x0010; /* DCS_TRIG_SERIES_INL */
pub const WM8961_DCS_TRIG_SERIES_INL_SHIFT: u32 = 4; /* DCS_TRIG_SERIES_INL */
pub const WM8961_DCS_TRIG_SERIES_INL_WIDTH: u32 = 1; /* DCS_TRIG_SERIES_INL */
pub const WM8961_DCS_ENA_CHAN_INR: u32 = 0x0008; /* DCS_ENA_CHAN_INR */
pub const WM8961_DCS_ENA_CHAN_INR_MASK: u32 = 0x0008; /* DCS_ENA_CHAN_INR */
pub const WM8961_DCS_ENA_CHAN_INR_SHIFT: u32 = 3; /* DCS_ENA_CHAN_INR */
pub const WM8961_DCS_ENA_CHAN_INR_WIDTH: u32 = 1; /* DCS_ENA_CHAN_INR */
pub const WM8961_DCS_TRIG_STARTUP_INR: u32 = 0x0004; /* DCS_TRIG_STARTUP_INR */
pub const WM8961_DCS_TRIG_STARTUP_INR_MASK: u32 = 0x0004; /* DCS_TRIG_STARTUP_INR */
pub const WM8961_DCS_TRIG_STARTUP_INR_SHIFT: u32 = 2; /* DCS_TRIG_STARTUP_INR */
pub const WM8961_DCS_TRIG_STARTUP_INR_WIDTH: u32 = 1; /* DCS_TRIG_STARTUP_INR */
pub const WM8961_DCS_TRIG_SERIES_INR: u32 = 0x0001; /* DCS_TRIG_SERIES_INR */
pub const WM8961_DCS_TRIG_SERIES_INR_MASK: u32 = 0x0001; /* DCS_TRIG_SERIES_INR */
pub const WM8961_DCS_TRIG_SERIES_INR_SHIFT: u32 = 0; /* DCS_TRIG_SERIES_INR */
pub const WM8961_DCS_TRIG_SERIES_INR_WIDTH: u32 = 1; /* DCS_TRIG_SERIES_INR */

/*
 * R61 (0x3D) - DC Servo 1
 */
pub const WM8961_DCS_ENA_CHAN_HPL: u32 = 0x0080; /* DCS_ENA_CHAN_HPL */
pub const WM8961_DCS_ENA_CHAN_HPL_MASK: u32 = 0x0080; /* DCS_ENA_CHAN_HPL */
pub const WM8961_DCS_ENA_CHAN_HPL_SHIFT: u32 = 7; /* DCS_ENA_CHAN_HPL */
pub const WM8961_DCS_ENA_CHAN_HPL_WIDTH: u32 = 1; /* DCS_ENA_CHAN_HPL */
pub const WM8961_DCS_TRIG_STARTUP_HPL: u32 = 0x0040; /* DCS_TRIG_STARTUP_HPL */
pub const WM8961_DCS_TRIG_STARTUP_HPL_MASK: u32 = 0x0040; /* DCS_TRIG_STARTUP_HPL */
pub const WM8961_DCS_TRIG_STARTUP_HPL_SHIFT: u32 = 6; /* DCS_TRIG_STARTUP_HPL */
pub const WM8961_DCS_TRIG_STARTUP_HPL_WIDTH: u32 = 1; /* DCS_TRIG_STARTUP_HPL */
pub const WM8961_DCS_TRIG_SERIES_HPL: u32 = 0x0010; /* DCS_TRIG_SERIES_HPL */
pub const WM8961_DCS_TRIG_SERIES_HPL_MASK: u32 = 0x0010; /* DCS_TRIG_SERIES_HPL */
pub const WM8961_DCS_TRIG_SERIES_HPL_SHIFT: u32 = 4; /* DCS_TRIG_SERIES_HPL */
pub const WM8961_DCS_TRIG_SERIES_HPL_WIDTH: u32 = 1; /* DCS_TRIG_SERIES_HPL */
pub const WM8961_DCS_ENA_CHAN_HPR: u32 = 0x0008; /* DCS_ENA_CHAN_HPR */
pub const WM8961_DCS_ENA_CHAN_HPR_MASK: u32 = 0x0008; /* DCS_ENA_CHAN_HPR */
pub const WM8961_DCS_ENA_CHAN_HPR_SHIFT: u32 = 3; /* DCS_ENA_CHAN_HPR */
pub const WM8961_DCS_ENA_CHAN_HPR_WIDTH: u32 = 1; /* DCS_ENA_CHAN_HPR */
pub const WM8961_DCS_TRIG_STARTUP_HPR: u32 = 0x0004; /* DCS_TRIG_STARTUP_HPR */
pub const WM8961_DCS_TRIG_STARTUP_HPR_MASK: u32 = 0x0004; /* DCS_TRIG_STARTUP_HPR */
pub const WM8961_DCS_TRIG_STARTUP_HPR_SHIFT: u32 = 2; /* DCS_TRIG_STARTUP_HPR */
pub const WM8961_DCS_TRIG_STARTUP_HPR_WIDTH: u32 = 1; /* DCS_TRIG_STARTUP_HPR */
pub const WM8961_DCS_TRIG_SERIES_HPR: u32 = 0x0001; /* DCS_TRIG_SERIES_HPR */
pub const WM8961_DCS_TRIG_SERIES_HPR_MASK: u32 = 0x0001; /* DCS_TRIG_SERIES_HPR */
pub const WM8961_DCS_TRIG_SERIES_HPR_SHIFT: u32 = 0; /* DCS_TRIG_SERIES_HPR */
pub const WM8961_DCS_TRIG_SERIES_HPR_WIDTH: u32 = 1; /* DCS_TRIG_SERIES_HPR */

/*
 * R63 (0x3F) - DC Servo 3
 */
pub const WM8961_DCS_FILT_BW_SERIES_MASK: u32 = 0x0030; /* DCS_FILT_BW_SERIES - [5:4] */
pub const WM8961_DCS_FILT_BW_SERIES_SHIFT: u32 = 4; /* DCS_FILT_BW_SERIES - [5:4] */
pub const WM8961_DCS_FILT_BW_SERIES_WIDTH: u32 = 2; /* DCS_FILT_BW_SERIES - [5:4] */

/*
 * R65 (0x41) - DC Servo 5
 */
pub const WM8961_DCS_SERIES_NO_HP_MASK: u32 = 0x007F; /* DCS_SERIES_NO_HP - [6:0] */
pub const WM8961_DCS_SERIES_NO_HP_SHIFT: u32 = 0; /* DCS_SERIES_NO_HP - [6:0] */
pub const WM8961_DCS_SERIES_NO_HP_WIDTH: u32 = 7; /* DCS_SERIES_NO_HP - [6:0] */

/*
 * R68 (0x44) - Analogue PGA Bias
 */
pub const WM8961_HP_PGAS_BIAS_MASK: u32 = 0x0007; /* HP_PGAS_BIAS - [2:0] */
pub const WM8961_HP_PGAS_BIAS_SHIFT: u32 = 0; /* HP_PGAS_BIAS - [2:0] */
pub const WM8961_HP_PGAS_BIAS_WIDTH: u32 = 3; /* HP_PGAS_BIAS - [2:0] */

/*
 * R69 (0x45) - Analogue HP 0
 */
pub const WM8961_HPL_RMV_SHORT: u32 = 0x0080; /* HPL_RMV_SHORT */
pub const WM8961_HPL_RMV_SHORT_MASK: u32 = 0x0080; /* HPL_RMV_SHORT */
pub const WM8961_HPL_RMV_SHORT_SHIFT: u32 = 7; /* HPL_RMV_SHORT */
pub const WM8961_HPL_RMV_SHORT_WIDTH: u32 = 1; /* HPL_RMV_SHORT */
pub const WM8961_HPL_ENA_OUTP: u32 = 0x0040; /* HPL_ENA_OUTP */
pub const WM8961_HPL_ENA_OUTP_MASK: u32 = 0x0040; /* HPL_ENA_OUTP */
pub const WM8961_HPL_ENA_OUTP_SHIFT: u32 = 6; /* HPL_ENA_OUTP */
pub const WM8961_HPL_ENA_OUTP_WIDTH: u32 = 1; /* HPL_ENA_OUTP */
pub const WM8961_HPL_ENA_DLY: u32 = 0x0020; /* HPL_ENA_DLY */
pub const WM8961_HPL_ENA_DLY_MASK: u32 = 0x0020; /* HPL_ENA_DLY */
pub const WM8961_HPL_ENA_DLY_SHIFT: u32 = 5; /* HPL_ENA_DLY */
pub const WM8961_HPL_ENA_DLY_WIDTH: u32 = 1; /* HPL_ENA_DLY */
pub const WM8961_HPL_ENA: u32 = 0x0010; /* HPL_ENA */
pub const WM8961_HPL_ENA_MASK: u32 = 0x0010; /* HPL_ENA */
pub const WM8961_HPL_ENA_SHIFT: u32 = 4; /* HPL_ENA */
pub const WM8961_HPL_ENA_WIDTH: u32 = 1; /* HPL_ENA */
pub const WM8961_HPR_RMV_SHORT: u32 = 0x0008; /* HPR_RMV_SHORT */
pub const WM8961_HPR_RMV_SHORT_MASK: u32 = 0x0008; /* HPR_RMV_SHORT */
pub const WM8961_HPR_RMV_SHORT_SHIFT: u32 = 3; /* HPR_RMV_SHORT */
pub const WM8961_HPR_RMV_SHORT_WIDTH: u32 = 1; /* HPR_RMV_SHORT */
pub const WM8961_HPR_ENA_OUTP: u32 = 0x0004; /* HPR_ENA_OUTP */
pub const WM8961_HPR_ENA_OUTP_MASK: u32 = 0x0004; /* HPR_ENA_OUTP */
pub const WM8961_HPR_ENA_OUTP_SHIFT: u32 = 2; /* HPR_ENA_OUTP */
pub const WM8961_HPR_ENA_OUTP_WIDTH: u32 = 1; /* HPR_ENA_OUTP */
pub const WM8961_HPR_ENA_DLY: u32 = 0x0002; /* HPR_ENA_DLY */
pub const WM8961_HPR_ENA_DLY_MASK: u32 = 0x0002; /* HPR_ENA_DLY */
pub const WM8961_HPR_ENA_DLY_SHIFT: u32 = 1; /* HPR_ENA_DLY */
pub const WM8961_HPR_ENA_DLY_WIDTH: u32 = 1; /* HPR_ENA_DLY */
pub const WM8961_HPR_ENA: u32 = 0x0001; /* HPR_ENA */
pub const WM8961_HPR_ENA_MASK: u32 = 0x0001; /* HPR_ENA */
pub const WM8961_HPR_ENA_SHIFT: u32 = 0; /* HPR_ENA */
pub const WM8961_HPR_ENA_WIDTH: u32 = 1; /* HPR_ENA */

/*
 * R71 (0x47) - Analogue HP 2
 */
pub const WM8961_HPL_VOL_MASK: u32 = 0x01C0; /* HPL_VOL - [8:6] */
pub const WM8961_HPL_VOL_SHIFT: u32 = 6; /* HPL_VOL - [8:6] */
pub const WM8961_HPL_VOL_WIDTH: u32 = 3; /* HPL_VOL - [8:6] */
pub const WM8961_HPR_VOL_MASK: u32 = 0x0038; /* HPR_VOL - [5:3] */
pub const WM8961_HPR_VOL_SHIFT: u32 = 3; /* HPR_VOL - [5:3] */
pub const WM8961_HPR_VOL_WIDTH: u32 = 3; /* HPR_VOL - [5:3] */
pub const WM8961_HP_BIAS_BOOST_MASK: u32 = 0x0007; /* HP_BIAS_BOOST - [2:0] */
pub const WM8961_HP_BIAS_BOOST_SHIFT: u32 = 0; /* HP_BIAS_BOOST - [2:0] */
pub const WM8961_HP_BIAS_BOOST_WIDTH: u32 = 3; /* HP_BIAS_BOOST - [2:0] */

/*
 * R72 (0x48) - Charge Pump 1
 */
pub const WM8961_CP_ENA: u32 = 0x0001; /* CP_ENA */
pub const WM8961_CP_ENA_MASK: u32 = 0x0001; /* CP_ENA */
pub const WM8961_CP_ENA_SHIFT: u32 = 0; /* CP_ENA */
pub const WM8961_CP_ENA_WIDTH: u32 = 1; /* CP_ENA */

/*
 * R82 (0x52) - Charge Pump B
 */
pub const WM8961_CP_DYN_PWR_MASK: u32 = 0x0003; /* CP_DYN_PWR - [1:0] */
pub const WM8961_CP_DYN_PWR_SHIFT: u32 = 0; /* CP_DYN_PWR - [1:0] */
pub const WM8961_CP_DYN_PWR_WIDTH: u32 = 2; /* CP_DYN_PWR - [1:0] */

/*
 * R87 (0x57) - Write Sequencer 1
 */
pub const WM8961_WSEQ_ENA: u32 = 0x0020; /* WSEQ_ENA */
pub const WM8961_WSEQ_ENA_MASK: u32 = 0x0020; /* WSEQ_ENA */
pub const WM8961_WSEQ_ENA_SHIFT: u32 = 5; /* WSEQ_ENA */
pub const WM8961_WSEQ_ENA_WIDTH: u32 = 1; /* WSEQ_ENA */
pub const WM8961_WSEQ_WRITE_INDEX_MASK: u32 = 0x001F; /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8961_WSEQ_WRITE_INDEX_SHIFT: u32 = 0; /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8961_WSEQ_WRITE_INDEX_WIDTH: u32 = 5; /* WSEQ_WRITE_INDEX - [4:0] */

/*
 * R88 (0x58) - Write Sequencer 2
 */
pub const WM8961_WSEQ_EOS: u32 = 0x0100; /* WSEQ_EOS */
pub const WM8961_WSEQ_EOS_MASK: u32 = 0x0100; /* WSEQ_EOS */
pub const WM8961_WSEQ_EOS_SHIFT: u32 = 8; /* WSEQ_EOS */
pub const WM8961_WSEQ_EOS_WIDTH: u32 = 1; /* WSEQ_EOS */
pub const WM8961_WSEQ_ADDR_MASK: u32 = 0x00FF; /* WSEQ_ADDR - [7:0] */
pub const WM8961_WSEQ_ADDR_SHIFT: u32 = 0; /* WSEQ_ADDR - [7:0] */
pub const WM8961_WSEQ_ADDR_WIDTH: u32 = 8; /* WSEQ_ADDR - [7:0] */

/*
 * R89 (0x59) - Write Sequencer 3
 */
pub const WM8961_WSEQ_DATA_MASK: u32 = 0x00FF; /* WSEQ_DATA - [7:0] */
pub const WM8961_WSEQ_DATA_SHIFT: u32 = 0; /* WSEQ_DATA - [7:0] */
pub const WM8961_WSEQ_DATA_WIDTH: u32 = 8; /* WSEQ_DATA - [7:0] */

/*
 * R90 (0x5A) - Write Sequencer 4
 */
pub const WM8961_WSEQ_ABORT: u32 = 0x0100; /* WSEQ_ABORT */
pub const WM8961_WSEQ_ABORT_MASK: u32 = 0x0100; /* WSEQ_ABORT */
pub const WM8961_WSEQ_ABORT_SHIFT: u32 = 8; /* WSEQ_ABORT */
pub const WM8961_WSEQ_ABORT_WIDTH: u32 = 1; /* WSEQ_ABORT */
pub const WM8961_WSEQ_START: u32 = 0x0080; /* WSEQ_START */
pub const WM8961_WSEQ_START_MASK: u32 = 0x0080; /* WSEQ_START */
pub const WM8961_WSEQ_START_SHIFT: u32 = 7; /* WSEQ_START */
pub const WM8961_WSEQ_START_WIDTH: u32 = 1; /* WSEQ_START */
pub const WM8961_WSEQ_START_INDEX_MASK: u32 = 0x003F; /* WSEQ_START_INDEX - [5:0] */
pub const WM8961_WSEQ_START_INDEX_SHIFT: u32 = 0; /* WSEQ_START_INDEX - [5:0] */
pub const WM8961_WSEQ_START_INDEX_WIDTH: u32 = 6; /* WSEQ_START_INDEX - [5:0] */

/*
 * R91 (0x5B) - Write Sequencer 5
 */
pub const WM8961_WSEQ_DATA_WIDTH_MASK: u32 = 0x0070; /* WSEQ_DATA_WIDTH - [6:4] */
pub const WM8961_WSEQ_DATA_WIDTH_SHIFT: u32 = 4; /* WSEQ_DATA_WIDTH - [6:4] */
pub const WM8961_WSEQ_DATA_WIDTH_WIDTH: u32 = 3; /* WSEQ_DATA_WIDTH - [6:4] */
pub const WM8961_WSEQ_DATA_START_MASK: u32 = 0x000F; /* WSEQ_DATA_START - [3:0] */
pub const WM8961_WSEQ_DATA_START_SHIFT: u32 = 0; /* WSEQ_DATA_START - [3:0] */
pub const WM8961_WSEQ_DATA_START_WIDTH: u32 = 4; /* WSEQ_DATA_START - [3:0] */

/*
 * R92 (0x5C) - Write Sequencer 6
 */
pub const WM8961_WSEQ_DELAY_MASK: u32 = 0x000F; /* WSEQ_DELAY - [3:0] */
pub const WM8961_WSEQ_DELAY_SHIFT: u32 = 0; /* WSEQ_DELAY - [3:0] */
pub const WM8961_WSEQ_DELAY_WIDTH: u32 = 4; /* WSEQ_DELAY - [3:0] */

/*
 * R93 (0x5D) - Write Sequencer 7
 */
pub const WM8961_WSEQ_BUSY: u32 = 0x0001; /* WSEQ_BUSY */
pub const WM8961_WSEQ_BUSY_MASK: u32 = 0x0001; /* WSEQ_BUSY */
pub const WM8961_WSEQ_BUSY_SHIFT: u32 = 0; /* WSEQ_BUSY */
pub const WM8961_WSEQ_BUSY_WIDTH: u32 = 1; /* WSEQ_BUSY */

/*
 * R252 (0xFC) - General test 1
 */
pub const WM8961_ARA_ENA: u32 = 0x0002; /* ARA_ENA */
pub const WM8961_ARA_ENA_MASK: u32 = 0x0002; /* ARA_ENA */
pub const WM8961_ARA_ENA_SHIFT: u32 = 1; /* ARA_ENA */
pub const WM8961_ARA_ENA_WIDTH: u32 = 1; /* ARA_ENA */
pub const WM8961_AUTO_INC: u32 = 0x0001; /* AUTO_INC */
pub const WM8961_AUTO_INC_MASK: u32 = 0x0001; /* AUTO_INC */
pub const WM8961_AUTO_INC_SHIFT: u32 = 0; /* AUTO_INC */
pub const WM8961_AUTO_INC_WIDTH: u32 = 1; /* AUTO_INC */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
