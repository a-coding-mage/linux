// SPDX-License-Identifier: GPL-2.0+
//
// OWL mux clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependencies corresponding to <linux/clk-provider.h>, <linux/regmap.h>,
// and "owl-mux.h" are supplied by the surrounding translation unit.

extern "C" {
    fn regmap_read(regmap: *mut regmap, reg: u32, value: *mut u32) -> i32;
    fn regmap_write(regmap: *mut regmap, reg: u32, value: u32) -> i32;
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct owl_clk_common {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct owl_mux_hw {
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct owl_mux {
    pub common: owl_clk_common,
    pub mux_hw: owl_mux_hw,
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
    pub determine_rate: Option<unsafe extern "C" fn() -> i32>,
}

extern "C" {
    fn __clk_mux_determine_rate() -> i32;
}

#[inline]
unsafe fn hw_to_owl_mux(hw: *mut clk_hw) -> *mut owl_mux {
    hw as *mut owl_mux
}

pub unsafe extern "C" fn owl_mux_helper_get_parent(
    common: *const owl_clk_common,
    mux_hw: *const owl_mux_hw,
) -> u8 {
    let mut reg: u32 = 0;
    let mut parent: u8;

    regmap_read((*common).regmap, (*mux_hw).reg, &mut reg);
    parent = (reg >> (*mux_hw).shift) as u8;
    parent &= ((1u32 << (*mux_hw).width) - 1) as u8;

    parent
}

unsafe extern "C" fn owl_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = hw_to_owl_mux(hw);

    owl_mux_helper_get_parent(&(*mux).common, &(*mux).mux_hw)
}

pub unsafe extern "C" fn owl_mux_helper_set_parent(
    common: *const owl_clk_common,
    mux_hw: *mut owl_mux_hw,
    index: u8,
) -> i32 {
    let mut reg: u32 = 0;

    regmap_read((*common).regmap, (*mux_hw).reg, &mut reg);
    reg &= !(((1u32 << ((*mux_hw).width + (*mux_hw).shift)) - 1)
        & !((1u32 << (*mux_hw).shift) - 1));
    regmap_write(
        (*common).regmap,
        (*mux_hw).reg,
        reg | ((index as u32) << (*mux_hw).shift),
    );

    0
}

unsafe extern "C" fn owl_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mux = hw_to_owl_mux(hw);

    owl_mux_helper_set_parent(&(*mux).common, &mut (*mux).mux_hw, index)
}

pub static owl_mux_ops: clk_ops = clk_ops {
    get_parent: Some(owl_mux_get_parent),
    set_parent: Some(owl_mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
