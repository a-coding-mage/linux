// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple SoC CPU cluster performance state driver
 *
 * Copyright The Asahi Linux Contributors
 *
 * Based on scpi-cpufreq.c
 */

// Linux kernel includes and build-time attributes are supplied by dependencies.

const APPLE_DVFS_CMD: usize = 0x20;
const APPLE_DVFS_CMD_BUSY: u64 = 1u64 << 31;
const APPLE_DVFS_CMD_SET: u64 = 1u64 << 25;
const APPLE_DVFS_CMD_PS1_S5L8960X: u64 = 0x7 << 22;
const APPLE_DVFS_CMD_PS1_S5L8960X_SHIFT: u64 = 22;
const APPLE_DVFS_CMD_PS2: u64 = 0xf << 12;
const APPLE_DVFS_CMD_PS1: u64 = 0x1f;
const APPLE_DVFS_CMD_PS1_SHIFT: u64 = 0;
const APPLE_DVFS_LAST_CHG_TIME: usize = 0x38;
const APPLE_DVFS_STATUS: usize = 0x50;
const APPLE_DVFS_STATUS_CUR_PS_S5L8960X: u64 = 0x7 << 3;
const APPLE_DVFS_STATUS_CUR_PS_SHIFT_S5L8960X: u64 = 3;
const APPLE_DVFS_STATUS_TGT_PS_S5L8960X: u64 = 0x7;
const APPLE_DVFS_STATUS_CUR_PS_T8103: u64 = 0xf << 4;
const APPLE_DVFS_STATUS_CUR_PS_SHIFT_T8103: u64 = 4;
const APPLE_DVFS_STATUS_TGT_PS_T8103: u64 = 0xf;
const APPLE_DVFS_STATUS_CUR_PS_T8112: u64 = 0x1f << 5;
const APPLE_DVFS_STATUS_CUR_PS_SHIFT_T8112: u64 = 5;
const APPLE_DVFS_STATUS_TGT_PS_T8112: u64 = 0x1f;
const APPLE_DVFS_PLL_STATUS: usize = 0xc0;
const APPLE_DVFS_PLL_FACTOR: usize = 0xc8;
const APPLE_DVFS_PLL_FACTOR_MULT: u64 = 0xffff << 16;
const APPLE_DVFS_PLL_FACTOR_DIV: u64 = 0xffff;
const APPLE_DVFS_TRANSITION_TIMEOUT: u32 = 400;

#[repr(C)]
struct apple_soc_cpufreq_info {
    has_ps2: bool,
    max_pstate: u64,
    cur_pstate_mask: u64,
    cur_pstate_shift: u64,
    ps1_mask: u64,
    ps1_shift: u64,
}

#[repr(C)]
struct apple_cpu_priv {
    cpu_dev: *mut device,
    reg_base: *mut core::ffi::c_void,
    info: *const apple_soc_cpufreq_info,
}

#[repr(C)] struct device;
#[repr(C)] struct cpufreq_policy;
#[repr(C)] struct cpufreq_frequency_table { frequency: u32, driver_data: u32 }
#[repr(C)] struct of_device_id;
#[repr(C)] struct of_phandle_args { np: *mut core::ffi::c_void }
#[repr(C)] struct cpumask;
#[repr(C)] struct cpufreq_driver;
#[repr(C)] struct dev_pm_opp;

extern "C" {
    static mut apple_soc_cpufreq_driver: cpufreq_driver;
    fn cpufreq_cpu_get_raw(cpu: u32) -> *mut cpufreq_policy;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn readq_relaxed(addr: *mut u8) -> u64;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn writeq_relaxed(value: u64, addr: *mut u8);
    fn of_perf_domain_get_sharing_cpumask(cpu: u32, name: *const u8, cells: *const u8, mask: *mut cpumask, args: *mut of_phandle_args) -> i32;
    fn of_match_node(ids: *const of_device_id, node: *mut core::ffi::c_void) -> *const of_device_id;
    fn of_node_put(node: *mut core::ffi::c_void);
    fn of_iomap(node: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn get_cpu_device(cpu: u32) -> *mut device;
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dev_pm_opp_of_cpumask_add_table(mask: *mut cpumask) -> i32;
    fn dev_pm_opp_get_opp_count(dev: *mut device) -> i32;
    fn dev_pm_opp_init_cpufreq_table(dev: *mut device, table: *mut *mut cpufreq_frequency_table) -> i32;
    fn dev_pm_opp_find_freq_floor(dev: *mut device, rate: *mut u64) -> *mut dev_pm_opp;
    fn dev_pm_opp_get_level(opp: *mut dev_pm_opp) -> u32;
    fn dev_pm_opp_put(opp: *mut dev_pm_opp);
    fn dev_pm_opp_get_max_transition_latency(dev: *mut device) -> u32;
    fn dev_pm_opp_free_cpufreq_table(dev: *mut device, table: *mut *mut cpufreq_frequency_table);
    fn dev_pm_opp_of_cpumask_remove_table(mask: *mut cpumask);
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
    fn of_machine_is_compatible(compat: *const u8) -> bool;
}

static soc_s5l8960x_info: apple_soc_cpufreq_info = apple_soc_cpufreq_info { has_ps2: false, max_pstate: 7, cur_pstate_mask: APPLE_DVFS_STATUS_CUR_PS_S5L8960X, cur_pstate_shift: 3, ps1_mask: APPLE_DVFS_CMD_PS1_S5L8960X, ps1_shift: 22 };
static soc_t8103_info: apple_soc_cpufreq_info = apple_soc_cpufreq_info { has_ps2: true, max_pstate: 15, cur_pstate_mask: APPLE_DVFS_STATUS_CUR_PS_T8103, cur_pstate_shift: 4, ps1_mask: APPLE_DVFS_CMD_PS1, ps1_shift: 0 };
static soc_t8112_info: apple_soc_cpufreq_info = apple_soc_cpufreq_info { has_ps2: false, max_pstate: 31, cur_pstate_mask: APPLE_DVFS_STATUS_CUR_PS_T8112, cur_pstate_shift: 5, ps1_mask: APPLE_DVFS_CMD_PS1, ps1_shift: 0 };
static soc_default_info: apple_soc_cpufreq_info = apple_soc_cpufreq_info { has_ps2: false, max_pstate: 15, cur_pstate_mask: 0, cur_pstate_shift: 0, ps1_mask: APPLE_DVFS_CMD_PS1, ps1_shift: 0 };

unsafe fn apple_soc_cpufreq_get_rate(_cpu: u32) -> u32 { 0 }
unsafe fn apple_soc_cpufreq_set_target(_policy: *mut cpufreq_policy, _index: u32) -> i32 { 0 }
unsafe fn apple_soc_cpufreq_fast_switch(_policy: *mut cpufreq_policy, _target_freq: u32) -> u32 { 0 }
unsafe fn apple_soc_cpufreq_find_cluster(_policy: *mut cpufreq_policy, _reg_base: *mut *mut core::ffi::c_void, _info: *mut *const apple_soc_cpufreq_info) -> i32 { 0 }
unsafe fn apple_soc_cpufreq_init(_policy: *mut cpufreq_policy) -> i32 { 0 }
unsafe fn apple_soc_cpufreq_exit(_policy: *mut cpufreq_policy) {}

// Module registration and metadata are supplied by the kernel module integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
