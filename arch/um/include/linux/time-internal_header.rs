/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2012 - 2014 Cisco Systems
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: list_head, WARN_ON, time_travel_mode, and TT_MODE_EXTERNAL.

pub const TIMER_MULTIPLIER: u32 = 256;
pub const TIMER_MIN_DELTA: u32 = 500;

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
#[repr(C)]
pub struct time_travel_event {
    pub time: u64,
    pub fn_: Option<unsafe extern "C" fn(d: *mut time_travel_event)>,
    pub list: crate::list_head,
    pub pending: bool,
    pub onstack: bool,
}

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
extern "C" {
    pub fn time_travel_sleep();
    pub fn __time_travel_propagate_time();
    pub fn __time_travel_wait_readable(fd: i32);
    pub fn time_travel_add_irq_event(e: *mut time_travel_event);
    pub fn time_travel_add_event_rel(e: *mut time_travel_event, delay_ns: u64);
    pub fn time_travel_del_event(e: *mut time_travel_event) -> bool;
}

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
#[inline]
pub unsafe fn time_travel_set_event_fn(
    e: *mut time_travel_event,
    fn_: Option<unsafe extern "C" fn(d: *mut time_travel_event)>,
) {
    (*e).fn_ = fn_;
}

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
#[inline]
pub unsafe fn time_travel_propagate_time() {
    if crate::time_travel_mode == crate::TT_MODE_EXTERNAL {
        __time_travel_propagate_time();
    }
}

#[cfg(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT")]
#[inline]
pub unsafe fn time_travel_wait_readable(fd: i32) {
    if crate::time_travel_mode == crate::TT_MODE_EXTERNAL {
        __time_travel_wait_readable(fd);
    }
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[repr(C)]
pub struct time_travel_event {
    _private: [u8; 0],
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[inline]
pub fn time_travel_sleep() {}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[macro_export]
macro_rules! time_travel_set_event_fn {
    ($e:expr, $fn_:expr) => {{ let _ = &$e; let _ = &$fn_; }};
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[inline]
pub fn time_travel_propagate_time() {}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[inline]
pub fn time_travel_wait_readable(_fd: i32) {}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[inline]
pub unsafe fn time_travel_add_irq_event(_e: *mut time_travel_event) {
    crate::WARN_ON(1);
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
extern "C" {
    pub fn time_travel_not_configured() -> !;
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[macro_export]
macro_rules! time_travel_add_event_rel {
    ($($arg:tt)*) => { $crate::time_travel_not_configured() };
}

#[cfg(not(feature = "CONFIG_UML_TIME_TRAVEL_SUPPORT"))]
#[macro_export]
macro_rules! time_travel_del_event {
    ($($arg:tt)*) => { $crate::time_travel_not_configured() };
}

extern "C" {
    pub static mut tt_extra_sched_jiffies: libc::c_ulong;
    pub fn time_travel_ndelay(nsec: libc::c_ulong);
    pub fn um_setup_timer() -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
