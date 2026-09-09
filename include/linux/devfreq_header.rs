/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * devfreq: Generic Dynamic Voltage and Frequency Scaling (DVFS) Framework
 *          for Non-CPU Devices.
 *
 * Copyright (C) 2011 Samsung Electronics
 *      MyungJoo Ham <myungjoo.ham@samsung.com>
 */

/* Dependencies supplied by the corresponding Linux headers. */

pub const DEVFREQ_GOV_SIMPLE_ONDEMAND: &str = "simple_ondemand";
pub const DEVFREQ_GOV_PERFORMANCE: &str = "performance";
pub const DEVFREQ_GOV_POWERSAVE: &str = "powersave";
pub const DEVFREQ_GOV_USERSPACE: &str = "userspace";
pub const DEVFREQ_GOV_PASSIVE: &str = "passive";

pub const DEVFREQ_TRANSITION_NOTIFIER: u32 = 0;
pub const DEVFREQ_PRECHANGE: u32 = 0;
pub const DEVFREQ_POSTCHANGE: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum devfreq_timer {
    DEVFREQ_TIMER_DEFERRABLE = 0,
    DEVFREQ_TIMER_DELAYED,
    DEVFREQ_TIMER_NUM,
}

#[repr(C)] pub struct devfreq_governor { _private: [u8; 0] }
#[repr(C)] pub struct devfreq_cpu_data { _private: [u8; 0] }
#[repr(C)] pub struct thermal_cooling_device { _private: [u8; 0] }

#[repr(C)]
pub struct devfreq_dev_status {
    pub total_time: ::core::ffi::c_ulong,
    pub busy_time: ::core::ffi::c_ulong,
    pub current_frequency: ::core::ffi::c_ulong,
    pub private_data: *mut ::core::ffi::c_void,
}

pub const DEVFREQ_FLAG_LEAST_UPPER_BOUND: u32 = 0x1;

#[repr(C)]
pub struct devfreq_dev_profile {
    pub initial_freq: ::core::ffi::c_ulong,
    pub polling_ms: u32,
    pub timer: devfreq_timer,
    pub target: Option<unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_ulong, u32) -> i32>,
    pub get_dev_status: Option<unsafe extern "C" fn(*mut device, *mut devfreq_dev_status) -> i32>,
    pub get_cur_freq: Option<unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_ulong) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut device)>,
    pub freq_table: *mut ::core::ffi::c_ulong,
    pub max_state: u32,
    pub is_cooling_device: bool,
    pub dev_groups: *const *const attribute_group,
}

#[repr(C)]
pub struct devfreq_stats {
    pub total_trans: u32,
    pub trans_table: *mut u32,
    pub time_in_state: *mut u64,
    pub last_update: u64,
}

#[repr(C)]
pub struct devfreq {
    pub node: list_head,
    pub lock: mutex,
    pub dev: device,
    pub profile: *mut devfreq_dev_profile,
    pub governor: *const devfreq_governor,
    pub opp_table: *mut opp_table,
    pub nb: notifier_block,
    pub work: delayed_work,
    pub freq_table: *mut ::core::ffi::c_ulong,
    pub max_state: u32,
    pub previous_freq: ::core::ffi::c_ulong,
    pub last_status: devfreq_dev_status,
    pub data: *mut ::core::ffi::c_void,
    pub governor_data: *mut ::core::ffi::c_void,
    pub user_min_freq_req: dev_pm_qos_request,
    pub user_max_freq_req: dev_pm_qos_request,
    pub scaling_min_freq: ::core::ffi::c_ulong,
    pub scaling_max_freq: ::core::ffi::c_ulong,
    pub stop_polling: bool,
    pub suspend_freq: ::core::ffi::c_ulong,
    pub resume_freq: ::core::ffi::c_ulong,
    pub suspend_count: atomic_t,
    pub stats: devfreq_stats,
    pub transition_notifier_list: srcu_notifier_head,
    pub cdev: *mut thermal_cooling_device,
    pub nb_min: notifier_block,
    pub nb_max: notifier_block,
}

#[repr(C)]
pub struct devfreq_freqs {
    pub old: ::core::ffi::c_ulong,
    pub new: ::core::ffi::c_ulong,
}

#[cfg(CONFIG_PM_DEVFREQ)]
extern "C" {
    pub fn devfreq_add_device(dev: *mut device, profile: *mut devfreq_dev_profile, governor_name: *const ::core::ffi::c_char, data: *mut ::core::ffi::c_void) -> *mut devfreq;
    pub fn devfreq_remove_device(devfreq: *mut devfreq) -> i32;
    pub fn devm_devfreq_add_device(dev: *mut device, profile: *mut devfreq_dev_profile, governor_name: *const ::core::ffi::c_char, data: *mut ::core::ffi::c_void) -> *mut devfreq;
    pub fn devm_devfreq_remove_device(dev: *mut device, devfreq: *mut devfreq);
    pub fn devfreq_suspend_device(devfreq: *mut devfreq) -> i32;
    pub fn devfreq_resume_device(devfreq: *mut devfreq) -> i32;
    pub fn devfreq_suspend();
    pub fn devfreq_resume();
    pub fn update_devfreq(devfreq: *mut devfreq) -> i32;
    pub fn devfreq_recommended_opp(dev: *mut device, freq: *mut ::core::ffi::c_ulong, flags: u32) -> *mut dev_pm_opp;
    pub fn devfreq_register_opp_notifier(dev: *mut device, devfreq: *mut devfreq) -> i32;
    pub fn devfreq_unregister_opp_notifier(dev: *mut device, devfreq: *mut devfreq) -> i32;
    pub fn devm_devfreq_register_opp_notifier(dev: *mut device, devfreq: *mut devfreq) -> i32;
    pub fn devm_devfreq_unregister_opp_notifier(dev: *mut device, devfreq: *mut devfreq);
    pub fn devfreq_register_notifier(devfreq: *mut devfreq, nb: *mut notifier_block, list: u32) -> i32;
    pub fn devfreq_unregister_notifier(devfreq: *mut devfreq, nb: *mut notifier_block, list: u32) -> i32;
    pub fn devm_devfreq_register_notifier(dev: *mut device, devfreq: *mut devfreq, nb: *mut notifier_block, list: u32) -> i32;
    pub fn devm_devfreq_unregister_notifier(dev: *mut device, devfreq: *mut devfreq, nb: *mut notifier_block, list: u32);
    pub fn devfreq_get_devfreq_by_node(node: *mut device_node) -> *mut devfreq;
    pub fn devfreq_get_devfreq_by_phandle(dev: *mut device, phandle_name: *const ::core::ffi::c_char, index: i32) -> *mut devfreq;
}

#[repr(C)]
pub struct devfreq_simple_ondemand_data {
    pub upthreshold: u32,
    pub downdifferential: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum devfreq_parent_dev_type {
    DEVFREQ_PARENT_DEV,
    CPUFREQ_PARENT_DEV,
}

#[repr(C)]
pub struct devfreq_passive_data {
    pub parent: *mut devfreq,
    pub get_target_freq: Option<unsafe extern "C" fn(*mut devfreq, *mut ::core::ffi::c_ulong) -> i32>,
    pub parent_type: devfreq_parent_dev_type,
    pub this: *mut devfreq,
    pub nb: notifier_block,
    pub cpu_data_list: list_head,
}

/* When CONFIG_PM_DEVFREQ is disabled, these are the header's inline stubs. */
#[cfg(not(CONFIG_PM_DEVFREQ))]
pub unsafe fn devfreq_add_device(_: *mut device, _: *mut devfreq_dev_profile, _: *const ::core::ffi::c_char, _: *mut ::core::ffi::c_void) -> *mut devfreq { ERR_PTR(-ENOSYS) }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_remove_device(_: *mut devfreq) -> i32 { 0 }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devm_devfreq_add_device(_: *mut device, _: *mut devfreq_dev_profile, _: *const ::core::ffi::c_char, _: *mut ::core::ffi::c_void) -> *mut devfreq { ERR_PTR(-ENOSYS) }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devm_devfreq_remove_device(_: *mut device, _: *mut devfreq) {}
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_suspend_device(_: *mut devfreq) -> i32 { 0 }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_resume_device(_: *mut devfreq) -> i32 { 0 }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_suspend() {}
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_resume() {}
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_recommended_opp(_: *mut device, _: *mut ::core::ffi::c_ulong, _: u32) -> *mut dev_pm_opp { ERR_PTR(-EINVAL) }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_register_opp_notifier(_: *mut device, _: *mut devfreq) -> i32 { -EINVAL }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_unregister_opp_notifier(_: *mut device, _: *mut devfreq) -> i32 { -EINVAL }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devm_devfreq_register_opp_notifier(_: *mut device, _: *mut devfreq) -> i32 { -EINVAL }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devm_devfreq_unregister_opp_notifier(_: *mut device, _: *mut devfreq) {}
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_register_notifier(_: *mut devfreq, _: *mut notifier_block, _: u32) -> i32 { 0 }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_unregister_notifier(_: *mut devfreq, _: *mut notifier_block, _: u32) -> i32 { 0 }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devm_devfreq_register_notifier(_: *mut device, _: *mut devfreq, _: *mut notifier_block, _: u32) -> i32 { 0 }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devm_devfreq_unregister_notifier(_: *mut device, _: *mut devfreq, _: *mut notifier_block, _: u32) {}
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_get_devfreq_by_node(_: *mut device_node) -> *mut devfreq { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_get_devfreq_by_phandle(_: *mut device, _: *const ::core::ffi::c_char, _: i32) -> *mut devfreq { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_PM_DEVFREQ))] pub unsafe fn devfreq_update_stats(_: *mut devfreq) -> i32 { -EINVAL }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
