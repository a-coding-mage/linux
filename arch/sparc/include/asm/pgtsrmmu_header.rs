/* SPDX-License-Identifier: GPL-2.0 */
/*
 * pgtsrmmu.h: SRMMU page table defines and code.
 *
 * Rust translation of the C header. C preprocessor inclusion and assembler
 * conditions are preserved here as comments; dependent symbols are external.
 */

// Number of contexts is implementation-dependent; 64k is the most we support.
pub const SRMMU_MAX_CONTEXTS: u32 = 65536;

// PTRS_PER_PTE, PTRS_PER_PMD, and PTRS_PER_PGD are supplied externally.
pub const SRMMU_PTE_TABLE_SIZE: usize = PTRS_PER_PTE * 4;
pub const SRMMU_PMD_TABLE_SIZE: usize = PTRS_PER_PMD * 4;
pub const SRMMU_PGD_TABLE_SIZE: usize = PTRS_PER_PGD * 4;

pub const SRMMU_ET_MASK: u32 = 0x3;
pub const SRMMU_ET_INVALID: u32 = 0x0;
pub const SRMMU_ET_PTD: u32 = 0x1;
pub const SRMMU_ET_PTE: u32 = 0x2;
pub const SRMMU_ET_REPTE: u32 = 0x3; // AIEEE, SuperSparc II reverse endian page!

pub const SRMMU_CTX_PMASK: u32 = 0xfffffff0;
pub const SRMMU_PTD_PMASK: u32 = 0xfffffff0;
pub const SRMMU_PTE_PMASK: u32 = 0xffffff00;

pub const SRMMU_CACHE: u32 = 0x80;
pub const SRMMU_DIRTY: u32 = 0x40;
pub const SRMMU_REF: u32 = 0x20;
pub const SRMMU_NOREAD: u32 = 0x10;
pub const SRMMU_EXEC: u32 = 0x08;
pub const SRMMU_WRITE: u32 = 0x04;
pub const SRMMU_VALID: u32 = 0x02; // SRMMU_ET_PTE
pub const SRMMU_PRIV: u32 = 0x1c;
pub const SRMMU_PRIV_RDONLY: u32 = 0x18;

pub const SRMMU_CHG_MASK: u32 = 0xffffff00 | SRMMU_REF | SRMMU_DIRTY;

pub const SRMMU_SWP_TYPE_MASK: u32 = 0x1f;
pub const SRMMU_SWP_TYPE_SHIFT: u32 = 7;
pub const SRMMU_SWP_OFF_MASK: u32 = 0xfffff;
pub const SRMMU_SWP_OFF_SHIFT: u32 = SRMMU_SWP_TYPE_SHIFT + 5;
// We borrow bit 6 to store the exclusive marker in swap PTEs.
pub const SRMMU_SWP_EXCLUSIVE: u32 = SRMMU_DIRTY;

// __pgprot is supplied externally and returns the platform protection type.
macro_rules! SRMMU_PAGE_NONE { () => { __pgprot(SRMMU_CACHE | SRMMU_PRIV | SRMMU_REF) }; }
macro_rules! SRMMU_PAGE_SHARED { () => { __pgprot(SRMMU_VALID | SRMMU_CACHE | SRMMU_EXEC | SRMMU_WRITE | SRMMU_REF) }; }
macro_rules! SRMMU_PAGE_COPY { () => { __pgprot(SRMMU_VALID | SRMMU_CACHE | SRMMU_EXEC | SRMMU_REF) }; }
macro_rules! SRMMU_PAGE_RDONLY { () => { __pgprot(SRMMU_VALID | SRMMU_CACHE | SRMMU_EXEC | SRMMU_REF) }; }
macro_rules! SRMMU_PAGE_KERNEL { () => { __pgprot(SRMMU_VALID | SRMMU_CACHE | SRMMU_PRIV | SRMMU_DIRTY | SRMMU_REF) }; }

pub const SRMMU_CTRL_REG: u32 = 0x00000000;
pub const SRMMU_CTXTBL_PTR: u32 = 0x00000100;
pub const SRMMU_CTX_REG: u32 = 0x00000200;
pub const SRMMU_FAULT_STATUS: u32 = 0x00000300;
pub const SRMMU_FAULT_ADDR: u32 = 0x00000400;

// Assembler-only WINDOW_FLUSH(tmp1, tmp2) macro is not executable Rust.
// It expands to the original SPARC register-window flush sequence.

#[cfg(not(feature = "assembler"))]
extern "C" {
    pub static mut last_valid_pfn: core::ffi::c_ulong;
    pub static mut srmmu_nocache_pool: *mut core::ffi::c_void;

    pub fn srmmu_get_mmureg() -> core::ffi::c_uint;
    pub fn srmmu_set_mmureg(regval: core::ffi::c_ulong);
    pub fn srmmu_set_ctable_ptr(paddr: core::ffi::c_ulong);
    pub fn srmmu_set_context(context: core::ffi::c_int);
    pub fn srmmu_get_context() -> core::ffi::c_int;
    pub fn srmmu_get_fstatus() -> core::ffi::c_uint;
    pub fn srmmu_get_faddr() -> core::ffi::c_uint;
}

// __nocache_pa, __nocache_va, and __nocache_fix retain the C macros' intent;
// __pa, __va, SRMMU_NOCACHE_VADDR, and the original expression type are external.

#[cfg(not(feature = "assembler"))]
pub unsafe fn srmmu_flush_whole_tlb() {
    // Original inline assembly: sta %%g0, [%0] %1, with address 0x400 and
    // ASI_M_FLUSH_PROBE, flushing the entire TLB.
    core::arch::asm!("sta %g0, [%0] %1", in(reg) 0x400u32, const ASI_M_FLUSH_PROBE);
}

#[cfg(not(feature = "assembler"))]
pub unsafe fn srmmu_get_pte(addr: core::ffi::c_ulong) -> core::ffi::c_ulong {
    // Original inline assembly performs lda[((addr & 0xfffff000) | 0x400)]
    // using ASI_M_FLUSH_PROBE and returns the loaded entry.
    let mut entry: core::ffi::c_ulong;
    core::arch::asm!("lda [%1] %2, %0", out(reg) entry, in(reg) (addr & 0xfffff000) | 0x400, const ASI_M_FLUSH_PROBE);
    entry
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
