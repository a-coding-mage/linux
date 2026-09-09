/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL divider clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependency supplied by owl-common.h in the C source.

#[repr(C)]
pub struct owl_divider_hw {
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
    pub div_flags: u8,
    pub table: *mut clk_div_table,
}

#[repr(C)]
pub struct owl_divider {
    pub div_hw: owl_divider_hw,
    pub common: owl_clk_common,
}

#[macro_export]
macro_rules! OWL_DIVIDER_HW {
    ($reg:expr, $shift:expr, $width:expr, $div_flags:expr, $table:expr) => {
        owl_divider_hw {
            reg: $reg,
            shift: $shift,
            width: $width,
            div_flags: $div_flags,
            table: $table,
        }
    };
}

#[macro_export]
macro_rules! OWL_DIVIDER {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr,
     $shift:expr, $width:expr, $table:expr, $div_flags:expr, $flags:expr) => {
        let mut $struct: owl_divider = owl_divider {
            div_hw: OWL_DIVIDER_HW!($reg, $shift, $width, $div_flags, $table),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw_init!($name, $parent, &owl_divider_ops, $flags),
            },
        };
    };
}

#[inline]
pub unsafe fn hw_to_owl_divider(hw: *mut clk_hw) -> *mut owl_divider {
    let common: *mut owl_clk_common = hw_to_owl_clk_common(hw);

    container_of!(common, owl_divider, common)
}

pub fn owl_divider_helper_recalc_rate(
    common: *mut owl_clk_common,
    div_hw: *const owl_divider_hw,
    parent_rate: u64,
) -> u64;

pub fn owl_divider_helper_set_rate(
    common: *const owl_clk_common,
    div_hw: *const owl_divider_hw,
    rate: u64,
    parent_rate: u64,
) -> i32;

extern "C" {
    pub static owl_divider_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
