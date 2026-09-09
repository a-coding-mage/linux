/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/time.h, linux/hrtimer.h, and linux/timerqueue.h.

use core::ffi::c_void;

#[repr(C)]
pub struct rtc_device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum alarmtimer_type {
    ALARM_REALTIME,
    ALARM_BOOTTIME,

    /* Supported types end here */
    ALARM_NUMTYPE,

    /* Used for tracing information. No usable types. */
    ALARM_REALTIME_FREEZER,
    ALARM_BOOTTIME_FREEZER,
}

pub const ALARMTIMER_STATE_INACTIVE: i32 = 0x00;
pub const ALARMTIMER_STATE_ENQUEUED: i32 = 0x01;

/* struct timerqueue_node, struct hrtimer, and ktime_t are supplied externally. */
#[repr(C)]
pub struct alarm {
    pub node: timerqueue_node,
    pub timer: hrtimer,
    pub function: Option<unsafe extern "C" fn(*mut alarm, ktime_t)>,
    pub type_: alarmtimer_type,
    pub state: i32,
    pub data: *mut c_void,
}

#[inline(always)]
pub unsafe fn alarm_get_expires(alarm: *mut alarm) -> ktime_t {
    (*alarm).node.expires
}

extern "C" {
    pub fn alarm_init(
        alarm: *mut alarm,
        type_: alarmtimer_type,
        function: Option<unsafe extern "C" fn(*mut alarm, ktime_t)>,
    );
    pub fn alarm_start_timer(alarm: *mut alarm, expires: ktime_t, relative: bool) -> bool;
    pub fn alarm_try_to_cancel(alarm: *mut alarm) -> i32;
    pub fn alarm_cancel(alarm: *mut alarm) -> i32;

    pub fn alarm_forward(alarm: *mut alarm, now: ktime_t, interval: ktime_t) -> u64;
    pub fn alarm_forward_now(alarm: *mut alarm, interval: ktime_t) -> u64;
    pub fn alarm_expires_remaining(alarm: *const alarm) -> ktime_t;

    #[cfg(CONFIG_RTC_CLASS)]
    pub fn alarmtimer_get_rtcdev() -> *mut rtc_device;
}

/* When CONFIG_RTC_CLASS is disabled, the C header provides a static inline NULL return. */
#[cfg(not(CONFIG_RTC_CLASS))]
#[inline]
pub unsafe fn alarmtimer_get_rtcdev() -> *mut rtc_device {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
