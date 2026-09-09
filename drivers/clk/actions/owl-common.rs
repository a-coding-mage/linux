// SPDX-License-Identifier: GPL-2.0+
//
// OWL common clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct RegmapConfig {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
}

#[repr(C)]
pub struct Regmap;

#[repr(C)]
pub struct Device;

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

#[repr(C)]
pub struct ClkHw {
    pub init: *const ClkInitData,
}

#[repr(C)]
pub struct ClkInitData {
    pub name: *const c_char,
}

#[repr(C)]
pub struct ClkHwOnecellData {
    pub num: u32,
    pub hws: *mut *mut ClkHw,
}

#[repr(C)]
pub struct OwlClkCommon {
    pub regmap: *mut Regmap,
}

#[repr(C)]
pub struct OwlClkDesc {
    pub num_clks: c_int,
    pub clks: *const *mut OwlClkCommon,
    pub regmap: *mut Regmap,
}

extern "C" {
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut Device,
        base: *mut c_void,
        config: *const RegmapConfig,
    ) -> *mut Regmap;
    fn devm_clk_hw_register(dev: *mut Device, hw: *mut ClkHw) -> c_int;
    fn devm_of_clk_add_hw_provider(
        dev: *mut Device,
        get: *const c_void,
        data: *mut ClkHwOnecellData,
    ) -> c_int;
    fn of_clk_hw_onecell_get() -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut Device, fmt: *const c_char, ...);
}

static OWL_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x00cc,
};

unsafe fn owl_clk_set_regmap(desc: *const OwlClkDesc, regmap: *mut Regmap) {
    let mut i = 0;
    let mut clks: *mut OwlClkCommon;

    while i < (*desc).num_clks {
        clks = *(*desc).clks.offset(i as isize);
        if clks.is_null() {
            i += 1;
            continue;
        }

        (*clks).regmap = regmap;
        i += 1;
    }
}

pub unsafe fn owl_clk_regmap_init(
    pdev: *mut PlatformDevice,
    desc: *mut OwlClkDesc,
) -> c_int {
    let base: *mut c_void;
    let regmap: *mut Regmap;

    base = devm_platform_ioremap_resource(pdev, 0);
    if base as isize == -1 {
        return base as c_int;
    }

    regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &OWL_REGMAP_CONFIG);
    if regmap as isize == -1 {
        pr_err(b"failed to init regmap\0".as_ptr() as *const c_char);
        return regmap as c_int;
    }

    owl_clk_set_regmap(desc, regmap);
    (*desc).regmap = regmap;

    0
}

pub unsafe fn owl_clk_probe(
    dev: *mut Device,
    hw_clks: *mut ClkHwOnecellData,
) -> c_int {
    let mut i = 0;
    let mut ret: c_int;
    let mut hw: *mut ClkHw;

    while i < (*hw_clks).num {
        let name: *const c_char;

        hw = *(*hw_clks).hws.offset(i as isize);
        if hw.is_null() || hw as isize == -1 {
            i += 1;
            continue;
        }

        name = (*(*hw).init).name;
        ret = devm_clk_hw_register(dev, hw);
        if ret != 0 {
            dev_err(dev, b"Couldn't register clock %d - %s\n\0".as_ptr() as *const c_char, i, name);
            return ret;
        }
        i += 1;
    }

    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get as *const c_void, hw_clks);
    if ret != 0 {
        dev_err(dev, b"Failed to add clock provider\n\0".as_ptr() as *const c_char);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
