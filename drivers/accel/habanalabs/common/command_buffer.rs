// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependencies are supplied by the surrounding kernel/Rust translation.

const CB_VA_POOL_SIZE: u64 = 4u64 * SZ_1G;

unsafe fn cb_map_mem(ctx: *mut hl_ctx, cb: *mut hl_cb) -> i32 {
    let hdev = (*ctx).hdev;
    let prop = &(*hdev).asic_prop;
    let page_size = prop.pmmu.page_size;
    let mut rc: i32;
    if !(*hdev).supports_cb_mapping { dev_err_ratelimited((*hdev).dev, "Mapping a CB to the device's MMU is not supported\n"); return -EINVAL; }
    if (*cb).is_mmu_mapped { return 0; }
    (*cb).roundup_size = roundup((*cb).size, page_size);
    (*cb).virtual_addr = gen_pool_alloc((*ctx).cb_va_pool, (*cb).roundup_size) as u64;
    if (*cb).virtual_addr == 0 { dev_err((*hdev).dev, "Failed to allocate device virtual address for CB\n"); return -ENOMEM; }
    mutex_lock(&mut (*hdev).mmu_lock);
    rc = hl_mmu_map_contiguous(ctx, (*cb).virtual_addr, (*cb).bus_address, (*cb).roundup_size);
    if rc != 0 { dev_err((*hdev).dev, "Failed to map VA %#llx to CB\n", (*cb).virtual_addr); goto err_va_pool_free; }
    rc = hl_mmu_invalidate_cache(hdev, false, MMU_OP_USERPTR | MMU_OP_SKIP_LOW_CACHE_INV);
    if rc != 0 { goto err_mmu_unmap; }
    mutex_unlock(&mut (*hdev).mmu_lock);
    (*cb).is_mmu_mapped = true;
    return 0;
err_mmu_unmap:
    hl_mmu_unmap_contiguous(ctx, (*cb).virtual_addr, (*cb).roundup_size);
err_va_pool_free:
    mutex_unlock(&mut (*hdev).mmu_lock);
    gen_pool_free((*ctx).cb_va_pool, (*cb).virtual_addr, (*cb).roundup_size);
    rc
}

unsafe fn cb_unmap_mem(ctx: *mut hl_ctx, cb: *mut hl_cb) {
    let hdev = (*ctx).hdev;
    mutex_lock(&mut (*hdev).mmu_lock);
    hl_mmu_unmap_contiguous(ctx, (*cb).virtual_addr, (*cb).roundup_size);
    hl_mmu_invalidate_cache(hdev, true, MMU_OP_USERPTR);
    mutex_unlock(&mut (*hdev).mmu_lock);
    gen_pool_free((*ctx).cb_va_pool, (*cb).virtual_addr, (*cb).roundup_size);
}

unsafe fn cb_fini(hdev: *mut hl_device, cb: *mut hl_cb) {
    if (*cb).is_internal { gen_pool_free((*hdev).internal_cb_pool, (*cb).kernel_address as usize, (*cb).size); }
    else { hl_asic_dma_free_coherent(hdev, (*cb).size, (*cb).kernel_address, (*cb).bus_address); }
    kfree(cb);
}

unsafe fn cb_do_release(hdev: *mut hl_device, cb: *mut hl_cb) {
    if (*cb).is_pool { atomic_set(&mut (*cb).is_handle_destroyed, 0); spin_lock(&mut (*hdev).cb_pool_lock); list_add(&mut (*cb).pool_list, &mut (*hdev).cb_pool); spin_unlock(&mut (*hdev).cb_pool_lock); }
    else { cb_fini(hdev, cb); }
}

unsafe fn hl_cb_alloc(hdev: *mut hl_device, cb_size: u32, ctx_id: i32, internal_cb: bool) -> *mut hl_cb {
    let mut cb: *mut hl_cb = core::ptr::null_mut();
    let mut cb_offset: usize;
    let mut p: *mut core::ffi::c_void;
    if ctx_id == HL_KERNEL_ASID_ID && !(*hdev).disabled { cb = kzalloc_obj::<hl_cb>(GFP_ATOMIC); }
    if cb.is_null() { cb = kzalloc_obj::<hl_cb>(GFP_KERNEL); }
    if cb.is_null() { return core::ptr::null_mut(); }
    if internal_cb {
        p = gen_pool_alloc((*hdev).internal_cb_pool, cb_size) as *mut _;
        if p.is_null() { kfree(cb); return core::ptr::null_mut(); }
        cb_offset = p as usize - (*hdev).internal_cb_pool_virt_addr as usize;
        (*cb).is_internal = true;
        (*cb).bus_address = (*hdev).internal_cb_va_base + cb_offset as u64;
    } else if ctx_id == HL_KERNEL_ASID_ID {
        p = hl_asic_dma_alloc_coherent(hdev, cb_size, &mut (*cb).bus_address, GFP_ATOMIC);
        if p.is_null() { p = hl_asic_dma_alloc_coherent(hdev, cb_size, &mut (*cb).bus_address, GFP_KERNEL); }
    } else { p = hl_asic_dma_alloc_coherent(hdev, cb_size, &mut (*cb).bus_address, GFP_USER | __GFP_ZERO); }
    if p.is_null() { dev_err((*hdev).dev, "failed to allocate %d of dma memory for CB\n", cb_size); kfree(cb); return core::ptr::null_mut(); }
    (*cb).kernel_address = p; (*cb).size = cb_size; cb
}

#[repr(C)]
struct hl_cb_mmap_mem_alloc_args { hdev: *mut hl_device, ctx: *mut hl_ctx, cb_size: u32, internal_cb: bool, map_cb: bool }

unsafe fn hl_cb_mmap_mem_release(buf: *mut hl_mmap_mem_buf) { let cb = (*buf).private as *mut hl_cb; hl_debugfs_remove_cb(cb); if (*cb).is_mmu_mapped { cb_unmap_mem((*cb).ctx, cb); } hl_ctx_put((*cb).ctx); cb_do_release((*cb).hdev, cb); }

unsafe fn hl_cb_mmap_mem_alloc(buf: *mut hl_mmap_mem_buf, _gfp: gfp_t, args: *mut core::ffi::c_void) -> i32 {
    let a = args as *mut hl_cb_mmap_mem_alloc_args; let ctx_id = (*(*a).ctx).asid; let mut cb: *mut hl_cb = core::ptr::null_mut(); let mut alloc_new = true;
    if !(*a).internal_cb { if (*a).cb_size < PAGE_SIZE { (*a).cb_size = PAGE_SIZE; } if ctx_id == HL_KERNEL_ASID_ID && (*a).cb_size <= (*(*a).hdev).asic_prop.cb_pool_cb_size { spin_lock(&mut (*(*a).hdev).cb_pool_lock); if !list_empty(&(*(*a).hdev).cb_pool) { cb = list_first_entry(&mut (*(*a).hdev).cb_pool, pool_list); list_del(&mut (*cb).pool_list); spin_unlock(&mut (*(*a).hdev).cb_pool_lock); alloc_new = false; } else { spin_unlock(&mut (*(*a).hdev).cb_pool_lock); dev_dbg((*(*a).hdev).dev, "CB pool is empty\n"); } } }
    if alloc_new { cb = hl_cb_alloc((*a).hdev, (*a).cb_size, ctx_id, (*a).internal_cb); if cb.is_null() { return -ENOMEM; } }
    (*cb).hdev = (*a).hdev; (*cb).ctx = (*a).ctx; (*cb).buf = buf; (*buf).mappable_size = (*cb).size; (*buf).private = cb; hl_ctx_get((*cb).ctx);
    if (*a).map_cb { if ctx_id == HL_KERNEL_ASID_ID { dev_err((*(*a).hdev).dev, "CB mapping is not supported for kernel context\n"); goto release_cb; } if cb_map_mem((*a).ctx, cb) != 0 { goto release_cb; } }
    hl_debugfs_add_cb(cb); return 0;
release_cb: hl_ctx_put((*cb).ctx); cb_do_release((*a).hdev, cb); -EINVAL
}

unsafe fn hl_cb_mmap(buf: *mut hl_mmap_mem_buf, vma: *mut vm_area_struct, _args: *mut core::ffi::c_void) -> i32 { let cb = (*buf).private as *mut hl_cb; ((*(*cb).hdev).asic_funcs).mmap((*cb).hdev, vma, (*cb).kernel_address, (*cb).bus_address, (*cb).size) }

static mut cb_behavior: hl_mmap_mem_buf_behavior = hl_mmap_mem_buf_behavior { topic: "CB", mem_id: HL_MMAP_TYPE_CB, alloc: hl_cb_mmap_mem_alloc, release: hl_cb_mmap_mem_release, mmap: hl_cb_mmap };

pub unsafe fn hl_cb_create(hdev: *mut hl_device, mmg: *mut hl_mem_mgr, ctx: *mut hl_ctx, cb_size: u32, internal_cb: bool, map_cb: bool, handle: *mut u64) -> i32 {
    if (*hdev).disabled || ((*hdev).reset_info.in_reset && (*ctx).asid != HL_KERNEL_ASID_ID) { dev_warn_ratelimited((*hdev).dev, "Device is disabled or in reset. Can't create new CBs\n"); return -EBUSY; }
    if cb_size > SZ_2M { dev_err((*hdev).dev, "CB size %d must be less than %d\n", cb_size, SZ_2M); return -EINVAL; }
    let mut args = hl_cb_mmap_mem_alloc_args { hdev, ctx, cb_size, internal_cb, map_cb };
    let buf = hl_mmap_mem_buf_alloc(mmg, &mut cb_behavior, if (*ctx).asid == HL_KERNEL_ASID_ID { GFP_ATOMIC } else { GFP_KERNEL }, &mut args);
    if buf.is_null() { return -ENOMEM; } *handle = (*buf).handle; 0
}

pub unsafe fn hl_cb_destroy(mmg: *mut hl_mem_mgr, cb_handle: u64) -> i32 {
    let cb = hl_cb_get(mmg, cb_handle); if cb.is_null() { dev_dbg((*mmg).dev, "CB destroy failed, no CB was found for handle %#llx\n", cb_handle); return -EINVAL; }
    let rc = atomic_cmpxchg(&mut (*cb).is_handle_destroyed, 0, 1); hl_cb_put(cb); if rc != 0 { return -EINVAL; }
    let rc = hl_mmap_mem_buf_put_handle(mmg, cb_handle); if rc < 0 { return rc; } if rc == 0 { dev_dbg((*mmg).dev, "CB 0x%llx is destroyed while still in use\n", cb_handle); } 0
}

unsafe fn hl_cb_info(mmg: *mut hl_mem_mgr, handle: u64, flags: u32, usage_cnt: *mut u32, device_va: *mut u64) -> i32 { let cb = hl_cb_get(mmg, handle); if cb.is_null() { return -EINVAL; } let mut rc = 0; if flags & HL_CB_FLAGS_GET_DEVICE_VA != 0 { if (*cb).is_mmu_mapped { *device_va = (*cb).virtual_addr; } else { rc = -EINVAL; } } else { *usage_cnt = atomic_read(&(*cb).cs_cnt) as u32; } hl_cb_put(cb); rc }

pub unsafe fn hl_cb_ioctl(_ddev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> i32 { let hpriv = (*file_priv).driver_priv; let hdev = (*hpriv).hdev; let args = data as *mut hl_cb_args; let mut handle = 0u64; let mut device_va = 0u64; let mut usage_cnt = 0u32; let mut status = core::mem::zeroed(); if !hl_device_operational(hdev, &mut status) { return -EBUSY; } let rc = match (*args).in_.op { HL_CB_OP_CREATE => { if (*args).in_.cb_size > HL_MAX_CB_SIZE { -EINVAL } else { hl_cb_create(hdev, &mut (*hpriv).mem_mgr, (*hpriv).ctx, (*args).in_.cb_size, false, (*args).in_.flags & HL_CB_FLAGS_MAP != 0, &mut handle) } }, HL_CB_OP_DESTROY => hl_cb_destroy(&mut (*hpriv).mem_mgr, (*args).in_.cb_handle), HL_CB_OP_INFO => hl_cb_info(&mut (*hpriv).mem_mgr, (*args).in_.cb_handle, (*args).in_.flags, &mut usage_cnt, &mut device_va), _ => -EINVAL }; if (*args).in_.op == HL_CB_OP_CREATE { core::ptr::write_bytes(args, 0, 1); (*args).out.cb_handle = handle; } else if (*args).in_.op == HL_CB_OP_INFO && rc == 0 { if (*args).in_.flags & HL_CB_FLAGS_GET_DEVICE_VA != 0 { (*args).out.device_va = device_va; } else { (*args).out.usage_cnt = usage_cnt; } } rc }

pub unsafe fn hl_cb_get(mmg: *mut hl_mem_mgr, handle: u64) -> *mut hl_cb { let buf = hl_mmap_mem_buf_get(mmg, handle); if buf.is_null() { core::ptr::null_mut() } else { (*buf).private as *mut hl_cb } }
pub unsafe fn hl_cb_put(cb: *mut hl_cb) { hl_mmap_mem_buf_put((*cb).buf); }
pub unsafe fn hl_cb_kernel_create(hdev: *mut hl_device, cb_size: u32, internal_cb: bool) -> *mut hl_cb { let mut handle = 0; if hl_cb_create(hdev, &mut (*hdev).kernel_mem_mgr, (*hdev).kernel_ctx, cb_size, internal_cb, false, &mut handle) != 0 { return core::ptr::null_mut(); } let cb = hl_cb_get(&mut (*hdev).kernel_mem_mgr, handle); if !cb.is_null() { cb } else { hl_cb_destroy(&mut (*hdev).kernel_mem_mgr, handle); core::ptr::null_mut() } }

pub unsafe fn hl_cb_pool_init(hdev: *mut hl_device) -> i32 { INIT_LIST_HEAD(&mut (*hdev).cb_pool); spin_lock_init(&mut (*hdev).cb_pool_lock); for _ in 0..(*hdev).asic_prop.cb_pool_cb_cnt { let cb = hl_cb_alloc(hdev, (*hdev).asic_prop.cb_pool_cb_size, HL_KERNEL_ASID_ID, false); if cb.is_null() { hl_cb_pool_fini(hdev); return -ENOMEM; } (*cb).is_pool = true; list_add(&mut (*cb).pool_list, &mut (*hdev).cb_pool); } 0 }
pub unsafe fn hl_cb_pool_fini(hdev: *mut hl_device) -> i32 { let mut pos = (*hdev).cb_pool.head; while !pos.is_null() { let cb = list_entry(pos, hl_cb, pool_list); pos = (*pos).next; list_del(&mut (*cb).pool_list); cb_fini(hdev, cb); } 0 }
pub unsafe fn hl_cb_va_pool_init(ctx: *mut hl_ctx) -> i32 { if !(*(*ctx).hdev).supports_cb_mapping { return 0; } (*ctx).cb_va_pool = gen_pool_create(__ffs((*(*ctx).hdev).asic_prop.pmmu.page_size), -1); if (*ctx).cb_va_pool.is_null() { return -ENOMEM; } (*ctx).cb_va_pool_base = hl_reserve_va_block((*ctx).hdev, ctx, HL_VA_RANGE_TYPE_HOST, CB_VA_POOL_SIZE, HL_MMU_VA_ALIGNMENT_NOT_NEEDED); if (*ctx).cb_va_pool_base == 0 { gen_pool_destroy((*ctx).cb_va_pool); return -ENOMEM; } let rc = gen_pool_add((*ctx).cb_va_pool, (*ctx).cb_va_pool_base, CB_VA_POOL_SIZE, -1); if rc != 0 { hl_unreserve_va_block((*ctx).hdev, ctx, (*ctx).cb_va_pool_base, CB_VA_POOL_SIZE); gen_pool_destroy((*ctx).cb_va_pool); } rc }
pub unsafe fn hl_cb_va_pool_fini(ctx: *mut hl_ctx) { if !(*(*ctx).hdev).supports_cb_mapping { return; } gen_pool_destroy((*ctx).cb_va_pool); hl_unreserve_va_block((*ctx).hdev, ctx, (*ctx).cb_va_pool_base, CB_VA_POOL_SIZE); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
