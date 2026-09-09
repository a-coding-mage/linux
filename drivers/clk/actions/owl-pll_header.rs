/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL pll clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependency intent: declarations supplied by "owl-common.h" are external.

pub const OWL_PLL_DEF_DELAY: u8 = 50;

/// Last entry should have rate = 0.
#[repr(C)]
pub struct clk_pll_table {
    pub val: u32,
    pub rate: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct owl_pll_hw {
    pub reg: u32,
    pub bfreq: u32,
    pub bit_idx: u8,
    pub shift: u8,
    pub width: u8,
    pub min_mul: u8,
    pub max_mul: u8,
    pub delay: u8,
    pub table: *const clk_pll_table,
}

#[repr(C)]
pub struct owl_pll {
    pub pll_hw: owl_pll_hw,
    pub common: owl_clk_common,
}

#[macro_export]
macro_rules! OWL_PLL_HW {
    ($reg:expr, $bfreq:expr, $bit_idx:expr, $shift:expr, $width:expr,
     $min_mul:expr, $max_mul:expr, $delay:expr, $table:expr) => {
        owl_pll_hw {
            reg: $reg,
            bfreq: $bfreq,
            bit_idx: $bit_idx,
            shift: $shift,
            width: $width,
            min_mul: $min_mul,
            max_mul: $max_mul,
            delay: $delay,
            table: $table,
        }
    };
}

#[macro_export]
macro_rules! OWL_PLL {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $bfreq:expr,
     $bit_idx:expr, $shift:expr, $width:expr, $min_mul:expr, $max_mul:expr,
     $table:expr, $flags:expr) => {
        let $struct = owl_pll {
            pll_hw: OWL_PLL_HW!($reg, $bfreq, $bit_idx, $shift, $width,
                                $min_mul, $max_mul, OWL_PLL_DEF_DELAY, $table),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw { init: CLK_HW_INIT!($name, $parent, &owl_pll_ops, $flags) },
            },
        };
    };
}

#[macro_export]
macro_rules! OWL_PLL_NO_PARENT {
    ($struct:ident, $name:expr, $reg:expr, $bfreq:expr, $bit_idx:expr,
     $shift:expr, $width:expr, $min_mul:expr, $max_mul:expr, $table:expr,
     $flags:expr) => {
        let $struct = owl_pll {
            pll_hw: OWL_PLL_HW!($reg, $bfreq, $bit_idx, $shift, $width,
                                $min_mul, $max_mul, OWL_PLL_DEF_DELAY, $table),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw { init: CLK_HW_INIT_NO_PARENT!($name, &owl_pll_ops, $flags) },
            },
        };
    };
}

#[macro_export]
macro_rules! OWL_PLL_NO_PARENT_DELAY {
    ($struct:ident, $name:expr, $reg:expr, $bfreq:expr, $bit_idx:expr,
     $shift:expr, $width:expr, $min_mul:expr, $max_mul:expr, $delay:expr,
     $table:expr, $flags:expr) => {
        let $struct = owl_pll {
            pll_hw: OWL_PLL_HW!($reg, $bfreq, $bit_idx, $shift, $width,
                                $min_mul, $max_mul, $delay, $table),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw { init: CLK_HW_INIT_NO_PARENT!($name, &owl_pll_ops, $flags) },
            },
        };
    };
}

#[inline]
pub unsafe fn mul_mask(m: *const owl_pll_hw) -> u32 {
    (1u32 << (*m).width) - 1
}

#[inline]
pub unsafe fn hw_to_owl_pll(hw: *mut clk_hw) -> *mut owl_pll {
    let common = hw_to_owl_clk_common(hw);
    container_of!(common, owl_pll, common)
}

pub extern "C" {
    pub static owl_pll_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
