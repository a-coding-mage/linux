/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 Amlogic, Inc. All rights reserved.
 */

// C header guard: __MESON_DDR_PMU_H__

pub const MAX_CHANNEL_NUM: usize = 8;

pub const ALL_CHAN_COUNTER_ID: i32 = 0;
pub const CHAN1_COUNTER_ID: i32 = 1;
pub const CHAN2_COUNTER_ID: i32 = 2;
pub const CHAN3_COUNTER_ID: i32 = 3;
pub const CHAN4_COUNTER_ID: i32 = 4;
pub const CHAN5_COUNTER_ID: i32 = 5;
pub const CHAN6_COUNTER_ID: i32 = 6;
pub const CHAN7_COUNTER_ID: i32 = 7;
pub const CHAN8_COUNTER_ID: i32 = 8;
pub const COUNTER_MAX_ID: i32 = 9;

#[repr(C)]
pub union dmc_counter__bindgen_ty_1 {
    pub all_req: u64,
    pub all_idle_cnt: u64,
    pub all_16bit_cnt: u64,
}

#[repr(C)]
pub struct dmc_counter {
    /// The count of all requests come in/out ddr controller
    pub all_cnt: u64,
    pub all_req: dmc_counter__bindgen_ty_1,
    /// To save a DMC bandwidth-monitor channel counter
    pub channel_cnt: [u64; MAX_CHANNEL_NUM],
}

#[repr(C)]
pub struct dmc_hw_info {
    pub enable: Option<unsafe extern "C" fn(info: *mut dmc_info)>,
    pub disable: Option<unsafe extern "C" fn(info: *mut dmc_info)>,
    /// Bind an axi line to a bandwidth-monitor channel
    pub set_axi_filter:
        Option<unsafe extern "C" fn(info: *mut dmc_info, axi_id: i32, chann: i32)>,
    pub irq_handler: Option<
        unsafe extern "C" fn(info: *mut dmc_info, counter: *mut dmc_counter) -> i32,
    >,
    pub get_counters:
        Option<unsafe extern "C" fn(info: *mut dmc_info, counter: *mut dmc_counter)>,
    /// The number of dmc controller
    pub dmc_nr: i32,
    /// The number of dmc bandwidth monitor channels
    pub chann_nr: i32,
    pub fmt_attr: *mut *mut attribute,
    pub capability: [u64; 2],
}

#[repr(C)]
pub struct dmc_info {
    pub hw_info: *const dmc_hw_info,
    pub ddr_reg: [*mut core::ffi::c_void; 4],
    /// Timer value in TIMER register
    pub timer_value: core::ffi::c_ulong,
    pub pll_reg: *mut core::ffi::c_void,
    /// irq vector number
    pub irq_num: i32,
}

#[repr(C)]
pub struct platform_device;

#[repr(C)]
pub struct attribute;

unsafe extern "C" {
    pub fn meson_ddr_pmu_create(pdev: *mut platform_device) -> i32;
    pub fn meson_ddr_pmu_remove(pdev: *mut platform_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
