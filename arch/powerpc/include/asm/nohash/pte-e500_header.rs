/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of the C header.  The original __KERNEL__ and
 * __ASSEMBLER__ build conditions are preserved as comments; this file
 * contains the kernel-side, non-assembler declarations.
 */

/* PTE bit definitions for processors compliant to the Book3E
 * architecture 2.06 or later. The position of the PTE bits
 * matches the HW definition of the optional Embedded Page Table
 * category.
 */

/* Architected bits */
pub const _PAGE_PRESENT: u64 = 0x000001; // software: pte contains a translation
pub const _PAGE_SW1: u64 = 0x000002;
pub const _PAGE_BAP_SR: u64 = 0x000004;
pub const _PAGE_BAP_UR: u64 = 0x000008;
pub const _PAGE_BAP_SW: u64 = 0x000010;
pub const _PAGE_BAP_UW: u64 = 0x000020;
pub const _PAGE_BAP_SX: u64 = 0x000040;
pub const _PAGE_BAP_UX: u64 = 0x000080;
pub const _PAGE_PSIZE_MSK: u64 = 0x000f00;
pub const _PAGE_TSIZE_4K: u64 = 0x000100;
pub const _PAGE_DIRTY: u64 = 0x001000; // C: page changed
pub const _PAGE_SW0: u64 = 0x002000;
pub const _PAGE_U3: u64 = 0x004000;
pub const _PAGE_U2: u64 = 0x008000;
pub const _PAGE_U1: u64 = 0x010000;
pub const _PAGE_U0: u64 = 0x020000;
pub const _PAGE_ACCESSED: u64 = 0x040000;
pub const _PAGE_ENDIAN: u64 = 0x080000;
pub const _PAGE_GUARDED: u64 = 0x100000;
pub const _PAGE_COHERENT: u64 = 0x200000; // M: enforce memory coherence
pub const _PAGE_NO_CACHE: u64 = 0x400000; // I: cache inhibit
pub const _PAGE_WRITETHRU: u64 = 0x800000; // W: cache write-through

pub const _PAGE_PSIZE_SHIFT: u32 = 7;
pub const _PAGE_PSIZE_SHIFT_OFFSET: u32 = 10;

/* "Higher level" linux bit combinations */
pub const _PAGE_EXEC: u64 = _PAGE_BAP_SX | _PAGE_BAP_UX; // .. and was cache cleaned
pub const _PAGE_READ: u64 = _PAGE_BAP_SR | _PAGE_BAP_UR; // User read permission
pub const _PAGE_WRITE: u64 = _PAGE_BAP_SW | _PAGE_BAP_UW; // User write permission

pub const _PAGE_KERNEL_RW: u64 = _PAGE_BAP_SW | _PAGE_BAP_SR | _PAGE_DIRTY;
pub const _PAGE_KERNEL_RO: u64 = _PAGE_BAP_SR;
pub const _PAGE_KERNEL_RWX: u64 = _PAGE_BAP_SW | _PAGE_BAP_SR | _PAGE_DIRTY | _PAGE_BAP_SX;
pub const _PAGE_KERNEL_ROX: u64 = _PAGE_BAP_SR | _PAGE_BAP_SX;

pub const _PAGE_NA: u64 = 0;
pub const _PAGE_NAX: u64 = _PAGE_BAP_UX;
pub const _PAGE_RO: u64 = _PAGE_READ;
pub const _PAGE_ROX: u64 = _PAGE_READ | _PAGE_BAP_UX;
pub const _PAGE_RW: u64 = _PAGE_READ | _PAGE_WRITE;
pub const _PAGE_RWX: u64 = _PAGE_READ | _PAGE_WRITE | _PAGE_BAP_UX;

pub const _PAGE_SPECIAL: u64 = _PAGE_SW0;

pub const PTE_RPN_SHIFT: u32 = 24;
pub const PTE_WIMGE_SHIFT: u32 = 19;
pub const PTE_BAP_SHIFT: u32 = 2;

/* On 32-bit, we never clear the top part of the PTE. */
/* CONFIG_PPC32: _PTE_NONE_MASK = 0xffffffff00000000ULL;
 * _PMD_PRESENT = 0; _PMD_PRESENT_MASK = PAGE_MASK;
 * _PMD_BAD = !PAGE_MASK; _PMD_USER = 0;
 */
#[cfg(not(CONFIG_PPC32))]
pub const _PTE_NONE_MASK: u64 = 0;

/*
 * We define 2 sets of base prot bits, one for basic pages (ie,
 * cacheable kernel and user pages) and one for non cacheable
 * pages. We always set _PAGE_COHERENT when SMP is enabled or
 * the processor might need it for DMA coherency.
 */
pub const _PAGE_BASE_NC: u64 = _PAGE_PRESENT | _PAGE_ACCESSED | _PAGE_TSIZE_4K;
/* CONFIG_SMP selects the coherent variant. */
#[cfg(CONFIG_SMP)]
pub const _PAGE_BASE: u64 = _PAGE_BASE_NC | _PAGE_COHERENT;
#[cfg(not(CONFIG_SMP))]
pub const _PAGE_BASE: u64 = _PAGE_BASE_NC;

/* Dependency supplied by asm/pgtable-masks.h. */

extern "C" {
    pub fn pte_val(pte: pte_t) -> pte_basic_t;
    pub fn __pte(value: pte_basic_t) -> pte_t;
    pub fn pmd_val(pmd: pmd_t) -> pte_basic_t;
    pub fn pud_val(pud: pud_t) -> pte_basic_t;
}

pub type pte_basic_t = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    pub val: pte_basic_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t {
    pub val: pte_basic_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pud_t {
    pub val: pte_basic_t,
}

pub unsafe fn pte_mkexec(pte: pte_t) -> pte_t {
    __pte((pte_val(pte) & !_PAGE_BAP_SX) | _PAGE_BAP_UX)
}

pub unsafe fn pte_huge_size(pte: pte_t) -> usize {
    1usize << (((pte_val(pte) & _PAGE_PSIZE_MSK) >> _PAGE_PSIZE_SHIFT)
        + _PAGE_PSIZE_SHIFT_OFFSET as u64)
}

pub unsafe fn pmd_leaf(pmd: pmd_t) -> i32 {
    /* IS_ENABLED(CONFIG_PPC64) is supplied by the build configuration. */
    if cfg!(CONFIG_PPC64) {
        (pmd_val(pmd) as i64 > 0) as i32
    } else {
        (pmd_val(pmd) & _PAGE_PSIZE_MSK) as i32
    }
}

pub unsafe fn pmd_leaf_size(pmd: pmd_t) -> usize {
    pte_huge_size(__pte(pmd_val(pmd)))
}

#[cfg(CONFIG_PPC64)]
pub unsafe fn pud_leaf(pud: pud_t) -> i32 {
    if cfg!(CONFIG_PPC64) {
        (pud_val(pud) as i64 > 0) as i32
    } else {
        (pud_val(pud) & _PAGE_PSIZE_MSK) as i32
    }
}

#[cfg(CONFIG_PPC64)]
pub unsafe fn pud_leaf_size(pud: pud_t) -> usize {
    pte_huge_size(__pte(pud_val(pud)))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
