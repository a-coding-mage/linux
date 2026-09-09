// SPDX-License-Identifier: GPL-2.0
/* Alarm timer interface; translated from alarmtimer.c. */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit. Their symbols are intentionally left as external names.

#[repr(C)]
struct AlarmBase {
    lock: spinlock_t,
    timerqueue: timerqueue_head,
    get_ktime: Option<unsafe extern "C" fn() -> ktime_t>,
    get_timespec: Option<unsafe extern "C" fn(*mut timespec64)>,
    base_clockid: clockid_t,
}

static mut alarm_bases: [AlarmBase; ALARM_NUMTYPE] = [AlarmBase {
    lock: unsafe { core::mem::zeroed() }, timerqueue: unsafe { core::mem::zeroed() },
    get_ktime: None, get_timespec: None, base_clockid: 0,
}; ALARM_NUMTYPE];

#[cfg(any(CONFIG_POSIX_TIMERS, CONFIG_RTC_CLASS))]
static mut freezer_alarmtype: alarmtimer_type = 0;
#[cfg(any(CONFIG_POSIX_TIMERS, CONFIG_RTC_CLASS))]
static mut freezer_expires: ktime_t = 0;
#[cfg(any(CONFIG_POSIX_TIMERS, CONFIG_RTC_CLASS))]
static mut freezer_delta: ktime_t = 0;
#[cfg(any(CONFIG_POSIX_TIMERS, CONFIG_RTC_CLASS))]
static mut freezer_delta_lock: spinlock_t = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_RTC_CLASS)]
static mut rtctimer: rtc_timer = unsafe { core::mem::zeroed() };
#[cfg(CONFIG_RTC_CLASS)]
static mut rtcdev: *mut rtc_device = core::ptr::null_mut();
#[cfg(CONFIG_RTC_CLASS)]
static mut rtcdev_lock: spinlock_t = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_RTC_CLASS)]
#[no_mangle]
pub unsafe extern "C" fn alarmtimer_get_rtcdev() -> *mut rtc_device {
    let _guard = rtcdev_lock;
    rtcdev
}

#[cfg(CONFIG_RTC_CLASS)]
unsafe extern "C" fn alarmtimer_rtc_add_device(dev: *mut device) -> c_int {
    let rtc = to_rtc_device(dev);
    let mut pdev: *mut platform_device;
    let mut ret = 0;
    if !rtcdev.is_null() { return -EBUSY; }
    if !test_bit(RTC_FEATURE_ALARM, (*rtc).features) || !device_may_wakeup((*rtc).dev.parent) { return -1; }
    pdev = platform_device_register_data(dev, b"alarmtimer\0".as_ptr() as *const _, PLATFORM_DEVID_AUTO, core::ptr::null(), 0);
    if !IS_ERR(pdev) { device_init_wakeup(&mut (*pdev).dev, true); }
    if !IS_ERR(pdev) && rtcdev.is_null() && try_module_get((*rtc).owner) {
        rtcdev = rtc; get_device(dev); pdev = core::ptr::null_mut();
    } else { ret = -1; }
    platform_device_unregister(pdev); ret
}

#[cfg(CONFIG_RTC_CLASS)]
unsafe extern "C" fn alarmtimer_rtc_timer_init() { rtc_timer_init(&mut rtctimer, None, None); }
#[cfg(not(CONFIG_RTC_CLASS))]
unsafe extern "C" fn alarmtimer_rtc_timer_init() {}
#[cfg(CONFIG_RTC_CLASS)]
unsafe extern "C" fn alarmtimer_rtc_interface_setup() -> c_int { alarmtimer_rtc_timer_init(); class_interface_register(&mut alarmtimer_rtc_interface) }
#[cfg(not(CONFIG_RTC_CLASS))]
unsafe extern "C" fn alarmtimer_rtc_interface_setup() -> c_int { 0 }
#[cfg(not(CONFIG_RTC_CLASS))]
unsafe extern "C" fn alarmtimer_rtc_interface_remove() {}
#[cfg(CONFIG_RTC_CLASS)]
unsafe extern "C" fn alarmtimer_rtc_interface_remove() { class_interface_unregister(&mut alarmtimer_rtc_interface); }

unsafe extern "C" fn alarmtimer_enqueue(base: *mut AlarmBase, alarm: *mut alarm) {
    if (*alarm).state & ALARMTIMER_STATE_ENQUEUED != 0 { timerqueue_del(&mut (*base).timerqueue, &mut (*alarm).node); }
    timerqueue_add(&mut (*base).timerqueue, &mut (*alarm).node);
    (*alarm).state |= ALARMTIMER_STATE_ENQUEUED;
}
unsafe extern "C" fn alarmtimer_dequeue(base: *mut AlarmBase, alarm: *mut alarm) {
    if (*alarm).state & ALARMTIMER_STATE_ENQUEUED == 0 { return; }
    timerqueue_del(&mut (*base).timerqueue, &mut (*alarm).node);
    (*alarm).state &= !ALARMTIMER_STATE_ENQUEUED;
}
unsafe extern "C" fn alarmtimer_fired(timer: *mut hrtimer) -> hrtimer_restart {
    let alarm = container_of!(timer, alarm, timer);
    let base = &mut alarm_bases[(*alarm).type_ as usize] as *mut AlarmBase;
    alarmtimer_dequeue(base, alarm);
    if let Some(f) = (*alarm).function { f(alarm, ((*base).get_ktime.unwrap())()); }
    trace_alarmtimer_fired(alarm, ((*base).get_ktime.unwrap())()); HRTIMER_NORESTART
}

#[no_mangle]
pub unsafe extern "C" fn alarm_expires_remaining(alarm: *const alarm) -> ktime_t {
    let base = &alarm_bases[(*alarm).type_ as usize]; ktime_sub((*alarm).node.expires, (base.get_ktime.unwrap())())
}

unsafe extern "C" fn __alarm_init(alarm: *mut alarm, type_: alarmtimer_type, function: Option<unsafe extern "C" fn(*mut alarm, ktime_t)>) {
    timerqueue_init(&mut (*alarm).node); (*alarm).function = function; (*alarm).type_ = type_; (*alarm).state = ALARMTIMER_STATE_INACTIVE;
}
#[no_mangle]
pub unsafe extern "C" fn alarm_init(alarm: *mut alarm, type_: alarmtimer_type, function: Option<unsafe extern "C" fn(*mut alarm, ktime_t)>) {
    hrtimer_setup(&mut (*alarm).timer, Some(alarmtimer_fired), alarm_bases[type_ as usize].base_clockid, HRTIMER_MODE_ABS);
    __alarm_init(alarm, type_, function);
}
#[no_mangle]
pub unsafe extern "C" fn alarm_start_timer(alarm: *mut alarm, mut expires: ktime_t, relative: bool) -> bool {
    let base = &mut alarm_bases[(*alarm).type_ as usize];
    if relative { expires = ktime_add_safe(expires, (base.get_ktime.unwrap())()); }
    trace_alarmtimer_start(alarm, (base.get_ktime.unwrap())()); (*alarm).node.expires = expires; alarmtimer_enqueue(base, alarm);
    if !hrtimer_start_range_ns_user(&mut (*alarm).timer, expires, 0, HRTIMER_MODE_ABS) { alarmtimer_dequeue(base, alarm); return false; } true
}
#[no_mangle]
pub unsafe extern "C" fn alarm_try_to_cancel(alarm: *mut alarm) -> c_int {
    let base = &mut alarm_bases[(*alarm).type_ as usize]; let ret = hrtimer_try_to_cancel(&mut (*alarm).timer);
    if ret >= 0 { alarmtimer_dequeue(base, alarm); } trace_alarmtimer_cancel(alarm, (base.get_ktime.unwrap())()); ret
}
#[no_mangle]
pub unsafe extern "C" fn alarm_cancel(alarm: *mut alarm) -> c_int {
    loop { let ret = alarm_try_to_cancel(alarm); if ret >= 0 { return ret; } hrtimer_cancel_wait_running(&mut (*alarm).timer); }
}
#[no_mangle]
pub unsafe extern "C" fn alarm_forward(alarm: *mut alarm, now: ktime_t, interval: ktime_t) -> u64 {
    let mut overrun = 1; let delta = ktime_sub(now, (*alarm).node.expires); if delta < 0 { return 0; }
    if delta >= interval { let incr = ktime_to_ns(interval); overrun = ktime_divns(delta, incr); (*alarm).node.expires = ktime_add_ns((*alarm).node.expires, incr * overrun as i64); if (*alarm).node.expires > now { return overrun; } overrun += 1; }
    (*alarm).node.expires = ktime_add_safe((*alarm).node.expires, interval); overrun
}
#[no_mangle]
pub unsafe extern "C" fn alarm_forward_now(alarm: *mut alarm, interval: ktime_t) -> u64 { let base = &alarm_bases[(*alarm).type_ as usize]; alarm_forward(alarm, (base.get_ktime.unwrap())(), interval) }

#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn clock2alarm(clockid: clockid_t) -> alarmtimer_type {
    if clockid == CLOCK_REALTIME_ALARM { return ALARM_REALTIME; } WARN_ON_ONCE(clockid != CLOCK_BOOTTIME_ALARM); ALARM_BOOTTIME
}
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_handle_timer(alarm: *mut alarm, _now: ktime_t) {
    let ptr = container_of!(alarm, k_itimer, it.alarm.alarmtimer); posix_timer_queue_signal(ptr);
}
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_timer_rearm(timr: *mut k_itimer) -> bool {
    let alarm = &mut (*timr).it.alarm.alarmtimer; (*timr).it_overrun += alarm_forward_now(alarm, (*timr).it_interval); alarm_start_timer(alarm, (*alarm).node.expires, false)
}
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_timer_forward(timr: *mut k_itimer, now: ktime_t) -> i64 { alarm_forward(&mut (*timr).it.alarm.alarmtimer, now, (*timr).it_interval) as i64 }
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_timer_remaining(timr: *mut k_itimer, now: ktime_t) -> ktime_t { ktime_sub((*timr).it.alarm.alarmtimer.node.expires, now) }
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_timer_try_to_cancel(timr: *mut k_itimer) -> c_int { alarm_try_to_cancel(&mut (*timr).it.alarm.alarmtimer) }
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_timer_wait_running(timr: *mut k_itimer) { hrtimer_cancel_wait_running(&mut (*timr).it.alarm.alarmtimer.timer); }
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_timer_arm(timr: *mut k_itimer, mut expires: ktime_t, absolute: bool, sigev_none: bool) -> bool {
    let alarm = &mut (*timr).it.alarm.alarmtimer; let base = &alarm_bases[(*alarm).type_ as usize];
    if !absolute { expires = ktime_add_safe(expires, (base.get_ktime.unwrap())()); }
    if sigev_none { alarm.node.expires = expires; true } else { alarm_start_timer(alarm, expires, false) }
}
#[cfg(CONFIG_POSIX_TIMERS)]
unsafe extern "C" fn alarm_clock_getres(_clock: clockid_t, tp: *mut timespec64) -> c_int {
    if alarmtimer_get_rtcdev().is_null() { return -EINVAL; } (*tp).tv_sec = 0; (*tp).tv_nsec = hrtimer_resolution; 0
}
#[cfg(CONFIG_POSIX_TIMERS)]
pub static mut alarm_clock: k_clock = unsafe { core::mem::zeroed() };


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
