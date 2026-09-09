// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google LLC
 * Author: Quentin Perret <qperret@google.com>
 */

// Types and symbols supplied by the corresponding kernel headers are external
// dependencies of this translation.

extern "C" {
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn hyp_phys_to_virt(phys: u64) -> *mut core::ffi::c_void;
    fn hyp_virt_to_phys(virt: *const core::ffi::c_void) -> u64;
}

pub static mut hyp_early_alloc_mm_ops: crate::kvm_pgtable_mm_ops = unsafe { core::mem::zeroed() };
pub static mut hyp_physvirt_offset: i64 = 0;

static mut base: usize = 0;
static mut end: usize = 0;
static mut cur: usize = 0;

pub unsafe extern "C" fn hyp_early_alloc_nr_used_pages() -> usize {
    (cur.wrapping_sub(base)) >> crate::PAGE_SHIFT
}

pub unsafe extern "C" fn hyp_early_alloc_contig(nr_pages: u32) -> *mut core::ffi::c_void {
    let size = (nr_pages as usize).wrapping_shl(crate::PAGE_SHIFT as u32);
    let ret = cur as *mut core::ffi::c_void;

    if nr_pages == 0 {
        return core::ptr::null_mut();
    }

    if end.wrapping_sub(cur) < size {
        return core::ptr::null_mut();
    }

    cur = cur.wrapping_add(size);
    memset(ret, 0, size);

    ret
}

pub unsafe extern "C" fn hyp_early_alloc_page(
    _arg: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    hyp_early_alloc_contig(1)
}

unsafe extern "C" fn hyp_early_alloc_get_page(_addr: *mut core::ffi::c_void) {}
unsafe extern "C" fn hyp_early_alloc_put_page(_addr: *mut core::ffi::c_void) {}

pub unsafe extern "C" fn hyp_early_alloc_init(virt: *mut core::ffi::c_void, size: usize) {
    base = virt as usize;
    cur = base;
    end = base.wrapping_add(size);

    hyp_early_alloc_mm_ops.zalloc_page = Some(hyp_early_alloc_page);
    hyp_early_alloc_mm_ops.phys_to_virt = Some(hyp_phys_to_virt);
    hyp_early_alloc_mm_ops.virt_to_phys = Some(hyp_virt_to_phys);
    hyp_early_alloc_mm_ops.get_page = Some(hyp_early_alloc_get_page);
    hyp_early_alloc_mm_ops.put_page = Some(hyp_early_alloc_put_page);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
