/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* C header dependencies are supplied by the surrounding kernel translation. */

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct suspend_state_t(pub i32);

pub const PM_SUSPEND_ON: suspend_state_t = suspend_state_t(0);
pub const PM_SUSPEND_TO_IDLE: suspend_state_t = suspend_state_t(1);
pub const PM_SUSPEND_STANDBY: suspend_state_t = suspend_state_t(2);
pub const PM_SUSPEND_MEM: suspend_state_t = suspend_state_t(3);
pub const PM_SUSPEND_MIN: suspend_state_t = PM_SUSPEND_TO_IDLE;
pub const PM_SUSPEND_MAX: suspend_state_t = suspend_state_t(4);

#[cfg(feature = "CONFIG_VT")]
extern "C" { pub fn pm_set_vt_switch(do_switch: ::core::ffi::c_int); }
#[cfg(not(feature = "CONFIG_VT"))]
#[inline] pub unsafe fn pm_set_vt_switch(_do_switch: ::core::ffi::c_int) {}

#[cfg(feature = "CONFIG_VT_CONSOLE_SLEEP")]
extern "C" { pub fn pm_prepare_console(); pub fn pm_restore_console(); }
#[cfg(not(feature = "CONFIG_VT_CONSOLE_SLEEP"))]
#[inline] pub unsafe fn pm_prepare_console() {}
#[cfg(not(feature = "CONFIG_VT_CONSOLE_SLEEP"))]
#[inline] pub unsafe fn pm_restore_console() {}

#[repr(C)]
pub struct platform_suspend_ops {
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> ::core::ffi::c_int>,
    pub begin: Option<unsafe extern "C" fn(suspend_state_t) -> ::core::ffi::c_int>,
    pub prepare: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub prepare_late: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> ::core::ffi::c_int>,
    pub wake: Option<unsafe extern "C" fn()>, pub finish: Option<unsafe extern "C" fn()>,
    pub suspend_again: Option<unsafe extern "C" fn() -> bool>,
    pub end: Option<unsafe extern "C" fn()>, pub recover: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct platform_s2idle_ops {
    pub begin: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub prepare: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub prepare_late: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub check: Option<unsafe extern "C" fn()>, pub wake: Option<unsafe extern "C" fn() -> bool>,
    pub restore_early: Option<unsafe extern "C" fn()>, pub restore: Option<unsafe extern "C" fn()>,
    pub end: Option<unsafe extern "C" fn()>,
}

#[cfg(feature = "CONFIG_SUSPEND")]
extern "C" {
    pub static mut pm_suspend_target_state: suspend_state_t;
    pub static mut mem_sleep_current: suspend_state_t;
    pub static mut mem_sleep_default: suspend_state_t;
    pub fn suspend_set_ops(ops: *const platform_suspend_ops);
    pub fn suspend_valid_only_mem(state: suspend_state_t) -> ::core::ffi::c_int;
    pub static mut pm_suspend_global_flags: u32;
    pub fn pm_suspend(state: suspend_state_t) -> ::core::ffi::c_int;
    pub static mut sync_on_suspend_enabled: bool;
}

pub const PM_SUSPEND_FLAG_FW_SUSPEND: u32 = 1 << 0;
pub const PM_SUSPEND_FLAG_FW_RESUME: u32 = 1 << 1;
pub const PM_SUSPEND_FLAG_NO_PLATFORM: u32 = 1 << 2;

#[cfg(feature = "CONFIG_SUSPEND")]
#[inline] pub unsafe fn pm_suspend_clear_flags() { pm_suspend_global_flags = 0; }
#[cfg(feature = "CONFIG_SUSPEND")]
#[inline] pub unsafe fn pm_set_suspend_via_firmware() { pm_suspend_global_flags |= PM_SUSPEND_FLAG_FW_SUSPEND; }
#[cfg(feature = "CONFIG_SUSPEND")]
#[inline] pub unsafe fn pm_set_resume_via_firmware() { pm_suspend_global_flags |= PM_SUSPEND_FLAG_FW_RESUME; }
#[cfg(feature = "CONFIG_SUSPEND")]
#[inline] pub unsafe fn pm_set_suspend_no_platform() { pm_suspend_global_flags |= PM_SUSPEND_FLAG_NO_PLATFORM; }
#[cfg(feature = "CONFIG_SUSPEND")]
#[inline] pub unsafe fn pm_suspend_via_firmware() -> bool { pm_suspend_global_flags & PM_SUSPEND_FLAG_FW_SUSPEND != 0 }
#[cfg(feature = "CONFIG_SUSPEND")]
#[inline] pub unsafe fn pm_resume_via_firmware() -> bool { pm_suspend_global_flags & PM_SUSPEND_FLAG_FW_RESUME != 0 }
#[cfg(feature = "CONFIG_SUSPEND")]
#[inline] pub unsafe fn pm_suspend_no_platform() -> bool { pm_suspend_global_flags & PM_SUSPEND_FLAG_NO_PLATFORM != 0 }

#[repr(C)]
pub struct pbe { pub address: *mut ::core::ffi::c_void, pub orig_address: *mut ::core::ffi::c_void, pub next: *mut pbe }

#[repr(C)]
pub struct platform_hibernation_ops {
    pub begin: Option<unsafe extern "C" fn(pm_message_t) -> ::core::ffi::c_int>,
    pub end: Option<unsafe extern "C" fn()>, pub pre_snapshot: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub finish: Option<unsafe extern "C" fn()>, pub prepare: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub enter: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>, pub leave: Option<unsafe extern "C" fn()>,
    pub pre_restore: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub restore_cleanup: Option<unsafe extern "C" fn()>, pub recover: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s2idle_states { S2IDLE_STATE_NONE, S2IDLE_STATE_ENTER, S2IDLE_STATE_WAKE }

#[cfg(feature = "CONFIG_SUSPEND")]
extern "C" {
    pub static mut s2idle_state: s2idle_states;
    pub fn pm_suspend_default_s2idle() -> bool;
    pub fn pm_states_init(); pub fn s2idle_set_ops(ops: *const platform_s2idle_ops); pub fn s2idle_wake();
    pub fn arch_suspend_disable_irqs(); pub fn arch_suspend_enable_irqs();
}
#[inline] pub unsafe fn idle_should_enter_s2idle() -> bool { s2idle_state == s2idle_states::S2IDLE_STATE_ENTER }

#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn pm_suspend_target_state() -> suspend_state_t { PM_SUSPEND_ON }
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn pm_suspend_clear_flags() {}
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn pm_set_suspend_via_firmware() {}
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn pm_set_resume_via_firmware() {}
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn pm_suspend_via_firmware() -> bool { false }
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn pm_resume_via_firmware() -> bool { false }
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn pm_suspend_no_platform() -> bool { false }
#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline] pub unsafe fn idle_should_enter_s2idle() -> bool { false }

#[inline] pub unsafe fn pm_suspend_in_progress() -> bool { pm_suspend_target_state() != PM_SUSPEND_ON }

/* The following declarations retain external kernel types and symbols supplied by dependencies. */
extern "C" {
    pub fn register_nosave_region(b: c_ulong, e: c_ulong);
    pub fn swsusp_page_is_forbidden(page: *mut page) -> c_int;
    pub fn swsusp_set_page_free(page: *mut page); pub fn swsusp_unset_page_free(page: *mut page);
    pub fn get_safe_page(gfp_mask: gfp_t) -> c_ulong;
    pub fn swsusp_arch_suspend() -> c_int; pub fn swsusp_arch_resume() -> c_int;
    pub static mut swsusp_hardware_signature: u32;
    pub fn hibernation_set_ops(ops: *const platform_hibernation_ops);
    pub fn hibernate() -> c_int; pub fn system_entering_hibernation() -> bool; pub fn hibernation_available() -> bool;
    pub fn swsusp_save() -> c_int; pub static mut restore_pblist: *mut pbe; pub fn pfn_is_nosave(pfn: c_ulong) -> c_int;
    pub fn hibernate_quiet_exec(func: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn hibernate_resume_nonboot_cpu_disable() -> c_int;
    pub fn arch_hibernation_header_save(addr: *mut c_void, max_size: c_uint) -> c_int;
    pub fn arch_hibernation_header_restore(addr: *mut c_void) -> c_int;
    pub fn arch_resume_nosmt() -> c_int;
    pub static mut system_transition_mutex: mutex;
    pub fn save_processor_state(); pub fn restore_processor_state();
    pub fn register_pm_notifier(nb: *mut notifier_block) -> c_int;
    pub fn unregister_pm_notifier(nb: *mut notifier_block) -> c_int;
    pub fn ksys_sync_helper();
    pub fn pm_report_hw_sleep_time(t: u64); pub fn pm_report_max_hw_sleep(t: u64);
    pub fn pm_restrict_gfp_mask(); pub fn pm_restore_gfp_mask();
    pub static mut events_check_enabled: bool;
    pub fn pm_wakeup_pending() -> bool; pub fn pm_system_wakeup(); pub fn pm_system_cancel_wakeup();
    pub fn pm_wakeup_clear(irq_number: c_uint); pub fn pm_system_irq_wakeup(irq_number: c_uint);
    pub fn pm_wakeup_irq() -> c_uint;
    pub fn pm_get_wakeup_count(count: *mut c_uint, block: bool) -> bool;
    pub fn pm_save_wakeup_count(count: c_uint) -> bool;
    pub fn pm_wakep_autosleep_enabled(set: bool); pub fn pm_print_active_wakeup_sources();
    pub fn lock_system_sleep() -> c_uint; pub fn unlock_system_sleep(flags: c_uint);
    pub fn pm_sleep_transition_in_progress() -> bool; pub fn pm_hibernate_is_recovering() -> bool;
}

#[inline] pub unsafe fn pm_suspended_storage() -> bool { !gfp_has_io_fs(gfp_allowed_mask) }

#[cfg(feature = "CONFIG_PM_SLEEP_DEBUG")]
extern "C" {
    pub static mut pm_print_times_enabled: bool;
    pub static mut pm_debug_messages_on: bool;
    pub fn pm_debug_messages_should_print() -> bool;
}

#[cfg(feature = "CONFIG_PM_AUTOSLEEP")]
extern "C" { pub fn queue_up_suspend_work(); }
#[cfg(not(feature = "CONFIG_PM_AUTOSLEEP"))]
#[inline] pub unsafe fn queue_up_suspend_work() {}

#[cfg(not(feature = "CONFIG_HIBERNATION"))]
#[inline] pub unsafe fn hibernate() -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_HIBERNATION"))]
#[inline] pub unsafe fn system_entering_hibernation() -> bool { false }
#[cfg(not(feature = "CONFIG_HIBERNATION"))]
#[inline] pub unsafe fn hibernation_available() -> bool { false }

pub const PM_HIBERNATION_PREPARE: u32 = 0x0001;
pub const PM_POST_HIBERNATION: u32 = 0x0002;
pub const PM_SUSPEND_PREPARE: u32 = 0x0003;
pub const PM_POST_SUSPEND: u32 = 0x0004;
pub const PM_RESTORE_PREPARE: u32 = 0x0005;
pub const PM_POST_RESTORE: u32 = 0x0006;

#[repr(C)] pub struct suspend_stat_step(pub c_int);
pub const SUSPEND_WORKING: c_int = 0;
pub const SUSPEND_FREEZE: c_int = 1;
pub const SUSPEND_PREPARE: c_int = 2;
pub const SUSPEND_SUSPEND: c_int = 3;
pub const SUSPEND_SUSPEND_LATE: c_int = 4;
pub const SUSPEND_SUSPEND_NOIRQ: c_int = 5;
pub const SUSPEND_RESUME_NOIRQ: c_int = 6;
pub const SUSPEND_RESUME_EARLY: c_int = 7;
pub const SUSPEND_RESUME: c_int = 8;

extern "C" {
    pub fn dpm_save_failed_dev(name: *const c_char);
    pub fn dpm_save_failed_step(step: suspend_stat_step);
}

use ::core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
/* External kernel types/constants intentionally remain unresolved dependencies. */
extern "C" { static mut ENOSYS: c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
