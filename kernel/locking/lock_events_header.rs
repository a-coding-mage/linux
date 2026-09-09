/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * Authors: Waiman Long <longman@redhat.com>
 */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
pub enum lock_events {
    // Contents supplied by the C preprocessor include "lock_events_list.h".
    lockevent_num,
    LOCKEVENT_reset_cnts = Self::lockevent_num as isize,
}

#[cfg(feature = "CONFIG_LOCK_EVENT_COUNTS")]
extern "C" {
    /* Per-cpu counters, supplied by the surrounding kernel translation. */
    pub static mut lockevents: *mut usize;
}

#[cfg(feature = "CONFIG_LOCK_EVENT_COUNTS")]
#[inline]
pub unsafe fn __lockevent_inc(event: lock_events, cond: bool) {
    if cond {
        // raw_cpu_inc(lockevents[event]);
        let _ = event;
    }
}

#[cfg(feature = "CONFIG_LOCK_EVENT_COUNTS")]
#[inline]
pub unsafe fn __lockevent_add(event: lock_events, inc: i32) {
    // raw_cpu_add(lockevents[event], inc);
    let _ = (event, inc);
}

#[cfg(feature = "CONFIG_LOCK_EVENT_COUNTS")]
#[macro_export]
macro_rules! lockevent_inc {
    ($ev:ident) => {
        $crate::__lockevent_inc($crate::lock_events::$ev, true)
    };
}

#[cfg(feature = "CONFIG_LOCK_EVENT_COUNTS")]
#[macro_export]
macro_rules! lockevent_cond_inc {
    ($ev:ident, $c:expr) => {
        $crate::__lockevent_inc($crate::lock_events::$ev, $c)
    };
}

#[cfg(feature = "CONFIG_LOCK_EVENT_COUNTS")]
#[macro_export]
macro_rules! lockevent_add {
    ($ev:ident, $c:expr) => {
        $crate::__lockevent_add($crate::lock_events::$ev, $c)
    };
}

#[cfg(not(feature = "CONFIG_LOCK_EVENT_COUNTS"))]
#[macro_export]
macro_rules! lockevent_inc { ($ev:ident) => {}; }

#[cfg(not(feature = "CONFIG_LOCK_EVENT_COUNTS"))]
#[macro_export]
macro_rules! lockevent_add {
    ($ev:ident, $c:expr) => {{ let _ = $c; }};
}

#[cfg(not(feature = "CONFIG_LOCK_EVENT_COUNTS"))]
#[macro_export]
macro_rules! lockevent_cond_inc {
    ($ev:ident, $c:expr) => {{ let _ = $c; }};
}

/* The following declarations are supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

extern "C" {
    pub fn lockevent_read(
        file: *mut file,
        user_buf: *mut u8,
        count: usize,
        ppos: *mut i64,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
