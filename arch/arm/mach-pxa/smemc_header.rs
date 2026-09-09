/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Static memory controller register definitions for PXA CPUs
 *
 * Copyright (C) 2010 Marek Vasut <marek.vasut@gmail.com>
 */

pub const PXA2XX_SMEMC_BASE: usize = 0x48000000;
pub const PXA3XX_SMEMC_BASE: usize = 0x4a000000;
pub const SMEMC_VIRT: usize = 0xf6000000;

pub const MDCNFG: usize = SMEMC_VIRT + 0x00; /* SDRAM Configuration Register 0 */
pub const MDREFR: usize = SMEMC_VIRT + 0x04; /* SDRAM Refresh Control Register */
pub const MSC0: usize = SMEMC_VIRT + 0x08; /* Static Memory Control Register 0 */
pub const MSC1: usize = SMEMC_VIRT + 0x0C; /* Static Memory Control Register 1 */
pub const MSC2: usize = SMEMC_VIRT + 0x10; /* Static Memory Control Register 2 */
pub const MECR: usize = SMEMC_VIRT + 0x14; /* Expansion Memory (PCMCIA/Compact Flash) Bus Configuration */
pub const SXLCR: usize = SMEMC_VIRT + 0x18; /* LCR value to be written to SDRAM-Timing Synchronous Flash */
pub const SXCNFG: usize = SMEMC_VIRT + 0x1C; /* Synchronous Static Memory Control Register */
pub const SXMRS: usize = SMEMC_VIRT + 0x24; /* MRS value to be written to Synchronous Flash or SMROM */
pub const MCMEM0: usize = SMEMC_VIRT + 0x28; /* Card interface Common Memory Space Socket 0 Timing */
pub const MCMEM1: usize = SMEMC_VIRT + 0x2C; /* Card interface Common Memory Space Socket 1 Timing */
pub const MCATT0: usize = SMEMC_VIRT + 0x30; /* Card interface Attribute Space Socket 0 Timing Configuration */
pub const MCATT1: usize = SMEMC_VIRT + 0x34; /* Card interface Attribute Space Socket 1 Timing Configuration */
pub const MCIO0: usize = SMEMC_VIRT + 0x38; /* Card interface I/O Space Socket 0 Timing Configuration */
pub const MCIO1: usize = SMEMC_VIRT + 0x3C; /* Card interface I/O Space Socket 1 Timing Configuration */
pub const MDMRS: usize = SMEMC_VIRT + 0x40; /* MRS value to be written to SDRAM */
pub const BOOT_DEF: usize = SMEMC_VIRT + 0x44; /* Read-Only Boot-Time Register. Contains BOOT_SEL and PKG_SEL */
pub const MEMCLKCFG: usize = SMEMC_VIRT + 0x68; /* Clock Configuration */
pub const CSADRCFG0: usize = SMEMC_VIRT + 0x80; /* Address Configuration Register for CS0 */
pub const CSADRCFG1: usize = SMEMC_VIRT + 0x84; /* Address Configuration Register for CS1 */
pub const CSADRCFG2: usize = SMEMC_VIRT + 0x88; /* Address Configuration Register for CS2 */
pub const CSADRCFG3: usize = SMEMC_VIRT + 0x8C; /* Address Configuration Register for CS3 */
pub const CSMSADRCFG: usize = SMEMC_VIRT + 0xA0; /* Chip Select Configuration Register */

/*
 * More handy macros for PCMCIA
 *
 * Arg is socket number
 */
pub const fn MCMEM(s: usize) -> usize {
    SMEMC_VIRT + 0x28 + (s << 2)
}

pub const fn MCATT(s: usize) -> usize {
    SMEMC_VIRT + 0x30 + (s << 2)
}

pub const fn MCIO(s: usize) -> usize {
    SMEMC_VIRT + 0x38 + (s << 2)
}

/* MECR register defines */
pub const MECR_NOS: u32 = 1 << 0; /* Number Of Sockets: 0 -> 1 sock, 1 -> 2 sock */
pub const MECR_CIT: u32 = 1 << 1; /* Card Is There: 0 -> no card, 1 -> card inserted */

pub const MDCNFG_DE0: u32 = 1 << 0; /* SDRAM Bank 0 Enable */
pub const MDCNFG_DE1: u32 = 1 << 1; /* SDRAM Bank 1 Enable */
pub const MDCNFG_DE2: u32 = 1 << 16; /* SDRAM Bank 2 Enable */
pub const MDCNFG_DE3: u32 = 1 << 17; /* SDRAM Bank 3 Enable */

pub const MDREFR_K0DB4: u32 = 1 << 29; /* SDCLK0 Divide by 4 Control/Status */
pub const MDREFR_K2FREE: u32 = 1 << 25; /* SDRAM Free-Running Control */
pub const MDREFR_K1FREE: u32 = 1 << 24; /* SDRAM Free-Running Control */
pub const MDREFR_K0FREE: u32 = 1 << 23; /* SDRAM Free-Running Control */
pub const MDREFR_SLFRSH: u32 = 1 << 22; /* SDRAM Self-Refresh Control/Status */
pub const MDREFR_APD: u32 = 1 << 20; /* SDRAM/SSRAM Auto-Power-Down Enable */
pub const MDREFR_K2DB2: u32 = 1 << 19; /* SDCLK2 Divide by 2 Control/Status */
pub const MDREFR_K2RUN: u32 = 1 << 18; /* SDCLK2 Run Control/Status */
pub const MDREFR_K1DB2: u32 = 1 << 17; /* SDCLK1 Divide by 2 Control/Status */
pub const MDREFR_K1RUN: u32 = 1 << 16; /* SDCLK1 Run Control/Status */
pub const MDREFR_E1PIN: u32 = 1 << 15; /* SDCKE1 Level Control/Status */
pub const MDREFR_K0DB2: u32 = 1 << 14; /* SDCLK0 Divide by 2 Control/Status */
pub const MDREFR_K0RUN: u32 = 1 << 13; /* SDCLK0 Run Control/Status */
pub const MDREFR_E0PIN: u32 = 1 << 12; /* SDCKE0 Level Control/Status */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
