// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2009 Wolfson Microelectronics plc
 *
 * S3C64xx CPUfreq Support
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the Linux kernel headers.
#[repr(C)]
pub struct regulator { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct cpufreq_policy {
    pub cpu: u32,
    pub clk: *mut clk,
    _private: [u8; 0],
}
#[repr(C)]
pub struct cpufreq_frequency_table {
    pub flags: u32,
    pub driver_data: u32,
    pub frequency: u32,
}
#[repr(C)]
pub struct cpufreq_driver {
    pub flags: u32,
    pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> c_int>,
    pub target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> u32>,
    pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> c_int>,
    pub name: *const c_char,
}

unsafe extern "C" {
    fn clk_get(dev: *mut c_void, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn clk_set_rate(clk: *mut clk, rate: usize) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: usize) -> usize;
    fn cpufreq_generic_init(policy: *mut cpufreq_policy,
                            table: *mut cpufreq_frequency_table,
                            transition_latency: u32);
    fn cpufreq_generic_frequency_table_verify(policy: *mut cpufreq_policy) -> c_int;
    fn cpufreq_generic_get(policy: *mut cpufreq_policy) -> u32;
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> c_int;
}

const EINVAL: c_int = 22;
const CPUFREQ_TABLE_END: u32 = 0xffff_ffff;
const CPUFREQ_ENTRY_INVALID: u32 = 0xffff_fffe;
const CPUFREQ_NEED_INITIAL_FREQ_CHECK: u32 = 1 << 4;

#[cfg(feature = "CONFIG_REGULATOR")]
unsafe extern "C" {
    fn regulator_count_voltages(regulator: *mut regulator) -> c_int;
    fn regulator_list_voltage(regulator: *mut regulator, selector: c_int) -> c_int;
    fn regulator_set_voltage(regulator: *mut regulator, min_uV: c_int, max_uV: c_int) -> c_int;
    fn regulator_get(dev: *mut c_void, id: *const c_char) -> *mut regulator;
}

static mut VDDARM: *mut regulator = core::ptr::null_mut();
static mut REGULATOR_LATENCY: usize = 0;

#[repr(C)]
struct s3c64xx_dvfs {
    vddarm_min: u32,
    vddarm_max: u32,
}

#[cfg(feature = "CONFIG_REGULATOR")]
static mut S3C64XX_DVFS_TABLE: [s3c64xx_dvfs; 5] = [
    s3c64xx_dvfs { vddarm_min: 1000000, vddarm_max: 1150000 },
    s3c64xx_dvfs { vddarm_min: 1050000, vddarm_max: 1150000 },
    s3c64xx_dvfs { vddarm_min: 1100000, vddarm_max: 1150000 },
    s3c64xx_dvfs { vddarm_min: 1200000, vddarm_max: 1350000 },
    s3c64xx_dvfs { vddarm_min: 1300000, vddarm_max: 1350000 },
];

static mut S3C64XX_FREQ_TABLE: [cpufreq_frequency_table; 13] = [
    cpufreq_frequency_table { flags: 0, driver_data: 0, frequency: 66000 },
    cpufreq_frequency_table { flags: 0, driver_data: 0, frequency: 100000 },
    cpufreq_frequency_table { flags: 0, driver_data: 0, frequency: 133000 },
    cpufreq_frequency_table { flags: 0, driver_data: 1, frequency: 200000 },
    cpufreq_frequency_table { flags: 0, driver_data: 1, frequency: 222000 },
    cpufreq_frequency_table { flags: 0, driver_data: 1, frequency: 266000 },
    cpufreq_frequency_table { flags: 0, driver_data: 2, frequency: 333000 },
    cpufreq_frequency_table { flags: 0, driver_data: 2, frequency: 400000 },
    cpufreq_frequency_table { flags: 0, driver_data: 2, frequency: 532000 },
    cpufreq_frequency_table { flags: 0, driver_data: 2, frequency: 533000 },
    cpufreq_frequency_table { flags: 0, driver_data: 3, frequency: 667000 },
    cpufreq_frequency_table { flags: 0, driver_data: 4, frequency: 800000 },
    cpufreq_frequency_table { flags: 0, driver_data: 0, frequency: CPUFREQ_TABLE_END },
];

unsafe fn s3c64xx_cpufreq_set_target(policy: *mut cpufreq_policy, index: u32) -> c_int {
    let new_freq = S3C64XX_FREQ_TABLE[index as usize].frequency;
    let mut ret: c_int;
    #[cfg(feature = "CONFIG_REGULATOR")]
    let old_freq = clk_get_rate((*policy).clk) / 1000;
    #[cfg(feature = "CONFIG_REGULATOR")]
    let dvfs = &S3C64XX_DVFS_TABLE[S3C64XX_FREQ_TABLE[index as usize].driver_data as usize];

    #[cfg(feature = "CONFIG_REGULATOR")]
    if !VDDARM.is_null() && (new_freq as usize) > old_freq {
        ret = regulator_set_voltage(VDDARM, dvfs.vddarm_min as c_int, dvfs.vddarm_max as c_int);
        if ret != 0 { return ret; }
    }
    ret = clk_set_rate((*policy).clk, (new_freq as usize) * 1000);
    if ret < 0 { return ret; }
    #[cfg(feature = "CONFIG_REGULATOR")]
    if !VDDARM.is_null() && (new_freq as usize) < old_freq {
        ret = regulator_set_voltage(VDDARM, dvfs.vddarm_min as c_int, dvfs.vddarm_max as c_int);
        if ret != 0 {
            if clk_set_rate((*policy).clk, old_freq * 1000) < 0 {}
            return ret;
        }
    }
    0
}

unsafe fn s3c64xx_cpufreq_driver_init(policy: *mut cpufreq_policy) -> c_int {
    if (*policy).cpu != 0 { return -EINVAL; }
    (*policy).clk = clk_get(core::ptr::null_mut(), b"armclk\0".as_ptr() as *const c_char);
    if (*policy).clk.is_null() { return -1; }
    #[cfg(feature = "CONFIG_REGULATOR")]
    {
        VDDARM = regulator_get(core::ptr::null_mut(), b"vddarm\0".as_ptr() as *const c_char);
        if !VDDARM.is_null() {
            REGULATOR_LATENCY = 1 * 1000 * 1000;
        }
    }
    let mut i = 0usize;
    while i < S3C64XX_FREQ_TABLE.len() {
        let freq = &mut S3C64XX_FREQ_TABLE[i];
        if freq.frequency == CPUFREQ_TABLE_END { break; }
        let r = clk_round_rate((*policy).clk, (freq.frequency as usize) * 1000) / 1000;
        if r != freq.frequency as usize { freq.frequency = CPUFREQ_ENTRY_INVALID; }
        #[cfg(feature = "CONFIG_REGULATOR")]
        if VDDARM.is_null() && (freq.frequency as usize) > clk_get_rate((*policy).clk) / 1000 {
            freq.frequency = CPUFREQ_ENTRY_INVALID;
        }
        i += 1;
    }
    cpufreq_generic_init(policy, S3C64XX_FREQ_TABLE.as_mut_ptr(), (500 * 1000 + REGULATOR_LATENCY) as u32);
    0
}

static mut S3C64XX_CPUFREQ_DRIVER: cpufreq_driver = cpufreq_driver {
    flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(s3c64xx_cpufreq_set_target),
    get: Some(cpufreq_generic_get),
    init: Some(s3c64xx_cpufreq_driver_init),
    name: b"s3c\0".as_ptr() as *const c_char,
};

unsafe fn s3c64xx_cpufreq_init() -> c_int {
    cpufreq_register_driver(&mut S3C64XX_CPUFREQ_DRIVER)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
