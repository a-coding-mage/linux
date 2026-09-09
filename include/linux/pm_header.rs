/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Power management interface. */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct wakeup_source;
#[repr(C)]
pub struct wake_irq;
#[repr(C)]
pub struct pm_domain_data;
#[repr(C)]
pub struct dev_pm_qos;
#[repr(C)]
pub struct spinlock_t;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct completion;
#[repr(C)]
pub struct hrtimer;
#[repr(C)]
pub struct work_struct;
#[repr(C)]
pub struct wait_queue_head_t;
#[repr(C)]
pub struct atomic_t;

pub type bool_ = bool;
pub type u32_ = u32;
pub type u64_ = u64;
pub type s32_ = i32;

extern "C" {
    pub static mut pm_power_off: Option<unsafe extern "C" fn()>;
}

#[cfg(feature = "CONFIG_VT_CONSOLE_SLEEP")]
extern "C" {
    pub fn pm_vt_switch_required(dev: *mut device, required: bool) -> i32;
    pub fn pm_vt_switch_unregister(dev: *mut device);
}
#[cfg(not(feature = "CONFIG_VT_CONSOLE_SLEEP"))]
#[inline]
pub unsafe fn pm_vt_switch_required(_dev: *mut device, _required: bool) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_VT_CONSOLE_SLEEP"))]
#[inline]
pub unsafe fn pm_vt_switch_unregister(_dev: *mut device) {}

#[cfg(feature = "CONFIG_CXL_SUSPEND")]
extern "C" { pub fn cxl_mem_active() -> bool; }
#[cfg(not(feature = "CONFIG_CXL_SUSPEND"))]
#[inline]
pub fn cxl_mem_active() -> bool { false }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_message { pub event: i32 }
pub type pm_message_t = pm_message;

#[repr(C)]
pub struct dev_pm_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub complete: Option<unsafe extern "C" fn(*mut device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub freeze: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub thaw: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub poweroff: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub restore: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub suspend_late: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume_early: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub freeze_late: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub thaw_early: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub poweroff_late: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub restore_early: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub suspend_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub freeze_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub thaw_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub poweroff_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub restore_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

// Build-time CONFIG_PM and CONFIG_PM_SLEEP conditions are retained as cfgs.
#[macro_export]
macro_rules! SYSTEM_SLEEP_PM_OPS { ($s:expr, $r:expr) => { suspend: $s, resume: $r, freeze: $s, thaw: $r, poweroff: $s, restore: $r } }
#[macro_export]
macro_rules! LATE_SYSTEM_SLEEP_PM_OPS { ($s:expr, $r:expr) => { suspend_late: $s, resume_early: $r, freeze_late: $s, thaw_early: $r, poweroff_late: $s, restore_early: $r } }
#[macro_export]
macro_rules! NOIRQ_SYSTEM_SLEEP_PM_OPS { ($s:expr, $r:expr) => { suspend_noirq: $s, resume_noirq: $r, freeze_noirq: $s, thaw_noirq: $r, poweroff_noirq: $s, restore_noirq: $r } }
#[macro_export]
macro_rules! RUNTIME_PM_OPS { ($s:expr, $r:expr, $i:expr) => { runtime_suspend: $s, runtime_resume: $r, runtime_idle: $i } }
#[macro_export]
macro_rules! DEFINE_SIMPLE_DEV_PM_OPS { ($name:ident, $s:expr, $r:expr) => { pub const $name: dev_pm_ops = dev_pm_ops { SYSTEM_SLEEP_PM_OPS!($s, $r), ..unsafe { core::mem::zeroed() } }; } }
#[macro_export]
macro_rules! DEFINE_NOIRQ_DEV_PM_OPS { ($name:ident, $s:expr, $r:expr) => { pub const $name: dev_pm_ops = dev_pm_ops { NOIRQ_SYSTEM_SLEEP_PM_OPS!($s, $r), ..unsafe { core::mem::zeroed() } }; } }

pub const PM_EVENT_INVALID: i32 = -1;
pub const PM_EVENT_ON: i32 = 0x0000;
pub const PM_EVENT_FREEZE: i32 = 0x0001;
pub const PM_EVENT_SUSPEND: i32 = 0x0002;
pub const PM_EVENT_HIBERNATE: i32 = 0x0004;
pub const PM_EVENT_QUIESCE: i32 = 0x0008;
pub const PM_EVENT_RESUME: i32 = 0x0010;
pub const PM_EVENT_THAW: i32 = 0x0020;
pub const PM_EVENT_RESTORE: i32 = 0x0040;
pub const PM_EVENT_RECOVER: i32 = 0x0080;
pub const PM_EVENT_USER: i32 = 0x0100;
pub const PM_EVENT_REMOTE: i32 = 0x0200;
pub const PM_EVENT_AUTO: i32 = 0x0400;
pub const PM_EVENT_POWEROFF: i32 = 0x0800;
pub const PM_EVENT_SLEEP: i32 = PM_EVENT_SUSPEND | PM_EVENT_HIBERNATE;
pub const PM_EVENT_USER_SUSPEND: i32 = PM_EVENT_USER | PM_EVENT_SUSPEND;
pub const PM_EVENT_USER_RESUME: i32 = PM_EVENT_USER | PM_EVENT_RESUME;
pub const PM_EVENT_REMOTE_RESUME: i32 = PM_EVENT_REMOTE | PM_EVENT_RESUME;
pub const PM_EVENT_AUTO_SUSPEND: i32 = PM_EVENT_AUTO | PM_EVENT_SUSPEND;
pub const PM_EVENT_AUTO_RESUME: i32 = PM_EVENT_AUTO | PM_EVENT_RESUME;

pub const PMSG_INVALID: pm_message = pm_message { event: PM_EVENT_INVALID };
pub const PMSG_ON: pm_message = pm_message { event: PM_EVENT_ON };
pub const PMSG_FREEZE: pm_message = pm_message { event: PM_EVENT_FREEZE };
pub const PMSG_QUIESCE: pm_message = pm_message { event: PM_EVENT_QUIESCE };
pub const PMSG_SUSPEND: pm_message = pm_message { event: PM_EVENT_SUSPEND };
pub const PMSG_HIBERNATE: pm_message = pm_message { event: PM_EVENT_HIBERNATE };
pub const PMSG_POWEROFF: pm_message = pm_message { event: PM_EVENT_POWEROFF };
pub const PMSG_RESUME: pm_message = pm_message { event: PM_EVENT_RESUME };
pub const PMSG_THAW: pm_message = pm_message { event: PM_EVENT_THAW };
pub const PMSG_RESTORE: pm_message = pm_message { event: PM_EVENT_RESTORE };
pub const PMSG_RECOVER: pm_message = pm_message { event: PM_EVENT_RECOVER };
pub const PMSG_USER_SUSPEND: pm_message = pm_message { event: PM_EVENT_USER_SUSPEND };
pub const PMSG_USER_RESUME: pm_message = pm_message { event: PM_EVENT_USER_RESUME };
pub const PMSG_REMOTE_RESUME: pm_message = pm_message { event: PM_EVENT_REMOTE_RESUME };
pub const PMSG_AUTO_SUSPEND: pm_message = pm_message { event: PM_EVENT_AUTO_SUSPEND };
pub const PMSG_AUTO_RESUME: pm_message = pm_message { event: PM_EVENT_AUTO_RESUME };
#[inline] pub fn PMSG_IS_AUTO(msg: pm_message) -> bool { (msg.event & PM_EVENT_AUTO) != 0 }
#[inline] pub fn PMSG_NO_WAKEUP(msg: pm_message) -> bool { (msg.event & (PM_EVENT_FREEZE | PM_EVENT_QUIESCE)) != 0 }

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpm_status { RPM_INVALID = -1, RPM_ACTIVE = 0, RPM_RESUMING, RPM_SUSPENDED, RPM_SUSPENDING, RPM_BLOCKED }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpm_request { RPM_REQ_NONE = 0, RPM_REQ_IDLE, RPM_REQ_SUSPEND, RPM_REQ_AUTOSUSPEND, RPM_REQ_RESUME }

#[repr(C)]
pub struct pm_subsys_data {
    pub lock: spinlock_t,
    pub refcount: u32,
    #[cfg(feature = "CONFIG_PM_CLK")] pub clock_op_might_sleep: u32,
    #[cfg(feature = "CONFIG_PM_CLK")] pub clock_mutex: mutex,
    #[cfg(feature = "CONFIG_PM_CLK")] pub clock_list: list_head,
    #[cfg(feature = "CONFIG_PM_GENERIC_DOMAINS")] pub domain_data: *mut pm_domain_data,
}

pub const DPM_FLAG_NO_DIRECT_COMPLETE: u32 = 1 << 0;
pub const DPM_FLAG_SMART_PREPARE: u32 = 1 << 1;
pub const DPM_FLAG_SMART_SUSPEND: u32 = 1 << 2;
pub const DPM_FLAG_MAY_SKIP_RESUME: u32 = 1 << 3;

#[repr(C)]
pub struct dev_pm_info {
    pub power_state: pm_message_t,
    pub can_wakeup: bool, pub async_suspend: bool, pub in_dpm_list: bool, pub is_prepared: bool,
    pub is_suspended: bool, pub is_noirq_suspended: bool, pub is_late_suspended: bool,
    pub no_pm: bool, pub early_init: bool, pub direct_complete: bool,
    pub driver_flags: u32, pub lock: spinlock_t,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub entry: list_head,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub completion: completion,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub wakeup: *mut wakeup_source,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub work_in_progress: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub wakeup_path: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub syscore: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub no_pm_callbacks: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub smart_suspend: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub must_resume: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub may_skip_resume: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub out_band_wakeup: bool,
    #[cfg(feature = "CONFIG_PM_SLEEP")] pub strict_midlayer: bool,
    #[cfg(not(feature = "CONFIG_PM_SLEEP"))] pub should_wakeup: bool,
    #[cfg(feature = "CONFIG_PM")] pub suspend_timer: hrtimer,
    #[cfg(feature = "CONFIG_PM")] pub timer_expires: u64,
    #[cfg(feature = "CONFIG_PM")] pub work: work_struct,
    #[cfg(feature = "CONFIG_PM")] pub wait_queue: wait_queue_head_t,
    #[cfg(feature = "CONFIG_PM")] pub wakeirq: *mut wake_irq,
    #[cfg(feature = "CONFIG_PM")] pub usage_count: atomic_t,
    #[cfg(feature = "CONFIG_PM")] pub child_count: atomic_t,
    #[cfg(feature = "CONFIG_PM")] pub disable_depth: u32,
    #[cfg(feature = "CONFIG_PM")] pub idle_notification: bool,
    #[cfg(feature = "CONFIG_PM")] pub request_pending: bool,
    #[cfg(feature = "CONFIG_PM")] pub deferred_resume: bool,
    #[cfg(feature = "CONFIG_PM")] pub needs_force_resume: bool,
    #[cfg(feature = "CONFIG_PM")] pub runtime_auto: bool,
    #[cfg(feature = "CONFIG_PM")] pub ignore_children: bool,
    #[cfg(feature = "CONFIG_PM")] pub no_callbacks: bool,
    #[cfg(feature = "CONFIG_PM")] pub irq_safe: bool,
    #[cfg(feature = "CONFIG_PM")] pub use_autosuspend: bool,
    #[cfg(feature = "CONFIG_PM")] pub timer_autosuspends: bool,
    #[cfg(feature = "CONFIG_PM")] pub memalloc_noio: bool,
    #[cfg(feature = "CONFIG_PM")] pub links_count: u32,
    #[cfg(feature = "CONFIG_PM")] pub request: rpm_request,
    #[cfg(feature = "CONFIG_PM")] pub runtime_status: rpm_status,
    #[cfg(feature = "CONFIG_PM")] pub last_status: rpm_status,
    #[cfg(feature = "CONFIG_PM")] pub runtime_error: i32,
    #[cfg(feature = "CONFIG_PM")] pub autosuspend_delay: i32,
    #[cfg(feature = "CONFIG_PM")] pub last_busy: u64,
    #[cfg(feature = "CONFIG_PM")] pub active_time: u64,
    #[cfg(feature = "CONFIG_PM")] pub suspended_time: u64,
    #[cfg(feature = "CONFIG_PM")] pub accounting_timestamp: u64,
    pub subsys_data: *mut pm_subsys_data,
    pub set_latency_tolerance: Option<unsafe extern "C" fn(*mut device, i32)>,
    pub qos: *mut dev_pm_qos,
    pub detach_power_off: bool,
}

extern "C" {
    pub fn dev_pm_get_subsys_data(dev: *mut device) -> i32;
    pub fn dev_pm_put_subsys_data(dev: *mut device);
}

#[repr(C)]
pub struct dev_pm_domain {
    pub ops: dev_pm_ops,
    pub start: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut device, bool)>,
    pub activate: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub sync: Option<unsafe extern "C" fn(*mut device)>,
    pub dismiss: Option<unsafe extern "C" fn(*mut device)>,
    pub set_performance_state: Option<unsafe extern "C" fn(*mut device, u32) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
