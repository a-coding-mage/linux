// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

/* Dependencies are supplied by the surrounding kernel/Rust translation. */

#[repr(C)]
pub struct amdxdna_carveout {
    pub addr: u64,
    pub size: u64,
    pub mm: drm_mm,
    pub lock: mutex,
}

#[repr(C)]
pub struct amdxdna_cbuf_priv {
    pub xdna: *mut amdxdna_dev,
    pub node: drm_mm_node,
}

pub unsafe fn amdxdna_use_carveout(xdna: *mut amdxdna_dev) -> bool {
    !(*xdna).carveout.is_null()
}

pub unsafe fn amdxdna_get_carveout_conf(
    xdna: *mut amdxdna_dev,
    addr: *mut u64,
    size: *mut u64,
) {
    if amdxdna_use_carveout(xdna) {
        *addr = (*(*xdna).carveout).addr;
        *size = (*(*xdna).carveout).size;
    } else {
        *addr = 0;
        *size = 0;
    }
}

pub unsafe fn amdxdna_carveout_init(
    xdna: *mut amdxdna_dev,
    carveout_addr: u64,
    carveout_size: u64,
) -> i32 {
    if amdxdna_use_carveout(xdna) {
        XDNA_ERR(xdna, "Carveout memory has already been set up.");
        return -EBUSY;
    }

    let carveout = kzalloc_obj::<amdxdna_carveout>();
    if carveout.is_null() {
        return -ENOMEM;
    }

    (*carveout).addr = carveout_addr;
    (*carveout).size = carveout_size;
    mutex_init(&mut (*carveout).lock);
    drm_mm_init(&mut (*carveout).mm, (*carveout).addr, (*carveout).size);

    (*xdna).carveout = carveout;
    XDNA_INFO(xdna, "Use carveout mem: 0x%llx@0x%llx\n", (*carveout).size, (*carveout).addr);
    0
}

pub unsafe fn amdxdna_carveout_fini(xdna: *mut amdxdna_dev) {
    let carveout = (*xdna).carveout;
    if !amdxdna_use_carveout(xdna) {
        return;
    }

    XDNA_INFO(xdna, "Cleanup carveout mem: 0x%llx@0x%llx\n", (*carveout).size, (*carveout).addr);
    drm_mm_takedown(&mut (*carveout).mm);
    mutex_destroy(&mut (*carveout).lock);
    kfree(carveout);
    (*xdna).carveout = core::ptr::null_mut();
}

unsafe fn amdxdna_cbuf_map(
    attach: *mut dma_buf_attachment,
    direction: dma_data_direction,
) -> *mut sg_table {
    let cbuf = (*(*attach).dmabuf).priv_ as *mut amdxdna_cbuf_priv;
    let dev = (*attach).dev;
    let sgt = kzalloc_obj::<sg_table>();
    if sgt.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let max_seg = core::cmp::min(u32::MAX as usize, dma_max_mapping_size(dev));
    let n_entries = ((*cbuf).node.size + max_seg - 1) / max_seg;
    let sgl = kzalloc_objs::<scatterlist>(n_entries);
    if sgl.is_null() {
        kfree(sgt);
        return ERR_PTR(-ENOMEM);
    }
    sg_init_table(sgl, n_entries);
    (*sgt).orig_nents = n_entries as u32;
    (*sgt).nents = n_entries as u32;
    (*sgt).sgl = sgl;

    let mut dma_size = (*cbuf).node.size;
    let mut dma_addr = dma_map_resource(dev, (*cbuf).node.start, dma_size, direction, DMA_ATTR_SKIP_CPU_SYNC);
    let ret = dma_mapping_error(dev, dma_addr);
    if ret != 0 {
        pr_err("Failed to dma_map_resource carveout dma buf, ret %d\n", ret);
        kfree(sgl);
        kfree(sgt);
        return ERR_PTR(ret);
    }

    let mut sg = (*sgt).sgl;
    for _i in 0..n_entries {
        let len = core::cmp::min(max_seg, dma_size);
        sg_dma_address(sg) = dma_addr;
        sg_dma_len(sg) = len;
        dma_addr += len;
        dma_size -= len;
        sg = sg_next(sg);
    }
    sgt
}

unsafe fn amdxdna_cbuf_unmap(
    attach: *mut dma_buf_attachment,
    sgt: *mut sg_table,
    direction: dma_data_direction,
) {
    dma_unmap_resource((*attach).dev, sg_dma_address((*sgt).sgl), drm_prime_get_contiguous_size(sgt), direction, DMA_ATTR_SKIP_CPU_SYNC);
    sg_free_table(sgt);
    kfree(sgt);
}

unsafe fn amdxdna_cbuf_release(dbuf: *mut dma_buf) {
    let cbuf = (*dbuf).priv_ as *mut amdxdna_cbuf_priv;
    let carveout = (*(*cbuf).xdna).carveout;
    mutex_lock(&mut (*carveout).lock);
    drm_mm_remove_node(&mut (*cbuf).node);
    mutex_unlock(&mut (*carveout).lock);
    kfree(cbuf);
}

unsafe fn amdxdna_cbuf_vm_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let vma = (*vmf).vma;
    let cbuf = (*vma).vm_private_data as *mut amdxdna_cbuf_priv;
    let pgoff = ((*vmf).address - (*vma).vm_start) >> PAGE_SHIFT;
    let pfn = ((*cbuf).node.start >> PAGE_SHIFT) + pgoff;
    vmf_insert_pfn(vma, (*vmf).address, pfn)
}

unsafe fn amdxdna_cbuf_mmap(dbuf: *mut dma_buf, vma: *mut vm_area_struct) -> i32 {
    let cbuf = (*dbuf).priv_ as *mut amdxdna_cbuf_priv;
    (*vma).vm_ops = &amdxdna_cbuf_vm_ops;
    (*vma).vm_private_data = cbuf as *mut core::ffi::c_void;
    vm_flags_set(vma, VM_PFNMAP | VM_DONTEXPAND | VM_DONTDUMP);
    0
}

unsafe fn amdxdna_cbuf_vmap(dbuf: *mut dma_buf, map: *mut iosys_map) -> i32 {
    let cbuf = (*dbuf).priv_ as *mut amdxdna_cbuf_priv;
    let kva = memremap((*cbuf).node.start, (*cbuf).node.size, MEMREMAP_WB);
    if kva.is_null() {
        pr_err("Failed to vmap carveout dma buf\n");
        return -ENOMEM;
    }
    iosys_map_set_vaddr(map, kva);
    0
}

unsafe fn amdxdna_cbuf_vunmap(_dbuf: *mut dma_buf, map: *mut iosys_map) {
    memunmap((*map).vaddr);
}

static amdxdna_cbuf_vm_ops: vm_operations_struct = vm_operations_struct { fault: Some(amdxdna_cbuf_vm_fault) };
static amdxdna_cbuf_dmabuf_ops: dma_buf_ops = dma_buf_ops {
    map_dma_buf: Some(amdxdna_cbuf_map), unmap_dma_buf: Some(amdxdna_cbuf_unmap),
    release: Some(amdxdna_cbuf_release), mmap: Some(amdxdna_cbuf_mmap),
    vmap: Some(amdxdna_cbuf_vmap), vunmap: Some(amdxdna_cbuf_vunmap),
};

unsafe fn amdxdna_cbuf_clear(dbuf: *mut dma_buf) -> i32 {
    let mut vmap = IOSYS_MAP_INIT_VADDR(core::ptr::null_mut());
    dma_buf_vmap(dbuf, &mut vmap);
    if vmap.vaddr.is_null() { return -EFAULT; }
    memset(vmap.vaddr, 0, (*dbuf).size);
    dma_buf_vunmap(dbuf, &mut vmap);
    0
}

pub unsafe fn amdxdna_get_cbuf(dev: *mut drm_device, size: usize, alignment: u64) -> *mut dma_buf {
    let xdna = to_xdna_dev(dev);
    let cbuf = kzalloc_obj::<amdxdna_cbuf_priv>();
    if cbuf.is_null() { return ERR_PTR(-ENOMEM); }
    (*cbuf).xdna = xdna;
    let carveout = (*xdna).carveout;
    mutex_lock(&mut (*carveout).lock);
    let mut ret = drm_mm_insert_node_generic(&mut (*carveout).mm, &mut (*cbuf).node, size, alignment, 0, DRM_MM_INSERT_BEST);
    mutex_unlock(&mut (*carveout).lock);
    if ret != 0 { kfree(cbuf); return ERR_PTR(ret); }

    let mut exp_info = DEFINE_DMA_BUF_EXPORT_INFO();
    exp_info.size = size;
    exp_info.ops = &amdxdna_cbuf_dmabuf_ops;
    exp_info.priv_ = cbuf as *mut core::ffi::c_void;
    exp_info.flags = O_RDWR;
    let dbuf = dma_buf_export(&mut exp_info);
    if IS_ERR(dbuf) {
        ret = PTR_ERR(dbuf);
        drm_mm_remove_node(&mut (*cbuf).node);
        kfree(cbuf);
        return ERR_PTR(ret);
    }
    ret = amdxdna_cbuf_clear(dbuf);
    if ret != 0 { dma_buf_put(dbuf); return ERR_PTR(ret); }
    dbuf
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
