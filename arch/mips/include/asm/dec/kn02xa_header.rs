/*
 * Hardware info common to DECstation 5000/1xx systems (otherwise
 * known as 3min or kn02ba) and Personal DECstations 5000/xx ones
 * (otherwise known as maxine or kn02ca).
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995,1996 by Paul M. Antoine, some code and definitions
 * are by courtesy of Chris Fraser.
 * Copyright (C) 2000, 2002, 2003, 2005  Maciej W. Rozycki
 *
 * These are addresses which have to be known early in the boot process.
 * For other addresses refer to tc.h, ioasic_addrs.h and friends.
 */

// Dependency supplied by <asm/dec/ioasic_addrs.h>.

pub const KN02XA_SLOT_BASE: u32 = 0x1c000000;

/*
 * Memory control ASIC registers.
 */
pub const KN02XA_MER: u32 = 0x0c400000; /* memory error register */
pub const KN02XA_MSR: u32 = 0x0c800000; /* memory size register */

/*
 * CPU control ASIC registers.
 */
pub const KN02XA_MEM_CONF: u32 = 0x0e000000; /* write timeout config */
pub const KN02XA_EAR: u32 = 0x0e000004; /* error address register */
pub const KN02XA_BOOT0: u32 = 0x0e000008; /* boot 0 register */
pub const KN02XA_MEM_INTR: u32 = 0x0e00000c; /* write err IRQ stat & ack */

/*
 * Memory Error Register bits, common definitions.
 * The rest is defined in system-specific headers.
 */
pub const KN02XA_MER_RES_28: u32 = 0xf << 28; /* unused */
pub const KN02XA_MER_RES_17: u32 = 0x3ff << 17; /* unused */
pub const KN02XA_MER_PAGERR: u32 = 1 << 16; /* 2k page boundary error */
pub const KN02XA_MER_TRANSERR: u32 = 1 << 15; /* transfer length error */
pub const KN02XA_MER_PARDIS: u32 = 1 << 14; /* parity error disable */
pub const KN02XA_MER_SIZE: u32 = 1 << 13; /* r/o mirror of MSR_SIZE */
pub const KN02XA_MER_RES_12: u32 = 1 << 12; /* unused */
pub const KN02XA_MER_BYTERR: u32 = 0xf << 8; /* byte lane error bitmask: */
pub const KN02XA_MER_BYTERR_3: u32 = 0x8 << 8; /* byte lane #3 */
pub const KN02XA_MER_BYTERR_2: u32 = 0x4 << 8; /* byte lane #2 */
pub const KN02XA_MER_BYTERR_1: u32 = 0x2 << 8; /* byte lane #1 */
pub const KN02XA_MER_BYTERR_0: u32 = 0x1 << 8; /* byte lane #0 */
pub const KN02XA_MER_RES_0: u32 = 0xff << 0; /* unused */

/*
 * Memory Size Register bits, common definitions.
 * The rest is defined in system-specific headers.
 */
pub const KN02XA_MSR_RES_27: u32 = 0x1f << 27; /* unused */
pub const KN02XA_MSR_RES_14: u32 = 0x7 << 14; /* unused */
pub const KN02XA_MSR_SIZE: u32 = 1 << 13; /* 16M/4M stride */
pub const KN02XA_MSR_RES_0: u32 = 0x1fff << 0; /* unused */

/*
 * Error Address Register bits.
 */
pub const KN02XA_EAR_RES_29: u32 = 0x7 << 29; /* unused */
pub const KN02XA_EAR_ADDRESS: u32 = 0x7ffffff << 2; /* address involved */
pub const KN02XA_EAR_RES_0: u32 = 0x3 << 0; /* unused */

// Dependency supplied by <linux/interrupt.h>.
pub type irqreturn_t = crate::irqreturn_t;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn dec_kn02xa_be_init();
    pub fn dec_kn02xa_be_handler(regs: *mut pt_regs, is_fixup: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn dec_kn02xa_be_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
