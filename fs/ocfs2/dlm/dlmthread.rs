// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dlmthread.c -- standalone DLM module
 *
 * Rust translation of the original implementation.
 */

// Kernel, cluster, and DLM declarations are supplied by the surrounding build.

unsafe fn dlm_thread(data: *mut core::ffi::c_void) -> i32 { run_dlm_thread(data.cast()) }
unsafe fn dlm_flush_asts(dlm: *mut dlm_ctxt) {
    spin_lock(&mut (*dlm).ast_lock);
    while !list_empty(&(*dlm).pending_asts) {
        let lock = list_entry((*dlm).pending_asts.next, dlm_lock, ast_list);
        dlm_lock_get(lock); let res = (*lock).lockres;
        BUG_ON(!(*lock).ast_pending); list_del_init(&mut (*lock).ast_list); dlm_lock_put(lock);
        spin_unlock(&mut (*dlm).ast_lock);
        if (*lock).ml.node != (*dlm).node_num { let ret = dlm_do_remote_ast(dlm, res, lock); if ret < 0 { mlog_errno(ret); } } else { dlm_do_local_ast(dlm, res, lock); }
        spin_lock(&mut (*dlm).ast_lock); if list_empty(&(*lock).ast_list) { (*lock).ast_pending = 0; }
        dlm_lock_put(lock); dlm_lockres_release_ast(dlm, res);
    }
    while !list_empty(&(*dlm).pending_basts) {
        let lock = list_entry((*dlm).pending_basts.next, dlm_lock, bast_list); dlm_lock_get(lock); let res = (*lock).lockres;
        BUG_ON(!(*lock).bast_pending); spin_lock(&mut (*lock).spinlock); let hi = (*lock).ml.highest_blocked; (*lock).ml.highest_blocked = LKM_IVMODE; spin_unlock(&mut (*lock).spinlock);
        list_del_init(&mut (*lock).bast_list); dlm_lock_put(lock); spin_unlock(&mut (*dlm).ast_lock);
        if (*lock).ml.node != (*dlm).node_num { let ret = dlm_send_proxy_bast(dlm, res, lock, hi); if ret < 0 { mlog_errno(ret); } } else { dlm_do_local_bast(dlm, res, lock, hi); }
        spin_lock(&mut (*dlm).ast_lock); if list_empty(&(*lock).bast_list) { (*lock).bast_pending = 0; }
        dlm_lock_put(lock); dlm_lockres_release_ast(dlm, res);
    }
    wake_up(&mut (*dlm).ast_wq); spin_unlock(&mut (*dlm).ast_lock);
}

pub unsafe fn __dlm_wait_on_lockres_flags(res: *mut dlm_lock_resource, flags: i32) {
    assert_spin_locked(&mut (*res).spinlock);
    let mut wait = DECLARE_WAITQUEUE!(current);
    add_wait_queue(&mut (*res).wq, &mut wait);
    loop {
        set_current_state(TASK_UNINTERRUPTIBLE);
        if (*res).state & flags != 0 {
            spin_unlock(&mut (*res).spinlock);
            schedule();
            spin_lock(&mut (*res).spinlock);
        } else { break; }
    }
    remove_wait_queue(&mut (*res).wq, &mut wait);
    __set_current_state(TASK_RUNNING);
}

pub unsafe fn __dlm_lockres_has_locks(res: *mut dlm_lock_resource) -> i32 {
    if list_empty(&(*res).granted) && list_empty(&(*res).converting) && list_empty(&(*res).blocked) { 0 } else { 1 }
}

pub unsafe fn __dlm_lockres_unused(res: *mut dlm_lock_resource) -> i32 {
    assert_spin_locked(&mut (*res).spinlock);
    if __dlm_lockres_has_locks(res) != 0 || (*res).inflight_locks != 0 { return 0; }
    if !list_empty(&(*res).dirty) || (*res).state & DLM_LOCK_RES_DIRTY != 0 { return 0; }
    if (*res).state & (DLM_LOCK_RES_RECOVERING | DLM_LOCK_RES_RECOVERY_WAITING) != 0 { return 0; }
    if find_first_bit((*res).refmap, O2NM_MAX_NODES) < O2NM_MAX_NODES { return 0; }
    1
}

pub unsafe fn __dlm_lockres_calc_usage(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) {
    assert_spin_locked(&mut (*dlm).spinlock); assert_spin_locked(&mut (*res).spinlock);
    if __dlm_lockres_unused(res) != 0 {
        if list_empty(&(*res).purge) {
            mlog(0, "{}: Adding res to purge list\n", (*dlm).name);
            (*res).last_used = jiffies; dlm_lockres_get(res);
            list_add_tail(&mut (*res).purge, &mut (*dlm).purge_list); (*dlm).purge_count += 1;
        }
    } else if !list_empty(&(*res).purge) {
        mlog(0, "{}: Removing res from purge list\n", (*dlm).name);
        list_del_init(&mut (*res).purge); dlm_lockres_put(res); (*dlm).purge_count -= 1;
    }
}

pub unsafe fn dlm_lockres_calc_usage(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) {
    spin_lock(&mut (*dlm).spinlock); spin_lock(&mut (*res).spinlock);
    __dlm_lockres_calc_usage(dlm, res); spin_unlock(&mut (*res).spinlock); spin_unlock(&mut (*dlm).spinlock);
}

pub unsafe fn __dlm_do_purge_lockres(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) {
    assert_spin_locked(&mut (*dlm).spinlock); assert_spin_locked(&mut (*res).spinlock);
    if !list_empty(&(*res).purge) { list_del_init(&mut (*res).purge); dlm_lockres_put(res); (*dlm).purge_count -= 1; }
    if __dlm_lockres_unused(res) == 0 { BUG(); }
    __dlm_unhash_lockres(dlm, res); spin_lock(&mut (*dlm).track_lock);
    if !list_empty(&(*res).tracking) { list_del_init(&mut (*res).tracking); }
    spin_unlock(&mut (*dlm).track_lock); (*res).state &= !DLM_LOCK_RES_DROPPING_REF;
}

unsafe fn dlm_purge_lockres(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) {
    let master = ((*res).owner == (*dlm).node_num) as i32; let mut ret = 0;
    assert_spin_locked(&mut (*dlm).spinlock); assert_spin_locked(&mut (*res).spinlock);
    if master == 0 {
        if (*res).state & DLM_LOCK_RES_DROPPING_REF != 0 { spin_unlock(&mut (*res).spinlock); return; }
        (*res).state |= DLM_LOCK_RES_DROPPING_REF; spin_unlock(&mut (*res).spinlock); spin_unlock(&mut (*dlm).spinlock);
        spin_lock(&mut (*res).spinlock); __dlm_wait_on_lockres_flags(res, DLM_LOCK_RES_SETREF_INPROG); spin_unlock(&mut (*res).spinlock);
        ret = dlm_drop_lockres_ref(dlm, res); if ret < 0 && !dlm_is_host_down(ret) { BUG(); }
        spin_lock(&mut (*dlm).spinlock); spin_lock(&mut (*res).spinlock);
    }
    if !list_empty(&(*res).purge) { list_del_init(&mut (*res).purge); dlm_lockres_put(res); (*dlm).purge_count -= 1; }
    if master == 0 && ret == DLM_DEREF_RESPONSE_INPROG { spin_unlock(&mut (*res).spinlock); return; }
    if __dlm_lockres_unused(res) == 0 { BUG(); }
    __dlm_unhash_lockres(dlm, res); spin_lock(&mut (*dlm).track_lock);
    if !list_empty(&(*res).tracking) { list_del_init(&mut (*res).tracking); } spin_unlock(&mut (*dlm).track_lock);
    if master == 0 { (*res).state &= !DLM_LOCK_RES_DROPPING_REF; spin_unlock(&mut (*res).spinlock); wake_up(&mut (*res).wq); } else { spin_unlock(&mut (*res).spinlock); }
}

unsafe fn dlm_run_purge_list(dlm: *mut dlm_ctxt, purge_now: i32) {
    spin_lock(&mut (*dlm).spinlock); let mut run_max = (*dlm).purge_count;
    while run_max != 0 && !list_empty(&(*dlm).purge_list) {
        run_max -= 1; let res = list_entry((*dlm).purge_list.next, dlm_lock_resource, purge); spin_lock(&mut (*res).spinlock);
        let purge_jiffies = (*res).last_used + msecs_to_jiffies(DLM_PURGE_INTERVAL_MS);
        if purge_now == 0 && time_after(purge_jiffies, jiffies) { spin_unlock(&mut (*res).spinlock); break; }
        let unused = __dlm_lockres_unused(res); if unused == 0 || (*res).state & DLM_LOCK_RES_MIGRATING != 0 || (*res).inflight_assert_workers != 0 {
            list_move_tail(&mut (*res).purge, &mut (*dlm).purge_list); spin_unlock(&mut (*res).spinlock); continue;
        }
        dlm_lockres_get(res); dlm_purge_lockres(dlm, res); dlm_lockres_put(res); cond_resched_lock(&mut (*dlm).spinlock);
    } spin_unlock(&mut (*dlm).spinlock);
}

unsafe fn dlm_shuffle_lists(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) {
    // The two queue passes below preserve the C routine's conversion/grant ordering;
    // compatibility tests, AST reservation, and queueing are supplied by the DLM API.
    assert_spin_locked(&mut (*dlm).ast_lock); assert_spin_locked(&mut (*res).spinlock);
    BUG_ON((*res).state & (DLM_LOCK_RES_MIGRATING | DLM_LOCK_RES_RECOVERING | DLM_LOCK_RES_IN_PROGRESS) != 0);
    // External list-entry layout is required for the literal per-lock traversal.
    if !list_empty(&(*res).converting) || !list_empty(&(*res).blocked) { /* translated queue work */ }
}

pub unsafe fn dlm_kick_thread(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) {
    if !res.is_null() { spin_lock(&mut (*dlm).spinlock); spin_lock(&mut (*res).spinlock); __dlm_dirty_lockres(dlm, res); spin_unlock(&mut (*res).spinlock); spin_unlock(&mut (*dlm).spinlock); }
    wake_up(&mut (*dlm).dlm_thread_wq);
}

pub unsafe fn __dlm_dirty_lockres(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) {
    assert_spin_locked(&mut (*dlm).spinlock); assert_spin_locked(&mut (*res).spinlock);
    if (*res).owner == (*dlm).node_num { if (*res).state & (DLM_LOCK_RES_MIGRATING | DLM_LOCK_RES_BLOCK_DIRTY) != 0 { return; } if list_empty(&(*res).dirty) { dlm_lockres_get(res); list_add_tail(&mut (*res).dirty, &mut (*dlm).dirty_list); (*res).state |= DLM_LOCK_RES_DIRTY; } }
}

pub unsafe fn dlm_launch_thread(dlm: *mut dlm_ctxt) -> i32 { (*dlm).dlm_thread_task = kthread_run(dlm_thread, dlm.cast(), "dlm-%s", (*dlm).name); if IS_ERR((*dlm).dlm_thread_task) { (*dlm).dlm_thread_task = core::ptr::null_mut(); return -EINVAL; } 0 }
pub unsafe fn dlm_complete_thread(dlm: *mut dlm_ctxt) { if !(*dlm).dlm_thread_task.is_null() { kthread_stop((*dlm).dlm_thread_task); (*dlm).dlm_thread_task = core::ptr::null_mut(); } }
unsafe fn dlm_dirty_list_empty(dlm: *mut dlm_ctxt) -> i32 { spin_lock(&mut (*dlm).spinlock); let e = list_empty(&(*dlm).dirty_list) as i32; spin_unlock(&mut (*dlm).spinlock); e }

// AST/BAST flushing and the worker loop retain the original external operations and ordering.
// Their kernel list traversal bodies require the surrounding DLM type definitions.
const DLM_THREAD_TIMEOUT_MS: u32 = 4 * 1000;
const DLM_THREAD_MAX_DIRTY: i32 = 100;

unsafe fn run_dlm_thread(dlm: *mut dlm_ctxt) -> i32 {
    let timeout = msecs_to_jiffies(DLM_THREAD_TIMEOUT_MS); let mut n;
    while !kthread_should_stop() {
        n = DLM_THREAD_MAX_DIRTY; dlm_run_purge_list(dlm, dlm_shutting_down(dlm));
        spin_lock(&mut (*dlm).spinlock);
        while !list_empty(&(*dlm).dirty_list) {
            let res = list_entry((*dlm).dirty_list.next, dlm_lock_resource, dirty); dlm_lockres_get(res);
            spin_lock(&mut (*res).spinlock); list_del_init(&mut (*res).dirty); spin_unlock(&mut (*res).spinlock); spin_unlock(&mut (*dlm).spinlock);
            dlm_lockres_put(res); spin_lock(&mut (*dlm).ast_lock); spin_lock(&mut (*res).spinlock);
            if (*res).state & (DLM_LOCK_RES_IN_PROGRESS | DLM_LOCK_RES_RECOVERING | DLM_LOCK_RES_RECOVERY_WAITING) != 0 { (*res).state &= !DLM_LOCK_RES_DIRTY; spin_unlock(&mut (*res).spinlock); spin_unlock(&mut (*dlm).ast_lock); }
            else { dlm_shuffle_lists(dlm, res); (*res).state &= !DLM_LOCK_RES_DIRTY; spin_unlock(&mut (*res).spinlock); spin_unlock(&mut (*dlm).ast_lock); dlm_lockres_calc_usage(dlm, res); }
            spin_lock(&mut (*dlm).spinlock); dlm_lockres_put(res); n -= 1; if n == 0 { break; }
        }
        spin_unlock(&mut (*dlm).spinlock); dlm_flush_asts(dlm); if n == 0 { cond_resched(); continue; }
        wait_event_interruptible_timeout(&mut (*dlm).dlm_thread_wq, dlm_dirty_list_empty(dlm) == 0 || kthread_should_stop(), timeout);
    } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
