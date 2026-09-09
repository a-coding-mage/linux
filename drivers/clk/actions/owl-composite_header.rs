/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL composite clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependencies supplied by: owl-common.h, owl-mux.h, owl-gate.h,
// owl-factor.h, owl-fixed-factor.h, and owl-divider.h.

#[repr(C)]
pub union owl_rate {
    pub div_hw: owl_divider_hw,
    pub factor_hw: owl_factor_hw,
    pub fix_fact_hw: clk_fixed_factor,
}

#[repr(C)]
pub struct owl_composite {
    pub mux_hw: owl_mux_hw,
    pub gate_hw: owl_gate_hw,
    pub rate: owl_rate,
    pub fix_fact_ops: *const clk_ops,
    pub common: owl_clk_common,
}

#[macro_export]
macro_rules! OWL_COMP_DIV {
    ($struct:ident, $name:expr, $parent:expr, $mux:expr, $gate:expr, $div:expr, $flags:expr) => {
        let mut $struct = owl_composite {
            mux_hw: $mux,
            gate_hw: $gate,
            rate: owl_rate { div_hw: $div },
            fix_fact_ops: core::ptr::null(),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw_init_parents($name, $parent, &owl_comp_div_ops, $flags),
            },
        };
    };
}

#[macro_export]
macro_rules! OWL_COMP_DIV_FIXED {
    ($struct:ident, $name:expr, $parent:expr, $gate:expr, $div:expr, $flags:expr) => {
        let mut $struct = owl_composite {
            mux_hw: unsafe { core::mem::zeroed() },
            gate_hw: $gate,
            rate: owl_rate { div_hw: $div },
            fix_fact_ops: core::ptr::null(),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw_init($name, $parent, &owl_comp_div_ops, $flags),
            },
        };
    };
}

#[macro_export]
macro_rules! OWL_COMP_FACTOR {
    ($struct:ident, $name:expr, $parent:expr, $mux:expr, $gate:expr, $factor:expr, $flags:expr) => {
        let mut $struct = owl_composite {
            mux_hw: $mux,
            gate_hw: $gate,
            rate: owl_rate { factor_hw: $factor },
            fix_fact_ops: core::ptr::null(),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw_init_parents($name, $parent, &owl_comp_fact_ops, $flags),
            },
        };
    };
}

#[macro_export]
macro_rules! OWL_COMP_FIXED_FACTOR {
    ($struct:ident, $name:expr, $parent:expr, $gate:expr, $mul:expr, $div:expr, $flags:expr) => {
        let mut $struct = owl_composite {
            mux_hw: unsafe { core::mem::zeroed() },
            gate_hw: $gate,
            rate: owl_rate {
                fix_fact_hw: clk_fixed_factor { mult: $mul, div: $div },
            },
            fix_fact_ops: &clk_fixed_factor_ops,
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw_init($name, $parent, &owl_comp_fix_fact_ops, $flags),
            },
        };
    };
}

#[macro_export]
macro_rules! OWL_COMP_PASS {
    ($struct:ident, $name:expr, $parent:expr, $mux:expr, $gate:expr, $flags:expr) => {
        let mut $struct = owl_composite {
            mux_hw: $mux,
            gate_hw: $gate,
            rate: unsafe { core::mem::zeroed() },
            fix_fact_ops: core::ptr::null(),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw_init_parents($name, $parent, &owl_comp_pass_ops, $flags),
            },
        };
    };
}

pub unsafe fn hw_to_owl_comp(hw: *mut clk_hw) -> *mut owl_composite {
    let common = hw_to_owl_clk_common(hw);
    container_of!(common, owl_composite, common)
}

extern "C" {
    pub static owl_comp_div_ops: clk_ops;
    pub static owl_comp_fact_ops: clk_ops;
    pub static owl_comp_fix_fact_ops: clk_ops;
    pub static owl_comp_pass_ops: clk_ops;
    pub static clk_fixed_factor_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
