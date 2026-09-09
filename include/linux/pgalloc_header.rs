/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding page-table and architecture
// layers are intentionally left external to this translation.

/*
 * {pgd,p4d}_populate_kernel() are defined as macros to allow
 * compile-time optimization based on the configured page table levels.
 * Without this, linking may fail because callers (e.g., KASAN) may rely
 * on calls to these functions being optimized away when passing symbols
 * that exist only for certain page table levels.
 */
macro_rules! pgd_populate_kernel {
    ($addr:expr, $pgd:expr, $p4d:expr) => {{
        pgd_populate(&init_mm, $pgd, $p4d);
        if ARCH_PAGE_TABLE_SYNC_MASK & PGTBL_PGD_MODIFIED != 0 {
            arch_sync_kernel_mappings($addr, $addr);
        }
    }};
}

macro_rules! p4d_populate_kernel {
    ($addr:expr, $p4d:expr, $pud:expr) => {{
        p4d_populate(&init_mm, $p4d, $pud);
        if ARCH_PAGE_TABLE_SYNC_MASK & PGTBL_P4D_MODIFIED != 0 {
            arch_sync_kernel_mappings($addr, $addr);
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
