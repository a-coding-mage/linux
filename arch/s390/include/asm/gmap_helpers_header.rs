/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Helper functions for KVM guest address space mapping code
 *
 *    Copyright IBM Corp. 2025
 */

use core::ffi::c_ulong;

extern "C" {
    pub fn gmap_helper_zap_one_page(mm: *mut mm_struct, vmaddr: c_ulong);
    pub fn gmap_helper_discard(
        mm: *mut mm_struct,
        vmaddr: c_ulong,
        end: c_ulong,
    );
    pub fn gmap_helper_disable_cow_sharing() -> i32;
    pub fn gmap_helper_try_set_pte_unused(mm: *mut mm_struct, vmaddr: c_ulong);
    pub fn try_get_locked_pte(
        mm: *mut mm_struct,
        addr: c_ulong,
        ptl: *mut *mut spinlock_t,
    ) -> *mut pte_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
