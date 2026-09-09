/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * based on nouveau_prime.c
 *
 * Authors: Alex Deucher
 */

//! PRIME Buffer Sharing.
//! Callback implementations for sharing GEM buffer objects between devices.

extern "C" {
    static amdgpu_dma_buf_attach_ops: dma_buf_attach_ops;
}

unsafe fn dma_buf_attach_adev(attach: *mut dma_buf_attachment) -> *mut amdgpu_device {
    if (*attach).importer_ops == &amdgpu_dma_buf_attach_ops {
        let obj = (*attach).importer_priv as *mut drm_gem_object;
        let bo = gem_to_amdgpu_bo(obj);
        return amdgpu_ttm_adev((*bo).tbo.bdev);
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn amdgpu_dma_buf_attach(
    dmabuf: *mut dma_buf, attach: *mut dma_buf_attachment,
) -> i32 {
    let attach_adev = dma_buf_attach_adev(attach);
    let obj = (*dmabuf).priv_ as *mut drm_gem_object;
    let bo = gem_to_amdgpu_bo(obj);
    let adev = amdgpu_ttm_adev((*bo).tbo.bdev);

    if amdgpu_ip_version(adev, GC_HWIP, 0) >= IP_VERSION(12, 0, 0)
        && ((*bo).flags & AMDGPU_GEM_CREATE_GFX12_DCC) != 0
    { (*attach).peer2peer = false; }
    if !amdgpu_dmabuf_is_xgmi_accessible(attach_adev, bo)
        && pci_p2pdma_distance((*adev).pdev, (*attach).dev, false) < 0
    { (*attach).peer2peer = false; }

    let r = dma_resv_lock((*bo).tbo.base.resv, core::ptr::null_mut());
    if r != 0 { return r; }
    amdgpu_vm_bo_update_shared(bo);
    dma_resv_unlock((*bo).tbo.base.resv);
    0
}

unsafe extern "C" fn amdgpu_dma_buf_pin(attach: *mut dma_buf_attachment) -> i32 {
    let dmabuf = (*attach).dmabuf;
    let bo = gem_to_amdgpu_bo((*dmabuf).priv_ as *mut drm_gem_object);
    let mut domains = (*bo).allowed_domains;
    dma_resv_assert_held((*dmabuf).resv);
    let mut pos = (*dmabuf).attachments;
    while !pos.is_null() {
        let a = container_of_attachment(pos);
        if !(*a).peer2peer { domains &= !AMDGPU_GEM_DOMAIN_VRAM; }
        pos = (*a).node.next;
    }
    if domains & AMDGPU_GEM_DOMAIN_VRAM != 0 { (*bo).flags |= AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED; }
    if WARN_ON(domains == 0) { return -EINVAL; }
    amdgpu_bo_pin(bo, domains)
}

unsafe extern "C" fn amdgpu_dma_buf_unpin(attach: *mut dma_buf_attachment) {
    let bo = gem_to_amdgpu_bo((*(*attach).dmabuf).priv_ as *mut drm_gem_object);
    amdgpu_bo_unpin(bo);
}

unsafe extern "C" fn amdgpu_dma_buf_map(
    attach: *mut dma_buf_attachment, dir: dma_data_direction,
) -> *mut sg_table {
    let dma_buf = (*attach).dmabuf;
    let obj = (*dma_buf).priv_ as *mut drm_gem_object;
    let bo = gem_to_amdgpu_bo(obj);
    let adev = amdgpu_ttm_adev((*bo).tbo.bdev);
    let mut sgt: *mut sg_table = core::ptr::null_mut();
    if (*bo).tbo.pin_count == 0 {
        let mut ctx = ttm_operation_ctx { interruptible: false, no_wait_gpu: false, ..core::mem::zeroed() };
        let mut domains = AMDGPU_GEM_DOMAIN_GTT;
        if ((*bo).preferred_domains & AMDGPU_GEM_DOMAIN_VRAM) != 0 && (*attach).peer2peer {
            (*bo).flags |= AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED; domains |= AMDGPU_GEM_DOMAIN_VRAM;
        }
        amdgpu_bo_placement_from_domain(bo, domains);
        let r = ttm_bo_validate(&mut (*bo).tbo, &mut (*bo).placement, &mut ctx);
        if r != 0 { return ERR_PTR(r); }
    }
    match (*(*bo).tbo.resource).mem_type {
        TTM_PL_TT => {
            sgt = drm_prime_pages_to_sg((*obj).dev, (*(*bo).tbo.ttm).pages, (*(*bo).tbo.ttm).num_pages);
            if IS_ERR(sgt) { return sgt; }
            if dma_map_sgtable((*attach).dev, sgt, dir, DMA_ATTR_SKIP_CPU_SYNC) != 0 { sg_free_table(sgt); kfree(sgt); return ERR_PTR(-EBUSY); }
        }
        TTM_PL_VRAM => {
            if WARN_ON(amdgpu_dmabuf_is_xgmi_accessible(dma_buf_attach_adev(attach), bo)) { return ERR_PTR(-EINVAL); }
            let r = amdgpu_vram_mgr_alloc_sgt(adev, (*bo).tbo.resource, 0, (*bo).tbo.base.size, (*attach).dev, dir, &mut sgt);
            if r != 0 { return ERR_PTR(r); }
        }
        AMDGPU_PL_MMIO_REMAP => {
            let r = amdgpu_ttm_mmio_remap_alloc_sgt(adev, (*bo).tbo.resource, (*attach).dev, dir, &mut sgt);
            if r != 0 { return ERR_PTR(r); }
        }
        _ => return ERR_PTR(-EINVAL),
    }
    sgt
}

unsafe extern "C" fn amdgpu_dma_buf_unmap(attach: *mut dma_buf_attachment, sgt: *mut sg_table, dir: dma_data_direction) {
    let bo = gem_to_amdgpu_bo((*(*attach).dmabuf).priv_ as *mut drm_gem_object);
    if !(*bo).tbo.resource.is_null() && (*(*bo).tbo.resource).mem_type == AMDGPU_PL_MMIO_REMAP { amdgpu_ttm_mmio_remap_free_sgt((*attach).dev, dir, sgt); return; }
    if !sg_page((*sgt).sgl).is_null() { dma_unmap_sgtable((*attach).dev, sgt, dir, 0); sg_free_table(sgt); kfree(sgt); }
    else { amdgpu_vram_mgr_free_sgt((*attach).dev, dir, sgt); }
}

unsafe extern "C" fn amdgpu_dma_buf_begin_cpu_access(dma_buf: *mut dma_buf, direction: dma_data_direction) -> i32 {
    let bo = gem_to_amdgpu_bo((*dma_buf).priv_ as *mut drm_gem_object);
    let adev = amdgpu_ttm_adev((*bo).tbo.bdev);
    let mut ctx = ttm_operation_ctx { interruptible: true, no_wait_gpu: false, ..core::mem::zeroed() };
    let domain = amdgpu_display_supported_domains(adev, (*bo).flags);
    let reads = direction == DMA_BIDIRECTIONAL || direction == DMA_FROM_DEVICE;
    if !reads || domain & AMDGPU_GEM_DOMAIN_GTT == 0 { return 0; }
    let mut ret = amdgpu_bo_reserve(bo, false); if ret != 0 { return ret; }
    if (*bo).tbo.pin_count == 0 && (*bo).allowed_domains & AMDGPU_GEM_DOMAIN_GTT != 0 { amdgpu_bo_placement_from_domain(bo, AMDGPU_GEM_DOMAIN_GTT); ret = ttm_bo_validate(&mut (*bo).tbo, &mut (*bo).placement, &mut ctx); }
    amdgpu_bo_unreserve(bo); ret
}

unsafe extern "C" fn amdgpu_dma_buf_vmap(dma_buf: *mut dma_buf, map: *mut iosys_map) -> i32 {
    let bo = gem_to_amdgpu_bo((*dma_buf).priv_ as *mut drm_gem_object);
    let mut ret = amdgpu_bo_pin(bo, AMDGPU_GEM_DOMAIN_GTT | AMDGPU_GEM_DOMAIN_VRAM);
    if ret != 0 { return ret; }
    ret = drm_gem_dmabuf_vmap(dma_buf, map); if ret != 0 { amdgpu_bo_unpin(bo); } ret
}

unsafe extern "C" fn amdgpu_dma_buf_vunmap(dma_buf: *mut dma_buf, map: *mut iosys_map) {
    let bo = gem_to_amdgpu_bo((*dma_buf).priv_ as *mut drm_gem_object); drm_gem_dmabuf_vunmap(dma_buf, map); amdgpu_bo_unpin(bo);
}

#[no_mangle] pub static amdgpu_dmabuf_ops: dma_buf_ops = dma_buf_ops { attach: Some(amdgpu_dma_buf_attach), pin: Some(amdgpu_dma_buf_pin), unpin: Some(amdgpu_dma_buf_unpin), map_dma_buf: Some(amdgpu_dma_buf_map), unmap_dma_buf: Some(amdgpu_dma_buf_unmap), release: Some(drm_gem_dmabuf_release), begin_cpu_access: Some(amdgpu_dma_buf_begin_cpu_access), mmap: Some(drm_gem_dmabuf_mmap), vmap: Some(amdgpu_dma_buf_vmap), vunmap: Some(amdgpu_dma_buf_vunmap), ..unsafe { core::mem::zeroed() } };

unsafe extern "C" fn amdgpu_gem_prime_export(gobj: *mut drm_gem_object, flags: i32) -> *mut dma_buf {
    let bo = gem_to_amdgpu_bo(gobj); if amdgpu_ttm_tt_get_usermm((*bo).tbo.ttm) || (*bo).flags & AMDGPU_GEM_CREATE_VM_ALWAYS_VALID != 0 { return ERR_PTR(-EPERM); }
    let mut ctx = ttm_operation_ctx { interruptible: true, no_wait_gpu: true, gfp_retry_mayfail: true, allow_res_evict: false, ..core::mem::zeroed() };
    let ret = ttm_bo_setup_export(&mut (*bo).tbo, &mut ctx); if ret != 0 { return ERR_PTR(ret); }
    let buf = drm_gem_prime_export(gobj, flags); if !IS_ERR(buf) { (*buf).ops = &amdgpu_dmabuf_ops; } buf
}

unsafe extern "C" fn amdgpu_dma_buf_create_obj(dev: *mut drm_device, dma_buf: *mut dma_buf) -> *mut drm_gem_object {
    let resv = (*dma_buf).resv; let adev = drm_to_adev(dev); let mut gobj = core::ptr::null_mut(); let mut flags: u64 = 0;
    dma_resv_lock(resv, core::ptr::null_mut());
    if (*dma_buf).ops == &amdgpu_dmabuf_ops { let other = gem_to_amdgpu_bo((*dma_buf).priv_ as *mut drm_gem_object); flags |= (*other).flags & (AMDGPU_GEM_CREATE_CPU_GTT_USWC | AMDGPU_GEM_CREATE_COHERENT | AMDGPU_GEM_CREATE_EXT_COHERENT | AMDGPU_GEM_CREATE_UNCACHED); }
    let ret = amdgpu_gem_object_create(adev, (*dma_buf).size, PAGE_SIZE, AMDGPU_GEM_DOMAIN_CPU, flags, ttm_bo_type_sg, resv, &mut gobj, 0);
    if ret != 0 { dma_resv_unlock(resv); return ERR_PTR(ret); }
    let bo = gem_to_amdgpu_bo(gobj); (*bo).allowed_domains = AMDGPU_GEM_DOMAIN_GTT; (*bo).preferred_domains = AMDGPU_GEM_DOMAIN_GTT; dma_resv_unlock(resv); gobj
}

unsafe extern "C" fn amdgpu_dma_buf_move_notify(attach: *mut dma_buf_attachment) {
    let obj = (*attach).importer_priv as *mut drm_gem_object; let ticket = dma_resv_locking_ctx((*obj).resv); let bo = gem_to_amdgpu_bo(obj); let adev = amdgpu_ttm_adev((*bo).tbo.bdev);
    let mut ctx = ttm_operation_ctx { interruptible: false, no_wait_gpu: false, ..core::mem::zeroed() }; let mut placement: ttm_placement = core::mem::zeroed(); amdgpu_vm_bo_invalidate(bo, false);
    if (*bo).tbo.resource.is_null() || (*(*bo).tbo.resource).mem_type == TTM_PL_SYSTEM { return; }
    let mut r = ttm_bo_validate(&mut (*bo).tbo, &mut placement, &mut ctx); if r != 0 { DRM_ERROR("Failed to invalidate DMA-buf import (%d))\n", r); return; }
    let mut bo_base = (*bo).vm_bo;
    while !bo_base.is_null() { let vm = (*bo_base).vm; let resv = (*(*vm).root.bo).tbo.base.resv; r = if !ticket.is_null() { dma_resv_lock(resv, ticket) } else if dma_resv_trylock(resv) { 0 } else { -EBUSY }; if r != 0 { bo_base = (*bo_base).next; continue; } r = dma_resv_reserve_fences(resv, 2); if r == 0 { r = amdgpu_vm_clear_freed(adev, vm, core::ptr::null_mut()); } if r == 0 { r = amdgpu_vm_handle_moved(adev, vm, core::ptr::null_mut()); } if r != 0 && r != -EBUSY { DRM_ERROR("Failed to invalidate VM page tables (%d))\n", r); } dma_resv_unlock(resv); bo_base = (*bo_base).next; }
}

#[no_mangle] pub static amdgpu_dma_buf_attach_ops: dma_buf_attach_ops = dma_buf_attach_ops { allow_peer2peer: true, invalidate_mappings: Some(amdgpu_dma_buf_move_notify) };

unsafe extern "C" fn amdgpu_gem_prime_import(dev: *mut drm_device, dma_buf: *mut dma_buf) -> *mut drm_gem_object {
    if (*dma_buf).ops == &amdgpu_dmabuf_ops { let obj = (*dma_buf).priv_ as *mut drm_gem_object; if (*obj).dev == dev { drm_gem_object_get(obj); return obj; } }
    let obj = amdgpu_dma_buf_create_obj(dev, dma_buf); if IS_ERR(obj) { return obj; }
    let attach = dma_buf_dynamic_attach(dma_buf, (*dev).dev, &amdgpu_dma_buf_attach_ops, obj); if IS_ERR(attach) { drm_gem_object_put(obj); return ERR_CAST(attach); }
    get_dma_buf(dma_buf); (*obj).import_attach = attach; obj
}

unsafe extern "C" fn amdgpu_dmabuf_is_xgmi_accessible(adev: *mut amdgpu_device, mut bo: *mut amdgpu_bo) -> bool {
    if adev.is_null() { return false; }
    let obj = &mut (*bo).tbo.base as *mut drm_gem_object;
    if drm_gem_is_imported(obj) { let dma_buf = (*(*obj).import_attach).dmabuf; if (*dma_buf).ops != &amdgpu_dmabuf_ops { return false; } bo = gem_to_amdgpu_bo((*dma_buf).priv_ as *mut drm_gem_object); }
    amdgpu_xgmi_same_hive(adev, amdgpu_ttm_adev((*bo).tbo.bdev)) && ((*bo).preferred_domains & AMDGPU_GEM_DOMAIN_VRAM != 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
