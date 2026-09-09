/* SPDX-License-Identifier: GPL-2.0 */

pub const TIMER_RETRY: ::core::ffi::c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum posix_timer_state {
    POSIX_TIMER_DISARMED,
    POSIX_TIMER_ARMED,
    POSIX_TIMER_REQUEUE_PENDING,
}

#[repr(C)]
pub struct k_clock {
    pub clock_getres: Option<unsafe extern "C" fn(which_clock: clockid_t, tp: *mut timespec64) -> ::core::ffi::c_int>,
    pub clock_set: Option<unsafe extern "C" fn(which_clock: clockid_t, tp: *const timespec64) -> ::core::ffi::c_int>,
    /* Returns the clock value in the current time namespace. */
    pub clock_get_timespec: Option<unsafe extern "C" fn(which_clock: clockid_t, tp: *mut timespec64) -> ::core::ffi::c_int>,
    /* Returns the clock value in the root time namespace. */
    pub clock_get_ktime: Option<unsafe extern "C" fn(which_clock: clockid_t) -> ktime_t>,
    pub clock_adj: Option<unsafe extern "C" fn(which_clock: clockid_t, tx: *mut __kernel_timex) -> ::core::ffi::c_int>,
    pub timer_create: Option<unsafe extern "C" fn(timer: *mut k_itimer) -> ::core::ffi::c_int>,
    pub nsleep: Option<unsafe extern "C" fn(which_clock: clockid_t, flags: ::core::ffi::c_int, _: *const timespec64) -> ::core::ffi::c_int>,
    pub timer_set: Option<unsafe extern "C" fn(timr: *mut k_itimer, flags: ::core::ffi::c_int, new_setting: *mut itimerspec64, old_setting: *mut itimerspec64) -> ::core::ffi::c_int>,
    pub timer_del: Option<unsafe extern "C" fn(timr: *mut k_itimer) -> ::core::ffi::c_int>,
    pub timer_get: Option<unsafe extern "C" fn(timr: *mut k_itimer, cur_setting: *mut itimerspec64)>,
    pub timer_rearm: Option<unsafe extern "C" fn(timr: *mut k_itimer) -> bool>,
    pub timer_forward: Option<unsafe extern "C" fn(timr: *mut k_itimer, now: ktime_t) -> s64>,
    pub timer_remaining: Option<unsafe extern "C" fn(timr: *mut k_itimer, now: ktime_t) -> ktime_t>,
    pub timer_try_to_cancel: Option<unsafe extern "C" fn(timr: *mut k_itimer) -> ::core::ffi::c_int>,
    pub timer_arm: Option<unsafe extern "C" fn(timr: *mut k_itimer, expires: ktime_t, absolute: bool, sigev_none: bool) -> bool>,
    pub timer_wait_running: Option<unsafe extern "C" fn(timr: *mut k_itimer)>,
}

extern "C" {
    pub static clock_posix_cpu: k_clock;
    pub static clock_posix_dynamic: k_clock;
    pub static clock_process: k_clock;
    pub static clock_thread: k_clock;
    pub static alarm_clock: k_clock;
    pub static clock_aux: k_clock;

    pub fn posix_timer_queue_signal(timr: *mut k_itimer);

    pub fn common_timer_get(timr: *mut k_itimer, cur_setting: *mut itimerspec64);
    pub fn common_timer_set(
        timr: *mut k_itimer,
        flags: ::core::ffi::c_int,
        new_setting: *mut itimerspec64,
        old_setting: *mut itimerspec64,
    ) -> ::core::ffi::c_int;
    pub fn posix_timer_set_common(timer: *mut k_itimer, new_setting: *mut itimerspec64);
    pub fn common_timer_del(timer: *mut k_itimer) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
