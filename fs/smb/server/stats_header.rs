/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2025, LG Electronics.
 *   Author(s): Hyunchul Lee <hyc.lee@gmail.com>
 *   Copyright (C) 2025, Samsung Electronics.
 *   Author(s): Vedansh Bhardwaj <v.bhardwaj@samsung.com>
 */

// Dependency supplied by ../common/smb2status.h is intentionally not defined here.

pub const KSMBD_COUNTER_MAX_REQS: usize = 19;

pub const KSMBD_COUNTER_SESSIONS: usize = 0;
pub const KSMBD_COUNTER_TREE_CONNS: usize = 1;
pub const KSMBD_COUNTER_REQUESTS: usize = 2;
pub const KSMBD_COUNTER_STATUS_SUCCESS: usize = 3;
pub const KSMBD_COUNTER_STATUS_INFORMATIONAL: usize = 4;
pub const KSMBD_COUNTER_STATUS_WARNING: usize = 5;
pub const KSMBD_COUNTER_STATUS_ERROR: usize = 6;
pub const KSMBD_COUNTER_ERROR_ACCESS_DENIED: usize = 7;
pub const KSMBD_COUNTER_ERROR_NOT_FOUND: usize = 8;
pub const KSMBD_COUNTER_ERROR_INVALID_PARAMETER: usize = 9;
pub const KSMBD_COUNTER_ERROR_SHARING_VIOLATION: usize = 10;
pub const KSMBD_COUNTER_ERROR_NOT_SUPPORTED: usize = 11;
pub const KSMBD_COUNTER_ERROR_OTHER: usize = 12;
pub const KSMBD_COUNTER_READ_BYTES: usize = 13;
pub const KSMBD_COUNTER_WRITE_BYTES: usize = 14;
pub const KSMBD_COUNTER_FIRST_REQ: usize = 15;
pub const KSMBD_COUNTER_LAST_REQ: usize = KSMBD_COUNTER_FIRST_REQ + KSMBD_COUNTER_MAX_REQS - 1;
pub const KSMBD_COUNTER_MAX: usize = 35;

// CONFIG_PROC_FS conditional declarations from the C header.
#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub static mut ksmbd_counters: ksmbd_counters;
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[repr(C)]
pub struct ksmbd_counters {
    pub counters: [percpu_counter; KSMBD_COUNTER_MAX],
}

// These types and functions are supplied by the surrounding kernel bindings.
#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub fn percpu_counter_inc(counter: *mut percpu_counter);
    pub fn percpu_counter_dec(counter: *mut percpu_counter);
    pub fn percpu_counter_add(counter: *mut percpu_counter, value: i64);
    pub fn percpu_counter_sub(counter: *mut percpu_counter, value: i64);
    pub fn percpu_counter_sum_positive(counter: *mut percpu_counter) -> i64;
}

#[cfg(feature = "CONFIG_PROC_FS")]
pub type __le32 = u32;

#[cfg(feature = "CONFIG_PROC_FS")]
#[inline]
pub unsafe fn ksmbd_counter_inc(type_: i32) {
    percpu_counter_inc(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[type_ as usize]);
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[inline]
pub unsafe fn ksmbd_counter_dec(type_: i32) {
    percpu_counter_dec(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[type_ as usize]);
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[inline]
pub unsafe fn ksmbd_counter_add(type_: i32, value: i64) {
    percpu_counter_add(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[type_ as usize], value);
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[inline]
pub unsafe fn ksmbd_counter_sub(type_: i32, value: i64) {
    percpu_counter_sub(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[type_ as usize], value);
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[inline]
pub unsafe fn ksmbd_counter_inc_reqs(cmd: u32, status: __le32) {
    let severity = status >> 30;
    let mut type_: usize;
    type_ = match severity {
        0 => KSMBD_COUNTER_STATUS_SUCCESS,
        1 => KSMBD_COUNTER_STATUS_INFORMATIONAL,
        2 => KSMBD_COUNTER_STATUS_WARNING,
        _ => KSMBD_COUNTER_STATUS_ERROR,
    };
    percpu_counter_inc(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[type_]);

    if severity == 3 {
        type_ = if status == STATUS_ACCESS_DENIED {
            KSMBD_COUNTER_ERROR_ACCESS_DENIED
        } else if status == STATUS_OBJECT_NAME_NOT_FOUND || status == STATUS_NO_SUCH_FILE {
            KSMBD_COUNTER_ERROR_NOT_FOUND
        } else if status == STATUS_INVALID_PARAMETER {
            KSMBD_COUNTER_ERROR_INVALID_PARAMETER
        } else if status == STATUS_SHARING_VIOLATION {
            KSMBD_COUNTER_ERROR_SHARING_VIOLATION
        } else if status == STATUS_NOT_SUPPORTED || status == STATUS_NOT_IMPLEMENTED {
            KSMBD_COUNTER_ERROR_NOT_SUPPORTED
        } else {
            KSMBD_COUNTER_ERROR_OTHER
        };
        percpu_counter_inc(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[type_]);
    }

    if cmd < KSMBD_COUNTER_MAX_REQS as u32 {
        percpu_counter_inc(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[KSMBD_COUNTER_REQUESTS]);
        percpu_counter_inc(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[KSMBD_COUNTER_FIRST_REQ + cmd as usize]);
    }
}

#[cfg(feature = "CONFIG_PROC_FS")]
#[inline]
pub unsafe fn ksmbd_counter_sum(type_: i32) -> i64 {
    percpu_counter_sum_positive(&mut (*core::ptr::addr_of_mut!(ksmbd_counters)).counters[type_ as usize])
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn ksmbd_counter_inc(_type_: i32) {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn ksmbd_counter_dec(_type_: i32) {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn ksmbd_counter_add(_type_: i32, _value: i64) {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn ksmbd_counter_sub(_type_: i32, _value: i64) {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn ksmbd_counter_inc_reqs(_cmd: u32, _status: u32) {}
#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn ksmbd_counter_sum(_type_: i32) -> i64 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
