/* SPDX-License-Identifier: GPL-2.0 */
/* swift.h: Specific definitions for the _broken_ Swift SRMMU
 *          MMU module.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

/* Swift is so brain damaged, here is the mmu control register. */
pub const SWIFT_ST: usize = 0x00800000; /* SW tablewalk enable */
pub const SWIFT_WP: usize = 0x00400000; /* Watchpoint enable   */

/* Branch folding (buggy, disable on production systems!)  */
pub const SWIFT_BF: usize = 0x00200000;
pub const SWIFT_PMC: usize = 0x00180000; /* Page mode control   */
pub const SWIFT_PE: usize = 0x00040000; /* Parity enable       */
pub const SWIFT_PC: usize = 0x00020000; /* Parity control      */
pub const SWIFT_AP: usize = 0x00010000; /* Graphics page mode control (TCX/SX) */
pub const SWIFT_AC: usize = 0x00008000; /* Alternate Cacheability (see viking.h) */
pub const SWIFT_BM: usize = 0x00004000; /* Boot mode */
pub const SWIFT_RC: usize = 0x00003c00; /* DRAM refresh control */
pub const SWIFT_IE: usize = 0x00000200; /* Instruction cache enable */
pub const SWIFT_DE: usize = 0x00000100; /* Data cache enable */
pub const SWIFT_SA: usize = 0x00000080; /* Store Allocate */
pub const SWIFT_NF: usize = 0x00000002; /* No fault mode */
pub const SWIFT_EN: usize = 0x00000001; /* MMU enable */

/* Bits [13:5] select one of 512 instruction cache tags */
#[inline]
pub unsafe fn swift_inv_insn_tag(addr: usize) {
    core::arch::asm!("sta %g0, [{0}] {1}", in(reg) addr, const ASI_M_TXTC_TAG, options(nostack));
}

/* Bits [12:4] select one of 512 data cache tags */
#[inline]
pub unsafe fn swift_inv_data_tag(addr: usize) {
    core::arch::asm!("sta %g0, [{0}] {1}", in(reg) addr, const ASI_M_DATAC_TAG, options(nostack));
}

#[inline]
pub unsafe fn swift_flush_dcache() {
    let mut addr: usize = 0;
    while addr < 0x2000 {
        swift_inv_data_tag(addr);
        addr = addr.wrapping_add(0x10);
    }
}

#[inline]
pub unsafe fn swift_flush_icache() {
    let mut addr: usize = 0;
    while addr < 0x4000 {
        swift_inv_insn_tag(addr);
        addr = addr.wrapping_add(0x20);
    }
}

#[inline]
pub unsafe fn swift_idflash_clear() {
    let mut addr: usize = 0;
    while addr < 0x2000 {
        swift_inv_insn_tag(addr << 1);
        swift_inv_data_tag(addr);
        addr = addr.wrapping_add(0x10);
    }
}

/* Swift is so broken, it isn't even safe to use the following. */
#[inline]
pub unsafe fn swift_flush_page(page: usize) {
    core::arch::asm!("sta %g0, [{0}] {1}", in(reg) page, const ASI_M_FLUSH_PAGE, options(nostack));
}

#[inline]
pub unsafe fn swift_flush_segment(addr: usize) {
    core::arch::asm!("sta %g0, [{0}] {1}", in(reg) addr, const ASI_M_FLUSH_SEG, options(nostack));
}

#[inline]
pub unsafe fn swift_flush_region(addr: usize) {
    core::arch::asm!("sta %g0, [{0}] {1}", in(reg) addr, const ASI_M_FLUSH_REGION, options(nostack));
}

#[inline]
pub unsafe fn swift_flush_context() {
    core::arch::asm!("sta %g0, [%g0] {0}", const ASI_M_FLUSH_CTX, options(nostack));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
