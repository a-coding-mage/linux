/* SPDX-License-Identifier: GPL-2.0 */
//
// Spreadtrum divider clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependency declarations from "common.h" are supplied by other files.

/**
 * struct sprd_div_internal - Internal divider description
 * @shift: Bit offset of the divider in its register
 * @width: Width of the divider field in its register
 *
 * That structure represents a single divider, and is meant to be
 * embedded in other structures representing the various clock
 * classes.
 */
#[repr(C)]
pub struct sprd_div_internal {
    pub offset: i32,
    pub shift: u8,
    pub width: u8,
}

#[macro_export]
macro_rules! _SPRD_DIV_CLK {
    ($offset:expr, $shift:expr, $width:expr) => {
        sprd_div_internal {
            offset: $offset,
            shift: $shift,
            width: $width,
        }
    };
}

#[repr(C)]
pub struct sprd_div {
    pub div: sprd_div_internal,
    pub common: sprd_clk_common,
}

#[macro_export]
macro_rules! SPRD_DIV_CLK_HW_INIT_FN {
    ($struct_:ident, $name:expr, $parent:expr, $reg:expr, $offset:expr,
     $shift:expr, $width:expr, $flags:expr, $fn_:ident) => {
        let mut $struct_: sprd_div = sprd_div {
            div: _SPRD_DIV_CLK!($offset, $shift, $width),
            common: sprd_clk_common {
                regmap: core::ptr::null_mut(),
                reg: $reg,
                hw: clk_hw {
                    init: $fn_($name, $parent, &sprd_div_ops, $flags),
                },
            },
        };
    };
}

#[macro_export]
macro_rules! SPRD_DIV_CLK {
    ($struct_:ident, $name:expr, $parent:expr, $reg:expr,
     $shift:expr, $width:expr, $flags:expr) => {
        SPRD_DIV_CLK_HW_INIT_FN!($struct_, $name, $parent, $reg, 0x0,
                                 $shift, $width, $flags, CLK_HW_INIT);
    };
}

#[macro_export]
macro_rules! SPRD_DIV_CLK_FW_NAME {
    ($struct_:ident, $name:expr, $parent:expr, $reg:expr,
     $shift:expr, $width:expr, $flags:expr) => {
        SPRD_DIV_CLK_HW_INIT_FN!($struct_, $name, $parent, $reg, 0x0,
                                 $shift, $width, $flags, CLK_HW_INIT_FW_NAME);
    };
}

#[macro_export]
macro_rules! SPRD_DIV_CLK_HW {
    ($struct_:ident, $name:expr, $parent:expr, $reg:expr,
     $shift:expr, $width:expr, $flags:expr) => {
        SPRD_DIV_CLK_HW_INIT_FN!($struct_, $name, $parent, $reg, 0x0,
                                 $shift, $width, $flags, CLK_HW_INIT_HW);
    };
}

pub unsafe fn hw_to_sprd_div(hw: *const clk_hw) -> *mut sprd_div {
    let common = hw_to_sprd_clk_common(hw);
    // Equivalent to container_of(common, struct sprd_div, common).
    (common as *mut u8).sub(core::mem::offset_of!(sprd_div, common)) as *mut sprd_div
}

extern "C" {
    pub fn sprd_div_helper_recalc_rate(
        common: *mut sprd_clk_common,
        div: *const sprd_div_internal,
        parent_rate: usize,
    ) -> usize;

    pub fn sprd_div_helper_set_rate(
        common: *const sprd_clk_common,
        div: *const sprd_div_internal,
        rate: usize,
        parent_rate: usize,
    ) -> i32;

    pub static sprd_div_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
