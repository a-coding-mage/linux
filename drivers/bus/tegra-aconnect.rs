/*
 * Tegra ACONNECT Bus Driver
 *
 * Copyright (C) 2016, NVIDIA CORPORATION.  All rights reserved.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub struct tegra_aconnect {
    pub ape_clk: *mut clk,
    pub apb2ape_clk: *mut clk,
}

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_clk_get(dev: *mut device, id: *const u8) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8) -> i32;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn of_platform_populate(node: *mut device_node, matches: *const core::ffi::c_void,
                            lookup: *const core::ffi::c_void, parent: *mut device) -> i32;
    fn dev_info(dev: *mut device, fmt: *const u8);
    fn dev_err(dev: *mut device, fmt: *const u8, ...) -> i32;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn pm_runtime_force_suspend(dev: *mut device) -> i32;
    fn pm_runtime_force_resume(dev: *mut device) -> i32;
}

const GFP_KERNEL: u32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

unsafe fn tegra_aconnect_probe(pdev: *mut platform_device) -> i32 {
    let mut aconnect: *mut tegra_aconnect;

    if (*pdev).dev.of_node.is_null() {
        return -EINVAL;
    }

    aconnect = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<tegra_aconnect>(), GFP_KERNEL)
        as *mut tegra_aconnect;
    if aconnect.is_null() {
        return -ENOMEM;
    }

    (*aconnect).ape_clk = devm_clk_get(&mut (*pdev).dev, b"ape\0".as_ptr());
    if (*aconnect).ape_clk as *mut core::ffi::c_void == core::ptr::invalid_mut(0) {
        return dev_err_probe(&mut (*pdev).dev, (*aconnect).ape_clk as isize as i32,
                             b"can't retrieve ape clock\n\0".as_ptr());
    }

    (*aconnect).apb2ape_clk = devm_clk_get(&mut (*pdev).dev, b"apb2ape\0".as_ptr());
    if (*aconnect).apb2ape_clk as *mut core::ffi::c_void == core::ptr::invalid_mut(0) {
        return dev_err_probe(&mut (*pdev).dev, (*aconnect).apb2ape_clk as isize as i32,
                             b"can't retrieve apb2ape clock\n\0".as_ptr());
    }

    dev_set_drvdata(&mut (*pdev).dev, aconnect as *mut core::ffi::c_void);
    pm_runtime_enable(&mut (*pdev).dev);
    of_platform_populate((*pdev).dev.of_node, core::ptr::null(), core::ptr::null(),
                         &mut (*pdev).dev);
    dev_info(&mut (*pdev).dev, b"Tegra ACONNECT bus registered\n\0".as_ptr());
    0
}

unsafe extern "C" fn tegra_aconnect_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn tegra_aconnect_runtime_resume(dev: *mut device) -> i32 {
    let aconnect = dev_get_drvdata(dev) as *mut tegra_aconnect;
    let mut ret = clk_prepare_enable((*aconnect).ape_clk);
    if ret != 0 {
        dev_err(dev, b"ape clk_enable failed: %d\n\0".as_ptr(), ret);
        return ret;
    }
    ret = clk_prepare_enable((*aconnect).apb2ape_clk);
    if ret != 0 {
        clk_disable_unprepare((*aconnect).ape_clk);
        dev_err(dev, b"apb2ape clk_enable failed: %d\n\0".as_ptr(), ret);
        return ret;
    }
    0
}

unsafe extern "C" fn tegra_aconnect_runtime_suspend(dev: *mut device) -> i32 {
    let aconnect = dev_get_drvdata(dev) as *mut tegra_aconnect;
    clk_disable_unprepare((*aconnect).ape_clk);
    clk_disable_unprepare((*aconnect).apb2ape_clk);
    0
}

// SET_RUNTIME_PM_OPS(tegra_aconnect_runtime_suspend, tegra_aconnect_runtime_resume, NULL)
// SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
static tegra_aconnect_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static tegra_aconnect_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"nvidia,tegra210-aconnect\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut tegra_aconnect_driver: platform_driver = platform_driver {
    probe: Some(tegra_aconnect_probe),
    remove: Some(tegra_aconnect_remove),
    driver: platform_driver_driver {
        name: b"tegra-aconnect\0".as_ptr(),
        of_match_table: tegra_aconnect_of_match.as_ptr(),
        pm: &tegra_aconnect_pm_ops,
    },
};

// module_platform_driver(tegra_aconnect_driver);
// MODULE_DEVICE_TABLE(of, tegra_aconnect_of_match);
// MODULE_DESCRIPTION("NVIDIA Tegra ACONNECT Bus Driver");
// MODULE_AUTHOR("Jon Hunter <jonathanh@nvidia.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
