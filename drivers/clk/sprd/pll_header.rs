/* SPDX-License-Identifier: GPL-2.0 */
//
// Spreadtrum pll clock driver
//
// Copyright (C) 2015~2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// C dependency: #include "common.h"

#[repr(C)]
pub struct RegCfg {
    pub val: u32,
    pub msk: u32,
}

#[repr(C)]
pub struct ClkBitField {
    pub shift: u8,
    pub width: u8,
}

#[repr(i32)]
pub enum PllFactor {
    PllLockDone,
    PllDivS,
    PllModEn,
    PllSdmEn,
    PllRefin,
    PllIbias,
    PllN,
    PllNint,
    PllKint,
    PllPrediv,
    PllPostdiv,
    PllFactMax,
}

/*
 * struct sprd_pll - definition of adjustable pll clock
 *
 * @reg: registers used to set the configuration of pll clock,
 *       reg[0] shows how many registers this pll clock uses.
 * @itable: pll ibias table, itable[0] means how many items this
 *          table includes
 * @udelay delay time after setting rate
 * @factors used to calculate the pll clock rate
 * @fvco: fvco threshold rate
 * @fflag: fvco flag
 */
#[repr(C)]
pub struct SprdPll {
    pub regs_num: u32,
    pub itable: *const u64,
    pub factors: *const ClkBitField,
    pub udelay: u16,
    pub k1: u16,
    pub k2: u16,
    pub fflag: u16,
    pub fvco: u64,
    pub common: SprdClkCommon,
}

// C macro translation. The referenced types, constants, and functions are supplied by common dependencies.
#[macro_export]
macro_rules! sprd_pll_hw_init_fn {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $regs_num:expr,
     $itable:expr, $factors:expr, $udelay:expr, $k1:expr, $k2:expr,
     $fflag:expr, $fvco:expr, $fn:ident) => {
        let $struct = SprdPll {
            regs_num: $regs_num,
            itable: $itable,
            factors: $factors,
            udelay: $udelay,
            k1: $k1,
            k2: $k2,
            fflag: $fflag,
            fvco: $fvco,
            common: SprdClkCommon {
                regmap: core::ptr::null_mut(),
                reg: $reg,
                hw: ClkHw {
                    init: $fn($name, $parent, &sprd_pll_ops, 0),
                },
            },
        };
    };
}

#[macro_export]
macro_rules! sprd_pll_with_itable_k_fvco {
    ($($args:tt)*) => { sprd_pll_hw_init_fn!($($args)*, clk_hw_init); };
}

#[macro_export]
macro_rules! sprd_pll_with_itable_k {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $regs_num:expr,
     $itable:expr, $factors:expr, $udelay:expr, $k1:expr, $k2:expr) => {
        sprd_pll_with_itable_k_fvco!($struct, $name, $parent, $reg, $regs_num,
            $itable, $factors, $udelay, $k1, $k2, 0, 0);
    };
}

#[macro_export]
macro_rules! sprd_pll_with_itable_1k {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $regs_num:expr,
     $itable:expr, $factors:expr, $udelay:expr) => {
        sprd_pll_with_itable_k_fvco!($struct, $name, $parent, $reg, $regs_num,
            $itable, $factors, $udelay, 1000, 1000, 0, 0);
    };
}

#[macro_export]
macro_rules! sprd_pll_fw_name {
    ($($args:tt)*) => { sprd_pll_hw_init_fn!($($args)*, clk_hw_init_fw_name); };
}

#[macro_export]
macro_rules! sprd_pll_hw {
    ($($args:tt)*) => { sprd_pll_hw_init_fn!($($args)*, clk_hw_init_hw); };
}

pub unsafe fn hw_to_sprd_pll(hw: *mut ClkHw) -> *mut SprdPll {
    let common = hw_to_sprd_clk_common(hw);
    container_of!(common, SprdPll, common)
}

extern "C" {
    pub static sprd_pll_ops: ClkOps;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
