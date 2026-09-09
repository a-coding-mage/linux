/*
 * Copyright (C) 2016 Chen-Yu Tsai <wens@csie.org>
 *
 * This file is dual-licensed: you can use it either under the terms
 * of the GPL or the X11 license, at your option. Note that this dual
 * licensing only applies to this file, and not this project as a
 * whole.
 *
 *  a) This file is free software; you can redistribute it and/or
 *     modify it under the terms of the GNU General Public License as
 *     published by the Free Software Foundation; either version 2 of
 *     the License, or (at your option) any later version.
 *
 *     This file is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 * Or, alternatively,
 *
 *  b) Permission is hereby granted, free of charge, to any person
 *     obtaining a copy of this software and associated documentation
 *     files (the "Software"), to deal in the Software without
 *     restriction, including without limitation the rights to use,
 *     copy, modify, merge, publish, distribute, sublicense, and/or
 *     sell copies of the Software, and to permit persons to whom the
 *     Software is furnished to do so, subject to the following
 *     conditions:
 *
 *     The above copyright notice and this permission notice shall be
 *     included in all copies or substantial portions of the Software.
 *
 *     THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 *     EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 *     OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 *     NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 *     HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 *     WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 *     FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 *     OTHER DEALINGS IN THE SOFTWARE.
 */

pub const CLK_PLL_VIDEO0_2X: u32 = 7;
pub const CLK_PLL_PERIPH: u32 = 10;
pub const CLK_PLL_VIDEO1_2X: u32 = 13;
pub const CLK_PLL_MIPI: u32 = 15;
pub const CLK_CPU: u32 = 18;

pub const CLK_AHB1_MIPIDSI: u32 = 23;
pub const CLK_AHB1_SS: u32 = 24;
pub const CLK_AHB1_DMA: u32 = 25;
pub const CLK_AHB1_MMC0: u32 = 26;
pub const CLK_AHB1_MMC1: u32 = 27;
pub const CLK_AHB1_MMC2: u32 = 28;
pub const CLK_AHB1_MMC3: u32 = 29;
pub const CLK_AHB1_NAND1: u32 = 30;
pub const CLK_AHB1_NAND0: u32 = 31;
pub const CLK_AHB1_SDRAM: u32 = 32;
pub const CLK_AHB1_EMAC: u32 = 33;
pub const CLK_AHB1_TS: u32 = 34;
pub const CLK_AHB1_HSTIMER: u32 = 35;
pub const CLK_AHB1_SPI0: u32 = 36;
pub const CLK_AHB1_SPI1: u32 = 37;
pub const CLK_AHB1_SPI2: u32 = 38;
pub const CLK_AHB1_SPI3: u32 = 39;
pub const CLK_AHB1_OTG: u32 = 40;
pub const CLK_AHB1_EHCI0: u32 = 41;
pub const CLK_AHB1_EHCI1: u32 = 42;
pub const CLK_AHB1_OHCI0: u32 = 43;
pub const CLK_AHB1_OHCI1: u32 = 44;
pub const CLK_AHB1_OHCI2: u32 = 45;
pub const CLK_AHB1_VE: u32 = 46;
pub const CLK_AHB1_LCD0: u32 = 47;
pub const CLK_AHB1_LCD1: u32 = 48;
pub const CLK_AHB1_CSI: u32 = 49;
pub const CLK_AHB1_HDMI: u32 = 50;
pub const CLK_AHB1_BE0: u32 = 51;
pub const CLK_AHB1_BE1: u32 = 52;
pub const CLK_AHB1_FE0: u32 = 53;
pub const CLK_AHB1_FE1: u32 = 54;
pub const CLK_AHB1_MP: u32 = 55;
pub const CLK_AHB1_GPU: u32 = 56;
pub const CLK_AHB1_DEU0: u32 = 57;
pub const CLK_AHB1_DEU1: u32 = 58;
pub const CLK_AHB1_DRC0: u32 = 59;
pub const CLK_AHB1_DRC1: u32 = 60;

pub const CLK_APB1_CODEC: u32 = 61;
pub const CLK_APB1_SPDIF: u32 = 62;
pub const CLK_APB1_DIGITAL_MIC: u32 = 63;
pub const CLK_APB1_PIO: u32 = 64;
pub const CLK_APB1_DAUDIO0: u32 = 65;
pub const CLK_APB1_DAUDIO1: u32 = 66;

pub const CLK_APB2_I2C0: u32 = 67;
pub const CLK_APB2_I2C1: u32 = 68;
pub const CLK_APB2_I2C2: u32 = 69;
pub const CLK_APB2_I2C3: u32 = 70;
pub const CLK_APB2_UART0: u32 = 71;
pub const CLK_APB2_UART1: u32 = 72;
pub const CLK_APB2_UART2: u32 = 73;
pub const CLK_APB2_UART3: u32 = 74;
pub const CLK_APB2_UART4: u32 = 75;
pub const CLK_APB2_UART5: u32 = 76;

pub const CLK_NAND0: u32 = 77;
pub const CLK_NAND1: u32 = 78;
pub const CLK_MMC0: u32 = 79;
pub const CLK_MMC0_SAMPLE: u32 = 80;
pub const CLK_MMC0_OUTPUT: u32 = 81;
pub const CLK_MMC1: u32 = 82;
pub const CLK_MMC1_SAMPLE: u32 = 83;
pub const CLK_MMC1_OUTPUT: u32 = 84;
pub const CLK_MMC2: u32 = 85;
pub const CLK_MMC2_SAMPLE: u32 = 86;
pub const CLK_MMC2_OUTPUT: u32 = 87;
pub const CLK_MMC3: u32 = 88;
pub const CLK_MMC3_SAMPLE: u32 = 89;
pub const CLK_MMC3_OUTPUT: u32 = 90;
pub const CLK_TS: u32 = 91;
pub const CLK_SS: u32 = 92;
pub const CLK_SPI0: u32 = 93;
pub const CLK_SPI1: u32 = 94;
pub const CLK_SPI2: u32 = 95;
pub const CLK_SPI3: u32 = 96;
pub const CLK_DAUDIO0: u32 = 97;
pub const CLK_DAUDIO1: u32 = 98;
pub const CLK_SPDIF: u32 = 99;
pub const CLK_USB_PHY0: u32 = 100;
pub const CLK_USB_PHY1: u32 = 101;
pub const CLK_USB_PHY2: u32 = 102;
pub const CLK_USB_OHCI0: u32 = 103;
pub const CLK_USB_OHCI1: u32 = 104;
pub const CLK_USB_OHCI2: u32 = 105;

pub const CLK_DRAM_VE: u32 = 110;
pub const CLK_DRAM_CSI_ISP: u32 = 111;
pub const CLK_DRAM_TS: u32 = 112;
pub const CLK_DRAM_DRC0: u32 = 113;
pub const CLK_DRAM_DRC1: u32 = 114;
pub const CLK_DRAM_DEU0: u32 = 115;
pub const CLK_DRAM_DEU1: u32 = 116;
pub const CLK_DRAM_FE0: u32 = 117;
pub const CLK_DRAM_FE1: u32 = 118;
pub const CLK_DRAM_BE0: u32 = 119;
pub const CLK_DRAM_BE1: u32 = 120;
pub const CLK_DRAM_MP: u32 = 121;

pub const CLK_BE0: u32 = 122;
pub const CLK_BE1: u32 = 123;
pub const CLK_FE0: u32 = 124;
pub const CLK_FE1: u32 = 125;
pub const CLK_MP: u32 = 126;
pub const CLK_LCD0_CH0: u32 = 127;
pub const CLK_LCD1_CH0: u32 = 128;
pub const CLK_LCD0_CH1: u32 = 129;
pub const CLK_LCD1_CH1: u32 = 130;
pub const CLK_CSI0_SCLK: u32 = 131;
pub const CLK_CSI0_MCLK: u32 = 132;
pub const CLK_CSI1_MCLK: u32 = 133;
pub const CLK_VE: u32 = 134;
pub const CLK_CODEC: u32 = 135;
pub const CLK_AVS: u32 = 136;
pub const CLK_DIGITAL_MIC: u32 = 137;
pub const CLK_HDMI: u32 = 138;
pub const CLK_HDMI_DDC: u32 = 139;
pub const CLK_PS: u32 = 140;

pub const CLK_MIPI_DSI: u32 = 143;
pub const CLK_MIPI_DSI_DPHY: u32 = 144;
pub const CLK_MIPI_CSI_DPHY: u32 = 145;
pub const CLK_IEP_DRC0: u32 = 146;
pub const CLK_IEP_DRC1: u32 = 147;
pub const CLK_IEP_DEU0: u32 = 148;
pub const CLK_IEP_DEU1: u32 = 149;
pub const CLK_GPU_CORE: u32 = 150;
pub const CLK_GPU_MEMORY: u32 = 151;
pub const CLK_GPU_HYD: u32 = 152;
pub const CLK_ATS: u32 = 153;
pub const CLK_TRACE: u32 = 154;

pub const CLK_OUT_A: u32 = 155;
pub const CLK_OUT_B: u32 = 156;
pub const CLK_OUT_C: u32 = 157;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
