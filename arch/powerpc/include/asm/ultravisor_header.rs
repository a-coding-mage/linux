/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Ultravisor definitions
 *
 * Copyright 2019, IBM Corporation.
 */

// Dependencies supplied by the corresponding PowerPC headers:
// asm-prototypes.h, ultravisor-api.h, and firmware.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    pub fn early_init_dt_scan_ultravisor(
        node: c_ulong,
        uname: *const c_char,
        depth: c_int,
        data: *mut c_void,
    ) -> c_int;

    fn firmware_has_feature(feature: c_ulong) -> c_int;
    fn mtspr(spr: c_ulong, val: u64);
    fn ucall_norets(op: u64, ...) -> c_int;
}

// In ultravisor enabled systems, PTCR becomes ultravisor privileged only for
// writing and an attempt to write to it will cause a Hypervisor Emulation
// Assistance interrupt.
pub unsafe fn set_ptcr_when_no_uv(val: u64) {
    if firmware_has_feature(FW_FEATURE_ULTRAVISOR) == 0 {
        mtspr(SPRN_PTCR, val);
    }
}

pub unsafe fn uv_register_pate(lpid: u64, dw0: u64, dw1: u64) -> c_int {
    ucall_norets(UV_WRITE_PATE, lpid, dw0, dw1)
}

pub unsafe fn uv_share_page(pfn: u64, npages: u64) -> c_int {
    ucall_norets(UV_SHARE_PAGE, pfn, npages)
}

pub unsafe fn uv_unshare_page(pfn: u64, npages: u64) -> c_int {
    ucall_norets(UV_UNSHARE_PAGE, pfn, npages)
}

pub unsafe fn uv_unshare_all_pages() -> c_int {
    ucall_norets(UV_UNSHARE_ALL_PAGES)
}

pub unsafe fn uv_page_in(
    lpid: u64,
    src_ra: u64,
    dst_gpa: u64,
    flags: u64,
    page_shift: u64,
) -> c_int {
    ucall_norets(UV_PAGE_IN, lpid, src_ra, dst_gpa, flags, page_shift)
}

pub unsafe fn uv_page_out(
    lpid: u64,
    dst_ra: u64,
    src_gpa: u64,
    flags: u64,
    page_shift: u64,
) -> c_int {
    ucall_norets(UV_PAGE_OUT, lpid, dst_ra, src_gpa, flags, page_shift)
}

pub unsafe fn uv_register_mem_slot(
    lpid: u64,
    start_gpa: u64,
    size: u64,
    flags: u64,
    slotid: u64,
) -> c_int {
    ucall_norets(UV_REGISTER_MEM_SLOT, lpid, start_gpa, size, flags, slotid)
}

pub unsafe fn uv_unregister_mem_slot(lpid: u64, slotid: u64) -> c_int {
    ucall_norets(UV_UNREGISTER_MEM_SLOT, lpid, slotid)
}

pub unsafe fn uv_page_inval(lpid: u64, gpa: u64, page_shift: u64) -> c_int {
    ucall_norets(UV_PAGE_INVAL, lpid, gpa, page_shift)
}

pub unsafe fn uv_svm_terminate(lpid: u64) -> c_int {
    ucall_norets(UV_SVM_TERMINATE, lpid)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
