/* SPDX-License-Identifier: GPL-2.0 */
/*
 * viking.h: Defines specific to the GNU/Viking MBUS module.
 *           This is SRMMU stuff.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependencies supplied by the corresponding architecture headers:
// asm/asi.h, asm/mxcc.h, asm/pgtable.h, and asm/pgtsrmmu.h.

pub const VIKING_MMUENABLE: u32 = 0x00000001;
pub const VIKING_NOFAULT: u32 = 0x00000002;
pub const VIKING_PSO: u32 = 0x00000080;
pub const VIKING_DCENABLE: u32 = 0x00000100; // Enable data cache
pub const VIKING_ICENABLE: u32 = 0x00000200; // Enable instruction cache
pub const VIKING_SBENABLE: u32 = 0x00000400; // Enable store buffer
pub const VIKING_MMODE: u32 = 0x00000800; // MBUS mode
pub const VIKING_PCENABLE: u32 = 0x00001000; // Enable parity checking
pub const VIKING_BMODE: u32 = 0x00002000;
pub const VIKING_SPENABLE: u32 = 0x00004000; // Enable bus cache snooping
pub const VIKING_ACENABLE: u32 = 0x00008000; // Enable alternate caching
pub const VIKING_TCENABLE: u32 = 0x00010000; // Enable table-walks to be cached
pub const VIKING_DPENABLE: u32 = 0x00040000; // Enable the data prefetcher

// GNU/Viking Breakpoint Action Register fields.
pub const VIKING_ACTION_MIX: u32 = 0x00001000; // Enable multiple instructions

// GNU/Viking Cache Tags.
pub const VIKING_PTAG_VALID: u32 = 0x01000000; // Cache block is valid
pub const VIKING_PTAG_DIRTY: u32 = 0x00010000; // Block has been modified
pub const VIKING_PTAG_SHARED: u32 = 0x00000100; // Shared with some other cache

#[inline]
pub unsafe fn viking_flush_icache() {
    core::arch::asm!("sta %g0, [%g0] {asi}", asi = const ASI_M_IC_FLCLEAR, options(nostack));
}

#[inline]
pub unsafe fn viking_flush_dcache() {
    core::arch::asm!("sta %g0, [%g0] {asi}", asi = const ASI_M_DC_FLCLEAR, options(nostack));
}

#[inline]
pub unsafe fn viking_unlock_icache() {
    let address: u64 = 0x80000000;
    core::arch::asm!("sta %g0, [{address}] {asi}", address = in(reg) address, asi = const ASI_M_IC_FLCLEAR, options(nostack));
}

#[inline]
pub unsafe fn viking_unlock_dcache() {
    let address: u64 = 0x80000000;
    core::arch::asm!("sta %g0, [{address}] {asi}", address = in(reg) address, asi = const ASI_M_DC_FLCLEAR, options(nostack));
}

#[inline]
pub unsafe fn viking_set_bpreg(regval: u64) {
    core::arch::asm!("sta {regval}, [%g0] {asi}", regval = in(reg) regval, asi = const ASI_M_ACTION, options(nostack));
}

#[inline]
pub unsafe fn viking_get_bpreg() -> u64 {
    let regval: u64;
    core::arch::asm!("lda [%g0] {asi}, {regval}", regval = out(reg) regval, asi = const ASI_M_ACTION, options(nostack));
    regval
}

#[inline]
pub unsafe fn viking_get_dcache_ptag(set: i32, block: i32, data: *mut u64) {
    let ptag = (((set & 0x7f) << 5) | ((block & 0x3) << 26)) as u64 | 0x80000000;
    let info: u64;
    let page: u64;
    core::arch::asm!(
        "ldda [{ptag}] {asi}, %g2",
        "or %g0, %g2, {info}",
        "or %g0, %g3, {page}",
        ptag = in(reg) ptag, info = out(reg) info, page = out(reg) page,
        asi = const ASI_M_DATAC_TAG, out("g2") _, out("g3") _, options(nostack)
    );
    *data.add(0) = info;
    *data.add(1) = page;
}

#[inline]
pub unsafe fn viking_mxcc_turn_off_parity(mregp: *mut u64, mxcc_cregp: *mut u64) {
    let mut mreg = *mregp;
    let mut mxcc_creg = *mxcc_cregp;
    mreg &= !(VIKING_PCENABLE as u64);
    mxcc_creg &= !(MXCC_CTL_PARE as u64);
    core::arch::asm!(
        "set 1f, %g2", "andcc %g2, 4, %g0", "bne 2f", " nop",
        "1:", "sta {mreg}, [%g0] {mmu}", "sta {creg}, [{addr}] {mxcc}",
        "b 1f", " nop", "nop", "2:",
        "sta {mreg}, [%g0] {mmu}", "sta {creg}, [{addr}] {mxcc}", "1:",
        mreg = in(reg) mreg, creg = in(reg) mxcc_creg, addr = in(reg) MXCC_CREG,
        mmu = const ASI_M_MMUREGS, mxcc = const ASI_M_MXCC,
        out("g2") _, options(nostack)
    );
    *mregp = mreg;
    *mxcc_cregp = mxcc_creg;
}

#[inline]
pub unsafe fn viking_hwprobe(mut vaddr: u64) -> u64 {
    let mut val: u64;
    vaddr &= PAGE_MASK as u64;
    core::arch::asm!("lda [{addr}] {asi}, {val}", addr = in(reg) vaddr | 0x400, val = out(reg) val, asi = const ASI_M_FLUSH_PROBE, options(nostack));
    if val == 0 { return 0; }
    core::arch::asm!("lda [{addr}] {asi}, {val}", addr = in(reg) vaddr | 0x200, val = out(reg) val, asi = const ASI_M_FLUSH_PROBE, options(nostack));
    if (val & SRMMU_ET_MASK as u64) == SRMMU_ET_PTE as u64 {
        vaddr &= !(PGDIR_MASK as u64); vaddr >>= PAGE_SHIFT;
        return val | (vaddr << 8);
    }
    core::arch::asm!("lda [{addr}] {asi}, {val}", addr = in(reg) vaddr | 0x100, val = out(reg) val, asi = const ASI_M_FLUSH_PROBE, options(nostack));
    if (val & SRMMU_ET_MASK as u64) == SRMMU_ET_PTE as u64 {
        vaddr &= !(PMD_MASK as u64); vaddr >>= PAGE_SHIFT;
        return val | (vaddr << 8);
    }
    core::arch::asm!("lda [{addr}] {asi}, {val}", addr = in(reg) vaddr, val = out(reg) val, asi = const ASI_M_FLUSH_PROBE, options(nostack));
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
