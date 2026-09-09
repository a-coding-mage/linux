/* SPDX-License-Identifier: GPL-2.0 */
//
// Spreadtrum gate clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependency provided by common.h in the C source.

#[repr(C)]
pub struct sprd_gate {
    pub enable_mask: u32,
    pub flags: u16,
    pub sc_offset: u16,
    pub udelay: u16,
    pub common: sprd_clk_common,
}

/*
 * sprd_gate::flags is used for:
 * CLK_GATE_SET_TO_DISABLE BIT(0)
 * CLK_GATE_HIWORD_MASK    BIT(1)
 * CLK_GATE_BIG_ENDIAN     BIT(2)
 * so we define new flags from BIT(3)
 */
pub const SPRD_GATE_NON_AON: u16 = 1u16 << 3; // not always powered on, check before read

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_HW_INIT_FN {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $udelay:expr, $ops:expr, $fn:expr) => {
        let $struct = sprd_gate {
            enable_mask: $enable_mask,
            sc_offset: $sc_offset,
            flags: $gate_flags,
            udelay: $udelay,
            common: sprd_clk_common {
                regmap: core::ptr::null_mut(),
                reg: $reg,
                hw: ($fn)($name, $parent, $ops, $flags),
            },
        };
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_OPS_UDELAY {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $udelay:expr, $ops:expr) => {
        SPRD_SC_GATE_CLK_HW_INIT_FN!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, $udelay, $ops, CLK_HW_INIT)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_OPS {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $ops:expr) => {
        SPRD_SC_GATE_CLK_OPS_UDELAY!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, 0, $ops)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr) => {
        SPRD_SC_GATE_CLK_OPS!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, &sprd_sc_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_GATE_CLK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $enable_mask:expr,
     $flags:expr, $gate_flags:expr) => {
        SPRD_SC_GATE_CLK_OPS!($struct, $name, $parent, $reg, 0,
            $enable_mask, $flags, $gate_flags, &sprd_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_PLL_SC_GATE_CLK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $udelay:expr) => {
        SPRD_SC_GATE_CLK_OPS_UDELAY!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, $udelay, &sprd_pll_sc_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_HW_OPS_UDELAY {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $udelay:expr, $ops:expr) => {
        SPRD_SC_GATE_CLK_HW_INIT_FN!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, $udelay, $ops, CLK_HW_INIT_HW)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_HW_OPS {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $ops:expr) => {
        SPRD_SC_GATE_CLK_HW_OPS_UDELAY!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, 0, $ops)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_HW {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr) => {
        SPRD_SC_GATE_CLK_HW_OPS!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, &sprd_sc_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_GATE_CLK_HW {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $enable_mask:expr,
     $flags:expr, $gate_flags:expr) => {
        SPRD_SC_GATE_CLK_HW_OPS!($struct, $name, $parent, $reg, 0,
            $enable_mask, $flags, $gate_flags, &sprd_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_PLL_SC_GATE_CLK_HW {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $udelay:expr) => {
        SPRD_SC_GATE_CLK_HW_OPS_UDELAY!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, $udelay, &sprd_pll_sc_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_FW_NAME_OPS_UDELAY {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $udelay:expr, $ops:expr) => {
        SPRD_SC_GATE_CLK_HW_INIT_FN!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, $udelay, $ops, CLK_HW_INIT_FW_NAME)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_FW_NAME_OPS {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $ops:expr) => {
        SPRD_SC_GATE_CLK_FW_NAME_OPS_UDELAY!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, 0, $ops)
    };
}

#[macro_export]
macro_rules! SPRD_SC_GATE_CLK_FW_NAME {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr) => {
        SPRD_SC_GATE_CLK_FW_NAME_OPS!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, &sprd_sc_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_GATE_CLK_FW_NAME {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $enable_mask:expr,
     $flags:expr, $gate_flags:expr) => {
        SPRD_SC_GATE_CLK_FW_NAME_OPS!($struct, $name, $parent, $reg, 0,
            $enable_mask, $flags, $gate_flags, &sprd_gate_ops)
    };
}

#[macro_export]
macro_rules! SPRD_PLL_SC_GATE_CLK_FW_NAME {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $sc_offset:expr,
     $enable_mask:expr, $flags:expr, $gate_flags:expr, $udelay:expr) => {
        SPRD_SC_GATE_CLK_FW_NAME_OPS_UDELAY!($struct, $name, $parent, $reg, $sc_offset,
            $enable_mask, $flags, $gate_flags, $udelay, &sprd_pll_sc_gate_ops)
    };
}

pub unsafe fn hw_to_sprd_gate(hw: *const clk_hw) -> *mut sprd_gate {
    let common = hw_to_sprd_clk_common(hw);
    container_of!(common, sprd_gate, common)
}

extern "C" {
    pub static sprd_gate_ops: clk_ops;
    pub static sprd_sc_gate_ops: clk_ops;
    pub static sprd_pll_sc_gate_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
