// SPDX-License-Identifier: GPL-2.0
/*
 * <linux/swait.h> (simple wait queues ) implementation:
 */

use kernel::ffi::{c_char, c_int, c_long};
use kernel::bindings::sched_waiting_native::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __init_swait_queue_head(q: *mut swait_queue_head, name: *const c_char, key: *mut lock_class_key) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    lupos_waiting_raw_spin_lock_init(&raw mut (*q).lock);
    lupos_waiting_raw_lockdep_class(&raw mut (*q).lock, key, name);
    lupos_waiting_init_list_head(&raw mut (*q).task_list);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn swake_up_locked(q: *mut swait_queue_head, wake_flags: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    if lupos_waiting_list_empty(&raw const (*q).task_list) { return; }
    let curr = lupos_waiting_first_swait(&raw mut (*q).task_list);
    try_to_wake_up((*curr).task, LUPOS_WAITING_TASK_NORMAL, wake_flags);
    lupos_waiting_list_del_init(&raw mut (*curr).task_list);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn swake_up_all_locked(q: *mut swait_queue_head) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    while !lupos_waiting_list_empty(&raw const (*q).task_list) { swake_up_locked(q, 0); }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn swake_up_one(q: *mut swait_queue_head) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags = 0;
    lupos_waiting_raw_spin_lock_irqsave(&raw mut (*q).lock, &mut flags);
    swake_up_locked(q, 0);
    lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*q).lock, flags);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn swake_up_all(q: *mut swait_queue_head) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut tmp = core::mem::MaybeUninit::<list_head>::uninit();
    // Keep one raw pointer while native links retain this stack head's address.
    let tmp = tmp.as_mut_ptr();
    lupos_waiting_init_list_head(tmp);
    lupos_waiting_raw_spin_lock_irq(&raw mut (*q).lock);
    lupos_waiting_list_splice_init(&raw mut (*q).task_list, tmp);
    while !lupos_waiting_list_empty(tmp) {
        let curr = lupos_waiting_first_swait(tmp);
        wake_up_state((*curr).task, LUPOS_WAITING_TASK_NORMAL);
        lupos_waiting_list_del_init(&raw mut (*curr).task_list);
        if lupos_waiting_list_empty(tmp) { break; }
        lupos_waiting_raw_spin_unlock_irq(&raw mut (*q).lock);
        lupos_waiting_raw_spin_lock_irq(&raw mut (*q).lock);
    }
    lupos_waiting_raw_spin_unlock_irq(&raw mut (*q).lock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __prepare_to_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    (*wait).task = lupos_waiting_current();
    if lupos_waiting_list_empty(&raw const (*wait).task_list) { lupos_waiting_list_add_tail(&raw mut (*wait).task_list, &raw mut (*q).task_list); }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_to_swait_exclusive(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags = 0;
    lupos_waiting_raw_spin_lock_irqsave(&raw mut (*q).lock, &mut flags);
    __prepare_to_swait(q, wait);
    lupos_waiting_swait_exclusive_state(state);
    lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*q).lock, flags);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_to_swait_event(q: *mut swait_queue_head, wait: *mut swait_queue, state: c_int) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags = 0;
    let mut ret: c_long = 0;
    lupos_waiting_raw_spin_lock_irqsave(&raw mut (*q).lock, &mut flags);
    if lupos_waiting_signal_pending_state(state, lupos_waiting_current()) {
        lupos_waiting_list_del_init(&raw mut (*wait).task_list);
        ret = -LUPOS_WAITING_ERESTARTSYS;
    } else {
        __prepare_to_swait(q, wait);
        lupos_waiting_swait_event_state(state);
    }
    lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*q).lock, flags);
    ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __finish_swait(_q: *mut swait_queue_head, wait: *mut swait_queue) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    lupos_waiting_swait_finish_locked_state();
    if !lupos_waiting_list_empty(&raw const (*wait).task_list) { lupos_waiting_list_del_init(&raw mut (*wait).task_list); }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags = 0;
    lupos_waiting_swait_finish_state();
    if !lupos_waiting_list_empty_careful(&raw const (*wait).task_list) {
        lupos_waiting_raw_spin_lock_irqsave(&raw mut (*q).lock, &mut flags);
        lupos_waiting_list_del_init(&raw mut (*wait).task_list);
        lupos_waiting_raw_spin_unlock_irqrestore(&raw mut (*q).lock, flags);
    }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
