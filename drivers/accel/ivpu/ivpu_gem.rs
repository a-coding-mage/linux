// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020-2026 Intel Corporation */

// Kernel and driver declarations are supplied by the surrounding translation unit.

static IVPU_GEM_FUNCS: drm_gem_object_funcs = drm_gem_object_funcs { /* initialized below */ };

#[inline]
unsafe fn ivpu_dbg_bo(vdev: *mut ivpu_device, bo: *mut ivpu_bo, action: *const c_char) {
    ivpu_dbg(vdev, BO, c"%6s: bo %8p size %9zu ctx %d vpu_addr %9llx pages %d sgt %d mmu_mapped %d wc %d imported %d\n".as_ptr(), action, bo, ivpu_bo_size(bo), (*bo).ctx_id, (*bo).vpu_addr, (!(*bo).base.pages.is_null()) as i32, (!(*bo).base.sgt.is_null()) as i32, (*bo).mmu_mapped, (*bo).base.map_wc, drm_gem_is_imported(&(*bo).base.base));
}

#[inline] unsafe fn ivpu_bo_lock(bo: *mut ivpu_bo) -> c_int { dma_resv_lock((*bo).base.base.resv, core::ptr::null_mut()) }
#[inline] unsafe fn ivpu_bo_unlock(bo: *mut ivpu_bo) { dma_resv_unlock((*bo).base.base.resv); }

unsafe fn ivpu_bo_map_attachment(vdev: *mut ivpu_device, bo: *mut ivpu_bo) -> *mut sg_table {
    drm_WARN_ON(&(*vdev).drm, !drm_gem_is_imported(&(*bo).base.base));
    ivpu_bo_lock(bo);
    let mut sgt = (*bo).base.sgt;
    if sgt.is_null() {
        sgt = dma_buf_map_attachment((*bo).base.base.import_attach, DMA_BIDIRECTIONAL);
        if IS_ERR(sgt) { ivpu_err(vdev, c"Failed to map BO in IOMMU: %ld\n".as_ptr(), PTR_ERR(sgt)); }
        else { (*bo).base.sgt = sgt; }
    }
    ivpu_bo_unlock(bo); sgt
}

pub unsafe fn ivpu_bo_bind(bo: *mut ivpu_bo) -> c_int {
    let vdev = ivpu_bo_to_vdev(bo); let sgt;
    ivpu_dbg_bo(vdev, bo, c"bind".as_ptr());
    if drm_gem_is_imported(&(*bo).base.base) { sgt = ivpu_bo_map_attachment(vdev, bo); }
    else { sgt = drm_gem_shmem_get_pages_sgt(&mut (*bo).base); }
    if IS_ERR(sgt) { let ret = PTR_ERR(sgt) as c_int; ivpu_err(vdev, c"Failed to map BO in IOMMU: %d\n".as_ptr(), ret); return ret; }
    ivpu_bo_lock(bo); let mut ret = 0;
    if !(*bo).mmu_mapped {
        drm_WARN_ON(&(*vdev).drm, (*bo).ctx.is_null());
        ret = ivpu_mmu_context_map_sgt(vdev, (*bo).ctx, (*bo).vpu_addr, sgt, ivpu_bo_size(bo), ivpu_bo_is_snooped(bo), ivpu_bo_is_read_only(bo));
        if ret != 0 { ivpu_err(vdev, c"Failed to map BO in MMU: %d\n".as_ptr(), ret); }
        else { (*bo).mmu_mapped = true; }
    }
    ivpu_bo_unlock(bo); ret
}

unsafe fn ivpu_bo_alloc_vpu_addr(bo: *mut ivpu_bo, ctx: *mut ivpu_mmu_context, range: *const ivpu_addr_range) -> c_int {
    let vdev = ivpu_bo_to_vdev(bo); let mut idx = 0;
    if !drm_dev_enter(&(*vdev).drm, &mut idx) { return -ENODEV; }
    ivpu_bo_lock(bo); let ret = ivpu_mmu_context_insert_node(ctx, range, ivpu_bo_size(bo), &mut (*bo).mm_node);
    if ret == 0 { (*bo).ctx = ctx; (*bo).ctx_id = (*ctx).id; (*bo).vpu_addr = (*bo).mm_node.start; ivpu_dbg_bo(vdev, bo, c"vaddr".as_ptr()); }
    ivpu_bo_unlock(bo); drm_dev_exit(idx); ret
}

unsafe fn ivpu_bo_unbind_locked(bo: *mut ivpu_bo) {
    let vdev = ivpu_bo_to_vdev(bo); dma_resv_assert_held((*bo).base.base.resv);
    if (*bo).mmu_mapped { drm_WARN_ON(&(*vdev).drm, (*bo).ctx.is_null()); drm_WARN_ON(&(*vdev).drm, (*bo).vpu_addr == 0); drm_WARN_ON(&(*vdev).drm, (*bo).base.sgt.is_null()); ivpu_mmu_context_unmap_sgt(vdev, (*bo).ctx, (*bo).vpu_addr, (*bo).base.sgt); (*bo).mmu_mapped = false; }
    if !(*bo).ctx.is_null() { ivpu_mmu_context_remove_node((*bo).ctx, &mut (*bo).mm_node); (*bo).ctx = core::ptr::null_mut(); }
    if !(*bo).base.sgt.is_null() { if drm_gem_is_imported(&(*bo).base.base) { dma_buf_unmap_attachment((*bo).base.base.import_attach, (*bo).base.sgt, DMA_BIDIRECTIONAL); } else { dma_unmap_sgtable((*vdev).drm.dev, (*bo).base.sgt, DMA_BIDIRECTIONAL, 0); sg_free_table((*bo).base.sgt); kfree((*bo).base.sgt as *mut c_void); } (*bo).base.sgt = core::ptr::null_mut(); }
}

pub unsafe fn ivpu_bo_unbind_all_bos_from_context(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context) { if drm_WARN_ON(&(*vdev).drm, ctx.is_null()) { return; } mutex_lock(&mut (*vdev).bo_list_lock); let mut bo; list_for_each_entry!(bo, &(*vdev).bo_list, bo_list_node) { ivpu_bo_lock(bo); if (*bo).ctx == ctx { ivpu_dbg_bo(vdev, bo, c"unbind".as_ptr()); ivpu_bo_unbind_locked(bo); } ivpu_bo_unlock(bo); } mutex_unlock(&mut (*vdev).bo_list_lock); }

pub unsafe fn ivpu_gem_create_object(dev: *mut drm_device, size: usize) -> *mut drm_gem_object { if size == 0 || !PAGE_ALIGNED(size) { return ERR_PTR(-EINVAL); } let bo = kzalloc_obj::<ivpu_bo>(); if bo.is_null() { return ERR_PTR(-ENOMEM); } (*bo).base.base.funcs = &IVPU_GEM_FUNCS; (*bo).base.pages_mark_dirty_on_put = true; INIT_LIST_HEAD(&mut (*bo).bo_list_node); &mut (*bo).base.base }

pub unsafe fn ivpu_gem_prime_import(dev: *mut drm_device, dma_buf: *mut dma_buf) -> *mut drm_gem_object { let vdev = to_ivpu_device(dev); let attach = dma_buf_attach(dma_buf, (*dev).dev); if IS_ERR(attach) { return ERR_CAST(attach); } get_dma_buf(dma_buf); let obj = drm_gem_shmem_prime_import_sg_table(dev, attach, core::ptr::null_mut()); if IS_ERR(obj) { dma_buf_detach(dma_buf, attach); dma_buf_put(dma_buf); return ERR_CAST(obj); } (*obj).import_attach = attach; (*obj).resv = (*dma_buf).resv; let bo = to_ivpu_bo(obj); mutex_lock(&mut (*vdev).bo_list_lock); list_add_tail(&mut (*bo).bo_list_node, &mut (*vdev).bo_list); mutex_unlock(&mut (*vdev).bo_list_lock); ivpu_dbg(vdev, BO, c"import: bo %8p size %9zu\n".as_ptr(), bo, ivpu_bo_size(bo)); obj }

pub unsafe fn ivpu_bo_create(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context, range: *mut ivpu_addr_range, size: u64, flags: u32) -> *mut ivpu_bo {
    if drm_WARN_ON(&(*vdev).drm, range.is_null()) { return core::ptr::null_mut(); }
    let bo = ivpu_bo_alloc(vdev, size, flags); if IS_ERR(bo) { return core::ptr::null_mut(); }
    if ivpu_bo_alloc_vpu_addr(bo, ctx, range) != 0 || ivpu_bo_bind(bo) != 0 { drm_gem_object_put(&mut (*bo).base.base); return core::ptr::null_mut(); } bo
}
pub unsafe fn ivpu_bo_create_runtime(vdev: *mut ivpu_device, addr: u64, size: u64, flags: u32) -> *mut ivpu_bo { let mut range = core::mem::zeroed(); if !ivpu_is_within_range(addr, size, &(*vdev).hw.ranges.runtime) || ivpu_hw_range_init(vdev, &mut range, addr, size) != 0 { return core::ptr::null_mut(); } ivpu_bo_create(vdev, &mut (*vdev).gctx, &mut range, size, flags) }
pub unsafe fn ivpu_bo_create_global(vdev: *mut ivpu_device, size: u64, flags: u32) -> *mut ivpu_bo { ivpu_bo_create(vdev, &mut (*vdev).gctx, &mut (*vdev).hw.ranges.global, size, flags) }
pub unsafe fn ivpu_bo_free(bo: *mut ivpu_bo) { if (*bo).flags & DRM_IVPU_BO_MAPPABLE != 0 { ivpu_bo_lock(bo); drm_gem_shmem_vunmap_locked(&mut (*bo).base, &mut IOSYS_MAP_INIT_VADDR((*bo).base.vaddr)); ivpu_bo_unlock(bo); } drm_gem_object_put(&mut (*bo).base.base); }
pub unsafe fn ivpu_bo_list(dev: *mut drm_device, p: *mut drm_printer) { let vdev = to_ivpu_device(dev); drm_printf(p, c"%-9s %-3s %-14s %-10s %-10s %-4s %s\n".as_ptr(), c"bo".as_ptr(), c"ctx".as_ptr(), c"vpu_addr".as_ptr(), c"size".as_ptr(), c"flags".as_ptr(), c"refs".as_ptr(), c"attribs".as_ptr()); mutex_lock(&mut (*vdev).bo_list_lock); let mut bo; list_for_each_entry!(bo, &(*vdev).bo_list, bo_list_node) { ivpu_bo_print_info(bo, p); } mutex_unlock(&mut (*vdev).bo_list_lock); }
pub unsafe fn ivpu_bo_list_print(dev: *mut drm_device) { let mut p = drm_info_printer((*dev).dev); ivpu_bo_list(dev, &mut p); }

unsafe fn ivpu_bo_print_info(bo: *mut ivpu_bo, p: *mut drm_printer) { ivpu_bo_lock(bo); drm_printf(p, c"%-9p %-3u 0x%-12llx %-10lu 0x%-8x %-4u".as_ptr(), bo, (*bo).ctx_id, (*bo).vpu_addr, (*bo).base.base.size, (*bo).flags, kref_read(&(*bo).base.base.refcount)); if !(*bo).base.pages.is_null() { drm_printf(p, c" has_pages".as_ptr()); } if (*bo).mmu_mapped { drm_printf(p, c" mmu_mapped".as_ptr()); } if drm_gem_is_imported(&(*bo).base.base) { drm_printf(p, c" imported".as_ptr()); } drm_printf(p, c"\n".as_ptr()); ivpu_bo_unlock(bo); }

extern "C" { pub fn ivpu_bo_create_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int; pub fn ivpu_bo_info_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int; pub fn ivpu_bo_wait_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
