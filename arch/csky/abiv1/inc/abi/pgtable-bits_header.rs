/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: __ASM_CSKY_PGTABLE_BITS_H

/* implemented in software */
pub const _PAGE_PRESENT: u32 = 1 << 0;
pub const _PAGE_READ: u32 = 1 << 1;
pub const _PAGE_WRITE: u32 = 1 << 2;
pub const _PAGE_ACCESSED: u32 = 1 << 3;
pub const _PAGE_MODIFIED: u32 = 1 << 4;

/* We borrow bit 9 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: u32 = 1 << 9;

/* implemented in hardware */
pub const _PAGE_GLOBAL: u32 = 1 << 6;
pub const _PAGE_VALID: u32 = 1 << 7;
pub const _PAGE_DIRTY: u32 = 1 << 8;

pub const _PAGE_CACHE: u32 = 3 << 9;
pub const _PAGE_UNCACHE: u32 = 2 << 9;
pub const _PAGE_SO: u32 = _PAGE_UNCACHE;
pub const _CACHE_MASK: u32 = 7 << 9;

pub const _CACHE_CACHED: u32 = _PAGE_CACHE;
pub const _CACHE_UNCACHED: u32 = _PAGE_UNCACHE;

pub const _PAGE_PROT_NONE: u32 = _PAGE_READ;

/*
 * Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
 * are !pte_none() && !pte_present().
 *
 * Format of swap PTE:
 *     bit          0:    _PAGE_PRESENT (zero)
 *     bit          1:    _PAGE_READ (zero)
 *     bit      2 - 5:    swap type[0 - 3]
 *     bit          6:    _PAGE_GLOBAL (zero)
 *     bit          7:    _PAGE_VALID (zero)
 *     bit          8:    swap type[4]
 *     bit          9:    exclusive marker
 *     bit    10 - 31:    swap offset
 */
#[macro_export]
macro_rules! __swp_type {
    ($x:expr) => {
        (((($x).val >> 2) & 0xf) | ((($x).val >> 4) & 0x10))
    };
}

#[macro_export]
macro_rules! __swp_offset {
    ($x:expr) => {
        (($x).val >> 10)
    };
}

#[macro_export]
macro_rules! __swp_entry {
    ($type:expr, $offset:expr) => {
        (swp_entry_t {
            val: (($type & 0xf) << 2) |
                (($type & 0x10) << 4) |
                (($offset) << 10),
        })
    };
}

// HAVE_ARCH_UNMAPPED_AREA

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
