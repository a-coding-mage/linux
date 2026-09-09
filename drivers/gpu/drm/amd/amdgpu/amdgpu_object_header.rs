/*
 * Copyright 2008 Advanced Micro Devices, Inc.
 * Copyright 2008 Red Hat Inc.
 * Copyright 2009 Jerome Glisse.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

pub const AMDGPU_BO_INVALID_OFFSET: libc::c_long = libc::LONG_MAX;
pub const AMDGPU_BO_MAX_PLACEMENTS: usize = 3;
pub const AMDGPU_AMDKFD_CREATE_USERPTR_BO: u64 = 1u64 << 63;

#[macro_export]
macro_rules! to_amdgpu_bo_user { ($abo:expr) => { container_of!($abo, amdgpu_bo_user, bo) }; }
#[macro_export]
macro_rules! to_amdgpu_bo_vm { ($abo:expr) => { container_of!($abo, amdgpu_bo_vm, bo) }; }

#[repr(C)]
pub struct amdgpu_bo_param {
    pub size: libc::c_ulong,
    pub byte_align: libc::c_int,
    pub bo_ptr_size: u32,
    pub domain: u32,
    pub preferred_domain: u32,
    pub flags: u64,
    pub r#type: ttm_bo_type,
    pub no_wait_gpu: bool,
    pub resv: *mut dma_resv,
    pub destroy: Option<unsafe extern "C" fn(*mut ttm_buffer_object)>,
    pub xcp_id_plus1: i8,
}

#[repr(C)]
pub struct amdgpu_bo_va_mapping {
    pub bo_va: *mut amdgpu_bo_va,
    pub list: list_head,
    pub rb: rb_node,
    pub start: u64,
    pub last: u64,
    pub __subtree_last: u64,
    pub offset: u64,
    pub flags: u32,
}

#[repr(C)]
pub struct amdgpu_bo_va {
    pub base: amdgpu_vm_bo_base,
    pub ref_count: libc::c_uint,
    pub last_pt_update: *mut dma_fence,
    pub invalids: list_head,
    pub valids: list_head,
    pub cleared: bool,
    pub is_xgmi: bool,
    pub queue_refcount: libc::c_uint,
    pub userq_va_mapped: bool,
}

#[repr(C)]
pub struct amdgpu_bo {
    pub preferred_domains: u32,
    pub allowed_domains: u32,
    pub placements: [ttm_place; AMDGPU_BO_MAX_PLACEMENTS],
    pub placement: ttm_placement,
    pub tbo: ttm_buffer_object,
    pub kmap: ttm_bo_kmap_obj,
    pub flags: u64,
    pub vm_bo: *mut amdgpu_vm_bo_base,
    pub parent: *mut amdgpu_bo,
    #[cfg(CONFIG_MMU_NOTIFIER)]
    pub notifier: mmu_interval_notifier,
    pub kfd_bo: *mut kgd_mem,
    pub xcp_id: i8,
}

#[repr(C)]
pub struct amdgpu_bo_user {
    pub bo: amdgpu_bo,
    pub tiling_flags: u64,
    pub metadata_flags: u64,
    pub metadata: *mut libc::c_void,
    pub metadata_size: u32,
}

#[repr(C)]
pub struct amdgpu_bo_vm {
    pub bo: amdgpu_bo,
    pub entries: [amdgpu_vm_bo_base; 0],
}

#[inline]
pub unsafe fn ttm_to_amdgpu_bo(tbo: *mut ttm_buffer_object) -> *mut amdgpu_bo {
    container_of!(tbo, amdgpu_bo, tbo)
}

#[inline]
pub fn amdgpu_mem_type_to_domain(mem_type: u32) -> u32 {
    match mem_type {
        TTM_PL_VRAM => AMDGPU_GEM_DOMAIN_VRAM,
        TTM_PL_TT => AMDGPU_GEM_DOMAIN_GTT,
        TTM_PL_SYSTEM => AMDGPU_GEM_DOMAIN_CPU,
        AMDGPU_PL_GDS => AMDGPU_GEM_DOMAIN_GDS,
        AMDGPU_PL_GWS => AMDGPU_GEM_DOMAIN_GWS,
        AMDGPU_PL_OA => AMDGPU_GEM_DOMAIN_OA,
        AMDGPU_PL_DOORBELL => AMDGPU_GEM_DOMAIN_DOORBELL,
        _ => 0,
    }
}

#[inline]
pub unsafe fn amdgpu_bo_reserve(bo: *mut amdgpu_bo, no_intr: bool) -> libc::c_int {
    let adev = amdgpu_ttm_adev((*bo).tbo.bdev);
    let r = ttm_bo_reserve(&mut (*bo).tbo, !no_intr, false, core::ptr::null_mut());
    if r != 0 {
        if r != -ERESTARTSYS { dev_err((*adev).dev, b"%p reserve failed\0".as_ptr(), bo); }
        return r;
    }
    0
}

#[inline] pub unsafe fn amdgpu_bo_unreserve(bo: *mut amdgpu_bo) { ttm_bo_unreserve(&mut (*bo).tbo); }
#[inline] pub unsafe fn amdgpu_bo_size(bo: *mut amdgpu_bo) -> libc::c_ulong { (*bo).tbo.base.size }
#[inline] pub unsafe fn amdgpu_bo_ngpu_pages(bo: *mut amdgpu_bo) -> libc::c_uint { (*bo).tbo.base.size / AMDGPU_GPU_PAGE_SIZE }
#[inline] pub unsafe fn amdgpu_bo_gpu_page_alignment(bo: *mut amdgpu_bo) -> libc::c_uint { ((*bo).tbo.page_alignment << PAGE_SHIFT) / AMDGPU_GPU_PAGE_SIZE }
#[inline] pub unsafe fn amdgpu_bo_mmap_offset(bo: *mut amdgpu_bo) -> u64 { drm_vma_node_offset_addr(&(*bo).tbo.base.vma_node) }
#[inline] pub unsafe fn amdgpu_bo_explicit_sync(bo: *mut amdgpu_bo) -> bool { (*bo).flags & AMDGPU_GEM_CREATE_EXPLICIT_SYNC != 0 }
#[inline] pub unsafe fn amdgpu_bo_encrypted(bo: *mut amdgpu_bo) -> bool { (*bo).flags & AMDGPU_GEM_CREATE_ENCRYPTED != 0 }

extern "C" {
    pub fn amdgpu_bo_is_amdgpu_bo(bo: *mut ttm_buffer_object) -> bool;
    pub fn amdgpu_bo_placement_from_domain(abo: *mut amdgpu_bo, domain: u32);
    pub fn amdgpu_bo_create(adev: *mut amdgpu_device, bp: *mut amdgpu_bo_param, bo_ptr: *mut *mut amdgpu_bo) -> libc::c_int;
    pub fn amdgpu_bo_create_reserved(adev: *mut amdgpu_device, size: libc::c_ulong, align: libc::c_int, domain: u32, bo_ptr: *mut *mut amdgpu_bo, gpu_addr: *mut u64, cpu_addr: *mut *mut libc::c_void) -> libc::c_int;
    pub fn amdgpu_bo_create_kernel(adev: *mut amdgpu_device, size: libc::c_ulong, align: libc::c_int, domain: u32, bo_ptr: *mut *mut amdgpu_bo, gpu_addr: *mut u64, cpu_addr: *mut *mut libc::c_void) -> libc::c_int;
    pub fn amdgpu_bo_create_isp_user(adev: *mut amdgpu_device, dbuf: *mut dma_buf, domain: u32, bo: *mut *mut amdgpu_bo, gpu_addr: *mut u64) -> libc::c_int;
    pub fn amdgpu_bo_create_kernel_at(adev: *mut amdgpu_device, offset: u64, size: u64, bo_ptr: *mut *mut amdgpu_bo, cpu_addr: *mut *mut libc::c_void) -> libc::c_int;
    pub fn amdgpu_bo_create_user(adev: *mut amdgpu_device, bp: *mut amdgpu_bo_param, ubo_ptr: *mut *mut amdgpu_bo_user) -> libc::c_int;
    pub fn amdgpu_bo_create_vm(adev: *mut amdgpu_device, bp: *mut amdgpu_bo_param, ubo_ptr: *mut *mut amdgpu_bo_vm) -> libc::c_int;
    pub fn amdgpu_bo_free_kernel(bo: *mut *mut amdgpu_bo, gpu_addr: *mut u64, cpu_addr: *mut *mut libc::c_void);
    pub fn amdgpu_bo_free_isp_user(bo: *mut amdgpu_bo);
    pub fn amdgpu_bo_kmap(bo: *mut amdgpu_bo, ptr: *mut *mut libc::c_void) -> libc::c_int;
    pub fn amdgpu_bo_kptr(bo: *mut amdgpu_bo) -> *mut libc::c_void;
    pub fn amdgpu_bo_kunmap(bo: *mut amdgpu_bo);
    pub fn amdgpu_bo_ref(bo: *mut amdgpu_bo) -> *mut amdgpu_bo;
    pub fn amdgpu_bo_unref(bo: *mut *mut amdgpu_bo);
    pub fn amdgpu_bo_pin(bo: *mut amdgpu_bo, domain: u32) -> libc::c_int;
    pub fn amdgpu_bo_unpin(bo: *mut amdgpu_bo);
    pub fn amdgpu_bo_init(adev: *mut amdgpu_device) -> libc::c_int;
    pub fn amdgpu_bo_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_bo_set_tiling_flags(bo: *mut amdgpu_bo, tiling_flags: u64) -> libc::c_int;
    pub fn amdgpu_bo_get_tiling_flags(bo: *mut amdgpu_bo, tiling_flags: *mut u64);
    pub fn amdgpu_bo_set_metadata(bo: *mut amdgpu_bo, metadata: *mut libc::c_void, metadata_size: u32, flags: u64) -> libc::c_int;
    pub fn amdgpu_bo_get_metadata(bo: *mut amdgpu_bo, buffer: *mut libc::c_void, buffer_size: usize, metadata_size: *mut u32, flags: *mut u64) -> libc::c_int;
    pub fn amdgpu_bo_move_notify(bo: *mut ttm_buffer_object, evict: bool, new_mem: *mut ttm_resource);
    pub fn amdgpu_bo_release_notify(bo: *mut ttm_buffer_object);
    pub fn amdgpu_bo_fault_reserve_notify(bo: *mut ttm_buffer_object) -> vm_fault_t;
    pub fn amdgpu_bo_fence(bo: *mut amdgpu_bo, fence: *mut dma_fence, shared: bool);
    pub fn amdgpu_bo_sync_wait_resv(adev: *mut amdgpu_device, resv: *mut dma_resv, sync_mode: amdgpu_sync_mode, owner: *mut libc::c_void, intr: bool) -> libc::c_int;
    pub fn amdgpu_bo_sync_wait(bo: *mut amdgpu_bo, owner: *mut libc::c_void, intr: bool) -> libc::c_int;
    pub fn amdgpu_bo_gpu_offset(bo: *mut amdgpu_bo) -> u64;
    pub fn amdgpu_bo_fb_aper_addr(bo: *mut amdgpu_bo) -> u64;
    pub fn amdgpu_bo_gpu_offset_no_check(bo: *mut amdgpu_bo) -> u64;
    pub fn amdgpu_bo_mem_stats_placement(bo: *mut amdgpu_bo) -> u32;
    pub fn amdgpu_bo_get_preferred_domain(adev: *mut amdgpu_device, domain: u32) -> u32;
    pub fn amdgpu_bo_support_uswc(bo_flags: u64) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
