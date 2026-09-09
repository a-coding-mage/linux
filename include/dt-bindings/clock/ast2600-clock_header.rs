/* SPDX-License-Identifier: GPL-2.0-or-later OR MIT */

pub const ASPEED_CLK_GATE_ECLK: u32 = 0;
pub const ASPEED_CLK_GATE_GCLK: u32 = 1;

pub const ASPEED_CLK_GATE_MCLK: u32 = 2;

pub const ASPEED_CLK_GATE_VCLK: u32 = 3;
pub const ASPEED_CLK_GATE_BCLK: u32 = 4;
pub const ASPEED_CLK_GATE_DCLK: u32 = 5;

pub const ASPEED_CLK_GATE_LCLK: u32 = 6;
pub const ASPEED_CLK_GATE_LHCCLK: u32 = 7;

pub const ASPEED_CLK_GATE_D1CLK: u32 = 8;
pub const ASPEED_CLK_GATE_YCLK: u32 = 9;

pub const ASPEED_CLK_GATE_REF0CLK: u32 = 10;
pub const ASPEED_CLK_GATE_REF1CLK: u32 = 11;

pub const ASPEED_CLK_GATE_ESPICLK: u32 = 12;

pub const ASPEED_CLK_GATE_USBUHCICLK: u32 = 13;
pub const ASPEED_CLK_GATE_USBPORT1CLK: u32 = 14;
pub const ASPEED_CLK_GATE_USBPORT2CLK: u32 = 15;

pub const ASPEED_CLK_GATE_RSACLK: u32 = 16;
pub const ASPEED_CLK_GATE_RVASCLK: u32 = 17;

pub const ASPEED_CLK_GATE_MAC1CLK: u32 = 18;
pub const ASPEED_CLK_GATE_MAC2CLK: u32 = 19;
pub const ASPEED_CLK_GATE_MAC3CLK: u32 = 20;
pub const ASPEED_CLK_GATE_MAC4CLK: u32 = 21;

pub const ASPEED_CLK_GATE_UART1CLK: u32 = 22;
pub const ASPEED_CLK_GATE_UART2CLK: u32 = 23;
pub const ASPEED_CLK_GATE_UART3CLK: u32 = 24;
pub const ASPEED_CLK_GATE_UART4CLK: u32 = 25;
pub const ASPEED_CLK_GATE_UART5CLK: u32 = 26;
pub const ASPEED_CLK_GATE_UART6CLK: u32 = 27;
pub const ASPEED_CLK_GATE_UART7CLK: u32 = 28;
pub const ASPEED_CLK_GATE_UART8CLK: u32 = 29;
pub const ASPEED_CLK_GATE_UART9CLK: u32 = 30;
pub const ASPEED_CLK_GATE_UART10CLK: u32 = 31;
pub const ASPEED_CLK_GATE_UART11CLK: u32 = 32;
pub const ASPEED_CLK_GATE_UART12CLK: u32 = 33;
pub const ASPEED_CLK_GATE_UART13CLK: u32 = 34;

pub const ASPEED_CLK_GATE_SDCLK: u32 = 35;
pub const ASPEED_CLK_GATE_EMMCCLK: u32 = 36;

pub const ASPEED_CLK_GATE_I3C0CLK: u32 = 37;
pub const ASPEED_CLK_GATE_I3C1CLK: u32 = 38;
pub const ASPEED_CLK_GATE_I3C2CLK: u32 = 39;
pub const ASPEED_CLK_GATE_I3C3CLK: u32 = 40;
pub const ASPEED_CLK_GATE_I3C4CLK: u32 = 41;
pub const ASPEED_CLK_GATE_I3C5CLK: u32 = 42;

pub const ASPEED_CLK_GATE_FSICLK: u32 = 45;

pub const ASPEED_CLK_HPLL: u32 = 46;
pub const ASPEED_CLK_MPLL: u32 = 47;
pub const ASPEED_CLK_DPLL: u32 = 48;
pub const ASPEED_CLK_EPLL: u32 = 49;
pub const ASPEED_CLK_APLL: u32 = 50;
pub const ASPEED_CLK_AHB: u32 = 51;
pub const ASPEED_CLK_APB1: u32 = 52;
pub const ASPEED_CLK_APB2: u32 = 53;
pub const ASPEED_CLK_BCLK: u32 = 54;
pub const ASPEED_CLK_D1CLK: u32 = 55;
pub const ASPEED_CLK_VCLK: u32 = 56;
pub const ASPEED_CLK_LHCLK: u32 = 57;
pub const ASPEED_CLK_UART: u32 = 58;
pub const ASPEED_CLK_UARTX: u32 = 59;
pub const ASPEED_CLK_SDIO: u32 = 60;
pub const ASPEED_CLK_EMMC: u32 = 61;
pub const ASPEED_CLK_ECLK: u32 = 62;
pub const ASPEED_CLK_ECLK_MUX: u32 = 63;
pub const ASPEED_CLK_MAC12: u32 = 64;
pub const ASPEED_CLK_MAC34: u32 = 65;
pub const ASPEED_CLK_USBPHY_40M: u32 = 66;
pub const ASPEED_CLK_MAC1RCLK: u32 = 67;
pub const ASPEED_CLK_MAC2RCLK: u32 = 68;
pub const ASPEED_CLK_MAC3RCLK: u32 = 69;
pub const ASPEED_CLK_MAC4RCLK: u32 = 70;
pub const ASPEED_CLK_I3C: u32 = 71;
pub const ASPEED_CLK_FSI: u32 = 72;

// Only list resets here that are not part of a clock gate + reset pair
pub const ASPEED_RESET_ADC: u32 = 55;
pub const ASPEED_RESET_JTAG_MASTER2: u32 = 54;

pub const ASPEED_RESET_MAC4: u32 = 53;
pub const ASPEED_RESET_MAC3: u32 = 52;

pub const ASPEED_RESET_I3C5: u32 = 45;
pub const ASPEED_RESET_I3C4: u32 = 44;
pub const ASPEED_RESET_I3C3: u32 = 43;
pub const ASPEED_RESET_I3C2: u32 = 42;
pub const ASPEED_RESET_I3C1: u32 = 41;
pub const ASPEED_RESET_I3C0: u32 = 40;
pub const ASPEED_RESET_I3C: u32 = 39;
pub const ASPEED_RESET_I3C_DMA: u32 = 39;

pub const ASPEED_RESET_PWM: u32 = 37;
pub const ASPEED_RESET_PECI: u32 = 36;
pub const ASPEED_RESET_MII: u32 = 35;
pub const ASPEED_RESET_I2C: u32 = 34;
pub const ASPEED_RESET_H2X: u32 = 31;
pub const ASPEED_RESET_GP_MCU: u32 = 30;
pub const ASPEED_RESET_DP_MCU: u32 = 29;
pub const ASPEED_RESET_DP: u32 = 28;
pub const ASPEED_RESET_RC_XDMA: u32 = 27;
pub const ASPEED_RESET_GRAPHICS: u32 = 26;
pub const ASPEED_RESET_DEV_XDMA: u32 = 25;
pub const ASPEED_RESET_DEV_MCTP: u32 = 24;
pub const ASPEED_RESET_RC_MCTP: u32 = 23;
pub const ASPEED_RESET_JTAG_MASTER: u32 = 22;
pub const ASPEED_RESET_PCIE_DEV_O: u32 = 21;
pub const ASPEED_RESET_PCIE_DEV_OEN: u32 = 20;
pub const ASPEED_RESET_PCIE_RC_O: u32 = 19;
pub const ASPEED_RESET_PCIE_RC_OEN: u32 = 18;
pub const ASPEED_RESET_MAC2: u32 = 12;
pub const ASPEED_RESET_MAC1: u32 = 11;
pub const ASPEED_RESET_PCI_DP: u32 = 5;
pub const ASPEED_RESET_HACE: u32 = 4;
pub const ASPEED_RESET_AHB: u32 = 1;
pub const ASPEED_RESET_SDRAM: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
