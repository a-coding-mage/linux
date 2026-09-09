// SPDX-License-Identifier: GPL-2.0-only
//
// Translation of recoverd.c. Dependencies and kernel primitives are supplied
// by the surrounding DLM implementation.

unsafe fn dlm_create_masters_list(ls: *mut dlm_ls) -> c_int {
    let mut r: *mut dlm_rsb;
    let mut error: c_int = 0;

    write_lock_bh(&mut (*ls).ls_masters_lock);
    if !list_empty(&mut (*ls).ls_masters_list) {
        log_error(ls, "root list not empty");
        error = -EINVAL;
        goto_out!();
    }

    read_lock_bh(&mut (*ls).ls_rsbtbl_lock);
    list_for_each_entry!(r, &mut (*ls).ls_slow_active, res_slow_list, {
        if (*r).res_nodeid != 0 { continue; }
        list_add(&mut (*r).res_masters_list, &mut (*ls).ls_masters_list);
        dlm_hold_rsb(r);
    });
    read_unlock_bh(&mut (*ls).ls_rsbtbl_lock);
    goto_out!();

    write_unlock_bh(&mut (*ls).ls_masters_lock);
    error
}

unsafe fn dlm_release_masters_list(ls: *mut dlm_ls) {
    let mut r: *mut dlm_rsb;
    let mut safe: *mut dlm_rsb;
    write_lock_bh(&mut (*ls).ls_masters_lock);
    list_for_each_entry_safe!(r, safe, &mut (*ls).ls_masters_list, res_masters_list, {
        list_del_init(&mut (*r).res_masters_list);
        dlm_put_rsb(r);
    });
    write_unlock_bh(&mut (*ls).ls_masters_lock);
}

unsafe fn dlm_create_root_list(ls: *mut dlm_ls, root_list: *mut list_head) {
    let mut r: *mut dlm_rsb;
    read_lock_bh(&mut (*ls).ls_rsbtbl_lock);
    list_for_each_entry!(r, &mut (*ls).ls_slow_active, res_slow_list, {
        list_add(&mut (*r).res_root_list, root_list);
        dlm_hold_rsb(r);
    });
    WARN_ON_ONCE(!list_empty(&mut (*ls).ls_slow_inactive));
    read_unlock_bh(&mut (*ls).ls_rsbtbl_lock);
}

unsafe fn dlm_release_root_list(root_list: *mut list_head) {
    let mut r: *mut dlm_rsb;
    let mut safe: *mut dlm_rsb;
    list_for_each_entry_safe!(r, safe, root_list, res_root_list, {
        list_del_init(&mut (*r).res_root_list);
        dlm_put_rsb(r);
    });
}

unsafe fn enable_locking(ls: *mut dlm_ls, seq: u64) -> c_int {
    let mut error = -EINTR;
    write_lock_bh(&mut (*ls).ls_recv_active);
    spin_lock_bh(&mut (*ls).ls_recover_lock);
    if (*ls).ls_recover_seq == seq {
        set_bit(LSFL_RUNNING, &mut (*ls).ls_flags);
        resume_scan_timer(ls);
        up_write(&mut (*ls).ls_in_recovery);
        clear_bit(LSFL_RECOVER_LOCK, &mut (*ls).ls_flags);
        error = 0;
    }
    spin_unlock_bh(&mut (*ls).ls_recover_lock);
    write_unlock_bh(&mut (*ls).ls_recv_active);
    error
}

unsafe fn ls_recover(ls: *mut dlm_ls, rv: *mut dlm_recover) -> c_int {
    let mut root_list: list_head = core::mem::zeroed();
    let mut start: c_ulong;
    let mut error: c_int;
    let mut neg: c_int = 0;

    log_rinfo(ls, "dlm_recover %llu", (*rv).seq as c_ulonglong);
    mutex_lock(&mut (*ls).ls_recoverd_active);
    dlm_callback_suspend(ls);
    dlm_clear_inactive(ls);
    dlm_create_root_list(ls, &mut root_list);

    error = dlm_recover_members(ls, rv, &mut neg);
    if error != 0 { log_rinfo(ls, "dlm_recover_members error %d", error); goto fail_root_list; }
    dlm_recover_dir_nodeid(ls, &mut root_list);
    error = dlm_create_masters_list(ls);
    if error != 0 { log_rinfo(ls, "dlm_create_masters_list error %d", error); goto fail_root_list; }
    (*ls).ls_recover_locks_in = 0;
    dlm_set_recover_status(ls, DLM_RS_NODES);
    error = dlm_recover_members_wait(ls, (*rv).seq);
    if error != 0 { log_rinfo(ls, "dlm_recover_members_wait error %d", error); dlm_release_masters_list(ls); goto fail_root_list; }
    start = jiffies;
    error = dlm_recover_directory(ls, (*rv).seq);
    if error != 0 { log_rinfo(ls, "dlm_recover_directory error %d", error); dlm_release_masters_list(ls); goto fail_root_list; }
    dlm_set_recover_status(ls, DLM_RS_DIR);
    error = dlm_recover_directory_wait(ls, (*rv).seq);
    if error != 0 { log_rinfo(ls, "dlm_recover_directory_wait error %d", error); dlm_release_masters_list(ls); goto fail_root_list; }
    dlm_release_masters_list(ls);
    dlm_recover_waiters_pre(ls);
    if dlm_recovery_stopped(ls) { error = -EINTR; goto fail_root_list; }
    if neg != 0 || dlm_no_directory(ls) {
        dlm_recover_purge(ls, &mut root_list);
        error = dlm_recover_masters(ls, (*rv).seq, &mut root_list);
        if error != 0 { log_rinfo(ls, "dlm_recover_masters error %d", error); goto fail_root_list; }
        error = dlm_recover_locks(ls, (*rv).seq, &mut root_list);
        if error != 0 { log_rinfo(ls, "dlm_recover_locks error %d", error); goto fail_root_list; }
        dlm_set_recover_status(ls, DLM_RS_LOCKS);
        error = dlm_recover_locks_wait(ls, (*rv).seq);
        if error != 0 { log_rinfo(ls, "dlm_recover_locks_wait error %d", error); goto fail_root_list; }
        log_rinfo(ls, "dlm_recover_locks %u in", (*ls).ls_recover_locks_in);
        dlm_recover_rsbs(ls, &mut root_list);
    } else {
        dlm_set_recover_status(ls, DLM_RS_LOCKS);
        error = dlm_recover_locks_wait(ls, (*rv).seq);
        if error != 0 { log_rinfo(ls, "dlm_recover_locks_wait error %d", error); goto fail_root_list; }
    }
    dlm_release_root_list(&mut root_list);
    dlm_purge_requestqueue(ls);
    dlm_set_recover_status(ls, DLM_RS_DONE);
    error = dlm_recover_done_wait(ls, (*rv).seq);
    if error != 0 { log_rinfo(ls, "dlm_recover_done_wait error %d", error); goto fail; }
    dlm_clear_members_gone(ls);
    dlm_callback_resume(ls);
    error = enable_locking(ls, (*rv).seq);
    if error != 0 { log_rinfo(ls, "enable_locking error %d", error); goto fail; }
    error = dlm_process_requestqueue(ls);
    if error != 0 { log_rinfo(ls, "dlm_process_requestqueue error %d", error); goto fail; }
    error = dlm_recover_waiters_post(ls);
    if error != 0 { log_rinfo(ls, "dlm_recover_waiters_post error %d", error); goto fail; }
    dlm_recover_grant(ls);
    log_rinfo(ls, "dlm_recover %llu generation %u done: %u ms", (*rv).seq as c_ulonglong, (*ls).ls_generation, jiffies_to_msecs(jiffies - start));
    mutex_unlock(&mut (*ls).ls_recoverd_active);
    return 0;
fail_root_list:
    dlm_release_root_list(&mut root_list);
fail:
    mutex_unlock(&mut (*ls).ls_recoverd_active);
    error
}

// The remaining thread lifecycle functions preserve the kernel kthread loop
// and call into externally supplied DLM/kernel primitives.
unsafe fn do_ls_recovery(ls: *mut dlm_ls) {
    let mut rv = (*ls).ls_recover_args;
    let mut error;
    spin_lock_bh(&mut (*ls).ls_recover_lock);
    (*ls).ls_recover_args = core::ptr::null_mut();
    if !rv.is_null() && (*ls).ls_recover_seq == (*rv).seq { clear_bit(LSFL_RECOVER_STOP, &mut (*ls).ls_flags); }
    spin_unlock_bh(&mut (*ls).ls_recover_lock);
    if !rv.is_null() {
        error = ls_recover(ls, rv);
        match error {
            0 => { (*ls).ls_recovery_result = 0; complete(&mut (*ls).ls_recovery_done); dlm_lsop_recover_done(ls); }
            -EINTR => { log_rinfo(ls, "do_ls_recovery %llu interrupted and should be queued to run again", (*rv).seq as c_ulonglong); }
            _ => { log_rinfo(ls, "do_ls_recovery %llu error %d", (*rv).seq as c_ulonglong, error); (*ls).ls_recovery_result = error; complete(&mut (*ls).ls_recovery_done); }
        }
        kfree((*rv).nodes);
        kfree(rv);
    }
}

unsafe fn dlm_recoverd(arg: *mut core::ffi::c_void) -> c_int {
    let ls = dlm_find_lockspace_local(arg);
    if ls.is_null() { log_print("dlm_recoverd: no lockspace %p", arg); return -1; }
    down_write(&mut (*ls).ls_in_recovery); set_bit(LSFL_RECOVER_LOCK, &mut (*ls).ls_flags); wake_up(&mut (*ls).ls_recover_lock_wait);
    loop {
        set_current_state(TASK_INTERRUPTIBLE);
        if kthread_should_stop() { set_current_state(TASK_RUNNING); break; }
        if !test_bit(LSFL_RECOVER_WORK, &(*ls).ls_flags) && !test_bit(LSFL_RECOVER_DOWN, &(*ls).ls_flags) { if kthread_should_stop() { break; } schedule(); }
        set_current_state(TASK_RUNNING);
        if test_and_clear_bit(LSFL_RECOVER_DOWN, &mut (*ls).ls_flags) { down_write(&mut (*ls).ls_in_recovery); set_bit(LSFL_RECOVER_LOCK, &mut (*ls).ls_flags); wake_up(&mut (*ls).ls_recover_lock_wait); }
        if test_and_clear_bit(LSFL_RECOVER_WORK, &mut (*ls).ls_flags) { do_ls_recovery(ls); }
    }
    if test_bit(LSFL_RECOVER_LOCK, &(*ls).ls_flags) { up_write(&mut (*ls).ls_in_recovery); }
    dlm_put_lockspace(ls); 0
}

pub unsafe fn dlm_recoverd_start(ls: *mut dlm_ls) -> c_int {
    let p = kthread_run(dlm_recoverd, ls, "dlm_recoverd");
    if IS_ERR(p) { PTR_ERR(p) } else { (*ls).ls_recoverd_task = p; 0 }
}

pub unsafe fn dlm_recoverd_stop(ls: *mut dlm_ls) { kthread_stop((*ls).ls_recoverd_task); }
pub unsafe fn dlm_recoverd_suspend(ls: *mut dlm_ls) { wake_up(&mut (*ls).ls_wait_general); mutex_lock(&mut (*ls).ls_recoverd_active); }
pub unsafe fn dlm_recoverd_resume(ls: *mut dlm_ls) { mutex_unlock(&mut (*ls).ls_recoverd_active); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
