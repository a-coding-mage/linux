// SPDX-License-Identifier: GPL-2.0-only
/*
 * Layerscape FlexSPI clock driver
 *
 * Copyright 2020 Michael Walle <michael@walle.cc>
 */

// Kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct clk_div_table {
    pub val: c_uint,
    pub div: c_uint,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct resource {
    pub start: usize,
}
#[repr(C)]
pub struct clk_hw;

extern "C" {
    fn device_get_match_data(dev: *const device) -> *const clk_div_table;
    fn platform_get_resource(pdev: *mut platform_device, ty: c_uint, index: c_uint) -> *mut resource;
    fn resource_size(res: *const resource) -> usize;
    fn devm_ioremap(dev: *mut device, offset: usize, size: usize) -> *mut c_void;
    fn of_clk_get_parent_name(np: *const device_node, index: c_int) -> *const c_char;
    fn of_property_read_string(
        np: *const device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn devm_clk_hw_register_divider_table(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_uint,
        reg: *mut c_void,
        shift: c_uint,
        width: c_uint,
        clk_flags: c_uint,
        table: *const clk_div_table,
        lock: *mut c_void,
    ) -> *mut clk_hw;
    fn is_err(ptr: *const clk_hw) -> bool;
    fn ptr_err(ptr: *const clk_hw) -> c_int;
    fn devm_of_clk_add_hw_provider(
        dev: *mut device,
        get: unsafe extern "C" fn(*mut device_node, *const c_void, *mut clk_hw) -> *mut clk_hw,
        hw: *mut clk_hw,
    ) -> c_int;
    fn of_clk_hw_simple_get(
        np: *mut device_node,
        args: *const c_void,
        data: *mut clk_hw,
    ) -> *mut clk_hw;
}

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const IORESOURCE_MEM: c_uint = 0x0000_0200;

static LS1028A_FLEXSPI_DIVS: [clk_div_table; 16] = [
    clk_div_table { val: 0, div: 1 },
    clk_div_table { val: 1, div: 2 },
    clk_div_table { val: 2, div: 3 },
    clk_div_table { val: 3, div: 4 },
    clk_div_table { val: 4, div: 5 },
    clk_div_table { val: 5, div: 6 },
    clk_div_table { val: 6, div: 7 },
    clk_div_table { val: 7, div: 8 },
    clk_div_table { val: 11, div: 12 },
    clk_div_table { val: 15, div: 16 },
    clk_div_table { val: 16, div: 20 },
    clk_div_table { val: 17, div: 24 },
    clk_div_table { val: 18, div: 28 },
    clk_div_table { val: 19, div: 32 },
    clk_div_table { val: 20, div: 80 },
    clk_div_table { val: 0, div: 0 },
];

static LX2160A_FLEXSPI_DIVS: [clk_div_table; 12] = [
    clk_div_table { val: 1, div: 2 },
    clk_div_table { val: 3, div: 4 },
    clk_div_table { val: 5, div: 6 },
    clk_div_table { val: 7, div: 8 },
    clk_div_table { val: 11, div: 12 },
    clk_div_table { val: 15, div: 16 },
    clk_div_table { val: 16, div: 20 },
    clk_div_table { val: 17, div: 24 },
    clk_div_table { val: 18, div: 28 },
    clk_div_table { val: 19, div: 32 },
    clk_div_table { val: 20, div: 80 },
    clk_div_table { val: 0, div: 0 },
];

#[no_mangle]
pub unsafe extern "C" fn fsl_flexspi_clk_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let np = (*(dev as *mut device_node)).of_node;
    let mut clk_name = (*np).name;
    let clk_parent: *const c_char;
    let res: *mut resource;
    let reg: *mut c_void;
    let hw: *mut clk_hw;
    let divs: *const clk_div_table;

    divs = device_get_match_data(dev);
    if divs.is_null() {
        return -ENOENT;
    }

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        return -ENOENT;
    }

    /*
     * Can't use devm_ioremap_resource() or devm_of_iomap() because the
     * resource might already be taken by the parent device.
     */
    reg = devm_ioremap(dev, (*res).start, resource_size(res));
    if reg.is_null() {
        return -ENOMEM;
    }

    clk_parent = of_clk_get_parent_name(np, 0);
    if clk_parent.is_null() {
        return -EINVAL;
    }

    of_property_read_string(np, b"clock-output-names\0".as_ptr() as *const c_char, &mut clk_name);

    hw = devm_clk_hw_register_divider_table(dev, clk_name, clk_parent, 0, reg, 0, 5, 0, divs, core::ptr::null_mut());
    if is_err(hw) {
        return ptr_err(hw);
    }

    devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, hw)
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

static FSL_FLEXSPI_CLK_DT_IDS: [of_device_id; 3] = [
    of_device_id { compatible: b"fsl,ls1028a-flexspi-clk\0".as_ptr() as *const c_char, data: &LS1028A_FLEXSPI_DIVS as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,lx2160a-flexspi-clk\0".as_ptr() as *const c_char, data: &LX2160A_FLEXSPI_DIVS as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)]
pub struct platform_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub probe: unsafe extern "C" fn(*mut platform_device) -> c_int,
}

static mut FSL_FLEXSPI_CLK_DRIVER: platform_driver = platform_driver {
    name: b"fsl-flexspi-clk\0".as_ptr() as *const c_char,
    of_match_table: FSL_FLEXSPI_CLK_DT_IDS.as_ptr(),
    probe: fsl_flexspi_clk_probe,
};

// MODULE_DEVICE_TABLE(of, fsl_flexspi_clk_dt_ids);
// module_platform_driver(fsl_flexspi_clk_driver);
// MODULE_DESCRIPTION("FlexSPI clock driver for Layerscape SoCs");
// MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
