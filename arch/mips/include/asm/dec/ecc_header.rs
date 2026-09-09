/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/dec/ecc.h
 *
 *	ECC handling logic definitions common to DECstation/DECsystem
 *	5000/200 (KN02), 5000/240 (KN03), 5000/260 (KN05) and
 *	DECsystem 5900 (KN03), 5900/260 (KN05) systems.
 *
 *	Copyright (C) 2003  Maciej W. Rozycki
 */

/*
 * Error Address Register bits.
 * The register is r/wc -- any write clears it.
 */
pub const KN0X_EAR_VALID: u32 = 1 << 31; /* error data valid, bus IRQ */
pub const KN0X_EAR_CPU: u32 = 1 << 30; /* CPU/DMA transaction */
pub const KN0X_EAR_WRITE: u32 = 1 << 29; /* write/read transaction */
pub const KN0X_EAR_ECCERR: u32 = 1 << 28; /* ECC/timeout or overrun */
pub const KN0X_EAR_RES_27: u32 = 1 << 27; /* unused */
pub const KN0X_EAR_ADDRESS: u32 = 0x7ffffff << 0; /* address involved */

/*
 * Error Syndrome Register bits.
 * The register is frozen when EAR.VALID is set, otherwise it records bits
 * from the last memory read.  The register is r/wc -- any write clears it.
 */
pub const KN0X_ESR_VLDHI: u32 = 1 << 31; /* error data valid hi word */
pub const KN0X_ESR_CHKHI: u32 = 0x7f << 24; /* check bits read from mem */
pub const KN0X_ESR_SNGHI: u32 = 1 << 23; /* single/double bit error */
pub const KN0X_ESR_SYNHI: u32 = 0x7f << 16; /* syndrome from ECC logic */
pub const KN0X_ESR_VLDLO: u32 = 1 << 15; /* error data valid lo word */
pub const KN0X_ESR_CHKLO: u32 = 0x7f << 8; /* check bits read from mem */
pub const KN0X_ESR_SNGLO: u32 = 1 << 7; /* single/double bit error */
pub const KN0X_ESR_SYNLO: u32 = 0x7f << 0; /* syndrome from ECC logic */

/* Supplied by the Linux interrupt definitions and other dependencies. */
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn dec_ecc_be_init();
    pub fn dec_ecc_be_handler(regs: *mut pt_regs, is_fixup: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn dec_ecc_be_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
}

/* Defined by <linux/interrupt.h>. */
pub type irqreturn_t = ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
