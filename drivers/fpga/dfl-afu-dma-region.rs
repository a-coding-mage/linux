// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Accelerated Function Unit (AFU) DMA Region Management
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Wu Hao <hao.wu@intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 */

// Dependencies supplied by the Linux kernel and dfl-afu.h are intentionally
// left as external symbols.

pub unsafe fn afu_dma_region_init(fdata: *mut dfl_feature_dev_data) {
    let afu = dfl_fpga_fdata_get_private(fdata);
    (*afu).dma_regions = RB_ROOT;
}

/// Pin pages of the given DMA memory region.
unsafe fn afu_dma_pin_pages(
    fdata: *mut dfl_feature_dev_data,
    region: *mut dfl_afu_dma_region,
) -> i32 {
    let npages = ((*region).length >> PAGE_SHIFT) as i32;
    let dev = &(*(*fdata).dev).dev;
    let mut ret = account_locked_vm((*current).mm, npages as u64, true);
    if ret != 0 { return ret; }

    (*region).pages = kzalloc_objs::<*mut page>(npages);
    if (*region).pages.is_null() {
        ret = -ENOMEM;
        account_locked_vm((*current).mm, npages as u64, false);
        return ret;
    }

    let pinned = pin_user_pages_fast((*region).user_addr, npages, FOLL_WRITE, (*region).pages);
    if pinned < 0 {
        ret = pinned;
        kfree((*region).pages);
        account_locked_vm((*current).mm, npages as u64, false);
        return ret;
    } else if pinned != npages {
        ret = -EFAULT;
        unpin_user_pages((*region).pages, pinned);
        kfree((*region).pages);
        account_locked_vm((*current).mm, npages as u64, false);
        return ret;
    }

    dev_dbg(dev, "%d pages pinned\n", pinned);
    0
}

/// Unpin pages of the given DMA memory region.
unsafe fn afu_dma_unpin_pages(
    fdata: *mut dfl_feature_dev_data,
    region: *mut dfl_afu_dma_region,
) {
    let npages = ((*region).length >> PAGE_SHIFT) as i64;
    let dev = &(*(*fdata).dev).dev;
    unpin_user_pages((*region).pages, npages as i32);
    kfree((*region).pages);
    account_locked_vm((*current).mm, npages as u64, false);
    dev_dbg(dev, "%ld pages unpinned\n", npages);
}

/// Check whether the pages in a DMA memory region are continuous.
unsafe fn afu_dma_check_continuous_pages(region: *mut dfl_afu_dma_region) -> bool {
    let npages = ((*region).length >> PAGE_SHIFT) as i32;
    for i in 0..(npages - 1) {
        if page_to_pfn(*(*region).pages.add(i as usize)) + 1
            != page_to_pfn(*(*region).pages.add((i + 1) as usize)) { return false; }
    }
    true
}

unsafe fn dma_region_check_iova(region: *mut dfl_afu_dma_region, iova: u64, size: u64) -> bool {
    if size == 0 && (*region).iova != iova { return false; }
    (*region).iova <= iova && (*region).length + (*region).iova >= iova + size
}

/// Add a DMA region to the red-black tree.
unsafe fn afu_dma_region_add(
    fdata: *mut dfl_feature_dev_data,
    region: *mut dfl_afu_dma_region,
) -> i32 {
    let afu = dfl_fpga_fdata_get_private(fdata);
    let mut new = &mut (*afu).dma_regions.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    dev_dbg(&(*(*fdata).dev).dev, "add region (iova = %llx)\n", (*region).iova);
    while !(*new).is_null() {
        let this = container_of!(*new, dfl_afu_dma_region, node);
        parent = *new;
        if dma_region_check_iova(this, (*region).iova, (*region).length) { return -EEXIST; }
        if (*region).iova < (*this).iova { new = &mut (**new).rb_left; }
        else if (*region).iova > (*this).iova { new = &mut (**new).rb_right; }
        else { return -EEXIST; }
    }
    rb_link_node(&mut (*region).node, parent, new);
    rb_insert_color(&mut (*region).node, &mut (*afu).dma_regions);
    0
}

unsafe fn afu_dma_region_remove(fdata: *mut dfl_feature_dev_data, region: *mut dfl_afu_dma_region) {
    dev_dbg(&(*(*fdata).dev).dev, "del region (iova = %llx)\n", (*region).iova);
    let afu = dfl_fpga_fdata_get_private(fdata);
    rb_erase(&mut (*region).node, &mut (*afu).dma_regions);
}

/// Destroy all regions in the red-black tree.
pub unsafe fn afu_dma_region_destroy(fdata: *mut dfl_feature_dev_data) {
    let afu = dfl_fpga_fdata_get_private(fdata);
    let mut node = rb_first(&mut (*afu).dma_regions);
    while !node.is_null() {
        let region = container_of!(node, dfl_afu_dma_region, node);
        dev_dbg(&(*(*fdata).dev).dev, "del region (iova = %llx)\n", (*region).iova);
        rb_erase(node, &mut (*afu).dma_regions);
        if (*region).iova != 0 { dma_unmap_page(dfl_fpga_fdata_to_parent(fdata), (*region).iova, (*region).length, DMA_BIDIRECTIONAL); }
        if !(*region).pages.is_null() { afu_dma_unpin_pages(fdata, region); }
        node = rb_next(node);
        kfree(region);
    }
}

pub unsafe fn afu_dma_region_find(fdata: *mut dfl_feature_dev_data, iova: u64, size: u64) -> *mut dfl_afu_dma_region {
    let afu = dfl_fpga_fdata_get_private(fdata);
    let mut node = (*afu).dma_regions.rb_node;
    while !node.is_null() {
        let region = container_of!(node, dfl_afu_dma_region, node);
        if dma_region_check_iova(region, iova, size) { return region; }
        if iova < (*region).iova { node = (*node).rb_left; }
        else if iova > (*region).iova { node = (*node).rb_right; }
        else { break; }
    }
    core::ptr::null_mut()
}

unsafe fn afu_dma_region_find_iova(fdata: *mut dfl_feature_dev_data, iova: u64) -> *mut dfl_afu_dma_region {
    afu_dma_region_find(fdata, iova, 0)
}

pub unsafe fn afu_dma_map_region(fdata: *mut dfl_feature_dev_data, user_addr: u64, length: u64, iova: *mut u64) -> i32 {
    let dev = &(*(*fdata).dev).dev;
    if !PAGE_ALIGNED(user_addr) || !PAGE_ALIGNED(length) || length == 0 || user_addr.wrapping_add(length) < user_addr { return -EINVAL; }
    let region = kzalloc::<dfl_afu_dma_region>();
    if region.is_null() { return -ENOMEM; }
    (*region).user_addr = user_addr; (*region).length = length;
    let mut ret = afu_dma_pin_pages(fdata, region);
    if ret != 0 { dev_err(dev, "failed to pin memory region\n"); kfree(region); return ret; }
    if !afu_dma_check_continuous_pages(region) { dev_err(dev, "pages are not continuous\n"); afu_dma_unpin_pages(fdata, region); kfree(region); return -EINVAL; }
    (*region).iova = dma_map_page(dfl_fpga_fdata_to_parent(fdata), *(*region).pages, 0, (*region).length, DMA_BIDIRECTIONAL);
    if dma_mapping_error(dfl_fpga_fdata_to_parent(fdata), (*region).iova) { dev_err(dev, "failed to map for dma\n"); afu_dma_unpin_pages(fdata, region); kfree(region); return -EFAULT; }
    *iova = (*region).iova;
    mutex_lock(&mut (*fdata).lock); ret = afu_dma_region_add(fdata, region); mutex_unlock(&mut (*fdata).lock);
    if ret != 0 { dev_err(dev, "failed to add dma region\n"); dma_unmap_page(dfl_fpga_fdata_to_parent(fdata), (*region).iova, (*region).length, DMA_BIDIRECTIONAL); afu_dma_unpin_pages(fdata, region); kfree(region); }
    ret
}

pub unsafe fn afu_dma_unmap_region(fdata: *mut dfl_feature_dev_data, iova: u64) -> i32 {
    mutex_lock(&mut (*fdata).lock);
    let region = afu_dma_region_find_iova(fdata, iova);
    if region.is_null() { mutex_unlock(&mut (*fdata).lock); return -EINVAL; }
    if (*region).in_use { mutex_unlock(&mut (*fdata).lock); return -EBUSY; }
    afu_dma_region_remove(fdata, region); mutex_unlock(&mut (*fdata).lock);
    dma_unmap_page(dfl_fpga_fdata_to_parent(fdata), (*region).iova, (*region).length, DMA_BIDIRECTIONAL);
    afu_dma_unpin_pages(fdata, region); kfree(region); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
