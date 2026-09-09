/* SPDX-License-Identifier: GPL-2.0 */
/* Generic watchdog definitions. C header guards and includes omitted. */

use core::ffi::c_void;

pub enum attribute_group {}
pub enum device {}
pub enum module {}
pub enum watchdog_governor {}
pub enum watchdog_core_data {}
pub enum watchdog_info {}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct watchdog_ops {
    pub owner: *mut module,
    pub start: Option<unsafe extern "C" fn(*mut watchdog_device) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut watchdog_device) -> i32>,
    pub ping: Option<unsafe extern "C" fn(*mut watchdog_device) -> i32>,
    pub status: Option<unsafe extern "C" fn(*mut watchdog_device) -> u32>,
    pub set_timeout: Option<unsafe extern "C" fn(*mut watchdog_device, u32) -> i32>,
    pub set_pretimeout: Option<unsafe extern "C" fn(*mut watchdog_device, u32) -> i32>,
    pub get_timeleft: Option<unsafe extern "C" fn(*mut watchdog_device) -> u32>,
    pub restart: Option<unsafe extern "C" fn(*mut watchdog_device, c_ulong, *mut c_void) -> i32>,
    pub ioctl: Option<unsafe extern "C" fn(*mut watchdog_device, u32, c_ulong) -> c_long>,
}

/* Bit numbers for status flags. */
pub const WDOG_ACTIVE: u32 = 0;
pub const WDOG_NO_WAY_OUT: u32 = 1;
pub const WDOG_STOP_ON_REBOOT: u32 = 2;
pub const WDOG_HW_RUNNING: u32 = 3;
pub const WDOG_STOP_ON_UNREGISTER: u32 = 4;
pub const WDOG_NO_PING_ON_SUSPEND: u32 = 5;

#[repr(C)]
pub struct watchdog_device {
    pub id: i32,
    pub parent: *mut device,
    pub groups: *const *const attribute_group,
    pub info: *const watchdog_info,
    pub ops: *const watchdog_ops,
    pub gov: *const watchdog_governor,
    pub bootstatus: u32,
    pub timeout: u32,
    pub pretimeout: u32,
    pub min_timeout: u32,
    pub max_timeout: u32,
    pub min_hw_heartbeat_ms: u32,
    pub max_hw_heartbeat_ms: u32,
    pub reboot_nb: notifier_block,
    pub restart_nb: notifier_block,
    pub pm_nb: notifier_block,
    pub driver_data: *mut c_void,
    pub wd_data: *mut watchdog_core_data,
    pub status: c_ulong,
    pub deferred: list_head,
}

/* Build-time CONFIG_WATCHDOG_NOWAYOUT / IS_BUILTIN condition. */
#[cfg(feature = "CONFIG_WATCHDOG_NOWAYOUT")]
pub const WATCHDOG_NOWAYOUT: bool = true;
#[cfg(not(feature = "CONFIG_WATCHDOG_NOWAYOUT"))]
pub const WATCHDOG_NOWAYOUT: bool = false;
pub const WATCHDOG_NOWAYOUT_INIT_STATUS: c_ulong =
    (WATCHDOG_NOWAYOUT as c_ulong) << WDOG_NO_WAY_OUT;

extern "C" {
    pub fn test_bit(nr: u32, addr: *const c_ulong) -> bool;
    pub fn set_bit(nr: u32, addr: *mut c_ulong);
}

#[inline]
pub unsafe fn watchdog_active(wdd: *const watchdog_device) -> bool {
    test_bit(WDOG_ACTIVE, &(*wdd).status)
}

#[inline]
pub unsafe fn watchdog_hw_running(wdd: *const watchdog_device) -> bool {
    test_bit(WDOG_HW_RUNNING, &(*wdd).status)
}

#[inline]
pub unsafe fn watchdog_set_nowayout(wdd: *mut watchdog_device, nowayout: bool) {
    if nowayout { set_bit(WDOG_NO_WAY_OUT, &mut (*wdd).status); }
}

#[inline]
pub unsafe fn watchdog_stop_on_reboot(wdd: *mut watchdog_device) {
    set_bit(WDOG_STOP_ON_REBOOT, &mut (*wdd).status);
}

#[inline]
pub unsafe fn watchdog_stop_on_unregister(wdd: *mut watchdog_device) {
    set_bit(WDOG_STOP_ON_UNREGISTER, &mut (*wdd).status);
}

#[inline]
pub unsafe fn watchdog_stop_ping_on_suspend(wdd: *mut watchdog_device) {
    set_bit(WDOG_NO_PING_ON_SUSPEND, &mut (*wdd).status);
}

#[inline]
pub unsafe fn watchdog_timeout_invalid(wdd: *const watchdog_device, t: u32) -> bool {
    t > u32::MAX / 1000 || t < (*wdd).min_timeout
        || (!(*wdd).max_hw_heartbeat_ms && (*wdd).max_timeout
            && t > (*wdd).max_timeout)
}

#[inline]
pub unsafe fn watchdog_pretimeout_invalid(wdd: *const watchdog_device, t: u32) -> bool {
    t != 0 && (*wdd).timeout != 0 && t >= (*wdd).timeout
}

#[inline]
pub unsafe fn watchdog_set_drvdata(wdd: *mut watchdog_device, data: *mut c_void) {
    (*wdd).driver_data = data;
}

#[inline]
pub unsafe fn watchdog_get_drvdata(wdd: *mut watchdog_device) -> *mut c_void {
    (*wdd).driver_data
}

#[cfg(feature = "CONFIG_WATCHDOG_PRETIMEOUT_GOV")]
extern "C" { pub fn watchdog_notify_pretimeout(wdd: *mut watchdog_device); }
#[cfg(not(feature = "CONFIG_WATCHDOG_PRETIMEOUT_GOV"))]
pub unsafe fn watchdog_notify_pretimeout(_wdd: *mut watchdog_device) {
    /* C fallback calls pr_alert("watchdog%d: pretimeout event\n", wdd->id). */
}

extern "C" {
    pub fn watchdog_set_restart_priority(wdd: *mut watchdog_device, priority: i32);
    pub fn watchdog_init_timeout(wdd: *mut watchdog_device, timeout_parm: u32,
                                 dev: *const device) -> i32;
    pub fn watchdog_register_device(wdd: *mut watchdog_device) -> i32;
    pub fn watchdog_unregister_device(wdd: *mut watchdog_device);
    pub fn watchdog_dev_suspend(wdd: *mut watchdog_device) -> i32;
    pub fn watchdog_dev_resume(wdd: *mut watchdog_device) -> i32;
    pub fn watchdog_set_last_hw_keepalive(wdd: *mut watchdog_device, val: u32) -> i32;
    pub fn devm_watchdog_register_device(dev: *mut device, wdd: *mut watchdog_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
