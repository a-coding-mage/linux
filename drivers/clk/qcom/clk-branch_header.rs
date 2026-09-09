/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2013, The Linux Foundation. All rights reserved. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here but not implemented in this header translation.

#[repr(C)]
pub struct clk_branch {
    pub hwcg_reg: u32,
    pub halt_reg: u32,
    pub hwcg_bit: u8,
    pub halt_bit: u8,
    pub halt_check: u8,
    pub clkr: clk_regmap,
}

pub const BRANCH_VOTED: u8 = 1u8 << 7; // Delay on disable
pub const BRANCH_HALT: u8 = 0; // pol: 1 = halt
pub const BRANCH_HALT_VOTED: u8 = BRANCH_HALT | BRANCH_VOTED;
pub const BRANCH_HALT_ENABLE: u8 = 1; // pol: 0 = halt
pub const BRANCH_HALT_ENABLE_VOTED: u8 = BRANCH_HALT_ENABLE | BRANCH_VOTED;
pub const BRANCH_HALT_DELAY: u8 = 2; // No bit to check; just delay
pub const BRANCH_HALT_SKIP: u8 = 3; // Don't check halt bit

#[repr(C)]
pub struct clk_mem_branch {
    pub mem_enable_reg: u32,
    pub mem_ack_reg: u32,
    pub mem_enable_ack_mask: u32,
    pub mem_enable_mask: u32,
    pub mem_enable_invert: bool,
    pub branch: clk_branch,
}

// Branch clock common bits for HLOS-owned clocks
pub const CBCR_CLK_OFF: u32 = 1u32 << 31;
pub const CBCR_NOC_FSM_STATUS: u32 = 0x7u32 << 28;
pub const FSM_STATUS_ON: u32 = 1u32 << 1;
pub const CBCR_FORCE_MEM_CORE_ON: u32 = 1u32 << 14;
pub const CBCR_FORCE_MEM_PERIPH_ON: u32 = 1u32 << 13;
pub const CBCR_FORCE_MEM_PERIPH_OFF: u32 = 1u32 << 12;
pub const CBCR_WAKEUP: u32 = 0xfu32 << 8;
pub const CBCR_SLEEP: u32 = 0xfu32 << 4;
pub const CBCR_CLOCK_ENABLE: u32 = 1u32;

#[inline]
pub unsafe fn qcom_branch_set_force_mem_core(
    regmap: *mut regmap,
    clk: clk_branch,
    on: bool,
) {
    regmap_update_bits(
        regmap,
        clk.halt_reg,
        CBCR_FORCE_MEM_CORE_ON,
        if on { CBCR_FORCE_MEM_CORE_ON } else { 0 },
    );
}

#[inline]
pub unsafe fn qcom_branch_set_force_periph_on(
    regmap: *mut regmap,
    clk: clk_branch,
    on: bool,
) {
    regmap_update_bits(
        regmap,
        clk.halt_reg,
        CBCR_FORCE_MEM_PERIPH_ON,
        if on { CBCR_FORCE_MEM_PERIPH_ON } else { 0 },
    );
}

#[inline]
pub unsafe fn qcom_branch_set_force_periph_off(
    regmap: *mut regmap,
    clk: clk_branch,
    on: bool,
) {
    regmap_update_bits(
        regmap,
        clk.halt_reg,
        CBCR_FORCE_MEM_PERIPH_OFF,
        if on { CBCR_FORCE_MEM_PERIPH_OFF } else { 0 },
    );
}

#[inline]
pub unsafe fn qcom_branch_set_wakeup(regmap: *mut regmap, clk: clk_branch, val: u32) {
    regmap_update_bits(regmap, clk.halt_reg, CBCR_WAKEUP, (val << 8) & CBCR_WAKEUP);
}

#[inline]
pub unsafe fn qcom_branch_set_sleep(regmap: *mut regmap, clk: clk_branch, val: u32) {
    regmap_update_bits(regmap, clk.halt_reg, CBCR_SLEEP, (val << 4) & CBCR_SLEEP);
}

#[inline]
pub unsafe fn qcom_branch_set_clk_en(regmap: *mut regmap, cbcr: u32) {
    regmap_update_bits(regmap, cbcr, CBCR_CLOCK_ENABLE, CBCR_CLOCK_ENABLE);
}

extern "C" {
    pub static clk_branch_ops: clk_ops;
    pub static clk_branch2_ops: clk_ops;
    pub static clk_branch_simple_ops: clk_ops;
    pub static clk_branch2_aon_ops: clk_ops;
    pub static clk_branch2_mem_ops: clk_ops;
    pub static clk_branch2_prepare_ops: clk_ops;
}

#[macro_export]
macro_rules! to_clk_branch {
    ($hw:expr) => {
        container_of!(to_clk_regmap!($hw), clk_branch, clkr)
    };
}

#[macro_export]
macro_rules! to_clk_mem_branch {
    ($hw:expr) => {
        container_of!(to_clk_branch!($hw), clk_mem_branch, branch)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
