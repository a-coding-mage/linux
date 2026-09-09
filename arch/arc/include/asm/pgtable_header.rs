/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Translated from asm/pgtable.h.
// Dependencies supplied by the surrounding kernel translation:
// TASK_SIZE, PGDIR_SIZE, PAGE_SIZE, and pgd_t.

/*
 * Number of entries a user land program use.
 * TASK_SIZE is the maximum vaddr that can be used by a userland program.
 */
pub const USER_PTRS_PER_PGD: usize = TASK_SIZE / PGDIR_SIZE;

/* C header content guarded by __ASSEMBLER__. */

// To cope with aliasing VIPT cache.
pub const HAVE_ARCH_UNMAPPED_AREA: bool = true;

// extern pgd_t swapper_pg_dir[] __aligned(PAGE_SIZE);
// The external array is page-aligned in the C declaration.
extern "C" {
    pub static mut swapper_pg_dir: [pgd_t; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
