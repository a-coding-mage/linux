/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of the MediaTek MT2701 clock bindings header. */
/* SPDX-License-Identifier: GPL-2.0-only */
/*


/* TOPCKGEN */
pub const CLK_TOP_SYSPLL: u32 = 1;
pub const CLK_TOP_SYSPLL_D2: u32 = 2;
pub const CLK_TOP_SYSPLL_D3: u32 = 3;
pub const CLK_TOP_SYSPLL_D5: u32 = 4;
pub const CLK_TOP_SYSPLL_D7: u32 = 5;
pub const CLK_TOP_SYSPLL1_D2: u32 = 6;
pub const CLK_TOP_SYSPLL1_D4: u32 = 7;
pub const CLK_TOP_SYSPLL1_D8: u32 = 8;
pub const CLK_TOP_SYSPLL1_D16: u32 = 9;
pub const CLK_TOP_SYSPLL2_D2: u32 = 10;
pub const CLK_TOP_SYSPLL2_D4: u32 = 11;
pub const CLK_TOP_SYSPLL2_D8: u32 = 12;
pub const CLK_TOP_SYSPLL3_D2: u32 = 13;
pub const CLK_TOP_SYSPLL3_D4: u32 = 14;
pub const CLK_TOP_SYSPLL4_D2: u32 = 15;
pub const CLK_TOP_SYSPLL4_D4: u32 = 16;
pub const CLK_TOP_UNIVPLL: u32 = 17;
pub const CLK_TOP_UNIVPLL_D2: u32 = 18;
pub const CLK_TOP_UNIVPLL_D3: u32 = 19;
pub const CLK_TOP_UNIVPLL_D5: u32 = 20;
pub const CLK_TOP_UNIVPLL_D7: u32 = 21;
pub const CLK_TOP_UNIVPLL_D26: u32 = 22;
pub const CLK_TOP_UNIVPLL_D52: u32 = 23;
pub const CLK_TOP_UNIVPLL_D108: u32 = 24;
pub const CLK_TOP_USB_PHY48M: u32 = 25;
pub const CLK_TOP_UNIVPLL1_D2: u32 = 26;
pub const CLK_TOP_UNIVPLL1_D4: u32 = 27;
pub const CLK_TOP_UNIVPLL1_D8: u32 = 28;
pub const CLK_TOP_UNIVPLL2_D2: u32 = 29;
pub const CLK_TOP_UNIVPLL2_D4: u32 = 30;
pub const CLK_TOP_UNIVPLL2_D8: u32 = 31;
pub const CLK_TOP_UNIVPLL2_D16: u32 = 32;
pub const CLK_TOP_UNIVPLL2_D32: u32 = 33;
pub const CLK_TOP_UNIVPLL3_D2: u32 = 34;
pub const CLK_TOP_UNIVPLL3_D4: u32 = 35;
pub const CLK_TOP_UNIVPLL3_D8: u32 = 36;
pub const CLK_TOP_MSDCPLL: u32 = 37;
pub const CLK_TOP_MSDCPLL_D2: u32 = 38;
pub const CLK_TOP_MSDCPLL_D4: u32 = 39;
pub const CLK_TOP_MSDCPLL_D8: u32 = 40;
pub const CLK_TOP_MMPLL: u32 = 41;
pub const CLK_TOP_MMPLL_D2: u32 = 42;
pub const CLK_TOP_DMPLL: u32 = 43;
pub const CLK_TOP_DMPLL_D2: u32 = 44;
pub const CLK_TOP_DMPLL_D4: u32 = 45;
pub const CLK_TOP_DMPLL_X2: u32 = 46;
pub const CLK_TOP_TVDPLL: u32 = 47;
pub const CLK_TOP_TVDPLL_D2: u32 = 48;
pub const CLK_TOP_TVDPLL_D4: u32 = 49;
pub const CLK_TOP_TVD2PLL: u32 = 50;
pub const CLK_TOP_TVD2PLL_D2: u32 = 51;
pub const CLK_TOP_HADDS2PLL_98M: u32 = 52;
pub const CLK_TOP_HADDS2PLL_294M: u32 = 53;
pub const CLK_TOP_HADDS2_FB: u32 = 54;
pub const CLK_TOP_MIPIPLL_D2: u32 = 55;
pub const CLK_TOP_MIPIPLL_D4: u32 = 56;
pub const CLK_TOP_HDMIPLL: u32 = 57;
pub const CLK_TOP_HDMIPLL_D2: u32 = 58;
pub const CLK_TOP_HDMIPLL_D3: u32 = 59;
pub const CLK_TOP_HDMI_SCL_RX: u32 = 60;
pub const CLK_TOP_HDMI_0_PIX340M: u32 = 61;
pub const CLK_TOP_HDMI_0_DEEP340M: u32 = 62;
pub const CLK_TOP_HDMI_0_PLL340M: u32 = 63;
pub const CLK_TOP_AUD1PLL_98M: u32 = 64;
pub const CLK_TOP_AUD2PLL_90M: u32 = 65;
pub const CLK_TOP_AUDPLL: u32 = 66;
pub const CLK_TOP_AUDPLL_D4: u32 = 67;
pub const CLK_TOP_AUDPLL_D8: u32 = 68;
pub const CLK_TOP_AUDPLL_D16: u32 = 69;
pub const CLK_TOP_AUDPLL_D24: u32 = 70;
pub const CLK_TOP_ETHPLL_500M: u32 = 71;
pub const CLK_TOP_VDECPLL: u32 = 72;
pub const CLK_TOP_VENCPLL: u32 = 73;
pub const CLK_TOP_MIPIPLL: u32 = 74;
pub const CLK_TOP_ARMPLL_1P3G: u32 = 75;

pub const CLK_TOP_MM_SEL: u32 = 76;
pub const CLK_TOP_DDRPHYCFG_SEL: u32 = 77;
pub const CLK_TOP_MEM_SEL: u32 = 78;
pub const CLK_TOP_AXI_SEL: u32 = 79;
pub const CLK_TOP_CAMTG_SEL: u32 = 80;
pub const CLK_TOP_MFG_SEL: u32 = 81;
pub const CLK_TOP_VDEC_SEL: u32 = 82;
pub const CLK_TOP_PWM_SEL: u32 = 83;
pub const CLK_TOP_MSDC30_0_SEL: u32 = 84;
pub const CLK_TOP_USB20_SEL: u32 = 85;
pub const CLK_TOP_SPI0_SEL: u32 = 86;
pub const CLK_TOP_UART_SEL: u32 = 87;
pub const CLK_TOP_AUDINTBUS_SEL: u32 = 88;
pub const CLK_TOP_AUDIO_SEL: u32 = 89;
pub const CLK_TOP_MSDC30_2_SEL: u32 = 90;
pub const CLK_TOP_MSDC30_1_SEL: u32 = 91;
pub const CLK_TOP_DPI1_SEL: u32 = 92;
pub const CLK_TOP_DPI0_SEL: u32 = 93;
pub const CLK_TOP_SCP_SEL: u32 = 94;
pub const CLK_TOP_PMICSPI_SEL: u32 = 95;
pub const CLK_TOP_APLL_SEL: u32 = 96;
pub const CLK_TOP_HDMI_SEL: u32 = 97;
pub const CLK_TOP_TVE_SEL: u32 = 98;
pub const CLK_TOP_EMMC_HCLK_SEL: u32 = 99;
pub const CLK_TOP_NFI2X_SEL: u32 = 100;
pub const CLK_TOP_RTC_SEL: u32 = 101;
pub const CLK_TOP_OSD_SEL: u32 = 102;
pub const CLK_TOP_NR_SEL: u32 = 103;
pub const CLK_TOP_DI_SEL: u32 = 104;
pub const CLK_TOP_FLASH_SEL: u32 = 105;
pub const CLK_TOP_ASM_M_SEL: u32 = 106;
pub const CLK_TOP_ASM_I_SEL: u32 = 107;
pub const CLK_TOP_INTDIR_SEL: u32 = 108;
pub const CLK_TOP_HDMIRX_BIST_SEL: u32 = 109;
pub const CLK_TOP_ETHIF_SEL: u32 = 110;
pub const CLK_TOP_MS_CARD_SEL: u32 = 111;
pub const CLK_TOP_ASM_H_SEL: u32 = 112;
pub const CLK_TOP_SPI1_SEL: u32 = 113;
pub const CLK_TOP_CMSYS_SEL: u32 = 114;
pub const CLK_TOP_MSDC30_3_SEL: u32 = 115;
pub const CLK_TOP_HDMIRX26_24_SEL: u32 = 116;
pub const CLK_TOP_AUD2DVD_SEL: u32 = 117;
pub const CLK_TOP_8BDAC_SEL: u32 = 118;
pub const CLK_TOP_SPI2_SEL: u32 = 119;
pub const CLK_TOP_AUD_MUX1_SEL: u32 = 120;
pub const CLK_TOP_AUD_MUX2_SEL: u32 = 121;
pub const CLK_TOP_AUDPLL_MUX_SEL: u32 = 122;
pub const CLK_TOP_AUD_K1_SRC_SEL: u32 = 123;
pub const CLK_TOP_AUD_K2_SRC_SEL: u32 = 124;
pub const CLK_TOP_AUD_K3_SRC_SEL: u32 = 125;
pub const CLK_TOP_AUD_K4_SRC_SEL: u32 = 126;
pub const CLK_TOP_AUD_K5_SRC_SEL: u32 = 127;
pub const CLK_TOP_AUD_K6_SRC_SEL: u32 = 128;
pub const CLK_TOP_PADMCLK_SEL: u32 = 129;
pub const CLK_TOP_AUD_EXTCK1_DIV: u32 = 130;
pub const CLK_TOP_AUD_EXTCK2_DIV: u32 = 131;
pub const CLK_TOP_AUD_MUX1_DIV: u32 = 132;
pub const CLK_TOP_AUD_MUX2_DIV: u32 = 133;
pub const CLK_TOP_AUD_K1_SRC_DIV: u32 = 134;
pub const CLK_TOP_AUD_K2_SRC_DIV: u32 = 135;
pub const CLK_TOP_AUD_K3_SRC_DIV: u32 = 136;
pub const CLK_TOP_AUD_K4_SRC_DIV: u32 = 137;
pub const CLK_TOP_AUD_K5_SRC_DIV: u32 = 138;
pub const CLK_TOP_AUD_K6_SRC_DIV: u32 = 139;
pub const CLK_TOP_AUD_I2S1_MCLK: u32 = 140;
pub const CLK_TOP_AUD_I2S2_MCLK: u32 = 141;
pub const CLK_TOP_AUD_I2S3_MCLK: u32 = 142;
pub const CLK_TOP_AUD_I2S4_MCLK: u32 = 143;
pub const CLK_TOP_AUD_I2S5_MCLK: u32 = 144;
pub const CLK_TOP_AUD_I2S6_MCLK: u32 = 145;
pub const CLK_TOP_AUD_48K_TIMING: u32 = 146;
pub const CLK_TOP_AUD_44K_TIMING: u32 = 147;

pub const CLK_TOP_32K_INTERNAL: u32 = 148;
pub const CLK_TOP_32K_EXTERNAL: u32 = 149;
pub const CLK_TOP_CLK26M_D8: u32 = 150;
pub const CLK_TOP_8BDAC: u32 = 151;
pub const CLK_TOP_WBG_DIG_416M: u32 = 152;
pub const CLK_TOP_DPI: u32 = 153;
pub const CLK_TOP_DSI0_LNTC_DSI: u32 = 154;
pub const CLK_TOP_AUD_EXT1: u32 = 155;
pub const CLK_TOP_AUD_EXT2: u32 = 156;
pub const CLK_TOP_NFI1X_PAD: u32 = 157;
pub const CLK_TOP_AXISEL_D4: u32 = 158;
pub const CLK_TOP_NR: u32 = 159;

/* APMIXEDSYS */

pub const CLK_APMIXED_ARMPLL: u32 = 1;
pub const CLK_APMIXED_MAINPLL: u32 = 2;
pub const CLK_APMIXED_UNIVPLL: u32 = 3;
pub const CLK_APMIXED_MMPLL: u32 = 4;
pub const CLK_APMIXED_MSDCPLL: u32 = 5;
pub const CLK_APMIXED_TVDPLL: u32 = 6;
pub const CLK_APMIXED_AUD1PLL: u32 = 7;
pub const CLK_APMIXED_TRGPLL: u32 = 8;
pub const CLK_APMIXED_ETHPLL: u32 = 9;
pub const CLK_APMIXED_VDECPLL: u32 = 10;
pub const CLK_APMIXED_HADDS2PLL: u32 = 11;
pub const CLK_APMIXED_AUD2PLL: u32 = 12;
pub const CLK_APMIXED_TVD2PLL: u32 = 13;
pub const CLK_APMIXED_HDMI_REF: u32 = 14;
pub const CLK_APMIXED_NR: u32 = 15;

/* DDRPHY */

pub const CLK_DDRPHY_VENCPLL: u32 = 1;
pub const CLK_DDRPHY_NR: u32 = 2;

/* INFRACFG */

pub const CLK_INFRA_DBG: u32 = 1;
pub const CLK_INFRA_SMI: u32 = 2;
pub const CLK_INFRA_QAXI_CM4: u32 = 3;
pub const CLK_INFRA_AUD_SPLIN_B: u32 = 4;
pub const CLK_INFRA_AUDIO: u32 = 5;
pub const CLK_INFRA_EFUSE: u32 = 6;
pub const CLK_INFRA_L2C_SRAM: u32 = 7;
pub const CLK_INFRA_M4U: u32 = 8;
pub const CLK_INFRA_CONNMCU: u32 = 9;
pub const CLK_INFRA_TRNG: u32 = 10;
pub const CLK_INFRA_RAMBUFIF: u32 = 11;
pub const CLK_INFRA_CPUM: u32 = 12;
pub const CLK_INFRA_KP: u32 = 13;
pub const CLK_INFRA_CEC: u32 = 14;
pub const CLK_INFRA_IRRX: u32 = 15;
pub const CLK_INFRA_PMICSPI: u32 = 16;
pub const CLK_INFRA_PMICWRAP: u32 = 17;
pub const CLK_INFRA_DDCCI: u32 = 18;
pub const CLK_INFRA_CLK_13M: u32 = 19;
pub const CLK_INFRA_CPUSEL: u32 = 20;
pub const CLK_INFRA_NR: u32 = 21;

/* PERICFG */

pub const CLK_PERI_NFI: u32 = 1;
pub const CLK_PERI_THERM: u32 = 2;
pub const CLK_PERI_PWM1: u32 = 3;
pub const CLK_PERI_PWM2: u32 = 4;
pub const CLK_PERI_PWM3: u32 = 5;
pub const CLK_PERI_PWM4: u32 = 6;
pub const CLK_PERI_PWM5: u32 = 7;
pub const CLK_PERI_PWM6: u32 = 8;
pub const CLK_PERI_PWM7: u32 = 9;
pub const CLK_PERI_PWM: u32 = 10;
pub const CLK_PERI_USB0: u32 = 11;
pub const CLK_PERI_USB1: u32 = 12;
pub const CLK_PERI_AP_DMA: u32 = 13;
pub const CLK_PERI_MSDC30_0: u32 = 14;
pub const CLK_PERI_MSDC30_1: u32 = 15;
pub const CLK_PERI_MSDC30_2: u32 = 16;
pub const CLK_PERI_MSDC30_3: u32 = 17;
pub const CLK_PERI_MSDC50_3: u32 = 18;
pub const CLK_PERI_NLI: u32 = 19;
pub const CLK_PERI_UART0: u32 = 20;
pub const CLK_PERI_UART1: u32 = 21;
pub const CLK_PERI_UART2: u32 = 22;
pub const CLK_PERI_UART3: u32 = 23;
pub const CLK_PERI_BTIF: u32 = 24;
pub const CLK_PERI_I2C0: u32 = 25;
pub const CLK_PERI_I2C1: u32 = 26;
pub const CLK_PERI_I2C2: u32 = 27;
pub const CLK_PERI_I2C3: u32 = 28;
pub const CLK_PERI_AUXADC: u32 = 29;
pub const CLK_PERI_SPI0: u32 = 30;
pub const CLK_PERI_ETH: u32 = 31;
pub const CLK_PERI_USB0_MCU: u32 = 32;

pub const CLK_PERI_USB1_MCU: u32 = 33;
pub const CLK_PERI_USB_SLV: u32 = 34;
pub const CLK_PERI_GCPU: u32 = 35;
pub const CLK_PERI_NFI_ECC: u32 = 36;
pub const CLK_PERI_NFI_PAD: u32 = 37;
pub const CLK_PERI_FLASH: u32 = 38;
pub const CLK_PERI_HOST89_INT: u32 = 39;
pub const CLK_PERI_HOST89_SPI: u32 = 40;
pub const CLK_PERI_HOST89_DVD: u32 = 41;
pub const CLK_PERI_SPI1: u32 = 42;
pub const CLK_PERI_SPI2: u32 = 43;
pub const CLK_PERI_FCI: u32 = 44;

pub const CLK_PERI_UART0_SEL: u32 = 45;
pub const CLK_PERI_UART1_SEL: u32 = 46;
pub const CLK_PERI_UART2_SEL: u32 = 47;
pub const CLK_PERI_UART3_SEL: u32 = 48;
pub const CLK_PERI_NR: u32 = 49;

/* AUDIO */

pub const CLK_AUD_AFE: u32 = 1;
pub const CLK_AUD_LRCK_DETECT: u32 = 2;
pub const CLK_AUD_I2S: u32 = 3;
pub const CLK_AUD_APLL_TUNER: u32 = 4;
pub const CLK_AUD_HDMI: u32 = 5;
pub const CLK_AUD_SPDF: u32 = 6;
pub const CLK_AUD_SPDF2: u32 = 7;
pub const CLK_AUD_APLL: u32 = 8;
pub const CLK_AUD_TML: u32 = 9;
pub const CLK_AUD_AHB_IDLE_EXT: u32 = 10;
pub const CLK_AUD_AHB_IDLE_INT: u32 = 11;

pub const CLK_AUD_I2SIN1: u32 = 12;
pub const CLK_AUD_I2SIN2: u32 = 13;
pub const CLK_AUD_I2SIN3: u32 = 14;
pub const CLK_AUD_I2SIN4: u32 = 15;
pub const CLK_AUD_I2SIN5: u32 = 16;
pub const CLK_AUD_I2SIN6: u32 = 17;
pub const CLK_AUD_I2SO1: u32 = 18;
pub const CLK_AUD_I2SO2: u32 = 19;
pub const CLK_AUD_I2SO3: u32 = 20;
pub const CLK_AUD_I2SO4: u32 = 21;
pub const CLK_AUD_I2SO5: u32 = 22;
pub const CLK_AUD_I2SO6: u32 = 23;
pub const CLK_AUD_ASRCI1: u32 = 24;
pub const CLK_AUD_ASRCI2: u32 = 25;
pub const CLK_AUD_ASRCO1: u32 = 26;
pub const CLK_AUD_ASRCO2: u32 = 27;
pub const CLK_AUD_ASRC11: u32 = 28;
pub const CLK_AUD_ASRC12: u32 = 29;
pub const CLK_AUD_HDMIRX: u32 = 30;
pub const CLK_AUD_INTDIR: u32 = 31;
pub const CLK_AUD_A1SYS: u32 = 32;
pub const CLK_AUD_A2SYS: u32 = 33;
pub const CLK_AUD_AFE_CONN: u32 = 34;
pub const CLK_AUD_AFE_PCMIF: u32 = 35;
pub const CLK_AUD_AFE_MRGIF: u32 = 36;

pub const CLK_AUD_MMIF_UL1: u32 = 37;
pub const CLK_AUD_MMIF_UL2: u32 = 38;
pub const CLK_AUD_MMIF_UL3: u32 = 39;
pub const CLK_AUD_MMIF_UL4: u32 = 40;
pub const CLK_AUD_MMIF_UL5: u32 = 41;
pub const CLK_AUD_MMIF_UL6: u32 = 42;
pub const CLK_AUD_MMIF_DL1: u32 = 43;
pub const CLK_AUD_MMIF_DL2: u32 = 44;
pub const CLK_AUD_MMIF_DL3: u32 = 45;
pub const CLK_AUD_MMIF_DL4: u32 = 46;
pub const CLK_AUD_MMIF_DL5: u32 = 47;
pub const CLK_AUD_MMIF_DL6: u32 = 48;
pub const CLK_AUD_MMIF_DLMCH: u32 = 49;
pub const CLK_AUD_MMIF_ARB1: u32 = 50;
pub const CLK_AUD_MMIF_AWB1: u32 = 51;
pub const CLK_AUD_MMIF_AWB2: u32 = 52;
pub const CLK_AUD_MMIF_DAI: u32 = 53;

pub const CLK_AUD_DMIC1: u32 = 54;
pub const CLK_AUD_DMIC2: u32 = 55;
pub const CLK_AUD_ASRCI3: u32 = 56;
pub const CLK_AUD_ASRCI4: u32 = 57;
pub const CLK_AUD_ASRCI5: u32 = 58;
pub const CLK_AUD_ASRCI6: u32 = 59;
pub const CLK_AUD_ASRCO3: u32 = 60;
pub const CLK_AUD_ASRCO4: u32 = 61;
pub const CLK_AUD_ASRCO5: u32 = 62;
pub const CLK_AUD_ASRCO6: u32 = 63;
pub const CLK_AUD_MEM_ASRC1: u32 = 64;
pub const CLK_AUD_MEM_ASRC2: u32 = 65;
pub const CLK_AUD_MEM_ASRC3: u32 = 66;
pub const CLK_AUD_MEM_ASRC4: u32 = 67;
pub const CLK_AUD_MEM_ASRC5: u32 = 68;
pub const CLK_AUD_DSD_ENC: u32 = 69;
pub const CLK_AUD_ASRC_BRG: u32 = 70;
pub const CLK_AUD_NR: u32 = 71;

/* MMSYS */

pub const CLK_MM_SMI_COMMON: u32 = 1;
pub const CLK_MM_SMI_LARB0: u32 = 2;
pub const CLK_MM_CMDQ: u32 = 3;
pub const CLK_MM_MUTEX: u32 = 4;
pub const CLK_MM_DISP_COLOR: u32 = 5;
pub const CLK_MM_DISP_BLS: u32 = 6;
pub const CLK_MM_DISP_WDMA: u32 = 7;
pub const CLK_MM_DISP_RDMA: u32 = 8;
pub const CLK_MM_DISP_OVL: u32 = 9;
pub const CLK_MM_MDP_TDSHP: u32 = 10;
pub const CLK_MM_MDP_WROT: u32 = 11;
pub const CLK_MM_MDP_WDMA: u32 = 12;
pub const CLK_MM_MDP_RSZ1: u32 = 13;
pub const CLK_MM_MDP_RSZ0: u32 = 14;
pub const CLK_MM_MDP_RDMA: u32 = 15;
pub const CLK_MM_MDP_BLS_26M: u32 = 16;
pub const CLK_MM_CAM_MDP: u32 = 17;
pub const CLK_MM_FAKE_ENG: u32 = 18;
pub const CLK_MM_MUTEX_32K: u32 = 19;
pub const CLK_MM_DISP_RDMA1: u32 = 20;
pub const CLK_MM_DISP_UFOE: u32 = 21;

pub const CLK_MM_DSI_ENGINE: u32 = 22;
pub const CLK_MM_DSI_DIG: u32 = 23;
pub const CLK_MM_DPI_DIGL: u32 = 24;
pub const CLK_MM_DPI_ENGINE: u32 = 25;
pub const CLK_MM_DPI1_DIGL: u32 = 26;
pub const CLK_MM_DPI1_ENGINE: u32 = 27;
pub const CLK_MM_TVE_OUTPUT: u32 = 28;
pub const CLK_MM_TVE_INPUT: u32 = 29;
pub const CLK_MM_HDMI_PIXEL: u32 = 30;
pub const CLK_MM_HDMI_PLL: u32 = 31;
pub const CLK_MM_HDMI_AUDIO: u32 = 32;
pub const CLK_MM_HDMI_SPDIF: u32 = 33;
pub const CLK_MM_TVE_FMM: u32 = 34;
pub const CLK_MM_NR: u32 = 35;

/* IMGSYS */

pub const CLK_IMG_SMI_COMM: u32 = 1;
pub const CLK_IMG_RESZ: u32 = 2;
pub const CLK_IMG_JPGDEC_SMI: u32 = 3;
pub const CLK_IMG_JPGDEC: u32 = 4;
pub const CLK_IMG_VENC_LT: u32 = 5;
pub const CLK_IMG_VENC: u32 = 6;
pub const CLK_IMG_NR: u32 = 7;

/* VDEC */

pub const CLK_VDEC_CKGEN: u32 = 1;
pub const CLK_VDEC_LARB: u32 = 2;
pub const CLK_VDEC_NR: u32 = 3;

/* HIFSYS */

pub const CLK_HIFSYS_USB0PHY: u32 = 1;
pub const CLK_HIFSYS_USB1PHY: u32 = 2;
pub const CLK_HIFSYS_PCIE0: u32 = 3;
pub const CLK_HIFSYS_PCIE1: u32 = 4;
pub const CLK_HIFSYS_PCIE2: u32 = 5;
pub const CLK_HIFSYS_NR: u32 = 6;

/* ETHSYS */
pub const CLK_ETHSYS_HSDMA: u32 = 1;
pub const CLK_ETHSYS_ESW: u32 = 2;
pub const CLK_ETHSYS_GP2: u32 = 3;
pub const CLK_ETHSYS_GP1: u32 = 4;
pub const CLK_ETHSYS_PCM: u32 = 5;
pub const CLK_ETHSYS_GDMA: u32 = 6;
pub const CLK_ETHSYS_I2S: u32 = 7;
pub const CLK_ETHSYS_CRYPTO: u32 = 8;
pub const CLK_ETHSYS_NR: u32 = 9;

/* G3DSYS */
pub const CLK_G3DSYS_CORE: u32 = 1;
pub const CLK_G3DSYS_NR: u32 = 2;

/* BDP */

pub const CLK_BDP_BRG_BA: u32 = 1;
pub const CLK_BDP_BRG_DRAM: u32 = 2;
pub const CLK_BDP_LARB_DRAM: u32 = 3;
pub const CLK_BDP_WR_VDI_PXL: u32 = 4;
pub const CLK_BDP_WR_VDI_DRAM: u32 = 5;
pub const CLK_BDP_WR_B: u32 = 6;
pub const CLK_BDP_DGI_IN: u32 = 7;
pub const CLK_BDP_DGI_OUT: u32 = 8;
pub const CLK_BDP_FMT_MAST_27: u32 = 9;
pub const CLK_BDP_FMT_B: u32 = 10;
pub const CLK_BDP_OSD_B: u32 = 11;
pub const CLK_BDP_OSD_DRAM: u32 = 12;
pub const CLK_BDP_OSD_AGENT: u32 = 13;
pub const CLK_BDP_OSD_PXL: u32 = 14;
pub const CLK_BDP_RLE_B: u32 = 15;
pub const CLK_BDP_RLE_AGENT: u32 = 16;
pub const CLK_BDP_RLE_DRAM: u32 = 17;
pub const CLK_BDP_F27M: u32 = 18;
pub const CLK_BDP_F27M_VDOUT: u32 = 19;
pub const CLK_BDP_F27_74_74: u32 = 20;
pub const CLK_BDP_F2FS: u32 = 21;
pub const CLK_BDP_F2FS74_148: u32 = 22;
pub const CLK_BDP_FB: u32 = 23;
pub const CLK_BDP_VDO_DRAM: u32 = 24;
pub const CLK_BDP_VDO_2FS: u32 = 25;
pub const CLK_BDP_VDO_B: u32 = 26;
pub const CLK_BDP_WR_DI_PXL: u32 = 27;
pub const CLK_BDP_WR_DI_DRAM: u32 = 28;
pub const CLK_BDP_WR_DI_B: u32 = 29;
pub const CLK_BDP_NR_PXL: u32 = 30;
pub const CLK_BDP_NR_DRAM: u32 = 31;
pub const CLK_BDP_NR_B: u32 = 32;

pub const CLK_BDP_RX_F: u32 = 33;
pub const CLK_BDP_RX_X: u32 = 34;
pub const CLK_BDP_RXPDT: u32 = 35;
pub const CLK_BDP_RX_CSCL_N: u32 = 36;
pub const CLK_BDP_RX_CSCL: u32 = 37;
pub const CLK_BDP_RX_DDCSCL_N: u32 = 38;
pub const CLK_BDP_RX_DDCSCL: u32 = 39;
pub const CLK_BDP_RX_VCO: u32 = 40;
pub const CLK_BDP_RX_DP: u32 = 41;
pub const CLK_BDP_RX_P: u32 = 42;
pub const CLK_BDP_RX_M: u32 = 43;
pub const CLK_BDP_RX_PLL: u32 = 44;
pub const CLK_BDP_BRG_RT_B: u32 = 45;
pub const CLK_BDP_BRG_RT_DRAM: u32 = 46;
pub const CLK_BDP_LARBRT_DRAM: u32 = 47;
pub const CLK_BDP_TMDS_SYN: u32 = 48;
pub const CLK_BDP_HDMI_MON: u32 = 49;
pub const CLK_BDP_NR: u32 = 50;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
