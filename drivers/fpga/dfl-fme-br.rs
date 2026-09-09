// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Bridge Driver for FPGA Management Engine (FME)
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

// Linux kernel dependencies and local headers are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct FmeBrPriv {
    pub pdata: *mut dfl_fme_br_pdata,
    pub port_ops: *mut dfl_fpga_port_ops,
    pub port_fdata: *mut dfl_feature_dev_data,
}

unsafe fn fme_bridge_enable_set(bridge: *mut fpga_bridge, enable: bool) -> i32 {
    let priv_ = (*bridge).priv_ as *mut FmeBrPriv;
    let mut port_fdata: *mut dfl_feature_dev_data;
    let mut ops: *mut dfl_fpga_port_ops;

    if (*priv_).port_fdata.is_null() {
        port_fdata = dfl_fpga_cdev_find_port_data(
            (*(*priv_).pdata).cdev,
            &mut (*(*priv_).pdata).port_id,
            dfl_fpga_check_port_id,
        );
        if port_fdata.is_null() {
            return -ENODEV;
        }

        (*priv_).port_fdata = port_fdata;
    }

    if !(*priv_).port_fdata.is_null() && (*priv_).port_ops.is_null() {
        ops = dfl_fpga_port_ops_get((*priv_).port_fdata);
        if ops.is_null() || (*ops).enable_set.is_none() {
            return -ENOENT;
        }

        (*priv_).port_ops = ops;
    }

    ((*(*priv_).port_ops).enable_set.unwrap())((*priv_).port_fdata, enable)
}

#[repr(C)]
pub struct fpga_bridge_ops {
    pub enable_set: Option<unsafe fn(*mut fpga_bridge, bool) -> i32>,
}

static FME_BRIDGE_OPS: fpga_bridge_ops = fpga_bridge_ops {
    enable_set: Some(fme_bridge_enable_set),
};

unsafe fn fme_br_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let priv_: *mut FmeBrPriv;
    let br: *mut fpga_bridge;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<FmeBrPriv>(), GFP_KERNEL)
        as *mut FmeBrPriv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).pdata = dev_get_platdata(dev);

    br = fpga_bridge_register(dev, c"DFL FPGA FME Bridge".as_ptr(), &FME_BRIDGE_OPS, priv_);
    if IS_ERR(br) {
        return PTR_ERR(br);
    }

    platform_set_drvdata(pdev, br);

    0
}

unsafe fn fme_br_remove(pdev: *mut platform_device) {
    let br = platform_get_drvdata(pdev);
    let priv_ = (*br).priv_ as *mut FmeBrPriv;

    fpga_bridge_unregister(br);

    if !(*priv_).port_ops.is_null() {
        dfl_fpga_port_ops_put((*priv_).port_ops);
    }
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe fn(*mut platform_device)>,
}

static mut FME_BR_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: DFL_FPGA_FME_BRIDGE,
    },
    probe: Some(fme_br_probe),
    remove: Some(fme_br_remove),
};

// Equivalent of module_platform_driver(fme_br_driver).
module_platform_driver!(FME_BR_DRIVER);

// MODULE_DESCRIPTION("FPGA Bridge for DFL FPGA Management Engine");
// MODULE_AUTHOR("Intel Corporation");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:dfl-fme-bridge");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
