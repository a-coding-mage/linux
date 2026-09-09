// SPDX-License-Identifier: GPL-2.0-only
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// Dependencies supplied by the surrounding kernel/Rocket bindings are intentionally
// referenced here rather than reimplemented in this translation unit.

unsafe fn rocket_gem_bo_free(obj: *mut drm_gem_object) {
    let bo: *mut rocket_gem_object = to_rocket_bo(obj);
    let rocket_priv: *mut rocket_file_priv = (*bo).driver_priv;
    let unmapped: usize;

    drm_WARN_ON((*obj).dev, refcount_read(&(*bo).base.pages_use_count) > 1);

    unmapped = iommu_unmap((*bo).domain->domain, (*bo).mm.start, (*bo).size);
    drm_WARN_ON((*obj).dev, unmapped != (*bo).size);

    mutex_lock(&mut (*rocket_priv).mm_lock);
    drm_mm_remove_node(&mut (*bo).mm);
    mutex_unlock(&mut (*rocket_priv).mm_lock);

    rocket_iommu_domain_put((*bo).domain);
    (*bo).domain = core::ptr::null_mut();

    drm_gem_shmem_free(&mut (*bo).base);
}

static rocket_gem_funcs: drm_gem_object_funcs = drm_gem_object_funcs {
    free: Some(rocket_gem_bo_free),
    print_info: Some(drm_gem_shmem_object_print_info),
    pin: Some(drm_gem_shmem_object_pin),
    unpin: Some(drm_gem_shmem_object_unpin),
    get_sg_table: Some(drm_gem_shmem_object_get_sg_table),
    vmap: Some(drm_gem_shmem_object_vmap),
    vunmap: Some(drm_gem_shmem_object_vunmap),
    mmap: Some(drm_gem_shmem_object_mmap),
    vm_ops: &drm_gem_shmem_vm_ops,
};

unsafe fn rocket_gem_create_object(dev: *mut drm_device, size: usize) -> *mut drm_gem_object {
    let obj: *mut rocket_gem_object;

    obj = kzalloc_obj::<rocket_gem_object>();
    if obj.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*obj).base.base.funcs = &rocket_gem_funcs;

    &mut (*obj).base.base
}

unsafe fn rocket_ioctl_create_bo(
    dev: *mut drm_device,
    data: *mut core::ffi::c_void,
    file: *mut drm_file,
) -> i32 {
    let rocket_priv: *mut rocket_file_priv = (*file).driver_priv;
    let args: *mut drm_rocket_create_bo = data as *mut drm_rocket_create_bo;
    let mut shmem_obj: *mut drm_gem_shmem_object;
    let mut rkt_obj: *mut rocket_gem_object;
    let mut gem_obj: *mut drm_gem_object;
    let mut sgt: *mut sg_table;
    let mut ret: i32;

    shmem_obj = drm_gem_shmem_create(dev, (*args).size);
    if IS_ERR(shmem_obj) {
        return PTR_ERR(shmem_obj);
    }

    gem_obj = &mut (*shmem_obj).base;
    rkt_obj = to_rocket_bo(gem_obj);

    (*rkt_obj).driver_priv = rocket_priv;
    (*rkt_obj).domain = rocket_iommu_domain_get(rocket_priv);
    (*rkt_obj).size = (*args).size;
    (*rkt_obj).offset = 0;

    sgt = drm_gem_shmem_get_pages_sgt(shmem_obj);
    if IS_ERR(sgt) {
        ret = PTR_ERR(sgt);
        goto err;
    }

    mutex_lock(&mut (*rocket_priv).mm_lock);
    ret = drm_mm_insert_node_generic(&mut (*rocket_priv).mm, &mut (*rkt_obj).mm,
                                     (*rkt_obj).size, PAGE_SIZE, 0, 0);
    mutex_unlock(&mut (*rocket_priv).mm_lock);
    if ret != 0 {
        goto err;
    }

    ret = iommu_map_sgtable((*rocket_priv).domain->domain, (*rkt_obj).mm.start,
                            (*shmem_obj).sgt, IOMMU_READ | IOMMU_WRITE);
    if ret < 0 || ret < (*args).size as i32 {
        drm_err(dev, "failed to map buffer: size=%d request_size=%u\n", ret, (*args).size);
        ret = -ENOMEM;
        goto err_remove_node;
    }

    // iommu_map_sgtable might have aligned the size
    (*rkt_obj).size = ret as usize;
    (*args).offset = drm_vma_node_offset_addr(&(*gem_obj).vma_node);
    (*args).dma_address = (*rkt_obj).mm.start;

    ret = drm_gem_handle_create(file, gem_obj, &mut (*args).handle);
    if ret != 0 {
        goto err_unmap;
    }

    drm_gem_object_put(gem_obj);
    return 0;

err_unmap:
    iommu_unmap((*rocket_priv).domain->domain, (*rkt_obj).mm.start, (*rkt_obj).size);

err_remove_node:
    mutex_lock(&mut (*rocket_priv).mm_lock);
    drm_mm_remove_node(&mut (*rkt_obj).mm);
    mutex_unlock(&mut (*rocket_priv).mm_lock);

err:
    drm_gem_shmem_object_free(gem_obj);
    ret
}

unsafe fn rocket_ioctl_prep_bo(
    dev: *mut drm_device,
    data: *mut core::ffi::c_void,
    file: *mut drm_file,
) -> i64 {
    let args: *mut drm_rocket_prep_bo = data as *mut drm_rocket_prep_bo;
    let timeout: ulong = drm_timeout_abs_to_jiffies((*args).timeout_ns);
    let mut gem_obj: *mut drm_gem_object;
    let shmem_obj: *mut drm_gem_shmem_object;
    let mut ret: i64 = 0;

    if (*args).reserved != 0 {
        drm_dbg(dev, "Reserved field in drm_rocket_prep_bo struct should be 0.\n");
        return -EINVAL as i64;
    }

    gem_obj = drm_gem_object_lookup(file, (*args).handle);
    if gem_obj.is_null() {
        return -ENOENT as i64;
    }

    ret = dma_resv_wait_timeout((*gem_obj).resv, DMA_RESV_USAGE_WRITE, true, timeout);
    if ret == 0 {
        ret = if timeout != 0 { -ETIMEDOUT } else { -EBUSY };
    } else if ret > 0 {
        ret = 0;
    }

    shmem_obj = &mut to_rocket_bo(gem_obj).base;
    dma_sync_sgtable_for_cpu((*dev).dev, (*shmem_obj).sgt, DMA_BIDIRECTIONAL);
    drm_gem_object_put(gem_obj);
    ret
}

unsafe fn rocket_ioctl_fini_bo(
    dev: *mut drm_device,
    data: *mut core::ffi::c_void,
    file: *mut drm_file,
) -> i32 {
    let args: *mut drm_rocket_fini_bo = data as *mut drm_rocket_fini_bo;
    let shmem_obj: *mut drm_gem_shmem_object;
    let rkt_obj: *mut rocket_gem_object;
    let gem_obj: *mut drm_gem_object;

    if (*args).reserved != 0 {
        drm_dbg(dev, "Reserved field in drm_rocket_fini_bo struct should be 0.\n");
        return -EINVAL;
    }

    gem_obj = drm_gem_object_lookup(file, (*args).handle);
    if gem_obj.is_null() {
        return -ENOENT;
    }

    rkt_obj = to_rocket_bo(gem_obj);
    shmem_obj = &mut (*rkt_obj).base;
    dma_sync_sgtable_for_device((*dev).dev, (*shmem_obj).sgt, DMA_BIDIRECTIONAL);
    drm_gem_object_put(gem_obj);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
