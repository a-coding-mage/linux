/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/*
 * These are actual hardware defined protection bits in the tlbacc register
 * which looks like this:
 *
 * 31 30 ... 26 25 24 23 22 21 20 19 18 ...  1  0
 * ignored........  C  R  W  X  G PFN............
 */
pub const _PAGE_GLOBAL: u32 = 1u32 << 20;
pub const _PAGE_EXEC: u32 = 1u32 << 21;
pub const _PAGE_WRITE: u32 = 1u32 << 22;
pub const _PAGE_READ: u32 = 1u32 << 23;
pub const _PAGE_CACHED: u32 = 1u32 << 24; /* C: data access cacheable */

/*
 * Software defined bits. They are ignored by the hardware and always read back
 * as zero, but can be written as non-zero.
 */
pub const _PAGE_PRESENT: u32 = 1u32 << 25; /* PTE contains a translation */
pub const _PAGE_ACCESSED: u32 = 1u32 << 26; /* page referenced */
pub const _PAGE_DIRTY: u32 = 1u32 << 27; /* dirty page */

/* We borrow bit 31 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: u32 = 1u32 << 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
