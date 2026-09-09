// SPDX-License-Identifier: GPL-2.0
//
// Spreadtrum clock infrastructure
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>
//
// Kernel dependencies supplied by the surrounding translation unit are
// intentionally left external here.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub fast_io: bool,
    pub max_register: usize,
}

#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct resource;
#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}
#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
}
#[repr(C)]
pub struct clk_hw_onecell_data {
    pub num: usize,
    pub hws: *mut *mut clk_hw,
}

#[repr(C)]
pub struct sprd_clk_common {
    pub regmap: *mut regmap,
}
#[repr(C)]
pub struct sprd_clk_desc {
    pub num_clk_clks: usize,
    pub clk_clks: *mut *mut sprd_clk_common,
}

unsafe extern "C" {
    fn of_property_present(node: *mut device_node, name: *const c_char) -> bool;
    fn syscon_regmap_lookup_by_phandle(
        node: *mut device_node,
        name: *const c_char,
    ) -> *mut regmap;
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn of_device_is_compatible(node: *mut device_node, compatible: *const c_char) -> bool;
    fn of_node_put(node: *mut device_node);
    fn device_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_int,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn resource_size(res: *mut resource) -> usize;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        base: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn devm_of_clk_add_hw_provider(
        dev: *mut device,
        get: *const c_void,
        data: *mut clk_hw_onecell_data,
    ) -> c_int;
    fn of_clk_hw_onecell_get;
}

static SPRDCLK_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    fast_io: true,
    max_register: 0,
};

unsafe fn sprd_clk_set_regmap(desc: *const sprd_clk_desc, regmap: *mut regmap) {
    let mut i = 0usize;
    while i < (*desc).num_clk_clks {
        let cclk = *(*desc).clk_clks.add(i);
        if !cclk.is_null() {
            (*cclk).regmap = regmap;
        }
        i += 1;
    }
}

pub unsafe fn sprd_clk_regmap_init(
    pdev: *mut platform_device,
    desc: *const sprd_clk_desc,
) -> c_int {
    let mut base: *mut c_void;
    let dev = &mut (*pdev).dev as *mut device;
    let node = (*dev).of_node;
    let mut np: *mut device_node;
    let regmap: *mut regmap;
    let mut res: *mut resource = core::ptr::null_mut();
    let mut reg_config = SPRDCLK_REGMAP_CONFIG;

    if of_property_present(node, c"sprd,syscon".as_ptr()) {
        regmap = syscon_regmap_lookup_by_phandle(node, c"sprd,syscon".as_ptr());
        if (regmap as isize) < 0 {
            return regmap as c_int;
        }
    } else {
        np = of_get_parent(node);
        if of_device_is_compatible(np, c"syscon".as_ptr()) || {
            of_node_put(np);
            false
        } {
            regmap = device_node_to_regmap(np);
            of_node_put(np);
            if (regmap as isize) < 0 {
                return regmap as c_int;
            }
        } else {
            base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
            if (base as isize) < 0 {
                return base as c_int;
            }

            reg_config.max_register = resource_size(res) - reg_config.reg_stride as usize;
            regmap = devm_regmap_init_mmio(dev, base, &reg_config);
            if (regmap as isize) < 0 {
                return regmap as c_int;
            }
        }
    }

    sprd_clk_set_regmap(desc, regmap);
    0
}

pub unsafe fn sprd_clk_probe(dev: *mut device, clkhw: *mut clk_hw_onecell_data) -> c_int {
    let mut i = 0usize;
    let mut ret: c_int;

    while i < (*clkhw).num {
        let hw = *(*clkhw).hws.add(i);
        if hw.is_null() {
            i += 1;
            continue;
        }

        let _name = (*(*hw).init).name;
        ret = devm_clk_hw_register(dev, hw);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get as *const c_void, clkhw);
    ret
}

// EXPORT_SYMBOL_GPL(sprd_clk_regmap_init);
// EXPORT_SYMBOL_GPL(sprd_clk_probe);
// MODULE_DESCRIPTION("Spreadtrum clock infrastructure");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
