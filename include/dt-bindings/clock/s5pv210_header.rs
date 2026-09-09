/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Author: Mateusz Krawczuk <m.krawczuk@partner.samsung.com>
 *
 * Device Tree binding constants for Samsung S5PV210 clock controller.
 */

/* Core clocks. */
pub const FIN_PLL: u32 = 1;
pub const FOUT_APLL: u32 = 2;
pub const FOUT_MPLL: u32 = 3;
pub const FOUT_EPLL: u32 = 4;
pub const FOUT_VPLL: u32 = 5;

/* Muxes. */
pub const MOUT_FLASH: u32 = 6;
pub const MOUT_PSYS: u32 = 7;
pub const MOUT_DSYS: u32 = 8;
pub const MOUT_MSYS: u32 = 9;
pub const MOUT_VPLL: u32 = 10;
pub const MOUT_EPLL: u32 = 11;
pub const MOUT_MPLL: u32 = 12;
pub const MOUT_APLL: u32 = 13;
pub const MOUT_VPLLSRC: u32 = 14;
pub const MOUT_CSIS: u32 = 15;
pub const MOUT_FIMD: u32 = 16;
pub const MOUT_CAM1: u32 = 17;
pub const MOUT_CAM0: u32 = 18;
pub const MOUT_DAC: u32 = 19;
pub const MOUT_MIXER: u32 = 20;
pub const MOUT_HDMI: u32 = 21;
pub const MOUT_G2D: u32 = 22;
pub const MOUT_MFC: u32 = 23;
pub const MOUT_G3D: u32 = 24;
pub const MOUT_FIMC2: u32 = 25;
pub const MOUT_FIMC1: u32 = 26;
pub const MOUT_FIMC0: u32 = 27;
pub const MOUT_UART3: u32 = 28;
pub const MOUT_UART2: u32 = 29;
pub const MOUT_UART1: u32 = 30;
pub const MOUT_UART0: u32 = 31;
pub const MOUT_MMC3: u32 = 32;
pub const MOUT_MMC2: u32 = 33;
pub const MOUT_MMC1: u32 = 34;
pub const MOUT_MMC0: u32 = 35;
pub const MOUT_PWM: u32 = 36;
pub const MOUT_SPI0: u32 = 37;
pub const MOUT_SPI1: u32 = 38;
pub const MOUT_DMC0: u32 = 39;
pub const MOUT_PWI: u32 = 40;
pub const MOUT_HPM: u32 = 41;
pub const MOUT_SPDIF: u32 = 42;
pub const MOUT_AUDIO2: u32 = 43;
pub const MOUT_AUDIO1: u32 = 44;
pub const MOUT_AUDIO0: u32 = 45;

/* Dividers. */
pub const DOUT_PCLKP: u32 = 46;
pub const DOUT_HCLKP: u32 = 47;
pub const DOUT_PCLKD: u32 = 48;
pub const DOUT_HCLKD: u32 = 49;
pub const DOUT_PCLKM: u32 = 50;
pub const DOUT_HCLKM: u32 = 51;
pub const DOUT_A2M: u32 = 52;
pub const DOUT_APLL: u32 = 53;
pub const DOUT_CSIS: u32 = 54;
pub const DOUT_FIMD: u32 = 55;
pub const DOUT_CAM1: u32 = 56;
pub const DOUT_CAM0: u32 = 57;
pub const DOUT_TBLK: u32 = 58;
pub const DOUT_G2D: u32 = 59;
pub const DOUT_MFC: u32 = 60;
pub const DOUT_G3D: u32 = 61;
pub const DOUT_FIMC2: u32 = 62;
pub const DOUT_FIMC1: u32 = 63;
pub const DOUT_FIMC0: u32 = 64;
pub const DOUT_UART3: u32 = 65;
pub const DOUT_UART2: u32 = 66;
pub const DOUT_UART1: u32 = 67;
pub const DOUT_UART0: u32 = 68;
pub const DOUT_MMC3: u32 = 69;
pub const DOUT_MMC2: u32 = 70;
pub const DOUT_MMC1: u32 = 71;
pub const DOUT_MMC0: u32 = 72;
pub const DOUT_PWM: u32 = 73;
pub const DOUT_SPI1: u32 = 74;
pub const DOUT_SPI0: u32 = 75;
pub const DOUT_DMC0: u32 = 76;
pub const DOUT_PWI: u32 = 77;
pub const DOUT_HPM: u32 = 78;
pub const DOUT_COPY: u32 = 79;
pub const DOUT_FLASH: u32 = 80;
pub const DOUT_AUDIO2: u32 = 81;
pub const DOUT_AUDIO1: u32 = 82;
pub const DOUT_AUDIO0: u32 = 83;
pub const DOUT_DPM: u32 = 84;
pub const DOUT_DVSEM: u32 = 85;

/* Gates */
pub const SCLK_FIMC: u32 = 86;
pub const CLK_CSIS: u32 = 87;
pub const CLK_ROTATOR: u32 = 88;
pub const CLK_FIMC2: u32 = 89;
pub const CLK_FIMC1: u32 = 90;
pub const CLK_FIMC0: u32 = 91;
pub const CLK_MFC: u32 = 92;
pub const CLK_G2D: u32 = 93;
pub const CLK_G3D: u32 = 94;
pub const CLK_IMEM: u32 = 95;
pub const CLK_PDMA1: u32 = 96;
pub const CLK_PDMA0: u32 = 97;
pub const CLK_MDMA: u32 = 98;
pub const CLK_DMC1: u32 = 99;
pub const CLK_DMC0: u32 = 100;
pub const CLK_NFCON: u32 = 101;
pub const CLK_SROMC: u32 = 102;
pub const CLK_CFCON: u32 = 103;
pub const CLK_NANDXL: u32 = 104;
pub const CLK_USB_HOST: u32 = 105;
pub const CLK_USB_OTG: u32 = 106;
pub const CLK_HDMI: u32 = 107;
pub const CLK_TVENC: u32 = 108;
pub const CLK_MIXER: u32 = 109;
pub const CLK_VP: u32 = 110;
pub const CLK_DSIM: u32 = 111;
pub const CLK_FIMD: u32 = 112;
pub const CLK_TZIC3: u32 = 113;
pub const CLK_TZIC2: u32 = 114;
pub const CLK_TZIC1: u32 = 115;
pub const CLK_TZIC0: u32 = 116;
pub const CLK_VIC3: u32 = 117;
pub const CLK_VIC2: u32 = 118;
pub const CLK_VIC1: u32 = 119;
pub const CLK_VIC0: u32 = 120;
pub const CLK_TSI: u32 = 121;
pub const CLK_HSMMC3: u32 = 122;
pub const CLK_HSMMC2: u32 = 123;
pub const CLK_HSMMC1: u32 = 124;
pub const CLK_HSMMC0: u32 = 125;
pub const CLK_JTAG: u32 = 126;
pub const CLK_MODEMIF: u32 = 127;
pub const CLK_CORESIGHT: u32 = 128;
pub const CLK_SDM: u32 = 129;
pub const CLK_SECSS: u32 = 130;
pub const CLK_PCM2: u32 = 131;
pub const CLK_PCM1: u32 = 132;
pub const CLK_PCM0: u32 = 133;
pub const CLK_SYSCON: u32 = 134;
pub const CLK_GPIO: u32 = 135;
pub const CLK_TSADC: u32 = 136;
pub const CLK_PWM: u32 = 137;
pub const CLK_WDT: u32 = 138;
pub const CLK_KEYIF: u32 = 139;
pub const CLK_UART3: u32 = 140;
pub const CLK_UART2: u32 = 141;
pub const CLK_UART1: u32 = 142;
pub const CLK_UART0: u32 = 143;
pub const CLK_SYSTIMER: u32 = 144;
pub const CLK_RTC: u32 = 145;
pub const CLK_SPI1: u32 = 146;
pub const CLK_SPI0: u32 = 147;
pub const CLK_I2C_HDMI_PHY: u32 = 148;
pub const CLK_I2C1: u32 = 149;
pub const CLK_I2C2: u32 = 150;
pub const CLK_I2C0: u32 = 151;
pub const CLK_I2S1: u32 = 152;
pub const CLK_I2S2: u32 = 153;
pub const CLK_I2S0: u32 = 154;
pub const CLK_AC97: u32 = 155;
pub const CLK_SPDIF: u32 = 156;
pub const CLK_TZPC3: u32 = 157;
pub const CLK_TZPC2: u32 = 158;
pub const CLK_TZPC1: u32 = 159;
pub const CLK_TZPC0: u32 = 160;
pub const CLK_SECKEY: u32 = 161;
pub const CLK_IEM_APC: u32 = 162;
pub const CLK_IEM_IEC: u32 = 163;
pub const CLK_CHIPID: u32 = 164;
pub const CLK_JPEG: u32 = 163;

/* Special clocks*/
pub const SCLK_PWI: u32 = 164;
pub const SCLK_SPDIF: u32 = 165;
pub const SCLK_AUDIO2: u32 = 166;
pub const SCLK_AUDIO1: u32 = 167;
pub const SCLK_AUDIO0: u32 = 168;
pub const SCLK_PWM: u32 = 169;
pub const SCLK_SPI1: u32 = 170;
pub const SCLK_SPI0: u32 = 171;
pub const SCLK_UART3: u32 = 172;
pub const SCLK_UART2: u32 = 173;
pub const SCLK_UART1: u32 = 174;
pub const SCLK_UART0: u32 = 175;
pub const SCLK_MMC3: u32 = 176;
pub const SCLK_MMC2: u32 = 177;
pub const SCLK_MMC1: u32 = 178;
pub const SCLK_MMC0: u32 = 179;
pub const SCLK_FINVPLL: u32 = 180;
pub const SCLK_CSIS: u32 = 181;
pub const SCLK_FIMD: u32 = 182;
pub const SCLK_CAM1: u32 = 183;
pub const SCLK_CAM0: u32 = 184;
pub const SCLK_DAC: u32 = 185;
pub const SCLK_MIXER: u32 = 186;
pub const SCLK_HDMI: u32 = 187;
pub const SCLK_FIMC2: u32 = 188;
pub const SCLK_FIMC1: u32 = 189;
pub const SCLK_FIMC0: u32 = 190;
pub const SCLK_HDMI27M: u32 = 191;
pub const SCLK_HDMIPHY: u32 = 192;
pub const SCLK_USBPHY0: u32 = 193;
pub const SCLK_USBPHY1: u32 = 194;

/* S5P6442-specific clocks */
pub const MOUT_D0SYNC: u32 = 195;
pub const MOUT_D1SYNC: u32 = 196;
pub const DOUT_MIXER: u32 = 197;
pub const CLK_ETB: u32 = 198;
pub const CLK_ETM: u32 = 199;

/* CLKOUT */
pub const FOUT_APLL_CLKOUT: u32 = 200;
pub const FOUT_MPLL_CLKOUT: u32 = 201;
pub const DOUT_APLL_CLKOUT: u32 = 202;
pub const MOUT_CLKSEL: u32 = 203;
pub const DOUT_CLKOUT: u32 = 204;
pub const MOUT_CLKOUT: u32 = 205;

/* Total number of clocks. */
pub const NR_CLKS: u32 = 206;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
