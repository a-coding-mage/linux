/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2015 Rockchip Electronics Co. Ltd.
 * Author: Xing Zheng <zhengxing@rock-chips.com>
 */

/* core clocks */
pub const PLL_APLL: u32 = 1;
pub const PLL_DPLL: u32 = 2;
pub const PLL_GPLL: u32 = 3;
pub const ARMCLK: u32 = 4;

/* sclk gates (special clocks) */
pub const SCLK_GPU: u32 = 64;
pub const SCLK_SPI: u32 = 65;
pub const SCLK_SDMMC: u32 = 68;
pub const SCLK_SDIO: u32 = 69;
pub const SCLK_EMMC: u32 = 71;
pub const SCLK_NANDC: u32 = 76;
pub const SCLK_UART0: u32 = 77;
pub const SCLK_UART1: u32 = 78;
pub const SCLK_UART2: u32 = 79;
pub const SCLK_I2S: u32 = 82;
pub const SCLK_SPDIF: u32 = 83;
pub const SCLK_TIMER0: u32 = 85;
pub const SCLK_TIMER1: u32 = 86;
pub const SCLK_TIMER2: u32 = 87;
pub const SCLK_TIMER3: u32 = 88;
pub const SCLK_OTGPHY0: u32 = 93;
pub const SCLK_LCDC: u32 = 100;
pub const SCLK_HDMI: u32 = 109;
pub const SCLK_HEVC: u32 = 111;
pub const SCLK_I2S_OUT: u32 = 113;
pub const SCLK_SDMMC_DRV: u32 = 114;
pub const SCLK_SDIO_DRV: u32 = 115;
pub const SCLK_EMMC_DRV: u32 = 117;
pub const SCLK_SDMMC_SAMPLE: u32 = 118;
pub const SCLK_SDIO_SAMPLE: u32 = 119;
pub const SCLK_EMMC_SAMPLE: u32 = 121;
pub const SCLK_PVTM_CORE: u32 = 123;
pub const SCLK_PVTM_GPU: u32 = 124;
pub const SCLK_PVTM_VIDEO: u32 = 125;
pub const SCLK_MAC: u32 = 151;
pub const SCLK_MACREF: u32 = 152;
pub const SCLK_MACPLL: u32 = 153;
pub const SCLK_SFC: u32 = 160;
pub const SCLK_USB480M: u32 = 161;

/* aclk gates */
pub const ACLK_DMAC2: u32 = 194;
pub const ACLK_LCDC: u32 = 197;
pub const ACLK_VIO: u32 = 203;
pub const ACLK_VCODEC: u32 = 208;
pub const ACLK_CPU: u32 = 209;
pub const ACLK_PERI: u32 = 210;

/* pclk gates */
pub const PCLK_GPIO0: u32 = 320;
pub const PCLK_GPIO1: u32 = 321;
pub const PCLK_GPIO2: u32 = 322;
pub const PCLK_GRF: u32 = 329;
pub const PCLK_I2C0: u32 = 332;
pub const PCLK_I2C1: u32 = 333;
pub const PCLK_I2C2: u32 = 334;
pub const PCLK_SPI: u32 = 338;
pub const PCLK_UART0: u32 = 341;
pub const PCLK_UART1: u32 = 342;
pub const PCLK_UART2: u32 = 343;
pub const PCLK_PWM: u32 = 350;
pub const PCLK_TIMER: u32 = 353;
pub const PCLK_HDMI: u32 = 360;
pub const PCLK_CPU: u32 = 362;
pub const PCLK_PERI: u32 = 363;
pub const PCLK_DDRUPCTL: u32 = 364;
pub const PCLK_WDT: u32 = 368;
pub const PCLK_ACODEC: u32 = 369;

/* hclk gates */
pub const HCLK_OTG0: u32 = 449;
pub const HCLK_OTG1: u32 = 450;
pub const HCLK_NANDC: u32 = 453;
pub const HCLK_SFC: u32 = 454;
pub const HCLK_SDMMC: u32 = 456;
pub const HCLK_SDIO: u32 = 457;
pub const HCLK_EMMC: u32 = 459;
pub const HCLK_MAC: u32 = 460;
pub const HCLK_I2S: u32 = 462;
pub const HCLK_LCDC: u32 = 465;
pub const HCLK_ROM: u32 = 467;
pub const HCLK_VIO_BUS: u32 = 472;
pub const HCLK_VCODEC: u32 = 476;
pub const HCLK_CPU: u32 = 477;
pub const HCLK_PERI: u32 = 478;

/* soft-reset indices */
pub const SRST_CORE0: u32 = 0;
pub const SRST_CORE1: u32 = 1;
pub const SRST_CORE0_DBG: u32 = 4;
pub const SRST_CORE1_DBG: u32 = 5;
pub const SRST_CORE0_POR: u32 = 8;
pub const SRST_CORE1_POR: u32 = 9;
pub const SRST_L2C: u32 = 12;
pub const SRST_TOPDBG: u32 = 13;
pub const SRST_STRC_SYS_A: u32 = 14;
pub const SRST_PD_CORE_NIU: u32 = 15;
pub const SRST_TIMER2: u32 = 16;
pub const SRST_CPUSYS_H: u32 = 17;
pub const SRST_AHB2APB_H: u32 = 19;
pub const SRST_TIMER3: u32 = 20;
pub const SRST_INTMEM: u32 = 21;
pub const SRST_ROM: u32 = 22;
pub const SRST_PERI_NIU: u32 = 23;
pub const SRST_I2S: u32 = 24;
pub const SRST_DDR_PLL: u32 = 25;
pub const SRST_GPU_DLL: u32 = 26;
pub const SRST_TIMER0: u32 = 27;
pub const SRST_TIMER1: u32 = 28;
pub const SRST_CORE_DLL: u32 = 29;
pub const SRST_EFUSE_P: u32 = 30;
pub const SRST_ACODEC_P: u32 = 31;
pub const SRST_GPIO0: u32 = 32;
pub const SRST_GPIO1: u32 = 33;
pub const SRST_GPIO2: u32 = 34;
pub const SRST_UART0: u32 = 39;
pub const SRST_UART1: u32 = 40;
pub const SRST_UART2: u32 = 41;
pub const SRST_I2C0: u32 = 43;
pub const SRST_I2C1: u32 = 44;
pub const SRST_I2C2: u32 = 45;
pub const SRST_SFC: u32 = 47;
pub const SRST_PWM0: u32 = 48;
pub const SRST_DAP: u32 = 51;
pub const SRST_DAP_SYS: u32 = 52;
pub const SRST_GRF: u32 = 55;
pub const SRST_PERIPHSYS_A: u32 = 57;
pub const SRST_PERIPHSYS_H: u32 = 58;
pub const SRST_PERIPHSYS_P: u32 = 59;
pub const SRST_CPU_PERI: u32 = 61;
pub const SRST_EMEM_PERI: u32 = 62;
pub const SRST_USB_PERI: u32 = 63;
pub const SRST_DMA2: u32 = 64;
pub const SRST_MAC: u32 = 66;
pub const SRST_NANDC: u32 = 68;
pub const SRST_USBOTG0: u32 = 69;
pub const SRST_OTGC0: u32 = 71;
pub const SRST_USBOTG1: u32 = 72;
pub const SRST_OTGC1: u32 = 74;
pub const SRST_DDRMSCH: u32 = 79;
pub const SRST_MMC0: u32 = 81;
pub const SRST_SDIO: u32 = 82;
pub const SRST_EMMC: u32 = 83;
pub const SRST_SPI0: u32 = 84;
pub const SRST_WDT: u32 = 86;
pub const SRST_DDRPHY: u32 = 88;
pub const SRST_DDRPHY_P: u32 = 89;
pub const SRST_DDRCTRL: u32 = 90;
pub const SRST_DDRCTRL_P: u32 = 91;
pub const SRST_HDMI_P: u32 = 96;
pub const SRST_VIO_BUS_H: u32 = 99;
pub const SRST_UTMI0: u32 = 103;
pub const SRST_UTMI1: u32 = 104;
pub const SRST_USBPOR: u32 = 105;
pub const SRST_VCODEC_A: u32 = 112;
pub const SRST_VCODEC_H: u32 = 113;
pub const SRST_VIO1_A: u32 = 114;
pub const SRST_HEVC: u32 = 115;
pub const SRST_VCODEC_NIU_A: u32 = 116;
pub const SRST_LCDC1_A: u32 = 117;
pub const SRST_LCDC1_H: u32 = 118;
pub const SRST_LCDC1_D: u32 = 119;
pub const SRST_GPU: u32 = 120;
pub const SRST_GPU_NIU_A: u32 = 122;
pub const SRST_DBG_P: u32 = 131;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
