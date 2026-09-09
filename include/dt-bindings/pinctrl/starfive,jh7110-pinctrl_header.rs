/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2022 Emil Renner Berthing <kernel@esmil.dk>
 * Copyright (C) 2022 StarFive Technology Co., Ltd.
 */

/* sys_iomux pins */
pub const PAD_GPIO0: i32 = 0;
pub const PAD_GPIO1: i32 = 1;
pub const PAD_GPIO2: i32 = 2;
pub const PAD_GPIO3: i32 = 3;
pub const PAD_GPIO4: i32 = 4;
pub const PAD_GPIO5: i32 = 5;
pub const PAD_GPIO6: i32 = 6;
pub const PAD_GPIO7: i32 = 7;
pub const PAD_GPIO8: i32 = 8;
pub const PAD_GPIO9: i32 = 9;
pub const PAD_GPIO10: i32 = 10;
pub const PAD_GPIO11: i32 = 11;
pub const PAD_GPIO12: i32 = 12;
pub const PAD_GPIO13: i32 = 13;
pub const PAD_GPIO14: i32 = 14;
pub const PAD_GPIO15: i32 = 15;
pub const PAD_GPIO16: i32 = 16;
pub const PAD_GPIO17: i32 = 17;
pub const PAD_GPIO18: i32 = 18;
pub const PAD_GPIO19: i32 = 19;
pub const PAD_GPIO20: i32 = 20;
pub const PAD_GPIO21: i32 = 21;
pub const PAD_GPIO22: i32 = 22;
pub const PAD_GPIO23: i32 = 23;
pub const PAD_GPIO24: i32 = 24;
pub const PAD_GPIO25: i32 = 25;
pub const PAD_GPIO26: i32 = 26;
pub const PAD_GPIO27: i32 = 27;
pub const PAD_GPIO28: i32 = 28;
pub const PAD_GPIO29: i32 = 29;
pub const PAD_GPIO30: i32 = 30;
pub const PAD_GPIO31: i32 = 31;
pub const PAD_GPIO32: i32 = 32;
pub const PAD_GPIO33: i32 = 33;
pub const PAD_GPIO34: i32 = 34;
pub const PAD_GPIO35: i32 = 35;
pub const PAD_GPIO36: i32 = 36;
pub const PAD_GPIO37: i32 = 37;
pub const PAD_GPIO38: i32 = 38;
pub const PAD_GPIO39: i32 = 39;
pub const PAD_GPIO40: i32 = 40;
pub const PAD_GPIO41: i32 = 41;
pub const PAD_GPIO42: i32 = 42;
pub const PAD_GPIO43: i32 = 43;
pub const PAD_GPIO44: i32 = 44;
pub const PAD_GPIO45: i32 = 45;
pub const PAD_GPIO46: i32 = 46;
pub const PAD_GPIO47: i32 = 47;
pub const PAD_GPIO48: i32 = 48;
pub const PAD_GPIO49: i32 = 49;
pub const PAD_GPIO50: i32 = 50;
pub const PAD_GPIO51: i32 = 51;
pub const PAD_GPIO52: i32 = 52;
pub const PAD_GPIO53: i32 = 53;
pub const PAD_GPIO54: i32 = 54;
pub const PAD_GPIO55: i32 = 55;
pub const PAD_GPIO56: i32 = 56;
pub const PAD_GPIO57: i32 = 57;
pub const PAD_GPIO58: i32 = 58;
pub const PAD_GPIO59: i32 = 59;
pub const PAD_GPIO60: i32 = 60;
pub const PAD_GPIO61: i32 = 61;
pub const PAD_GPIO62: i32 = 62;
pub const PAD_GPIO63: i32 = 63;
pub const PAD_SD0_CLK: i32 = 64;
pub const PAD_SD0_CMD: i32 = 65;
pub const PAD_SD0_DATA0: i32 = 66;
pub const PAD_SD0_DATA1: i32 = 67;
pub const PAD_SD0_DATA2: i32 = 68;
pub const PAD_SD0_DATA3: i32 = 69;
pub const PAD_SD0_DATA4: i32 = 70;
pub const PAD_SD0_DATA5: i32 = 71;
pub const PAD_SD0_DATA6: i32 = 72;
pub const PAD_SD0_DATA7: i32 = 73;
pub const PAD_SD0_STRB: i32 = 74;
pub const PAD_GMAC1_MDC: i32 = 75;
pub const PAD_GMAC1_MDIO: i32 = 76;
pub const PAD_GMAC1_RXD0: i32 = 77;
pub const PAD_GMAC1_RXD1: i32 = 78;
pub const PAD_GMAC1_RXD2: i32 = 79;
pub const PAD_GMAC1_RXD3: i32 = 80;
pub const PAD_GMAC1_RXDV: i32 = 81;
pub const PAD_GMAC1_RXC: i32 = 82;
pub const PAD_GMAC1_TXD0: i32 = 83;
pub const PAD_GMAC1_TXD1: i32 = 84;
pub const PAD_GMAC1_TXD2: i32 = 85;
pub const PAD_GMAC1_TXD3: i32 = 86;
pub const PAD_GMAC1_TXEN: i32 = 87;
pub const PAD_GMAC1_TXC: i32 = 88;
pub const PAD_QSPI_SCLK: i32 = 89;
pub const PAD_QSPI_CS0: i32 = 90;
pub const PAD_QSPI_DATA0: i32 = 91;
pub const PAD_QSPI_DATA1: i32 = 92;
pub const PAD_QSPI_DATA2: i32 = 93;
pub const PAD_QSPI_DATA3: i32 = 94;

/* aon_iomux pins */
pub const PAD_TESTEN: i32 = 0;
pub const PAD_RGPIO0: i32 = 1;
pub const PAD_RGPIO1: i32 = 2;
pub const PAD_RGPIO2: i32 = 3;
pub const PAD_RGPIO3: i32 = 4;
pub const PAD_RSTN: i32 = 5;
pub const PAD_GMAC0_MDC: i32 = 6;
pub const PAD_GMAC0_MDIO: i32 = 7;
pub const PAD_GMAC0_RXD0: i32 = 8;
pub const PAD_GMAC0_RXD1: i32 = 9;
pub const PAD_GMAC0_RXD2: i32 = 10;
pub const PAD_GMAC0_RXD3: i32 = 11;
pub const PAD_GMAC0_RXDV: i32 = 12;
pub const PAD_GMAC0_RXC: i32 = 13;
pub const PAD_GMAC0_TXD0: i32 = 14;
pub const PAD_GMAC0_TXD1: i32 = 15;
pub const PAD_GMAC0_TXD2: i32 = 16;
pub const PAD_GMAC0_TXD3: i32 = 17;
pub const PAD_GMAC0_TXEN: i32 = 18;
pub const PAD_GMAC0_TXC: i32 = 19;

pub const GPOUT_LOW: i32 = 0;
pub const GPOUT_HIGH: i32 = 1;

pub const GPOEN_ENABLE: i32 = 0;
pub const GPOEN_DISABLE: i32 = 1;

pub const GPI_NONE: i32 = 255;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
