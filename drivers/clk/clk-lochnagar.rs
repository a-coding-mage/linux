// SPDX-License-Identifier: GPL-2.0
/*
 * Lochnagar clock control
 *
 * Copyright (c) 2017-2018 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 *
 * Author: Charles Keepax <ckeepax@opensource.cirrus.com>
 */

// C dependencies supplied by the kernel and Lochnagar headers are external
// dependencies of this translation.

const LOCHNAGAR_NUM_CLOCKS: usize = LOCHNAGAR_SPDIF_CLKOUT + 1;

#[repr(C)]
struct lochnagar_clk {
    name: *const core::ffi::c_char,
    hw: clk_hw,
    priv_: *mut lochnagar_clk_priv,
    cfg_reg: u16,
    ena_mask: u16,
    src_reg: u16,
    src_mask: u16,
}

#[repr(C)]
struct lochnagar_clk_priv {
    dev: *mut device,
    regmap: *mut regmap,
    lclks: [lochnagar_clk; LOCHNAGAR_NUM_CLOCKS],
}

#[repr(C)]
struct lochnagar_config {
    parents: *const clk_parent_data,
    nparents: i32,
    clks: *const lochnagar_clk,
}

// External kernel types, constants, functions, and registration facilities.
#[repr(C)] struct clk_hw { init: *const clk_init_data }
#[repr(C)] struct clk_parent_data { name: *const core::ffi::c_char, fw_name: *const core::ffi::c_char }
#[repr(C)] struct clk_init_data { ops: *const clk_ops, name: *const core::ffi::c_char, parent_data: *const clk_parent_data, num_parents: i32 }
#[repr(C)] struct clk_ops { prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>, unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>, determine_rate: Option<unsafe extern "C" fn()>, set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>, get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8> }
#[repr(C)] struct device { parent: *mut device }
#[repr(C)] struct regmap;
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct of_phandle_args { args: [u32; 1] }
#[repr(C)] struct of_device_id { compatible: *const core::ffi::c_char, data: *const core::ffi::c_void }

extern "C" {
    fn regmap_update_bits(*mut regmap, u16, u16, u16) -> i32;
    fn regmap_read(*mut regmap, u16, *mut u32) -> i32;
    fn clk_hw_get_num_parents(*mut clk_hw) -> u8;
    fn dev_get_regmap(*mut device, *const core::ffi::c_char) -> *mut regmap;
    fn device_get_match_data(*mut device) -> *const core::ffi::c_void;
    fn devm_clk_hw_register(*mut device, *mut clk_hw) -> i32;
    fn devm_of_clk_add_hw_provider(*mut device, unsafe extern "C" fn(*mut of_phandle_args, *mut core::ffi::c_void) -> *mut clk_hw, *mut lochnagar_clk_priv) -> i32;
}

static LOCHNAGAR1_CLK_PARENTS: [clk_parent_data; 11] = [
    clk_parent_data { name: b"ln-none\0".as_ptr() as _, fw_name: b"ln-none\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-spdif-mclk\0".as_ptr() as _, fw_name: b"ln-spdif-mclk\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-psia1-mclk\0".as_ptr() as _, fw_name: b"ln-psia1-mclk\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-psia2-mclk\0".as_ptr() as _, fw_name: b"ln-psia2-mclk\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-cdc-clkout\0".as_ptr() as _, fw_name: b"ln-cdc-clkout\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-dsp-clkout\0".as_ptr() as _, fw_name: b"ln-dsp-clkout\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-pmic-32k\0".as_ptr() as _, fw_name: b"ln-pmic-32k\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-gf-mclk1\0".as_ptr() as _, fw_name: b"ln-gf-mclk1\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-gf-mclk3\0".as_ptr() as _, fw_name: b"ln-gf-mclk3\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-gf-mclk2\0".as_ptr() as _, fw_name: b"ln-gf-mclk2\0".as_ptr() as _ },
    clk_parent_data { name: b"ln-gf-mclk4\0".as_ptr() as _, fw_name: b"ln-gf-mclk4\0".as_ptr() as _ },
];

static LOCHNAGAR2_CLK_PARENTS: [&[u8]; 19] = [
 b"ln-none\0", b"ln-cdc-clkout\0", b"ln-dsp-clkout\0", b"ln-pmic-32k\0", b"ln-spdif-mclk\0",
 b"ln-clk-12m\0", b"ln-clk-11m\0", b"ln-clk-24m\0", b"ln-clk-22m\0", b"ln-clk-8m\0",
 b"ln-usb-clk-24m\0", b"ln-gf-mclk1\0", b"ln-gf-mclk3\0", b"ln-gf-mclk2\0",
 b"ln-psia1-mclk\0", b"ln-psia2-mclk\0", b"ln-spdif-clkout\0", b"ln-adat-mclk\0", b"ln-usb-clk-12m\0",
];

// LN1_CLK and LN2_CLK expand to designated initializers in the source. The
// register and mask constants below are supplied by the Lochnagar headers.
static mut LOCHNAGAR1_CLKS: [lochnagar_clk; LOCHNAGAR_NUM_CLOCKS] = [lochnagar_clk { name: core::ptr::null(), hw: clk_hw { init: core::ptr::null() }, priv_: core::ptr::null_mut(), cfg_reg: 0, ena_mask: 0, src_reg: 0, src_mask: 0 }; LOCHNAGAR_NUM_CLOCKS];
static mut LOCHNAGAR2_CLKS: [lochnagar_clk; LOCHNAGAR_NUM_CLOCKS] = [lochnagar_clk { name: core::ptr::null(), hw: clk_hw { init: core::ptr::null() }, priv_: core::ptr::null_mut(), cfg_reg: 0, ena_mask: 0, src_reg: 0, src_mask: 0 }; LOCHNAGAR_NUM_CLOCKS];

unsafe extern "C" fn lochnagar_clk_prepare(hw: *mut clk_hw) -> i32 { let lclk = lochnagar_hw_to_lclk(hw); regmap_update_bits((*(*lclk).priv_).regmap, (*lclk).cfg_reg, (*lclk).ena_mask, (*lclk).ena_mask) }
unsafe extern "C" fn lochnagar_clk_unprepare(hw: *mut clk_hw) { let lclk = lochnagar_hw_to_lclk(hw); let _ = regmap_update_bits((*(*lclk).priv_).regmap, (*lclk).cfg_reg, (*lclk).ena_mask, 0); }
unsafe extern "C" fn lochnagar_clk_set_parent(hw: *mut clk_hw, index: u8) -> i32 { let lclk = lochnagar_hw_to_lclk(hw); regmap_update_bits((*(*lclk).priv_).regmap, (*lclk).src_reg, (*lclk).src_mask, index as u16) }
unsafe extern "C" fn lochnagar_clk_get_parent(hw: *mut clk_hw) -> u8 { let lclk = lochnagar_hw_to_lclk(hw); let mut val = 0; if regmap_read((*(*lclk).priv_).regmap, (*lclk).src_reg, &mut val) < 0 { return clk_hw_get_num_parents(hw); } (val as u16 & (*lclk).src_mask) as u8 }

static LOCHNAGAR_CLK_OPS: clk_ops = clk_ops { prepare: Some(lochnagar_clk_prepare), unprepare: Some(lochnagar_clk_unprepare), determine_rate: None, set_parent: Some(lochnagar_clk_set_parent), get_parent: Some(lochnagar_clk_get_parent) };

unsafe fn lochnagar_hw_to_lclk(hw: *mut clk_hw) -> *mut lochnagar_clk { hw as *mut lochnagar_clk }

unsafe extern "C" fn lochnagar_of_clk_hw_get(clkspec: *mut of_phandle_args, data: *mut core::ffi::c_void) -> *mut clk_hw { let priv_ = data as *mut lochnagar_clk_priv; let idx = (*clkspec).args[0] as usize; if idx >= LOCHNAGAR_NUM_CLOCKS { return core::ptr::null_mut(); } &mut (*priv_).lclks[idx].hw }

unsafe extern "C" fn lochnagar_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let conf = device_get_match_data(dev) as *const lochnagar_config;
    let mut priv_ = core::mem::MaybeUninit::<lochnagar_clk_priv>::zeroed().assume_init();
    priv_.dev = dev;
    priv_.regmap = dev_get_regmap((*dev).parent, core::ptr::null());
    core::ptr::copy_nonoverlapping((*conf).clks, priv_.lclks.as_mut_ptr(), LOCHNAGAR_NUM_CLOCKS);
    let mut clk_init = clk_init_data { ops: &LOCHNAGAR_CLK_OPS, name: core::ptr::null(), parent_data: (*conf).parents, num_parents: (*conf).nparents };
    for i in 0..LOCHNAGAR_NUM_CLOCKS {
        let lclk = &mut priv_.lclks[i];
        if lclk.name.is_null() { continue; }
        clk_init.name = lclk.name;
        lclk.priv_ = &mut priv_;
        lclk.hw.init = &clk_init;
        let ret = devm_clk_hw_register(dev, &mut lclk.hw);
        if ret != 0 { return ret; }
    }
    devm_of_clk_add_hw_provider(dev, lochnagar_of_clk_hw_get, &mut priv_)
}

// MODULE_DEVICE_TABLE(of), module_platform_driver, and module metadata are
// emitted by the surrounding kernel build integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
