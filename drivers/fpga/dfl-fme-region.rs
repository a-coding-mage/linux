// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Region Driver for FPGA Management Engine (FME)
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Wu Hao <hao.wu@intel.com>
 *   Joseph Grecco <joe.grecco@intel.com>
 *   Enno Luebbers <enno.luebbers@intel.com>
 *   Tim Whisonant <tim.whisonant@intel.com>
 *   Ananda Ravuri <ananda.ravuri@intel.com>
 *   Henry Mitchel <henry.mitchel@intel.com>
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_int;

extern "C" {
    fn fpga_bridge_get_to_list(
        dev: *mut device,
        info: *mut fpga_region_info,
        bridge_list: *mut list_head,
    ) -> c_int;
    fn fpga_mgr_get(dev: *mut device) -> *mut fpga_manager;
    fn fpga_mgr_put(mgr: *mut fpga_manager);
    fn fpga_region_register_full(
        dev: *mut device,
        info: *mut fpga_region_info,
    ) -> *mut fpga_region;
    fn fpga_region_unregister(region: *mut fpga_region);
    fn dev_get_platdata(dev: *mut device) -> *mut dfl_fme_region_pdata;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut fpga_region);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut fpga_region;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct fpga_manager {
    pub compat_id: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct fpga_region {
    pub priv_: *mut core::ffi::c_void,
    pub info: *mut fpga_region_info,
    pub bridge_list: list_head,
    pub mgr: *mut fpga_manager,
}

#[repr(C)]
pub struct fpga_region_info {
    pub mgr: *mut fpga_manager,
    pub compat_id: *mut core::ffi::c_void,
    pub get_bridges: Option<unsafe extern "C" fn(*mut fpga_region) -> c_int>,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct dfl_fme_region_pdata {
    pub br: *mut dfl_fme_bridge,
    pub mgr: *mut dfl_fme_manager,
}

#[repr(C)]
pub struct dfl_fme_bridge {
    pub dev: device,
}

#[repr(C)]
pub struct dfl_fme_manager {
    pub dev: device,
}

unsafe extern "C" fn fme_region_get_bridges(region: *mut fpga_region) -> c_int {
    let pdata = (*region).priv_ as *mut dfl_fme_region_pdata;
    let dev = &mut (*(*pdata).br).dev as *mut device;

    fpga_bridge_get_to_list(dev, (*region).info, &mut (*region).bridge_list)
}

unsafe extern "C" fn fme_region_probe(pdev: *mut platform_device) -> c_int {
    let pdata = dev_get_platdata(&mut (*pdev).dev);
    let mut info: fpga_region_info = core::mem::zeroed();
    let dev = &mut (*pdev).dev as *mut device;
    let mut region: *mut fpga_region;
    let mgr: *mut fpga_manager;
    let ret: c_int;

    mgr = fpga_mgr_get(&mut (*(*pdata).mgr).dev);
    if mgr.is_null() {
        return -517; // -EPROBE_DEFER
    }

    info.mgr = mgr;
    info.compat_id = (*mgr).compat_id;
    info.get_bridges = Some(fme_region_get_bridges);
    info.priv_ = pdata as *mut core::ffi::c_void;
    region = fpga_region_register_full(dev, &mut info);
    if region.is_null() {
        ret = -1; // PTR_ERR(region), supplied by the kernel error-pointer ABI
        fpga_mgr_put(mgr);
        return ret;
    }

    platform_set_drvdata(pdev, region);

    // dev_dbg(dev, "DFL FME FPGA Region probed\n");

    0
}

unsafe extern "C" fn fme_region_remove(pdev: *mut platform_device) {
    let region = platform_get_drvdata(pdev);
    let mgr = (*region).mgr;

    fpga_region_unregister(region);
    fpga_mgr_put(mgr);
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct driver {
    pub name: *const u8,
}

// DFL_FPGA_FME_REGION and module_platform_driver are kernel build-time definitions.
static mut fme_region_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"DFL_FPGA_FME_REGION\0".as_ptr(),
    },
    probe: Some(fme_region_probe),
    remove: Some(fme_region_remove),
};

// module_platform_driver(fme_region_driver);
// MODULE_DESCRIPTION("FPGA Region for DFL FPGA Management Engine");
// MODULE_AUTHOR("Intel Corporation");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:dfl-fme-region");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
