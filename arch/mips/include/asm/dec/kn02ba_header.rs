/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/asm-mips/dec/kn02ba.h
 *
 * DECstation 5000/1xx (3min or KN02-BA) definitions.
 *
 * Copyright (C) 2002, 2003 Maciej W. Rozycki
 */

// Common definitions are supplied by asm/dec/kn02xa.h.

/*
 * CPU interrupt bits.
 */
pub const KN02BA_CPU_INR_HALT: u32 = 6; /* HALT button */
pub const KN02BA_CPU_INR_CASCADE: u32 = 5; /* I/O ASIC cascade */
pub const KN02BA_CPU_INR_TC2: u32 = 4; /* TURBOchannel slot #2 */
pub const KN02BA_CPU_INR_TC1: u32 = 3; /* TURBOchannel slot #1 */
pub const KN02BA_CPU_INR_TC0: u32 = 2; /* TURBOchannel slot #0 */

/*
 * I/O ASIC interrupt bits. Star marks denote non-IRQ status bits.
 */
pub const KN02BA_IO_INR_RES_15: u32 = 15; /* unused */
pub const KN02BA_IO_INR_NVRAM: u32 = 14; /* (*) NVRAM clear jumper */
pub const KN02BA_IO_INR_RES_13: u32 = 13; /* unused */
pub const KN02BA_IO_INR_BUS: u32 = 12; /* memory, I/O bus read/write errors */
pub const KN02BA_IO_INR_RES_11: u32 = 11; /* unused */
pub const KN02BA_IO_INR_NRMOD: u32 = 10; /* (*) NRMOD manufacturing jumper */
pub const KN02BA_IO_INR_ASC: u32 = 9; /* ASC (NCR53C94) SCSI */
pub const KN02BA_IO_INR_LANCE: u32 = 8; /* LANCE (Am7990) Ethernet */
pub const KN02BA_IO_INR_SCC1: u32 = 7; /* SCC (Z85C30) serial #1 */
pub const KN02BA_IO_INR_SCC0: u32 = 6; /* SCC (Z85C30) serial #0 */
pub const KN02BA_IO_INR_RTC: u32 = 5; /* DS1287 RTC */
pub const KN02BA_IO_INR_PSU: u32 = 4; /* power supply unit warning */
pub const KN02BA_IO_INR_RES_3: u32 = 3; /* unused */
pub const KN02BA_IO_INR_ASC_DATA: u32 = 2; /* SCSI data ready (for PIO) */
pub const KN02BA_IO_INR_PBNC: u32 = 1; /* ~HALT button debouncer */
pub const KN02BA_IO_INR_PBNO: u32 = 0; /* HALT button debouncer */

/*
 * Memory Error Register bits.
 */
pub const KN02BA_MER_RES_27: u32 = 1u32 << 27; /* unused */

/*
 * Memory Size Register bits.
 */
pub const KN02BA_MSR_RES_17: u32 = 0x3ffu32 << 17; /* unused */

/*
 * I/O ASIC System Support Register bits.
 */
pub const KN02BA_IO_SSR_TXDIS1: u32 = 1u32 << 14; /* SCC1 transmit disable */
pub const KN02BA_IO_SSR_TXDIS0: u32 = 1u32 << 13; /* SCC0 transmit disable */
pub const KN02BA_IO_SSR_RES_12: u32 = 1u32 << 12; /* unused */

pub const KN02BA_IO_SSR_LEDS: u32 = 0xffu32 << 0; /* ~diagnostic LEDs */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
