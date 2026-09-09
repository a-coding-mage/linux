/* SPDX-License-Identifier: GPL-2.0-only */

/* C header translated literally. External kernel types/functions are supplied by dependencies. */

#[cfg(not(feature = "WW_RT"))]
pub type MutexType = mutex;
#[cfg(not(feature = "WW_RT"))]
pub type MutexWaiterType = mutex_waiter;
#[cfg(feature = "WW_RT")]
pub type MutexType = rt_mutex;
#[cfg(feature = "WW_RT")]
pub type MutexWaiterType = rt_mutex_waiter;

#[cfg(not(feature = "WW_RT"))]
pub unsafe fn __ww_waiter_first(lock: *mut mutex) -> *mut mutex_waiter { (*lock).first_waiter }
#[cfg(not(feature = "WW_RT"))]
pub unsafe fn __ww_waiter_next(lock: *mut mutex, mut w: *mut mutex_waiter) -> *mut mutex_waiter {
    w = list_next_entry(w, list);
    if (*lock).first_waiter == w { core::ptr::null_mut() } else { w }
}
#[cfg(not(feature = "WW_RT"))]
pub unsafe fn __ww_waiter_prev(lock: *mut mutex, w: *mut mutex_waiter) -> *mut mutex_waiter {
    if (*lock).first_waiter == w { core::ptr::null_mut() } else { list_prev_entry(w, list) }
}
#[cfg(not(feature = "WW_RT"))]
pub unsafe fn __ww_waiter_last(lock: *mut mutex) -> *mut mutex_waiter {
    let w = (*lock).first_waiter;
    if !w.is_null() { list_prev_entry(w, list) } else { w }
}
#[cfg(not(feature = "WW_RT"))]
pub unsafe fn __ww_waiter_add(lock: *mut mutex, waiter: *mut mutex_waiter, pos: *mut mutex_waiter) {
    __mutex_add_waiter(lock, waiter, pos);
}
#[cfg(not(feature = "WW_RT"))]
pub unsafe fn __ww_mutex_owner(lock: *mut mutex) -> *mut task_struct { __mutex_owner(lock) }
#[cfg(not(feature = "WW_RT"))]
pub unsafe fn __ww_mutex_has_waiters(lock: *mut mutex) -> bool { atomic_long_read(&(*lock).owner) & MUTEX_FLAG_WAITERS != 0 }

#[cfg(feature = "WW_RT")]
pub unsafe fn __ww_waiter_first(lock: *mut rt_mutex) -> *mut rt_mutex_waiter { let n = rb_first(&(*lock).rtmutex.waiters.rb_root); if n.is_null() { core::ptr::null_mut() } else { rb_entry(n, rt_mutex_waiter, tree.entry) } }
#[cfg(feature = "WW_RT")]
pub unsafe fn __ww_waiter_next(_: *mut rt_mutex, w: *mut rt_mutex_waiter) -> *mut rt_mutex_waiter { let n = rb_next(&(*w).tree.entry); if n.is_null() { core::ptr::null_mut() } else { rb_entry(n, rt_mutex_waiter, tree.entry) } }
#[cfg(feature = "WW_RT")]
pub unsafe fn __ww_waiter_prev(_: *mut rt_mutex, w: *mut rt_mutex_waiter) -> *mut rt_mutex_waiter { let n = rb_prev(&(*w).tree.entry); if n.is_null() { core::ptr::null_mut() } else { rb_entry(n, rt_mutex_waiter, tree.entry) } }
#[cfg(feature = "WW_RT")]
pub unsafe fn __ww_waiter_last(lock: *mut rt_mutex) -> *mut rt_mutex_waiter { let n = rb_last(&(*lock).rtmutex.waiters.rb_root); if n.is_null() { core::ptr::null_mut() } else { rb_entry(n, rt_mutex_waiter, tree.entry) } }
#[cfg(feature = "WW_RT")]
pub unsafe fn __ww_waiter_add(_: *mut rt_mutex, _: *mut rt_mutex_waiter, _: *mut rt_mutex_waiter) { /* RT unconditionally adds the waiter first and then removes it on error */ }
#[cfg(feature = "WW_RT")]
pub unsafe fn __ww_mutex_owner(lock: *mut rt_mutex) -> *mut task_struct { rt_mutex_owner(&(*lock).rtmutex) }
#[cfg(feature = "WW_RT")]
pub unsafe fn __ww_mutex_has_waiters(lock: *mut rt_mutex) -> bool { rt_mutex_has_waiters(&(*lock).rtmutex) }

#[cfg(not(feature = "WW_RT"))]
pub unsafe fn lock_wait_lock(lock: *mut mutex, flags: *mut c_ulong) { raw_spin_lock_irqsave(&mut (*lock).wait_lock, *flags); }
#[cfg(not(feature = "WW_RT"))]
pub unsafe fn unlock_wait_lock(lock: *mut mutex, flags: *mut c_ulong) { raw_spin_unlock_irqrestore(&mut (*lock).wait_lock, *flags); }
#[cfg(feature = "WW_RT")]
pub unsafe fn lock_wait_lock(lock: *mut rt_mutex, flags: *mut c_ulong) { raw_spin_lock_irqsave(&mut (*lock).rtmutex.wait_lock, *flags); }
#[cfg(feature = "WW_RT")]
pub unsafe fn unlock_wait_lock(lock: *mut rt_mutex, flags: *mut c_ulong) { raw_spin_unlock_irqrestore(&mut (*lock).rtmutex.wait_lock, *flags); }

pub unsafe fn ww_mutex_lock_acquired(ww: *mut ww_mutex, ww_ctx: *mut ww_acquire_ctx) {
    #[cfg(feature = "DEBUG_WW_MUTEXES")] {
        DEBUG_LOCKS_WARN_ON(!(*ww).ctx.is_null());
        DEBUG_LOCKS_WARN_ON((*ww_ctx).done_acquire);
        if !(*ww_ctx).contending_lock.is_null() {
            DEBUG_LOCKS_WARN_ON((*ww_ctx).contending_lock != ww);
            DEBUG_LOCKS_WARN_ON((*ww_ctx).acquired > 0);
            (*ww_ctx).contending_lock = core::ptr::null_mut();
        }
        DEBUG_LOCKS_WARN_ON((*ww_ctx).ww_class != (*ww).ww_class);
    }
    (*ww_ctx).acquired += 1;
    (*ww).ctx = ww_ctx;
}

pub unsafe fn __ww_ctx_less(a: *mut ww_acquire_ctx, b: *mut ww_acquire_ctx) -> bool {
    #[cfg(feature = "WW_RT")] {
        let a_prio = (*(*a).task).prio; let b_prio = (*(*b).task).prio;
        if rt_or_dl_prio(a_prio) || rt_or_dl_prio(b_prio) {
            if a_prio > b_prio { return true; } if a_prio < b_prio { return false; }
            if dl_prio(a_prio) { if dl_time_before((*(*b).task).dl.deadline, (*(*a).task).dl.deadline) { return true; } if dl_time_before((*(*a).task).dl.deadline, (*(*b).task).dl.deadline) { return false; } }
        }
    }
    ( ((*a).stamp.wrapping_sub((*b).stamp)) as isize ) > 0
}

pub unsafe fn __ww_mutex_die(lock: *mut MutexType, waiter: *mut MutexWaiterType, ww_ctx: *mut ww_acquire_ctx, wake_q: *mut wake_q_head) -> bool {
    if !(*ww_ctx).is_wait_die { return false; }
    if (*(*waiter).ww_ctx).acquired > 0 && __ww_ctx_less((*waiter).ww_ctx, ww_ctx) {
        #[cfg(not(feature = "WW_RT"))] { debug_mutex_wake_waiter(lock, waiter); }
        clear_task_blocked_on((*waiter).task, lock); wake_q_add(wake_q, (*waiter).task);
    }
    true
}

pub unsafe fn __ww_mutex_wound(lock: *mut MutexType, ww_ctx: *mut ww_acquire_ctx, hold_ctx: *mut ww_acquire_ctx, wake_q: *mut wake_q_head) -> bool {
    let owner = __ww_mutex_owner(lock); if hold_ctx.is_null() || owner.is_null() { return false; }
    if (*ww_ctx).acquired > 0 && __ww_ctx_less(hold_ctx, ww_ctx) {
        (*hold_ctx).wounded = 1;
        if owner != current { clear_task_blocked_on(owner, core::ptr::null_mut()); wake_q_add(wake_q, owner); }
        return true;
    } false
}

pub unsafe fn __ww_mutex_check_waiters(lock: *mut MutexType, ww_ctx: *mut ww_acquire_ctx, wake_q: *mut wake_q_head) {
    let mut cur = __ww_waiter_first(lock); while !cur.is_null() { if !(*cur).ww_ctx.is_null() && (__ww_mutex_die(lock, cur, ww_ctx, wake_q) || __ww_mutex_wound(lock, (*cur).ww_ctx, ww_ctx, wake_q)) { break; } cur = __ww_waiter_next(lock, cur); }
}

pub unsafe fn ww_mutex_set_context_fastpath(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) {
    let mut wake_q = DEFINE_WAKE_Q(); let mut flags: c_ulong = 0; ww_mutex_lock_acquired(lock, ctx); smp_mb();
    if !data_race(__ww_mutex_has_waiters(&mut (*lock).base)) { return; }
    lock_wait_lock(&mut (*lock).base, &mut flags); __ww_mutex_check_waiters(&mut (*lock).base, ctx, &mut wake_q); preempt_disable(); unlock_wait_lock(&mut (*lock).base, &mut flags); wake_up_q(&mut wake_q); preempt_enable();
}

pub unsafe fn __ww_mutex_kill(lock: *mut MutexType, ww_ctx: *mut ww_acquire_ctx) -> c_int {
    if (*ww_ctx).acquired > 0 { #[cfg(feature = "DEBUG_WW_MUTEXES")] { let ww = container_of!(lock, ww_mutex, base); DEBUG_LOCKS_WARN_ON(!(*ww_ctx).contending_lock.is_null()); (*ww_ctx).contending_lock = ww; } return -EDEADLK; } 0
}

pub unsafe fn __ww_mutex_check_kill(lock: *mut MutexType, waiter: *mut MutexWaiterType, ctx: *mut ww_acquire_ctx) -> c_int {
    let ww: *mut ww_mutex = container_of!(lock, ww_mutex, base); let hold_ctx = READ_ONCE((*ww).ctx);
    if (*ctx).acquired == 0 { return 0; }
    if !(*ctx).is_wait_die { return if (*ctx).wounded { __ww_mutex_kill(lock, ctx) } else { 0 }; }
    if !hold_ctx.is_null() && __ww_ctx_less(ctx, hold_ctx) { return __ww_mutex_kill(lock, ctx); }
    let mut cur = __ww_waiter_prev(lock, waiter); while !cur.is_null() { if !(*cur).ww_ctx.is_null() { return __ww_mutex_kill(lock, ctx); } cur = __ww_waiter_prev(lock, cur); } 0
}

pub unsafe fn __ww_mutex_add_waiter(waiter: *mut MutexWaiterType, lock: *mut MutexType, ww_ctx: *mut ww_acquire_ctx, wake_q: *mut wake_q_head) -> c_int {
    if ww_ctx.is_null() { __ww_waiter_add(lock, waiter, core::ptr::null_mut()); return 0; }
    let is_wait_die = (*ww_ctx).is_wait_die; let mut pos: *mut MutexWaiterType = core::ptr::null_mut();
    let mut cur = __ww_waiter_last(lock);
    while !cur.is_null() {
        if (*cur).ww_ctx.is_null() { cur = __ww_waiter_prev(lock, cur); continue; }
        if __ww_ctx_less(ww_ctx, (*cur).ww_ctx) {
            if is_wait_die { let ret = __ww_mutex_kill(lock, ww_ctx); if ret != 0 { return ret; } }
            break;
        }
        pos = cur; __ww_mutex_die(lock, cur, ww_ctx, wake_q); cur = __ww_waiter_prev(lock, cur);
    }
    __ww_waiter_add(lock, waiter, pos);
    if !is_wait_die {
        let ww: *mut ww_mutex = container_of!(lock, ww_mutex, base); smp_mb(); __ww_mutex_wound(lock, ww_ctx, (*ww).ctx, wake_q);
    }
    0
}

pub unsafe fn __ww_mutex_unlock(lock: *mut ww_mutex) {
    if !(*lock).ctx.is_null() { #[cfg(feature = "DEBUG_WW_MUTEXES")] { DEBUG_LOCKS_WARN_ON((*(*lock).ctx).acquired == 0); } if (*(*lock).ctx).acquired > 0 { (*(*lock).ctx).acquired -= 1; } (*lock).ctx = core::ptr::null_mut(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
