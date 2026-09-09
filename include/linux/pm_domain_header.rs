/* SPDX-License-Identifier: GPL-2.0-only */
/* Definitions and headers related to device power domains. */

// C dependencies supplied by other translated headers are intentionally not implemented here.

pub const PD_FLAG_NO_DEV_LINK: u32 = 1 << 0;
pub const PD_FLAG_DEV_LINK_ON: u32 = 1 << 1;
pub const PD_FLAG_REQUIRED_OPP: u32 = 1 << 2;
pub const PD_FLAG_ATTACH_POWER_ON: u32 = 1 << 3;
pub const PD_FLAG_DETACH_POWER_OFF: u32 = 1 << 4;

#[repr(C)]
pub struct dev_pm_domain_attach_data {
    pub pd_names: *const *const core::ffi::c_char,
    pub num_pd_names: u32,
    pub pd_flags: u32,
}

#[repr(C)]
pub struct dev_pm_domain_list {
    pub pd_devs: *mut *mut device,
    pub pd_links: *mut *mut device_link,
    pub opp_tokens: *mut u32,
    pub num_pds: u32,
}

pub const GENPD_FLAG_PM_CLK: u32 = 1 << 0;
pub const GENPD_FLAG_IRQ_SAFE: u32 = 1 << 1;
pub const GENPD_FLAG_ALWAYS_ON: u32 = 1 << 2;
pub const GENPD_FLAG_ACTIVE_WAKEUP: u32 = 1 << 3;
pub const GENPD_FLAG_CPU_DOMAIN: u32 = 1 << 4;
pub const GENPD_FLAG_RPM_ALWAYS_ON: u32 = 1 << 5;
pub const GENPD_FLAG_MIN_RESIDENCY: u32 = 1 << 6;
pub const GENPD_FLAG_OPP_TABLE_FW: u32 = 1 << 7;
pub const GENPD_FLAG_DEV_NAME_FW: u32 = 1 << 8;
pub const GENPD_FLAG_NO_SYNC_STATE: u32 = 1 << 9;
pub const GENPD_FLAG_NO_STAY_ON: u32 = 1 << 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gpd_status { GENPD_STATE_ON = 0, GENPD_STATE_OFF }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum genpd_notication { GENPD_NOTIFY_PRE_OFF = 0, GENPD_NOTIFY_OFF, GENPD_NOTIFY_PRE_ON, GENPD_NOTIFY_ON }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum genpd_sync_state { GENPD_SYNC_STATE_OFF = 0, GENPD_SYNC_STATE_SIMPLE, GENPD_SYNC_STATE_ONECELL }

#[repr(C)]
pub struct dev_power_governor {
    pub system_power_down_ok: Option<unsafe extern "C" fn(*mut dev_pm_domain) -> bool>,
    pub power_down_ok: Option<unsafe extern "C" fn(*mut dev_pm_domain) -> bool>,
    pub suspend_ok: Option<unsafe extern "C" fn(*mut device) -> bool>,
}
#[repr(C)]
pub struct gpd_dev_ops {
    pub start: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut device) -> i32>,
}
#[repr(C)]
pub struct genpd_governor_data {
    pub max_off_time_ns: i64, pub max_off_time_changed: bool, pub next_wakeup: ktime_t,
    pub next_hrtimer: ktime_t, pub last_enter: ktime_t, pub reflect_residency: bool,
    pub cached_power_down_ok: bool, pub cached_power_down_state_idx: bool,
}
#[repr(C)]
pub struct genpd_power_state {
    pub name: *const core::ffi::c_char, pub power_off_latency_ns: i64,
    pub power_on_latency_ns: i64, pub residency_ns: i64, pub usage: u64,
    pub rejected: u64, pub above: u64, pub below: u64, pub usage_s2idle: u64,
    pub fwnode: *mut fwnode_handle, pub idle_time: u64, pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct generic_pm_domain {
    pub dev: device, pub domain: dev_pm_domain, pub gpd_list_node: list_head,
    pub parent_links: list_head, pub child_links: list_head, pub dev_list: list_head,
    pub gov: *mut dev_power_governor, pub gd: *mut genpd_governor_data,
    pub power_off_work: work_struct, pub provider: *mut fwnode_handle, pub has_provider: bool,
    pub name: *const core::ffi::c_char, pub sd_count: atomic_t, pub status: gpd_status,
    pub device_count: u32, pub device_id: u32, pub suspended_count: u32, pub prepared_count: u32,
    pub performance_state: u32, pub cpus: cpumask_var_t, pub synced_poweroff: bool,
    pub stay_on: bool, pub sync_state: genpd_sync_state,
    pub power_off: Option<unsafe extern "C" fn(*mut generic_pm_domain) -> i32>,
    pub power_on: Option<unsafe extern "C" fn(*mut generic_pm_domain) -> i32>,
    pub power_notifiers: raw_notifier_head, pub opp_table: *mut opp_table,
    pub set_performance_state: Option<unsafe extern "C" fn(*mut generic_pm_domain, u32) -> i32>,
    pub dev_ops: gpd_dev_ops,
    pub set_hwmode_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device, bool) -> i32>,
    pub get_hwmode_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device) -> bool>,
    pub attach_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device) -> i32>,
    pub detach_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device)>,
    pub flags: u32, pub states: *mut genpd_power_state,
    pub free_states: Option<unsafe extern "C" fn(*mut genpd_power_state, u32)>,
    pub state_count: u32, pub state_idx: u32, pub on_time: u64, pub accounting_time: u64,
    pub lock_ops: *const genpd_lock_ops,
    pub lock: generic_pm_domain_lock,
}

#[inline]
pub unsafe fn pd_to_genpd(pd: *mut dev_pm_domain) -> *mut generic_pm_domain {
    (pd as *mut u8).sub(core::mem::offset_of!(generic_pm_domain, domain)) as *mut generic_pm_domain
}

#[repr(C)]
pub union generic_pm_domain_lock {
    pub mlock: mutex,
    pub slock: generic_pm_domain_spin_lock,
    pub raw_slock: generic_pm_domain_raw_spin_lock,
}
#[repr(C)] pub struct generic_pm_domain_spin_lock { pub slock: spinlock_t, pub lock_flags: u64 }
#[repr(C)] pub struct generic_pm_domain_raw_spin_lock { pub raw_slock: raw_spinlock_t, pub raw_lock_flags: u64 }

#[repr(C)] pub struct gpd_link { pub parent: *mut generic_pm_domain, pub parent_node: list_head, pub child: *mut generic_pm_domain, pub child_node: list_head, pub performance_state: u32, pub prev_performance_state: u32 }
#[repr(C)] pub struct gpd_timing_data { pub suspend_latency_ns: i64, pub resume_latency_ns: i64, pub effective_constraint_ns: i64, pub next_wakeup: ktime_t, pub constraint_changed: bool, pub cached_suspend_ok: bool }
#[repr(C)] pub struct pm_domain_data { pub list_node: list_head, pub dev: *mut device }
#[repr(C)] pub struct generic_pm_domain_data { pub base: pm_domain_data, pub td: *mut gpd_timing_data, pub nb: notifier_block, pub power_nb: *mut notifier_block, pub cpu: i32, pub performance_state: u32, pub default_pstate: u32, pub rpm_pstate: u32, pub opp_token: u32, pub hw_mode: bool, pub rpm_always_on: bool, pub data: *mut core::ffi::c_void }

extern "C" {
    pub fn pm_genpd_add_device(genpd: *mut generic_pm_domain, dev: *mut device) -> i32;
    pub fn pm_genpd_remove_device(dev: *mut device) -> i32;
    pub fn pm_genpd_add_subdomain(genpd: *mut generic_pm_domain, subdomain: *mut generic_pm_domain) -> i32;
    pub fn pm_genpd_remove_subdomain(genpd: *mut generic_pm_domain, subdomain: *mut generic_pm_domain) -> i32;
    pub fn pm_genpd_init(genpd: *mut generic_pm_domain, gov: *mut dev_power_governor, is_off: bool) -> i32;
    pub fn pm_genpd_remove(genpd: *mut generic_pm_domain) -> i32;
    pub fn pm_genpd_inc_rejected(genpd: *mut generic_pm_domain, state_idx: u32);
    pub fn dev_to_genpd_dev(dev: *mut device) -> *mut device;
    pub fn dev_pm_genpd_set_performance_state(dev: *mut device, state: u32) -> i32;
    pub fn dev_pm_genpd_add_notifier(dev: *mut device, nb: *mut notifier_block) -> i32;
    pub fn dev_pm_genpd_remove_notifier(dev: *mut device) -> i32;
    pub fn dev_pm_genpd_set_next_wakeup(dev: *mut device, next: ktime_t);
    pub fn dev_pm_genpd_get_next_hrtimer(dev: *mut device) -> ktime_t;
    pub fn dev_pm_genpd_synced_poweroff(dev: *mut device);
    pub fn dev_pm_genpd_set_hwmode(dev: *mut device, enable: bool) -> i32;
    pub fn dev_pm_genpd_get_hwmode(dev: *mut device) -> bool;
    pub fn dev_pm_genpd_rpm_always_on(dev: *mut device, on: bool) -> i32;
    pub fn dev_pm_genpd_is_on(dev: *mut device) -> bool;
    pub fn dev_pm_domain_attach(dev: *mut device, flags: u32) -> i32;
    pub fn dev_pm_domain_attach_by_id(dev: *mut device, index: u32) -> *mut device;
    pub fn dev_pm_domain_attach_by_name(dev: *mut device, name: *const core::ffi::c_char) -> *mut device;
    pub fn dev_pm_domain_attach_list(dev: *mut device, data: *const dev_pm_domain_attach_data, list: *mut *mut dev_pm_domain_list) -> i32;
    pub fn devm_pm_domain_attach_list(dev: *mut device, data: *const dev_pm_domain_attach_data, list: *mut *mut dev_pm_domain_list) -> i32;
    pub fn dev_pm_domain_detach(dev: *mut device, power_off: bool);
    pub fn dev_pm_domain_detach_list(list: *mut dev_pm_domain_list);
    pub fn dev_pm_domain_start(dev: *mut device) -> i32;
    pub fn dev_pm_domain_set(dev: *mut device, pd: *mut dev_pm_domain);
    pub fn dev_pm_domain_set_performance_state(dev: *mut device, state: u32) -> i32;
}

// The following declarations retain the OF-provider interfaces and their configuration intent.
pub type genpd_xlate_t = unsafe extern "C" fn(*const of_phandle_args, *mut core::ffi::c_void) -> *mut generic_pm_domain;
#[repr(C)] pub struct genpd_onecell_data { pub domains: *mut *mut generic_pm_domain, pub num_domains: u32, pub xlate: Option<genpd_xlate_t> }

extern "C" {
    pub fn of_genpd_add_provider_simple(np: *mut device_node, genpd: *mut generic_pm_domain) -> i32;
    pub fn of_genpd_add_provider_onecell(np: *mut device_node, data: *mut genpd_onecell_data) -> i32;
    pub fn of_genpd_del_provider(np: *mut device_node);
    pub fn of_genpd_add_device(args: *const of_phandle_args, dev: *mut device) -> i32;
    pub fn of_genpd_add_subdomain(parent_spec: *const of_phandle_args, subdomain_spec: *const of_phandle_args) -> i32;
    pub fn of_genpd_remove_subdomain(parent_spec: *const of_phandle_args, subdomain_spec: *const of_phandle_args) -> i32;
    pub fn of_genpd_remove_last(np: *mut device_node) -> *mut generic_pm_domain;
    pub fn of_genpd_parse_idle_states(dn: *mut device_node, states: *mut *mut genpd_power_state, n: *mut i32) -> i32;
    pub fn of_genpd_sync_state(np: *mut device_node);
    pub fn of_genpd_add_child_ids(np: *mut device_node, data: *mut genpd_onecell_data) -> i32;
    pub fn of_genpd_remove_child_ids(np: *mut device_node, data: *mut genpd_onecell_data) -> i32;
    pub fn genpd_dev_pm_attach(dev: *mut device) -> i32;
    pub fn genpd_dev_pm_attach_by_id(dev: *mut device, index: u32) -> *mut device;
    pub fn genpd_dev_pm_attach_by_name(dev: *mut device, name: *const core::ffi::c_char) -> *mut device;
    pub fn dev_pm_genpd_suspend(dev: *mut device);
    pub fn dev_pm_genpd_resume(dev: *mut device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
