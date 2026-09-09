/*
 * Hardware info about DECstation 5000/200 systems (otherwise known as
 * 3max or KN02).
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995,1996 by Paul M. Antoine, some code and definitions
 * are by courtesy of Chris Fraser.
 * Copyright (C) 2002, 2003, 2005  Maciej W. Rozycki
 */

pub const KN02_SLOT_BASE: u32 = 0x1fc00000;
pub const KN02_SLOT_SIZE: u32 = 0x00080000;

/*
 * Address ranges decoded by the "system slot" logic for onboard devices.
 */
pub const KN02_SYS_ROM: u32 = 0 * KN02_SLOT_SIZE; /* system board ROM */
pub const KN02_RES_1: u32 = 1 * KN02_SLOT_SIZE; /* unused */
pub const KN02_CHKSYN: u32 = 2 * KN02_SLOT_SIZE; /* ECC syndrome */
pub const KN02_ERRADDR: u32 = 3 * KN02_SLOT_SIZE; /* bus error address */
pub const KN02_DZ11: u32 = 4 * KN02_SLOT_SIZE; /* DZ11 (DC7085) serial */
pub const KN02_RTC: u32 = 5 * KN02_SLOT_SIZE; /* DS1287 RTC */
pub const KN02_CSR: u32 = 6 * KN02_SLOT_SIZE; /* system ctrl & status reg */
pub const KN02_SYS_ROM_7: u32 = 7 * KN02_SLOT_SIZE; /* system board ROM (alias) */

/*
 * System Control & Status Register bits.
 */
pub const KN02_CSR_RES_28: u32 = 0xf << 28; /* unused */
pub const KN02_CSR_PSU: u32 = 1 << 27; /* power supply unit warning */
pub const KN02_CSR_NVRAM: u32 = 1 << 26; /* ~NVRAM clear jumper */
pub const KN02_CSR_REFEVEN: u32 = 1 << 25; /* mem refresh bank toggle */
pub const KN02_CSR_NRMOD: u32 = 1 << 24; /* ~NRMOD manufact. jumper */
pub const KN02_CSR_IOINTEN: u32 = 0xff << 16; /* IRQ mask bits */
pub const KN02_CSR_DIAGCHK: u32 = 1 << 15; /* diagn/norml ECC reads */
pub const KN02_CSR_DIAGGEN: u32 = 1 << 14; /* diagn/norml ECC writes */
pub const KN02_CSR_CORRECT: u32 = 1 << 13; /* ECC correct/check */
pub const KN02_CSR_LEDIAG: u32 = 1 << 12; /* ECC diagn. latch strobe */
pub const KN02_CSR_TXDIS: u32 = 1 << 11; /* DZ11 transmit disable */
pub const KN02_CSR_BNK32M: u32 = 1 << 10; /* 32M/8M stride */
pub const KN02_CSR_DIAGDN: u32 = 1 << 9; /* DIAGDN manufact. jumper */
pub const KN02_CSR_BAUD38: u32 = 1 << 8; /* DZ11 38/19kbps ext. rate */
pub const KN02_CSR_IOINT: u32 = 0xff << 0; /* IRQ status bits (r/o) */
pub const KN02_CSR_LEDS: u32 = 0xff << 0; /* ~diagnostic LEDs (w/o) */

/*
 * CPU interrupt bits.
 */
pub const KN02_CPU_INR_RES_6: u32 = 6; /* unused */
pub const KN02_CPU_INR_BUS: u32 = 5; /* memory, I/O bus read/write errors */
pub const KN02_CPU_INR_RES_4: u32 = 4; /* unused */
pub const KN02_CPU_INR_RTC: u32 = 3; /* DS1287 RTC */
pub const KN02_CPU_INR_CASCADE: u32 = 2; /* CSR cascade */

/*
 * CSR interrupt bits.
 */
pub const KN02_CSR_INR_DZ11: u32 = 7; /* DZ11 (DC7085) serial */
pub const KN02_CSR_INR_LANCE: u32 = 6; /* LANCE (Am7990) Ethernet */
pub const KN02_CSR_INR_ASC: u32 = 5; /* ASC (NCR53C94) SCSI */
pub const KN02_CSR_INR_RES_4: u32 = 4; /* unused */
pub const KN02_CSR_INR_RES_3: u32 = 3; /* unused */
pub const KN02_CSR_INR_TC2: u32 = 2; /* TURBOchannel slot #2 */
pub const KN02_CSR_INR_TC1: u32 = 1; /* TURBOchannel slot #1 */
pub const KN02_CSR_INR_TC0: u32 = 0; /* TURBOchannel slot #0 */

pub const KN02_IRQ_BASE: u32 = 8; /* first IRQ assigned to CSR */
pub const KN02_IRQ_LINES: u32 = 8; /* number of CSR interrupts */

#[inline]
pub const fn KN02_IRQ_NR(n: u32) -> u32 {
    n + KN02_IRQ_BASE
}

#[inline]
pub const fn KN02_IRQ_MASK(n: u32) -> u32 {
    1 << n
}

pub const KN02_IRQ_ALL: u32 = 0xff;

unsafe extern "C" {
    pub static mut cached_kn02_csr: u32;
    pub fn init_kn02_irqs(base: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
