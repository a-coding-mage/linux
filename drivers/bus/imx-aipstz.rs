// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2025 NXP
 */

// Translated from the Linux kernel implementation. Kernel headers and symbols
// below are supplied by the surrounding Rust environment.

const IMX_AIPSTZ_MPR0: usize = 0x0;
const IMX_AIPSTZ_OPACR0: usize = 0x40;
const IMX_AIPSTZ_OPACR1: usize = 0x44;
const IMX_AIPSTZ_OPACR2: usize = 0x48;
const IMX_AIPSTZ_OPACR3: usize = 0x4c;
const IMX_AIPSTZ_OPACR4: usize = 0x50;

#[repr(C)]
struct imx_aipstz_config {
    mpr0: u32,
    opacr0: u32,
    opacr1: u32,
    opacr2: u32,
    opacr3: u32,
    opacr4: u32,
}

#[repr(C)]
struct imx_aipstz_data {
    base: *mut core::ffi::c_void,
    default_cfg: *const imx_aipstz_config,
}

unsafe fn imx_aipstz_apply_default(data: *mut imx_aipstz_data) {
    writel((*(*data).default_cfg).mpr0, (*data).base.byte_add(IMX_AIPSTZ_MPR0));
    writel((*(*data).default_cfg).opacr0, (*data).base.byte_add(IMX_AIPSTZ_OPACR0));
    writel((*(*data).default_cfg).opacr1, (*data).base.byte_add(IMX_AIPSTZ_OPACR1));
    writel((*(*data).default_cfg).opacr2, (*data).base.byte_add(IMX_AIPSTZ_OPACR2));
    writel((*(*data).default_cfg).opacr3, (*data).base.byte_add(IMX_AIPSTZ_OPACR3));
    writel((*(*data).default_cfg).opacr4, (*data).base.byte_add(IMX_AIPSTZ_OPACR4));
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
}

static IMX_AIPSTZ_MATCH_TABLE: &[of_device_id] = &[
    of_device_id { compatible: c"simple-bus".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn imx_aipstz_probe(pdev: *mut platform_device) -> i32 {
    let mut data: *mut imx_aipstz_data = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<imx_aipstz_data>(),
        GFP_KERNEL,
    ) as *mut imx_aipstz_data;
    if data.is_null() {
        return dev_err_probe(&mut (*pdev).dev, -ENOMEM, c"failed to allocate data memory\n".as_ptr());
    }

    (*data).base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if is_err((*data).base) {
        return dev_err_probe(&mut (*pdev).dev, -ENOMEM, c"failed to get/ioremap AC memory\n".as_ptr());
    }

    (*data).default_cfg = of_device_get_match_data(&mut (*pdev).dev);
    imx_aipstz_apply_default(data);
    dev_set_drvdata(&mut (*pdev).dev, data as *mut core::ffi::c_void);
    pm_runtime_set_active(&mut (*pdev).dev);
    devm_pm_runtime_enable(&mut (*pdev).dev);
    of_platform_populate((*pdev).dev.of_node, IMX_AIPSTZ_MATCH_TABLE.as_ptr(), core::ptr::null(), &mut (*pdev).dev)
}

unsafe fn imx_aipstz_remove(pdev: *mut platform_device) {
    of_platform_depopulate(&mut (*pdev).dev);
}

unsafe fn imx_aipstz_runtime_resume(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut imx_aipstz_data;
    // restore potentially lost configuration during domain power-off
    imx_aipstz_apply_default(data);
    0
}

#[repr(C)]
struct dev_pm_ops {
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

static IMX_AIPSTZ_PM_OPS: dev_pm_ops = dev_pm_ops {
    runtime_resume: Some(imx_aipstz_runtime_resume),
};

/*
 * following configuration is equivalent to:
 *     masters 0-7 => trusted for R/W + use AHB's HPROT[1] to det. privilege
 */
static IMX8MP_AIPSTZ_DEFAULT_CFG: imx_aipstz_config = imx_aipstz_config {
    mpr0: 0x77777777,
    opacr0: 0,
    opacr1: 0,
    opacr2: 0,
    opacr3: 0,
    opacr4: 0,
};

static IMX_AIPSTZ_OF_IDS: &[of_device_id] = &[
    of_device_id {
        compatible: c"fsl,imx8mp-aipstz".as_ptr(),
        data: &IMX8MP_AIPSTZ_DEFAULT_CFG as *const _ as *const core::ffi::c_void,
    },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

static mut IMX_AIPSTZ_OF_DRIVER: platform_driver = platform_driver {
    probe: Some(imx_aipstz_probe),
    remove: Some(imx_aipstz_remove),
};

extern "C" {
    type platform_device;
    type device;
    static GFP_KERNEL: u32;
    static ENOMEM: i32;
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_err_probe(dev: *mut device, error: i32, fmt: *const core::ffi::c_char) -> i32;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: i32, resource: *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn of_device_get_match_data(dev: *mut device) -> *const imx_aipstz_config;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn pm_runtime_set_active(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device);
    fn of_platform_populate(node: *mut core::ffi::c_void, matches: *const of_device_id, data: *const core::ffi::c_void, parent: *mut device) -> i32;
    fn of_platform_depopulate(dev: *mut device);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
