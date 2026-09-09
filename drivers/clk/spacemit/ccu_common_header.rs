/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024 SpacemiT Technology Co. Ltd
 * Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_char;

#[allow(non_camel_case_types)]
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ccu_common_ddn_mix {
    pub reg_ctrl: u32,
    pub reg_fc: u32,
    pub mask_fc: u32,
}

#[repr(C)]
pub struct ccu_common_pll {
    pub reg_swcr1: u32,
    pub reg_swcr2: u32,
    pub reg_swcr3: u32,
}

#[repr(C)]
pub union ccu_common_registers {
    /* For DDN and MIX */
    pub ddn_mix: core::mem::ManuallyDrop<ccu_common_ddn_mix>,
    /* For PLL */
    pub pll: core::mem::ManuallyDrop<ccu_common_pll>,
}

#[repr(C)]
pub struct ccu_common {
    pub regmap: *mut regmap,
    pub lock_regmap: *mut regmap,
    pub registers: ccu_common_registers,
    pub hw: clk_hw,
}

pub unsafe fn hw_to_ccu_common(hw: *mut clk_hw) -> *mut ccu_common {
    (hw as *mut u8).sub(core::mem::offset_of!(ccu_common, hw)) as *mut ccu_common
}

#[repr(C)]
pub struct spacemit_ccu_data {
    pub reset_name: *const c_char,
    pub hws: *mut *mut clk_hw,
    pub num: usize,
}

#[macro_export]
macro_rules! ccu_read {
    ($c:expr, $reg:ident) => {{
        let mut tmp: $crate::u32 = 0;
        unsafe {
            regmap_read(
                (*$c).regmap,
                (*$c).registers.ddn_mix.$reg,
                &mut tmp as *mut $crate::u32,
            );
        }
        tmp
    }};
}

#[macro_export]
macro_rules! ccu_update {
    ($c:expr, $reg:ident, $mask:expr, $val:expr) => {{
        unsafe {
            regmap_update_bits(
                (*$c).regmap,
                (*$c).registers.ddn_mix.$reg,
                $mask,
                $val,
            )
        }
    }};
}

extern "C" {
    pub fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> core::ffi::c_int;
    pub fn regmap_update_bits(
        map: *mut regmap,
        reg: u32,
        mask: u32,
        val: u32,
    ) -> core::ffi::c_int;
    pub fn spacemit_ccu_probe(pdev: *mut platform_device, compat: *const c_char) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
