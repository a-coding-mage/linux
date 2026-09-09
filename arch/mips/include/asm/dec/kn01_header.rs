/*
 * Hardware info about DECstation DS2100/3100 systems (otherwise known as
 * pmin/pmax or KN01).
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995,1996 by Paul M. Antoine, some code and definitions
 * are by courtesy of Chris Fraser.
 * Copyright (C) 2002, 2003, 2005  Maciej W. Rozycki
 */

pub const KN01_SLOT_BASE: u32 = 0x10000000;
pub const KN01_SLOT_SIZE: u32 = 0x01000000;

/*
 * Address ranges for devices.
 */
pub const KN01_PMASK: u32 = 0 * KN01_SLOT_SIZE; /* color plane mask */
pub const KN01_PCC: u32 = 1 * KN01_SLOT_SIZE; /* PCC (DC503) cursor */
pub const KN01_VDAC: u32 = 2 * KN01_SLOT_SIZE; /* color map */
pub const KN01_RES_3: u32 = 3 * KN01_SLOT_SIZE; /* unused */
pub const KN01_RES_4: u32 = 4 * KN01_SLOT_SIZE; /* unused */
pub const KN01_RES_5: u32 = 5 * KN01_SLOT_SIZE; /* unused */
pub const KN01_RES_6: u32 = 6 * KN01_SLOT_SIZE; /* unused */
pub const KN01_ERRADDR: u32 = 7 * KN01_SLOT_SIZE; /* write error address */
pub const KN01_LANCE: u32 = 8 * KN01_SLOT_SIZE; /* LANCE (Am7990) Ethernet */
pub const KN01_LANCE_MEM: u32 = 9 * KN01_SLOT_SIZE; /* LANCE buffer memory */
pub const KN01_SII: u32 = 10 * KN01_SLOT_SIZE; /* SII (DC7061) SCSI */
pub const KN01_SII_MEM: u32 = 11 * KN01_SLOT_SIZE; /* SII buffer memory */
pub const KN01_DZ11: u32 = 12 * KN01_SLOT_SIZE; /* DZ11 (DC7085) serial */
pub const KN01_RTC: u32 = 13 * KN01_SLOT_SIZE; /* DS1287 RTC (bytes #0) */
pub const KN01_ESAR: u32 = 13 * KN01_SLOT_SIZE; /* MAC address (bytes #1) */
pub const KN01_CSR: u32 = 14 * KN01_SLOT_SIZE; /* system ctrl & status reg */
pub const KN01_SYS_ROM: u32 = 15 * KN01_SLOT_SIZE; /* system board ROM */

/*
 * Frame buffer memory address.
 */
pub const KN01_VFB_MEM: u32 = 0x0fc00000;

/*
 * CPU interrupt bits.
 */
pub const KN01_CPU_INR_BUS: i32 = 6; /* memory, I/O bus read/write errors */
pub const KN01_CPU_INR_VIDEO: i32 = 6; /* PCC area detect #2 */
pub const KN01_CPU_INR_RTC: i32 = 5; /* DS1287 RTC */
pub const KN01_CPU_INR_DZ11: i32 = 4; /* DZ11 (DC7085) serial */
pub const KN01_CPU_INR_LANCE: i32 = 3; /* LANCE (Am7990) Ethernet */
pub const KN01_CPU_INR_SII: i32 = 2; /* SII (DC7061) SCSI */

/*
 * System Control & Status Register bits.
 */
pub const KN01_CSR_MNFMOD: u32 = 1 << 15; /* MNFMOD manufacturing jumper */
pub const KN01_CSR_STATUS: u32 = 1 << 14; /* self-test result status output */
pub const KN01_CSR_PARDIS: u32 = 1 << 13; /* parity error disable */
pub const KN01_CSR_CRSRTST: u32 = 1 << 12; /* PCC test output */
pub const KN01_CSR_MONO: u32 = 1 << 11; /* mono/color fb SIMM installed */
pub const KN01_CSR_MEMERR: u32 = 1 << 10; /* write timeout error status & ack*/
pub const KN01_CSR_VINT: u32 = 1 << 9; /* PCC area detect #2 status & ack */
pub const KN01_CSR_TXDIS: u32 = 1 << 8; /* DZ11 transmit disable */
pub const KN01_CSR_VBGTRG: u32 = 1 << 2; /* blue DAC voltage over green (r/o) */
pub const KN01_CSR_VRGTRG: u32 = 1 << 1; /* red DAC voltage over green (r/o) */
pub const KN01_CSR_VRGTRB: u32 = 1 << 0; /* red DAC voltage over blue (r/o) */
pub const KN01_CSR_LEDS: u32 = 0xff << 0; /* ~diagnostic LEDs (w/o) */

/* Included Linux declarations are external dependencies supplied elsewhere. */
pub struct pt_regs;

extern "C" {
    pub static mut cached_kn01_csr: u16;

    pub fn dec_kn01_be_init();
    pub fn dec_kn01_be_handler(regs: *mut pt_regs, is_fixup: i32) -> i32;
    pub fn dec_kn01_be_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
