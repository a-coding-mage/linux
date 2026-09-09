/* SPDX-License-Identifier: GPL-2.0 */
//
// Spreadtrum composite clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependencies supplied by common.h, mux.h, and div.h are intentionally left
// external to this translation.

#[repr(C)]
pub struct sprd_comp {
    pub mux: sprd_mux_ssel,
    pub div: sprd_div_internal,
    pub common: sprd_clk_common,
}

#[macro_export]
macro_rules! SPRD_COMP_CLK_HW_INIT_FN {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $table:expr,
     $mshift:expr, $mwidth:expr, $doffset:expr, $dshift:expr,
     $dwidth:expr, $flags:expr, $fn:ident) => {
        let mut $struct = sprd_comp {
            mux: _SPRD_MUX_CLK!($mshift, $mwidth, $table),
            div: _SPRD_DIV_CLK!($doffset, $dshift, $dwidth),
            common: sprd_clk_common {
                regmap: core::ptr::null_mut(),
                reg: $reg,
                hw: clk_hw_container {
                    init: $fn!($name, $parent, &sprd_comp_ops, $flags),
                },
            },
        };
    };
}

#[macro_export]
macro_rules! SPRD_COMP_CLK_TABLE {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $table:expr,
     $mshift:expr, $mwidth:expr, $dshift:expr, $dwidth:expr, $flags:expr) => {
        SPRD_COMP_CLK_HW_INIT_FN!($struct, $name, $parent, $reg, $table,
                                  $mshift, $mwidth, 0x0, $dshift, $dwidth,
                                  $flags, CLK_HW_INIT_PARENTS);
    };
}

#[macro_export]
macro_rules! SPRD_COMP_CLK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $mshift:expr,
     $mwidth:expr, $dshift:expr, $dwidth:expr, $flags:expr) => {
        SPRD_COMP_CLK_TABLE!($struct, $name, $parent, $reg, core::ptr::null(),
                             $mshift, $mwidth, $dshift, $dwidth, $flags);
    };
}

#[macro_export]
macro_rules! SPRD_COMP_CLK_DATA_TABLE {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $table:expr,
     $mshift:expr, $mwidth:expr, $dshift:expr, $dwidth:expr, $flags:expr) => {
        SPRD_COMP_CLK_HW_INIT_FN!($struct, $name, $parent, $reg, $table,
                                  $mshift, $mwidth, 0x0, $dshift, $dwidth,
                                  $flags, CLK_HW_INIT_PARENTS_DATA);
    };
}

#[macro_export]
macro_rules! SPRD_COMP_CLK_DATA {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $mshift:expr,
     $mwidth:expr, $dshift:expr, $dwidth:expr, $flags:expr) => {
        SPRD_COMP_CLK_DATA_TABLE!($struct, $name, $parent, $reg,
                                  core::ptr::null(), $mshift, $mwidth,
                                  $dshift, $dwidth, $flags);
    };
}

#[macro_export]
macro_rules! SPRD_COMP_CLK_DATA_TABLE_OFFSET {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $table:expr,
     $mshift:expr, $mwidth:expr, $doffset:expr, $dshift:expr,
     $dwidth:expr, $flags:expr) => {
        SPRD_COMP_CLK_HW_INIT_FN!($struct, $name, $parent, $reg, $table,
                                  $mshift, $mwidth, $doffset, $dshift,
                                  $dwidth, $flags, CLK_HW_INIT_PARENTS_DATA);
    };
}

#[macro_export]
macro_rules! SPRD_COMP_CLK_DATA_OFFSET {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $mshift:expr,
     $mwidth:expr, $doffset:expr, $dshift:expr, $dwidth:expr, $flags:expr) => {
        SPRD_COMP_CLK_DATA_TABLE_OFFSET!($struct, $name, $parent, $reg,
                                         core::ptr::null(), $mshift, $mwidth,
                                         $doffset, $dshift, $dwidth, $flags);
    };
}

pub unsafe fn hw_to_sprd_comp(hw: *const clk_hw) -> *mut sprd_comp {
    let common = hw_to_sprd_clk_common(hw);
    container_of!(common, sprd_comp, common)
}

extern "C" {
    pub static sprd_comp_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
