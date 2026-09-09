// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2025 Intel Corporation
 */

// Linux DRM, dma-buf, memory-management, scatterlist, and IVPU driver
// dependencies are supplied by the surrounding kernel/Rust bindings.

unsafe fn ivpu_gem_userptr_dmabuf_map(
    attachment: *mut dma_buf_attachment,
    direction: dma_data_direction,
) -> *mut sg_table {
    let sgt = (*(*attachment).dmabuf).priv_;
    let ret = dma_map_sgtable(
        (*attachment).dev,
        sgt,
        direction,
        DMA_ATTR_SKIP_CPU_SYNC,
    );
    if ret != 0 {
        return ERR_PTR(ret);
    }
    sgt
}

unsafe fn ivpu_gem_userptr_dmabuf_unmap(
    attachment: *mut dma_buf_attachment,
    sgt: *mut sg_table,
    direction: dma_data_direction,
) {
    dma_unmap_sgtable(
        (*attachment).dev,
        sgt,
        direction,
        DMA_ATTR_SKIP_CPU_SYNC,
    );
}

unsafe fn ivpu_gem_userptr_dmabuf_release(dma_buf: *mut dma_buf) {
    let sgt = (*dma_buf).priv_;
    let mut page_iter = sg_page_iter::default();
    let mut page: *mut page;

    // for_each_sgtable_page(sgt, &page_iter, 0)
    for_each_sgtable_page!(sgt, &mut page_iter, 0, {
        page = sg_page_iter_page(&page_iter);
        unpin_user_page(page);
    });

    sg_free_table(sgt);
    kfree(sgt);
}

static IVPU_GEM_USERPTR_DMABUF_OPS: dma_buf_ops = dma_buf_ops {
    map_dma_buf: Some(ivpu_gem_userptr_dmabuf_map),
    unmap_dma_buf: Some(ivpu_gem_userptr_dmabuf_unmap),
    release: Some(ivpu_gem_userptr_dmabuf_release),
};

unsafe fn ivpu_create_userptr_dmabuf(
    vdev: *mut ivpu_device,
    user_ptr: *mut core::ffi::c_void,
    size: usize,
    flags: u32,
) -> *mut dma_buf {
    let mut exp_info: dma_buf_export_info = core::mem::zeroed();
    let dma_buf: *mut dma_buf;
    let sgt: *mut sg_table;
    let pages: *mut *mut page;
    let nr_pages: usize = size >> PAGE_SHIFT;
    let mut gup_flags: u32 = FOLL_LONGTERM;
    let mut ret: i32;
    let mut i: i32;
    let pinned: i32;

    // Add FOLL_WRITE only if the BO is not read-only
    if flags & DRM_IVPU_BO_READ_ONLY == 0 {
        gup_flags |= FOLL_WRITE;
    }

    pages = kvmalloc_objs!(page, nr_pages);
    if pages.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    pinned = pin_user_pages_fast(user_ptr as usize, nr_pages, gup_flags, pages);
    if pinned < 0 {
        ret = pinned;
        ivpu_dbg!(vdev, IOCTL, "Failed to pin user pages: %d\n", ret);
        goto!(free_pages_array);
    }

    if pinned as usize != nr_pages {
        ivpu_dbg!(vdev, IOCTL, "Pinned %d pages, expected %lu\n", pinned, nr_pages);
        ret = -EFAULT;
        goto!(unpin_pages);
    }

    sgt = kmalloc_obj!(sg_table);
    if sgt.is_null() {
        ret = -ENOMEM;
        goto!(unpin_pages);
    }

    ret = sg_alloc_table_from_pages(sgt, pages, nr_pages, 0, size, GFP_KERNEL);
    if ret != 0 {
        ivpu_dbg!(vdev, IOCTL, "Failed to create sg table: %d\n", ret);
        goto!(free_sgt);
    }

    exp_info.exp_name = c"ivpu_userptr_dmabuf".as_ptr();
    exp_info.owner = THIS_MODULE;
    exp_info.ops = &IVPU_GEM_USERPTR_DMABUF_OPS;
    exp_info.size = size;
    exp_info.flags = O_RDWR | O_CLOEXEC;
    exp_info.priv_ = sgt;

    dma_buf = dma_buf_export(&exp_info);
    if IS_ERR!(dma_buf) {
        ret = PTR_ERR(dma_buf);
        ivpu_dbg!(vdev, IOCTL, "Failed to export userptr dma-buf: %d\n", ret);
        goto!(free_sg_table);
    }

    kvfree(pages);
    return dma_buf;

free_sg_table:
    sg_free_table(sgt);
free_sgt:
    kfree(sgt);
unpin_pages:
    i = 0;
    while i < pinned {
        unpin_user_page(*pages.add(i as usize));
        i += 1;
    }
free_pages_array:
    kvfree(pages);
    ERR_PTR(ret)
}

unsafe fn ivpu_bo_create_from_userptr(
    vdev: *mut ivpu_device,
    user_ptr: *mut core::ffi::c_void,
    size: usize,
    flags: u32,
) -> *mut ivpu_bo {
    let dma_buf = ivpu_create_userptr_dmabuf(vdev, user_ptr, size, flags);
    if IS_ERR!(dma_buf) {
        return ERR_CAST!(dma_buf);
    }

    let obj = ivpu_gem_prime_import(&mut (*vdev).drm, dma_buf);
    if IS_ERR!(obj) {
        dma_buf_put(dma_buf);
        return ERR_CAST!(obj);
    }

    dma_buf_put(dma_buf);

    let bo = to_ivpu_bo(obj);
    (*bo).flags = flags;
    bo
}

unsafe fn ivpu_bo_create_from_userptr_ioctl(
    dev: *mut drm_device,
    data: *mut core::ffi::c_void,
    file: *mut drm_file,
) -> i32 {
    let args = data as *mut drm_ivpu_bo_create_from_userptr;
    let file_priv = (*file).driver_priv as *mut ivpu_file_priv;
    let vdev = to_ivpu_device(dev);
    let user_ptr = u64_to_user_ptr((*args).user_ptr);
    let bo: *mut ivpu_bo;
    let mut ret: i32;

    if (*args).flags & !(DRM_IVPU_BO_HIGH_MEM | DRM_IVPU_BO_DMA_MEM | DRM_IVPU_BO_READ_ONLY) != 0 {
        ivpu_dbg!(vdev, IOCTL, "Invalid BO flags: 0x%x\n", (*args).flags);
        return -EINVAL;
    }

    if (*args).user_ptr == 0 || (*args).size == 0 {
        ivpu_dbg!(vdev, IOCTL, "Userptr or size are zero: ptr %llx size %llu\n", (*args).user_ptr, (*args).size);
        return -EINVAL;
    }

    if !PAGE_ALIGNED!((*args).user_ptr) || !PAGE_ALIGNED!((*args).size) {
        ivpu_dbg!(vdev, IOCTL, "Userptr or size not page aligned: ptr %llx size %llu\n", (*args).user_ptr, (*args).size);
        return -EINVAL;
    }

    if !access_ok(user_ptr, (*args).size) {
        ivpu_dbg!(vdev, IOCTL, "Userptr is not accessible: ptr %llx size %llu\n", (*args).user_ptr, (*args).size);
        return -EFAULT;
    }

    bo = ivpu_bo_create_from_userptr(vdev, user_ptr, (*args).size, (*args).flags);
    if IS_ERR!(bo) {
        return PTR_ERR(bo);
    }

    ret = drm_gem_handle_create(file, &mut (*bo).base.base, &mut (*args).handle);
    if ret != 0 {
        ivpu_dbg!(vdev, IOCTL, "Failed to create handle for BO: %pe ctx %u size %llu flags 0x%x\n", bo, (*file_priv).ctx.id, (*args).size, (*args).flags);
    } else {
        ivpu_dbg!(vdev, BO, "Created userptr BO: handle=%u vpu_addr=0x%llx size=%llu flags=0x%x\n", (*args).handle, (*bo).vpu_addr, (*args).size, (*bo).flags);
        (*args).vpu_addr = (*bo).vpu_addr;
    }

    drm_gem_object_put(&mut (*bo).base.base);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
