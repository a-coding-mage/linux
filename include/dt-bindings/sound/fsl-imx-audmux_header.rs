/* SPDX-License-Identifier: GPL-2.0 */

pub const MX27_AUDMUX_HPCR1_SSI0: u32 = 0;
pub const MX27_AUDMUX_HPCR2_SSI1: u32 = 1;
pub const MX27_AUDMUX_HPCR3_SSI_PINS_4: u32 = 2;
pub const MX27_AUDMUX_PPCR1_SSI_PINS_1: u32 = 3;
pub const MX27_AUDMUX_PPCR2_SSI_PINS_2: u32 = 4;
pub const MX27_AUDMUX_PPCR3_SSI_PINS_3: u32 = 5;

pub const MX31_AUDMUX_PORT1_SSI0: u32 = 0;
pub const MX31_AUDMUX_PORT2_SSI1: u32 = 1;
pub const MX31_AUDMUX_PORT3_SSI_PINS_3: u32 = 2;
pub const MX31_AUDMUX_PORT4_SSI_PINS_4: u32 = 3;
pub const MX31_AUDMUX_PORT5_SSI_PINS_5: u32 = 4;
pub const MX31_AUDMUX_PORT6_SSI_PINS_6: u32 = 5;
pub const MX31_AUDMUX_PORT7_SSI_PINS_7: u32 = 6;

pub const MX51_AUDMUX_PORT1_SSI0: u32 = 0;
pub const MX51_AUDMUX_PORT2_SSI1: u32 = 1;
pub const MX51_AUDMUX_PORT3: u32 = 2;
pub const MX51_AUDMUX_PORT4: u32 = 3;
pub const MX51_AUDMUX_PORT5: u32 = 4;
pub const MX51_AUDMUX_PORT6: u32 = 5;
pub const MX51_AUDMUX_PORT7: u32 = 6;

/*
 * TFCSEL/RFCSEL (i.MX27) or TFSEL/TCSEL/RFSEL/RCSEL (i.MX31/51/53/6Q)
 * can be sourced from Rx/Tx.
 */
pub const IMX_AUDMUX_RXFS: u32 = 0x8;
pub const IMX_AUDMUX_RXCLK: u32 = 0x8;

/* Register definitions for the i.MX21/27 Digital Audio Multiplexer */
pub const fn IMX_AUDMUX_V1_PCR_INMMASK(x: u32) -> u32 { x & 0xff }
pub const IMX_AUDMUX_V1_PCR_INMEN: u32 = 1 << 8;
pub const IMX_AUDMUX_V1_PCR_TXRXEN: u32 = 1 << 10;
pub const IMX_AUDMUX_V1_PCR_SYN: u32 = 1 << 12;
pub const fn IMX_AUDMUX_V1_PCR_RXDSEL(x: u32) -> u32 { (x & 0x7) << 13 }
pub const fn IMX_AUDMUX_V1_PCR_RFCSEL(x: u32) -> u32 { (x & 0xf) << 20 }
pub const IMX_AUDMUX_V1_PCR_RCLKDIR: u32 = 1 << 24;
pub const IMX_AUDMUX_V1_PCR_RFSDIR: u32 = 1 << 25;
pub const fn IMX_AUDMUX_V1_PCR_TFCSEL(x: u32) -> u32 { (x & 0xf) << 26 }
pub const IMX_AUDMUX_V1_PCR_TCLKDIR: u32 = 1 << 30;
pub const IMX_AUDMUX_V1_PCR_TFSDIR: u32 = 1 << 31;

/* Register definitions for the i.MX25/31/35/51 Digital Audio Multiplexer */
pub const IMX_AUDMUX_V2_PTCR_TFSDIR: u32 = 1 << 31;
pub const fn IMX_AUDMUX_V2_PTCR_TFSEL(x: u32) -> u32 { (x & 0xf) << 27 }
pub const IMX_AUDMUX_V2_PTCR_TCLKDIR: u32 = 1 << 26;
pub const fn IMX_AUDMUX_V2_PTCR_TCSEL(x: u32) -> u32 { (x & 0xf) << 22 }
pub const IMX_AUDMUX_V2_PTCR_RFSDIR: u32 = 1 << 21;
pub const fn IMX_AUDMUX_V2_PTCR_RFSEL(x: u32) -> u32 { (x & 0xf) << 17 }
pub const IMX_AUDMUX_V2_PTCR_RCLKDIR: u32 = 1 << 16;
pub const fn IMX_AUDMUX_V2_PTCR_RCSEL(x: u32) -> u32 { (x & 0xf) << 12 }
pub const IMX_AUDMUX_V2_PTCR_SYN: u32 = 1 << 11;

pub const fn IMX_AUDMUX_V2_PDCR_RXDSEL(x: u32) -> u32 { (x & 0x7) << 13 }
pub const IMX_AUDMUX_V2_PDCR_TXRXEN: u32 = 1 << 12;
pub const fn IMX_AUDMUX_V2_PDCR_MODE(x: u32) -> u32 { (x & 0x3) << 8 }
pub const fn IMX_AUDMUX_V2_PDCR_INMMASK(x: u32) -> u32 { x & 0xff }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
