// Translated from rtc.h.
// TRACE_SYSTEM: rtc
// The original include guard and tracepoint includes are intentionally omitted;
// their declarations are supplied by the surrounding translation unit.

use core::ffi::c_void;

// C dependency types supplied by the Linux RTC and tracepoint headers.
pub type Time64T = i64;
pub type KtimeT = i64;

#[repr(C)]
pub struct RtcTimer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RtcTimeAlarmEntry {
    pub secs: Time64T,
    pub err: i32,
}

#[repr(C)]
pub struct RtcIrqSetFreqEntry {
    pub freq: i32,
    pub err: i32,
}

#[repr(C)]
pub struct RtcIrqSetStateEntry {
    pub enabled: i32,
    pub err: i32,
}

#[repr(C)]
pub struct RtcAlarmIrqEnableEntry {
    pub enabled: u32,
    pub err: i32,
}

#[repr(C)]
pub struct RtcOffsetEntry {
    pub offset: isize,
    pub err: i32,
}

#[repr(C)]
pub struct RtcTimerEntry {
    pub timer: *mut RtcTimer,
    pub expires: KtimeT,
    pub period: KtimeT,
}

// DECLARE_EVENT_CLASS(rtc_time_alarm_class):
// TP_fast_assign assigns secs and err; TP_printk is "UTC (%lld) (%d)".
// DEFINE_EVENT instances:
extern "C" {
    pub fn rtc_set_time(secs: Time64T, err: i32);
    pub fn rtc_read_time(secs: Time64T, err: i32);
    pub fn rtc_set_alarm(secs: Time64T, err: i32);
    pub fn rtc_read_alarm(secs: Time64T, err: i32);

    // TRACE_EVENT(rtc_irq_set_freq):
    // TP_fast_assign assigns freq and err; TP_printk is
    // "set RTC periodic IRQ frequency:%u (%d)".
    pub fn rtc_irq_set_freq(freq: i32, err: i32);

    // TRACE_EVENT(rtc_irq_set_state):
    // TP_fast_assign assigns enabled and err; TP_printk selects "enable"
    // when enabled is nonzero and "disable" otherwise.
    pub fn rtc_irq_set_state(enabled: i32, err: i32);

    // TRACE_EVENT(rtc_alarm_irq_enable):
    // TP_fast_assign assigns enabled and err; TP_printk selects "enable"
    // when enabled is nonzero and "disable" otherwise.
    pub fn rtc_alarm_irq_enable(enabled: u32, err: i32);

    // DECLARE_EVENT_CLASS(rtc_offset_class):
    // TP_fast_assign assigns offset and err; TP_printk is
    // "RTC offset: %ld (%d)".
    // DEFINE_EVENT instances:
    pub fn rtc_set_offset(offset: isize, err: i32);
    pub fn rtc_read_offset(offset: isize, err: i32);

    // DECLARE_EVENT_CLASS(rtc_timer_class):
    // TP_fast_assign assigns timer, timer->node.expires, and timer->period;
    // TP_printk is "RTC timer:(%p) expires:%lld period:%lld".
    pub fn rtc_timer_enqueue(timer: *mut RtcTimer);
    pub fn rtc_timer_dequeue(timer: *mut RtcTimer);
    pub fn rtc_timer_fired(timer: *mut RtcTimer);
}

// The original file includes <trace/define_trace.h> outside the guard.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
