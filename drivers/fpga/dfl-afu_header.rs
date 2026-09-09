/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for FPGA Accelerated Function Unit (AFU) Driver
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *     Wu Hao <hao.wu@intel.com>
 *     Xiao Guangrong <guangrong.xiao@linux.intel.com>
 *     Joseph Grecco <joe.grecco@intel.com>
 *     Enno Luebbers <enno.luebbers@intel.com>
 *     Tim Whisonant <tim.whisonant@intel.com>
 *     Ananda Ravuri <ananda.ravuri@intel.com>
 *     Henry Mitchel <henry.mitchel@intel.com>
 */

// Dependencies supplied by the Linux and DFL headers are intentionally left external.

#[repr(C)]
pub struct dfl_afu_mmio_region {
    pub index: u32,
    pub flags: u32,
    pub size: u64,
    pub offset: u64,
    pub phys: u64,
    pub node: list_head,
}

#[repr(C)]
pub struct dfl_afu_dma_region {
    pub user_addr: u64,
    pub length: u64,
    pub iova: u64,
    pub pages: *mut *mut page,
    pub node: rb_node,
    pub in_use: bool,
}

#[repr(C)]
pub struct dfl_afu {
    pub region_cur_offset: u64,
    pub num_regions: i32,
    pub num_umsgs: u8,
    pub regions: list_head,
    pub dma_regions: rb_root,
}

/* hold fdata->lock when call __afu_port_enable/disable */
extern "C" {
    pub fn __afu_port_enable(fdata: *mut dfl_feature_dev_data) -> i32;
    pub fn __afu_port_disable(fdata: *mut dfl_feature_dev_data) -> i32;

    pub fn afu_mmio_region_init(fdata: *mut dfl_feature_dev_data);
    pub fn afu_mmio_region_add(
        fdata: *mut dfl_feature_dev_data,
        region_index: u32,
        region_size: u64,
        phys: u64,
        flags: u32,
    ) -> i32;
    pub fn afu_mmio_region_destroy(fdata: *mut dfl_feature_dev_data);
    pub fn afu_mmio_region_get_by_index(
        fdata: *mut dfl_feature_dev_data,
        region_index: u32,
        pregion: *mut dfl_afu_mmio_region,
    ) -> i32;
    pub fn afu_mmio_region_get_by_offset(
        fdata: *mut dfl_feature_dev_data,
        offset: u64,
        size: u64,
        pregion: *mut dfl_afu_mmio_region,
    ) -> i32;
    pub fn afu_dma_region_init(fdata: *mut dfl_feature_dev_data);
    pub fn afu_dma_region_destroy(fdata: *mut dfl_feature_dev_data);
    pub fn afu_dma_map_region(
        fdata: *mut dfl_feature_dev_data,
        user_addr: u64,
        length: u64,
        iova: *mut u64,
    ) -> i32;
    pub fn afu_dma_unmap_region(fdata: *mut dfl_feature_dev_data, iova: u64) -> i32;
    pub fn afu_dma_region_find(
        fdata: *mut dfl_feature_dev_data,
        iova: u64,
        size: u64,
    ) -> *mut dfl_afu_dma_region;

    pub static port_err_ops: dfl_feature_ops;
    pub static port_err_id_table: [dfl_feature_id; 0];
    pub static port_err_group: attribute_group;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
