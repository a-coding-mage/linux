/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The original header is guarded by __ASSEMBLER__; the declarations below
 * correspond to the non-assembler portion of that conditional.
 */

pub type PtevalT = u64;
pub type PmdvalT = u64;
pub type PudvalT = u64;
pub type P4dvalT = u64;
pub type PgdvalT = u64;
pub type PgprotvalT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PteFields {
    pub pte_low: usize,
    pub pte_high: usize,
}

#[repr(C)]
pub union PteT {
    pub fields: PteFields,
    pub pte: PtevalT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PmdFields {
    pub pmd_low: usize,
    pub pmd_high: usize,
}

#[repr(C)]
pub union PmdT {
    pub fields: PmdFields,
    pub pmd: PmdvalT,
}

pub const ARCH_PAGE_TABLE_SYNC_MASK: u64 = PGTBL_PMD_MODIFIED;

/*
 * PGDIR_SHIFT determines what a top-level page table entry can map
 */
pub const PGDIR_SHIFT: u32 = 30;
pub const PTRS_PER_PGD: usize = 4;

/*
 * PMD_SHIFT determines the size of the area a middle-level
 * page table can map
 */
pub const PMD_SHIFT: u32 = 21;
pub const PTRS_PER_PMD: usize = 512;

/*
 * entries per page directory level
 */
pub const PTRS_PER_PTE: usize = 512;

pub const MAX_POSSIBLE_PHYSMEM_BITS: u32 = 36;
pub const PGD_KERNEL_START: usize = CONFIG_PAGE_OFFSET >> PGDIR_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
