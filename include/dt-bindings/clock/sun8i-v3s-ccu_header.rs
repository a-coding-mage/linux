/*
 * Copyright (c) 2016 Icenowy Zheng <icenowy@aosc.xyz>
 *
 * Based on sun8i-h3-ccu.h, which is:
 * Copyright (C) 2016 Maxime Ripard <maxime.ripard@free-electrons.com>
 *
 * This file is dual-licensed: you can use it either under the terms
 * of the GPL or the X11 license, at your option. Note that this dual
 * licensing only applies to this file, and not this project as a
 * whole.
 *
 *  a) This file is free software: you can redistribute it and/or
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

pub const CLK_CPU: i32 = 14;

pub const CLK_BUS_CE: i32 = 20;
pub const CLK_BUS_DMA: i32 = 21;
pub const CLK_BUS_MMC0: i32 = 22;
pub const CLK_BUS_MMC1: i32 = 23;
pub const CLK_BUS_MMC2: i32 = 24;
pub const CLK_BUS_DRAM: i32 = 25;
pub const CLK_BUS_EMAC: i32 = 26;
pub const CLK_BUS_HSTIMER: i32 = 27;
pub const CLK_BUS_SPI0: i32 = 28;
pub const CLK_BUS_OTG: i32 = 29;
pub const CLK_BUS_EHCI0: i32 = 30;
pub const CLK_BUS_OHCI0: i32 = 31;
pub const CLK_BUS_VE: i32 = 32;
pub const CLK_BUS_TCON0: i32 = 33;
pub const CLK_BUS_CSI: i32 = 34;
pub const CLK_BUS_DE: i32 = 35;
pub const CLK_BUS_CODEC: i32 = 36;
pub const CLK_BUS_PIO: i32 = 37;
pub const CLK_BUS_I2C0: i32 = 38;
pub const CLK_BUS_I2C1: i32 = 39;
pub const CLK_BUS_UART0: i32 = 40;
pub const CLK_BUS_UART1: i32 = 41;
pub const CLK_BUS_UART2: i32 = 42;
pub const CLK_BUS_EPHY: i32 = 43;
pub const CLK_BUS_DBG: i32 = 44;

pub const CLK_MMC0: i32 = 45;
pub const CLK_MMC0_SAMPLE: i32 = 46;
pub const CLK_MMC0_OUTPUT: i32 = 47;
pub const CLK_MMC1: i32 = 48;
pub const CLK_MMC1_SAMPLE: i32 = 49;
pub const CLK_MMC1_OUTPUT: i32 = 50;
pub const CLK_MMC2: i32 = 51;
pub const CLK_MMC2_SAMPLE: i32 = 52;
pub const CLK_MMC2_OUTPUT: i32 = 53;
pub const CLK_CE: i32 = 54;
pub const CLK_SPI0: i32 = 55;
pub const CLK_USB_PHY0: i32 = 56;
pub const CLK_USB_OHCI0: i32 = 57;

pub const CLK_DRAM_VE: i32 = 59;
pub const CLK_DRAM_CSI: i32 = 60;
pub const CLK_DRAM_EHCI: i32 = 61;
pub const CLK_DRAM_OHCI: i32 = 62;
pub const CLK_DE: i32 = 63;
pub const CLK_TCON0: i32 = 64;
pub const CLK_CSI_MISC: i32 = 65;
pub const CLK_CSI0_MCLK: i32 = 66;
pub const CLK_CSI_SCLK: i32 = 67;
pub const CLK_CSI1_MCLK: i32 = 68;
pub const CLK_VE: i32 = 69;
pub const CLK_AC_DIG: i32 = 70;
pub const CLK_AVS: i32 = 71;

pub const CLK_MIPI_CSI: i32 = 73;

/* Clocks not available on V3s */
pub const CLK_BUS_I2S0: i32 = 75;
pub const CLK_I2S0: i32 = 76;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
