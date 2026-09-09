/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by the corresponding page-table module.
use crate::kvm_pgtable::kvm_pgtable_mm_ops;

pub unsafe extern "C" fn hyp_early_alloc_init(
    virt: *mut core::ffi::c_void,
    size: core::ffi::c_ulong,
);

pub unsafe extern "C" fn hyp_early_alloc_nr_used_pages() -> core::ffi::c_ulong;

pub unsafe extern "C" fn hyp_early_alloc_page(
    arg: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void;

pub unsafe extern "C" fn hyp_early_alloc_contig(
    nr_pages: core::ffi::c_uint,
) -> *mut core::ffi::c_void;

extern "C" {
    pub static mut hyp_early_alloc_mm_ops: kvm_pgtable_mm_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
