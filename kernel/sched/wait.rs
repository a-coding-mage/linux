// SPDX-License-Identifier: GPL-2.0-only
/* Generic waiting primitives. */

use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use kernel::bindings::sched_waiting_native::*;
use super::wait_bit::__var_wake_key;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __init_waitqueue_head(wq_head: *mut wait_queue_head, name: *const c_char, key: *mut lock_class_key) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    lupos_waiting_spin_lock_init(&raw mut (*wq_head).lock);
    lupos_waiting_spin_lockdep_class(&raw mut (*wq_head).lock, key, name);
    lupos_waiting_init_list_head(&raw mut (*wq_head).head);

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    lupos_waiting_and_wait_flags(wq_entry, !LUPOS_WAITING_WQ_FLAG_EXCLUSIVE);
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    lupos_waiting_add_wait_queue(wq_head, wq_entry);
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_wait_queue_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    lupos_waiting_or_wait_flags(wq_entry, LUPOS_WAITING_WQ_FLAG_EXCLUSIVE);
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    lupos_waiting_add_wait_queue_tail(wq_head, wq_entry);
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_wait_queue_priority(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    lupos_waiting_or_wait_flags(wq_entry, LUPOS_WAITING_WQ_FLAG_PRIORITY);
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    lupos_waiting_add_wait_queue(wq_head, wq_entry);
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_wait_queue_priority_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let head = &raw mut (*wq_head).head;
    lupos_waiting_or_wait_flags(wq_entry, LUPOS_WAITING_WQ_FLAG_EXCLUSIVE | LUPOS_WAITING_WQ_FLAG_PRIORITY);
    let mut flags: c_ulong = 0;
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    if !lupos_waiting_list_empty(head) && lupos_waiting_read_wait_flags(lupos_waiting_first_wait(head)) & LUPOS_WAITING_WQ_FLAG_PRIORITY != 0 {
        lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);
        return -LUPOS_WAITING_EBUSY;
    }
    lupos_waiting_list_add(&raw mut (*wq_entry).entry, head);
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);
    0

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    lupos_waiting_remove_wait_queue(wq_head, wq_entry);
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);

    }
}

unsafe fn __wake_up_common(wq_head: *mut wait_queue_head, mode: c_uint, mut nr_exclusive: c_int, wake_flags: c_int, key: *mut c_void) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    lupos_waiting_assert_wait_lock(wq_head);
    let head = &raw mut (*wq_head).head;
    let mut curr = lupos_waiting_first_wait(head);
    while !curr.is_null() {
        let next = lupos_waiting_next_wait(curr, head);
        let flags = lupos_waiting_read_wait_flags(curr);
        let ret = lupos_waiting_call_wait_func(curr, mode, wake_flags, key);
        if ret < 0 { break; }
        if ret != 0 && flags & LUPOS_WAITING_WQ_FLAG_EXCLUSIVE != 0 {
            nr_exclusive = nr_exclusive.wrapping_sub(1);
            if nr_exclusive == 0 { break; }
        }
        curr = next;
    }
    nr_exclusive

    }
}

unsafe fn __wake_up_common_lock(wq_head: *mut wait_queue_head, mode: c_uint, nr_exclusive: c_int, wake_flags: c_int, key: *mut c_void) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    let remaining = __wake_up_common(wq_head, mode, nr_exclusive, wake_flags, key);
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);
    nr_exclusive.wrapping_sub(remaining)

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up(wq_head: *mut wait_queue_head, mode: c_uint, nr_exclusive: c_int, key: *mut c_void) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { __wake_up_common_lock(wq_head, mode, nr_exclusive, 0, key)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_on_current_cpu(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { __wake_up_common_lock(wq_head, mode, 1, LUPOS_WAITING_WF_CURRENT_CPU, key);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_locked(wq_head: *mut wait_queue_head, mode: c_uint, nr: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { __wake_up_common(wq_head, mode, nr, 0, core::ptr::null_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_locked_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { __wake_up_common(wq_head, mode, 1, 0, key);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { if wq_head.is_null() { return; } __wake_up_common_lock(wq_head, mode, 1, LUPOS_WAITING_WF_SYNC, key);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_locked_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { __wake_up_common(wq_head, mode, 1, LUPOS_WAITING_WF_SYNC, key);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_sync(wq_head: *mut wait_queue_head, mode: c_uint) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { __wake_up_sync_key(wq_head, mode, core::ptr::null_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_pollfree(wq_head: *mut wait_queue_head) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { __wake_up(wq_head, LUPOS_WAITING_TASK_NORMAL, 0, lupos_waiting_pollfree_key()); lupos_waiting_warn_pollfree_active(wq_head);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_to_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0;
    lupos_waiting_and_wait_flags(wq_entry, !LUPOS_WAITING_WQ_FLAG_EXCLUSIVE);
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    if lupos_waiting_list_empty(&raw mut (*wq_entry).entry) { lupos_waiting_add_wait_queue(wq_head, wq_entry); }
    lupos_waiting_wait_prepare_state(state);
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags);

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_to_wait_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> bool {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0; let mut was_empty = false;
    lupos_waiting_or_wait_flags(wq_entry, LUPOS_WAITING_WQ_FLAG_EXCLUSIVE);
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    if lupos_waiting_list_empty(&raw mut (*wq_entry).entry) { was_empty = lupos_waiting_list_empty(&raw mut (*wq_head).head); lupos_waiting_add_wait_queue_tail(wq_head, wq_entry); }
    lupos_waiting_wait_exclusive_state(state); lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags); was_empty

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_wait_entry(wq_entry: *mut wait_queue_entry, flags: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { (*wq_entry).flags = flags as c_uint; (*wq_entry).private = lupos_waiting_current().cast(); (*wq_entry).func = Some(autoremove_wake_function); lupos_waiting_init_list_head(&raw mut (*wq_entry).entry);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_to_wait_event(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut flags: c_ulong = 0; let mut ret: c_long = 0;
    lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags);
    if lupos_waiting_signal_pending_state(state, lupos_waiting_current()) { lupos_waiting_list_del_init(&raw mut (*wq_entry).entry); ret = -LUPOS_WAITING_ERESTARTSYS; }
    else { if lupos_waiting_list_empty(&raw mut (*wq_entry).entry) { if lupos_waiting_read_wait_flags(wq_entry) & LUPOS_WAITING_WQ_FLAG_EXCLUSIVE != 0 { lupos_waiting_add_wait_queue_tail(wq_head, wq_entry); } else { lupos_waiting_add_wait_queue(wq_head, wq_entry); } } lupos_waiting_wait_event_state(state); }
    lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags); ret

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_wait_intr(wq: *mut wait_queue_head, wait: *mut wait_queue_entry) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { if lupos_waiting_list_empty(&raw mut (*wait).entry) { lupos_waiting_add_wait_queue_tail(wq, wait); } lupos_waiting_wait_intr_state(); if lupos_waiting_signal_pending(lupos_waiting_current()) { return -(LUPOS_WAITING_ERESTARTSYS as c_int); } lupos_waiting_spin_unlock(&raw mut (*wq).lock); schedule(); lupos_waiting_spin_lock(&raw mut (*wq).lock); 0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_wait_intr_irq(wq: *mut wait_queue_head, wait: *mut wait_queue_entry) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { if lupos_waiting_list_empty(&raw mut (*wait).entry) { lupos_waiting_add_wait_queue_tail(wq, wait); } lupos_waiting_wait_intr_irq_state(); if lupos_waiting_signal_pending(lupos_waiting_current()) { return -(LUPOS_WAITING_ERESTARTSYS as c_int); } lupos_waiting_spin_unlock_irq(&raw mut (*wq).lock); schedule(); lupos_waiting_spin_lock_irq(&raw mut (*wq).lock); 0
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn finish_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { let mut flags: c_ulong = 0; lupos_waiting_wait_finish_state(); if !lupos_waiting_list_empty_careful(&raw mut (*wq_entry).entry) { lupos_waiting_spin_lock_irqsave(&raw mut (*wq_head).lock, &mut flags); lupos_waiting_list_del_init(&raw mut (*wq_entry).entry); lupos_waiting_spin_unlock_irqrestore(&raw mut (*wq_head).lock, flags); }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn autoremove_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, key: *mut c_void) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { let ret = default_wake_function(wq_entry, mode, sync, key); if ret != 0 { lupos_waiting_list_del_init_careful(&raw mut (*wq_entry).entry); } ret
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wait_woken(wq_entry: *mut wait_queue_entry, mode: c_uint, mut timeout: c_long) -> c_long {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { lupos_waiting_wait_woken_sleep_state(mode); if lupos_waiting_read_wait_flags(wq_entry) & LUPOS_WAITING_WQ_FLAG_WOKEN == 0 && !kthread_should_stop_or_park() { timeout = schedule_timeout(timeout); } lupos_waiting_wait_woken_running_state(); lupos_waiting_store_mb_flags(&raw mut (*wq_entry).flags, lupos_waiting_read_wait_flags(wq_entry) & !LUPOS_WAITING_WQ_FLAG_WOKEN); timeout
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn woken_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, key: *mut c_void) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { lupos_waiting_smp_mb(); lupos_waiting_or_wait_flags(wq_entry, LUPOS_WAITING_WQ_FLAG_WOKEN); default_wake_function(wq_entry, mode, sync, key)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn woken_wake_bit_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, arg: *mut c_void) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe { let key = __var_wake_key(wq_entry, arg); if key.is_null() { return 0; } lupos_waiting_smp_mb(); lupos_waiting_or_wait_flags(wq_entry, LUPOS_WAITING_WQ_FLAG_WOKEN); default_wake_function(wq_entry, mode, sync, key as *mut c_void)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
