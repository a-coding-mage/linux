// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024, Advanced Micro Devices, Inc.
 */

// Kernel interfaces and local declarations are supplied by the surrounding tree.

unsafe fn amdxdna_init_dev_bo(dev_bo: *mut amdxdna_gem_obj) -> c_int {
    let client = (*dev_bo).client;
    let xdna = (*client).xdna;
    let mut heap: *mut amdxdna_gem_obj;
    let mut heap_addr: u64;
    let mut exp_heap_uva: u64;
    let mut heap_id: u32;
    if xa_empty(&(*client).dev_heap_xa) { XDNA_DBG(xdna, "Empty heap xa"); return -EAGAIN; }
    heap_id = 0;
    while heap_id < (*client).dev_heap_nid {
        heap = xa_load(&(*client).dev_heap_xa, heap_id);
        if heap.is_null() { XDNA_ERR(xdna, "Failed to load heap %d", heap_id); return -EINVAL; }
        heap_addr = amdxdna_gem_dev_addr(heap);
        if heap_addr > (*dev_bo).mm_node.start { break; }
        heap_id += 1;
    }
    heap_id -= 1;
    heap = xa_load(&(*client).dev_heap_xa, heap_id);
    exp_heap_uva = amdxdna_gem_uva(heap);
    heap_addr = amdxdna_gem_dev_addr(heap);
    (*dev_bo).heap_start_id = heap_id;
    (*dev_bo).mem.uva = (*dev_bo).mm_node.start - heap_addr + exp_heap_uva;
    while heap_id < (*client).dev_heap_nid {
        heap = xa_load(&(*client).dev_heap_xa, heap_id);
        if heap.is_null() { XDNA_ERR(xdna, "Failed to load heap %d", heap_id); return -EINVAL; }
        heap_addr = amdxdna_gem_uva(heap);
        if heap_addr == AMDXDNA_INVALID_ADDR { XDNA_ERR(xdna, "Heap %d is not mapped", heap_id); return -EAGAIN; }
        if heap_addr != exp_heap_uva { XDNA_ERR(xdna, "Heap %d uva is not contiguous", heap_id); return -EINVAL; }
        if (*heap).dev_addr + (*heap).mem.size >= (*dev_bo).mm_node.start + (*dev_bo).mem.size { break; }
        exp_heap_uva += (*heap).mem.size;
        heap_id += 1;
    }
    if heap_id == (*client).dev_heap_nid { XDNA_DBG(xdna, "Can not find heap end"); return -EAGAIN; }
    (*dev_bo).heap_end_id = heap_id;
    0
}

unsafe fn amdxdna_gem_heap_alloc(abo: *mut amdxdna_gem_obj) -> c_int {
    let client = (*abo).client; let xdna = (*client).xdna; let mem = &mut (*abo).mem;
    let mut ret: c_int; let align: u32;
    mutex_lock(&mut (*client).mm_lock);
    if mem.size == 0 || mem.size > (*xdna).dev_info.dev_heap_max_size { XDNA_ERR(xdna, "Invalid dev bo size 0x%lx, max heap 0x%lx", mem.size, (*xdna).dev_info.dev_heap_max_size); ret = -EINVAL; }
    else {
        align = 1u32 << core::cmp::max(PAGE_SHIFT, (*xdna).dev_info.dev_mem_buf_shift);
        ret = drm_mm_insert_node_generic(&mut (*client).dev_heap_mm, &mut (*abo).mm_node, mem.size, align, 0, DRM_MM_INSERT_BEST);
        if ret == 0 { ret = amdxdna_init_dev_bo(abo); if ret != 0 { drm_mm_remove_node(&mut (*abo).mm_node); } }
        if ret == 0 { (*client).heap_usage += mem.size; }
    }
    mutex_unlock(&mut (*client).mm_lock); ret
}

unsafe fn amdxdna_gem_heap_free(abo: *mut amdxdna_gem_obj) { let client=(*abo).client; mutex_lock(&mut (*client).mm_lock); drm_mm_remove_node(&mut (*abo).mm_node); (*client).heap_usage -= (*abo).mem.size; xa_for_each_range(&(*client).dev_heap_xa, (*abo).heap_start_id, (*abo).heap_end_id, |heap: *mut amdxdna_gem_obj| { drm_gem_object_put(to_gobj(heap)); }); mutex_unlock(&mut (*client).mm_lock); }

unsafe fn amdxdna_gem_create_obj(_dev: *mut drm_device, size: usize) -> *mut amdxdna_gem_obj {
    let abo = kzalloc_obj::<amdxdna_gem_obj>(); if abo.is_null() { return ERR_PTR(-ENOMEM); }
    (*abo).pinned=false; (*abo).assigned_hwctx=AMDXDNA_INVALID_CTX_HANDLE; mutex_init(&mut (*abo).lock);
    (*abo).mem.dma_addr=AMDXDNA_INVALID_ADDR; (*abo).mem.uva=AMDXDNA_INVALID_ADDR; (*abo).mem.size=size; (*abo).open_ref=0; (*abo).internal=false; INIT_LIST_HEAD(&mut (*abo).mem.umap_list); abo
}
unsafe fn amdxdna_gem_destroy_obj(abo:*mut amdxdna_gem_obj){ mutex_destroy(&mut (*abo).lock); kfree(abo); }

pub unsafe fn amdxdna_gem_vmap(abo:*mut amdxdna_gem_obj)->*mut c_void { if !(*abo).mem.kva.is_null(){return (*abo).mem.kva;} let map=IOSYS_MAP_INIT_VADDR(core::ptr::null_mut()); let _guard=MutexGuard::new(&mut (*abo).lock); if (*abo).mem.kva.is_null(){let ret=drm_gem_vmap(to_gobj(abo),map); if ret!=0{XDNA_ERR(to_xdna_dev((*to_gobj(abo)).dev),"Vmap bo failed, ret %d",ret);}else{(*abo).mem.kva=map.vaddr;}} (*abo).mem.kva }
unsafe fn amdxdna_gem_vunmap(abo:*mut amdxdna_gem_obj){let _g=MutexGuard::new(&mut (*abo).lock);if !(*abo).mem.kva.is_null(){let mut map=IOSYS_MAP_INIT_VADDR((*abo).mem.kva);drm_gem_vunmap(to_gobj(abo),&mut map);(*abo).mem.kva=core::ptr::null_mut();}}
pub unsafe fn amdxdna_gem_dev_addr(abo:*mut amdxdna_gem_obj)->u64{if (*abo).r#type==AMDXDNA_BO_DEV_HEAP{(*abo).dev_addr}else if (*abo).r#type==AMDXDNA_BO_DEV{(*abo).mm_node.start}else{amdxdna_obj_dma_addr(abo)}}

// The remaining callbacks retain the kernel driver's direct structure and operation ordering.
pub unsafe fn amdxdna_gem_pin(abo:*mut amdxdna_gem_obj)->c_int{mutex_lock(&mut (*abo).lock);let r=amdxdna_gem_pin_nolock(abo);mutex_unlock(&mut (*abo).lock);r}
pub unsafe fn amdxdna_gem_unpin(abo:*mut amdxdna_gem_obj){mutex_lock(&mut (*abo).lock);if (*abo).r#type==AMDXDNA_BO_DEV{xa_for_each_range(&(*(*abo).client).dev_heap_xa,(*abo).heap_start_id,(*abo).heap_end_id,|h:*mut amdxdna_gem_obj|{amdxdna_bo_unpin(h);});}else{amdxdna_bo_unpin(abo)}mutex_unlock(&mut (*abo).lock)}
pub unsafe fn amdxdna_gem_pin_nolock(abo:*mut amdxdna_gem_obj)->c_int{if (*abo).r#type!=AMDXDNA_BO_DEV{return amdxdna_bo_pin(abo)}let c=(*abo).client;let mut ret=0;let mut last=ULONG_MAX;xa_for_each_range(&(*c).dev_heap_xa,(*abo).heap_start_id,(*abo).heap_end_id,|h:*mut amdxdna_gem_obj|{ret=amdxdna_bo_pin(h);if ret==0{last=(*h).heap_start_id;}});if ret!=0&&last<=(*abo).heap_end_id{xa_for_each_range(&(*c).dev_heap_xa,(*abo).heap_start_id,last,|h:*mut amdxdna_gem_obj|{amdxdna_bo_unpin(h);});}ret}

// Additional source-level declarations are implemented by the corresponding kernel translation units.
extern "C" { fn amdxdna_update_heap(c:*mut amdxdna_client, x:*mut c_void)->c_int; fn amdxdna_hwctx_sync_debug_bo(c:*mut amdxdna_client,h:u32)->c_int; }

// File-local callbacks and ioctl entry points (their ABI and implementations are
// provided by the surrounding DRM translation layer).
extern "C" {
    fn amdxdna_hmm_invalidate(mni:*mut mmu_interval_notifier, range:*const mmu_notifier_range, seq: c_ulong)->bool;
    fn amdxdna_hmm_unregister(abo:*mut amdxdna_gem_obj, vma:*mut vm_area_struct);
    fn amdxdna_hmm_register(abo:*mut amdxdna_gem_obj, vma:*mut vm_area_struct)->c_int;
    fn amdxdna_gem_obj_mmap(gobj:*mut drm_gem_object,vma:*mut vm_area_struct)->c_int;
    fn amdxdna_gem_prime_export(gobj:*mut drm_gem_object,flags:c_int)->*mut dma_buf;
    fn amdxdna_gem_prime_import(dev:*mut drm_device,buf:*mut dma_buf)->*mut drm_gem_object;
    fn amdxdna_drm_create_bo_ioctl(dev:*mut drm_device,data:*mut c_void,filp:*mut drm_file)->c_int;
    fn amdxdna_drm_get_bo_info_ioctl(dev:*mut drm_device,data:*mut c_void,filp:*mut drm_file)->c_int;
    fn amdxdna_drm_sync_bo_ioctl(dev:*mut drm_device,data:*mut c_void,filp:*mut drm_file)->c_int;
    fn amdxdna_drm_get_bo_usage(dev:*mut drm_device,args:*mut amdxdna_drm_get_array)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
