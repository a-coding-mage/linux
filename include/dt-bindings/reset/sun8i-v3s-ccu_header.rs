/*
 * Copyright (C) 2016 Icenowy Zheng <icenowy@aosc.xyz>
 *
 * Based on sun8i-v3s-ccu.h, which is
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

pub const RST_USB_PHY0: i32 = 0;

pub const RST_MBUS: i32 = 1;

pub const RST_BUS_CE: i32 = 5;
pub const RST_BUS_DMA: i32 = 6;
pub const RST_BUS_MMC0: i32 = 7;
pub const RST_BUS_MMC1: i32 = 8;
pub const RST_BUS_MMC2: i32 = 9;
pub const RST_BUS_DRAM: i32 = 11;
pub const RST_BUS_EMAC: i32 = 12;
pub const RST_BUS_HSTIMER: i32 = 14;
pub const RST_BUS_SPI0: i32 = 15;
pub const RST_BUS_OTG: i32 = 17;
pub const RST_BUS_EHCI0: i32 = 18;
pub const RST_BUS_OHCI0: i32 = 22;
pub const RST_BUS_VE: i32 = 26;
pub const RST_BUS_TCON0: i32 = 27;
pub const RST_BUS_CSI: i32 = 30;
pub const RST_BUS_DE: i32 = 34;
pub const RST_BUS_DBG: i32 = 38;
pub const RST_BUS_EPHY: i32 = 39;
pub const RST_BUS_CODEC: i32 = 40;
pub const RST_BUS_I2C0: i32 = 46;
pub const RST_BUS_I2C1: i32 = 47;
pub const RST_BUS_UART0: i32 = 49;
pub const RST_BUS_UART1: i32 = 50;
pub const RST_BUS_UART2: i32 = 51;

/* Reset lines not available on V3s */
pub const RST_BUS_I2S0: i32 = 52;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
