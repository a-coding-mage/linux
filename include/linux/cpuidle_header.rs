/*
 * cpuidle.h - a generic framework for CPU idle power management
 *
 * (C) 2007 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 *          Shaohua Li <shaohua.li@intel.com>
 *          Adam Belay <abelay@novell.com>
 *
 * This code is licenced under the GPL.
 */

pub const CPUIDLE_STATE_MAX: usize = 10;
pub const CPUIDLE_NAME_LEN: usize = 16;
pub const CPUIDLE_DESC_LEN: usize = 32;

pub const CPUIDLE_STATE_DISABLED_BY_USER: u32 = 1 << 0;
pub const CPUIDLE_STATE_DISABLED_BY_DRIVER: u32 = 1 << 1;

#[repr(C)]
pub struct cpuidle_state_usage {
    pub disable: u64,
    pub usage: u64,
    pub time_ns: u64,
    pub above: u64,
    pub below: u64,
    pub rejected: u64,
    #[cfg(feature = "CONFIG_SUSPEND")]
    pub s2idle_usage: u64,
    #[cfg(feature = "CONFIG_SUSPEND")]
    pub s2idle_time: u64,
}

#[repr(C)]
pub struct cpuidle_state {
    pub name: [core::ffi::c_char; CPUIDLE_NAME_LEN],
    pub desc: [core::ffi::c_char; CPUIDLE_DESC_LEN],
    pub exit_latency_ns: i64,
    pub target_residency_ns: i64,
    pub flags: u32,
    pub exit_latency: u32,
    pub power_usage: i32,
    pub target_residency: u32,
    pub enter: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
    pub enter_dead: Option<unsafe extern "C" fn(*mut cpuidle_device, i32)>,
    pub enter_s2idle: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
}

pub const CPUIDLE_FLAG_NONE: u32 = 0x00;
pub const CPUIDLE_FLAG_POLLING: u32 = 1 << 0;
pub const CPUIDLE_FLAG_COUPLED: u32 = 1 << 1;
pub const CPUIDLE_FLAG_TIMER_STOP: u32 = 1 << 2;
pub const CPUIDLE_FLAG_UNUSABLE: u32 = 1 << 3;
pub const CPUIDLE_FLAG_OFF: u32 = 1 << 4;
pub const CPUIDLE_FLAG_TLB_FLUSHED: u32 = 1 << 5;
pub const CPUIDLE_FLAG_RCU_IDLE: u32 = 1 << 6;

#[repr(C)]
pub struct cpuidle_device {
    pub registered: u32,
    pub enabled: u32,
    pub poll_time_limit: u32,
    pub cpu: u32,
    pub next_hrtimer: ktime_t,
    pub last_state_idx: i32,
    pub last_residency_ns: u64,
    pub poll_limit_ns: u64,
    pub forced_idle_latency_limit_ns: u64,
    pub states_usage: [cpuidle_state_usage; CPUIDLE_STATE_MAX],
    pub kobjs: [*mut cpuidle_state_kobj; CPUIDLE_STATE_MAX],
    pub kobj_driver: *mut cpuidle_driver_kobj,
    pub kobj_dev: *mut cpuidle_device_kobj,
    pub device_list: list_head,
    #[cfg(feature = "CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED")]
    pub coupled_cpus: cpumask_t,
    #[cfg(feature = "CONFIG_ARCH_NEEDS_CPU_IDLE_COUPLED")]
    pub coupled: *mut cpuidle_coupled,
}

pub type cpuidle_devices = *mut cpuidle_device;

extern "C" {
    pub static mut cpuidle_devices: *mut *mut cpuidle_device;
    pub static mut cpuidle_dev: *mut cpuidle_device;
}

pub unsafe fn ct_cpuidle_enter() {
    lockdep_assert_irqs_disabled();
    trace_hardirqs_on_prepare();
    lockdep_hardirqs_on_prepare();
    instrumentation_end();
    ct_idle_enter();
    lockdep_hardirqs_on(core::ptr::null());
}

pub unsafe fn ct_cpuidle_exit() {
    lockdep_hardirqs_off(core::ptr::null());
    ct_idle_exit();
    instrumentation_begin();
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const core::ffi::c_char,
    pub owner: *mut module,
    pub bctimer: u32,
    pub states: [cpuidle_state; CPUIDLE_STATE_MAX],
    pub state_count: i32,
    pub safe_state_index: i32,
    pub cpumask: *mut cpumask,
    pub governor: *const core::ffi::c_char,
}

#[repr(C)]
pub struct cpuidle_governor {
    pub name: [core::ffi::c_char; CPUIDLE_NAME_LEN],
    pub governor_list: list_head,
    pub rating: u32,
    pub enable: Option<unsafe extern "C" fn(*mut cpuidle_driver, *mut cpuidle_device) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut cpuidle_driver, *mut cpuidle_device)>,
    pub select: Option<unsafe extern "C" fn(*mut cpuidle_driver, *mut cpuidle_device, *mut bool) -> i32>,
    pub reflect: Option<unsafe extern "C" fn(*mut cpuidle_device, i32)>,
}

extern "C" {
    pub fn disable_cpuidle();
    pub fn cpuidle_not_available(drv: *mut cpuidle_driver, dev: *mut cpuidle_device) -> bool;
    pub fn cpuidle_select(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, stop_tick: *mut bool) -> i32;
    pub fn cpuidle_enter(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, index: i32) -> i32;
    pub fn cpuidle_reflect(dev: *mut cpuidle_device, index: i32);
    pub fn cpuidle_poll_time(drv: *mut cpuidle_driver, dev: *mut cpuidle_device) -> u64;
    pub fn cpuidle_register_driver(drv: *mut cpuidle_driver) -> i32;
    pub fn cpuidle_get_driver() -> *mut cpuidle_driver;
    pub fn cpuidle_driver_state_disabled(drv: *mut cpuidle_driver, idx: i32, disable: bool);
    pub fn cpuidle_unregister_driver(drv: *mut cpuidle_driver);
    pub fn cpuidle_register_device(dev: *mut cpuidle_device) -> i32;
    pub fn cpuidle_unregister_device(dev: *mut cpuidle_device);
    pub fn cpuidle_unregister_device_no_lock(dev: *mut cpuidle_device);
    pub fn cpuidle_register(drv: *mut cpuidle_driver, coupled_cpus: *const cpumask) -> i32;
    pub fn cpuidle_unregister(drv: *mut cpuidle_driver);
    pub fn cpuidle_pause_and_lock();
    pub fn cpuidle_resume_and_unlock();
    pub fn cpuidle_pause();
    pub fn cpuidle_resume();
    pub fn cpuidle_enable_device(dev: *mut cpuidle_device) -> i32;
    pub fn cpuidle_disable_device(dev: *mut cpuidle_device);
    pub fn cpuidle_play_dead() -> i32;
    pub fn cpuidle_get_cpu_driver(dev: *mut cpuidle_device) -> *mut cpuidle_driver;
    pub fn cpuidle_get_device() -> *mut cpuidle_device;
    pub fn cpuidle_find_deepest_state(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, latency_limit_ns: u64) -> i32;
    pub fn cpuidle_enter_s2idle(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, latency_limit_ns: u64) -> i32;
    pub fn cpuidle_use_deepest_state(latency_limit_ns: u64);
    pub fn sched_idle_set_state(idle_state: *mut cpuidle_state);
    pub fn default_idle_call();
    pub fn cpuidle_register_governor(gov: *mut cpuidle_governor) -> i32;
    pub fn cpuidle_governor_latency_req(cpu: u32) -> i64;
}

#[macro_export]
macro_rules! __CPU_PM_CPU_IDLE_ENTER {
    ($low_level_idle_enter:expr, $idx:expr, $state:expr, $is_retention:expr, $is_rcu:expr) => {{
        let mut __ret: i32 = 0;
        if $idx == 0 { unsafe { cpu_do_idle(); } return $idx; }
        if !$is_retention { __ret = unsafe { cpu_pm_enter() }; }
        if __ret == 0 {
            if !$is_rcu { unsafe { $crate::ct_cpuidle_enter(); } }
            __ret = unsafe { $low_level_idle_enter($state) };
            if !$is_rcu { unsafe { $crate::ct_cpuidle_exit(); } }
            if !$is_retention { unsafe { cpu_pm_exit(); } }
        }
        if __ret != 0 { -1 } else { $idx }
    }};
}

#[macro_export]
macro_rules! CPU_PM_CPU_IDLE_ENTER { ($f:expr, $idx:expr) => { $crate::__CPU_PM_CPU_IDLE_ENTER!($f, $idx, $idx, false, false) }; }
#[macro_export]
macro_rules! CPU_PM_CPU_IDLE_ENTER_RETENTION { ($f:expr, $idx:expr) => { $crate::__CPU_PM_CPU_IDLE_ENTER!($f, $idx, $idx, true, false) }; }
#[macro_export]
macro_rules! CPU_PM_CPU_IDLE_ENTER_PARAM { ($f:expr, $idx:expr, $state:expr) => { $crate::__CPU_PM_CPU_IDLE_ENTER!($f, $idx, $state, false, false) }; }
#[macro_export]
macro_rules! CPU_PM_CPU_IDLE_ENTER_PARAM_RCU { ($f:expr, $idx:expr, $state:expr) => { $crate::__CPU_PM_CPU_IDLE_ENTER!($f, $idx, $state, false, true) }; }
#[macro_export]
macro_rules! CPU_PM_CPU_IDLE_ENTER_RETENTION_PARAM { ($f:expr, $idx:expr, $state:expr) => { $crate::__CPU_PM_CPU_IDLE_ENTER!($f, $idx, $state, true, false) }; }
#[macro_export]
macro_rules! CPU_PM_CPU_IDLE_ENTER_RETENTION_PARAM_RCU { ($f:expr, $idx:expr, $state:expr) => { $crate::__CPU_PM_CPU_IDLE_ENTER!($f, $idx, $state, true, true) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
