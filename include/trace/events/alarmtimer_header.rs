/* SPDX-License-Identifier: GPL-2.0 */

// C translation of trace/events/alarmtimer.h.
// The Linux tracepoint macros and the types supplied by the included headers
// are external dependencies and are intentionally not reimplemented here.

pub const ALARM_REALTIME: u32 = 0;
pub const ALARM_BOOTTIME: u32 = 1;
pub const ALARM_REALTIME_FREEZER: u32 = 2;
pub const ALARM_BOOTTIME_FREEZER: u32 = 3;

/// Rust equivalent of show_alarm_type(type), preserving the trace formatter's
/// flag names and separator.
pub fn show_alarm_type(mut type_: u32) -> String {
    let mut result = String::new();
    let flags = [
        (1u32 << ALARM_REALTIME, "REALTIME"),
        (1u32 << ALARM_BOOTTIME, "BOOTTIME"),
        (1u32 << ALARM_REALTIME_FREEZER, "REALTIME Freezer"),
        (1u32 << ALARM_BOOTTIME_FREEZER, "BOOTTIME Freezer"),
    ];

    for (mask, name) in flags {
        if (type_ & mask) != 0 {
            if !result.is_empty() {
                result.push_str(" | ");
            }
            result.push_str(name);
            type_ &= !mask;
        }
    }
    result
}

#[cfg(CONFIG_RTC_CLASS)]
#[repr(C)]
pub struct AlarmtimerSuspendEntry {
    pub expires: i64,
    pub alarm_type: u8,
}

#[cfg(CONFIG_RTC_CLASS)]
#[inline]
pub fn alarmtimer_suspend_trace(expires: i64, flag: i32) -> AlarmtimerSuspendEntry {
    AlarmtimerSuspendEntry {
        expires,
        alarm_type: flag as u8,
    }
}

#[repr(C)]
pub struct AlarmClassEntry {
    pub alarm: *mut core::ffi::c_void,
    pub alarm_type: u8,
    pub expires: i64,
    pub now: i64,
}

// struct alarm and its node.expires field are supplied by linux/alarmtimer.h.
// The following trace-event declarations correspond to the C event class and
// its generated events; their registration and printing are external.
pub const ALARM_CLASS_NAME: &str = "alarm_class";
pub const ALARMTIMER_FIRED_EVENT: &str = "alarmtimer_fired";
pub const ALARMTIMER_START_EVENT: &str = "alarmtimer_start";
pub const ALARMTIMER_CANCEL_EVENT: &str = "alarmtimer_cancel";

// TP_PROTO(struct alarm *alarm, ktime_t now)
// TP_fast_assign:
//   __entry->alarm = alarm;
//   __entry->alarm_type = alarm->type;
//   __entry->expires = alarm->node.expires;
//   __entry->now = now;
// TP_printk("alarmtimer:%p type:%s expires:%llu now:%llu", ...)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
