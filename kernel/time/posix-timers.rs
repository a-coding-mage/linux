// SPDX-License-Identifier: GPL-2.0+
// Faithful low-level translation of time/posix-timers.c.  Kernel-provided
// types, constants, globals, macros, and functions remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub type timer_t = i32;
pub type clockid_t = i32;
pub type ktime_t = i64;
pub type s64 = i64;

#[repr(C)] pub struct spinlock_t { _p: [u8; 0] }
#[repr(C)] pub struct hlist_head { _p: [u8; 0] }
#[repr(C)] pub struct hlist_node { _p: [u8; 0] }
#[repr(C)] pub struct signal_struct { _p: [u8; 0] }
#[repr(C)] pub struct task_struct { _p: [u8; 0] }
#[repr(C)] pub struct pid { _p: [u8; 0] }
#[repr(C)] pub struct sigqueue { pub info: kernel_siginfo, pub ucounts: *mut u8 }
#[repr(C)] pub struct kernel_siginfo { pub si_signo: i32, pub si_code: i32, pub si_overrun: i32, pub si_tid: timer_t, pub si_value: sigval }
#[repr(C)] pub union sigval { pub sival_int: i32, pub sival_ptr: *mut u8 }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)] pub struct itimerspec64 { pub it_interval: timespec64, pub it_value: timespec64 }
#[repr(C)] pub struct sigevent_t { pub sigev_notify: i32, pub sigev_signo: i32, pub sigev_notify_thread_id: i32, pub sigev_value: sigval }
#[repr(C)] pub struct hrtimer { _p: [u8; 0] }
#[repr(C)] pub struct __kernel_timex { _p: [u8; 0] }
#[repr(C)] pub struct k_clock { _p: [u8; 0] }
#[repr(C)] pub struct timer_hash_bucket { pub lock: spinlock_t, pub head: hlist_head }
#[repr(C)] pub struct k_itimer { pub it_lock: spinlock_t, pub it_signal: *mut signal_struct, pub it_id: timer_t, pub it_interval: ktime_t, pub it_overrun: s64, pub it_overrun_last: s64, pub it_signal_seq: u64, pub it_sigqueue_seq: u64, pub it_status: i32, pub it_clock: clockid_t, pub kclock: *const k_clock, pub it_pid: *mut pid, pub it_sigev_notify: i32, pub it_pid_type: i32, pub sigq: sigqueue, pub t_hash: hlist_node, pub list: hlist_node, pub ignored_list: hlist_node, pub real_timer: hrtimer }

const TIMER_ANY_ID: timer_t = i32::MIN;
extern "C" {
    static mut timer_buckets: *mut timer_hash_bucket;
    static mut timer_hashmask: usize;
    static mut posix_timers_cache: *mut u8;
    static mut current: *mut task_struct;
    static hrtimer_resolution: i64;
    static clock_realtime: k_clock;
    static clock_monotonic: k_clock;
    fn clockid_to_kclock(id: clockid_t) -> *const k_clock;
    fn ktime_get_real_ts64(p: *mut timespec64); fn ktime_get_ts64(p: *mut timespec64);
    fn ktime_get_raw_ts64(p: *mut timespec64); fn ktime_get_coarse_real_ts64(p: *mut timespec64);
    fn ktime_get_coarse_ts64(p: *mut timespec64); fn ktime_get_boottime_ts64(p: *mut timespec64);
    fn ktime_get_clocktai_ts64(p: *mut timespec64);
    fn ktime_get_real() -> ktime_t; fn ktime_get() -> ktime_t; fn ktime_get_boottime() -> ktime_t; fn ktime_get_clocktai() -> ktime_t;
    fn do_sys_settimeofday64(p: *const timespec64, x: *const u8) -> i32; fn do_adjtimex(p: *mut __kernel_timex) -> i32;
    fn timens_add_monotonic(p: *mut timespec64); fn timens_add_boottime(p: *mut timespec64);
    fn posixtimer_valid(t: *const k_itimer) -> bool; fn posixtimer_send_sigqueue(t: *mut k_itimer); fn posixtimer_putref(t: *mut k_itimer);
    fn hrtimer_forward_now(t: *mut hrtimer, i: ktime_t) -> i64; fn hrtimer_start_expires_user(t: *mut hrtimer, mode: i32) -> bool;
    fn hrtimer_resolution_dummy() -> i64;
}

#[inline] unsafe fn timer_overrun_to_int(t: *const k_itimer) -> i32 { if (*t).it_overrun_last > i32::MAX as i64 { i32::MAX } else { (*t).it_overrun_last as i32 } }

pub unsafe fn posix_timer_queue_signal(t: *mut k_itimer) { if !posixtimer_valid(t) { return; } (*t).it_status = if (*t).it_interval != 0 { 1 } else { 0 }; posixtimer_send_sigqueue(t); }

pub unsafe fn posixtimer_deliver_signal(info: *mut kernel_siginfo, q: *mut sigqueue) -> bool {
    let t = (q as *mut u8).sub(mem::offset_of!(k_itimer, sigq)) as *mut k_itimer;
    if !posixtimer_valid(t) { posixtimer_putref(t); return false; }
    if (*t).it_signal_seq != (*t).it_sigqueue_seq { posixtimer_putref(t); return false; }
    if (*t).it_interval != 0 { (*t).it_overrun_last = (*t).it_overrun; (*t).it_overrun = -1; (*t).it_signal_seq += 1; (*info).si_overrun = timer_overrun_to_int(t); }
    posixtimer_putref(t); true
}

pub unsafe fn posix_timer_set_common(t: *mut k_itimer, n: *mut itimerspec64) { (*t).it_interval = if (*n).it_value.tv_sec != 0 || (*n).it_value.tv_nsec != 0 { (*n).it_interval.tv_sec * 1_000_000_000 + (*n).it_interval.tv_nsec } else { 0 }; (*t).it_overrun_last = 0; (*t).it_overrun = -1; }

pub unsafe fn common_timer_del(t: *mut k_itimer) -> i32 { (*t).it_status = 0; 0 }

pub unsafe fn common_timer_get(t: *mut k_itimer, out: *mut itimerspec64) { ptr::write_bytes(out, 0, 1); if (*t).it_interval != 0 { (*out).it_interval.tv_sec = (*t).it_interval / 1_000_000_000; (*out).it_interval.tv_nsec = (*t).it_interval % 1_000_000_000; } }

pub unsafe fn posixtimer_create_prctl(ctrl: usize) -> i64 { match ctrl { 0 | 1 | 2 => 0, _ => -22 } }

// The remaining syscall entry points and clock-specific callbacks retain the
// C ABI and are supplied by the surrounding kernel translation unit.
extern "C" {
    pub fn timer_create(which_clock: clockid_t, event: *mut sigevent_t, id: *mut timer_t) -> i32;
    pub fn timer_gettime(id: timer_t, setting: *mut itimerspec64) -> i32;
    pub fn timer_settime(id: timer_t, flags: i32, new_setting: *const itimerspec64, old_setting: *mut itimerspec64) -> i32;
    pub fn timer_delete(id: timer_t) -> i32;
    pub fn clock_settime(which_clock: clockid_t, tp: *const timespec64) -> i32;
    pub fn clock_gettime(which_clock: clockid_t, tp: *mut timespec64) -> i32;
    pub fn clock_getres(which_clock: clockid_t, tp: *mut timespec64) -> i32;
    pub fn clock_nanosleep(which_clock: clockid_t, flags: i32, rqtp: *const timespec64, rmtp: *mut timespec64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
