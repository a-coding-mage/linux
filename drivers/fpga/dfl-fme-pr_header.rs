/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for FPGA Management Engine (FME) Partial Reconfiguration Driver
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

use core::ffi::c_void;

// Types supplied by the platform-device and FPGA subsystems.

/**
 * struct dfl_fme_region - FME fpga region data structure
 *
 * @region: platform device of the FPGA region.
 * @node: used to link fme_region to a list.
 * @port_id: indicate which port this region connected to.
 */
#[repr(C)]
pub struct dfl_fme_region {
    pub region: *mut platform_device,
    pub node: list_head,
    pub port_id: i32,
}

/**
 * struct dfl_fme_region_pdata - platform data for FME region platform device.
 *
 * @mgr: platform device of the FPGA manager.
 * @br: platform device of the FPGA bridge.
 * @region_id: region id (same as port_id).
 */
#[repr(C)]
pub struct dfl_fme_region_pdata {
    pub mgr: *mut platform_device,
    pub br: *mut platform_device,
    pub region_id: i32,
}

/**
 * struct dfl_fme_bridge - FME fpga bridge data structure
 *
 * @br: platform device of the FPGA bridge.
 * @node: used to link fme_bridge to a list.
 */
#[repr(C)]
pub struct dfl_fme_bridge {
    pub br: *mut platform_device,
    pub node: list_head,
}

/**
 * struct dfl_fme_br_pdata - platform data for FME bridge platform device.
 *
 * @cdev: container device.
 * @port_id: port id.
 */
#[repr(C)]
pub struct dfl_fme_br_pdata {
    pub cdev: *mut dfl_fpga_cdev,
    pub port_id: i32,
}

/**
 * struct dfl_fme_mgr_pdata - platform data for FME manager platform device.
 *
 * @ioaddr: mapped io address for FME manager platform device.
 */
#[repr(C)]
pub struct dfl_fme_mgr_pdata {
    pub ioaddr: *mut c_void,
}

pub const DFL_FPGA_FME_MGR: &str = "dfl-fme-mgr";
pub const DFL_FPGA_FME_BRIDGE: &str = "dfl-fme-bridge";
pub const DFL_FPGA_FME_REGION: &str = "dfl-fme-region";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
