/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: asm-generic/pgtable-nopmd.h

/*
 * traditional two-level paging structure
 */
pub const PAGETABLE_LEVELS: usize = 2;

/* PTE bits */
pub const PTE_MAGNITUDE: usize = 2; /* 32-bit PTEs */

pub const PTE_SHIFT: usize = PAGE_SHIFT;
pub const PTE_BITS: usize = PTE_SHIFT - PTE_MAGNITUDE;

/* PGD bits */
pub const PGDIR_SHIFT: usize = PTE_SHIFT + PTE_BITS;

pub const PTRS_PER_PGD: usize = PAGE_SIZE / (1usize << PTE_MAGNITUDE);
pub const USER_PTRS_PER_PGD: usize = TASK_SIZE / PGDIR_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
