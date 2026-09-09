/*
 * Copyright (C) 2016 Maxime Ripard <maxime.ripard@free-electrons.com>
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

pub const CLK_PLL_MIPI: u32 = 13;
pub const CLK_CPUX: u32 = 18;
pub const CLK_BUS_MIPI_DSI: u32 = 23;
pub const CLK_BUS_SS: u32 = 24;
pub const CLK_BUS_DMA: u32 = 25;
pub const CLK_BUS_MMC0: u32 = 26;
pub const CLK_BUS_MMC1: u32 = 27;
pub const CLK_BUS_MMC2: u32 = 28;
pub const CLK_BUS_NAND: u32 = 29;
pub const CLK_BUS_DRAM: u32 = 30;
pub const CLK_BUS_HSTIMER: u32 = 31;
pub const CLK_BUS_SPI0: u32 = 32;
pub const CLK_BUS_SPI1: u32 = 33;
pub const CLK_BUS_OTG: u32 = 34;
pub const CLK_BUS_EHCI: u32 = 35;
pub const CLK_BUS_OHCI: u32 = 36;
pub const CLK_BUS_VE: u32 = 37;
pub const CLK_BUS_LCD: u32 = 38;
pub const CLK_BUS_CSI: u32 = 39;
pub const CLK_BUS_DE_BE: u32 = 40;
pub const CLK_BUS_DE_FE: u32 = 41;
pub const CLK_BUS_GPU: u32 = 42;
pub const CLK_BUS_MSGBOX: u32 = 43;
pub const CLK_BUS_SPINLOCK: u32 = 44;
pub const CLK_BUS_DRC: u32 = 45;
pub const CLK_BUS_SAT: u32 = 46;
pub const CLK_BUS_CODEC: u32 = 47;
pub const CLK_BUS_PIO: u32 = 48;
pub const CLK_BUS_I2S0: u32 = 49;
pub const CLK_BUS_I2S1: u32 = 50;
pub const CLK_BUS_I2C0: u32 = 51;
pub const CLK_BUS_I2C1: u32 = 52;
pub const CLK_BUS_I2C2: u32 = 53;
pub const CLK_BUS_UART0: u32 = 54;
pub const CLK_BUS_UART1: u32 = 55;
pub const CLK_BUS_UART2: u32 = 56;
pub const CLK_BUS_UART3: u32 = 57;
pub const CLK_BUS_UART4: u32 = 58;
pub const CLK_NAND: u32 = 59;
pub const CLK_MMC0: u32 = 60;
pub const CLK_MMC0_SAMPLE: u32 = 61;
pub const CLK_MMC0_OUTPUT: u32 = 62;
pub const CLK_MMC1: u32 = 63;
pub const CLK_MMC1_SAMPLE: u32 = 64;
pub const CLK_MMC1_OUTPUT: u32 = 65;
pub const CLK_MMC2: u32 = 66;
pub const CLK_MMC2_SAMPLE: u32 = 67;
pub const CLK_MMC2_OUTPUT: u32 = 68;
pub const CLK_SS: u32 = 69;
pub const CLK_SPI0: u32 = 70;
pub const CLK_SPI1: u32 = 71;
pub const CLK_I2S0: u32 = 72;
pub const CLK_I2S1: u32 = 73;
pub const CLK_USB_PHY0: u32 = 74;
pub const CLK_USB_PHY1: u32 = 75;
pub const CLK_USB_HSIC: u32 = 76;
pub const CLK_USB_HSIC_12M: u32 = 77;
pub const CLK_USB_OHCI: u32 = 78;
pub const CLK_DRAM_VE: u32 = 80;
pub const CLK_DRAM_CSI: u32 = 81;
pub const CLK_DRAM_DRC: u32 = 82;
pub const CLK_DRAM_DE_FE: u32 = 83;
pub const CLK_DRAM_DE_BE: u32 = 84;
pub const CLK_DE_BE: u32 = 85;
pub const CLK_DE_FE: u32 = 86;
pub const CLK_LCD_CH0: u32 = 87;
pub const CLK_LCD_CH1: u32 = 88;
pub const CLK_CSI_SCLK: u32 = 89;
pub const CLK_CSI_MCLK: u32 = 90;
pub const CLK_VE: u32 = 91;
pub const CLK_AC_DIG: u32 = 92;
pub const CLK_AC_DIG_4X: u32 = 93;
pub const CLK_AVS: u32 = 94;
pub const CLK_DSI_SCLK: u32 = 96;
pub const CLK_DSI_DPHY: u32 = 97;
pub const CLK_DRC: u32 = 98;
pub const CLK_GPU: u32 = 99;
pub const CLK_ATS: u32 = 100;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
