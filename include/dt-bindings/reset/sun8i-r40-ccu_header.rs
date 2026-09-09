/*
 * Copyright (C) 2017 Icenowy Zheng <icenowy@aosc.io>
 *
 * This file is dual-licensed: you can use it either under the terms
 * of the GPL or the X11 license, at your option. Note that this dual
 * licensing only applies to this file, and not this project as a
 * whole.
 *
 *  a) This file is free software; you can redistribute it and/or
 *     modify it under the terms of the GNU General Public License as
 *     published by the Free Software Foundation; either version 2 of the
 *     License, or (at your option) any later version.
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

pub const RST_DRAM: u32 = 3;
pub const RST_MBUS: u32 = 4;

pub const RST_BUS_MIPI_DSI: u32 = 5;
pub const RST_BUS_CE: u32 = 6;
pub const RST_BUS_DMA: u32 = 7;
pub const RST_BUS_MMC0: u32 = 8;
pub const RST_BUS_MMC1: u32 = 9;
pub const RST_BUS_MMC2: u32 = 10;
pub const RST_BUS_MMC3: u32 = 11;
pub const RST_BUS_NAND: u32 = 12;
pub const RST_BUS_DRAM: u32 = 13;
pub const RST_BUS_EMAC: u32 = 14;
pub const RST_BUS_TS: u32 = 15;
pub const RST_BUS_HSTIMER: u32 = 16;
pub const RST_BUS_SPI0: u32 = 17;
pub const RST_BUS_SPI1: u32 = 18;
pub const RST_BUS_SPI2: u32 = 19;
pub const RST_BUS_SPI3: u32 = 20;
pub const RST_BUS_SATA: u32 = 21;
pub const RST_BUS_OTG: u32 = 22;
pub const RST_BUS_EHCI0: u32 = 23;
pub const RST_BUS_EHCI1: u32 = 24;
pub const RST_BUS_EHCI2: u32 = 25;
pub const RST_BUS_OHCI0: u32 = 26;
pub const RST_BUS_OHCI1: u32 = 27;
pub const RST_BUS_OHCI2: u32 = 28;
pub const RST_BUS_VE: u32 = 29;
pub const RST_BUS_MP: u32 = 30;
pub const RST_BUS_DEINTERLACE: u32 = 31;
pub const RST_BUS_CSI0: u32 = 32;
pub const RST_BUS_CSI1: u32 = 33;
pub const RST_BUS_HDMI0: u32 = 34;
pub const RST_BUS_HDMI1: u32 = 35;
pub const RST_BUS_DE: u32 = 36;
pub const RST_BUS_TVE0: u32 = 37;
pub const RST_BUS_TVE1: u32 = 38;
pub const RST_BUS_TVE_TOP: u32 = 39;
pub const RST_BUS_GMAC: u32 = 40;
pub const RST_BUS_GPU: u32 = 41;
pub const RST_BUS_TVD0: u32 = 42;
pub const RST_BUS_TVD1: u32 = 43;
pub const RST_BUS_TVD2: u32 = 44;
pub const RST_BUS_TVD3: u32 = 45;
pub const RST_BUS_TVD_TOP: u32 = 46;
pub const RST_BUS_TCON_LCD0: u32 = 47;
pub const RST_BUS_TCON_LCD1: u32 = 48;
pub const RST_BUS_TCON_TV0: u32 = 49;
pub const RST_BUS_TCON_TV1: u32 = 50;
pub const RST_BUS_TCON_TOP: u32 = 51;
pub const RST_BUS_DBG: u32 = 52;
pub const RST_BUS_LVDS: u32 = 53;
pub const RST_BUS_CODEC: u32 = 54;
pub const RST_BUS_SPDIF: u32 = 55;
pub const RST_BUS_AC97: u32 = 56;
pub const RST_BUS_IR0: u32 = 57;
pub const RST_BUS_IR1: u32 = 58;
pub const RST_BUS_THS: u32 = 59;
pub const RST_BUS_KEYPAD: u32 = 60;
pub const RST_BUS_I2S0: u32 = 61;
pub const RST_BUS_I2S1: u32 = 62;
pub const RST_BUS_I2S2: u32 = 63;
pub const RST_BUS_I2C0: u32 = 64;
pub const RST_BUS_I2C1: u32 = 65;
pub const RST_BUS_I2C2: u32 = 66;
pub const RST_BUS_I2C3: u32 = 67;
pub const RST_BUS_CAN: u32 = 68;
pub const RST_BUS_SCR: u32 = 69;
pub const RST_BUS_PS20: u32 = 70;
pub const RST_BUS_PS21: u32 = 71;
pub const RST_BUS_I2C4: u32 = 72;
pub const RST_BUS_UART0: u32 = 73;
pub const RST_BUS_UART1: u32 = 74;
pub const RST_BUS_UART2: u32 = 75;
pub const RST_BUS_UART3: u32 = 76;
pub const RST_BUS_UART4: u32 = 77;
pub const RST_BUS_UART5: u32 = 78;
pub const RST_BUS_UART6: u32 = 79;
pub const RST_BUS_UART7: u32 = 80;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
