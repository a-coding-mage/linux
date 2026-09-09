/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL mux clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependency intent: declarations from "owl-common.h" are supplied elsewhere.

#[repr(C)]
pub struct owl_mux_hw {
    pub reg: u32,
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct owl_mux {
    pub mux_hw: owl_mux_hw,
    pub common: owl_clk_common,
}

#[macro_export]
macro_rules! OWL_MUX_HW {
    ($reg:expr, $shift:expr, $width:expr) => {
        owl_mux_hw {
            reg: $reg,
            shift: $shift,
            width: $width,
        }
    };
}

#[macro_export]
macro_rules! OWL_MUX {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr, $shift:expr, $width:expr, $flags:expr) => {
        let mut $struct: owl_mux = owl_mux {
            mux_hw: OWL_MUX_HW!($reg, $shift, $width),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw {
                    init: CLK_HW_INIT_PARENTS!($name, $parents, &owl_mux_ops, $flags),
                },
            },
        };
    };
}

#[inline]
pub unsafe fn hw_to_owl_mux(hw: *mut clk_hw) -> *mut owl_mux {
    let common: *mut owl_clk_common = hw_to_owl_clk_common(hw);

    container_of!(common, owl_mux, common)
}

extern "C" {
    pub fn owl_mux_helper_get_parent(
        common: *const owl_clk_common,
        mux_hw: *const owl_mux_hw,
    ) -> u8;
    pub fn owl_mux_helper_set_parent(
        common: *const owl_clk_common,
        mux_hw: *mut owl_mux_hw,
        index: u8,
    ) -> i32;

    pub static owl_mux_ops: clk_ops;
}

// External declarations supplied by owl-common.h and related clock code.
extern "C" {
    fn hw_to_owl_clk_common(hw: *mut clk_hw) -> *mut owl_clk_common;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
