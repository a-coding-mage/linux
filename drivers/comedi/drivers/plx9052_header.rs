/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Definitions for the PLX-9052 PCI interface chip
 *
 * Copyright (C) 2002 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * INTCSR - Interrupt Control/Status register
 */
pub const PLX9052_INTCSR: u32 = 0x4c;
pub const PLX9052_INTCSR_LI1ENAB: u32 = 1u32 << 0; /* LI1 enabled */
pub const PLX9052_INTCSR_LI1POL: u32 = 1u32 << 1; /* LI1 active high */
pub const PLX9052_INTCSR_LI1STAT: u32 = 1u32 << 2; /* LI1 active */
pub const PLX9052_INTCSR_LI2ENAB: u32 = 1u32 << 3; /* LI2 enabled */
pub const PLX9052_INTCSR_LI2POL: u32 = 1u32 << 4; /* LI2 active high */
pub const PLX9052_INTCSR_LI2STAT: u32 = 1u32 << 5; /* LI2 active */
pub const PLX9052_INTCSR_PCIENAB: u32 = 1u32 << 6; /* PCIINT enabled */
pub const PLX9052_INTCSR_SOFTINT: u32 = 1u32 << 7; /* generate soft int */
pub const PLX9052_INTCSR_LI1SEL: u32 = 1u32 << 8; /* LI1 edge */
pub const PLX9052_INTCSR_LI2SEL: u32 = 1u32 << 9; /* LI2 edge */
pub const PLX9052_INTCSR_LI1CLRINT: u32 = 1u32 << 10; /* LI1 clear int */
pub const PLX9052_INTCSR_LI2CLRINT: u32 = 1u32 << 11; /* LI2 clear int */
pub const PLX9052_INTCSR_ISAMODE: u32 = 1u32 << 12; /* ISA interface mode */

/*
 * CNTRL - User I/O, Direct Slave Response, Serial EEPROM, and
 * Initialization Control register
 */
pub const PLX9052_CNTRL: u32 = 0x50;
pub const PLX9052_CNTRL_WAITO: u32 = 1u32 << 0; /* UIO0 or WAITO# select */
pub const PLX9052_CNTRL_UIO0_DIR: u32 = 1u32 << 1; /* UIO0 direction */
pub const PLX9052_CNTRL_UIO0_DATA: u32 = 1u32 << 2; /* UIO0 data */
pub const PLX9052_CNTRL_LLOCKO: u32 = 1u32 << 3; /* UIO1 or LLOCKo# select */
pub const PLX9052_CNTRL_UIO1_DIR: u32 = 1u32 << 4; /* UIO1 direction */
pub const PLX9052_CNTRL_UIO1_DATA: u32 = 1u32 << 5; /* UIO1 data */
pub const PLX9052_CNTRL_CS2: u32 = 1u32 << 6; /* UIO2 or CS2# select */
pub const PLX9052_CNTRL_UIO2_DIR: u32 = 1u32 << 7; /* UIO2 direction */
pub const PLX9052_CNTRL_UIO2_DATA: u32 = 1u32 << 8; /* UIO2 data */
pub const PLX9052_CNTRL_CS3: u32 = 1u32 << 9; /* UIO3 or CS3# select */
pub const PLX9052_CNTRL_UIO3_DIR: u32 = 1u32 << 10; /* UIO3 direction */
pub const PLX9052_CNTRL_UIO3_DATA: u32 = 1u32 << 11; /* UIO3 data */
#[inline]
pub const fn PLX9052_CNTRL_PCIBAR(x: u32) -> u32 { (x & 0x3) << 12 }
pub const PLX9052_CNTRL_PCIBAR01: u32 = PLX9052_CNTRL_PCIBAR(0); /* mem and IO */
pub const PLX9052_CNTRL_PCIBAR0: u32 = PLX9052_CNTRL_PCIBAR(1); /* mem only */
pub const PLX9052_CNTRL_PCIBAR1: u32 = PLX9052_CNTRL_PCIBAR(2); /* IO only */
pub const PLX9052_CNTRL_PCI2_1_FEATURES: u32 = 1u32 << 14; /* PCI v2.1 features enabled */
pub const PLX9052_CNTRL_PCI_R_W_FLUSH: u32 = 1u32 << 15; /* read w/write flush mode */
pub const PLX9052_CNTRL_PCI_R_NO_FLUSH: u32 = 1u32 << 16; /* read no flush mode */
pub const PLX9052_CNTRL_PCI_R_NO_WRITE: u32 = 1u32 << 17; /* read no write mode */
pub const PLX9052_CNTRL_PCI_W_RELEASE: u32 = 1u32 << 18; /* write release bus mode */
#[inline]
pub const fn PLX9052_CNTRL_RETRY_CLKS(x: u32) -> u32 { (x & 0xf) << 19 } /* retry clks */
pub const PLX9052_CNTRL_LOCK_ENAB: u32 = 1u32 << 23; /* slave LOCK# enable */
pub const PLX9052_CNTRL_EEPROM_MASK: u32 = 0x1f << 24; /* EEPROM bits */
pub const PLX9052_CNTRL_EEPROM_CLK: u32 = 1u32 << 24; /* EEPROM clock */
pub const PLX9052_CNTRL_EEPROM_CS: u32 = 1u32 << 25; /* EEPROM chip select */
pub const PLX9052_CNTRL_EEPROM_DOUT: u32 = 1u32 << 26; /* EEPROM write bit */
pub const PLX9052_CNTRL_EEPROM_DIN: u32 = 1u32 << 27; /* EEPROM read bit */
pub const PLX9052_CNTRL_EEPROM_PRESENT: u32 = 1u32 << 28; /* EEPROM present */
pub const PLX9052_CNTRL_RELOAD_CFG: u32 = 1u32 << 29; /* reload configuration */
pub const PLX9052_CNTRL_PCI_RESET: u32 = 1u32 << 30; /* PCI adapter reset */
pub const PLX9052_CNTRL_MASK_REV: u32 = 1u32 << 31; /* mask revision */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
