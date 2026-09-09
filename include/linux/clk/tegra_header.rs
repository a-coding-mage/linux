/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2012-2020, NVIDIA CORPORATION. All rights reserved. */

// Translated from linux/clk/tegra.h. C configuration conditions are preserved
// as comments where their build-time values are supplied externally.

#[repr(C)]
pub struct tegra_cpu_car_ops {
    pub wait_for_reset: Option<unsafe extern "C" fn(cpu: u32)>,
    pub put_in_reset: Option<unsafe extern "C" fn(cpu: u32)>,
    pub out_of_reset: Option<unsafe extern "C" fn(cpu: u32)>,
    pub enable_clock: Option<unsafe extern "C" fn(cpu: u32)>,
    pub disable_clock: Option<unsafe extern "C" fn(cpu: u32)>,
    // CONFIG_PM_SLEEP
    pub rail_off_ready: Option<unsafe extern "C" fn() -> bool>,
    pub suspend: Option<unsafe extern "C" fn()>,
    pub resume: Option<unsafe extern "C" fn()>,
}

// CONFIG_ARCH_TEGRA
extern "C" {
    pub static mut tegra_cpu_car_ops: *mut tegra_cpu_car_ops;
}

pub unsafe fn tegra_wait_cpu_in_reset(cpu: u32) {
    if (*tegra_cpu_car_ops).wait_for_reset.is_none() { return; }
    ((*tegra_cpu_car_ops).wait_for_reset.unwrap())(cpu);
}
pub unsafe fn tegra_put_cpu_in_reset(cpu: u32) {
    if (*tegra_cpu_car_ops).put_in_reset.is_none() { return; }
    ((*tegra_cpu_car_ops).put_in_reset.unwrap())(cpu);
}
pub unsafe fn tegra_cpu_out_of_reset(cpu: u32) {
    if (*tegra_cpu_car_ops).out_of_reset.is_none() { return; }
    ((*tegra_cpu_car_ops).out_of_reset.unwrap())(cpu);
}
pub unsafe fn tegra_enable_cpu_clock(cpu: u32) {
    if (*tegra_cpu_car_ops).enable_clock.is_none() { return; }
    ((*tegra_cpu_car_ops).enable_clock.unwrap())(cpu);
}
pub unsafe fn tegra_disable_cpu_clock(cpu: u32) {
    if (*tegra_cpu_car_ops).disable_clock.is_none() { return; }
    ((*tegra_cpu_car_ops).disable_clock.unwrap())(cpu);
}

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct tegra_emc { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }

pub type tegra20_clk_emc_round_cb = unsafe extern "C" fn(u64, u64, u64, *mut core::ffi::c_void) -> i64;
pub type tegra124_emc_prepare_timing_change_cb = unsafe extern "C" fn(*mut tegra_emc, u64) -> i32;
pub type tegra124_emc_complete_timing_change_cb = unsafe extern "C" fn(*mut tegra_emc, u64);

#[repr(C)]
pub struct tegra210_clk_emc_config {
    pub rate: u64,
    pub same_freq: bool,
    pub value: u32,
    pub parent_rate: u64,
    pub parent: u8,
}

#[repr(C)]
pub struct tegra210_clk_emc_provider {
    pub owner: *mut module,
    pub dev: *mut device,
    pub configs: *mut tegra210_clk_emc_config,
    pub num_configs: u32,
    pub set_rate: Option<unsafe extern "C" fn(*mut device, *const tegra210_clk_emc_config) -> i32>,
}

// CONFIG_ARCH_TEGRA_2x_SOC || CONFIG_ARCH_TEGRA_3x_SOC
extern "C" {
    pub fn tegra20_clk_set_emc_round_callback(round_cb: Option<tegra20_clk_emc_round_cb>, cb_arg: *mut core::ffi::c_void);
    pub fn tegra20_clk_prepare_emc_mc_same_freq(emc_clk: *mut clk, same: bool) -> i32;
}

// CONFIG_TEGRA124_CLK_EMC
extern "C" {
    pub fn tegra124_clk_set_emc_callbacks(prep_cb: Option<tegra124_emc_prepare_timing_change_cb>, complete_cb: Option<tegra124_emc_complete_timing_change_cb>);
}

// CONFIG_ARCH_TEGRA_210_SOC
extern "C" {
    pub fn tegra210_plle_hw_sequence_start() -> i32;
    pub fn tegra210_plle_hw_sequence_is_enabled() -> bool;
    pub fn tegra210_xusb_pll_hw_control_enable();
    pub fn tegra210_xusb_pll_hw_sequence_start();
    pub fn tegra210_sata_pll_hw_control_enable();
    pub fn tegra210_sata_pll_hw_sequence_start();
    pub fn tegra210_set_sata_pll_seq_sw(state: bool);
    pub fn tegra210_put_utmipll_in_iddq();
    pub fn tegra210_put_utmipll_out_iddq();
    pub fn tegra210_clk_handle_mbist_war(id: u32) -> i32;
    pub fn tegra210_clk_emc_dll_enable(flag: bool);
    pub fn tegra210_clk_emc_dll_update_setting(emc_dll_src_value: u32);
    pub fn tegra210_clk_emc_update_setting(emc_src_value: u32);
    pub fn tegra210_clk_emc_attach(clk: *mut clk, provider: *mut tegra210_clk_emc_provider) -> i32;
    pub fn tegra210_clk_emc_detach(clk: *mut clk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
