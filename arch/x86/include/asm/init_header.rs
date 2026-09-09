/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

#[repr(C)]
pub struct x86_mapping_info {
    pub alloc_pgt_page: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    /* allocate buf for page table */
    pub free_pgt_page: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    /* free buf for page table */
    pub context: *mut c_void,
    /* context for alloc_pgt_page */
    pub page_flag: c_ulong,
    /* page flag for PMD or PUD entry */
    pub offset: c_ulong,
    /* ident mapping offset */
    pub direct_gbpages: bool,
    /* PUD level 1GB page support */
    pub kernpg_flag: c_ulong,
    /* kernel pagetable flag override */
}

extern "C" {
    pub fn kernel_ident_mapping_init(
        info: *mut x86_mapping_info,
        pgd_page: *mut pgd_t,
        pstart: c_ulong,
        pend: c_ulong,
    ) -> c_int;

    pub fn kernel_ident_mapping_free(info: *mut x86_mapping_info, pgd: *mut pgd_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
