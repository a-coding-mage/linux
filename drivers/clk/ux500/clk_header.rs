/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Clocks for ux500 platforms
 *
 * Copyright (C) 2012 ST-Ericsson SA
 * Author: Ulf Hansson <ulf.hansson@linaro.org>
 */

use core::ffi::{c_char, c_int, c_ulong};

// Supplied by the Linux type and device dependencies.
pub type resource_size_t = u64;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn clk_reg_prcc_pclk(
        name: *const c_char,
        parent_name: *const c_char,
        phy_base: resource_size_t,
        cg_sel: u32,
        flags: c_ulong,
    ) -> *mut clk;

    pub fn clk_reg_prcc_kclk(
        name: *const c_char,
        parent_name: *const c_char,
        phy_base: resource_size_t,
        cg_sel: u32,
        flags: c_ulong,
    ) -> *mut clk;

    pub fn clk_reg_prcmu_scalable(
        name: *const c_char,
        parent_name: *const c_char,
        cg_sel: u8,
        rate: c_ulong,
        flags: c_ulong,
    ) -> *mut clk_hw;

    pub fn clk_reg_prcmu_gate(
        name: *const c_char,
        parent_name: *const c_char,
        cg_sel: u8,
        flags: c_ulong,
    ) -> *mut clk_hw;

    pub fn clk_reg_prcmu_scalable_rate(
        name: *const c_char,
        parent_name: *const c_char,
        cg_sel: u8,
        rate: c_ulong,
        flags: c_ulong,
    ) -> *mut clk_hw;

    pub fn clk_reg_prcmu_rate(
        name: *const c_char,
        parent_name: *const c_char,
        cg_sel: u8,
        flags: c_ulong,
    ) -> *mut clk_hw;

    pub fn clk_reg_prcmu_opp_gate(
        name: *const c_char,
        parent_name: *const c_char,
        cg_sel: u8,
        flags: c_ulong,
    ) -> *mut clk_hw;

    pub fn clk_reg_prcmu_opp_volt_scalable(
        name: *const c_char,
        parent_name: *const c_char,
        cg_sel: u8,
        rate: c_ulong,
        flags: c_ulong,
    ) -> *mut clk_hw;

    pub fn clk_reg_prcmu_clkout(
        name: *const c_char,
        parent_names: *const *const c_char,
        num_parents: c_int,
        source: u8,
        divider: u8,
    ) -> *mut clk_hw;

    pub fn clk_reg_sysctrl_gate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        reg_sel: u16,
        reg_mask: u8,
        reg_bits: u8,
        enable_delay_us: c_ulong,
        flags: c_ulong,
    ) -> *mut clk;

    pub fn clk_reg_sysctrl_gate_fixed_rate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        reg_sel: u16,
        reg_mask: u8,
        reg_bits: u8,
        rate: c_ulong,
        enable_delay_us: c_ulong,
        flags: c_ulong,
    ) -> *mut clk;

    pub fn clk_reg_sysctrl_set_parent(
        dev: *mut device,
        name: *const c_char,
        parent_names: *mut *const c_char,
        num_parents: u8,
        reg_sel: *mut u16,
        reg_mask: *mut u8,
        reg_bits: *mut u8,
        flags: c_ulong,
    ) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
