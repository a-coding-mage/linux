/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// Dependency supplied by the Linux clkdev translation.

/* clock speeds */
pub const CLOCK_33M: u64 = 33333333;
pub const CLOCK_60M: u64 = 60000000;
pub const CLOCK_62_5M: u64 = 62500000;
pub const CLOCK_83M: u64 = 83333333;
pub const CLOCK_83_5M: u64 = 83500000;
pub const CLOCK_98_304M: u64 = 98304000;
pub const CLOCK_100M: u64 = 100000000;
pub const CLOCK_111M: u64 = 111111111;
pub const CLOCK_125M: u64 = 125000000;
pub const CLOCK_133M: u64 = 133333333;
pub const CLOCK_150M: u64 = 150000000;
pub const CLOCK_166M: u64 = 166666666;
pub const CLOCK_167M: u64 = 166666667;
pub const CLOCK_196_608M: u64 = 196608000;
pub const CLOCK_200M: u64 = 200000000;
pub const CLOCK_222M: u64 = 222000000;
pub const CLOCK_240M: u64 = 240000000;
pub const CLOCK_250M: u64 = 250000000;
pub const CLOCK_266M: u64 = 266666666;
pub const CLOCK_288M: u64 = 288888888;
pub const CLOCK_300M: u64 = 300000000;
pub const CLOCK_333M: u64 = 333333333;
pub const CLOCK_360M: u64 = 360000000;
pub const CLOCK_393M: u64 = 393215332;
pub const CLOCK_400M: u64 = 400000000;
pub const CLOCK_432M: u64 = 432000000;
pub const CLOCK_450M: u64 = 450000000;
pub const CLOCK_500M: u64 = 500000000;
pub const CLOCK_600M: u64 = 600000000;
pub const CLOCK_666M: u64 = 666666666;
pub const CLOCK_720M: u64 = 720000000;

/* clock out speeds */
pub const CLOCK_32_768K: u64 = 32768;
pub const CLOCK_1_536M: u64 = 1536000;
pub const CLOCK_2_5M: u64 = 2500000;
pub const CLOCK_12M: u64 = 12000000;
pub const CLOCK_24M: u64 = 24000000;
pub const CLOCK_25M: u64 = 25000000;
pub const CLOCK_30M: u64 = 30000000;
pub const CLOCK_40M: u64 = 40000000;
pub const CLOCK_48M: u64 = 48000000;
pub const CLOCK_50M: u64 = 50000000;
pub const CLOCK_60M: u64 = 60000000;

#[repr(C)]
pub struct clk {
    pub cl: crate::clkdev::clk_lookup,
    pub rate: u64,
    pub rates: *mut u64,
    pub module: u32,
    pub bits: u32,
    pub get_rate: Option<unsafe extern "C" fn() -> u64>,
    pub enable: Option<unsafe extern "C" fn(clk: *mut clk) -> i32>,
    pub disable: Option<unsafe extern "C" fn(clk: *mut clk)>,
    pub activate: Option<unsafe extern "C" fn(clk: *mut clk) -> i32>,
    pub deactivate: Option<unsafe extern "C" fn(clk: *mut clk)>,
    pub reboot: Option<unsafe extern "C" fn(clk: *mut clk)>,
}

unsafe extern "C" {
    pub fn clkdev_add_static(cpu: u64, fpi: u64, io: u64, ppe: u64);

    pub fn ltq_danube_cpu_hz() -> u64;
    pub fn ltq_danube_fpi_hz() -> u64;
    pub fn ltq_danube_pp32_hz() -> u64;

    pub fn ltq_ar9_cpu_hz() -> u64;
    pub fn ltq_ar9_fpi_hz() -> u64;

    pub fn ltq_vr9_cpu_hz() -> u64;
    pub fn ltq_vr9_fpi_hz() -> u64;
    pub fn ltq_vr9_pp32_hz() -> u64;

    pub fn ltq_ar10_cpu_hz() -> u64;
    pub fn ltq_ar10_fpi_hz() -> u64;
    pub fn ltq_ar10_pp32_hz() -> u64;

    pub fn ltq_grx390_cpu_hz() -> u64;
    pub fn ltq_grx390_fpi_hz() -> u64;
    pub fn ltq_grx390_pp32_hz() -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
