// SPDX-License-Identifier: GPL-2.0
/*
 * Simple Power-Managed Bus Driver
 *
 * Copyright (C) 2014-2015 Glider bvba
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
struct simple_pm_bus {
    clks: *mut clk_bulk_data,
    num_clks: i32,
}

unsafe fn simple_pm_bus_probe(pdev: *mut platform_device) -> i32 {
    let dev: *const device = unsafe { &(*pdev).dev };
    let lookup: *const of_dev_auxdata = unsafe { dev_get_platdata(dev) };
    let np: *mut device_node = unsafe { (*dev).of_node };
    let mut bus: *mut simple_pm_bus;

    /*
     * Allow user to use driver_override to bind this driver to a
     * transparent bus device which has a different compatible string
     * that's not listed in simple_pm_bus_of_match. We don't want to do any
     * of the simple-pm-bus tasks for these devices, so return early.
     */
    if unsafe { device_has_driver_override(&(*pdev).dev) } {
        return 0;
    }

    let match_: *const of_device_id = unsafe {
        of_match_device((*(*dev).driver).of_match_table, dev)
    };
    /*
     * These are transparent bus devices (not simple-pm-bus matches) that
     * have their child nodes populated automatically.  So, don't need to
     * do anything more. We only match with the device if this driver is
     * the most specific match because we don't want to incorrectly bind to
     * a device that has a more specific driver.
     */
    if !match_.is_null() && unsafe { !(*match_).data.is_null() } {
        if unsafe { of_property_match_string(np, c"compatible".as_ptr(), (*match_).compatible) } == 0 {
            return 0;
        } else {
            return -19; // -ENODEV
        }
    }

    bus = unsafe { devm_kzalloc(&(*pdev).dev, core::mem::size_of::<simple_pm_bus>(), GFP_KERNEL) as *mut simple_pm_bus };
    if bus.is_null() {
        return -12; // -ENOMEM
    }

    unsafe {
        (*bus).num_clks = devm_clk_bulk_get_all(&(*pdev).dev, &mut (*bus).clks);
    }
    if unsafe { (*bus).num_clks } < 0 {
        return unsafe { dev_err_probe(&(*pdev).dev, (*bus).num_clks, c"failed to get clocks\n".as_ptr()) };
    }

    unsafe { dev_set_drvdata(&(*pdev).dev, bus as *mut core::ffi::c_void) };
    unsafe { dev_dbg(&(*pdev).dev, c"%s\n".as_ptr(), c"simple_pm_bus_probe\0".as_ptr()) };
    unsafe { pm_runtime_enable(&(*pdev).dev) };

    if !np.is_null() {
        unsafe { of_platform_populate(np, core::ptr::null(), lookup, &(*pdev).dev) };
    }
    0
}

unsafe fn simple_pm_bus_remove(pdev: *mut platform_device) {
    let data: *const core::ffi::c_void = unsafe { of_device_get_match_data(&(*pdev).dev) };
    if unsafe { device_has_driver_override(&(*pdev).dev) } || !data.is_null() {
        return;
    }
    unsafe { dev_dbg(&(*pdev).dev, c"%s\n".as_ptr(), c"simple_pm_bus_remove\0".as_ptr()) };
    unsafe { pm_runtime_disable(&(*pdev).dev) };
}

unsafe fn simple_pm_bus_runtime_suspend(dev: *mut device) -> i32 {
    let bus = unsafe { dev_get_drvdata(dev) as *mut simple_pm_bus };
    unsafe { clk_bulk_disable_unprepare((*bus).num_clks, (*bus).clks) };
    0
}

unsafe fn simple_pm_bus_runtime_resume(dev: *mut device) -> i32 {
    let bus = unsafe { dev_get_drvdata(dev) as *mut simple_pm_bus };
    let ret = unsafe { clk_bulk_prepare_enable((*bus).num_clks, (*bus).clks) };
    if ret != 0 {
        unsafe { dev_err(dev, c"failed to enable clocks: %d\n".as_ptr(), ret) };
        return ret;
    }
    0
}

unsafe fn simple_pm_bus_suspend(dev: *mut device) -> i32 {
    let bus = unsafe { dev_get_drvdata(dev) };
    if bus.is_null() { return 0; }
    unsafe { pm_runtime_force_suspend(dev) }
}

unsafe fn simple_pm_bus_resume(dev: *mut device) -> i32 {
    let bus = unsafe { dev_get_drvdata(dev) };
    if bus.is_null() { return 0; }
    unsafe { pm_runtime_force_resume(dev) }
}

static simple_pm_bus_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(simple_pm_bus_runtime_suspend),
    runtime_resume: Some(simple_pm_bus_runtime_resume),
    runtime_idle: None,
    suspend: Some(simple_pm_bus_suspend),
    resume: Some(simple_pm_bus_resume),
};

// Match if the device is only a bus.
const ONLY_BUS: *mut core::ffi::c_void = 1 as *mut core::ffi::c_void;

static simple_pm_bus_of_match: [of_device_id; 12] = [
    of_device_id { compatible: c"simple-pm-bus".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: c"simple-bus".as_ptr(), data: ONLY_BUS },
    of_device_id { compatible: c"simple-mfd".as_ptr(), data: ONLY_BUS },
    of_device_id { compatible: c"isa".as_ptr(), data: ONLY_BUS },
    of_device_id { compatible: c"arm,amba-bus".as_ptr(), data: ONLY_BUS },
    of_device_id { compatible: c"fsl,ls1021a-scfg".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: c"fsl,ls1043a-scfg".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: c"fsl,ls1046a-scfg".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: c"fsl,ls1088a-isc".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: c"fsl,ls2080a-isc".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: c"fsl,lx2160a-isc".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut simple_pm_bus_driver: platform_driver = platform_driver {
    probe: Some(simple_pm_bus_probe),
    remove: Some(simple_pm_bus_remove),
    driver: device_driver {
        name: c"simple-pm-bus".as_ptr(),
        of_match_table: simple_pm_bus_of_match.as_ptr(),
        pm: Some(&simple_pm_bus_pm_ops),
    },
};

// module_platform_driver(simple_pm_bus_driver);
// MODULE_DEVICE_TABLE(of, simple_pm_bus_of_match);
// MODULE_DESCRIPTION("Simple Power-Managed Bus Driver");
// MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
