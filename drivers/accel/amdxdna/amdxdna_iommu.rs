// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/driver translation.

static mut force_iova: bool = false;

unsafe fn amdxdna_iommu_alloc_iova(
    xdna: *mut amdxdna_dev,
    mut size: usize,
    dma_addr: *mut dma_addr_t,
    size_aligned: bool,
) -> *mut iova {
    let shift: c_ulong;
    let end: c_ulong;
    let iova: *mut iova;

    end = (*(*xdna).domain).geometry.aperture_end;
    shift = iova_shift(&(*xdna).iovad);
    size = iova_align(&(*xdna).iovad, size);

    iova = alloc_iova(&mut (*xdna).iovad, size >> shift, end >> shift, size_aligned);
    if iova.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    *dma_addr = iova_dma_addr(&(*xdna).iovad, iova);

    iova
}

pub unsafe fn amdxdna_dma_map_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj) -> c_int {
    let mut contig_sz: c_ulong;
    let sgt: *mut sg_table;
    let mut dma_addr: dma_addr_t = 0;
    let mut iova: *mut iova;
    let mut size: ssize_t;

    if (*abo).type_ != AMDXDNA_BO_DEV_HEAP && (*abo).type_ != AMDXDNA_BO_SHARE {
        return 0;
    }

    sgt = drm_gem_shmem_get_pages_sgt(&mut (*abo).base);
    if IS_ERR(sgt) {
        XDNA_ERR!(xdna, "Get sgt failed, ret %ld", PTR_ERR(sgt));
        return PTR_ERR(sgt);
    }

    if (*sgt).orig_nents == 0 {
        XDNA_ERR!(xdna, "sgl is zero length");
        return -EOPNOTSUPP;
    }

    if amdxdna_iova_on(xdna) {
        if sg_page((*sgt).sgl).is_null() {
            XDNA_ERR!(xdna, "sgl is not page backed");
            return -EOPNOTSUPP;
        }

        iova = amdxdna_iommu_alloc_iova(
            xdna,
            (*abo).mem.size,
            &mut dma_addr,
            (*abo).type_ == AMDXDNA_BO_DEV_HEAP,
        );
        if IS_ERR(iova) {
            XDNA_ERR!(xdna, "Alloc iova failed, ret %ld", PTR_ERR(iova));
            return PTR_ERR(iova);
        }

        size = iommu_map_sgtable(
            (*xdna).domain,
            dma_addr,
            sgt,
            IOMMU_READ | IOMMU_WRITE,
        );
        if size < 0 {
            XDNA_ERR!(xdna, "iommu_map_sgtable failed: %zd", size);
            __free_iova(&mut (*xdna).iovad, iova);
            return size as c_int;
        }
        if size < (*abo).mem.size as ssize_t {
            iommu_unmap((*xdna).domain, dma_addr, size as usize);
            __free_iova(&mut (*xdna).iovad, iova);
            return -ENXIO;
        }
        (*abo).mem.dma_addr = dma_addr;
    } else {
        /* Device doesn't support scatter/gather list, fail non-contiguous mapping. */
        contig_sz = drm_prime_get_contiguous_size(sgt);
        if contig_sz < (*abo).mem.size {
            XDNA_ERR!(
                xdna,
                "noncontiguous dma addr, contig size:%ld, expected size:%ld",
                contig_sz,
                (*abo).mem.size
            );
            return -EINVAL;
        }
        (*abo).mem.dma_addr = sg_dma_address((*sgt).sgl);
    }
    0
}

pub unsafe fn amdxdna_dma_unmap_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj) {
    let size: usize;

    if (*abo).mem.dma_addr == AMDXDNA_INVALID_ADDR {
        return;
    }

    if amdxdna_iova_on(xdna) {
        size = iova_align(&(*xdna).iovad, (*abo).mem.size);
        iommu_unmap((*xdna).domain, (*abo).mem.dma_addr, size);
        free_iova(
            &mut (*xdna).iovad,
            iova_pfn(&(*xdna).iovad, (*abo).mem.dma_addr),
        );
    }
    (*abo).mem.dma_addr = AMDXDNA_INVALID_ADDR;
}

pub unsafe fn amdxdna_iommu_alloc(
    xdna: *mut amdxdna_dev,
    size: usize,
    dma_addr: *mut dma_addr_t,
) -> *mut c_void {
    let iova: *mut iova;
    let cpu_addr: *mut c_void;
    let mut ret: c_int;

    iova = amdxdna_iommu_alloc_iova(xdna, size, dma_addr, true);
    if IS_ERR(iova) {
        XDNA_ERR!(xdna, "Alloc iova failed, ret %ld", PTR_ERR(iova));
        return iova as *mut c_void;
    }

    cpu_addr = __get_free_pages(GFP_KERNEL, get_order(size)) as *mut c_void;
    if cpu_addr.is_null() {
        ret = -ENOMEM;
        goto free_iova;
    }

    ret = iommu_map(
        (*xdna).domain,
        *dma_addr,
        virt_to_phys(cpu_addr),
        iova_align(&(*xdna).iovad, size),
        IOMMU_READ | IOMMU_WRITE,
        GFP_KERNEL,
    );
    if ret != 0 {
        goto free_cpu_addr;
    }

    cpu_addr

    free_cpu_addr:
    free_pages(cpu_addr as c_ulong, get_order(size));
    free_iova:
    __free_iova(&mut (*xdna).iovad, iova);
    ERR_PTR(ret) as *mut c_void
}

pub unsafe fn amdxdna_iommu_free(
    xdna: *mut amdxdna_dev,
    size: usize,
    cpu_addr: *mut c_void,
    dma_addr: dma_addr_t,
) {
    iommu_unmap((*xdna).domain, dma_addr, iova_align(&(*xdna).iovad, size));
    free_iova(&mut (*xdna).iovad, iova_pfn(&(*xdna).iovad, dma_addr));
    free_pages(cpu_addr as c_ulong, get_order(size));
}

unsafe fn amdxdna_cleanup_force_iova(dev: *mut drm_device, _res: *mut c_void) {
    let xdna: *mut amdxdna_dev = to_xdna_dev(dev);

    if !(*xdna).domain.is_null() {
        iommu_detach_group((*xdna).domain, (*xdna).group);
        put_iova_domain(&mut (*xdna).iovad);
        iova_cache_put();
        iommu_domain_free((*xdna).domain);
    }

    iommu_group_put((*xdna).group);
}

pub unsafe fn amdxdna_iommu_fini(xdna: *mut amdxdna_dev) {
    if !(*xdna).group.is_null() && (*xdna).domain.is_null() {
        iommu_group_put((*xdna).group);
    }
}

pub unsafe fn amdxdna_iommu_init(xdna: *mut amdxdna_dev) -> c_int {
    let mut order: c_ulong;
    let mut ret: c_int = 0;

    (*xdna).group = iommu_group_get((*xdna).ddev.dev);
    if (*xdna).group.is_null() || !force_iova {
        return 0;
    }

    XDNA_WARN!(xdna, "Enabled force_iova mode.");
    (*xdna).domain = iommu_paging_domain_alloc_flags((*xdna).ddev.dev, IOMMU_HWPT_ALLOC_PASID);
    if IS_ERR((*xdna).domain) {
        XDNA_ERR!(xdna, "Failed to alloc iommu domain");
        ret = PTR_ERR((*xdna).domain);
        goto put_group;
    }

    ret = iova_cache_get();
    if ret != 0 {
        goto free_domain;
    }

    order = __ffs((*(*xdna).domain).pgsize_bitmap);
    init_iova_domain(&mut (*xdna).iovad, 1UL << order, 0);

    ret = iommu_attach_group((*xdna).domain, (*xdna).group);
    if ret != 0 {
        goto put_iova;
    }

    ret = drmm_add_action(&mut (*xdna).ddev, amdxdna_cleanup_force_iova, core::ptr::null_mut());
    if ret != 0 {
        goto detach_group;
    }

    return 0;

    detach_group:
    iommu_detach_group((*xdna).domain, (*xdna).group);
    put_iova:
    put_iova_domain(&mut (*xdna).iovad);
    iova_cache_put();
    free_domain:
    iommu_domain_free((*xdna).domain);
    put_group:
    iommu_group_put((*xdna).group);
    (*xdna).group = core::ptr::null_mut();
    (*xdna).domain = core::ptr::null_mut();

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
