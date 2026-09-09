/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>
 */

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>
 */


/* TOPCKGEN */

pub const CLK_TOP_AXI_SEL: u32 = 0;
pub const CLK_TOP_SPM_SEL: u32 = 1;
pub const CLK_TOP_SCP_SEL: u32 = 2;
pub const CLK_TOP_BUS_AXIMEM_SEL: u32 = 3;
pub const CLK_TOP_DISP_SEL: u32 = 4;
pub const CLK_TOP_MDP_SEL: u32 = 5;
pub const CLK_TOP_IMG1_SEL: u32 = 6;
pub const CLK_TOP_IMG2_SEL: u32 = 7;
pub const CLK_TOP_IPE_SEL: u32 = 8;
pub const CLK_TOP_DPE_SEL: u32 = 9;
pub const CLK_TOP_CAM_SEL: u32 = 10;
pub const CLK_TOP_CCU_SEL: u32 = 11;
pub const CLK_TOP_DSP7_SEL: u32 = 12;
pub const CLK_TOP_MFG_REF_SEL: u32 = 13;
pub const CLK_TOP_MFG_PLL_SEL: u32 = 14;
pub const CLK_TOP_CAMTG_SEL: u32 = 15;
pub const CLK_TOP_CAMTG2_SEL: u32 = 16;
pub const CLK_TOP_CAMTG3_SEL: u32 = 17;
pub const CLK_TOP_CAMTG4_SEL: u32 = 18;
pub const CLK_TOP_CAMTG5_SEL: u32 = 19;
pub const CLK_TOP_CAMTG6_SEL: u32 = 20;
pub const CLK_TOP_UART_SEL: u32 = 21;
pub const CLK_TOP_SPI_SEL: u32 = 22;
pub const CLK_TOP_MSDC50_0_H_SEL: u32 = 23;
pub const CLK_TOP_MSDC50_0_SEL: u32 = 24;
pub const CLK_TOP_MSDC30_1_SEL: u32 = 25;
pub const CLK_TOP_MSDC30_2_SEL: u32 = 26;
pub const CLK_TOP_AUDIO_SEL: u32 = 27;
pub const CLK_TOP_AUD_INTBUS_SEL: u32 = 28;
pub const CLK_TOP_PWRAP_ULPOSC_SEL: u32 = 29;
pub const CLK_TOP_ATB_SEL: u32 = 30;
pub const CLK_TOP_DPI_SEL: u32 = 31;
pub const CLK_TOP_SCAM_SEL: u32 = 32;
pub const CLK_TOP_DISP_PWM_SEL: u32 = 33;
pub const CLK_TOP_USB_TOP_SEL: u32 = 34;
pub const CLK_TOP_SSUSB_XHCI_SEL: u32 = 35;
pub const CLK_TOP_I2C_SEL: u32 = 36;
pub const CLK_TOP_SENINF_SEL: u32 = 37;
pub const CLK_TOP_SENINF1_SEL: u32 = 38;
pub const CLK_TOP_SENINF2_SEL: u32 = 39;
pub const CLK_TOP_SENINF3_SEL: u32 = 40;
pub const CLK_TOP_TL_SEL: u32 = 41;
pub const CLK_TOP_DXCC_SEL: u32 = 42;
pub const CLK_TOP_AUD_ENGEN1_SEL: u32 = 43;
pub const CLK_TOP_AUD_ENGEN2_SEL: u32 = 44;
pub const CLK_TOP_AES_UFSFDE_SEL: u32 = 45;
pub const CLK_TOP_UFS_SEL: u32 = 46;
pub const CLK_TOP_AUD_1_SEL: u32 = 47;
pub const CLK_TOP_AUD_2_SEL: u32 = 48;
pub const CLK_TOP_ADSP_SEL: u32 = 49;
pub const CLK_TOP_DPMAIF_MAIN_SEL: u32 = 50;
pub const CLK_TOP_VENC_SEL: u32 = 51;
pub const CLK_TOP_VDEC_SEL: u32 = 52;
pub const CLK_TOP_CAMTM_SEL: u32 = 53;
pub const CLK_TOP_PWM_SEL: u32 = 54;
pub const CLK_TOP_AUDIO_H_SEL: u32 = 55;
pub const CLK_TOP_SPMI_MST_SEL: u32 = 56;
pub const CLK_TOP_AES_MSDCFDE_SEL: u32 = 57;
pub const CLK_TOP_SFLASH_SEL: u32 = 58;
pub const CLK_TOP_APLL_I2S0_M_SEL: u32 = 59;
pub const CLK_TOP_APLL_I2S1_M_SEL: u32 = 60;
pub const CLK_TOP_APLL_I2S2_M_SEL: u32 = 61;
pub const CLK_TOP_APLL_I2S3_M_SEL: u32 = 62;
pub const CLK_TOP_APLL_I2S4_M_SEL: u32 = 63;
pub const CLK_TOP_APLL_I2S5_M_SEL: u32 = 64;
pub const CLK_TOP_APLL_I2S6_M_SEL: u32 = 65;
pub const CLK_TOP_APLL_I2S7_M_SEL: u32 = 66;
pub const CLK_TOP_APLL_I2S8_M_SEL: u32 = 67;
pub const CLK_TOP_APLL_I2S9_M_SEL: u32 = 68;
pub const CLK_TOP_MAINPLL_D3: u32 = 69;
pub const CLK_TOP_MAINPLL_D4: u32 = 70;
pub const CLK_TOP_MAINPLL_D4_D2: u32 = 71;
pub const CLK_TOP_MAINPLL_D4_D4: u32 = 72;
pub const CLK_TOP_MAINPLL_D4_D8: u32 = 73;
pub const CLK_TOP_MAINPLL_D4_D16: u32 = 74;
pub const CLK_TOP_MAINPLL_D5: u32 = 75;
pub const CLK_TOP_MAINPLL_D5_D2: u32 = 76;
pub const CLK_TOP_MAINPLL_D5_D4: u32 = 77;
pub const CLK_TOP_MAINPLL_D5_D8: u32 = 78;
pub const CLK_TOP_MAINPLL_D6: u32 = 79;
pub const CLK_TOP_MAINPLL_D6_D2: u32 = 80;
pub const CLK_TOP_MAINPLL_D6_D4: u32 = 81;
pub const CLK_TOP_MAINPLL_D7: u32 = 82;
pub const CLK_TOP_MAINPLL_D7_D2: u32 = 83;
pub const CLK_TOP_MAINPLL_D7_D4: u32 = 84;
pub const CLK_TOP_MAINPLL_D7_D8: u32 = 85;
pub const CLK_TOP_UNIVPLL_D3: u32 = 86;
pub const CLK_TOP_UNIVPLL_D4: u32 = 87;
pub const CLK_TOP_UNIVPLL_D4_D2: u32 = 88;
pub const CLK_TOP_UNIVPLL_D4_D4: u32 = 89;
pub const CLK_TOP_UNIVPLL_D4_D8: u32 = 90;
pub const CLK_TOP_UNIVPLL_D5: u32 = 91;
pub const CLK_TOP_UNIVPLL_D5_D2: u32 = 92;
pub const CLK_TOP_UNIVPLL_D5_D4: u32 = 93;
pub const CLK_TOP_UNIVPLL_D5_D8: u32 = 94;
pub const CLK_TOP_UNIVPLL_D6: u32 = 95;
pub const CLK_TOP_UNIVPLL_D6_D2: u32 = 96;
pub const CLK_TOP_UNIVPLL_D6_D4: u32 = 97;
pub const CLK_TOP_UNIVPLL_D6_D8: u32 = 98;
pub const CLK_TOP_UNIVPLL_D6_D16: u32 = 99;
pub const CLK_TOP_UNIVPLL_D7: u32 = 100;
pub const CLK_TOP_APLL1: u32 = 101;
pub const CLK_TOP_APLL1_D2: u32 = 102;
pub const CLK_TOP_APLL1_D4: u32 = 103;
pub const CLK_TOP_APLL1_D8: u32 = 104;
pub const CLK_TOP_APLL2: u32 = 105;
pub const CLK_TOP_APLL2_D2: u32 = 106;
pub const CLK_TOP_APLL2_D4: u32 = 107;
pub const CLK_TOP_APLL2_D8: u32 = 108;
pub const CLK_TOP_MMPLL_D4: u32 = 109;
pub const CLK_TOP_MMPLL_D4_D2: u32 = 110;
pub const CLK_TOP_MMPLL_D5: u32 = 111;
pub const CLK_TOP_MMPLL_D5_D2: u32 = 112;
pub const CLK_TOP_MMPLL_D6: u32 = 113;
pub const CLK_TOP_MMPLL_D6_D2: u32 = 114;
pub const CLK_TOP_MMPLL_D7: u32 = 115;
pub const CLK_TOP_MMPLL_D9: u32 = 116;
pub const CLK_TOP_APUPLL: u32 = 117;
pub const CLK_TOP_NPUPLL: u32 = 118;
pub const CLK_TOP_TVDPLL: u32 = 119;
pub const CLK_TOP_TVDPLL_D2: u32 = 120;
pub const CLK_TOP_TVDPLL_D4: u32 = 121;
pub const CLK_TOP_TVDPLL_D8: u32 = 122;
pub const CLK_TOP_TVDPLL_D16: u32 = 123;
pub const CLK_TOP_MSDCPLL: u32 = 124;
pub const CLK_TOP_MSDCPLL_D2: u32 = 125;
pub const CLK_TOP_MSDCPLL_D4: u32 = 126;
pub const CLK_TOP_ULPOSC: u32 = 127;
pub const CLK_TOP_OSC_D2: u32 = 128;
pub const CLK_TOP_OSC_D4: u32 = 129;
pub const CLK_TOP_OSC_D8: u32 = 130;
pub const CLK_TOP_OSC_D10: u32 = 131;
pub const CLK_TOP_OSC_D16: u32 = 132;
pub const CLK_TOP_OSC_D20: u32 = 133;
pub const CLK_TOP_CSW_F26M_D2: u32 = 134;
pub const CLK_TOP_ADSPPLL: u32 = 135;
pub const CLK_TOP_UNIVPLL_192M: u32 = 136;
pub const CLK_TOP_UNIVPLL_192M_D2: u32 = 137;
pub const CLK_TOP_UNIVPLL_192M_D4: u32 = 138;
pub const CLK_TOP_UNIVPLL_192M_D8: u32 = 139;
pub const CLK_TOP_UNIVPLL_192M_D16: u32 = 140;
pub const CLK_TOP_UNIVPLL_192M_D32: u32 = 141;
pub const CLK_TOP_APLL12_DIV0: u32 = 142;
pub const CLK_TOP_APLL12_DIV1: u32 = 143;
pub const CLK_TOP_APLL12_DIV2: u32 = 144;
pub const CLK_TOP_APLL12_DIV3: u32 = 145;
pub const CLK_TOP_APLL12_DIV4: u32 = 146;
pub const CLK_TOP_APLL12_DIVB: u32 = 147;
pub const CLK_TOP_APLL12_DIV5: u32 = 148;
pub const CLK_TOP_APLL12_DIV6: u32 = 149;
pub const CLK_TOP_APLL12_DIV7: u32 = 150;
pub const CLK_TOP_APLL12_DIV8: u32 = 151;
pub const CLK_TOP_APLL12_DIV9: u32 = 152;
pub const CLK_TOP_SSUSB_TOP_REF: u32 = 153;
pub const CLK_TOP_SSUSB_PHY_REF: u32 = 154;
pub const CLK_TOP_NR_CLK: u32 = 155;

/* INFRACFG */

pub const CLK_INFRA_PMIC_TMR: u32 = 0;
pub const CLK_INFRA_PMIC_AP: u32 = 1;
pub const CLK_INFRA_PMIC_MD: u32 = 2;
pub const CLK_INFRA_PMIC_CONN: u32 = 3;
pub const CLK_INFRA_SCPSYS: u32 = 4;
pub const CLK_INFRA_SEJ: u32 = 5;
pub const CLK_INFRA_APXGPT: u32 = 6;
pub const CLK_INFRA_GCE: u32 = 7;
pub const CLK_INFRA_GCE2: u32 = 8;
pub const CLK_INFRA_THERM: u32 = 9;
pub const CLK_INFRA_I2C0: u32 = 10;
pub const CLK_INFRA_AP_DMA_PSEUDO: u32 = 11;
pub const CLK_INFRA_I2C2: u32 = 12;
pub const CLK_INFRA_I2C3: u32 = 13;
pub const CLK_INFRA_PWM_H: u32 = 14;
pub const CLK_INFRA_PWM1: u32 = 15;
pub const CLK_INFRA_PWM2: u32 = 16;
pub const CLK_INFRA_PWM3: u32 = 17;
pub const CLK_INFRA_PWM4: u32 = 18;
pub const CLK_INFRA_PWM: u32 = 19;
pub const CLK_INFRA_UART0: u32 = 20;
pub const CLK_INFRA_UART1: u32 = 21;
pub const CLK_INFRA_UART2: u32 = 22;
pub const CLK_INFRA_UART3: u32 = 23;
pub const CLK_INFRA_GCE_26M: u32 = 24;
pub const CLK_INFRA_CQ_DMA_FPC: u32 = 25;
pub const CLK_INFRA_BTIF: u32 = 26;
pub const CLK_INFRA_SPI0: u32 = 27;
pub const CLK_INFRA_MSDC0: u32 = 28;
pub const CLK_INFRA_MSDC1: u32 = 29;
pub const CLK_INFRA_MSDC2: u32 = 30;
pub const CLK_INFRA_MSDC0_SRC: u32 = 31;
pub const CLK_INFRA_GCPU: u32 = 32;
pub const CLK_INFRA_TRNG: u32 = 33;
pub const CLK_INFRA_AUXADC: u32 = 34;
pub const CLK_INFRA_CPUM: u32 = 35;
pub const CLK_INFRA_CCIF1_AP: u32 = 36;
pub const CLK_INFRA_CCIF1_MD: u32 = 37;
pub const CLK_INFRA_AUXADC_MD: u32 = 38;
pub const CLK_INFRA_PCIE_TL_26M: u32 = 39;
pub const CLK_INFRA_MSDC1_SRC: u32 = 40;
pub const CLK_INFRA_MSDC2_SRC: u32 = 41;
pub const CLK_INFRA_PCIE_TL_96M: u32 = 42;
pub const CLK_INFRA_PCIE_PL_P_250M: u32 = 43;
pub const CLK_INFRA_DEVICE_APC: u32 = 44;
pub const CLK_INFRA_CCIF_AP: u32 = 45;
pub const CLK_INFRA_DEBUGSYS: u32 = 46;
pub const CLK_INFRA_AUDIO: u32 = 47;
pub const CLK_INFRA_CCIF_MD: u32 = 48;
pub const CLK_INFRA_DXCC_SEC_CORE: u32 = 49;
pub const CLK_INFRA_DXCC_AO: u32 = 50;
pub const CLK_INFRA_DBG_TRACE: u32 = 51;
pub const CLK_INFRA_DEVMPU_B: u32 = 52;
pub const CLK_INFRA_DRAMC_F26M: u32 = 53;
pub const CLK_INFRA_IRTX: u32 = 54;
pub const CLK_INFRA_SSUSB: u32 = 55;
pub const CLK_INFRA_DISP_PWM: u32 = 56;
pub const CLK_INFRA_CLDMA_B: u32 = 57;
pub const CLK_INFRA_AUDIO_26M_B: u32 = 58;
pub const CLK_INFRA_MODEM_TEMP_SHARE: u32 = 59;
pub const CLK_INFRA_SPI1: u32 = 60;
pub const CLK_INFRA_I2C4: u32 = 61;
pub const CLK_INFRA_SPI2: u32 = 62;
pub const CLK_INFRA_SPI3: u32 = 63;
pub const CLK_INFRA_UNIPRO_SYS: u32 = 64;
pub const CLK_INFRA_UNIPRO_TICK: u32 = 65;
pub const CLK_INFRA_UFS_MP_SAP_B: u32 = 66;
pub const CLK_INFRA_MD32_B: u32 = 67;
pub const CLK_INFRA_UNIPRO_MBIST: u32 = 68;
pub const CLK_INFRA_I2C5: u32 = 69;
pub const CLK_INFRA_I2C5_ARBITER: u32 = 70;
pub const CLK_INFRA_I2C5_IMM: u32 = 71;
pub const CLK_INFRA_I2C1_ARBITER: u32 = 72;
pub const CLK_INFRA_I2C1_IMM: u32 = 73;
pub const CLK_INFRA_I2C2_ARBITER: u32 = 74;
pub const CLK_INFRA_I2C2_IMM: u32 = 75;
pub const CLK_INFRA_SPI4: u32 = 76;
pub const CLK_INFRA_SPI5: u32 = 77;
pub const CLK_INFRA_CQ_DMA: u32 = 78;
pub const CLK_INFRA_UFS: u32 = 79;
pub const CLK_INFRA_AES_UFSFDE: u32 = 80;
pub const CLK_INFRA_UFS_TICK: u32 = 81;
pub const CLK_INFRA_SSUSB_XHCI: u32 = 82;
pub const CLK_INFRA_MSDC0_SELF: u32 = 83;
pub const CLK_INFRA_MSDC1_SELF: u32 = 84;
pub const CLK_INFRA_MSDC2_SELF: u32 = 85;
pub const CLK_INFRA_UFS_AXI: u32 = 86;
pub const CLK_INFRA_I2C6: u32 = 87;
pub const CLK_INFRA_AP_MSDC0: u32 = 88;
pub const CLK_INFRA_MD_MSDC0: u32 = 89;
pub const CLK_INFRA_CCIF5_AP: u32 = 90;
pub const CLK_INFRA_CCIF5_MD: u32 = 91;
pub const CLK_INFRA_PCIE_TOP_H_133M: u32 = 92;
pub const CLK_INFRA_FLASHIF_TOP_H_133M: u32 = 93;
pub const CLK_INFRA_PCIE_PERI_26M: u32 = 94;
pub const CLK_INFRA_CCIF2_AP: u32 = 95;
pub const CLK_INFRA_CCIF2_MD: u32 = 96;
pub const CLK_INFRA_CCIF3_AP: u32 = 97;
pub const CLK_INFRA_CCIF3_MD: u32 = 98;
pub const CLK_INFRA_SEJ_F13M: u32 = 99;
pub const CLK_INFRA_AES: u32 = 100;
pub const CLK_INFRA_I2C7: u32 = 101;
pub const CLK_INFRA_I2C8: u32 = 102;
pub const CLK_INFRA_FBIST2FPC: u32 = 103;
pub const CLK_INFRA_DEVICE_APC_SYNC: u32 = 104;
pub const CLK_INFRA_DPMAIF_MAIN: u32 = 105;
pub const CLK_INFRA_PCIE_TL_32K: u32 = 106;
pub const CLK_INFRA_CCIF4_AP: u32 = 107;
pub const CLK_INFRA_CCIF4_MD: u32 = 108;
pub const CLK_INFRA_SPI6: u32 = 109;
pub const CLK_INFRA_SPI7: u32 = 110;
pub const CLK_INFRA_133M: u32 = 111;
pub const CLK_INFRA_66M: u32 = 112;
pub const CLK_INFRA_66M_PERI_BUS: u32 = 113;
pub const CLK_INFRA_FREE_DCM_133M: u32 = 114;
pub const CLK_INFRA_FREE_DCM_66M: u32 = 115;
pub const CLK_INFRA_PERI_BUS_DCM_133M: u32 = 116;
pub const CLK_INFRA_PERI_BUS_DCM_66M: u32 = 117;
pub const CLK_INFRA_FLASHIF_PERI_26M: u32 = 118;
pub const CLK_INFRA_FLASHIF_SFLASH: u32 = 119;
pub const CLK_INFRA_AP_DMA: u32 = 120;
pub const CLK_INFRA_NR_CLK: u32 = 121;

/* PERICFG */

pub const CLK_PERI_PERIAXI: u32 = 0;
pub const CLK_PERI_NR_CLK: u32 = 1;

/* APMIXEDSYS */

pub const CLK_APMIXED_MAINPLL: u32 = 0;
pub const CLK_APMIXED_UNIVPLL: u32 = 1;
pub const CLK_APMIXED_USBPLL: u32 = 2;
pub const CLK_APMIXED_MSDCPLL: u32 = 3;
pub const CLK_APMIXED_MMPLL: u32 = 4;
pub const CLK_APMIXED_ADSPPLL: u32 = 5;
pub const CLK_APMIXED_MFGPLL: u32 = 6;
pub const CLK_APMIXED_TVDPLL: u32 = 7;
pub const CLK_APMIXED_APLL1: u32 = 8;
pub const CLK_APMIXED_APLL2: u32 = 9;
pub const CLK_APMIXED_MIPID26M: u32 = 10;
pub const CLK_APMIXED_NR_CLK: u32 = 11;

/* SCP_ADSP */

pub const CLK_SCP_ADSP_AUDIODSP: u32 = 0;
pub const CLK_SCP_ADSP_NR_CLK: u32 = 1;

/* IMP_IIC_WRAP_C */

pub const CLK_IMP_IIC_WRAP_C_I2C10: u32 = 0;
pub const CLK_IMP_IIC_WRAP_C_I2C11: u32 = 1;
pub const CLK_IMP_IIC_WRAP_C_I2C12: u32 = 2;
pub const CLK_IMP_IIC_WRAP_C_I2C13: u32 = 3;
pub const CLK_IMP_IIC_WRAP_C_NR_CLK: u32 = 4;

/* AUDSYS */

pub const CLK_AUD_AFE: u32 = 0;
pub const CLK_AUD_22M: u32 = 1;
pub const CLK_AUD_24M: u32 = 2;
pub const CLK_AUD_APLL2_TUNER: u32 = 3;
pub const CLK_AUD_APLL_TUNER: u32 = 4;
pub const CLK_AUD_TDM: u32 = 5;
pub const CLK_AUD_ADC: u32 = 6;
pub const CLK_AUD_DAC: u32 = 7;
pub const CLK_AUD_DAC_PREDIS: u32 = 8;
pub const CLK_AUD_TML: u32 = 9;
pub const CLK_AUD_NLE: u32 = 10;
pub const CLK_AUD_I2S1_B: u32 = 11;
pub const CLK_AUD_I2S2_B: u32 = 12;
pub const CLK_AUD_I2S3_B: u32 = 13;
pub const CLK_AUD_I2S4_B: u32 = 14;
pub const CLK_AUD_CONNSYS_I2S_ASRC: u32 = 15;
pub const CLK_AUD_GENERAL1_ASRC: u32 = 16;
pub const CLK_AUD_GENERAL2_ASRC: u32 = 17;
pub const CLK_AUD_DAC_HIRES: u32 = 18;
pub const CLK_AUD_ADC_HIRES: u32 = 19;
pub const CLK_AUD_ADC_HIRES_TML: u32 = 20;
pub const CLK_AUD_ADDA6_ADC: u32 = 21;
pub const CLK_AUD_ADDA6_ADC_HIRES: u32 = 22;
pub const CLK_AUD_3RD_DAC: u32 = 23;
pub const CLK_AUD_3RD_DAC_PREDIS: u32 = 24;
pub const CLK_AUD_3RD_DAC_TML: u32 = 25;
pub const CLK_AUD_3RD_DAC_HIRES: u32 = 26;
pub const CLK_AUD_I2S5_B: u32 = 27;
pub const CLK_AUD_I2S6_B: u32 = 28;
pub const CLK_AUD_I2S7_B: u32 = 29;
pub const CLK_AUD_I2S8_B: u32 = 30;
pub const CLK_AUD_I2S9_B: u32 = 31;
pub const CLK_AUD_NR_CLK: u32 = 32;

/* IMP_IIC_WRAP_E */

pub const CLK_IMP_IIC_WRAP_E_I2C3: u32 = 0;
pub const CLK_IMP_IIC_WRAP_E_NR_CLK: u32 = 1;

/* IMP_IIC_WRAP_S */

pub const CLK_IMP_IIC_WRAP_S_I2C7: u32 = 0;
pub const CLK_IMP_IIC_WRAP_S_I2C8: u32 = 1;
pub const CLK_IMP_IIC_WRAP_S_I2C9: u32 = 2;
pub const CLK_IMP_IIC_WRAP_S_NR_CLK: u32 = 3;

/* IMP_IIC_WRAP_WS */

pub const CLK_IMP_IIC_WRAP_WS_I2C1: u32 = 0;
pub const CLK_IMP_IIC_WRAP_WS_I2C2: u32 = 1;
pub const CLK_IMP_IIC_WRAP_WS_I2C4: u32 = 2;
pub const CLK_IMP_IIC_WRAP_WS_NR_CLK: u32 = 3;

/* IMP_IIC_WRAP_W */

pub const CLK_IMP_IIC_WRAP_W_I2C5: u32 = 0;
pub const CLK_IMP_IIC_WRAP_W_NR_CLK: u32 = 1;

/* IMP_IIC_WRAP_N */

pub const CLK_IMP_IIC_WRAP_N_I2C0: u32 = 0;
pub const CLK_IMP_IIC_WRAP_N_I2C6: u32 = 1;
pub const CLK_IMP_IIC_WRAP_N_NR_CLK: u32 = 2;

/* MSDC_TOP */

pub const CLK_MSDC_TOP_AES_0P: u32 = 0;
pub const CLK_MSDC_TOP_SRC_0P: u32 = 1;
pub const CLK_MSDC_TOP_SRC_1P: u32 = 2;
pub const CLK_MSDC_TOP_SRC_2P: u32 = 3;
pub const CLK_MSDC_TOP_P_MSDC0: u32 = 4;
pub const CLK_MSDC_TOP_P_MSDC1: u32 = 5;
pub const CLK_MSDC_TOP_P_MSDC2: u32 = 6;
pub const CLK_MSDC_TOP_P_CFG: u32 = 7;
pub const CLK_MSDC_TOP_AXI: u32 = 8;
pub const CLK_MSDC_TOP_H_MST_0P: u32 = 9;
pub const CLK_MSDC_TOP_H_MST_1P: u32 = 10;
pub const CLK_MSDC_TOP_H_MST_2P: u32 = 11;
pub const CLK_MSDC_TOP_MEM_OFF_DLY_26M: u32 = 12;
pub const CLK_MSDC_TOP_32K: u32 = 13;
pub const CLK_MSDC_TOP_AHB2AXI_BRG_AXI: u32 = 14;
pub const CLK_MSDC_TOP_NR_CLK: u32 = 15;

/* MSDC */

pub const CLK_MSDC_AXI_WRAP: u32 = 0;
pub const CLK_MSDC_NR_CLK: u32 = 1;

/* MFGCFG */

pub const CLK_MFG_BG3D: u32 = 0;
pub const CLK_MFG_NR_CLK: u32 = 1;

/* MMSYS */

pub const CLK_MM_DISP_MUTEX0: u32 = 0;
pub const CLK_MM_DISP_CONFIG: u32 = 1;
pub const CLK_MM_DISP_OVL0: u32 = 2;
pub const CLK_MM_DISP_RDMA0: u32 = 3;
pub const CLK_MM_DISP_OVL0_2L: u32 = 4;
pub const CLK_MM_DISP_WDMA0: u32 = 5;
pub const CLK_MM_DISP_UFBC_WDMA0: u32 = 6;
pub const CLK_MM_DISP_RSZ0: u32 = 7;
pub const CLK_MM_DISP_AAL0: u32 = 8;
pub const CLK_MM_DISP_CCORR0: u32 = 9;
pub const CLK_MM_DISP_DITHER0: u32 = 10;
pub const CLK_MM_SMI_INFRA: u32 = 11;
pub const CLK_MM_DISP_GAMMA0: u32 = 12;
pub const CLK_MM_DISP_POSTMASK0: u32 = 13;
pub const CLK_MM_DISP_DSC_WRAP0: u32 = 14;
pub const CLK_MM_DSI0: u32 = 15;
pub const CLK_MM_DISP_COLOR0: u32 = 16;
pub const CLK_MM_SMI_COMMON: u32 = 17;
pub const CLK_MM_DISP_FAKE_ENG0: u32 = 18;
pub const CLK_MM_DISP_FAKE_ENG1: u32 = 19;
pub const CLK_MM_MDP_TDSHP4: u32 = 20;
pub const CLK_MM_MDP_RSZ4: u32 = 21;
pub const CLK_MM_MDP_AAL4: u32 = 22;
pub const CLK_MM_MDP_HDR4: u32 = 23;
pub const CLK_MM_MDP_RDMA4: u32 = 24;
pub const CLK_MM_MDP_COLOR4: u32 = 25;
pub const CLK_MM_DISP_Y2R0: u32 = 26;
pub const CLK_MM_SMI_GALS: u32 = 27;
pub const CLK_MM_DISP_OVL2_2L: u32 = 28;
pub const CLK_MM_DISP_RDMA4: u32 = 29;
pub const CLK_MM_DISP_DPI0: u32 = 30;
pub const CLK_MM_SMI_IOMMU: u32 = 31;
pub const CLK_MM_DSI_DSI0: u32 = 32;
pub const CLK_MM_DPI_DPI0: u32 = 33;
pub const CLK_MM_26MHZ: u32 = 34;
pub const CLK_MM_32KHZ: u32 = 35;
pub const CLK_MM_NR_CLK: u32 = 36;

/* IMGSYS */

pub const CLK_IMG_LARB9: u32 = 0;
pub const CLK_IMG_LARB10: u32 = 1;
pub const CLK_IMG_DIP: u32 = 2;
pub const CLK_IMG_GALS: u32 = 3;
pub const CLK_IMG_NR_CLK: u32 = 4;

/* IMGSYS2 */

pub const CLK_IMG2_LARB11: u32 = 0;
pub const CLK_IMG2_LARB12: u32 = 1;
pub const CLK_IMG2_MFB: u32 = 2;
pub const CLK_IMG2_WPE: u32 = 3;
pub const CLK_IMG2_MSS: u32 = 4;
pub const CLK_IMG2_GALS: u32 = 5;
pub const CLK_IMG2_NR_CLK: u32 = 6;

/* VDECSYS_SOC */

pub const CLK_VDEC_SOC_LARB1: u32 = 0;
pub const CLK_VDEC_SOC_LAT: u32 = 1;
pub const CLK_VDEC_SOC_LAT_ACTIVE: u32 = 2;
pub const CLK_VDEC_SOC_VDEC: u32 = 3;
pub const CLK_VDEC_SOC_VDEC_ACTIVE: u32 = 4;
pub const CLK_VDEC_SOC_NR_CLK: u32 = 5;

/* VDECSYS */

pub const CLK_VDEC_LARB1: u32 = 0;
pub const CLK_VDEC_LAT: u32 = 1;
pub const CLK_VDEC_LAT_ACTIVE: u32 = 2;
pub const CLK_VDEC_VDEC: u32 = 3;
pub const CLK_VDEC_ACTIVE: u32 = 4;
pub const CLK_VDEC_NR_CLK: u32 = 5;

/* VENCSYS */

pub const CLK_VENC_SET0_LARB: u32 = 0;
pub const CLK_VENC_SET1_VENC: u32 = 1;
pub const CLK_VENC_SET2_JPGENC: u32 = 2;
pub const CLK_VENC_SET5_GALS: u32 = 3;
pub const CLK_VENC_NR_CLK: u32 = 4;

/* CAMSYS */

pub const CLK_CAM_LARB13: u32 = 0;
pub const CLK_CAM_DFP_VAD: u32 = 1;
pub const CLK_CAM_LARB14: u32 = 2;
pub const CLK_CAM_CAM: u32 = 3;
pub const CLK_CAM_CAMTG: u32 = 4;
pub const CLK_CAM_SENINF: u32 = 5;
pub const CLK_CAM_CAMSV0: u32 = 6;
pub const CLK_CAM_CAMSV1: u32 = 7;
pub const CLK_CAM_CAMSV2: u32 = 8;
pub const CLK_CAM_CAMSV3: u32 = 9;
pub const CLK_CAM_CCU0: u32 = 10;
pub const CLK_CAM_CCU1: u32 = 11;
pub const CLK_CAM_MRAW0: u32 = 12;
pub const CLK_CAM_FAKE_ENG: u32 = 13;
pub const CLK_CAM_CCU_GALS: u32 = 14;
pub const CLK_CAM_CAM2MM_GALS: u32 = 15;
pub const CLK_CAM_NR_CLK: u32 = 16;

/* CAMSYS_RAWA */

pub const CLK_CAM_RAWA_LARBX: u32 = 0;
pub const CLK_CAM_RAWA_CAM: u32 = 1;
pub const CLK_CAM_RAWA_CAMTG: u32 = 2;
pub const CLK_CAM_RAWA_NR_CLK: u32 = 3;

/* CAMSYS_RAWB */

pub const CLK_CAM_RAWB_LARBX: u32 = 0;
pub const CLK_CAM_RAWB_CAM: u32 = 1;
pub const CLK_CAM_RAWB_CAMTG: u32 = 2;
pub const CLK_CAM_RAWB_NR_CLK: u32 = 3;

/* CAMSYS_RAWC */

pub const CLK_CAM_RAWC_LARBX: u32 = 0;
pub const CLK_CAM_RAWC_CAM: u32 = 1;
pub const CLK_CAM_RAWC_CAMTG: u32 = 2;
pub const CLK_CAM_RAWC_NR_CLK: u32 = 3;

/* IPESYS */

pub const CLK_IPE_LARB19: u32 = 0;
pub const CLK_IPE_LARB20: u32 = 1;
pub const CLK_IPE_SMI_SUBCOM: u32 = 2;
pub const CLK_IPE_FD: u32 = 3;
pub const CLK_IPE_FE: u32 = 4;
pub const CLK_IPE_RSC: u32 = 5;
pub const CLK_IPE_DPE: u32 = 6;
pub const CLK_IPE_GALS: u32 = 7;
pub const CLK_IPE_NR_CLK: u32 = 8;

/* MDPSYS */

pub const CLK_MDP_RDMA0: u32 = 0;
pub const CLK_MDP_TDSHP0: u32 = 1;
pub const CLK_MDP_IMG_DL_ASYNC0: u32 = 2;
pub const CLK_MDP_IMG_DL_ASYNC1: u32 = 3;
pub const CLK_MDP_RDMA1: u32 = 4;
pub const CLK_MDP_TDSHP1: u32 = 5;
pub const CLK_MDP_SMI0: u32 = 6;
pub const CLK_MDP_APB_BUS: u32 = 7;
pub const CLK_MDP_WROT0: u32 = 8;
pub const CLK_MDP_RSZ0: u32 = 9;
pub const CLK_MDP_HDR0: u32 = 10;
pub const CLK_MDP_MUTEX0: u32 = 11;
pub const CLK_MDP_WROT1: u32 = 12;
pub const CLK_MDP_RSZ1: u32 = 13;
pub const CLK_MDP_HDR1: u32 = 14;
pub const CLK_MDP_FAKE_ENG0: u32 = 15;
pub const CLK_MDP_AAL0: u32 = 16;
pub const CLK_MDP_AAL1: u32 = 17;
pub const CLK_MDP_COLOR0: u32 = 18;
pub const CLK_MDP_COLOR1: u32 = 19;
pub const CLK_MDP_IMG_DL_RELAY0_ASYNC0: u32 = 20;
pub const CLK_MDP_IMG_DL_RELAY1_ASYNC1: u32 = 21;
pub const CLK_MDP_NR_CLK: u32 = 22;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
