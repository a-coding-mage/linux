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

pub const RST_USB_PHY0: u32 = 0;
pub const RST_USB_PHY1: u32 = 1;
pub const RST_USB_HSIC: u32 = 2;
pub const RST_MBUS: u32 = 3;
pub const RST_BUS_MIPI_DSI: u32 = 4;
pub const RST_BUS_SS: u32 = 5;
pub const RST_BUS_DMA: u32 = 6;
pub const RST_BUS_MMC0: u32 = 7;
pub const RST_BUS_MMC1: u32 = 8;
pub const RST_BUS_MMC2: u32 = 9;
pub const RST_BUS_NAND: u32 = 10;
pub const RST_BUS_DRAM: u32 = 11;
pub const RST_BUS_HSTIMER: u32 = 12;
pub const RST_BUS_SPI0: u32 = 13;
pub const RST_BUS_SPI1: u32 = 14;
pub const RST_BUS_OTG: u32 = 15;
pub const RST_BUS_EHCI: u32 = 16;
pub const RST_BUS_OHCI: u32 = 17;
pub const RST_BUS_VE: u32 = 18;
pub const RST_BUS_LCD: u32 = 19;
pub const RST_BUS_CSI: u32 = 20;
pub const RST_BUS_DE_BE: u32 = 21;
pub const RST_BUS_DE_FE: u32 = 22;
pub const RST_BUS_GPU: u32 = 23;
pub const RST_BUS_MSGBOX: u32 = 24;
pub const RST_BUS_SPINLOCK: u32 = 25;
pub const RST_BUS_DRC: u32 = 26;
pub const RST_BUS_SAT: u32 = 27;
pub const RST_BUS_LVDS: u32 = 28;
pub const RST_BUS_CODEC: u32 = 29;
pub const RST_BUS_I2S0: u32 = 30;
pub const RST_BUS_I2S1: u32 = 31;
pub const RST_BUS_I2C0: u32 = 32;
pub const RST_BUS_I2C1: u32 = 33;
pub const RST_BUS_I2C2: u32 = 34;
pub const RST_BUS_UART0: u32 = 35;
pub const RST_BUS_UART1: u32 = 36;
pub const RST_BUS_UART2: u32 = 37;
pub const RST_BUS_UART3: u32 = 38;
pub const RST_BUS_UART4: u32 = 39;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
