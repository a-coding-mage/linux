/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>
 */


/* TOPCKGEN */

pub const CLK_TOP_AXI: u32 = 0;
pub const CLK_TOP_SPM: u32 = 1;
pub const CLK_TOP_SCP: u32 = 2;
pub const CLK_TOP_BUS_AXIMEM: u32 = 3;
pub const CLK_TOP_VPP: u32 = 4;
pub const CLK_TOP_ETHDR: u32 = 5;
pub const CLK_TOP_IPE: u32 = 6;
pub const CLK_TOP_CAM: u32 = 7;
pub const CLK_TOP_CCU: u32 = 8;
pub const CLK_TOP_IMG: u32 = 9;
pub const CLK_TOP_CAMTM: u32 = 10;
pub const CLK_TOP_DSP: u32 = 11;
pub const CLK_TOP_DSP1: u32 = 12;
pub const CLK_TOP_DSP2: u32 = 13;
pub const CLK_TOP_DSP3: u32 = 14;
pub const CLK_TOP_DSP4: u32 = 15;
pub const CLK_TOP_DSP5: u32 = 16;
pub const CLK_TOP_DSP6: u32 = 17;
pub const CLK_TOP_DSP7: u32 = 18;
pub const CLK_TOP_IPU_IF: u32 = 19;
pub const CLK_TOP_MFG_CORE_TMP: u32 = 20;
pub const CLK_TOP_CAMTG: u32 = 21;
pub const CLK_TOP_CAMTG2: u32 = 22;
pub const CLK_TOP_CAMTG3: u32 = 23;
pub const CLK_TOP_CAMTG4: u32 = 24;
pub const CLK_TOP_CAMTG5: u32 = 25;
pub const CLK_TOP_UART: u32 = 26;
pub const CLK_TOP_SPI: u32 = 27;
pub const CLK_TOP_SPIS: u32 = 28;
pub const CLK_TOP_MSDC50_0_HCLK: u32 = 29;
pub const CLK_TOP_MSDC50_0: u32 = 30;
pub const CLK_TOP_MSDC30_1: u32 = 31;
pub const CLK_TOP_MSDC30_2: u32 = 32;
pub const CLK_TOP_INTDIR: u32 = 33;
pub const CLK_TOP_AUD_INTBUS: u32 = 34;
pub const CLK_TOP_AUDIO_H: u32 = 35;
pub const CLK_TOP_PWRAP_ULPOSC: u32 = 36;
pub const CLK_TOP_ATB: u32 = 37;
pub const CLK_TOP_PWRMCU: u32 = 38;
pub const CLK_TOP_DP: u32 = 39;
pub const CLK_TOP_EDP: u32 = 40;
pub const CLK_TOP_DPI: u32 = 41;
pub const CLK_TOP_DISP_PWM0: u32 = 42;
pub const CLK_TOP_DISP_PWM1: u32 = 43;
pub const CLK_TOP_USB_TOP: u32 = 44;
pub const CLK_TOP_SSUSB_XHCI: u32 = 45;
pub const CLK_TOP_USB_TOP_1P: u32 = 46;
pub const CLK_TOP_SSUSB_XHCI_1P: u32 = 47;
pub const CLK_TOP_USB_TOP_2P: u32 = 48;
pub const CLK_TOP_SSUSB_XHCI_2P: u32 = 49;
pub const CLK_TOP_USB_TOP_3P: u32 = 50;
pub const CLK_TOP_SSUSB_XHCI_3P: u32 = 51;
pub const CLK_TOP_I2C: u32 = 52;
pub const CLK_TOP_SENINF: u32 = 53;
pub const CLK_TOP_SENINF1: u32 = 54;
pub const CLK_TOP_SENINF2: u32 = 55;
pub const CLK_TOP_SENINF3: u32 = 56;
pub const CLK_TOP_GCPU: u32 = 57;
pub const CLK_TOP_DXCC: u32 = 58;
pub const CLK_TOP_DPMAIF_MAIN: u32 = 59;
pub const CLK_TOP_AES_UFSFDE: u32 = 60;
pub const CLK_TOP_UFS: u32 = 61;
pub const CLK_TOP_UFS_TICK1US: u32 = 62;
pub const CLK_TOP_UFS_MP_SAP_CFG: u32 = 63;
pub const CLK_TOP_VENC: u32 = 64;
pub const CLK_TOP_VDEC: u32 = 65;
pub const CLK_TOP_PWM: u32 = 66;
pub const CLK_TOP_MCUPM: u32 = 67;
pub const CLK_TOP_SPMI_P_MST: u32 = 68;
pub const CLK_TOP_SPMI_M_MST: u32 = 69;
pub const CLK_TOP_DVFSRC: u32 = 70;
pub const CLK_TOP_TL: u32 = 71;
pub const CLK_TOP_TL_P1: u32 = 72;
pub const CLK_TOP_AES_MSDCFDE: u32 = 73;
pub const CLK_TOP_DSI_OCC: u32 = 74;
pub const CLK_TOP_WPE_VPP: u32 = 75;
pub const CLK_TOP_HDCP: u32 = 76;
pub const CLK_TOP_HDCP_24M: u32 = 77;
pub const CLK_TOP_HD20_DACR_REF_CLK: u32 = 78;
pub const CLK_TOP_HD20_HDCP_CCLK: u32 = 79;
pub const CLK_TOP_HDMI_XTAL: u32 = 80;
pub const CLK_TOP_HDMI_APB: u32 = 81;
pub const CLK_TOP_SNPS_ETH_250M: u32 = 82;
pub const CLK_TOP_SNPS_ETH_62P4M_PTP: u32 = 83;
pub const CLK_TOP_SNPS_ETH_50M_RMII: u32 = 84;
pub const CLK_TOP_DGI_OUT: u32 = 85;
pub const CLK_TOP_NNA0: u32 = 86;
pub const CLK_TOP_NNA1: u32 = 87;
pub const CLK_TOP_ADSP: u32 = 88;
pub const CLK_TOP_ASM_H: u32 = 89;
pub const CLK_TOP_ASM_M: u32 = 90;
pub const CLK_TOP_ASM_L: u32 = 91;
pub const CLK_TOP_APLL1: u32 = 92;
pub const CLK_TOP_APLL2: u32 = 93;
pub const CLK_TOP_APLL3: u32 = 94;
pub const CLK_TOP_APLL4: u32 = 95;
pub const CLK_TOP_APLL5: u32 = 96;
pub const CLK_TOP_I2SO1_MCK: u32 = 97;
pub const CLK_TOP_I2SO2_MCK: u32 = 98;
pub const CLK_TOP_I2SI1_MCK: u32 = 99;
pub const CLK_TOP_I2SI2_MCK: u32 = 100;
pub const CLK_TOP_DPTX_MCK: u32 = 101;
pub const CLK_TOP_AUD_IEC_CLK: u32 = 102;
pub const CLK_TOP_A1SYS_HP: u32 = 103;
pub const CLK_TOP_A2SYS_HF: u32 = 104;
pub const CLK_TOP_A3SYS_HF: u32 = 105;
pub const CLK_TOP_A4SYS_HF: u32 = 106;
pub const CLK_TOP_SPINFI_BCLK: u32 = 107;
pub const CLK_TOP_NFI1X: u32 = 108;
pub const CLK_TOP_ECC: u32 = 109;
pub const CLK_TOP_AUDIO_LOCAL_BUS: u32 = 110;
pub const CLK_TOP_SPINOR: u32 = 111;
pub const CLK_TOP_DVIO_DGI_REF: u32 = 112;
pub const CLK_TOP_ULPOSC: u32 = 113;
pub const CLK_TOP_ULPOSC_CORE: u32 = 114;
pub const CLK_TOP_SRCK: u32 = 115;
pub const CLK_TOP_MFG_CK_FAST_REF: u32 = 116;
pub const CLK_TOP_CLK26M_D2: u32 = 117;
pub const CLK_TOP_CLK26M_D52: u32 = 118;
pub const CLK_TOP_IN_DGI: u32 = 119;
pub const CLK_TOP_IN_DGI_D2: u32 = 120;
pub const CLK_TOP_IN_DGI_D4: u32 = 121;
pub const CLK_TOP_IN_DGI_D6: u32 = 122;
pub const CLK_TOP_IN_DGI_D8: u32 = 123;
pub const CLK_TOP_MAINPLL_D3: u32 = 124;
pub const CLK_TOP_MAINPLL_D4: u32 = 125;
pub const CLK_TOP_MAINPLL_D4_D2: u32 = 126;
pub const CLK_TOP_MAINPLL_D4_D4: u32 = 127;
pub const CLK_TOP_MAINPLL_D4_D8: u32 = 128;
pub const CLK_TOP_MAINPLL_D5: u32 = 129;
pub const CLK_TOP_MAINPLL_D5_D2: u32 = 130;
pub const CLK_TOP_MAINPLL_D5_D4: u32 = 131;
pub const CLK_TOP_MAINPLL_D5_D8: u32 = 132;
pub const CLK_TOP_MAINPLL_D6: u32 = 133;
pub const CLK_TOP_MAINPLL_D6_D2: u32 = 134;
pub const CLK_TOP_MAINPLL_D6_D4: u32 = 135;
pub const CLK_TOP_MAINPLL_D6_D8: u32 = 136;
pub const CLK_TOP_MAINPLL_D7: u32 = 137;
pub const CLK_TOP_MAINPLL_D7_D2: u32 = 138;
pub const CLK_TOP_MAINPLL_D7_D4: u32 = 139;
pub const CLK_TOP_MAINPLL_D7_D8: u32 = 140;
pub const CLK_TOP_MAINPLL_D9: u32 = 141;
pub const CLK_TOP_UNIVPLL_D2: u32 = 142;
pub const CLK_TOP_UNIVPLL_D3: u32 = 143;
pub const CLK_TOP_UNIVPLL_D4: u32 = 144;
pub const CLK_TOP_UNIVPLL_D4_D2: u32 = 145;
pub const CLK_TOP_UNIVPLL_D4_D4: u32 = 146;
pub const CLK_TOP_UNIVPLL_D4_D8: u32 = 147;
pub const CLK_TOP_UNIVPLL_D5: u32 = 148;
pub const CLK_TOP_UNIVPLL_D5_D2: u32 = 149;
pub const CLK_TOP_UNIVPLL_D5_D4: u32 = 150;
pub const CLK_TOP_UNIVPLL_D5_D8: u32 = 151;
pub const CLK_TOP_UNIVPLL_D6: u32 = 152;
pub const CLK_TOP_UNIVPLL_D6_D2: u32 = 153;
pub const CLK_TOP_UNIVPLL_D6_D4: u32 = 154;
pub const CLK_TOP_UNIVPLL_D6_D8: u32 = 155;
pub const CLK_TOP_UNIVPLL_D6_D16: u32 = 156;
pub const CLK_TOP_UNIVPLL_D7: u32 = 157;
pub const CLK_TOP_UNIVPLL_192M: u32 = 158;
pub const CLK_TOP_UNIVPLL_192M_D4: u32 = 159;
pub const CLK_TOP_UNIVPLL_192M_D8: u32 = 160;
pub const CLK_TOP_UNIVPLL_192M_D16: u32 = 161;
pub const CLK_TOP_UNIVPLL_192M_D32: u32 = 162;
pub const CLK_TOP_APLL1_D3: u32 = 163;
pub const CLK_TOP_APLL1_D4: u32 = 164;
pub const CLK_TOP_APLL2_D3: u32 = 165;
pub const CLK_TOP_APLL2_D4: u32 = 166;
pub const CLK_TOP_APLL3_D4: u32 = 167;
pub const CLK_TOP_APLL4_D4: u32 = 168;
pub const CLK_TOP_APLL5_D4: u32 = 169;
pub const CLK_TOP_HDMIRX_APLL_D3: u32 = 170;
pub const CLK_TOP_HDMIRX_APLL_D4: u32 = 171;
pub const CLK_TOP_HDMIRX_APLL_D6: u32 = 172;
pub const CLK_TOP_MMPLL_D4: u32 = 173;
pub const CLK_TOP_MMPLL_D4_D2: u32 = 174;
pub const CLK_TOP_MMPLL_D4_D4: u32 = 175;
pub const CLK_TOP_MMPLL_D5: u32 = 176;
pub const CLK_TOP_MMPLL_D5_D2: u32 = 177;
pub const CLK_TOP_MMPLL_D5_D4: u32 = 178;
pub const CLK_TOP_MMPLL_D6: u32 = 179;
pub const CLK_TOP_MMPLL_D6_D2: u32 = 180;
pub const CLK_TOP_MMPLL_D7: u32 = 181;
pub const CLK_TOP_MMPLL_D9: u32 = 182;
pub const CLK_TOP_TVDPLL1_D2: u32 = 183;
pub const CLK_TOP_TVDPLL1_D4: u32 = 184;
pub const CLK_TOP_TVDPLL1_D8: u32 = 185;
pub const CLK_TOP_TVDPLL1_D16: u32 = 186;
pub const CLK_TOP_TVDPLL2_D2: u32 = 187;
pub const CLK_TOP_TVDPLL2_D4: u32 = 188;
pub const CLK_TOP_TVDPLL2_D8: u32 = 189;
pub const CLK_TOP_TVDPLL2_D16: u32 = 190;
pub const CLK_TOP_MSDCPLL_D2: u32 = 191;
pub const CLK_TOP_MSDCPLL_D4: u32 = 192;
pub const CLK_TOP_MSDCPLL_D16: u32 = 193;
pub const CLK_TOP_ETHPLL_D2: u32 = 194;
pub const CLK_TOP_ETHPLL_D8: u32 = 195;
pub const CLK_TOP_ETHPLL_D10: u32 = 196;
pub const CLK_TOP_DGIPLL_D2: u32 = 197;
pub const CLK_TOP_ULPOSC1: u32 = 198;
pub const CLK_TOP_ULPOSC1_D2: u32 = 199;
pub const CLK_TOP_ULPOSC1_D4: u32 = 200;
pub const CLK_TOP_ULPOSC1_D7: u32 = 201;
pub const CLK_TOP_ULPOSC1_D8: u32 = 202;
pub const CLK_TOP_ULPOSC1_D10: u32 = 203;
pub const CLK_TOP_ULPOSC1_D16: u32 = 204;
pub const CLK_TOP_ULPOSC2: u32 = 205;
pub const CLK_TOP_ADSPPLL_D2: u32 = 206;
pub const CLK_TOP_ADSPPLL_D4: u32 = 207;
pub const CLK_TOP_ADSPPLL_D8: u32 = 208;
pub const CLK_TOP_MEM_466M: u32 = 209;
pub const CLK_TOP_MPHONE_SLAVE_B: u32 = 210;
pub const CLK_TOP_PEXTP_PIPE: u32 = 211;
pub const CLK_TOP_UFS_RX_SYMBOL: u32 = 212;
pub const CLK_TOP_UFS_TX_SYMBOL: u32 = 213;
pub const CLK_TOP_SSUSB_U3PHY_P1_P_P0: u32 = 214;
pub const CLK_TOP_UFS_RX_SYMBOL1: u32 = 215;
pub const CLK_TOP_FPC: u32 = 216;
pub const CLK_TOP_HDMIRX_P: u32 = 217;
pub const CLK_TOP_APLL12_DIV0: u32 = 218;
pub const CLK_TOP_APLL12_DIV1: u32 = 219;
pub const CLK_TOP_APLL12_DIV2: u32 = 220;
pub const CLK_TOP_APLL12_DIV3: u32 = 221;
pub const CLK_TOP_APLL12_DIV4: u32 = 222;
pub const CLK_TOP_APLL12_DIV9: u32 = 223;
pub const CLK_TOP_CFG_VPP0: u32 = 224;
pub const CLK_TOP_CFG_VPP1: u32 = 225;
pub const CLK_TOP_CFG_VDO0: u32 = 226;
pub const CLK_TOP_CFG_VDO1: u32 = 227;
pub const CLK_TOP_CFG_UNIPLL_SES: u32 = 228;
pub const CLK_TOP_CFG_26M_VPP0: u32 = 229;
pub const CLK_TOP_CFG_26M_VPP1: u32 = 230;
pub const CLK_TOP_CFG_26M_AUD: u32 = 231;
pub const CLK_TOP_CFG_AXI_EAST: u32 = 232;
pub const CLK_TOP_CFG_AXI_EAST_NORTH: u32 = 233;
pub const CLK_TOP_CFG_AXI_NORTH: u32 = 234;
pub const CLK_TOP_CFG_AXI_SOUTH: u32 = 235;
pub const CLK_TOP_CFG_EXT_TEST: u32 = 236;
pub const CLK_TOP_SSUSB_REF: u32 = 237;
pub const CLK_TOP_SSUSB_PHY_REF: u32 = 238;
pub const CLK_TOP_SSUSB_P1_REF: u32 = 239;
pub const CLK_TOP_SSUSB_PHY_P1_REF: u32 = 240;
pub const CLK_TOP_SSUSB_P2_REF: u32 = 241;
pub const CLK_TOP_SSUSB_PHY_P2_REF: u32 = 242;
pub const CLK_TOP_SSUSB_P3_REF: u32 = 243;
pub const CLK_TOP_SSUSB_PHY_P3_REF: u32 = 244;
pub const CLK_TOP_NR_CLK: u32 = 245;

/* INFRACFG_AO */

pub const CLK_INFRA_AO_PMIC_TMR: u32 = 0;
pub const CLK_INFRA_AO_PMIC_AP: u32 = 1;
pub const CLK_INFRA_AO_PMIC_MD: u32 = 2;
pub const CLK_INFRA_AO_PMIC_CONN: u32 = 3;
pub const CLK_INFRA_AO_SEJ: u32 = 4;
pub const CLK_INFRA_AO_APXGPT: u32 = 5;
pub const CLK_INFRA_AO_GCE: u32 = 6;
pub const CLK_INFRA_AO_GCE2: u32 = 7;
pub const CLK_INFRA_AO_THERM: u32 = 8;
pub const CLK_INFRA_AO_PWM_H: u32 = 9;
pub const CLK_INFRA_AO_PWM1: u32 = 10;
pub const CLK_INFRA_AO_PWM2: u32 = 11;
pub const CLK_INFRA_AO_PWM3: u32 = 12;
pub const CLK_INFRA_AO_PWM4: u32 = 13;
pub const CLK_INFRA_AO_PWM: u32 = 14;
pub const CLK_INFRA_AO_UART0: u32 = 15;
pub const CLK_INFRA_AO_UART1: u32 = 16;
pub const CLK_INFRA_AO_UART2: u32 = 17;
pub const CLK_INFRA_AO_UART3: u32 = 18;
pub const CLK_INFRA_AO_UART4: u32 = 19;
pub const CLK_INFRA_AO_GCE_26M: u32 = 20;
pub const CLK_INFRA_AO_CQ_DMA_FPC: u32 = 21;
pub const CLK_INFRA_AO_UART5: u32 = 22;
pub const CLK_INFRA_AO_HDMI_26M: u32 = 23;
pub const CLK_INFRA_AO_SPI0: u32 = 24;
pub const CLK_INFRA_AO_MSDC0: u32 = 25;
pub const CLK_INFRA_AO_MSDC1: u32 = 26;
pub const CLK_INFRA_AO_CG1_MSDC2: u32 = 27;
pub const CLK_INFRA_AO_MSDC0_SRC: u32 = 28;
pub const CLK_INFRA_AO_TRNG: u32 = 29;
pub const CLK_INFRA_AO_AUXADC: u32 = 30;
pub const CLK_INFRA_AO_CPUM: u32 = 31;
pub const CLK_INFRA_AO_HDMI_32K: u32 = 32;
pub const CLK_INFRA_AO_CEC_66M_H: u32 = 33;
pub const CLK_INFRA_AO_IRRX: u32 = 34;
pub const CLK_INFRA_AO_PCIE_TL_26M: u32 = 35;
pub const CLK_INFRA_AO_MSDC1_SRC: u32 = 36;
pub const CLK_INFRA_AO_CEC_66M_B: u32 = 37;
pub const CLK_INFRA_AO_PCIE_TL_96M: u32 = 38;
pub const CLK_INFRA_AO_DEVICE_APC: u32 = 39;
pub const CLK_INFRA_AO_ECC_66M_H: u32 = 40;
pub const CLK_INFRA_AO_DEBUGSYS: u32 = 41;
pub const CLK_INFRA_AO_AUDIO: u32 = 42;
pub const CLK_INFRA_AO_PCIE_TL_32K: u32 = 43;
pub const CLK_INFRA_AO_DBG_TRACE: u32 = 44;
pub const CLK_INFRA_AO_DRAMC_F26M: u32 = 45;
pub const CLK_INFRA_AO_IRTX: u32 = 46;
pub const CLK_INFRA_AO_SSUSB: u32 = 47;
pub const CLK_INFRA_AO_DISP_PWM: u32 = 48;
pub const CLK_INFRA_AO_CLDMA_B: u32 = 49;
pub const CLK_INFRA_AO_AUDIO_26M_B: u32 = 50;
pub const CLK_INFRA_AO_SPI1: u32 = 51;
pub const CLK_INFRA_AO_SPI2: u32 = 52;
pub const CLK_INFRA_AO_SPI3: u32 = 53;
pub const CLK_INFRA_AO_UNIPRO_SYS: u32 = 54;
pub const CLK_INFRA_AO_UNIPRO_TICK: u32 = 55;
pub const CLK_INFRA_AO_UFS_MP_SAP_B: u32 = 56;
pub const CLK_INFRA_AO_PWRMCU: u32 = 57;
pub const CLK_INFRA_AO_PWRMCU_BUS_H: u32 = 58;
pub const CLK_INFRA_AO_APDMA_B: u32 = 59;
pub const CLK_INFRA_AO_SPI4: u32 = 60;
pub const CLK_INFRA_AO_SPI5: u32 = 61;
pub const CLK_INFRA_AO_CQ_DMA: u32 = 62;
pub const CLK_INFRA_AO_AES_UFSFDE: u32 = 63;
pub const CLK_INFRA_AO_AES: u32 = 64;
pub const CLK_INFRA_AO_UFS_TICK: u32 = 65;
pub const CLK_INFRA_AO_SSUSB_XHCI: u32 = 66;
pub const CLK_INFRA_AO_MSDC0_SELF: u32 = 67;
pub const CLK_INFRA_AO_MSDC1_SELF: u32 = 68;
pub const CLK_INFRA_AO_MSDC2_SELF: u32 = 69;
pub const CLK_INFRA_AO_I2S_DMA: u32 = 70;
pub const CLK_INFRA_AO_AP_MSDC0: u32 = 71;
pub const CLK_INFRA_AO_MD_MSDC0: u32 = 72;
pub const CLK_INFRA_AO_CG3_MSDC2: u32 = 73;
pub const CLK_INFRA_AO_GCPU: u32 = 74;
pub const CLK_INFRA_AO_PCIE_PERI_26M: u32 = 75;
pub const CLK_INFRA_AO_GCPU_66M_B: u32 = 76;
pub const CLK_INFRA_AO_GCPU_133M_B: u32 = 77;
pub const CLK_INFRA_AO_DISP_PWM1: u32 = 78;
pub const CLK_INFRA_AO_FBIST2FPC: u32 = 79;
pub const CLK_INFRA_AO_DEVICE_APC_SYNC: u32 = 80;
pub const CLK_INFRA_AO_PCIE_P1_PERI_26M: u32 = 81;
pub const CLK_INFRA_AO_SPIS0: u32 = 82;
pub const CLK_INFRA_AO_SPIS1: u32 = 83;
pub const CLK_INFRA_AO_133M_M_PERI: u32 = 84;
pub const CLK_INFRA_AO_66M_M_PERI: u32 = 85;
pub const CLK_INFRA_AO_PCIE_PL_P_250M_P0: u32 = 86;
pub const CLK_INFRA_AO_PCIE_PL_P_250M_P1: u32 = 87;
pub const CLK_INFRA_AO_PCIE_P1_TL_96M: u32 = 88;
pub const CLK_INFRA_AO_AES_MSDCFDE_0P: u32 = 89;
pub const CLK_INFRA_AO_UFS_TX_SYMBOL: u32 = 90;
pub const CLK_INFRA_AO_UFS_RX_SYMBOL: u32 = 91;
pub const CLK_INFRA_AO_UFS_RX_SYMBOL1: u32 = 92;
pub const CLK_INFRA_AO_PERI_UFS_MEM_SUB: u32 = 93;
pub const CLK_INFRA_AO_NR_CLK: u32 = 94;

/* APMIXEDSYS */

pub const CLK_APMIXED_NNAPLL: u32 = 0;
pub const CLK_APMIXED_RESPLL: u32 = 1;
pub const CLK_APMIXED_ETHPLL: u32 = 2;
pub const CLK_APMIXED_MSDCPLL: u32 = 3;
pub const CLK_APMIXED_TVDPLL1: u32 = 4;
pub const CLK_APMIXED_TVDPLL2: u32 = 5;
pub const CLK_APMIXED_MMPLL: u32 = 6;
pub const CLK_APMIXED_MAINPLL: u32 = 7;
pub const CLK_APMIXED_VDECPLL: u32 = 8;
pub const CLK_APMIXED_IMGPLL: u32 = 9;
pub const CLK_APMIXED_UNIVPLL: u32 = 10;
pub const CLK_APMIXED_HDMIPLL1: u32 = 11;
pub const CLK_APMIXED_HDMIPLL2: u32 = 12;
pub const CLK_APMIXED_HDMIRX_APLL: u32 = 13;
pub const CLK_APMIXED_USB1PLL: u32 = 14;
pub const CLK_APMIXED_ADSPPLL: u32 = 15;
pub const CLK_APMIXED_APLL1: u32 = 16;
pub const CLK_APMIXED_APLL2: u32 = 17;
pub const CLK_APMIXED_APLL3: u32 = 18;
pub const CLK_APMIXED_APLL4: u32 = 19;
pub const CLK_APMIXED_APLL5: u32 = 20;
pub const CLK_APMIXED_MFGPLL: u32 = 21;
pub const CLK_APMIXED_DGIPLL: u32 = 22;
pub const CLK_APMIXED_PLL_SSUSB26M: u32 = 23;
pub const CLK_APMIXED_NR_CLK: u32 = 24;

/* SCP_ADSP */

pub const CLK_SCP_ADSP_AUDIODSP: u32 = 0;
pub const CLK_SCP_ADSP_NR_CLK: u32 = 1;

/* PERICFG_AO */

pub const CLK_PERI_AO_ETHERNET: u32 = 0;
pub const CLK_PERI_AO_ETHERNET_BUS: u32 = 1;
pub const CLK_PERI_AO_FLASHIF_BUS: u32 = 2;
pub const CLK_PERI_AO_FLASHIF_FLASH: u32 = 3;
pub const CLK_PERI_AO_SSUSB_1P_BUS: u32 = 4;
pub const CLK_PERI_AO_SSUSB_1P_XHCI: u32 = 5;
pub const CLK_PERI_AO_SSUSB_2P_BUS: u32 = 6;
pub const CLK_PERI_AO_SSUSB_2P_XHCI: u32 = 7;
pub const CLK_PERI_AO_SSUSB_3P_BUS: u32 = 8;
pub const CLK_PERI_AO_SSUSB_3P_XHCI: u32 = 9;
pub const CLK_PERI_AO_SPINFI: u32 = 10;
pub const CLK_PERI_AO_ETHERNET_MAC: u32 = 11;
pub const CLK_PERI_AO_NFI_H: u32 = 12;
pub const CLK_PERI_AO_FNFI1X: u32 = 13;
pub const CLK_PERI_AO_PCIE_P0_MEM: u32 = 14;
pub const CLK_PERI_AO_PCIE_P1_MEM: u32 = 15;
pub const CLK_PERI_AO_NR_CLK: u32 = 16;

/* IMP_IIC_WRAP_S */

pub const CLK_IMP_IIC_WRAP_S_I2C5: u32 = 0;
pub const CLK_IMP_IIC_WRAP_S_I2C6: u32 = 1;
pub const CLK_IMP_IIC_WRAP_S_I2C7: u32 = 2;
pub const CLK_IMP_IIC_WRAP_S_NR_CLK: u32 = 3;

/* IMP_IIC_WRAP_W */

pub const CLK_IMP_IIC_WRAP_W_I2C0: u32 = 0;
pub const CLK_IMP_IIC_WRAP_W_I2C1: u32 = 1;
pub const CLK_IMP_IIC_WRAP_W_I2C2: u32 = 2;
pub const CLK_IMP_IIC_WRAP_W_I2C3: u32 = 3;
pub const CLK_IMP_IIC_WRAP_W_I2C4: u32 = 4;
pub const CLK_IMP_IIC_WRAP_W_NR_CLK: u32 = 5;

/* MFGCFG */

pub const CLK_MFG_BG3D: u32 = 0;
pub const CLK_MFG_NR_CLK: u32 = 1;

/* VPPSYS0 */

pub const CLK_VPP0_MDP_FG: u32 = 0;
pub const CLK_VPP0_STITCH: u32 = 1;
pub const CLK_VPP0_PADDING: u32 = 2;
pub const CLK_VPP0_MDP_TCC: u32 = 3;
pub const CLK_VPP0_WARP0_ASYNC_TX: u32 = 4;
pub const CLK_VPP0_WARP1_ASYNC_TX: u32 = 5;
pub const CLK_VPP0_MUTEX: u32 = 6;
pub const CLK_VPP0_VPP02VPP1_RELAY: u32 = 7;
pub const CLK_VPP0_VPP12VPP0_ASYNC: u32 = 8;
pub const CLK_VPP0_MMSYSRAM_TOP: u32 = 9;
pub const CLK_VPP0_MDP_AAL: u32 = 10;
pub const CLK_VPP0_MDP_RSZ: u32 = 11;
pub const CLK_VPP0_SMI_COMMON: u32 = 12;
pub const CLK_VPP0_GALS_VDO0_LARB0: u32 = 13;
pub const CLK_VPP0_GALS_VDO0_LARB1: u32 = 14;
pub const CLK_VPP0_GALS_VENCSYS: u32 = 15;
pub const CLK_VPP0_GALS_VENCSYS_CORE1: u32 = 16;
pub const CLK_VPP0_GALS_INFRA: u32 = 17;
pub const CLK_VPP0_GALS_CAMSYS: u32 = 18;
pub const CLK_VPP0_GALS_VPP1_LARB5: u32 = 19;
pub const CLK_VPP0_GALS_VPP1_LARB6: u32 = 20;
pub const CLK_VPP0_SMI_REORDER: u32 = 21;
pub const CLK_VPP0_SMI_IOMMU: u32 = 22;
pub const CLK_VPP0_GALS_IMGSYS_CAMSYS: u32 = 23;
pub const CLK_VPP0_MDP_RDMA: u32 = 24;
pub const CLK_VPP0_MDP_WROT: u32 = 25;
pub const CLK_VPP0_GALS_EMI0_EMI1: u32 = 26;
pub const CLK_VPP0_SMI_SUB_COMMON_REORDER: u32 = 27;
pub const CLK_VPP0_SMI_RSI: u32 = 28;
pub const CLK_VPP0_SMI_COMMON_LARB4: u32 = 29;
pub const CLK_VPP0_GALS_VDEC_VDEC_CORE1: u32 = 30;
pub const CLK_VPP0_GALS_VPP1_WPE: u32 = 31;
pub const CLK_VPP0_GALS_VDO0_VDO1_VENCSYS_CORE1: u32 = 32;
pub const CLK_VPP0_FAKE_ENG: u32 = 33;
pub const CLK_VPP0_MDP_HDR: u32 = 34;
pub const CLK_VPP0_MDP_TDSHP: u32 = 35;
pub const CLK_VPP0_MDP_COLOR: u32 = 36;
pub const CLK_VPP0_MDP_OVL: u32 = 37;
pub const CLK_VPP0_WARP0_RELAY: u32 = 38;
pub const CLK_VPP0_WARP0_MDP_DL_ASYNC: u32 = 39;
pub const CLK_VPP0_WARP1_RELAY: u32 = 40;
pub const CLK_VPP0_WARP1_MDP_DL_ASYNC: u32 = 41;
pub const CLK_VPP0_NR_CLK: u32 = 42;

/* WPESYS */

pub const CLK_WPE_VPP0: u32 = 0;
pub const CLK_WPE_VPP1: u32 = 1;
pub const CLK_WPE_SMI_LARB7: u32 = 2;
pub const CLK_WPE_SMI_LARB8: u32 = 3;
pub const CLK_WPE_EVENT_TX: u32 = 4;
pub const CLK_WPE_SMI_LARB7_P: u32 = 5;
pub const CLK_WPE_SMI_LARB8_P: u32 = 6;
pub const CLK_WPE_NR_CLK: u32 = 7;

/* WPESYS_VPP0 */

pub const CLK_WPE_VPP0_VECI: u32 = 0;
pub const CLK_WPE_VPP0_VEC2I: u32 = 1;
pub const CLK_WPE_VPP0_VEC3I: u32 = 2;
pub const CLK_WPE_VPP0_WPEO: u32 = 3;
pub const CLK_WPE_VPP0_MSKO: u32 = 4;
pub const CLK_WPE_VPP0_VGEN: u32 = 5;
pub const CLK_WPE_VPP0_EXT: u32 = 6;
pub const CLK_WPE_VPP0_VFC: u32 = 7;
pub const CLK_WPE_VPP0_CACH0_TOP: u32 = 8;
pub const CLK_WPE_VPP0_CACH0_DMA: u32 = 9;
pub const CLK_WPE_VPP0_CACH1_TOP: u32 = 10;
pub const CLK_WPE_VPP0_CACH1_DMA: u32 = 11;
pub const CLK_WPE_VPP0_CACH2_TOP: u32 = 12;
pub const CLK_WPE_VPP0_CACH2_DMA: u32 = 13;
pub const CLK_WPE_VPP0_CACH3_TOP: u32 = 14;
pub const CLK_WPE_VPP0_CACH3_DMA: u32 = 15;
pub const CLK_WPE_VPP0_PSP: u32 = 16;
pub const CLK_WPE_VPP0_PSP2: u32 = 17;
pub const CLK_WPE_VPP0_SYNC: u32 = 18;
pub const CLK_WPE_VPP0_C24: u32 = 19;
pub const CLK_WPE_VPP0_MDP_CROP: u32 = 20;
pub const CLK_WPE_VPP0_ISP_CROP: u32 = 21;
pub const CLK_WPE_VPP0_TOP: u32 = 22;
pub const CLK_WPE_VPP0_NR_CLK: u32 = 23;

/* WPESYS_VPP1 */

pub const CLK_WPE_VPP1_VECI: u32 = 0;
pub const CLK_WPE_VPP1_VEC2I: u32 = 1;
pub const CLK_WPE_VPP1_VEC3I: u32 = 2;
pub const CLK_WPE_VPP1_WPEO: u32 = 3;
pub const CLK_WPE_VPP1_MSKO: u32 = 4;
pub const CLK_WPE_VPP1_VGEN: u32 = 5;
pub const CLK_WPE_VPP1_EXT: u32 = 6;
pub const CLK_WPE_VPP1_VFC: u32 = 7;
pub const CLK_WPE_VPP1_CACH0_TOP: u32 = 8;
pub const CLK_WPE_VPP1_CACH0_DMA: u32 = 9;
pub const CLK_WPE_VPP1_CACH1_TOP: u32 = 10;
pub const CLK_WPE_VPP1_CACH1_DMA: u32 = 11;
pub const CLK_WPE_VPP1_CACH2_TOP: u32 = 12;
pub const CLK_WPE_VPP1_CACH2_DMA: u32 = 13;
pub const CLK_WPE_VPP1_CACH3_TOP: u32 = 14;
pub const CLK_WPE_VPP1_CACH3_DMA: u32 = 15;
pub const CLK_WPE_VPP1_PSP: u32 = 16;
pub const CLK_WPE_VPP1_PSP2: u32 = 17;
pub const CLK_WPE_VPP1_SYNC: u32 = 18;
pub const CLK_WPE_VPP1_C24: u32 = 19;
pub const CLK_WPE_VPP1_MDP_CROP: u32 = 20;
pub const CLK_WPE_VPP1_ISP_CROP: u32 = 21;
pub const CLK_WPE_VPP1_TOP: u32 = 22;
pub const CLK_WPE_VPP1_NR_CLK: u32 = 23;

/* VPPSYS1 */

pub const CLK_VPP1_SVPP1_MDP_OVL: u32 = 0;
pub const CLK_VPP1_SVPP1_MDP_TCC: u32 = 1;
pub const CLK_VPP1_SVPP1_MDP_WROT: u32 = 2;
pub const CLK_VPP1_SVPP1_VPP_PAD: u32 = 3;
pub const CLK_VPP1_SVPP2_MDP_WROT: u32 = 4;
pub const CLK_VPP1_SVPP2_VPP_PAD: u32 = 5;
pub const CLK_VPP1_SVPP3_MDP_WROT: u32 = 6;
pub const CLK_VPP1_SVPP3_VPP_PAD: u32 = 7;
pub const CLK_VPP1_SVPP1_MDP_RDMA: u32 = 8;
pub const CLK_VPP1_SVPP1_MDP_FG: u32 = 9;
pub const CLK_VPP1_SVPP2_MDP_RDMA: u32 = 10;
pub const CLK_VPP1_SVPP2_MDP_FG: u32 = 11;
pub const CLK_VPP1_SVPP3_MDP_RDMA: u32 = 12;
pub const CLK_VPP1_SVPP3_MDP_FG: u32 = 13;
pub const CLK_VPP1_VPP_SPLIT: u32 = 14;
pub const CLK_VPP1_SVPP2_VDO0_DL_RELAY: u32 = 15;
pub const CLK_VPP1_SVPP1_MDP_TDSHP: u32 = 16;
pub const CLK_VPP1_SVPP1_MDP_COLOR: u32 = 17;
pub const CLK_VPP1_SVPP3_VDO1_DL_RELAY: u32 = 18;
pub const CLK_VPP1_SVPP2_VPP_MERGE: u32 = 19;
pub const CLK_VPP1_SVPP2_MDP_COLOR: u32 = 20;
pub const CLK_VPP1_VPPSYS1_GALS: u32 = 21;
pub const CLK_VPP1_SVPP3_VPP_MERGE: u32 = 22;
pub const CLK_VPP1_SVPP3_MDP_COLOR: u32 = 23;
pub const CLK_VPP1_VPPSYS1_LARB: u32 = 24;
pub const CLK_VPP1_SVPP1_MDP_RSZ: u32 = 25;
pub const CLK_VPP1_SVPP1_MDP_HDR: u32 = 26;
pub const CLK_VPP1_SVPP1_MDP_AAL: u32 = 27;
pub const CLK_VPP1_SVPP2_MDP_HDR: u32 = 28;
pub const CLK_VPP1_SVPP2_MDP_AAL: u32 = 29;
pub const CLK_VPP1_DL_ASYNC: u32 = 30;
pub const CLK_VPP1_LARB5_FAKE_ENG: u32 = 31;
pub const CLK_VPP1_SVPP3_MDP_HDR: u32 = 32;
pub const CLK_VPP1_SVPP3_MDP_AAL: u32 = 33;
pub const CLK_VPP1_SVPP2_VDO1_DL_RELAY: u32 = 34;
pub const CLK_VPP1_LARB6_FAKE_ENG: u32 = 35;
pub const CLK_VPP1_SVPP2_MDP_RSZ: u32 = 36;
pub const CLK_VPP1_SVPP3_MDP_RSZ: u32 = 37;
pub const CLK_VPP1_SVPP3_VDO0_DL_RELAY: u32 = 38;
pub const CLK_VPP1_DISP_MUTEX: u32 = 39;
pub const CLK_VPP1_SVPP2_MDP_TDSHP: u32 = 40;
pub const CLK_VPP1_SVPP3_MDP_TDSHP: u32 = 41;
pub const CLK_VPP1_VPP0_DL1_RELAY: u32 = 42;
pub const CLK_VPP1_HDMI_META: u32 = 43;
pub const CLK_VPP1_VPP_SPLIT_HDMI: u32 = 44;
pub const CLK_VPP1_DGI_IN: u32 = 45;
pub const CLK_VPP1_DGI_OUT: u32 = 46;
pub const CLK_VPP1_VPP_SPLIT_DGI: u32 = 47;
pub const CLK_VPP1_VPP0_DL_ASYNC: u32 = 48;
pub const CLK_VPP1_VPP0_DL_RELAY: u32 = 49;
pub const CLK_VPP1_VPP_SPLIT_26M: u32 = 50;
pub const CLK_VPP1_NR_CLK: u32 = 51;

/* IMGSYS */

pub const CLK_IMG_LARB9: u32 = 0;
pub const CLK_IMG_TRAW0: u32 = 1;
pub const CLK_IMG_TRAW1: u32 = 2;
pub const CLK_IMG_TRAW2: u32 = 3;
pub const CLK_IMG_TRAW3: u32 = 4;
pub const CLK_IMG_DIP0: u32 = 5;
pub const CLK_IMG_WPE0: u32 = 6;
pub const CLK_IMG_IPE: u32 = 7;
pub const CLK_IMG_DIP1: u32 = 8;
pub const CLK_IMG_WPE1: u32 = 9;
pub const CLK_IMG_GALS: u32 = 10;
pub const CLK_IMG_NR_CLK: u32 = 11;

/* IMGSYS1_DIP_TOP */

pub const CLK_IMG1_DIP_TOP_LARB10: u32 = 0;
pub const CLK_IMG1_DIP_TOP_DIP_TOP: u32 = 1;
pub const CLK_IMG1_DIP_TOP_NR_CLK: u32 = 2;

/* IMGSYS1_DIP_NR */

pub const CLK_IMG1_DIP_NR_RESERVE: u32 = 0;
pub const CLK_IMG1_DIP_NR_DIP_NR: u32 = 1;
pub const CLK_IMG1_DIP_NR_NR_CLK: u32 = 2;

/* IMGSYS1_WPE */

pub const CLK_IMG1_WPE_LARB11: u32 = 0;
pub const CLK_IMG1_WPE_WPE: u32 = 1;
pub const CLK_IMG1_WPE_NR_CLK: u32 = 2;

/* IPESYS */

pub const CLK_IPE_DPE: u32 = 0;
pub const CLK_IPE_FDVT: u32 = 1;
pub const CLK_IPE_ME: u32 = 2;
pub const CLK_IPE_TOP: u32 = 3;
pub const CLK_IPE_SMI_LARB12: u32 = 4;
pub const CLK_IPE_NR_CLK: u32 = 5;

/* CAMSYS */

pub const CLK_CAM_LARB13: u32 = 0;
pub const CLK_CAM_LARB14: u32 = 1;
pub const CLK_CAM_MAIN_CAM: u32 = 2;
pub const CLK_CAM_MAIN_CAMTG: u32 = 3;
pub const CLK_CAM_SENINF: u32 = 4;
pub const CLK_CAM_GCAMSVA: u32 = 5;
pub const CLK_CAM_GCAMSVB: u32 = 6;
pub const CLK_CAM_GCAMSVC: u32 = 7;
pub const CLK_CAM_SCAMSA: u32 = 8;
pub const CLK_CAM_SCAMSB: u32 = 9;
pub const CLK_CAM_CAMSV_TOP: u32 = 10;
pub const CLK_CAM_CAMSV_CQ: u32 = 11;
pub const CLK_CAM_ADL: u32 = 12;
pub const CLK_CAM_ASG: u32 = 13;
pub const CLK_CAM_PDA: u32 = 14;
pub const CLK_CAM_FAKE_ENG: u32 = 15;
pub const CLK_CAM_MAIN_MRAW0: u32 = 16;
pub const CLK_CAM_MAIN_MRAW1: u32 = 17;
pub const CLK_CAM_MAIN_MRAW2: u32 = 18;
pub const CLK_CAM_MAIN_MRAW3: u32 = 19;
pub const CLK_CAM_CAM2MM0_GALS: u32 = 20;
pub const CLK_CAM_CAM2MM1_GALS: u32 = 21;
pub const CLK_CAM_CAM2SYS_GALS: u32 = 22;
pub const CLK_CAM_NR_CLK: u32 = 23;

/* CAMSYS_RAWA */

pub const CLK_CAM_RAWA_LARBX: u32 = 0;
pub const CLK_CAM_RAWA_CAM: u32 = 1;
pub const CLK_CAM_RAWA_CAMTG: u32 = 2;
pub const CLK_CAM_RAWA_NR_CLK: u32 = 3;

/* CAMSYS_YUVA */

pub const CLK_CAM_YUVA_LARBX: u32 = 0;
pub const CLK_CAM_YUVA_CAM: u32 = 1;
pub const CLK_CAM_YUVA_CAMTG: u32 = 2;
pub const CLK_CAM_YUVA_NR_CLK: u32 = 3;

/* CAMSYS_RAWB */

pub const CLK_CAM_RAWB_LARBX: u32 = 0;
pub const CLK_CAM_RAWB_CAM: u32 = 1;
pub const CLK_CAM_RAWB_CAMTG: u32 = 2;
pub const CLK_CAM_RAWB_NR_CLK: u32 = 3;

/* CAMSYS_YUVB */

pub const CLK_CAM_YUVB_LARBX: u32 = 0;
pub const CLK_CAM_YUVB_CAM: u32 = 1;
pub const CLK_CAM_YUVB_CAMTG: u32 = 2;
pub const CLK_CAM_YUVB_NR_CLK: u32 = 3;

/* CAMSYS_MRAW */

pub const CLK_CAM_MRAW_LARBX: u32 = 0;
pub const CLK_CAM_MRAW_CAMTG: u32 = 1;
pub const CLK_CAM_MRAW_MRAW0: u32 = 2;
pub const CLK_CAM_MRAW_MRAW1: u32 = 3;
pub const CLK_CAM_MRAW_MRAW2: u32 = 4;
pub const CLK_CAM_MRAW_MRAW3: u32 = 5;
pub const CLK_CAM_MRAW_NR_CLK: u32 = 6;

/* CCUSYS */

pub const CLK_CCU_LARB18: u32 = 0;
pub const CLK_CCU_AHB: u32 = 1;
pub const CLK_CCU_CCU0: u32 = 2;
pub const CLK_CCU_CCU1: u32 = 3;
pub const CLK_CCU_NR_CLK: u32 = 4;

/* VDECSYS_SOC */

pub const CLK_VDEC_SOC_LARB1: u32 = 0;
pub const CLK_VDEC_SOC_LAT: u32 = 1;
pub const CLK_VDEC_SOC_VDEC: u32 = 2;
pub const CLK_VDEC_SOC_NR_CLK: u32 = 3;

/* VDECSYS */

pub const CLK_VDEC_LARB1: u32 = 0;
pub const CLK_VDEC_LAT: u32 = 1;
pub const CLK_VDEC_VDEC: u32 = 2;
pub const CLK_VDEC_NR_CLK: u32 = 3;

/* VDECSYS_CORE1 */

pub const CLK_VDEC_CORE1_LARB1: u32 = 0;
pub const CLK_VDEC_CORE1_LAT: u32 = 1;
pub const CLK_VDEC_CORE1_VDEC: u32 = 2;
pub const CLK_VDEC_CORE1_NR_CLK: u32 = 3;

/* APUSYS_PLL */

pub const CLK_APUSYS_PLL_APUPLL: u32 = 0;
pub const CLK_APUSYS_PLL_NPUPLL: u32 = 1;
pub const CLK_APUSYS_PLL_APUPLL1: u32 = 2;
pub const CLK_APUSYS_PLL_APUPLL2: u32 = 3;
pub const CLK_APUSYS_PLL_NR_CLK: u32 = 4;

/* VENCSYS */

pub const CLK_VENC_LARB: u32 = 0;
pub const CLK_VENC_VENC: u32 = 1;
pub const CLK_VENC_JPGENC: u32 = 2;
pub const CLK_VENC_JPGDEC: u32 = 3;
pub const CLK_VENC_JPGDEC_C1: u32 = 4;
pub const CLK_VENC_GALS: u32 = 5;
pub const CLK_VENC_NR_CLK: u32 = 6;

/* VENCSYS_CORE1 */

pub const CLK_VENC_CORE1_LARB: u32 = 0;
pub const CLK_VENC_CORE1_VENC: u32 = 1;
pub const CLK_VENC_CORE1_JPGENC: u32 = 2;
pub const CLK_VENC_CORE1_JPGDEC: u32 = 3;
pub const CLK_VENC_CORE1_JPGDEC_C1: u32 = 4;
pub const CLK_VENC_CORE1_GALS: u32 = 5;
pub const CLK_VENC_CORE1_NR_CLK: u32 = 6;

/* VDOSYS0 */

pub const CLK_VDO0_DISP_OVL0: u32 = 0;
pub const CLK_VDO0_DISP_COLOR0: u32 = 1;
pub const CLK_VDO0_DISP_COLOR1: u32 = 2;
pub const CLK_VDO0_DISP_CCORR0: u32 = 3;
pub const CLK_VDO0_DISP_CCORR1: u32 = 4;
pub const CLK_VDO0_DISP_AAL0: u32 = 5;
pub const CLK_VDO0_DISP_AAL1: u32 = 6;
pub const CLK_VDO0_DISP_GAMMA0: u32 = 7;
pub const CLK_VDO0_DISP_GAMMA1: u32 = 8;
pub const CLK_VDO0_DISP_DITHER0: u32 = 9;
pub const CLK_VDO0_DISP_DITHER1: u32 = 10;
pub const CLK_VDO0_DISP_OVL1: u32 = 11;
pub const CLK_VDO0_DISP_WDMA0: u32 = 12;
pub const CLK_VDO0_DISP_WDMA1: u32 = 13;
pub const CLK_VDO0_DISP_RDMA0: u32 = 14;
pub const CLK_VDO0_DISP_RDMA1: u32 = 15;
pub const CLK_VDO0_DSI0: u32 = 16;
pub const CLK_VDO0_DSI1: u32 = 17;
pub const CLK_VDO0_DSC_WRAP0: u32 = 18;
pub const CLK_VDO0_VPP_MERGE0: u32 = 19;
pub const CLK_VDO0_DP_INTF0: u32 = 20;
pub const CLK_VDO0_DISP_MUTEX0: u32 = 21;
pub const CLK_VDO0_DISP_IL_ROT0: u32 = 22;
pub const CLK_VDO0_APB_BUS: u32 = 23;
pub const CLK_VDO0_FAKE_ENG0: u32 = 24;
pub const CLK_VDO0_FAKE_ENG1: u32 = 25;
pub const CLK_VDO0_DL_ASYNC0: u32 = 26;
pub const CLK_VDO0_DL_ASYNC1: u32 = 27;
pub const CLK_VDO0_DL_ASYNC2: u32 = 28;
pub const CLK_VDO0_DL_ASYNC3: u32 = 29;
pub const CLK_VDO0_DL_ASYNC4: u32 = 30;
pub const CLK_VDO0_DISP_MONITOR0: u32 = 31;
pub const CLK_VDO0_DISP_MONITOR1: u32 = 32;
pub const CLK_VDO0_DISP_MONITOR2: u32 = 33;
pub const CLK_VDO0_DISP_MONITOR3: u32 = 34;
pub const CLK_VDO0_DISP_MONITOR4: u32 = 35;
pub const CLK_VDO0_SMI_GALS: u32 = 36;
pub const CLK_VDO0_SMI_COMMON: u32 = 37;
pub const CLK_VDO0_SMI_EMI: u32 = 38;
pub const CLK_VDO0_SMI_IOMMU: u32 = 39;
pub const CLK_VDO0_SMI_LARB: u32 = 40;
pub const CLK_VDO0_SMI_RSI: u32 = 41;
pub const CLK_VDO0_DSI0_DSI: u32 = 42;
pub const CLK_VDO0_DSI1_DSI: u32 = 43;
pub const CLK_VDO0_DP_INTF0_DP_INTF: u32 = 44;
pub const CLK_VDO0_NR_CLK: u32 = 45;

/* VDOSYS1 */

pub const CLK_VDO1_SMI_LARB2: u32 = 0;
pub const CLK_VDO1_SMI_LARB3: u32 = 1;
pub const CLK_VDO1_GALS: u32 = 2;
pub const CLK_VDO1_FAKE_ENG0: u32 = 3;
pub const CLK_VDO1_FAKE_ENG: u32 = 4;
pub const CLK_VDO1_MDP_RDMA0: u32 = 5;
pub const CLK_VDO1_MDP_RDMA1: u32 = 6;
pub const CLK_VDO1_MDP_RDMA2: u32 = 7;
pub const CLK_VDO1_MDP_RDMA3: u32 = 8;
pub const CLK_VDO1_VPP_MERGE0: u32 = 9;
pub const CLK_VDO1_VPP_MERGE1: u32 = 10;
pub const CLK_VDO1_VPP_MERGE2: u32 = 11;
pub const CLK_VDO1_VPP_MERGE3: u32 = 12;
pub const CLK_VDO1_VPP_MERGE4: u32 = 13;
pub const CLK_VDO1_VPP2_TO_VDO1_DL_ASYNC: u32 = 14;
pub const CLK_VDO1_VPP3_TO_VDO1_DL_ASYNC: u32 = 15;
pub const CLK_VDO1_DISP_MUTEX: u32 = 16;
pub const CLK_VDO1_MDP_RDMA4: u32 = 17;
pub const CLK_VDO1_MDP_RDMA5: u32 = 18;
pub const CLK_VDO1_MDP_RDMA6: u32 = 19;
pub const CLK_VDO1_MDP_RDMA7: u32 = 20;
pub const CLK_VDO1_DP_INTF0_MM: u32 = 21;
pub const CLK_VDO1_DPI0_MM: u32 = 22;
pub const CLK_VDO1_DPI1_MM: u32 = 23;
pub const CLK_VDO1_DISP_MONITOR: u32 = 24;
pub const CLK_VDO1_MERGE0_DL_ASYNC: u32 = 25;
pub const CLK_VDO1_MERGE1_DL_ASYNC: u32 = 26;
pub const CLK_VDO1_MERGE2_DL_ASYNC: u32 = 27;
pub const CLK_VDO1_MERGE3_DL_ASYNC: u32 = 28;
pub const CLK_VDO1_MERGE4_DL_ASYNC: u32 = 29;
pub const CLK_VDO1_VDO0_DSC_TO_VDO1_DL_ASYNC: u32 = 30;
pub const CLK_VDO1_VDO0_MERGE_TO_VDO1_DL_ASYNC: u32 = 31;
pub const CLK_VDO1_HDR_VDO_FE0: u32 = 32;
pub const CLK_VDO1_HDR_GFX_FE0: u32 = 33;
pub const CLK_VDO1_HDR_VDO_BE: u32 = 34;
pub const CLK_VDO1_HDR_VDO_FE1: u32 = 35;
pub const CLK_VDO1_HDR_GFX_FE1: u32 = 36;
pub const CLK_VDO1_DISP_MIXER: u32 = 37;
pub const CLK_VDO1_HDR_VDO_FE0_DL_ASYNC: u32 = 38;
pub const CLK_VDO1_HDR_VDO_FE1_DL_ASYNC: u32 = 39;
pub const CLK_VDO1_HDR_GFX_FE0_DL_ASYNC: u32 = 40;
pub const CLK_VDO1_HDR_GFX_FE1_DL_ASYNC: u32 = 41;
pub const CLK_VDO1_HDR_VDO_BE_DL_ASYNC: u32 = 42;
pub const CLK_VDO1_DPI0: u32 = 43;
pub const CLK_VDO1_DISP_MONITOR_DPI0: u32 = 44;
pub const CLK_VDO1_DPI1: u32 = 45;
pub const CLK_VDO1_DISP_MONITOR_DPI1: u32 = 46;
pub const CLK_VDO1_DPINTF: u32 = 47;
pub const CLK_VDO1_DISP_MONITOR_DPINTF: u32 = 48;
pub const CLK_VDO1_26M_SLOW: u32 = 49;
pub const CLK_VDO1_DPI1_HDMI: u32 = 50;
pub const CLK_VDO1_NR_CLK: u32 = 51;




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
