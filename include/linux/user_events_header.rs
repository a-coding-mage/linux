/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022, Microsoft Corporation.
 *
 * Authors:
 *   Beau Belgrave <beaub@linux.microsoft.com>
 */

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcu_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub user_event_mm: *mut user_event_mm,
}

pub type u64 = ::core::primitive::u64;

pub const CLONE_VM: u64 = 0x0000_0100;

#[repr(C)]
pub struct user_event_mm {
    pub mms_link: list_head,
    pub enablers: list_head,
    pub mm: *mut mm_struct,
    // Used for one-shot lists, protected by event_mutex.
    pub next: *mut user_event_mm,
    pub refcnt: refcount_t,
    pub tasks: refcount_t,
    pub put_rwork: rcu_work,
}

#[cfg(feature = "CONFIG_USER_EVENTS")]
unsafe extern "C" {
    pub fn user_event_mm_dup(t: *mut task_struct, old_mm: *mut user_event_mm);
    pub fn user_event_mm_remove(t: *mut task_struct);
    pub static mut current: *mut task_struct;
}

#[cfg(feature = "CONFIG_USER_EVENTS")]
#[inline]
pub unsafe fn user_events_fork(t: *mut task_struct, clone_flags: u64) {
    let old_mm: *mut user_event_mm;

    if t.is_null() || (*current).user_event_mm.is_null() {
        return;
    }

    old_mm = (*current).user_event_mm;

    if clone_flags & CLONE_VM != 0 {
        (*t).user_event_mm = old_mm;
        // refcount_inc(&old_mm->tasks); supplied by the kernel refcount API.
        refcount_inc(&mut (*old_mm).tasks);
        return;
    }

    user_event_mm_dup(t, old_mm);
}

#[cfg(feature = "CONFIG_USER_EVENTS")]
#[inline]
pub unsafe fn user_events_execve(t: *mut task_struct) {
    if t.is_null() || (*t).user_event_mm.is_null() {
        return;
    }

    user_event_mm_remove(t);
}

#[cfg(feature = "CONFIG_USER_EVENTS")]
#[inline]
pub unsafe fn user_events_exit(t: *mut task_struct) {
    if t.is_null() || (*t).user_event_mm.is_null() {
        return;
    }

    user_event_mm_remove(t);
}

#[cfg(not(feature = "CONFIG_USER_EVENTS"))]
#[inline]
pub unsafe fn user_events_fork(_t: *mut task_struct, _clone_flags: u64) {}

#[cfg(not(feature = "CONFIG_USER_EVENTS"))]
#[inline]
pub unsafe fn user_events_execve(_t: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_USER_EVENTS"))]
#[inline]
pub unsafe fn user_events_exit(_t: *mut task_struct) {}

// Supplied by the kernel refcount API.
unsafe extern "C" {
    pub fn refcount_inc(r: *mut refcount_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
