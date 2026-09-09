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

pub const RST_USB_PHY0: u32 = 0;
pub const RST_USB_PHY1: u32 = 1;
pub const RST_USB_PHY2: u32 = 2;

pub const RST_AHB1_MIPI_DSI: u32 = 3;
pub const RST_AHB1_SS: u32 = 4;
pub const RST_AHB1_DMA: u32 = 5;
pub const RST_AHB1_MMC0: u32 = 6;
pub const RST_AHB1_MMC1: u32 = 7;
pub const RST_AHB1_MMC2: u32 = 8;
pub const RST_AHB1_MMC3: u32 = 9;
pub const RST_AHB1_NAND1: u32 = 10;
pub const RST_AHB1_NAND0: u32 = 11;
pub const RST_AHB1_SDRAM: u32 = 12;
pub const RST_AHB1_EMAC: u32 = 13;
pub const RST_AHB1_TS: u32 = 14;
pub const RST_AHB1_HSTIMER: u32 = 15;
pub const RST_AHB1_SPI0: u32 = 16;
pub const RST_AHB1_SPI1: u32 = 17;
pub const RST_AHB1_SPI2: u32 = 18;
pub const RST_AHB1_SPI3: u32 = 19;
pub const RST_AHB1_OTG: u32 = 20;
pub const RST_AHB1_EHCI0: u32 = 21;
pub const RST_AHB1_EHCI1: u32 = 22;
pub const RST_AHB1_OHCI0: u32 = 23;
pub const RST_AHB1_OHCI1: u32 = 24;
pub const RST_AHB1_OHCI2: u32 = 25;
pub const RST_AHB1_VE: u32 = 26;
pub const RST_AHB1_LCD0: u32 = 27;
pub const RST_AHB1_LCD1: u32 = 28;
pub const RST_AHB1_CSI: u32 = 29;
pub const RST_AHB1_HDMI: u32 = 30;
pub const RST_AHB1_BE0: u32 = 31;
pub const RST_AHB1_BE1: u32 = 32;
pub const RST_AHB1_FE0: u32 = 33;
pub const RST_AHB1_FE1: u32 = 34;
pub const RST_AHB1_MP: u32 = 35;
pub const RST_AHB1_GPU: u32 = 36;
pub const RST_AHB1_DEU0: u32 = 37;
pub const RST_AHB1_DEU1: u32 = 38;
pub const RST_AHB1_DRC0: u32 = 39;
pub const RST_AHB1_DRC1: u32 = 40;
pub const RST_AHB1_LVDS: u32 = 41;

pub const RST_APB1_CODEC: u32 = 42;
pub const RST_APB1_SPDIF: u32 = 43;
pub const RST_APB1_DIGITAL_MIC: u32 = 44;
pub const RST_APB1_DAUDIO0: u32 = 45;
pub const RST_APB1_DAUDIO1: u32 = 46;
pub const RST_APB2_I2C0: u32 = 47;
pub const RST_APB2_I2C1: u32 = 48;
pub const RST_APB2_I2C2: u32 = 49;
pub const RST_APB2_I2C3: u32 = 50;
pub const RST_APB2_UART0: u32 = 51;
pub const RST_APB2_UART1: u32 = 52;
pub const RST_APB2_UART2: u32 = 53;
pub const RST_APB2_UART3: u32 = 54;
pub const RST_APB2_UART4: u32 = 55;
pub const RST_APB2_UART5: u32 = 56;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
