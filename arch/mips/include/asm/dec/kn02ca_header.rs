/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/dec/kn02ca.h
 *
 *	Personal DECstation 5000/xx (Maxine or KN02-CA) definitions.
 *
 *	Copyright (C) 2002, 2003  Maciej W. Rozycki
 */

// Common definitions are supplied by asm/dec/kn02xa.h.

/*
 * CPU interrupt bits.
 */
pub const KN02CA_CPU_INR_HALT: i32 = 6; // HALT from ACCESS.Bus
pub const KN02CA_CPU_INR_CASCADE: i32 = 5; // I/O ASIC cascade
pub const KN02CA_CPU_INR_BUS: i32 = 4; // memory, I/O bus read/write errors
pub const KN02CA_CPU_INR_RTC: i32 = 3; // DS1287 RTC
pub const KN02CA_CPU_INR_TIMER: i32 = 2; // ARC periodic timer

/*
 * I/O ASIC interrupt bits.  Star marks denote non-IRQ status bits.
 */
pub const KN02CA_IO_INR_FLOPPY: i32 = 15; // 82077 FDC
pub const KN02CA_IO_INR_NVRAM: i32 = 14; // (*) NVRAM clear jumper
pub const KN02CA_IO_INR_POWERON: i32 = 13; // (*) ACCESS.Bus/power-on reset
pub const KN02CA_IO_INR_TC0: i32 = 12; // TURBOchannel slot #0
pub const KN02CA_IO_INR_TIMER: i32 = 12; // ARC periodic timer (?)
pub const KN02CA_IO_INR_ISDN: i32 = 11; // Am79C30A ISDN
pub const KN02CA_IO_INR_NRMOD: i32 = 10; // (*) NRMOD manufacturing jumper
pub const KN02CA_IO_INR_ASC: i32 = 9; // ASC (NCR53C94) SCSI
pub const KN02CA_IO_INR_LANCE: i32 = 8; // LANCE (Am7990) Ethernet
pub const KN02CA_IO_INR_HDFLOPPY: i32 = 7; // (*) HD (1.44MB) floppy status
pub const KN02CA_IO_INR_SCC0: i32 = 6; // SCC (Z85C30) serial #0
pub const KN02CA_IO_INR_TC1: i32 = 5; // TURBOchannel slot #1
pub const KN02CA_IO_INR_XDFLOPPY: i32 = 4; // (*) XD (2.88MB) floppy status
pub const KN02CA_IO_INR_VIDEO: i32 = 3; // framebuffer
pub const KN02CA_IO_INR_XVIDEO: i32 = 2; // ~framebuffer
pub const KN02CA_IO_INR_AB_XMIT: i32 = 1; // ACCESS.bus transmit
pub const KN02CA_IO_INR_AB_RECV: i32 = 0; // ACCESS.bus receive

/*
 * Memory Error Register bits.
 */
pub const KN02CA_MER_INTR: u32 = 1u32 << 27; // ARC IRQ status & ack

/*
 * Memory Size Register bits.
 */
pub const KN02CA_MSR_INTREN: u32 = 1u32 << 26; // ARC periodic IRQ enable
pub const KN02CA_MSR_MS10EN: u32 = 1u32 << 25; // 10/1ms IRQ period select
pub const KN02CA_MSR_PFORCE: u32 = 0xfu32 << 21; // byte lane error force
pub const KN02CA_MSR_MABEN: u32 = 1u32 << 20; // A side VFB address enable
pub const KN02CA_MSR_LASTBANK: u32 = 0x7u32 << 17; // onboard RAM bank #

/*
 * I/O ASIC System Support Register bits.
 */
pub const KN03CA_IO_SSR_RES_14: u32 = 1u32 << 14; // unused
pub const KN03CA_IO_SSR_RES_13: u32 = 1u32 << 13; // unused
pub const KN03CA_IO_SSR_ISDN_RST: u32 = 1u32 << 12; // ~ISDN (Am79C30A) reset

pub const KN03CA_IO_SSR_FLOPPY_RST: u32 = 1u32 << 7; // ~FDC (82077) reset
pub const KN03CA_IO_SSR_VIDEO_RST: u32 = 1u32 << 6; // ~framebuffer reset
pub const KN03CA_IO_SSR_AB_RST: u32 = 1u32 << 5; // ACCESS.bus reset
pub const KN03CA_IO_SSR_RES_4: u32 = 1u32 << 4; // unused
pub const KN03CA_IO_SSR_RES_3: u32 = 1u32 << 4; // unused
pub const KN03CA_IO_SSR_RES_2: u32 = 1u32 << 2; // unused
pub const KN03CA_IO_SSR_RES_1: u32 = 1u32 << 1; // unused
pub const KN03CA_IO_SSR_LED: u32 = 1u32 << 0; // power LED

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
