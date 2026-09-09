/* SPDX-License-Identifier: GPL-2.0 */
//
// Spreadtrum multiplexer clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependency supplied by the corresponding common header.

/// struct sprd_mux_ssel - Mux clock's source select bits in its register
/// @shift: Bit offset of the divider in its register
/// @width: Width of the divider field in its register
/// @table: For some mux clocks, not all sources are used on some special
///         chips, this matches the value of mux clock's register and the
///         sources which are used for this mux clock
#[repr(C)]
pub struct sprd_mux_ssel {
    pub shift: u8,
    pub width: u8,
    pub table: *const u8,
}

#[repr(C)]
pub struct sprd_mux {
    pub mux: sprd_mux_ssel,
    pub common: sprd_clk_common,
}

#[macro_export]
macro_rules! _SPRD_MUX_CLK {
    ($shift:expr, $width:expr, $table:expr) => {
        sprd_mux_ssel {
            shift: $shift,
            width: $width,
            table: $table,
        }
    };
}

#[macro_export]
macro_rules! SPRD_MUX_CLK_HW_INIT_FN {
    ($struct:ident, $name:expr, $parents:expr, $table:expr,
     $reg:expr, $shift:expr, $width:expr, $flags:expr, $fn:expr) => {
        let mut $struct: sprd_mux = sprd_mux {
            mux: _SPRD_MUX_CLK!($shift, $width, $table),
            common: sprd_clk_common {
                regmap: core::ptr::null_mut(),
                reg: $reg,
                hw: clk_hw {
                    init: $fn($name, $parents, &sprd_mux_ops, $flags),
                },
            },
        };
    };
}

#[macro_export]
macro_rules! SPRD_MUX_CLK_TABLE {
    ($struct:ident, $name:expr, $parents:expr, $table:expr,
     $reg:expr, $shift:expr, $width:expr, $flags:expr) => {
        SPRD_MUX_CLK_HW_INIT_FN!($struct, $name, $parents, $table,
                                 $reg, $shift, $width, $flags,
                                 CLK_HW_INIT_PARENTS)
    };
}

#[macro_export]
macro_rules! SPRD_MUX_CLK {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $shift:expr, $width:expr, $flags:expr) => {
        SPRD_MUX_CLK_TABLE!($struct, $name, $parents, core::ptr::null(),
                            $reg, $shift, $width, $flags)
    };
}

#[macro_export]
macro_rules! SPRD_MUX_CLK_DATA_TABLE {
    ($struct:ident, $name:expr, $parents:expr, $table:expr,
     $reg:expr, $shift:expr, $width:expr, $flags:expr) => {
        SPRD_MUX_CLK_HW_INIT_FN!($struct, $name, $parents, $table,
                                 $reg, $shift, $width, $flags,
                                 CLK_HW_INIT_PARENTS_DATA)
    };
}

#[macro_export]
macro_rules! SPRD_MUX_CLK_DATA {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $shift:expr, $width:expr, $flags:expr) => {
        SPRD_MUX_CLK_DATA_TABLE!($struct, $name, $parents, core::ptr::null(),
                                 $reg, $shift, $width, $flags)
    };
}

#[inline]
pub unsafe fn hw_to_sprd_mux(hw: *const clk_hw) -> *mut sprd_mux {
    let common: *mut sprd_clk_common = hw_to_sprd_clk_common(hw);

    container_of!(common, sprd_mux, common)
}

extern "C" {
    pub static sprd_mux_ops: clk_ops;

    pub fn sprd_mux_helper_get_parent(
        common: *const sprd_clk_common,
        mux: *const sprd_mux_ssel,
    ) -> u8;
    pub fn sprd_mux_helper_set_parent(
        common: *const sprd_clk_common,
        mux: *const sprd_mux_ssel,
        index: u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
