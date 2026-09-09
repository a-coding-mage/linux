/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/* Copyright © 2024 Intel Corporation */

/* Linux dependencies supplied by the surrounding translation unit. */

#[repr(C)]
pub struct drm_gpusvm_ops {
    pub notifier_alloc: Option<unsafe extern "C" fn() -> *mut drm_gpusvm_notifier>,
    pub notifier_free: Option<unsafe extern "C" fn(*mut drm_gpusvm_notifier)>,
    pub range_alloc: Option<unsafe extern "C" fn(*mut drm_gpusvm) -> *mut drm_gpusvm_range>,
    pub range_free: Option<unsafe extern "C" fn(*mut drm_gpusvm_range)>,
    pub invalidate: Option<unsafe extern "C" fn(*mut drm_gpusvm, *mut drm_gpusvm_notifier, *const mmu_notifier_range)>,
}

#[repr(C)]
pub struct drm_gpusvm_notifier {
    pub gpusvm: *mut drm_gpusvm,
    pub notifier: mmu_interval_notifier,
    pub itree: interval_tree_node,
    pub entry: list_head,
    pub root: rb_root_cached,
    pub range_list: list_head,
    pub flags: drm_gpusvm_notifier_flags,
}

#[repr(C)]
pub union drm_gpusvm_notifier_flags {
    pub bits: drm_gpusvm_notifier_flag_bits,
    pub __flags: u32,
}
#[repr(C)]
pub struct drm_gpusvm_notifier_flag_bits { pub removed: u32 }

#[repr(C)]
pub union drm_gpusvm_pages_flags {
    pub bits: drm_gpusvm_pages_flag_bits,
    pub __flags: u16,
}
#[repr(C)]
pub struct drm_gpusvm_pages_flag_bits {
    pub unmapped: u16,
    pub has_devmem_pages: u16,
    pub has_dma_mapping: u16,
}

#[repr(C)]
pub struct drm_gpusvm_pages {
    pub drm: *mut drm_device,
    pub dma_addr: *mut drm_pagemap_addr,
    pub dpagemap: *mut drm_pagemap,
    pub state: dma_iova_state,
    pub state_offset: c_ulong,
    pub notifier_seq: c_ulong,
    pub flags: drm_gpusvm_pages_flags,
}

#[repr(C)]
pub union drm_gpusvm_range_flags {
    pub bits: drm_gpusvm_range_flag_bits,
    pub __flags: u16,
}
#[repr(C)]
pub struct drm_gpusvm_range_flag_bits {
    pub migrate_devmem: u16,
    pub unmapped: u16,
    pub partial_unmap: u16,
}

#[repr(C)]
pub struct drm_gpusvm_range {
    pub gpusvm: *mut drm_gpusvm,
    pub notifier: *mut drm_gpusvm_notifier,
    pub refcount: kref,
    pub itree: interval_tree_node,
    pub entry: list_head,
    pub flags: drm_gpusvm_range_flags,
}

#[repr(C)]
pub struct drm_gpusvm {
    pub name: *const c_char,
    pub mm: *mut mm_struct,
    pub mm_start: c_ulong,
    pub mm_range: c_ulong,
    pub notifier_size: c_ulong,
    pub ops: *const drm_gpusvm_ops,
    pub chunk_sizes: *const c_ulong,
    pub num_chunks: c_int,
    pub notifier_lock: rw_semaphore,
    pub root: rb_root_cached,
    pub notifier_list: list_head,
    #[cfg(CONFIG_LOCKDEP)]
    pub lock_dep_map: *mut lockdep_map,
}

#[repr(C)]
pub struct drm_gpusvm_ctx {
    pub device_private_page_owner: *mut c_void,
    pub check_pages_threshold: c_ulong,
    pub timeslice_ms: c_ulong,
    pub in_notifier: u32,
    pub read_only: u32,
    pub devmem_possible: u32,
    pub devmem_only: u32,
    pub allow_mixed: u32,
}

pub unsafe extern "C" fn drm_gpusvm_init(gpusvm: *mut drm_gpusvm, name: *const c_char, mm: *mut mm_struct, mm_start: c_ulong, mm_range: c_ulong, notifier_size: c_ulong, ops: *const drm_gpusvm_ops, chunk_sizes: *const c_ulong, num_chunks: c_int) -> c_int;
pub unsafe extern "C" fn drm_gpusvm_fini(gpusvm: *mut drm_gpusvm);
pub unsafe extern "C" fn drm_gpusvm_free(gpusvm: *mut drm_gpusvm);
pub unsafe extern "C" fn drm_gpusvm_find_vma_start(gpusvm: *mut drm_gpusvm, start: c_ulong, end: c_ulong) -> c_ulong;
pub unsafe extern "C" fn drm_gpusvm_range_find_or_insert(gpusvm: *mut drm_gpusvm, fault_addr: c_ulong, gpuva_start: c_ulong, gpuva_end: c_ulong, ctx: *const drm_gpusvm_ctx) -> *mut drm_gpusvm_range;
pub unsafe extern "C" fn drm_gpusvm_range_remove(gpusvm: *mut drm_gpusvm, range: *mut drm_gpusvm_range);
pub unsafe extern "C" fn drm_gpusvm_range_evict(gpusvm: *mut drm_gpusvm, range: *mut drm_gpusvm_range) -> c_int;
pub unsafe extern "C" fn drm_gpusvm_range_get(range: *mut drm_gpusvm_range) -> *mut drm_gpusvm_range;
pub unsafe extern "C" fn drm_gpusvm_range_put(range: *mut drm_gpusvm_range);
pub unsafe extern "C" fn drm_gpusvm_pages_valid(gpusvm: *mut drm_gpusvm, pages: *mut drm_gpusvm_pages) -> bool;
pub unsafe extern "C" fn drm_gpusvm_has_mapping(gpusvm: *mut drm_gpusvm, start: c_ulong, end: c_ulong) -> bool;
pub unsafe extern "C" fn drm_gpusvm_notifier_find(gpusvm: *mut drm_gpusvm, start: c_ulong, end: c_ulong) -> *mut drm_gpusvm_notifier;
pub unsafe extern "C" fn drm_gpusvm_range_find(notifier: *mut drm_gpusvm_notifier, start: c_ulong, end: c_ulong) -> *mut drm_gpusvm_range;
pub unsafe extern "C" fn drm_gpusvm_range_set_unmapped(range: *mut drm_gpusvm_range, pages: *mut drm_gpusvm_pages, pages_count: c_uint, mmu_range: *const mmu_notifier_range);
pub unsafe extern "C" fn drm_gpusvm_get_pages(gpusvm: *mut drm_gpusvm, pages: *mut drm_gpusvm_pages, mm: *mut mm_struct, notifier: *mut mmu_interval_notifier, pages_start: c_ulong, pages_end: c_ulong, ctx: *const drm_gpusvm_ctx) -> c_int;
pub unsafe extern "C" fn drm_gpusvm_unmap_pages(gpusvm: *mut drm_gpusvm, pages: *mut drm_gpusvm_pages, npages: c_ulong, ctx: *const drm_gpusvm_ctx);
pub unsafe extern "C" fn drm_gpusvm_free_pages(gpusvm: *mut drm_gpusvm, pages: *mut drm_gpusvm_pages, npages: c_ulong);

pub unsafe fn drm_gpusvm_init_pages(svm_pages: *mut drm_gpusvm_pages, drm: *mut drm_device) {
    core::ptr::write_bytes(svm_pages, 0, 1);
    (*svm_pages).drm = drm;
    (*svm_pages).notifier_seq = c_ulong::MAX;
}

pub const DRM_GPUSVM_SCAN_UNPOPULATED: c_int = 0;
pub const DRM_GPUSVM_SCAN_EQUAL: c_int = 1;
pub const DRM_GPUSVM_SCAN_OTHER: c_int = 2;
pub const DRM_GPUSVM_SCAN_SYSTEM: c_int = 3;
pub const DRM_GPUSVM_SCAN_MIXED_DEVICE: c_int = 4;
pub const DRM_GPUSVM_SCAN_MIXED: c_int = 5;
pub type drm_gpusvm_scan_result = c_int;
pub unsafe extern "C" fn drm_gpusvm_scan_mm(range: *mut drm_gpusvm_range, dev_private_owner: *mut c_void, pagemap: *const dev_pagemap) -> drm_gpusvm_scan_result;

pub unsafe fn drm_gpusvm_range_start(range: *mut drm_gpusvm_range) -> c_ulong { (*range).itree.start }
pub unsafe fn drm_gpusvm_range_end(range: *mut drm_gpusvm_range) -> c_ulong { (*range).itree.last + 1 }
pub unsafe fn drm_gpusvm_range_size(range: *mut drm_gpusvm_range) -> c_ulong { drm_gpusvm_range_end(range) - drm_gpusvm_range_start(range) }
pub unsafe fn drm_gpusvm_notifier_start(notifier: *mut drm_gpusvm_notifier) -> c_ulong { (*notifier).itree.start }
pub unsafe fn drm_gpusvm_notifier_end(notifier: *mut drm_gpusvm_notifier) -> c_ulong { (*notifier).itree.last + 1 }
pub unsafe fn drm_gpusvm_notifier_size(notifier: *mut drm_gpusvm_notifier) -> c_ulong { drm_gpusvm_notifier_end(notifier) - drm_gpusvm_notifier_start(notifier) }

/* The following C iteration and locking macros retain their source intent. */
#[macro_export] macro_rules! drm_gpusvm_notifier_lock { ($g:expr) => { unsafe { down_read(&mut (*$g).notifier_lock) } }; }
#[macro_export] macro_rules! drm_gpusvm_notifier_unlock { ($g:expr) => { unsafe { up_read(&mut (*$g).notifier_lock) } }; }

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
#[repr(C)] pub struct drm_device;
#[repr(C)] pub struct drm_pagemap_addr;
#[repr(C)] pub struct drm_pagemap;
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct dev_pagemap;
#[repr(C)] pub struct mmu_notifier_range;
#[repr(C)] pub struct mmu_interval_notifier;
#[repr(C)] pub struct interval_tree_node { pub start: c_ulong, pub last: c_ulong }
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct rb_root_cached;
#[repr(C)] pub struct dma_iova_state;
#[repr(C)] pub struct kref;
#[repr(C)] pub struct rw_semaphore;
#[repr(C)] pub struct lockdep_map;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
