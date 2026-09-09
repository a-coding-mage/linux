/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Power management wakeup interface. */

/* C dependencies: linux/types.h and the device/power-management declarations. */

use core::ffi::c_char;

#[repr(C)]
pub struct wake_irq { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct timer_list { _private: [u8; 0] }
#[repr(C)]
pub struct ktime_t { _private: [u8; 0] }

/* The definitions of device and its power member are supplied by the device header. */
#[repr(C)]
pub struct device { _private: [u8; 0] }

#[repr(C)]
pub struct wakeup_source {
    pub name: *const c_char,
    pub id: core::ffi::c_int,
    pub entry: list_head,
    pub lock: spinlock_t,
    pub wakeirq: *mut wake_irq,
    pub timer: timer_list,
    pub timer_expires: usize,
    pub total_time: ktime_t,
    pub max_time: ktime_t,
    pub last_time: ktime_t,
    pub start_prevent_time: ktime_t,
    pub prevent_sleep_time: ktime_t,
    pub event_count: usize,
    pub active_count: usize,
    pub relax_count: usize,
    pub expire_count: usize,
    pub wakeup_count: usize,
    pub dev: *mut device,
    pub active: bool,
    pub autosleep_enabled: bool,
}

extern "C" {
    pub fn wakeup_source_register(dev: *mut device, name: *const c_char) -> *mut wakeup_source;
    pub fn wakeup_source_unregister(ws: *mut wakeup_source);
    pub fn wakeup_sources_read_lock() -> core::ffi::c_int;
    pub fn wakeup_sources_read_unlock(idx: core::ffi::c_int);
    pub fn wakeup_sources_walk_start() -> *mut wakeup_source;
    pub fn wakeup_sources_walk_next(ws: *mut wakeup_source) -> *mut wakeup_source;
    pub fn device_wakeup_enable(dev: *mut device) -> core::ffi::c_int;
    pub fn device_wakeup_disable(dev: *mut device);
    pub fn device_set_wakeup_capable(dev: *mut device, capable: bool);
    pub fn device_set_wakeup_enable(dev: *mut device, enable: bool) -> core::ffi::c_int;
    pub fn __pm_stay_awake(ws: *mut wakeup_source);
    pub fn pm_stay_awake(dev: *mut device);
    pub fn __pm_relax(ws: *mut wakeup_source);
    pub fn pm_relax(dev: *mut device);
    pub fn pm_wakeup_ws_event(ws: *mut wakeup_source, msec: u32, hard: bool);
    pub fn pm_wakeup_dev_event(dev: *mut device, msec: u32, hard: bool);
}

/* CONFIG_PM_SLEEP selects the external implementations above; without it the
 * following inline stubs are used.  The device power-field accesses remain
 * dependent on the corresponding device header. */

#[inline]
pub unsafe fn device_awake_path(dev: *mut device) -> bool { device_wakeup_path(dev) }
#[inline]
pub unsafe fn device_set_awake_path(dev: *mut device) { device_set_wakeup_path(dev); }
#[inline]
pub unsafe fn __pm_wakeup_event(ws: *mut wakeup_source, msec: u32) { pm_wakeup_ws_event(ws, msec, false); }
#[inline]
pub unsafe fn pm_wakeup_event(dev: *mut device, msec: u32) { pm_wakeup_dev_event(dev, msec, false); }
#[inline]
pub unsafe fn pm_wakeup_hard_event(dev: *mut device) { pm_wakeup_dev_event(dev, 0, true); }

/* Device power fields are represented by these dependency-provided operations. */
extern "C" {
    fn device_can_wakeup(dev: *mut device) -> bool;
    fn device_may_wakeup(dev: *mut device) -> bool;
    fn device_wakeup_path(dev: *mut device) -> bool;
    fn device_set_wakeup_path(dev: *mut device);
    fn device_set_out_band_wakeup(dev: *mut device);
    fn device_out_band_wakeup(dev: *mut device) -> bool;
}

#[inline]
pub unsafe fn device_init_wakeup(dev: *mut device, enable: bool) -> core::ffi::c_int {
    if enable {
        device_set_wakeup_capable(dev, true);
        device_wakeup_enable(dev)
    } else {
        device_wakeup_disable(dev);
        device_set_wakeup_capable(dev, false);
        0
    }
}

unsafe extern "C" fn device_disable_wakeup(dev: *mut core::ffi::c_void) {
    device_init_wakeup(dev as *mut device, false);
}

/* devm_add_action_or_reset is supplied by the device-management dependency. */
extern "C" {
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut core::ffi::c_void),
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn devm_device_init_wakeup(dev: *mut device) -> core::ffi::c_int {
    device_init_wakeup(dev, true);
    devm_add_action_or_reset(dev, device_disable_wakeup, dev as *mut core::ffi::c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
