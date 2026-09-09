// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2012 ARM Limited
 */

// Dependencies are supplied by the surrounding kernel Rust environment.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct regmap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: c_uint,
    pub num_parents: c_uint,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct vexpress_osc {
    reg: *mut regmap,
    hw: clk_hw,
    rate_min: c_ulong,
    rate_max: c_ulong,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_ulong) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_vexpress_config(dev: *mut device) -> *mut regmap;
    fn of_property_read_u32_array(node: *mut device_node, name: *const c_char,
                                  out: *mut u32, count: usize) -> c_int;
    fn of_property_read_string(node: *mut device_node, name: *const c_char,
                               out: *mut *const c_char) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const c_void,
                                   data: *mut clk_hw) -> c_int;
    fn clk_hw_set_rate_range(hw: *mut clk_hw, min: c_ulong, max: c_ulong);
    fn of_clk_hw_simple_get() -> c_void;
}

unsafe fn to_vexpress_osc(hw: *mut clk_hw) -> *mut vexpress_osc {
    (hw as *mut u8).sub(core::mem::offset_of!(vexpress_osc, hw)) as *mut vexpress_osc
}

unsafe extern "C" fn vexpress_osc_recalc_rate(hw: *mut clk_hw,
                                                _parent_rate: c_ulong) -> c_ulong {
    let osc = &mut *to_vexpress_osc(hw);
    let mut rate: u32 = 0;
    regmap_read(osc.reg, 0, &mut rate);
    rate as c_ulong
}

unsafe extern "C" fn vexpress_osc_determine_rate(hw: *mut clk_hw,
                                                  req: *mut clk_rate_request) -> c_int {
    let osc = &*to_vexpress_osc(hw);
    let req = &mut *req;
    if osc.rate_min != 0 && req.rate < osc.rate_min {
        req.rate = osc.rate_min;
    }
    if osc.rate_max != 0 && req.rate > osc.rate_max {
        req.rate = osc.rate_max;
    }
    0
}

unsafe extern "C" fn vexpress_osc_set_rate(hw: *mut clk_hw, rate: c_ulong,
                                             _parent_rate: c_ulong) -> c_int {
    let osc = &*to_vexpress_osc(hw);
    regmap_write(osc.reg, 0, rate)
}

static VEXPRESS_OSC_OPS: clk_ops = clk_ops {
    recalc_rate: Some(vexpress_osc_recalc_rate),
    determine_rate: Some(vexpress_osc_determine_rate),
    set_rate: Some(vexpress_osc_set_rate),
};

unsafe extern "C" fn vexpress_osc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let osc = devm_kzalloc(dev, core::mem::size_of::<vexpress_osc>(), 0) as *mut vexpress_osc;
    if osc.is_null() {
        return -12;
    }

    (*osc).reg = devm_regmap_init_vexpress_config(dev);
    if (*osc).reg.is_null() {
        return -1;
    }

    let mut range = [0u32; 2];
    if of_property_read_u32_array((*dev).of_node, b"freq-range\0".as_ptr() as *const c_char,
                                  range.as_mut_ptr(), range.len()) == 0 {
        (*osc).rate_min = range[0] as c_ulong;
        (*osc).rate_max = range[1] as c_ulong;
    }

    let mut init = clk_init_data {
        name: core::ptr::null(),
        ops: &VEXPRESS_OSC_OPS,
        flags: 0,
        num_parents: 0,
    };
    if of_property_read_string((*dev).of_node,
                                b"clock-output-names\0".as_ptr() as *const c_char,
                                &mut init.name) != 0 {
        init.name = dev_name(dev);
    }

    (*osc).hw.init = &init;
    let ret = devm_clk_hw_register(dev, &mut (*osc).hw);
    if ret < 0 {
        return ret;
    }

    devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get as *const c_void, &mut (*osc).hw);
    clk_hw_set_rate_range(&mut (*osc).hw, (*osc).rate_min, (*osc).rate_max);
    0
}

static VEXPRESS_OSC_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"arm,vexpress-osc\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

#[no_mangle]
pub static mut VEXPRESS_OSC_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"vexpress-osc\0".as_ptr() as *const c_char,
        of_match_table: VEXPRESS_OSC_OF_MATCH.as_ptr(),
    },
    probe: Some(vexpress_osc_probe),
};

// MODULE_DEVICE_TABLE(of, vexpress_osc_of_match);
// module_platform_driver(vexpress_osc_driver);
// MODULE_DESCRIPTION("Clock driver for Versatile Express OSC clock generators");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
