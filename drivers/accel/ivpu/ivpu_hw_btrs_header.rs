/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Dependencies supplied by the corresponding C headers:
// ivpu_drv.h, ivpu_hw_37xx_reg.h, ivpu_hw_40xx_reg.h, ivpu_hw_reg_io.h

pub const PLL_PROFILING_FREQ_DEFAULT: u32 = 38_400_000;
pub const PLL_PROFILING_FREQ_HIGH: u32 = 400_000_000;

pub const DCT_DEFAULT_ACTIVE_PERCENT: u32 = 30u32;
pub const DCT_PERIOD_US: u32 = 35_300u32;

pub struct ivpu_device;

unsafe extern "C" {
    pub fn ivpu_hw_btrs_info_init(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_freq_ratios_init(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_irqs_clear_with_0_mtl(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_wp_drive(vdev: *mut ivpu_device, enable: bool) -> i32;
    pub fn ivpu_hw_btrs_wait_for_clock_res_own_ack(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_d0i3_enable(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_d0i3_disable(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_set_port_arbitration_weights_lnl(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_is_idle(vdev: *mut ivpu_device) -> bool;
    pub fn ivpu_hw_btrs_wait_for_idle(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_ip_reset(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_profiling_freq_reg_set_lnl(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_ats_print_lnl(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_clock_relinquish_disable_lnl(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_pll_ratio_to_mhz(vdev: *mut ivpu_device, pll_ratio: u8) -> u32;
    pub fn ivpu_hw_btrs_pll_ratio_to_hz(vdev: *mut ivpu_device, pll_ratio: u8) -> u32;
    pub fn ivpu_hw_btrs_current_freq_get(vdev: *mut ivpu_device) -> u32;
    pub fn ivpu_hw_btrs_cfg_min_freq_set(vdev: *mut ivpu_device, freq_mhz: u32) -> i32;
    pub fn ivpu_hw_btrs_cfg_max_freq_set(vdev: *mut ivpu_device, freq_mhz: u32) -> i32;
    pub fn ivpu_hw_btrs_cfg_freq_init(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_hw_btrs_irq_handler_mtl(vdev: *mut ivpu_device, irq: i32) -> bool;
    pub fn ivpu_hw_btrs_irq_handler_lnl(vdev: *mut ivpu_device, irq: i32) -> bool;
    pub fn ivpu_hw_btrs_dct_get_request(vdev: *mut ivpu_device, enable: *mut bool) -> i32;
    pub fn ivpu_hw_btrs_dct_set_status(vdev: *mut ivpu_device, enable: bool, active_percent: u8);
    pub fn ivpu_hw_btrs_telemetry_offset_get(vdev: *mut ivpu_device) -> u32;
    pub fn ivpu_hw_btrs_telemetry_size_get(vdev: *mut ivpu_device) -> u32;
    pub fn ivpu_hw_btrs_telemetry_enable_get(vdev: *mut ivpu_device) -> u32;
    pub fn ivpu_hw_btrs_global_int_enable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_global_int_disable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_irq_enable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_irq_disable(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_diagnose_failure(vdev: *mut ivpu_device);
    pub fn ivpu_hw_btrs_platform_read(vdev: *mut ivpu_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
