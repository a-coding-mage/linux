/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_ulong;

extern "C" {
    pub fn generic_max_swapfile_size() -> c_ulong;
    pub fn arch_max_swapfile_size() -> c_ulong;

    /* Maximum swapfile size supported for the arch (not inclusive). */
    pub static mut swapfile_maximum_size: c_ulong;

    /* Whether swap migration entry supports storing A/D bits for the arch */
    pub static mut swap_migration_ad_supported: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
