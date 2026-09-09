/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL factor clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependency supplied by owl-common.h.

#[repr(C)]
pub struct clk_factor_table {
    pub val: ::std::os::raw::c_uint,
    pub mul: ::std::os::raw::c_uint,
    pub div: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct owl_factor_hw {
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
    pub fct_flags: u8,
    pub table: *mut clk_factor_table,
}

#[repr(C)]
pub struct owl_factor {
    pub factor_hw: owl_factor_hw,
    pub common: owl_clk_common,
}

#[macro_export]
macro_rules! OWL_FACTOR_HW {
    ($reg:expr, $shift:expr, $width:expr, $fct_flags:expr, $table:expr) => {
        owl_factor_hw {
            reg: $reg,
            shift: $shift,
            width: $width,
            fct_flags: $fct_flags,
            table: $table,
        }
    };
}

#[macro_export]
macro_rules! div_mask {
    ($d:expr) => {
        ((1i32 << (($d).width as i32)) - 1)
    };
}

// The C OWL_FACTOR initializer is retained as a Rust macro; CLK_HW_INIT and
// the surrounding common-clock definitions are supplied by owl-common.h.
#[macro_export]
macro_rules! OWL_FACTOR {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $shift:expr,
     $width:expr, $table:expr, $fct_flags:expr, $flags:expr) => {
        let $struct: owl_factor = owl_factor {
            factor_hw: OWL_FACTOR_HW!($reg, $shift, $width, $fct_flags, $table),
            common: owl_clk_common {
                regmap: ::std::ptr::null_mut(),
                hw: CLK_HW_INIT!($name, $parent, &owl_factor_ops, $flags),
            },
        };
    };
}

pub unsafe fn hw_to_owl_factor(hw: *mut clk_hw) -> *mut owl_factor {
    let common = hw_to_owl_clk_common(hw);
    container_of!(common, owl_factor, common)
}

extern "C" {
    pub fn owl_factor_helper_round_rate(
        common: *mut owl_clk_common,
        factor_hw: *const owl_factor_hw,
        rate: ::std::os::raw::c_ulong,
        parent_rate: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;

    pub fn owl_factor_helper_recalc_rate(
        common: *mut owl_clk_common,
        factor_hw: *const owl_factor_hw,
        parent_rate: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;

    pub fn owl_factor_helper_set_rate(
        common: *const owl_clk_common,
        factor_hw: *const owl_factor_hw,
        rate: ::std::os::raw::c_ulong,
        parent_rate: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;

    pub static owl_factor_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
