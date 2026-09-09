/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/compiler.h, linux/clk-provider.h

pub const CCU_FEATURE_FRACTIONAL: u32 = 1u32 << 0;
pub const CCU_FEATURE_VARIABLE_PREDIV: u32 = 1u32 << 1;
pub const CCU_FEATURE_FIXED_PREDIV: u32 = 1u32 << 2;
pub const CCU_FEATURE_FIXED_POSTDIV: u32 = 1u32 << 3;
pub const CCU_FEATURE_ALL_PREDIV: u32 = 1u32 << 4;
pub const CCU_FEATURE_LOCK_REG: u32 = 1u32 << 5;
pub const CCU_FEATURE_MMC_TIMING_SWITCH: u32 = 1u32 << 6;
pub const CCU_FEATURE_SIGMA_DELTA_MOD: u32 = 1u32 << 7;
pub const CCU_FEATURE_KEY_FIELD: u32 = 1u32 << 8;
pub const CCU_FEATURE_CLOSEST_RATE: u32 = 1u32 << 9;
pub const CCU_FEATURE_DUAL_DIV: u32 = 1u32 << 10;
pub const CCU_FEATURE_UPDATE_BIT: u32 = 1u32 << 11;
pub const CCU_FEATURE_IOSC_CALIBRATION: u32 = 1u32 << 12;

/* MMC timing mode switch bit */
pub const CCU_MMC_NEW_TIMING_MODE: u32 = 1u32 << 30;

/* Some clocks need this bit to actually apply register changes */
pub const CCU_SUNXI_UPDATE_BIT: u32 = 1u32 << 27;

pub enum device_node {}

#[repr(C)]
pub struct ccu_common {
    pub base: *mut core::ffi::c_void,
    pub reg: u16,
    pub lock_reg: u16,
    pub prediv: u32,

    pub min_rate: usize,
    pub max_rate: usize,

    pub features: usize,
    pub lock: *mut spinlock_t,
    pub hw: clk_hw,
}

pub unsafe fn hw_to_ccu_common(hw: *mut clk_hw) -> *mut ccu_common {
    (hw as *mut u8).sub(core::mem::offset_of!(ccu_common, hw)) as *mut ccu_common
}

#[repr(C)]
pub struct sunxi_ccu_desc {
    pub ccu_clks: *mut *mut ccu_common,
    pub num_ccu_clks: usize,

    pub hw_clks: *mut clk_hw_onecell_data,

    pub resets: *const ccu_reset_map,
    pub num_resets: usize,
}

unsafe extern "C" {
    pub fn ccu_helper_wait_for_lock(common: *mut ccu_common, lock: u32);

    pub fn ccu_is_better_rate(
        common: *mut ccu_common,
        target_rate: usize,
        current_rate: usize,
        best_rate: usize,
    ) -> bool;
}

#[repr(C)]
pub struct ccu_pll_nb {
    pub clk_nb: notifier_block,
    pub common: *mut ccu_common,

    pub enable: u32,
    pub lock: u32,
}

pub unsafe fn to_ccu_pll_nb(nb: *mut notifier_block) -> *mut ccu_pll_nb {
    (nb as *mut u8).sub(core::mem::offset_of!(ccu_pll_nb, clk_nb)) as *mut ccu_pll_nb
}

unsafe extern "C" {
    pub fn ccu_pll_notifier_register(pll_nb: *mut ccu_pll_nb) -> core::ffi::c_int;

    pub fn devm_sunxi_ccu_probe(
        dev: *mut device,
        reg: *mut core::ffi::c_void,
        desc: *const sunxi_ccu_desc,
    ) -> core::ffi::c_int;

    pub fn of_sunxi_ccu_probe(
        node: *mut device_node,
        reg: *mut core::ffi::c_void,
        desc: *const sunxi_ccu_desc,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
