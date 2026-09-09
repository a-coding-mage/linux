/* SPDX-License-Identifier: GPL-2.0+
 *
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal interface for Audio Codec '97
 *
 *  For more details look to AC '97 component specification revision 2.1
 *  by Intel Corporation (http://developer.intel.com).
 */
/* AC'97 codec registers */

pub const AC97_RESET: u16 = 0x00; pub const AC97_MASTER: u16 = 0x02; pub const AC97_HEADPHONE: u16 = 0x04;
pub const AC97_MASTER_MONO: u16 = 0x06; pub const AC97_MASTER_TONE: u16 = 0x08; pub const AC97_PC_BEEP: u16 = 0x0a;
pub const AC97_PHONE: u16 = 0x0c; pub const AC97_MIC: u16 = 0x0e; pub const AC97_LINE: u16 = 0x10;
pub const AC97_CD: u16 = 0x12; pub const AC97_VIDEO: u16 = 0x14; pub const AC97_AUX: u16 = 0x16;
pub const AC97_PCM: u16 = 0x18; pub const AC97_REC_SEL: u16 = 0x1a; pub const AC97_REC_GAIN: u16 = 0x1c;
pub const AC97_REC_GAIN_MIC: u16 = 0x1e; pub const AC97_GENERAL_PURPOSE: u16 = 0x20; pub const AC97_3D_CONTROL: u16 = 0x22;
pub const AC97_INT_PAGING: u16 = 0x24; pub const AC97_POWERDOWN: u16 = 0x26;
// range 0x28-0x3a - AUDIO AC'97 2.0 extensions
pub const AC97_EXTENDED_ID: u16 = 0x28; pub const AC97_EXTENDED_STATUS: u16 = 0x2a;
pub const AC97_PCM_FRONT_DAC_RATE: u16 = 0x2c; pub const AC97_PCM_SURR_DAC_RATE: u16 = 0x2e; pub const AC97_PCM_LFE_DAC_RATE: u16 = 0x30;
pub const AC97_PCM_LR_ADC_RATE: u16 = 0x32; pub const AC97_PCM_MIC_ADC_RATE: u16 = 0x34; pub const AC97_CENTER_LFE_MASTER: u16 = 0x36;
pub const AC97_SURROUND_MASTER: u16 = 0x38; pub const AC97_SPDIF: u16 = 0x3a;
// range 0x3c-0x58 - MODEM
pub const AC97_EXTENDED_MID: u16 = 0x3c; pub const AC97_EXTENDED_MSTATUS: u16 = 0x3e; pub const AC97_LINE1_RATE: u16 = 0x40;
pub const AC97_LINE2_RATE: u16 = 0x42; pub const AC97_HANDSET_RATE: u16 = 0x44; pub const AC97_LINE1_LEVEL: u16 = 0x46;
pub const AC97_LINE2_LEVEL: u16 = 0x48; pub const AC97_HANDSET_LEVEL: u16 = 0x4a; pub const AC97_GPIO_CFG: u16 = 0x4c;
pub const AC97_GPIO_POLARITY: u16 = 0x4e; pub const AC97_GPIO_STICKY: u16 = 0x50; pub const AC97_GPIO_WAKEUP: u16 = 0x52;
pub const AC97_GPIO_STATUS: u16 = 0x54; pub const AC97_MISC_AFE: u16 = 0x56;
// range 0x5a-0x7b - Vendor Specific
pub const AC97_VENDOR_ID1: u16 = 0x7c; pub const AC97_VENDOR_ID2: u16 = 0x7e;
// range 0x60-0x6f (page 1) - extended codec registers
pub const AC97_CODEC_CLASS_REV: u16 = 0x60; pub const AC97_PCI_SVID: u16 = 0x62; pub const AC97_PCI_SID: u16 = 0x64;
pub const AC97_FUNC_SELECT: u16 = 0x66; pub const AC97_FUNC_INFO: u16 = 0x68; pub const AC97_SENSE_INFO: u16 = 0x6a;

pub const AC97_MUTE_MASK_MONO: u16 = 0x8000; pub const AC97_MUTE_MASK_STEREO: u16 = 0x8080;
pub const AC97_SLOT_TAG: u16 = 0; pub const AC97_SLOT_CMD_ADDR: u16 = 1; pub const AC97_SLOT_CMD_DATA: u16 = 2;
pub const AC97_SLOT_PCM_LEFT: u16 = 3; pub const AC97_SLOT_PCM_RIGHT: u16 = 4; pub const AC97_SLOT_MODEM_LINE1: u16 = 5;
pub const AC97_SLOT_PCM_CENTER: u16 = 6; pub const AC97_SLOT_MIC: u16 = 6; pub const AC97_SLOT_SPDIF_LEFT1: u16 = 6;
pub const AC97_SLOT_PCM_SLEFT: u16 = 7; pub const AC97_SLOT_PCM_LEFT_0: u16 = 7; pub const AC97_SLOT_SPDIF_LEFT: u16 = 7;
pub const AC97_SLOT_PCM_SRIGHT: u16 = 8; pub const AC97_SLOT_PCM_RIGHT_0: u16 = 8; pub const AC97_SLOT_SPDIF_RIGHT: u16 = 8;
pub const AC97_SLOT_LFE: u16 = 9; pub const AC97_SLOT_SPDIF_RIGHT1: u16 = 9; pub const AC97_SLOT_MODEM_LINE2: u16 = 10;
pub const AC97_SLOT_PCM_LEFT_1: u16 = 10; pub const AC97_SLOT_SPDIF_LEFT2: u16 = 10; pub const AC97_SLOT_HANDSET: u16 = 11;
pub const AC97_SLOT_PCM_RIGHT_1: u16 = 11; pub const AC97_SLOT_SPDIF_RIGHT2: u16 = 11; pub const AC97_SLOT_MODEM_GPIO: u16 = 12; pub const AC97_SLOT_PCM_CENTER_1: u16 = 12;

pub const AC97_BC_DEDICATED_MIC: u16 = 0x0001; pub const AC97_BC_RESERVED1: u16 = 0x0002; pub const AC97_BC_BASS_TREBLE: u16 = 0x0004; pub const AC97_BC_SIM_STEREO: u16 = 0x0008;
pub const AC97_BC_HEADPHONE: u16 = 0x0010; pub const AC97_BC_LOUDNESS: u16 = 0x0020; pub const AC97_BC_16BIT_DAC: u16 = 0x0000; pub const AC97_BC_18BIT_DAC: u16 = 0x0040; pub const AC97_BC_20BIT_DAC: u16 = 0x0080; pub const AC97_BC_DAC_MASK: u16 = 0x00c0;
pub const AC97_BC_16BIT_ADC: u16 = 0x0000; pub const AC97_BC_18BIT_ADC: u16 = 0x0100; pub const AC97_BC_20BIT_ADC: u16 = 0x0200; pub const AC97_BC_ADC_MASK: u16 = 0x0300; pub const AC97_BC_3D_TECH_ID_MASK: u16 = 0x7c00;
pub const AC97_GP_DRSS_MASK: u16 = 0x0c00; pub const AC97_GP_DRSS_1011: u16 = 0x0000; pub const AC97_GP_DRSS_78: u16 = 0x0400;
pub const AC97_PD_ADC_STATUS: u16 = 0x0001; pub const AC97_PD_DAC_STATUS: u16 = 0x0002; pub const AC97_PD_MIXER_STATUS: u16 = 0x0004; pub const AC97_PD_VREF_STATUS: u16 = 0x0008;
pub const AC97_PD_PR0: u16 = 0x0100; pub const AC97_PD_PR1: u16 = 0x0200; pub const AC97_PD_PR2: u16 = 0x0400; pub const AC97_PD_PR3: u16 = 0x0800; pub const AC97_PD_PR4: u16 = 0x1000; pub const AC97_PD_PR5: u16 = 0x2000; pub const AC97_PD_PR6: u16 = 0x4000; pub const AC97_PD_EAPD: u16 = 0x8000;

pub const AC97_EI_VRA: u16 = 1; pub const AC97_EI_DRA: u16 = 2; pub const AC97_EI_SPDIF: u16 = 4; pub const AC97_EI_VRM: u16 = 8; pub const AC97_EI_DACS_SLOT_MASK: u16 = 0x0030; pub const AC97_EI_DACS_SLOT_SHIFT: u16 = 4; pub const AC97_EI_CDAC: u16 = 0x0040; pub const AC97_EI_SDAC: u16 = 0x0080; pub const AC97_EI_LDAC: u16 = 0x0100; pub const AC97_EI_AMAP: u16 = 0x0200; pub const AC97_EI_REV_MASK: u16 = 0x0c00; pub const AC97_EI_REV_22: u16 = 0x0400; pub const AC97_EI_REV_23: u16 = 0x0800; pub const AC97_EI_REV_SHIFT: u16 = 10; pub const AC97_EI_ADDR_MASK: u16 = 0xc000; pub const AC97_EI_ADDR_SHIFT: u16 = 14;
pub const AC97_EA_VRA: u16 = 1; pub const AC97_EA_DRA: u16 = 2; pub const AC97_EA_SPDIF: u16 = 4; pub const AC97_EA_VRM: u16 = 8; pub const AC97_EA_SPSA_SLOT_MASK: u16 = 0x0030; pub const AC97_EA_SPSA_SLOT_SHIFT: u16 = 4; pub const AC97_EA_SPSA_3_4: u16 = 0; pub const AC97_EA_SPSA_7_8: u16 = 0x0010; pub const AC97_EA_SPSA_6_9: u16 = 0x0020; pub const AC97_EA_SPSA_10_11: u16 = 0x0030; pub const AC97_EA_CDAC: u16 = 0x0040; pub const AC97_EA_SDAC: u16 = 0x0080; pub const AC97_EA_LDAC: u16 = 0x0100; pub const AC97_EA_MDAC: u16 = 0x0200; pub const AC97_EA_SPCV: u16 = 0x0400; pub const AC97_EA_PRI: u16 = 0x0800; pub const AC97_EA_PRJ: u16 = 0x1000; pub const AC97_EA_PRK: u16 = 0x2000; pub const AC97_EA_PRL: u16 = 0x4000;

pub const AC97_SC_PRO: u16 = 1; pub const AC97_SC_NAUDIO: u16 = 2; pub const AC97_SC_COPY: u16 = 4; pub const AC97_SC_PRE: u16 = 8; pub const AC97_SC_CC_MASK: u16 = 0x07f0; pub const AC97_SC_CC_SHIFT: u16 = 4; pub const AC97_SC_L: u16 = 0x0800; pub const AC97_SC_SPSR_MASK: u16 = 0x3000; pub const AC97_SC_SPSR_SHIFT: u16 = 12; pub const AC97_SC_SPSR_44K: u16 = 0; pub const AC97_SC_SPSR_48K: u16 = 0x2000; pub const AC97_SC_SPSR_32K: u16 = 0x3000; pub const AC97_SC_DRS: u16 = 0x4000; pub const AC97_SC_V: u16 = 0x8000;
pub const AC97_PAGE_MASK: u16 = 0x000f; pub const AC97_PAGE_VENDOR: u16 = 0; pub const AC97_PAGE_1: u16 = 1; pub const AC97_INT_ENABLE: u16 = 0x0800; pub const AC97_INT_SENSE: u16 = 0x1000; pub const AC97_INT_CAUSE_SENSE: u16 = 0x2000; pub const AC97_INT_CAUSE_GPIO: u16 = 0x4000; pub const AC97_INT_STATUS: u16 = 0x8000;
pub const AC97_MEI_LINE1: u16 = 1; pub const AC97_MEI_LINE2: u16 = 2; pub const AC97_MEI_HANDSET: u16 = 4; pub const AC97_MEI_CID1: u16 = 8; pub const AC97_MEI_CID2: u16 = 0x0010; pub const AC97_MEI_ADDR_MASK: u16 = 0xc000; pub const AC97_MEI_ADDR_SHIFT: u16 = 14;
pub const AC97_MEA_GPIO: u16 = 1; pub const AC97_MEA_MREF: u16 = 2; pub const AC97_MEA_ADC1: u16 = 4; pub const AC97_MEA_DAC1: u16 = 8; pub const AC97_MEA_ADC2: u16 = 0x10; pub const AC97_MEA_DAC2: u16 = 0x20; pub const AC97_MEA_HADC: u16 = 0x40; pub const AC97_MEA_HDAC: u16 = 0x80; pub const AC97_MEA_PRA: u16 = 0x0100; pub const AC97_MEA_PRB: u16 = 0x0200; pub const AC97_MEA_PRC: u16 = 0x0400; pub const AC97_MEA_PRD: u16 = 0x0800; pub const AC97_MEA_PRE: u16 = 0x1000; pub const AC97_MEA_PRF: u16 = 0x2000; pub const AC97_MEA_PRG: u16 = 0x4000; pub const AC97_MEA_PRH: u16 = 0x8000;

pub const AC97_GPIO_LINE1_OH: u16 = 0x0001; pub const AC97_GPIO_LINE1_RI: u16 = 0x0002; pub const AC97_GPIO_LINE1_CID: u16 = 0x0004; pub const AC97_GPIO_LINE1_LCS: u16 = 0x0008; pub const AC97_GPIO_LINE1_PULSE: u16 = 0x0010; pub const AC97_GPIO_LINE1_HL1R: u16 = 0x0020; pub const AC97_GPIO_LINE1_HOHD: u16 = 0x0040; pub const AC97_GPIO_LINE12_AC: u16 = 0x0080; pub const AC97_GPIO_LINE12_DC: u16 = 0x0100; pub const AC97_GPIO_LINE12_RS: u16 = 0x0200; pub const AC97_GPIO_LINE2_OH: u16 = 0x0400; pub const AC97_GPIO_LINE2_RI: u16 = 0x0800; pub const AC97_GPIO_LINE2_CID: u16 = 0x1000; pub const AC97_GPIO_LINE2_LCS: u16 = 0x2000; pub const AC97_GPIO_LINE2_PULSE: u16 = 0x4000; pub const AC97_GPIO_LINE2_HL1R: u16 = 0x8000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
