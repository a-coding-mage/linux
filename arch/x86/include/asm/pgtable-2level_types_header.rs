/* SPDX-License-Identifier: GPL-2.0 */

/* The original header guard is omitted from executable Rust. */

/* These declarations are available only when not compiling as an assembler. */
pub type PtevalT = usize;
pub type PmdvalT = usize;
pub type PudvalT = usize;
pub type P4dvalT = usize;
pub type PgdvalT = usize;
pub type PgprotvalT = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub union PteT {
    pub pte: PtevalT,
    pub pte_low: PtevalT,
}

pub const ARCH_PAGE_TABLE_SYNC_MASK: usize = PGTBL_PMD_MODIFIED;

/*
 * Traditional i386 two-level paging structure:
 */
pub const PGDIR_SHIFT: usize = 22;
pub const PTRS_PER_PGD: usize = 1024;

/*
 * The i386 is two-level, so we don't really have any
 * PMD directory physically:
 */
pub const PTRS_PER_PMD: usize = 1;

pub const PTRS_PER_PTE: usize = 1024;

/* This covers all VMSPLIT_* and VMSPLIT_*_OPT variants */
pub const PGD_KERNEL_START: usize = CONFIG_PAGE_OFFSET >> PGDIR_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
