// SPDX-License-Identifier: GPL-2.0-only
/* Generic waiting primitives. */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn __init_waitqueue_head(wq_head: *mut wait_queue_head, name: *const core::ffi::c_char, key: *mut lock_class_key) {
    spin_lock_init(unsafe { &mut (*wq_head).lock });
    lockdep_set_class_and_name(unsafe { &mut (*wq_head).lock }, key, name);
    INIT_LIST_HEAD(unsafe { &mut (*wq_head).head });
}

pub unsafe fn add_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags: c_ulong = 0;
    unsafe { (*wq_entry).flags &= !WQ_FLAG_EXCLUSIVE; }
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    __add_wait_queue(wq_head, wq_entry);
    spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags);
}

pub unsafe fn add_wait_queue_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags: c_ulong = 0;
    unsafe { (*wq_entry).flags |= WQ_FLAG_EXCLUSIVE; }
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    __add_wait_queue_entry_tail(wq_head, wq_entry);
    spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags);
}

pub unsafe fn add_wait_queue_priority(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags: c_ulong = 0;
    unsafe { (*wq_entry).flags |= WQ_FLAG_PRIORITY; }
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    __add_wait_queue(wq_head, wq_entry);
    spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags);
}

pub unsafe fn add_wait_queue_priority_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) -> c_int {
    let head = unsafe { &mut (*wq_head).head };
    unsafe { (*wq_entry).flags |= WQ_FLAG_EXCLUSIVE | WQ_FLAG_PRIORITY; }
    let _guard = guard_spinlock_irqsave(unsafe { &mut (*wq_head).lock });
    if !list_empty(head) && unsafe { (*list_first_entry(head, core::mem::size_of::<wait_queue_entry>(), 0)).flags & WQ_FLAG_PRIORITY != 0 } {
        return -EBUSY;
    }
    list_add(unsafe { &mut (*wq_entry).entry }, head);
    0
}

pub unsafe fn remove_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    __remove_wait_queue(wq_head, wq_entry);
    spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags);
}

unsafe fn __wake_up_common(wq_head: *mut wait_queue_head, mode: c_uint, mut nr_exclusive: c_int, wake_flags: c_int, key: *mut c_void) -> c_int {
    let mut curr: *mut wait_queue_entry = list_first_entry(unsafe { &mut (*wq_head).head }, core::mem::size_of::<wait_queue_entry>(), 0);
    if unsafe { &mut (*curr).entry } as *mut _ == unsafe { &mut (*wq_head).head } as *mut _ { return nr_exclusive; }
    let mut next: *mut wait_queue_entry = core::ptr::null_mut();
    list_for_each_entry_safe_from(&mut curr, &mut next, unsafe { &mut (*wq_head).head }, 0, {
        let flags = unsafe { (*curr).flags };
        let ret = unsafe { ((*curr).func)(curr, mode, wake_flags, key) };
        if ret < 0 { break; }
        if ret != 0 && flags & WQ_FLAG_EXCLUSIVE != 0 {
            nr_exclusive -= 1;
            if nr_exclusive == 0 { break; }
        }
    });
    nr_exclusive
}

unsafe fn __wake_up_common_lock(wq_head: *mut wait_queue_head, mode: c_uint, nr_exclusive: c_int, wake_flags: c_int, key: *mut c_void) -> c_int {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    let remaining = __wake_up_common(wq_head, mode, nr_exclusive, wake_flags, key);
    spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags);
    nr_exclusive - remaining
}

pub unsafe fn __wake_up(wq_head: *mut wait_queue_head, mode: c_uint, nr_exclusive: c_int, key: *mut c_void) -> c_int { __wake_up_common_lock(wq_head, mode, nr_exclusive, 0, key) }
pub unsafe fn __wake_up_on_current_cpu(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) { __wake_up_common_lock(wq_head, mode, 1, WF_CURRENT_CPU, key); }
pub unsafe fn __wake_up_locked(wq_head: *mut wait_queue_head, mode: c_uint, nr: c_int) { __wake_up_common(wq_head, mode, nr, 0, core::ptr::null_mut()); }
pub unsafe fn __wake_up_locked_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) { __wake_up_common(wq_head, mode, 1, 0, key); }
pub unsafe fn __wake_up_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) { if wq_head.is_null() { return; } __wake_up_common_lock(wq_head, mode, 1, WF_SYNC, key); }
pub unsafe fn __wake_up_locked_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void) { __wake_up_common(wq_head, mode, 1, WF_SYNC, key); }
pub unsafe fn __wake_up_sync(wq_head: *mut wait_queue_head, mode: c_uint) { __wake_up_sync_key(wq_head, mode, core::ptr::null_mut()); }
pub unsafe fn __wake_up_pollfree(wq_head: *mut wait_queue_head) { __wake_up(wq_head, TASK_NORMAL, 0, poll_to_key(EPOLLHUP | POLLFREE)); WARN_ON_ONCE(waitqueue_active(wq_head)); }

pub unsafe fn prepare_to_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) {
    let mut flags: c_ulong = 0;
    unsafe { (*wq_entry).flags &= !WQ_FLAG_EXCLUSIVE; }
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    if list_empty(unsafe { &mut (*wq_entry).entry }) { __add_wait_queue(wq_head, wq_entry); }
    set_current_state(state);
    spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags);
}

pub unsafe fn prepare_to_wait_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> bool {
    let mut flags: c_ulong = 0; let mut was_empty = false;
    unsafe { (*wq_entry).flags |= WQ_FLAG_EXCLUSIVE; }
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    if list_empty(unsafe { &mut (*wq_entry).entry }) { was_empty = list_empty(unsafe { &mut (*wq_head).head }); __add_wait_queue_entry_tail(wq_head, wq_entry); }
    set_current_state(state); spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags); was_empty
}

pub unsafe fn init_wait_entry(wq_entry: *mut wait_queue_entry, flags: c_uint) { unsafe { (*wq_entry).flags = flags; (*wq_entry).private = current; (*wq_entry).func = autoremove_wake_function; INIT_LIST_HEAD(&mut (*wq_entry).entry); } }

pub unsafe fn prepare_to_wait_event(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> c_long {
    let mut flags: c_ulong = 0; let mut ret: c_long = 0;
    spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags);
    if signal_pending_state(state, current) { list_del_init(unsafe { &mut (*wq_entry).entry }); ret = -ERESTARTSYS; }
    else { if list_empty(unsafe { &mut (*wq_entry).entry }) { if unsafe { (*wq_entry).flags } & WQ_FLAG_EXCLUSIVE != 0 { __add_wait_queue_entry_tail(wq_head, wq_entry); } else { __add_wait_queue(wq_head, wq_entry); } } set_current_state(state); }
    spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags); ret
}

pub unsafe fn do_wait_intr(wq: *mut wait_queue_head, wait: *mut wait_queue_entry) -> c_int { if list_empty(unsafe { &mut (*wait).entry }) { __add_wait_queue_entry_tail(wq, wait); } set_current_state(TASK_INTERRUPTIBLE); if signal_pending(current) { return -ERESTARTSYS; } spin_unlock(unsafe { &mut (*wq).lock }); schedule(); spin_lock(unsafe { &mut (*wq).lock }); 0 }
pub unsafe fn do_wait_intr_irq(wq: *mut wait_queue_head, wait: *mut wait_queue_entry) -> c_int { if list_empty(unsafe { &mut (*wait).entry }) { __add_wait_queue_entry_tail(wq, wait); } set_current_state(TASK_INTERRUPTIBLE); if signal_pending(current) { return -ERESTARTSYS; } spin_unlock_irq(unsafe { &mut (*wq).lock }); schedule(); spin_lock_irq(unsafe { &mut (*wq).lock }); 0 }
pub unsafe fn finish_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry) { let mut flags: c_ulong = 0; __set_current_state(TASK_RUNNING); if !list_empty_careful(unsafe { &mut (*wq_entry).entry }) { spin_lock_irqsave(unsafe { &mut (*wq_head).lock }, &mut flags); list_del_init(unsafe { &mut (*wq_entry).entry }); spin_unlock_irqrestore(unsafe { &mut (*wq_head).lock }, flags); } }
pub unsafe fn autoremove_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, key: *mut c_void) -> c_int { let ret = default_wake_function(wq_entry, mode, sync, key); if ret != 0 { list_del_init_careful(unsafe { &mut (*wq_entry).entry }); } ret }
pub unsafe fn wait_woken(wq_entry: *mut wait_queue_entry, mode: c_uint, mut timeout: c_long) -> c_long { set_current_state(mode as c_int); if unsafe { (*wq_entry).flags } & WQ_FLAG_WOKEN == 0 && !kthread_should_stop_or_park() { timeout = schedule_timeout(timeout); } __set_current_state(TASK_RUNNING); smp_store_mb(unsafe { &mut (*wq_entry).flags }, unsafe { (*wq_entry).flags } & !WQ_FLAG_WOKEN); timeout }
pub unsafe fn woken_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, key: *mut c_void) -> c_int { smp_mb(); unsafe { (*wq_entry).flags |= WQ_FLAG_WOKEN; } default_wake_function(wq_entry, mode, sync, key) }
pub unsafe fn woken_wake_bit_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, arg: *mut c_void) -> c_int { let key = __var_wake_key(wq_entry, arg); if key.is_null() { return 0; } smp_mb(); unsafe { (*wq_entry).flags |= WQ_FLAG_WOKEN; } default_wake_function(wq_entry, mode, sync, key as *mut c_void) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
