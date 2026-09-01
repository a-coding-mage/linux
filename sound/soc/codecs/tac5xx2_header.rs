/* SPDX-License-Identifier: GPL-2.0
 *
 * ALSA SoC Texas Instruments TAC5XX2 Audio Smart Amplifier
 *
 * Copyright (C) 2025 Texas Instruments Incorporated
 * https://www.ti.com
 *
 * This the header file for TAC5XX2 family of devices
 * which includes TAC5572, TAC5672, TAC5682 and TAS2883
 *
 * Author: Niranjan H Y <niranjanhy@ti.com>
 */

/* for soundwire */
pub const fn TAC_REG_SDW(book: u32, page: u32, reg: u32) -> u32 {
    (book * 256 * 128) + 0x3000000 + (page * 128) + reg
}

/* page 0 registers */
pub const TAC_SW_RESET: u32 = TAC_REG_SDW(0, 0, 1);
pub const TAC_SLEEP_MODEZ: u32 = TAC_REG_SDW(0, 0, 2);
pub const TAC_FEATURE_PDZ: u32 = TAC_REG_SDW(0, 0, 3);
pub const TAC_TX_CH_EN: u32 = TAC_REG_SDW(0, 0, 4);
pub const TAC_RX_CH_PD: u32 = TAC_REG_SDW(0, 0, 5);
pub const TAC_SHDNZ_CFG: u32 = TAC_REG_SDW(0, 0, 6);
pub const TAC_MISC_CFG0: u32 = TAC_REG_SDW(0, 0, 7);
pub const TAC_MISC_CFG1: u32 = TAC_REG_SDW(0, 0, 8);
pub const TAC_GPIO1_CFG0: u32 = TAC_REG_SDW(0, 0, 9);
pub const TAC_GPIO2_CFG0: u32 = TAC_REG_SDW(0, 0, 10);
pub const TAC_GPIO3_CFG0: u32 = TAC_REG_SDW(0, 0, 11);
pub const TAC_GPIO4_CFG0: u32 = TAC_REG_SDW(0, 0, 12);
pub const TAC_GPIO5_CFG0: u32 = TAC_REG_SDW(0, 0, 13);
pub const TAC_GPIO6_CFG0: u32 = TAC_REG_SDW(0, 0, 14);
pub const TAC_INTF_CFG1: u32 = TAC_REG_SDW(0, 0, 15);
pub const TAC_INTF_CFG5: u32 = TAC_REG_SDW(0, 0, 16);
pub const TAC_PASI_BCLK_CFG0: u32 = TAC_REG_SDW(0, 0, 17);
pub const TAC_PASI_FSYNC_CFG0: u32 = TAC_REG_SDW(0, 0, 18);
pub const TAC_PASI_DIN1_CFG0: u32 = TAC_REG_SDW(0, 0, 19);
pub const TAC_PASI_DIN2_CFG0: u32 = TAC_REG_SDW(0, 0, 20);
pub const TAC_PDM_DIN1_CFG0: u32 = TAC_REG_SDW(0, 0, 21);
pub const TAC_PDM_DIN2_CFG0: u32 = TAC_REG_SDW(0, 0, 22);
pub const TAC_MCLK_SEL: u32 = TAC_REG_SDW(0, 0, 23);
pub const TAC_I2C2_CFG0: u32 = TAC_REG_SDW(0, 0, 24);
pub const TAC_SDW_IO_CFG0: u32 = TAC_REG_SDW(0, 0, 25);
pub const TAC_SDW_CLK_CFG0: u32 = TAC_REG_SDW(0, 0, 26);
pub const TAC_PASI_CFG0: u32 = TAC_REG_SDW(0, 0, 27);
pub const TAC_PASI_CFG1: u32 = TAC_REG_SDW(0, 0, 28);
pub const TAC_PASI_TX_CFG0: u32 = TAC_REG_SDW(0, 0, 29);
pub const TAC_PASI_TX_CFG1: u32 = TAC_REG_SDW(0, 0, 30);
pub const TAC_PASI_TX_CFG2: u32 = TAC_REG_SDW(0, 0, 31);
pub const TAC_PASI_TX_CFG3: u32 = TAC_REG_SDW(0, 0, 32);
pub const TAC_PASI_TX_CH1_CFG0: u32 = TAC_REG_SDW(0, 0, 33);
pub const TAC_PASI_TX_CH2_CFG0: u32 = TAC_REG_SDW(0, 0, 34);
pub const TAC_PASI_TX_CH3_CFG0: u32 = TAC_REG_SDW(0, 0, 35);
pub const TAC_PASI_TX_CH4_CFG0: u32 = TAC_REG_SDW(0, 0, 36);
pub const TAC_PASI_TX_CH5_CFG0: u32 = TAC_REG_SDW(0, 0, 37);
pub const TAC_PASI_TX_CH6_CFG0: u32 = TAC_REG_SDW(0, 0, 38);
pub const TAC_PASI_TX_CH7_CFG0: u32 = TAC_REG_SDW(0, 0, 39);
pub const TAC_PASI_TX_CH8_CFG0: u32 = TAC_REG_SDW(0, 0, 40);
pub const TAC_PASI_RX_CFG0: u32 = TAC_REG_SDW(0, 0, 41);
pub const TAC_PASI_RX_CFG1: u32 = TAC_REG_SDW(0, 0, 42);
pub const TAC_PASI_RX_CFG2: u32 = TAC_REG_SDW(0, 0, 43);
pub const TAC_PASI_RX_CH1_CFG0: u32 = TAC_REG_SDW(0, 0, 44);
pub const TAC_PASI_RX_CH2_CFG0: u32 = TAC_REG_SDW(0, 0, 45);
pub const TAC_PASI_RX_CH3_CFG0: u32 = TAC_REG_SDW(0, 0, 46);
pub const TAC_PASI_RX_CH4_CFG0: u32 = TAC_REG_SDW(0, 0, 47);
pub const TAC_PASI_RX_CH5_CFG0: u32 = TAC_REG_SDW(0, 0, 48);
pub const TAC_PASI_RX_CH6_CFG0: u32 = TAC_REG_SDW(0, 0, 49);
pub const TAC_PASI_RX_CH7_CFG0: u32 = TAC_REG_SDW(0, 0, 50);
pub const TAC_PASI_RX_CH8_CFG0: u32 = TAC_REG_SDW(0, 0, 51);
pub const TAC_ADC_CH1_CFG0: u32 = TAC_REG_SDW(0, 0, 52);
pub const TAC_ADC_DVOL_CFG0: u32 = TAC_REG_SDW(0, 0, 53);
pub const TAC_ADC_CH1_FGAIN: u32 = TAC_REG_SDW(0, 0, 54);
pub const TAC_ADC_CH1_CFG1: u32 = TAC_REG_SDW(0, 0, 55);
pub const TAC_ADC_CH2_CFG0: u32 = TAC_REG_SDW(0, 0, 57);
pub const TAC_ADC_DVOL_CFG1: u32 = TAC_REG_SDW(0, 0, 58);
pub const TAC_ADC_CH2_FGAIN: u32 = TAC_REG_SDW(0, 0, 59);
pub const TAC_ADC_CH2_CFG1: u32 = TAC_REG_SDW(0, 0, 60);
pub const TAC_ADC_CFG1: u32 = TAC_REG_SDW(0, 0, 62);
pub const TAC_PDM_CH1_DVOL: u32 = TAC_REG_SDW(0, 0, 63);
pub const TAC_PDM_CH1_FGAIN: u32 = TAC_REG_SDW(0, 0, 64);
pub const TAC_PDM_CH1_CFG0: u32 = TAC_REG_SDW(0, 0, 65);
pub const TAC_PDM_CH2_DVOL: u32 = TAC_REG_SDW(0, 0, 67);
pub const TAC_PDM_CH2_FGAIN: u32 = TAC_REG_SDW(0, 0, 68);
pub const TAC_PDM_CH2_CFG2: u32 = TAC_REG_SDW(0, 0, 69);
pub const TAC_PDM_CH3_DVOL: u32 = TAC_REG_SDW(0, 0, 71);
pub const TAC_PDM_CH3_FGAIN: u32 = TAC_REG_SDW(0, 0, 72);
pub const TAC_PDM_CH3_CFG0: u32 = TAC_REG_SDW(0, 0, 73);
pub const TAC_PDM_CH4_DVOL: u32 = TAC_REG_SDW(0, 0, 75);
pub const TAC_PDM_CH4_FGAIN: u32 = TAC_REG_SDW(0, 0, 76);
pub const TAC_PDM_CH4_CFG0: u32 = TAC_REG_SDW(0, 0, 77);
pub const TAC_MICBIAS_CFG0: u32 = TAC_REG_SDW(0, 0, 79);
pub const TAC_MICPREAMP_CFG: u32 = TAC_REG_SDW(0, 0, 80);
pub const TAC_MICBIAS_CFG1: u32 = TAC_REG_SDW(0, 0, 81);
pub const TAC_CLASSD_CH1_DVOL: u32 = TAC_REG_SDW(0, 0, 82);
pub const TAC_CLASSD_CH1_FGAIN: u32 = TAC_REG_SDW(0, 0, 83);
pub const TAC_CLASSD_CH2_DVOL: u32 = TAC_REG_SDW(0, 0, 85);
pub const TAC_CLASSD_CH2_FGAIN: u32 = TAC_REG_SDW(0, 0, 86);
pub const TAC_GCHP_CH1_DVOL: u32 = TAC_REG_SDW(0, 0, 88);
pub const TAC_GCHP_CH1_FGAIN: u32 = TAC_REG_SDW(0, 0, 89);
pub const TAC_GCHP_CH2_DVOL: u32 = TAC_REG_SDW(0, 0, 91);
pub const TAC_GCHP_CH2_FGAIN: u32 = TAC_REG_SDW(0, 0, 92);
pub const TAC_AMP_LVL_CFG0: u32 = TAC_REG_SDW(0, 0, 94);
pub const TAC_AMP_LVL_CFG1: u32 = TAC_REG_SDW(0, 0, 95);
pub const TAC_AMP_LVL_CFG2: u32 = TAC_REG_SDW(0, 0, 96);
pub const TAC_AMP_LVL_CFG3: u32 = TAC_REG_SDW(0, 0, 97);
pub const TAC_EFF_MODE_CFG0: u32 = TAC_REG_SDW(0, 0, 98);
pub const TAC_EFF_MODE_CFG1: u32 = TAC_REG_SDW(0, 0, 99);
pub const TAC_CLASSD_CFG0: u32 = TAC_REG_SDW(0, 0, 100);
pub const TAC_CLASSD_CFG1: u32 = TAC_REG_SDW(0, 0, 101);
pub const TAC_CLASSD_CFG3: u32 = TAC_REG_SDW(0, 0, 102);
pub const TAC_CLASSD_CFG4: u32 = TAC_REG_SDW(0, 0, 103);
pub const TAC_CLASSD_CFG5: u32 = TAC_REG_SDW(0, 0, 104);
pub const TAC_CLASSD_CFG6: u32 = TAC_REG_SDW(0, 0, 105);
pub const TAC_CLASSD_CFG8: u32 = TAC_REG_SDW(0, 0, 106);
pub const TAC_ISNS_CFG: u32 = TAC_REG_SDW(0, 0, 107);
pub const TAC_DSP_CFG0: u32 = TAC_REG_SDW(0, 0, 108);
pub const TAC_DSP_CFG1: u32 = TAC_REG_SDW(0, 0, 109);
pub const TAC_DSP_CFG2: u32 = TAC_REG_SDW(0, 0, 110);
pub const TAC_DSP_CFG3: u32 = TAC_REG_SDW(0, 0, 111);
pub const TAC_JACK_DET_CFG1: u32 = TAC_REG_SDW(0, 0, 112);
pub const TAC_JACK_DET_CFG2: u32 = TAC_REG_SDW(0, 0, 113);
pub const TAC_JACK_DET_CFG3: u32 = TAC_REG_SDW(0, 0, 114);
pub const TAC_JACK_DET_CFG4: u32 = TAC_REG_SDW(0, 0, 115);
pub const TAC_JACK_DET_CFG7: u32 = TAC_REG_SDW(0, 0, 116);
pub const TAC_UJ_IMPEDANCE_L: u32 = TAC_REG_SDW(0, 0, 117);
pub const TAC_UJ_IMPEDANCE_R: u32 = TAC_REG_SDW(0, 0, 118);
pub const UJ_IMPEDANCE_L: u32 = TAC_REG_SDW(0, 0, 119);
pub const UJ_IMPEDANCE_R: u32 = TAC_REG_SDW(0, 0, 120);
pub const TAC_GP_ANA_STS: u32 = TAC_REG_SDW(0, 0, 123);
pub const TAC_DEV_ID: u32 = TAC_REG_SDW(0, 0, 124);
pub const TAC_REV_ID: u32 = TAC_REG_SDW(0, 0, 125);
pub const TAC_I2C_CKSUM: u32 = TAC_REG_SDW(0, 0, 126);
pub const TAC_BOOK: u32 = TAC_REG_SDW(0, 0, 127);

pub const TAC_INT_CFG: u32 = TAC_REG_SDW(0, 2, 1);
pub const TAC_INT_CFG_CLR_REG: u32 = 1 << 3;

/* smartamp function */
pub const TAC_FUNCTION_ID_SA: u32 = 0x1;

pub const TAC_SDCA_ENT_ENT0: u32 = 0x0;
pub const TAC_SDCA_ENT_PPU21: u32 = 0x1;
pub const TAC_SDCA_ENT_FU21: u32 = 0x2;
pub const TAC_SDCA_ENT_FU26: u32 = 0x3;
pub const TAC_SDCA_ENT_XU22: u32 = 0x4;
pub const TAC_SDCA_ENT_CS24: u32 = 0x5;
pub const TAC_SDCA_ENT_CS21: u32 = 0x6;
pub const TAC_SDCA_ENT_CS25: u32 = 0x7;
pub const TAC_SDCA_ENT_CS26: u32 = 0x8;
pub const TAC_SDCA_ENT_CS28: u32 = 0x9;
pub const TAC_SDCA_ENT_PPU26: u32 = 0xa;
pub const TAC_SDCA_ENT_FU23: u32 = 0xb;
pub const TAC_SDCA_ENT_PDE23: u32 = 0xc;
pub const TAC_SDCA_ENT_TG23: u32 = 0x12;
pub const TAC_SDCA_ENT_IT21: u32 = 0x13;
pub const TAC_SDCA_ENT_IT29: u32 = 0x14;
pub const TAC_SDCA_ENT_IT26: u32 = 0x15;
pub const TAC_SDCA_ENT_IT28: u32 = 0x16;
pub const TAC_SDCA_ENT_OT24: u32 = 0x17;
pub const TAC_SDCA_ENT_OT23: u32 = 0x18;
pub const TAC_SDCA_ENT_OT25: u32 = 0x19;
pub const TAC_SDCA_ENT_OT28: u32 = 0x1a;
pub const TAC_SDCA_ENT_OT27: u32 = 0x1c;
pub const TAC_SDCA_ENT_SPE199: u32 = 0x21;
pub const TAC_SDCA_ENT_OT20: u32 = 0x24;
pub const TAC_SDCA_ENT_FU27: u32 = 0x26;
pub const TAC_SDCA_ENT_FU20: u32 = 0x27;
pub const TAC_SDCA_ENT_PDE24: u32 = 0x2e;
pub const TAC_SDCA_ENT_PDE27: u32 = 0x2f;
pub const TAC_SDCA_ENT_PDE28: u32 = 0x30;
pub const TAC_SDCA_ENT_PDE20: u32 = 0x31;
pub const TAC_SDCA_ENT_SAPU29: u32 = 0x35;

/* Control selector definitions */
pub const TAC_SDCA_MASTER_MUTE: u32 = 0x01;
pub const TAC_SDCA_CHANNEL_MUTE: u32 = 0x01;
pub const TAC_SDCA_CHANNEL_VOLUME: u32 = 0x02;
pub const TAC_SDCA_POSTURENUMBER: u32 = 0x10;
pub const TAC_SDCA_REQUESTED_PS: u32 = 0x01;
pub const TAC_SDCA_ACTUAL_PS: u32 = 0x10;
pub const TAC_SDCA_CHANNEL_GAIN: u32 = 0x0B;

/* 2. smart mic function */
pub const TAC_FUNCTION_ID_SM: u32 = 0x2;

pub const TAC_SDCA_ENT_IT11: u32 = 0x1;
pub const TAC_SDCA_ENT_OT113: u32 = 0x2;
pub const TAC_SDCA_ENT_CS11: u32 = 0x3;
pub const TAC_SDCA_ENT_CS18: u32 = 0x4;
pub const TAC_SDCA_ENT_FU113: u32 = 0x5;
pub const TAC_SDCA_ENT_FU13: u32 = 0x6;
pub const TAC_SDCA_ENT_FU11: u32 = 0x8;
pub const TAC_SDCA_ENT_XU12: u32 = 0xa;
pub const TAC_SDCA_ENT_CS113: u32 = 0xc;
pub const TAC_SDCA_ENT_CX11: u32 = 0xf;
pub const TAC_SDCA_ENT_PDE11: u32 = 0x12;
pub const TAC_SDCA_ENT_PPU11: u32 = 0x9;

/* controls */
pub const TAC_SDCA_CTL_USAGE: u32 = 0x04;
pub const TAC_SDCA_CTL_IT_CLUSTER: u32 = 0x10;
pub const TAC_SDCA_CTL_OT_DP_SEL: u32 = 0x11;
pub const TAC_SDCA_CTL_XU_BYPASS: u32 = 0x01;
/* cx */
pub const TAC_SDCA_CTL_CX_CLK_SEL: u32 = 0x01;
/* cs */
pub const TAC_SDCA_CTL_CS_CLKVLD: u32 = 0x02;
pub const TAC_SDCA_CTL_CS_SAMP_RATE_IDX: u32 = 0x10;
/* cs113 end */
/* ppu */
pub const TAC_SDCA_CTL_PPU_POSTURE_NUM: u32 = 0x10;

/* 3. UAJ function */
pub const TAC_FUNCTION_ID_UAJ: u32 = 0x3;
pub const TAC_SDCA_ENT_PDE47: u32 = 0x35;
pub const TAC_SDCA_ENT_PDE34: u32 = 0x32;
pub const TAC_SDCA_ENT_FU41: u32 = 0x26; /* user */
pub const TAC_SDCA_ENT_IT41: u32 = 0x07;
pub const TAC_SDCA_ENT_XU42: u32 = 0x2C;
pub const TAC_SDCA_ENT_CS41: u32 = 0x30;
pub const TAC_SDCA_ENT_OT45: u32 = 0x0E;
pub const TAC_SDCA_ENT_IT33: u32 = 0x03;
pub const TAC_SDCA_ENT_OT36: u32 = 0x0A;
pub const TAC_SDCA_ENT_FU36: u32 = 0x28;
pub const TAC_SDCA_ENT_CS36: u32 = 0x2E;
pub const TAC_SDCA_ENT_GE35: u32 = 0x3B; /* 59 */

pub const TAC_SDCA_CTL_SEL_MODE: u32 = 0x1;
pub const TAC_SDCA_CTL_DET_MODE: u32 = 0x2;

/* 4. HID function */
pub const TAC_FUNCTION_ID_HID: u32 = 0x4;
pub const TAC_SDCA_ENT_HID1: u32 = 0x1;
/* HID Control Selectors */
pub const TAC_SDCA_CTL_HIDTX_CURRENT_OWNER: u32 = 0x10;
pub const TAC_SDCA_CTL_HIDTX_MESSAGE_OFFSET: u32 = 0x12;
pub const TAC_SDCA_CTL_HIDTX_MESSAGE_LENGTH: u32 = 0x13;
pub const TAC_SDCA_CTL_DETECTED_MODE: u32 = 0x10;
pub const TAC_SDCA_CTL_SELECTED_MODE: u32 = 0x11;

pub const TAC_BUF_ADDR_HID1: u32 = 0x44007F80;

/* DAI interfaces */
pub const TAC5XX2_SPK: u32 = 0;
pub const TAC5XX2_DMIC: u32 = 2;
pub const TAC5XX2_UAJ: u32 = 3;

/* Port numbers for DAIs */
pub const TAC_SDW_PORT_NUM_SPK_PLAYBACK: u32 = 1;
pub const TAC_SDW_PORT_NUM_SPK_CAPTURE: u32 = 2;
pub const TAC_SDW_PORT_NUM_DMIC: u32 = 3;
pub const TAC_SDW_PORT_NUM_UAJ_PLAYBACK: u32 = 4;
pub const TAC_SDW_PORT_NUM_UAJ_CAPTURE: u32 = 7;
pub const TAC_SDW_PORT_NUM_IV_SENSE: u32 = 8;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
