/* SPDX-License-Identifier: GPL-2.0-only */
/* pm_runtime.h - Device run-time power management helper functions. */

// C header dependencies are supplied by the surrounding translation.

pub const RPM_ASYNC: ::core::ffi::c_int = 0x01;
pub const RPM_NOWAIT: ::core::ffi::c_int = 0x02;
pub const RPM_GET_PUT: ::core::ffi::c_int = 0x04;
pub const RPM_AUTO: ::core::ffi::c_int = 0x08;
pub const RPM_TRANSPARENT: ::core::ffi::c_int = 0x10;

// DEFINE_RUNTIME_DEV_PM_OPS and its EXPORT_* variants are build-system
// registration macros in C; their expansion is intentionally retained here.
macro_rules! DEFINE_RUNTIME_DEV_PM_OPS { ($($t:tt)*) => { /* C macro */ }; }
macro_rules! EXPORT_RUNTIME_DEV_PM_OPS { ($($t:tt)*) => { /* C macro */ }; }
macro_rules! EXPORT_GPL_RUNTIME_DEV_PM_OPS { ($($t:tt)*) => { /* C macro */ }; }
macro_rules! EXPORT_NS_RUNTIME_DEV_PM_OPS { ($($t:tt)*) => { /* C macro */ }; }
macro_rules! EXPORT_NS_GPL_RUNTIME_DEV_PM_OPS { ($($t:tt)*) => { /* C macro */ }; }

#[cfg(CONFIG_PM)]
extern "C" {
    pub static mut pm_wq: *mut workqueue_struct;
    pub fn pm_generic_runtime_suspend(dev: *mut device) -> ::core::ffi::c_int;
    pub fn pm_generic_runtime_resume(dev: *mut device) -> ::core::ffi::c_int;
    pub fn pm_runtime_force_suspend(dev: *mut device) -> ::core::ffi::c_int;
    pub fn __pm_runtime_idle(dev: *mut device, rpmflags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __pm_runtime_suspend(dev: *mut device, rpmflags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __pm_runtime_resume(dev: *mut device, rpmflags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn pm_runtime_get_if_active(dev: *mut device) -> ::core::ffi::c_int;
    pub fn pm_runtime_get_if_in_use(dev: *mut device) -> ::core::ffi::c_int;
    pub fn pm_schedule_suspend(dev: *mut device, delay: u32) -> ::core::ffi::c_int;
    pub fn __pm_runtime_set_status(dev: *mut device, status: u32) -> ::core::ffi::c_int;
    pub fn pm_runtime_barrier(dev: *mut device);
    pub fn pm_runtime_block_if_disabled(dev: *mut device) -> bool;
    pub fn pm_runtime_unblock(dev: *mut device);
    pub fn pm_runtime_enable(dev: *mut device);
    pub fn __pm_runtime_disable(dev: *mut device, check_resume: bool);
    pub fn pm_runtime_allow(dev: *mut device);
    pub fn pm_runtime_forbid(dev: *mut device);
    pub fn pm_runtime_no_callbacks(dev: *mut device);
    pub fn pm_runtime_irq_safe(dev: *mut device);
    pub fn __pm_runtime_use_autosuspend(dev: *mut device, use_: bool);
    pub fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: ::core::ffi::c_int);
    pub fn pm_runtime_autosuspend_expiration(dev: *mut device) -> u64;
    pub fn pm_runtime_set_memalloc_noio(dev: *mut device, enable: bool);
    pub fn pm_runtime_get_suppliers(dev: *mut device);
    pub fn pm_runtime_put_suppliers(dev: *mut device);
    pub fn pm_runtime_new_link(dev: *mut device);
    pub fn pm_runtime_drop_link(link: *mut device_link);
    pub fn pm_runtime_release_supplier(link: *mut device_link);
    pub fn devm_pm_runtime_set_active_enabled(dev: *mut device) -> ::core::ffi::c_int;
    pub fn devm_pm_runtime_enable(dev: *mut device) -> ::core::ffi::c_int;
    pub fn devm_pm_runtime_get_noresume(dev: *mut device) -> ::core::ffi::c_int;
    pub fn pm_runtime_suspended_time(dev: *mut device) -> u64;
}

#[cfg(not(CONFIG_PM))]
pub unsafe fn queue_pm_work(_work: *mut work_struct) -> bool { false }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_generic_runtime_suspend(_: *mut device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_generic_runtime_resume(_: *mut device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_force_suspend(_: *mut device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn __pm_runtime_idle(_: *mut device, _: ::core::ffi::c_int) -> ::core::ffi::c_int { -ENOSYS }
#[cfg(not(CONFIG_PM))] pub unsafe fn __pm_runtime_suspend(_: *mut device, _: ::core::ffi::c_int) -> ::core::ffi::c_int { -ENOSYS }
#[cfg(not(CONFIG_PM))] pub unsafe fn __pm_runtime_resume(_: *mut device, _: ::core::ffi::c_int) -> ::core::ffi::c_int { 1 }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_schedule_suspend(_: *mut device, _: u32) -> ::core::ffi::c_int { -ENOSYS }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_get_if_in_use(_: *mut device) -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_get_if_active(_: *mut device) -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(CONFIG_PM))] pub unsafe fn __pm_runtime_set_status(_: *mut device, _: u32) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_barrier(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_block_if_disabled(_: *mut device) -> bool { true }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_unblock(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_enable(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn __pm_runtime_disable(_: *mut device, _: bool) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_allow(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_forbid(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn devm_pm_runtime_set_active_enabled(_: *mut device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn devm_pm_runtime_enable(_: *mut device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn devm_pm_runtime_get_noresume(_: *mut device) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_no_callbacks(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_irq_safe(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_is_irq_safe(_: *mut device) -> bool { false }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_has_no_callbacks(_: *mut device) -> bool { false }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_mark_last_busy(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn __pm_runtime_use_autosuspend(_: *mut device, _: bool) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_set_autosuspend_delay(_: *mut device, _: ::core::ffi::c_int) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_autosuspend_expiration(_: *mut device) -> u64 { 0 }
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_set_memalloc_noio(_: *mut device, _: bool) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_get_suppliers(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_put_suppliers(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_new_link(_: *mut device) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_drop_link(_: *mut device_link) {}
#[cfg(not(CONFIG_PM))] pub unsafe fn pm_runtime_release_supplier(_: *mut device_link) {}

#[cfg(CONFIG_PM_SLEEP)] extern "C" { pub fn pm_runtime_need_not_resume(dev: *mut device) -> bool; pub fn pm_runtime_force_resume(dev: *mut device) -> ::core::ffi::c_int; }
#[cfg(not(CONFIG_PM_SLEEP))] pub unsafe fn pm_runtime_need_not_resume(_: *mut device) -> bool { true }
#[cfg(not(CONFIG_PM_SLEEP))] pub unsafe fn pm_runtime_force_resume(_: *mut device) -> ::core::ffi::c_int { -ENXIO }

#[inline] pub unsafe fn pm_suspend_ignore_children(dev: *mut device, enable: bool) { (*dev).power.ignore_children = enable; }
#[inline] pub unsafe fn pm_runtime_get_noresume(dev: *mut device) { atomic_inc(&mut (*dev).power.usage_count); }
#[inline] pub unsafe fn pm_runtime_put_noidle(dev: *mut device) { atomic_add_unless(&mut (*dev).power.usage_count, -1, 0); }
#[inline] pub unsafe fn pm_runtime_suspended(dev: *mut device) -> bool { (*dev).power.runtime_status == RPM_SUSPENDED && (*dev).power.disable_depth == 0 }
#[inline] pub unsafe fn pm_runtime_active(dev: *mut device) -> bool { (*dev).power.runtime_status == RPM_ACTIVE || (*dev).power.disable_depth != 0 }
#[inline] pub unsafe fn pm_runtime_status_suspended(dev: *mut device) -> bool { (*dev).power.runtime_status == RPM_SUSPENDED }
#[inline] pub unsafe fn pm_runtime_enabled(dev: *mut device) -> bool { (*dev).power.disable_depth == 0 }
#[inline] pub unsafe fn pm_runtime_blocked(dev: *mut device) -> bool { (*dev).power.last_status == RPM_BLOCKED }
#[inline] pub unsafe fn pm_runtime_has_no_callbacks(dev: *mut device) -> bool { (*dev).power.no_callbacks }
#[inline] pub unsafe fn pm_runtime_mark_last_busy(dev: *mut device) { WRITE_ONCE!((*dev).power.last_busy, ktime_get_mono_fast_ns()); }
#[inline] pub unsafe fn pm_runtime_is_irq_safe(dev: *mut device) -> bool { (*dev).power.irq_safe }

#[inline] pub unsafe fn pm_runtime_idle(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_idle(dev, 0) }
#[inline] pub unsafe fn pm_runtime_suspend(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_suspend(dev, 0) }
#[inline] pub unsafe fn pm_runtime_autosuspend(dev: *mut device) -> ::core::ffi::c_int { pm_runtime_mark_last_busy(dev); __pm_runtime_suspend(dev, RPM_AUTO) }
#[inline] pub unsafe fn pm_runtime_resume(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_resume(dev, 0) }
#[inline] pub unsafe fn pm_request_idle(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_idle(dev, RPM_ASYNC) }
#[inline] pub unsafe fn pm_request_resume(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_resume(dev, RPM_ASYNC) }
#[inline] pub unsafe fn pm_request_autosuspend(dev: *mut device) -> ::core::ffi::c_int { pm_runtime_mark_last_busy(dev); __pm_runtime_suspend(dev, RPM_ASYNC | RPM_AUTO) }
#[inline] pub unsafe fn pm_runtime_get(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_resume(dev, RPM_GET_PUT | RPM_ASYNC) }
#[inline] pub unsafe fn pm_runtime_get_sync(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_resume(dev, RPM_GET_PUT) }
#[inline] pub unsafe fn pm_runtime_get_active(dev: *mut device, rpmflags: ::core::ffi::c_int) -> ::core::ffi::c_int { let ret = __pm_runtime_resume(dev, RPM_GET_PUT | rpmflags); if ret < 0 { pm_runtime_put_noidle(dev); return ret; } 0 }
#[inline] pub unsafe fn pm_runtime_resume_and_get(dev: *mut device) -> ::core::ffi::c_int { pm_runtime_get_active(dev, 0) }
#[inline] pub unsafe fn pm_runtime_put(dev: *mut device) { __pm_runtime_idle(dev, RPM_GET_PUT | RPM_ASYNC); }
#[inline] pub unsafe fn __pm_runtime_put_autosuspend(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_suspend(dev, RPM_GET_PUT | RPM_ASYNC | RPM_AUTO) }
#[inline] pub unsafe fn pm_runtime_put_autosuspend(dev: *mut device) -> ::core::ffi::c_int { pm_runtime_mark_last_busy(dev); __pm_runtime_put_autosuspend(dev) }
#[inline] pub unsafe fn pm_runtime_put_sync(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_idle(dev, RPM_GET_PUT) }
#[inline] pub unsafe fn pm_runtime_put_sync_suspend(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_suspend(dev, RPM_GET_PUT) }
#[inline] pub unsafe fn pm_runtime_put_sync_autosuspend(dev: *mut device) -> ::core::ffi::c_int { pm_runtime_mark_last_busy(dev); __pm_runtime_suspend(dev, RPM_GET_PUT | RPM_AUTO) }
#[inline] pub unsafe fn pm_runtime_set_active(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_set_status(dev, RPM_ACTIVE) }
#[inline] pub unsafe fn pm_runtime_set_suspended(dev: *mut device) -> ::core::ffi::c_int { __pm_runtime_set_status(dev, RPM_SUSPENDED) }
#[inline] pub unsafe fn pm_runtime_disable(dev: *mut device) { __pm_runtime_disable(dev, true); }
#[inline] pub unsafe fn pm_runtime_use_autosuspend(dev: *mut device) { __pm_runtime_use_autosuspend(dev, true); }
#[inline] pub unsafe fn pm_runtime_dont_use_autosuspend(dev: *mut device) { __pm_runtime_use_autosuspend(dev, false); }

// DEFINE_GUARD, DEFINE_GUARD_COND, and ACQUIRE wrapper macros are retained as
// source-level macro markers because their implementations are external.
macro_rules! DEFINE_GUARD { ($($t:tt)*) => {}; }
macro_rules! DEFINE_GUARD_COND { ($($t:tt)*) => {}; }
macro_rules! PM_RUNTIME_ACQUIRE { ($($t:tt)*) => {}; }
macro_rules! PM_RUNTIME_ACQUIRE_AUTOSUSPEND { ($($t:tt)*) => {}; }
macro_rules! PM_RUNTIME_ACQUIRE_IF_ENABLED { ($($t:tt)*) => {}; }
macro_rules! PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND { ($($t:tt)*) => {}; }
macro_rules! PM_RUNTIME_ACQUIRE_ERR { ($($t:tt)*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
