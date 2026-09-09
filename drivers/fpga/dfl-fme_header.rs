/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for FPGA Management Engine (FME) Driver
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Kang Luwei <luwei.kang@intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 *   Wu Hao <hao.wu@intel.com>
 *   Joseph Grecco <joe.grecco@intel.com>
 *   Enno Luebbers <enno.luebbers@intel.com>
 *   Tim Whisonant <tim.whisonant@intel.com>
 *   Ananda Ravuri <ananda.ravuri@intel.com>
 *   Henry Mitchel <henry.mitchel@intel.com>
 */

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dfl_feature_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dfl_feature_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

/**
 * struct dfl_fme - dfl fme private data
 *
 * @mgr: FME's FPGA manager platform device.
 * @region_list: linked list of FME's FPGA regions.
 * @bridge_list: linked list of FME's FPGA bridges.
 */
#[repr(C)]
pub struct dfl_fme {
    pub mgr: *mut platform_device,
    pub region_list: list_head,
    pub bridge_list: list_head,
}

extern "C" {
    pub static fme_pr_mgmt_ops: dfl_feature_ops;
    pub static fme_pr_mgmt_id_table: [dfl_feature_id; 0];
    pub static fme_global_err_ops: dfl_feature_ops;
    pub static fme_global_err_id_table: [dfl_feature_id; 0];
    pub static fme_global_err_group: attribute_group;
    pub static fme_perf_ops: dfl_feature_ops;
    pub static fme_perf_id_table: [dfl_feature_id; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
