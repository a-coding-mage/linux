/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  linux/include/asm-arm/hardware/amba_kmi.h
 *
 *  Internal header file for AMBA KMI ports
 *
 *  Copyright (C) 2000 Deep Blue Solutions Ltd.
 *
 * ---------------------------------------------------------------------------
 *  From ARM PrimeCell(tm) PS2 Keyboard/Mouse Interface (PL050) Technical
 *  Reference Manual - ARM DDI 0143B - see http://www.arm.com/
 * ---------------------------------------------------------------------------
 */

// KMI control register:
//  KMICR_TYPE       0 = PS2/AT mode, 1 = No line control bit mode
//  KMICR_RXINTREN   1 = enable RX interrupts
//  KMICR_TXINTREN   1 = enable TX interrupts
//  KMICR_EN         1 = enable KMI
//  KMICR_FD         1 = force KMI data low
//  KMICR_FC         1 = force KMI clock low
pub const KMICR: usize = KMI_BASE + 0x00;
pub const KMICR_TYPE: usize = 1 << 5;
pub const KMICR_RXINTREN: usize = 1 << 4;
pub const KMICR_TXINTREN: usize = 1 << 3;
pub const KMICR_EN: usize = 1 << 2;
pub const KMICR_FD: usize = 1 << 1;
pub const KMICR_FC: usize = 1 << 0;

// KMI status register:
//  KMISTAT_TXEMPTY  1 = transmitter register empty
//  KMISTAT_TXBUSY   1 = currently sending data
//  KMISTAT_RXFULL   1 = receiver register ready to be read
//  KMISTAT_RXBUSY   1 = currently receiving data
//  KMISTAT_RXPARITY parity of last databyte received
//  KMISTAT_IC       current level of KMI clock input
//  KMISTAT_ID       current level of KMI data input
pub const KMISTAT: usize = KMI_BASE + 0x04;
pub const KMISTAT_TXEMPTY: usize = 1 << 6;
pub const KMISTAT_TXBUSY: usize = 1 << 5;
pub const KMISTAT_RXFULL: usize = 1 << 4;
pub const KMISTAT_RXBUSY: usize = 1 << 3;
pub const KMISTAT_RXPARITY: usize = 1 << 2;
pub const KMISTAT_IC: usize = 1 << 1;
pub const KMISTAT_ID: usize = 1 << 0;

// KMI data register
pub const KMIDATA: usize = KMI_BASE + 0x08;

// KMI clock divisor: to generate 8MHz internal clock
//  div = (ref / 8MHz) - 1; 0 <= div <= 15
pub const KMICLKDIV: usize = KMI_BASE + 0x0c;

// KMI interrupt register:
//  KMIIR_TXINTR     1 = transmit interrupt asserted
//  KMIIR_RXINTR     1 = receive interrupt asserted
pub const KMIIR: usize = KMI_BASE + 0x10;
pub const KMIIR_TXINTR: usize = 1 << 1;
pub const KMIIR_RXINTR: usize = 1 << 0;

// The size of the KMI primecell
pub const KMI_SIZE: usize = 0x100;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
