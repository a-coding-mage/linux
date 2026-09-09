/*
 * Hardware info about DECstation 5000/2x0 systems (otherwise known as
 * 3max+) and DECsystem 5900 systems (otherwise known as bigmax) which
 * differ mechanically but are otherwise identical (both are known as
 * KN03).
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995,1996 by Paul M. Antoine, some code and definitions
 * are by courtesy of Chris Fraser.
 * Copyright (C) 2000, 2002, 2003, 2005  Maciej W. Rozycki
 */

// Dependencies supplied by the original header:
// #include <asm/dec/ecc.h>
// #include <asm/dec/ioasic_addrs.h>

pub const KN03_SLOT_BASE: u32 = 0x1f800000;

/*
 * CPU interrupt bits.
 */
pub const KN03_CPU_INR_HALT: u32 = 6; // HALT button
pub const KN03_CPU_INR_BUS: u32 = 5; // memory, I/O bus read/write errors
pub const KN03_CPU_INR_RES_4: u32 = 4; // unused
pub const KN03_CPU_INR_RTC: u32 = 3; // DS1287 RTC
pub const KN03_CPU_INR_CASCADE: u32 = 2; // I/O ASIC cascade

/*
 * I/O ASIC interrupt bits.  Star marks denote non-IRQ status bits.
 */
pub const KN03_IO_INR_3MAXP: u32 = 15; // (*) 3max+/bigmax ID
pub const KN03_IO_INR_NVRAM: u32 = 14; // (*) NVRAM clear jumper
pub const KN03_IO_INR_TC2: u32 = 13; // TURBOchannel slot #2
pub const KN03_IO_INR_TC1: u32 = 12; // TURBOchannel slot #1
pub const KN03_IO_INR_TC0: u32 = 11; // TURBOchannel slot #0
pub const KN03_IO_INR_NRMOD: u32 = 10; // (*) NRMOD manufacturing jumper
pub const KN03_IO_INR_ASC: u32 = 9; // ASC (NCR53C94) SCSI
pub const KN03_IO_INR_LANCE: u32 = 8; // LANCE (Am7990) Ethernet
pub const KN03_IO_INR_SCC1: u32 = 7; // SCC (Z85C30) serial #1
pub const KN03_IO_INR_SCC0: u32 = 6; // SCC (Z85C30) serial #0
pub const KN03_IO_INR_RTC: u32 = 5; // DS1287 RTC
pub const KN03_IO_INR_PSU: u32 = 4; // power supply unit warning
pub const KN03_IO_INR_RES_3: u32 = 3; // unused
pub const KN03_IO_INR_ASC_DATA: u32 = 2; // SCSI data ready (for PIO)
pub const KN03_IO_INR_PBNC: u32 = 1; // ~HALT button debouncer
pub const KN03_IO_INR_PBNO: u32 = 0; // HALT button debouncer

/*
 * Memory Control Register bits.
 */
pub const KN03_MCR_RES_16: u32 = 0xffffu32 << 16; // unused
pub const KN03_MCR_DIAGCHK: u32 = 1u32 << 15; // diagn/norml ECC reads
pub const KN03_MCR_DIAGGEN: u32 = 1u32 << 14; // diagn/norml ECC writes
pub const KN03_MCR_CORRECT: u32 = 1u32 << 13; // ECC correct/check
pub const KN03_MCR_RES_11: u32 = 0x3u32 << 12; // unused
pub const KN03_MCR_BNK32M: u32 = 1u32 << 10; // 32M/8M stride
pub const KN03_MCR_RES_7: u32 = 0x7u32 << 7; // unused
pub const KN03_MCR_CHECK: u32 = 0x7fu32 << 0; // diagnostic check bits

/*
 * I/O ASIC System Support Register bits.
 */
pub const KN03_IO_SSR_TXDIS1: u32 = 1u32 << 14; // SCC1 transmit disable
pub const KN03_IO_SSR_TXDIS0: u32 = 1u32 << 13; // SCC0 transmit disable
pub const KN03_IO_SSR_RES_12: u32 = 1u32 << 12; // unused

pub const KN03_IO_SSR_LEDS: u32 = 0xffu32 << 0; // ~diagnostic LEDs

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
