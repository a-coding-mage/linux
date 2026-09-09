/* SPDX-License-Identifier: GPL-2.0 */

/* implemented in software */
pub const _PAGE_ACCESSED: i32 = 1 << 7;
pub const _PAGE_READ: i32 = 1 << 8;
pub const _PAGE_WRITE: i32 = 1 << 9;
pub const _PAGE_PRESENT: i32 = 1 << 10;
pub const _PAGE_MODIFIED: i32 = 1 << 11;

/* We borrow bit 7 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: i32 = 1 << 7;

/* implemented in hardware */
pub const _PAGE_GLOBAL: i32 = 1 << 0;
pub const _PAGE_VALID: i32 = 1 << 1;
pub const _PAGE_DIRTY: i32 = 1 << 2;

pub const _PAGE_SO: i32 = 1 << 5;
pub const _PAGE_BUF: i32 = 1 << 6;
pub const _PAGE_CACHE: i32 = 1 << 3;
pub const _CACHE_MASK: i32 = _PAGE_CACHE;

pub const _CACHE_CACHED: i32 = _PAGE_CACHE | _PAGE_BUF;
pub const _CACHE_UNCACHED: i32 = 0;

pub const _PAGE_PROT_NONE: i32 = _PAGE_WRITE;

/*
 * Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
 * are !pte_none() && !pte_present().
 *
 * Format of swap PTE:
 *     bit          0:    _PAGE_GLOBAL (zero)
 *     bit          1:    _PAGE_VALID (zero)
 *     bit      2 - 6:    swap type
 *     bit          7:    exclusive marker
 *     bit          8:    swap offset[0]
 *     bit          9:    _PAGE_WRITE (zero)
 *     bit         10:    _PAGE_PRESENT (zero)
 *     bit    11 - 31:    swap offset[1 - 21]
 */
#[macro_export]
macro_rules! __swp_type {
    ($x:expr) => {
        (($x.val >> 2) & 0x1f)
    };
}

#[macro_export]
macro_rules! __swp_offset {
    ($x:expr) => {
        ((($x.val >> 8) & 0x1) | (($x.val >> 10) & 0x3ffffe))
    };
}

#[macro_export]
macro_rules! __swp_entry {
    ($type:expr, $offset:expr) => {
        swp_entry_t {
            val: (($type & 0x1f) << 2)
                | (($offset & 0x1) << 8)
                | (($offset & 0x3ffffe) << 10),
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
