/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// Dependencies supplied by the surrounding translation unit:
// asm-generic/pgtable-nopmd.h, linux/bits.h, and linux/const.h.

/* Size of region mapped by a page global directory */
pub const PGDIR_SHIFT: u32 = 22;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

pub const MAX_POSSIBLE_PHYSMEM_BITS: u32 = 34;

/*
 * rv32 PTE format:
 * | XLEN-1  10 | 9             8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0
 *       PFN      reserved for SW   D   A   G   U   X   W   R   V
 */
pub const _PAGE_PFN_MASK: u32 = 0xffff_fc00;

pub const _PAGE_NOCACHE: u32 = 0;
pub const _PAGE_IO: u32 = 0;
pub const _PAGE_MTMASK: u32 = 0;

/* Set of bits to preserve across pte_modify() */
pub const _PAGE_CHG_MASK: usize = !(_PAGE_PRESENT as usize
    | _PAGE_READ as usize
    | _PAGE_WRITE as usize
    | _PAGE_EXEC as usize
    | _PAGE_USER as usize
    | _PAGE_GLOBAL as usize);

static pgtable_l4_enabled: i32 = 0;
static pgtable_l5_enabled: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
