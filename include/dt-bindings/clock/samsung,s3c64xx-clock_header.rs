// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2013 Tomasz Figa <tomasz.figa at gmail.com>
//
// Device Tree binding constants for Samsung S3C64xx clock controller.

// Let each exported clock get a unique index, which is used on DT-enabled
// platforms to lookup the clock from a clock specifier. These indices are
// therefore considered an ABI and so must not be changed. This implies
// that new clocks should be added either in free spaces between clock groups
// or at the end.

// Core clocks.
pub const CLK27M: u32 = 1;
pub const CLK48M: u32 = 2;
pub const FOUT_APLL: u32 = 3;
pub const FOUT_MPLL: u32 = 4;
pub const FOUT_EPLL: u32 = 5;
pub const ARMCLK: u32 = 6;
pub const HCLKX2: u32 = 7;
pub const HCLK: u32 = 8;
pub const PCLK: u32 = 9;

// HCLK bus clocks.
pub const HCLK_3DSE: u32 = 16;
pub const HCLK_UHOST: u32 = 17;
pub const HCLK_SECUR: u32 = 18;
pub const HCLK_SDMA1: u32 = 19;
pub const HCLK_SDMA0: u32 = 20;
pub const HCLK_IROM: u32 = 21;
pub const HCLK_DDR1: u32 = 22;
pub const HCLK_MEM1: u32 = 23;
pub const HCLK_MEM0: u32 = 24;
pub const HCLK_USB: u32 = 25;
pub const HCLK_HSMMC2: u32 = 26;
pub const HCLK_HSMMC1: u32 = 27;
pub const HCLK_HSMMC0: u32 = 28;
pub const HCLK_MDP: u32 = 29;
pub const HCLK_DHOST: u32 = 30;
pub const HCLK_IHOST: u32 = 31;
pub const HCLK_DMA1: u32 = 32;
pub const HCLK_DMA0: u32 = 33;
pub const HCLK_JPEG: u32 = 34;
pub const HCLK_CAMIF: u32 = 35;
pub const HCLK_SCALER: u32 = 36;
pub const HCLK_2D: u32 = 37;
pub const HCLK_TV: u32 = 38;
pub const HCLK_POST0: u32 = 39;
pub const HCLK_ROT: u32 = 40;
pub const HCLK_LCD: u32 = 41;
pub const HCLK_TZIC: u32 = 42;
pub const HCLK_INTC: u32 = 43;
pub const HCLK_MFC: u32 = 44;
pub const HCLK_DDR0: u32 = 45;

// PCLK bus clocks.
pub const PCLK_IIC1: u32 = 48;
pub const PCLK_IIS2: u32 = 49;
pub const PCLK_SKEY: u32 = 50;
pub const PCLK_CHIPID: u32 = 51;
pub const PCLK_SPI1: u32 = 52;
pub const PCLK_SPI0: u32 = 53;
pub const PCLK_HSIRX: u32 = 54;
pub const PCLK_HSITX: u32 = 55;
pub const PCLK_GPIO: u32 = 56;
pub const PCLK_IIC0: u32 = 57;
pub const PCLK_IIS1: u32 = 58;
pub const PCLK_IIS0: u32 = 59;
pub const PCLK_AC97: u32 = 60;
pub const PCLK_TZPC: u32 = 61;
pub const PCLK_TSADC: u32 = 62;
pub const PCLK_KEYPAD: u32 = 63;
pub const PCLK_IRDA: u32 = 64;
pub const PCLK_PCM1: u32 = 65;
pub const PCLK_PCM0: u32 = 66;
pub const PCLK_PWM: u32 = 67;
pub const PCLK_RTC: u32 = 68;
pub const PCLK_WDT: u32 = 69;
pub const PCLK_UART3: u32 = 70;
pub const PCLK_UART2: u32 = 71;
pub const PCLK_UART1: u32 = 72;
pub const PCLK_UART0: u32 = 73;
pub const PCLK_MFC: u32 = 74;

// Special clocks.
pub const SCLK_UHOST: u32 = 80;
pub const SCLK_MMC2_48: u32 = 81;
pub const SCLK_MMC1_48: u32 = 82;
pub const SCLK_MMC0_48: u32 = 83;
pub const SCLK_MMC2: u32 = 84;
pub const SCLK_MMC1: u32 = 85;
pub const SCLK_MMC0: u32 = 86;
pub const SCLK_SPI1_48: u32 = 87;
pub const SCLK_SPI0_48: u32 = 88;
pub const SCLK_SPI1: u32 = 89;
pub const SCLK_SPI0: u32 = 90;
pub const SCLK_DAC27: u32 = 91;
pub const SCLK_TV27: u32 = 92;
pub const SCLK_SCALER27: u32 = 93;
pub const SCLK_SCALER: u32 = 94;
pub const SCLK_LCD27: u32 = 95;
pub const SCLK_LCD: u32 = 96;
pub const SCLK_FIMC: u32 = 97;
pub const SCLK_POST0_27: u32 = 98;
pub const SCLK_AUDIO2: u32 = 99;
pub const SCLK_POST0: u32 = 100;
pub const SCLK_AUDIO1: u32 = 101;
pub const SCLK_AUDIO0: u32 = 102;
pub const SCLK_SECUR: u32 = 103;
pub const SCLK_IRDA: u32 = 104;
pub const SCLK_UART: u32 = 105;
pub const SCLK_MFC: u32 = 106;
pub const SCLK_CAM: u32 = 107;
pub const SCLK_JPEG: u32 = 108;
pub const SCLK_ONENAND: u32 = 109;

// MEM0 bus clocks - S3C6410-specific.
pub const MEM0_CFCON: u32 = 112;
pub const MEM0_ONENAND1: u32 = 113;
pub const MEM0_ONENAND0: u32 = 114;
pub const MEM0_NFCON: u32 = 115;
pub const MEM0_SROM: u32 = 116;

// Muxes.
pub const MOUT_APLL: u32 = 128;
pub const MOUT_MPLL: u32 = 129;
pub const MOUT_EPLL: u32 = 130;
pub const MOUT_MFC: u32 = 131;
pub const MOUT_AUDIO0: u32 = 132;
pub const MOUT_AUDIO1: u32 = 133;
pub const MOUT_UART: u32 = 134;
pub const MOUT_SPI0: u32 = 135;
pub const MOUT_SPI1: u32 = 136;
pub const MOUT_MMC0: u32 = 137;
pub const MOUT_MMC1: u32 = 138;
pub const MOUT_MMC2: u32 = 139;
pub const MOUT_UHOST: u32 = 140;
pub const MOUT_IRDA: u32 = 141;
pub const MOUT_LCD: u32 = 142;
pub const MOUT_SCALER: u32 = 143;
pub const MOUT_DAC27: u32 = 144;
pub const MOUT_TV27: u32 = 145;
pub const MOUT_AUDIO2: u32 = 146;

// Dividers.
pub const DOUT_MPLL: u32 = 160;
pub const DOUT_SECUR: u32 = 161;
pub const DOUT_CAM: u32 = 162;
pub const DOUT_JPEG: u32 = 163;
pub const DOUT_MFC: u32 = 164;
pub const DOUT_MMC0: u32 = 165;
pub const DOUT_MMC1: u32 = 166;
pub const DOUT_MMC2: u32 = 167;
pub const DOUT_LCD: u32 = 168;
pub const DOUT_SCALER: u32 = 169;
pub const DOUT_UHOST: u32 = 170;
pub const DOUT_SPI0: u32 = 171;
pub const DOUT_SPI1: u32 = 172;
pub const DOUT_AUDIO0: u32 = 173;
pub const DOUT_AUDIO1: u32 = 174;
pub const DOUT_UART: u32 = 175;
pub const DOUT_IRDA: u32 = 176;
pub const DOUT_FIMC: u32 = 177;
pub const DOUT_AUDIO2: u32 = 178;

// Total number of clocks.
pub const NR_CLKS: u32 = DOUT_AUDIO2 + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
