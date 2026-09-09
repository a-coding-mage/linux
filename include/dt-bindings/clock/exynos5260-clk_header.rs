/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Rahul Sharma <rahul.sharma@samsung.com>
 *
 * Provides Constants for Exynos5260 clocks.
 */


/* Clock names: <cmu><type><IP> */

/* List Of Clocks For CMU_TOP */

pub const TOP_FOUT_DISP_PLL: u32 = 1;
pub const TOP_FOUT_AUD_PLL: u32 = 2;
pub const TOP_MOUT_AUDTOP_PLL_USER: u32 = 3;
pub const TOP_MOUT_AUD_PLL: u32 = 4;
pub const TOP_MOUT_DISP_PLL: u32 = 5;
pub const TOP_MOUT_BUSTOP_PLL_USER: u32 = 6;
pub const TOP_MOUT_MEMTOP_PLL_USER: u32 = 7;
pub const TOP_MOUT_MEDIATOP_PLL_USER: u32 = 8;
pub const TOP_MOUT_DISP_DISP_333: u32 = 9;
pub const TOP_MOUT_ACLK_DISP_333: u32 = 10;
pub const TOP_MOUT_DISP_DISP_222: u32 = 11;
pub const TOP_MOUT_ACLK_DISP_222: u32 = 12;
pub const TOP_MOUT_DISP_MEDIA_PIXEL: u32 = 13;
pub const TOP_MOUT_FIMD1: u32 = 14;
pub const TOP_MOUT_SCLK_PERI_SPI0_CLK: u32 = 15;
pub const TOP_MOUT_SCLK_PERI_SPI1_CLK: u32 = 16;
pub const TOP_MOUT_SCLK_PERI_SPI2_CLK: u32 = 17;
pub const TOP_MOUT_SCLK_PERI_UART0_UCLK: u32 = 18;
pub const TOP_MOUT_SCLK_PERI_UART2_UCLK: u32 = 19;
pub const TOP_MOUT_SCLK_PERI_UART1_UCLK: u32 = 20;
pub const TOP_MOUT_BUS4_BUSTOP_100: u32 = 21;
pub const TOP_MOUT_BUS4_BUSTOP_400: u32 = 22;
pub const TOP_MOUT_BUS3_BUSTOP_100: u32 = 23;
pub const TOP_MOUT_BUS3_BUSTOP_400: u32 = 24;
pub const TOP_MOUT_BUS2_BUSTOP_400: u32 = 25;
pub const TOP_MOUT_BUS2_BUSTOP_100: u32 = 26;
pub const TOP_MOUT_BUS1_BUSTOP_100: u32 = 27;
pub const TOP_MOUT_BUS1_BUSTOP_400: u32 = 28;
pub const TOP_MOUT_SCLK_FSYS_USB: u32 = 29;
pub const TOP_MOUT_SCLK_FSYS_MMC0_SDCLKIN_A: u32 = 30;
pub const TOP_MOUT_SCLK_FSYS_MMC1_SDCLKIN_A: u32 = 31;
pub const TOP_MOUT_SCLK_FSYS_MMC2_SDCLKIN_A: u32 = 32;
pub const TOP_MOUT_SCLK_FSYS_MMC0_SDCLKIN_B: u32 = 33;
pub const TOP_MOUT_SCLK_FSYS_MMC1_SDCLKIN_B: u32 = 34;
pub const TOP_MOUT_SCLK_FSYS_MMC2_SDCLKIN_B: u32 = 35;
pub const TOP_MOUT_ACLK_ISP1_266: u32 = 36;
pub const TOP_MOUT_ISP1_MEDIA_266: u32 = 37;
pub const TOP_MOUT_ACLK_ISP1_400: u32 = 38;
pub const TOP_MOUT_ISP1_MEDIA_400: u32 = 39;
pub const TOP_MOUT_SCLK_ISP1_SPI0: u32 = 40;
pub const TOP_MOUT_SCLK_ISP1_SPI1: u32 = 41;
pub const TOP_MOUT_SCLK_ISP1_UART: u32 = 42;
pub const TOP_MOUT_SCLK_ISP1_SENSOR2: u32 = 43;
pub const TOP_MOUT_SCLK_ISP1_SENSOR1: u32 = 44;
pub const TOP_MOUT_SCLK_ISP1_SENSOR0: u32 = 45;
pub const TOP_MOUT_ACLK_MFC_333: u32 = 46;
pub const TOP_MOUT_MFC_BUSTOP_333: u32 = 47;
pub const TOP_MOUT_ACLK_G2D_333: u32 = 48;
pub const TOP_MOUT_G2D_BUSTOP_333: u32 = 49;
pub const TOP_MOUT_ACLK_GSCL_FIMC: u32 = 50;
pub const TOP_MOUT_GSCL_BUSTOP_FIMC: u32 = 51;
pub const TOP_MOUT_ACLK_GSCL_333: u32 = 52;
pub const TOP_MOUT_GSCL_BUSTOP_333: u32 = 53;
pub const TOP_MOUT_ACLK_GSCL_400: u32 = 54;
pub const TOP_MOUT_M2M_MEDIATOP_400: u32 = 55;
pub const TOP_DOUT_ACLK_MFC_333: u32 = 56;
pub const TOP_DOUT_ACLK_G2D_333: u32 = 57;
pub const TOP_DOUT_SCLK_ISP1_SENSOR2_A: u32 = 58;
pub const TOP_DOUT_SCLK_ISP1_SENSOR1_A: u32 = 59;
pub const TOP_DOUT_SCLK_ISP1_SENSOR0_A: u32 = 60;
pub const TOP_DOUT_ACLK_GSCL_FIMC: u32 = 61;
pub const TOP_DOUT_ACLK_GSCL_400: u32 = 62;
pub const TOP_DOUT_ACLK_GSCL_333: u32 = 63;
pub const TOP_DOUT_SCLK_ISP1_SPI0_B: u32 = 64;
pub const TOP_DOUT_SCLK_ISP1_SPI0_A: u32 = 65;
pub const TOP_DOUT_ACLK_ISP1_400: u32 = 66;
pub const TOP_DOUT_ACLK_ISP1_266: u32 = 67;
pub const TOP_DOUT_SCLK_ISP1_UART: u32 = 68;
pub const TOP_DOUT_SCLK_ISP1_SPI1_B: u32 = 69;
pub const TOP_DOUT_SCLK_ISP1_SPI1_A: u32 = 70;
pub const TOP_DOUT_SCLK_ISP1_SENSOR2_B: u32 = 71;
pub const TOP_DOUT_SCLK_ISP1_SENSOR1_B: u32 = 72;
pub const TOP_DOUT_SCLK_ISP1_SENSOR0_B: u32 = 73;
pub const TOP_DOUTTOP__SCLK_HPM_TARGETCLK: u32 = 74;
pub const TOP_DOUT_SCLK_DISP_PIXEL: u32 = 75;
pub const TOP_DOUT_ACLK_DISP_222: u32 = 76;
pub const TOP_DOUT_ACLK_DISP_333: u32 = 77;
pub const TOP_DOUT_ACLK_BUS4_100: u32 = 78;
pub const TOP_DOUT_ACLK_BUS4_400: u32 = 79;
pub const TOP_DOUT_ACLK_BUS3_100: u32 = 80;
pub const TOP_DOUT_ACLK_BUS3_400: u32 = 81;
pub const TOP_DOUT_ACLK_BUS2_100: u32 = 82;
pub const TOP_DOUT_ACLK_BUS2_400: u32 = 83;
pub const TOP_DOUT_ACLK_BUS1_100: u32 = 84;
pub const TOP_DOUT_ACLK_BUS1_400: u32 = 85;
pub const TOP_DOUT_SCLK_PERI_SPI1_B: u32 = 86;
pub const TOP_DOUT_SCLK_PERI_SPI1_A: u32 = 87;
pub const TOP_DOUT_SCLK_PERI_SPI0_B: u32 = 88;
pub const TOP_DOUT_SCLK_PERI_SPI0_A: u32 = 89;
pub const TOP_DOUT_SCLK_PERI_UART0: u32 = 90;
pub const TOP_DOUT_SCLK_PERI_UART2: u32 = 91;
pub const TOP_DOUT_SCLK_PERI_UART1: u32 = 92;
pub const TOP_DOUT_SCLK_PERI_SPI2_B: u32 = 93;
pub const TOP_DOUT_SCLK_PERI_SPI2_A: u32 = 94;
pub const TOP_DOUT_ACLK_PERI_AUD: u32 = 95;
pub const TOP_DOUT_ACLK_PERI_66: u32 = 96;
pub const TOP_DOUT_SCLK_FSYS_MMC0_SDCLKIN_B: u32 = 97;
pub const TOP_DOUT_SCLK_FSYS_MMC0_SDCLKIN_A: u32 = 98;
pub const TOP_DOUT_SCLK_FSYS_USBDRD30_SUSPEND_CLK: u32 = 99;
pub const TOP_DOUT_ACLK_FSYS_200: u32 = 100;
pub const TOP_DOUT_SCLK_FSYS_MMC2_SDCLKIN_B: u32 = 101;
pub const TOP_DOUT_SCLK_FSYS_MMC2_SDCLKIN_A: u32 = 102;
pub const TOP_DOUT_SCLK_FSYS_MMC1_SDCLKIN_B: u32 = 103;
pub const TOP_DOUT_SCLK_FSYS_MMC1_SDCLKIN_A: u32 = 104;
pub const TOP_SCLK_FIMD1: u32 = 105;
pub const TOP_SCLK_MMC2: u32 = 106;
pub const TOP_SCLK_MMC1: u32 = 107;
pub const TOP_SCLK_MMC0: u32 = 108;
pub const PHYCLK_DPTX_PHY_CH3_TXD_CLK: u32 = 109;
pub const PHYCLK_DPTX_PHY_CH2_TXD_CLK: u32 = 110;
pub const PHYCLK_DPTX_PHY_CH1_TXD_CLK: u32 = 111;
pub const PHYCLK_DPTX_PHY_CH0_TXD_CLK: u32 = 112;
pub const phyclk_hdmi_phy_tmds_clko: u32 = 113;
pub const PHYCLK_HDMI_PHY_PIXEL_CLKO: u32 = 114;
pub const PHYCLK_HDMI_LINK_O_TMDS_CLKHI: u32 = 115;
pub const PHYCLK_MIPI_DPHY_4L_M_TXBYTECLKHS: u32 = 116;
pub const PHYCLK_DPTX_PHY_O_REF_CLK_24M: u32 = 117;
pub const PHYCLK_DPTX_PHY_CLK_DIV2: u32 = 118;
pub const PHYCLK_MIPI_DPHY_4L_M_RXCLKESC0: u32 = 119;
pub const PHYCLK_USBHOST20_PHY_PHYCLOCK: u32 = 120;
pub const PHYCLK_USBHOST20_PHY_FREECLK: u32 = 121;
pub const PHYCLK_USBHOST20_PHY_CLK48MOHCI: u32 = 122;
pub const PHYCLK_USBDRD30_UDRD30_PIPE_PCLK: u32 = 123;
pub const PHYCLK_USBDRD30_UDRD30_PHYCLOCK: u32 = 124;

/* List Of Clocks For CMU_EGL */

pub const EGL_FOUT_EGL_PLL: u32 = 1;
pub const EGL_FOUT_EGL_DPLL: u32 = 2;
pub const EGL_MOUT_EGL_B: u32 = 3;
pub const EGL_MOUT_EGL_PLL: u32 = 4;
pub const EGL_DOUT_EGL_PLL: u32 = 5;
pub const EGL_DOUT_EGL_PCLK_DBG: u32 = 6;
pub const EGL_DOUT_EGL_ATCLK: u32 = 7;
pub const EGL_DOUT_PCLK_EGL: u32 = 8;
pub const EGL_DOUT_ACLK_EGL: u32 = 9;
pub const EGL_DOUT_EGL2: u32 = 10;
pub const EGL_DOUT_EGL1: u32 = 11;

/* List Of Clocks For CMU_KFC */

pub const KFC_FOUT_KFC_PLL: u32 = 1;
pub const KFC_MOUT_KFC_PLL: u32 = 2;
pub const KFC_MOUT_KFC: u32 = 3;
pub const KFC_DOUT_KFC_PLL: u32 = 4;
pub const KFC_DOUT_PCLK_KFC: u32 = 5;
pub const KFC_DOUT_ACLK_KFC: u32 = 6;
pub const KFC_DOUT_KFC_PCLK_DBG: u32 = 7;
pub const KFC_DOUT_KFC_ATCLK: u32 = 8;
pub const KFC_DOUT_KFC2: u32 = 9;
pub const KFC_DOUT_KFC1: u32 = 10;

/* List Of Clocks For CMU_MIF */

pub const MIF_FOUT_MEM_PLL: u32 = 1;
pub const MIF_FOUT_MEDIA_PLL: u32 = 2;
pub const MIF_FOUT_BUS_PLL: u32 = 3;
pub const MIF_MOUT_CLK2X_PHY: u32 = 4;
pub const MIF_MOUT_MIF_DREX2X: u32 = 5;
pub const MIF_MOUT_CLKM_PHY: u32 = 6;
pub const MIF_MOUT_MIF_DREX: u32 = 7;
pub const MIF_MOUT_MEDIA_PLL: u32 = 8;
pub const MIF_MOUT_BUS_PLL: u32 = 9;
pub const MIF_MOUT_MEM_PLL: u32 = 10;
pub const MIF_DOUT_ACLK_BUS_100: u32 = 11;
pub const MIF_DOUT_ACLK_BUS_200: u32 = 12;
pub const MIF_DOUT_ACLK_MIF_466: u32 = 13;
pub const MIF_DOUT_CLK2X_PHY: u32 = 14;
pub const MIF_DOUT_CLKM_PHY: u32 = 15;
pub const MIF_DOUT_BUS_PLL: u32 = 16;
pub const MIF_DOUT_MEM_PLL: u32 = 17;
pub const MIF_DOUT_MEDIA_PLL: u32 = 18;
pub const MIF_CLK_LPDDR3PHY_WRAP1: u32 = 19;
pub const MIF_CLK_LPDDR3PHY_WRAP0: u32 = 20;
pub const MIF_CLK_MONOCNT: u32 = 21;
pub const MIF_CLK_MIF_RTC: u32 = 22;
pub const MIF_CLK_DREX1: u32 = 23;
pub const MIF_CLK_DREX0: u32 = 24;
pub const MIF_CLK_INTMEM: u32 = 25;
pub const MIF_SCLK_LPDDR3PHY_WRAP_U1: u32 = 26;
pub const MIF_SCLK_LPDDR3PHY_WRAP_U0: u32 = 27;

/* List Of Clocks For CMU_G3D */

pub const G3D_FOUT_G3D_PLL: u32 = 1;
pub const G3D_MOUT_G3D_PLL: u32 = 2;
pub const G3D_DOUT_PCLK_G3D: u32 = 3;
pub const G3D_DOUT_ACLK_G3D: u32 = 4;
pub const G3D_CLK_G3D_HPM: u32 = 5;
pub const G3D_CLK_G3D: u32 = 6;

/* List Of Clocks For CMU_AUD */

pub const AUD_MOUT_SCLK_AUD_PCM: u32 = 1;
pub const AUD_MOUT_SCLK_AUD_I2S: u32 = 2;
pub const AUD_MOUT_AUD_PLL_USER: u32 = 3;
pub const AUD_DOUT_ACLK_AUD_131: u32 = 4;
pub const AUD_DOUT_SCLK_AUD_UART: u32 = 5;
pub const AUD_DOUT_SCLK_AUD_PCM: u32 = 6;
pub const AUD_DOUT_SCLK_AUD_I2S: u32 = 7;
pub const AUD_CLK_AUD_UART: u32 = 8;
pub const AUD_CLK_PCM: u32 = 9;
pub const AUD_CLK_I2S: u32 = 10;
pub const AUD_CLK_DMAC: u32 = 11;
pub const AUD_CLK_SRAMC: u32 = 12;
pub const AUD_SCLK_AUD_UART: u32 = 13;
pub const AUD_SCLK_PCM: u32 = 14;
pub const AUD_SCLK_I2S: u32 = 15;

/* List Of Clocks For CMU_MFC */

pub const MFC_MOUT_ACLK_MFC_333_USER: u32 = 1;
pub const MFC_DOUT_PCLK_MFC_83: u32 = 2;
pub const MFC_CLK_MFC: u32 = 3;
pub const MFC_CLK_SMMU2_MFCM1: u32 = 4;
pub const MFC_CLK_SMMU2_MFCM0: u32 = 5;

/* List Of Clocks For CMU_GSCL */

pub const GSCL_MOUT_ACLK_CSIS: u32 = 1;
pub const GSCL_MOUT_ACLK_GSCL_FIMC_USER: u32 = 2;
pub const GSCL_MOUT_ACLK_M2M_400_USER: u32 = 3;
pub const GSCL_MOUT_ACLK_GSCL_333_USER: u32 = 4;
pub const GSCL_DOUT_ACLK_CSIS_200: u32 = 5;
pub const GSCL_DOUT_PCLK_M2M_100: u32 = 6;
pub const GSCL_CLK_PIXEL_GSCL1: u32 = 7;
pub const GSCL_CLK_PIXEL_GSCL0: u32 = 8;
pub const GSCL_CLK_MSCL1: u32 = 9;
pub const GSCL_CLK_MSCL0: u32 = 10;
pub const GSCL_CLK_GSCL1: u32 = 11;
pub const GSCL_CLK_GSCL0: u32 = 12;
pub const GSCL_CLK_FIMC_LITE_D: u32 = 13;
pub const GSCL_CLK_FIMC_LITE_B: u32 = 14;
pub const GSCL_CLK_FIMC_LITE_A: u32 = 15;
pub const GSCL_CLK_CSIS1: u32 = 16;
pub const GSCL_CLK_CSIS0: u32 = 17;
pub const GSCL_CLK_SMMU3_LITE_D: u32 = 18;
pub const GSCL_CLK_SMMU3_LITE_B: u32 = 19;
pub const GSCL_CLK_SMMU3_LITE_A: u32 = 20;
pub const GSCL_CLK_SMMU3_GSCL0: u32 = 21;
pub const GSCL_CLK_SMMU3_GSCL1: u32 = 22;
pub const GSCL_CLK_SMMU3_MSCL0: u32 = 23;
pub const GSCL_CLK_SMMU3_MSCL1: u32 = 24;
pub const GSCL_SCLK_CSIS1_WRAP: u32 = 25;
pub const GSCL_SCLK_CSIS0_WRAP: u32 = 26;

/* List Of Clocks For CMU_FSYS */

pub const FSYS_MOUT_PHYCLK_USBHOST20_PHYCLK_USER: u32 = 1;
pub const FSYS_MOUT_PHYCLK_USBHOST20_FREECLK_USER: u32 = 2;
pub const FSYS_MOUT_PHYCLK_USBHOST20_CLK48MOHCI_USER: u32 = 3;
pub const FSYS_MOUT_PHYCLK_USBDRD30_PIPE_PCLK_USER: u32 = 4;
pub const FSYS_MOUT_PHYCLK_USBDRD30_PHYCLOCK_USER: u32 = 5;
pub const FSYS_CLK_TSI: u32 = 6;
pub const FSYS_CLK_USBLINK: u32 = 7;
pub const FSYS_CLK_USBHOST20: u32 = 8;
pub const FSYS_CLK_USBDRD30: u32 = 9;
pub const FSYS_CLK_SROMC: u32 = 10;
pub const FSYS_CLK_PDMA: u32 = 11;
pub const FSYS_CLK_MMC2: u32 = 12;
pub const FSYS_CLK_MMC1: u32 = 13;
pub const FSYS_CLK_MMC0: u32 = 14;
pub const FSYS_CLK_RTIC: u32 = 15;
pub const FSYS_CLK_SMMU_RTIC: u32 = 16;
pub const FSYS_PHYCLK_USBDRD30: u32 = 17;
pub const FSYS_PHYCLK_USBHOST20: u32 = 18;

/* List Of Clocks For CMU_PERI */

pub const PERI_MOUT_SCLK_SPDIF: u32 = 1;
pub const PERI_MOUT_SCLK_I2SCOD: u32 = 2;
pub const PERI_MOUT_SCLK_PCM: u32 = 3;
pub const PERI_DOUT_I2S: u32 = 4;
pub const PERI_DOUT_PCM: u32 = 5;
pub const PERI_CLK_WDT_KFC: u32 = 6;
pub const PERI_CLK_WDT_EGL: u32 = 7;
pub const PERI_CLK_HSIC3: u32 = 8;
pub const PERI_CLK_HSIC2: u32 = 9;
pub const PERI_CLK_HSIC1: u32 = 10;
pub const PERI_CLK_HSIC0: u32 = 11;
pub const PERI_CLK_PCM: u32 = 12;
pub const PERI_CLK_MCT: u32 = 13;
pub const PERI_CLK_I2S: u32 = 14;
pub const PERI_CLK_I2CHDMI: u32 = 15;
pub const PERI_CLK_I2C7: u32 = 16;
pub const PERI_CLK_I2C6: u32 = 17;
pub const PERI_CLK_I2C5: u32 = 18;
pub const PERI_CLK_I2C4: u32 = 19;
pub const PERI_CLK_I2C9: u32 = 20;
pub const PERI_CLK_I2C8: u32 = 21;
pub const PERI_CLK_I2C11: u32 = 22;
pub const PERI_CLK_I2C10: u32 = 23;
pub const PERI_CLK_HDMICEC: u32 = 24;
pub const PERI_CLK_EFUSE_WRITER: u32 = 25;
pub const PERI_CLK_ABB: u32 = 26;
pub const PERI_CLK_UART2: u32 = 27;
pub const PERI_CLK_UART1: u32 = 28;
pub const PERI_CLK_UART0: u32 = 29;
pub const PERI_CLK_ADC: u32 = 30;
pub const PERI_CLK_TMU4: u32 = 31;
pub const PERI_CLK_TMU3: u32 = 32;
pub const PERI_CLK_TMU2: u32 = 33;
pub const PERI_CLK_TMU1: u32 = 34;
pub const PERI_CLK_TMU0: u32 = 35;
pub const PERI_CLK_SPI2: u32 = 36;
pub const PERI_CLK_SPI1: u32 = 37;
pub const PERI_CLK_SPI0: u32 = 38;
pub const PERI_CLK_SPDIF: u32 = 39;
pub const PERI_CLK_PWM: u32 = 40;
pub const PERI_CLK_UART4: u32 = 41;
pub const PERI_CLK_CHIPID: u32 = 42;
pub const PERI_CLK_PROVKEY0: u32 = 43;
pub const PERI_CLK_PROVKEY1: u32 = 44;
pub const PERI_CLK_SECKEY: u32 = 45;
pub const PERI_CLK_TOP_RTC: u32 = 46;
pub const PERI_CLK_TZPC10: u32 = 47;
pub const PERI_CLK_TZPC9: u32 = 48;
pub const PERI_CLK_TZPC8: u32 = 49;
pub const PERI_CLK_TZPC7: u32 = 50;
pub const PERI_CLK_TZPC6: u32 = 51;
pub const PERI_CLK_TZPC5: u32 = 52;
pub const PERI_CLK_TZPC4: u32 = 53;
pub const PERI_CLK_TZPC3: u32 = 54;
pub const PERI_CLK_TZPC2: u32 = 55;
pub const PERI_CLK_TZPC1: u32 = 56;
pub const PERI_CLK_TZPC0: u32 = 57;
pub const PERI_SCLK_UART2: u32 = 58;
pub const PERI_SCLK_UART1: u32 = 59;
pub const PERI_SCLK_UART0: u32 = 60;
pub const PERI_SCLK_SPI2: u32 = 61;
pub const PERI_SCLK_SPI1: u32 = 62;
pub const PERI_SCLK_SPI0: u32 = 63;
pub const PERI_SCLK_SPDIF: u32 = 64;
pub const PERI_SCLK_I2S: u32 = 65;
pub const PERI_SCLK_PCM1: u32 = 66;

/* List Of Clocks For CMU_DISP */

pub const DISP_MOUT_SCLK_HDMI_SPDIF: u32 = 1;
pub const DISP_MOUT_SCLK_HDMI_PIXEL: u32 = 2;
pub const DISP_MOUT_PHYCLK_MIPI_DPHY_4LMRXCLK_ESC0_USER: u32 = 3;
pub const DISP_MOUT_PHYCLK_HDMI_PHY_TMDS_CLKO_USER: u32 = 4;
pub const DISP_MOUT_PHYCLK_HDMI_PHY_REF_CLKO_USER: u32 = 5;
pub const DISP_MOUT_HDMI_PHY_PIXEL: u32 = 6;
pub const DISP_MOUT_PHYCLK_HDMI_LINK_O_TMDS_CLKHI_USER: u32 = 7;
pub const DISP_MOUT_PHYCLK_MIPI_DPHY_4L_M_TXBYTE_CLKHS: u32 = 8;
pub const DISP_MOUT_PHYCLK_DPTX_PHY_O_REF_CLK_24M_USER: u32 = 9;
pub const DISP_MOUT_PHYCLK_DPTX_PHY_CLK_DIV2_USER: u32 = 10;
pub const DISP_MOUT_PHYCLK_DPTX_PHY_CH3_TXD_CLK_USER: u32 = 11;
pub const DISP_MOUT_PHYCLK_DPTX_PHY_CH2_TXD_CLK_USER: u32 = 12;
pub const DISP_MOUT_PHYCLK_DPTX_PHY_CH1_TXD_CLK_USER: u32 = 13;
pub const DISP_MOUT_PHYCLK_DPTX_PHY_CH0_TXD_CLK_USER: u32 = 14;
pub const DISP_MOUT_ACLK_DISP_222_USER: u32 = 15;
pub const DISP_MOUT_SCLK_DISP_PIXEL_USER: u32 = 16;
pub const DISP_MOUT_ACLK_DISP_333_USER: u32 = 17;
pub const DISP_DOUT_SCLK_HDMI_PHY_PIXEL_CLKI: u32 = 18;
pub const DISP_DOUT_SCLK_FIMD1_EXTCLKPLL: u32 = 19;
pub const DISP_DOUT_PCLK_DISP_111: u32 = 20;
pub const DISP_CLK_SMMU_TV: u32 = 21;
pub const DISP_CLK_SMMU_FIMD1M1: u32 = 22;
pub const DISP_CLK_SMMU_FIMD1M0: u32 = 23;
pub const DISP_CLK_PIXEL_MIXER: u32 = 24;
pub const DISP_CLK_PIXEL_DISP: u32 = 25;
pub const DISP_CLK_MIXER: u32 = 26;
pub const DISP_CLK_MIPIPHY: u32 = 27;
pub const DISP_CLK_HDMIPHY: u32 = 28;
pub const DISP_CLK_HDMI: u32 = 29;
pub const DISP_CLK_FIMD1: u32 = 30;
pub const DISP_CLK_DSIM1: u32 = 31;
pub const DISP_CLK_DPPHY: u32 = 32;
pub const DISP_CLK_DP: u32 = 33;
pub const DISP_SCLK_PIXEL: u32 = 34;
pub const DISP_MOUT_HDMI_PHY_PIXEL_USER: u32 = 35;

/* List Of Clocks For CMU_G2D */

pub const G2D_MOUT_ACLK_G2D_333_USER: u32 = 1;
pub const G2D_DOUT_PCLK_G2D_83: u32 = 2;
pub const G2D_CLK_SMMU3_JPEG: u32 = 3;
pub const G2D_CLK_MDMA: u32 = 4;
pub const G2D_CLK_JPEG: u32 = 5;
pub const G2D_CLK_G2D: u32 = 6;
pub const G2D_CLK_SSS: u32 = 7;
pub const G2D_CLK_SLIM_SSS: u32 = 8;
pub const G2D_CLK_SMMU_SLIM_SSS: u32 = 9;
pub const G2D_CLK_SMMU_SSS: u32 = 10;
pub const G2D_CLK_SMMU_MDMA: u32 = 11;
pub const G2D_CLK_SMMU3_G2D: u32 = 12;

/* List Of Clocks For CMU_ISP */

pub const ISP_MOUT_ISP_400_USER: u32 = 1;
pub const ISP_MOUT_ISP_266_USER: u32 = 2;
pub const ISP_DOUT_SCLK_MPWM: u32 = 3;
pub const ISP_DOUT_CA5_PCLKDBG: u32 = 4;
pub const ISP_DOUT_CA5_ATCLKIN: u32 = 5;
pub const ISP_DOUT_PCLK_ISP_133: u32 = 6;
pub const ISP_DOUT_PCLK_ISP_66: u32 = 7;
pub const ISP_CLK_GIC: u32 = 8;
pub const ISP_CLK_WDT: u32 = 9;
pub const ISP_CLK_UART: u32 = 10;
pub const ISP_CLK_SPI1: u32 = 11;
pub const ISP_CLK_SPI0: u32 = 12;
pub const ISP_CLK_SMMU_SCALERP: u32 = 13;
pub const ISP_CLK_SMMU_SCALERC: u32 = 14;
pub const ISP_CLK_SMMU_ISPCX: u32 = 15;
pub const ISP_CLK_SMMU_ISP: u32 = 16;
pub const ISP_CLK_SMMU_FD: u32 = 17;
pub const ISP_CLK_SMMU_DRC: u32 = 18;
pub const ISP_CLK_PWM: u32 = 19;
pub const ISP_CLK_MTCADC: u32 = 20;
pub const ISP_CLK_MPWM: u32 = 21;
pub const ISP_CLK_MCUCTL: u32 = 22;
pub const ISP_CLK_I2C1: u32 = 23;
pub const ISP_CLK_I2C0: u32 = 24;
pub const ISP_CLK_FIMC_SCALERP: u32 = 25;
pub const ISP_CLK_FIMC_SCALERC: u32 = 26;
pub const ISP_CLK_FIMC: u32 = 27;
pub const ISP_CLK_FIMC_FD: u32 = 28;
pub const ISP_CLK_FIMC_DRC: u32 = 29;
pub const ISP_CLK_CA5: u32 = 30;
pub const ISP_SCLK_SPI0_EXT: u32 = 31;
pub const ISP_SCLK_SPI1_EXT: u32 = 32;
pub const ISP_SCLK_UART_EXT: u32 = 33;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
