// SPDX-License-Identifier: GPL-2.0
//
// Spreadtrum multiplexer clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependencies supplied by the Linux clock, regmap, and mux headers are
// intentionally left as external Rust items.

extern "C" {
    fn regmap_read(regmap: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(regmap: *mut regmap, reg: u32, val: u32) -> i32;
    fn clk_hw_get_num_parents(hw: *const clk_hw) -> i32;
    fn __clk_mux_determine_rate(hw: *mut clk_hw, rate: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sprd_clk_common {
    pub regmap: *mut regmap,
    pub reg: u32,
    pub hw: clk_hw,
}

#[repr(C)]
pub struct sprd_mux_ssel {
    pub shift: u32,
    pub width: u32,
    pub table: *const u32,
}

#[repr(C)]
pub struct sprd_mux {
    pub common: sprd_clk_common,
    pub mux: sprd_mux_ssel,
}

#[repr(C)]
pub struct clk_ops {
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
    pub determine_rate:
        Option<unsafe extern "C" fn(*mut clk_hw, *mut core::ffi::c_void) -> i32>,
}

extern "C" {
    fn hw_to_sprd_mux(hw: *mut clk_hw) -> *mut sprd_mux;
}

pub unsafe extern "C" fn sprd_mux_helper_get_parent(
    common: *const sprd_clk_common,
    mux: *const sprd_mux_ssel,
) -> u8 {
    let mut reg: u32 = 0;
    let mut parent: u8;
    let num_parents: i32;
    let mut i: i32;

    regmap_read((*common).regmap, (*common).reg, &mut reg);
    parent = (reg >> (*mux).shift) as u8;
    parent &= ((1u32 << (*mux).width) - 1) as u8;

    if (*mux).table.is_null() {
        return parent;
    }

    num_parents = clk_hw_get_num_parents(&(*common).hw);

    i = 0;
    while i < num_parents - 1 {
        if parent as u32 >= *(*mux).table.add(i as usize)
            && (parent as u32) < *(*mux).table.add((i + 1) as usize)
        {
            return i as u8;
        }
        i += 1;
    }

    (num_parents - 1) as u8
}

unsafe extern "C" fn sprd_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let cm = hw_to_sprd_mux(hw);

    sprd_mux_helper_get_parent(&(*cm).common, &(*cm).mux)
}

pub unsafe extern "C" fn sprd_mux_helper_set_parent(
    common: *const sprd_clk_common,
    mux: *const sprd_mux_ssel,
    mut index: u8,
) -> i32 {
    let mut reg: u32 = 0;

    if !(*mux).table.is_null() {
        index = *(*mux).table.add(index as usize) as u8;
    }

    regmap_read((*common).regmap, (*common).reg, &mut reg);
    let mask = ((1u32 << ((*mux).width + (*mux).shift)) - 1)
        & !((1u32 << (*mux).shift) - 1);
    reg &= !mask;
    regmap_write(
        (*common).regmap,
        (*common).reg,
        reg | ((index as u32) << (*mux).shift),
    );

    0
}

unsafe extern "C" fn sprd_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let cm = hw_to_sprd_mux(hw);

    sprd_mux_helper_set_parent(&(*cm).common, &(*cm).mux, index)
}

pub static sprd_mux_ops: clk_ops = clk_ops {
    get_parent: Some(sprd_mux_get_parent),
    set_parent: Some(sprd_mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
