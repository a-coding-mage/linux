/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Christian König
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const GART_ENTRY_WITHOUT_BO_COLOR: u32 = 1;

#[inline]
unsafe fn to_gtt_mgr(man: *mut ttm_resource_manager) -> *mut amdgpu_gtt_mgr {
    container_of!(man, amdgpu_gtt_mgr, manager)
}

/**
 * DOC: mem_info_gtt_total
 *
 * The amdgpu driver provides a sysfs API for reporting current total size of
 * the GTT.
 * The file mem_info_gtt_total is used for this, and returns the total size of
 * the GTT block, in bytes
 */
unsafe fn amdgpu_mem_info_gtt_total_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let ddev = dev_get_drvdata(dev) as *mut drm_device;
    let adev = drm_to_adev(ddev);
    let man = ttm_manager_type(&mut (*(*adev).mman).bdev, TTM_PL_TT);
    sysfs_emit(buf, "%llu\n", (*man).size)
}

/**
 * DOC: mem_info_gtt_used
 *
 * The amdgpu driver provides a sysfs API for reporting current total amount of
 * used GTT.
 * The file mem_info_gtt_used is used for this, and returns the current used
 * size of the GTT block, in bytes
 */
unsafe fn amdgpu_mem_info_gtt_used_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let ddev = dev_get_drvdata(dev) as *mut drm_device;
    let adev = drm_to_adev(ddev);
    let man = &mut (*adev).mman.gtt_mgr.manager as *mut ttm_resource_manager;
    sysfs_emit(buf, "%llu\n", ttm_resource_manager_usage(man))
}

static DEVICE_ATTR_MEM_INFO_GTT_TOTAL: device_attribute = device_attribute!();
static DEVICE_ATTR_MEM_INFO_GTT_USED: device_attribute = device_attribute!();

static mut amdgpu_gtt_mgr_attributes: [*mut attribute; 3] = [
    &DEVICE_ATTR_MEM_INFO_GTT_TOTAL.attr as *const _ as *mut _,
    &DEVICE_ATTR_MEM_INFO_GTT_USED.attr as *const _ as *mut _,
    core::ptr::null_mut(),
];

static mut amdgpu_gtt_mgr_attr_group: attribute_group = attribute_group {
    attrs: unsafe { amdgpu_gtt_mgr_attributes.as_mut_ptr() },
};

/**
 * amdgpu_gtt_mgr_has_gart_addr - Check if mem has address space
 *
 * @res: the mem object to check
 *
 * Check if a mem object has already address space allocated.
 */
pub unsafe fn amdgpu_gtt_mgr_has_gart_addr(res: *mut ttm_resource) -> bool {
    let node = to_ttm_range_mgr_node(res);
    drm_mm_node_allocated(&mut (*node).mm_nodes[0])
}

/**
 * amdgpu_gtt_mgr_mark_bo_teardown - exclude a BO from GART recovery
 *
 * @tbo: TTM BO whose TT backing is about to be destroyed
 *
 * Keep the GART range allocated until the resource is freed, but make recovery
 * treat it like a range without a BO so it isn't touched after TT teardown has
 * started.
 */
pub unsafe fn amdgpu_gtt_mgr_mark_bo_teardown(tbo: *mut ttm_buffer_object) {
    let adev = amdgpu_ttm_adev((*tbo).bdev);
    let node = to_ttm_range_mgr_node((*tbo).resource);
    let mgr = &mut (*adev).mman.gtt_mgr;

    dma_resv_assert_held((*tbo).base.resv);

    spin_lock(&mut (*mgr).lock);
    if drm_mm_node_allocated(&mut (*node).mm_nodes[0]) {
        (*node).mm_nodes[0].color = GART_ENTRY_WITHOUT_BO_COLOR;
    }
    spin_unlock(&mut (*mgr).lock);
}

/** Allocate a new node. Dummy, allocate the node but no space for it yet. */
unsafe fn amdgpu_gtt_mgr_new(
    man: *mut ttm_resource_manager,
    tbo: *mut ttm_buffer_object,
    place: *const ttm_place,
    res: *mut *mut ttm_resource,
) -> c_int {
    let mgr = to_gtt_mgr(man);
    let num_pages: u32 = PFN_UP!((*tbo).base.size);
    let node = kzalloc_flex!(ttm_range_mgr_node, mm_nodes, 1);
    if node.is_null() { return -ENOMEM; }

    ttm_resource_init(tbo, place, &mut (*node).base);
    if ((*place).flags & TTM_PL_FLAG_TEMPORARY) == 0 &&
        ttm_resource_manager_usage(man) > (*man).size {
        let r = -ENOSPC;
        ttm_resource_fini(man, &mut (*node).base);
        kfree(node);
        return r;
    }

    if (*place).lpfn != 0 {
        spin_lock(&mut (*mgr).lock);
        let r = drm_mm_insert_node_in_range(
            &mut (*mgr).mm, &mut (*node).mm_nodes[0], num_pages,
            (*tbo).page_alignment, 0, (*place).fpfn, (*place).lpfn,
            DRM_MM_INSERT_BEST,
        );
        spin_unlock(&mut (*mgr).lock);
        if r != 0 {
            ttm_resource_fini(man, &mut (*node).base);
            kfree(node);
            return r;
        }
        (*node).base.start = (*node).mm_nodes[0].start;
    } else {
        (*node).mm_nodes[0].start = 0;
        (*node).mm_nodes[0].size = PFN_UP!((*node).base.size);
        (*node).base.start = AMDGPU_BO_INVALID_OFFSET;
    }

    *res = &mut (*node).base;
    0
}

/** Free the allocated GTT range again. */
unsafe fn amdgpu_gtt_mgr_del(man: *mut ttm_resource_manager, res: *mut ttm_resource) {
    let node = to_ttm_range_mgr_node(res);
    let mgr = to_gtt_mgr(man);
    spin_lock(&mut (*mgr).lock);
    if drm_mm_node_allocated(&mut (*node).mm_nodes[0]) {
        drm_mm_remove_node(&mut (*node).mm_nodes[0]);
    }
    spin_unlock(&mut (*mgr).lock);
    ttm_resource_fini(man, res);
    kfree(node);
}

/** Helper to dynamically allocate GART entries without a GTT BO. */
pub unsafe fn amdgpu_gtt_mgr_alloc_entries(
    mgr: *mut amdgpu_gtt_mgr,
    mm_node: *mut drm_mm_node,
    mut num_pages: u64,
    mode: drm_mm_insert_mode,
) -> c_int {
    let adev = container_of!(mgr, amdgpu_device, mman.gtt_mgr);
    let mut alignment: u32 = 0;
    if (*adev).family == AMDGPU_FAMILY_SI {
        alignment = 32 * 1024 / AMDGPU_GPU_PAGE_SIZE;
        num_pages = ALIGN!(num_pages, alignment);
    }
    spin_lock(&mut (*mgr).lock);
    let r = drm_mm_insert_node_in_range(
        &mut (*mgr).mm, mm_node, num_pages, alignment,
        GART_ENTRY_WITHOUT_BO_COLOR, 0,
        (*adev).gmc.gart_size >> PAGE_SHIFT, mode,
    );
    spin_unlock(&mut (*mgr).lock);
    r
}

/** Free GART entries not associated with a GTT BO. */
pub unsafe fn amdgpu_gtt_mgr_free_entries(mgr: *mut amdgpu_gtt_mgr, mm_node: *mut drm_mm_node) {
    spin_lock(&mut (*mgr).lock);
    if drm_mm_node_allocated(mm_node) { drm_mm_remove_node(mm_node); }
    spin_unlock(&mut (*mgr).lock);
}

/** Re-init the GART for each known BO in the GTT. */
pub unsafe fn amdgpu_gtt_mgr_recover(mgr: *mut amdgpu_gtt_mgr) {
    let adev = container_of!(mgr, amdgpu_device, mman.gtt_mgr);
    let mut mm_node: *mut drm_mm_node;
    spin_lock(&mut (*mgr).lock);
    drm_mm_for_each_node!(mm_node, &mut (*mgr).mm, {
        if (*mm_node).color == GART_ENTRY_WITHOUT_BO_COLOR { continue; }
        let node = container_of!(mm_node, ttm_range_mgr_node, mm_nodes[0]);
        amdgpu_ttm_recover_gart((*node).base.bo);
    });
    spin_unlock(&mut (*mgr).lock);
}

/** Simplified intersection test, only interesting if we need GART or not. */
unsafe fn amdgpu_gtt_mgr_intersects(
    man: *mut ttm_resource_manager, res: *mut ttm_resource,
    place: *const ttm_place, size: usize,
) -> bool {
    let node = &(*to_ttm_range_mgr_node(res)).mm_nodes[0];
    let num_pages: u32 = PFN_UP!(size);
    if (*place).lpfn == 0 { return true; }
    if !amdgpu_gtt_mgr_has_gart_addr(res) { return false; }
    if (*place).fpfn >= node.start + num_pages ||
        ((*place).lpfn != 0 && (*place).lpfn <= node.start) { return false; }
    true
}

/** Simplified compatibility test. */
unsafe fn amdgpu_gtt_mgr_compatible(
    man: *mut ttm_resource_manager, res: *mut ttm_resource,
    place: *const ttm_place, size: usize,
) -> bool {
    let node = &(*to_ttm_range_mgr_node(res)).mm_nodes[0];
    let num_pages: u32 = PFN_UP!(size);
    if (*place).lpfn == 0 { return true; }
    if !amdgpu_gtt_mgr_has_gart_addr(res) { return false; }
    if node.start < (*place).fpfn ||
        ((*place).lpfn != 0 && node.start + num_pages > (*place).lpfn) { return false; }
    true
}

/** Dump the table content using printk. */
unsafe fn amdgpu_gtt_mgr_debug(man: *mut ttm_resource_manager, printer: *mut drm_printer) {
    let mgr = to_gtt_mgr(man);
    spin_lock(&mut (*mgr).lock);
    drm_mm_print(&mut (*mgr).mm, printer);
    spin_unlock(&mut (*mgr).lock);
}

static amdgpu_gtt_mgr_func: ttm_resource_manager_func = ttm_resource_manager_func {
    alloc: Some(amdgpu_gtt_mgr_new), free: Some(amdgpu_gtt_mgr_del),
    intersects: Some(amdgpu_gtt_mgr_intersects), compatible: Some(amdgpu_gtt_mgr_compatible),
    debug: Some(amdgpu_gtt_mgr_debug),
};

/** Allocate and initialize the GTT manager. */
pub unsafe fn amdgpu_gtt_mgr_init(adev: *mut amdgpu_device, gtt_size: u64) -> c_int {
    let mgr = &mut (*adev).mman.gtt_mgr;
    let man = &mut mgr.manager;
    man.use_tt = true;
    man.func = &amdgpu_gtt_mgr_func;
    ttm_resource_manager_init(man, &mut (*adev).mman.bdev, gtt_size);
    drm_mm_init(&mut mgr.mm, 0, (*adev).gmc.gart_size >> PAGE_SHIFT);
    spin_lock_init(&mut mgr.lock);
    ttm_set_driver_manager(&mut (*adev).mman.bdev, TTM_PL_TT, &mut mgr.manager);
    ttm_resource_manager_set_used(man, true);
    0
}

/** Destroy and free the GTT manager, returning if ranges remain allocated. */
pub unsafe fn amdgpu_gtt_mgr_fini(adev: *mut amdgpu_device) {
    let mgr = &mut (*adev).mman.gtt_mgr;
    let man = &mut mgr.manager;
    ttm_resource_manager_set_used(man, false);
    let ret = ttm_resource_manager_evict_all(&mut (*adev).mman.bdev, man);
    if ret != 0 { return; }
    spin_lock(&mut mgr.lock);
    drm_mm_takedown(&mut mgr.mm);
    spin_unlock(&mut mgr.lock);
    ttm_resource_manager_cleanup(man);
    ttm_set_driver_manager(&mut (*adev).mman.bdev, TTM_PL_TT, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
