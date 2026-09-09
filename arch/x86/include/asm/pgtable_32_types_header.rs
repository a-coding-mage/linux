/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The Linux x86 paging architecture is 'compile-time dual-mode', it
 * implements both the traditional 2-level x86 page tables and the
 * newer 3-level PAE-mode page tables.
 *
 * The original header includes either pgtable-3level_types.h or
 * pgtable-2level_types.h depending on CONFIG_X86_PAE. Those dependencies
 * are supplied externally.
 */
#[cfg(feature = "CONFIG_X86_PAE")]
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;

#[cfg(feature = "CONFIG_X86_PAE")]
pub const PMD_MASK: usize = !(PMD_SIZE - 1usize);

pub const fn pgtable_l5_enabled() -> i32 {
    0
}

pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1usize);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
