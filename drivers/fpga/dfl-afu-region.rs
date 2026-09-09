// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Accelerated Function Unit (AFU) MMIO Region Management
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Wu Hao <hao.wu@linux.intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 */

// Types, constants, allocation helpers, locking primitives, and list helpers
// are supplied by dfl-afu.h and the surrounding kernel environment.

pub unsafe fn afu_mmio_region_init(fdata: *mut dfl_feature_dev_data) {
    let afu = dfl_fpga_fdata_get_private(fdata);
    INIT_LIST_HEAD(&mut (*afu).regions);
}

unsafe fn get_region_by_index(
    afu: *mut dfl_afu,
    region_index: u32,
) -> *mut dfl_afu_mmio_region {
    let mut region: *mut dfl_afu_mmio_region = core::ptr::null_mut();
    // Equivalent to for_each_region(region, afu).
    list_for_each_entry(region, &mut (*afu).regions, node) {
        if (*region).index == region_index {
            return region;
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn afu_mmio_region_add(
    fdata: *mut dfl_feature_dev_data,
    region_index: u32,
    mut region_size: u64,
    phys: u64,
    flags: u32,
) -> i32 {
    let dev = &mut (*(*fdata).dev).dev;
    let region = devm_kzalloc(dev, core::mem::size_of::<dfl_afu_mmio_region>(), GFP_KERNEL)
        as *mut dfl_afu_mmio_region;
    if region.is_null() {
        return -ENOMEM;
    }

    (*region).index = region_index;
    (*region).size = region_size;
    (*region).phys = phys;
    (*region).flags = flags;

    mutex_lock(&mut (*fdata).lock);
    let afu = dfl_fpga_fdata_get_private(fdata);

    if !get_region_by_index(afu, region_index).is_null() {
        mutex_unlock(&mut (*fdata).lock);
        devm_kfree(dev, region as *mut core::ffi::c_void);
        return -EEXIST;
    }

    region_size = PAGE_ALIGN(region_size);
    (*region).offset = (*afu).region_cur_offset;
    list_add(&mut (*region).node, &mut (*afu).regions);
    (*afu).region_cur_offset = (*afu).region_cur_offset.wrapping_add(region_size);
    (*afu).num_regions = (*afu).num_regions.wrapping_add(1);
    mutex_unlock(&mut (*fdata).lock);
    0
}

pub unsafe fn afu_mmio_region_destroy(fdata: *mut dfl_feature_dev_data) {
    let afu = dfl_fpga_fdata_get_private(fdata);
    let mut tmp: *mut dfl_afu_mmio_region = core::ptr::null_mut();
    let mut region: *mut dfl_afu_mmio_region = core::ptr::null_mut();
    list_for_each_entry_safe(region, tmp, &mut (*afu).regions, node) {
        devm_kfree(
            &mut (*(*fdata).dev).dev,
            region as *mut core::ffi::c_void,
        );
    }
}

pub unsafe fn afu_mmio_region_get_by_index(
    fdata: *mut dfl_feature_dev_data,
    region_index: u32,
    pregion: *mut dfl_afu_mmio_region,
) -> i32 {
    mutex_lock(&mut (*fdata).lock);
    let afu = dfl_fpga_fdata_get_private(fdata);
    let region = get_region_by_index(afu, region_index);
    if region.is_null() {
        mutex_unlock(&mut (*fdata).lock);
        return -EINVAL;
    }
    *pregion = *region;
    mutex_unlock(&mut (*fdata).lock);
    0
}

pub unsafe fn afu_mmio_region_get_by_offset(
    fdata: *mut dfl_feature_dev_data,
    offset: u64,
    size: u64,
    pregion: *mut dfl_afu_mmio_region,
) -> i32 {
    mutex_lock(&mut (*fdata).lock);
    let afu = dfl_fpga_fdata_get_private(fdata);
    let mut region: *mut dfl_afu_mmio_region = core::ptr::null_mut();
    list_for_each_entry(region, &mut (*afu).regions, node) {
        if (*region).offset <= offset
            && (*region).offset.wrapping_add((*region).size)
                >= offset.wrapping_add(size)
        {
            *pregion = *region;
            mutex_unlock(&mut (*fdata).lock);
            return 0;
        }
    }
    mutex_unlock(&mut (*fdata).lock);
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
