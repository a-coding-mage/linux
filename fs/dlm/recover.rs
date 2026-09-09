// SPDX-License-Identifier: GPL-2.0-only
// Translation of recover.c. Types, constants, macros, and functions supplied by
// the surrounding DLM implementation remain external dependencies.

unsafe fn dlm_wait_function(ls: *mut dlm_ls, testfn: unsafe fn(*mut dlm_ls) -> i32) -> i32 {
    let mut error = 0;
    loop {
        let rv = wait_event_timeout((*ls).ls_wait_general,
            testfn(ls) != 0 || dlm_recovery_stopped(ls) != 0,
            dlm_config.ci_recover_timer * HZ);
        if rv != 0 { break; }
        if test_bit(LSFL_RCOM_WAIT, &(*ls).ls_flags) != 0 {
            log_debug(ls, "dlm_wait_function timed out");
            return -ETIMEDOUT;
        }
    }
    if dlm_recovery_stopped(ls) != 0 {
        log_debug(ls, "dlm_wait_function aborted");
        error = -EINTR;
    }
    error
}

unsafe fn dlm_recover_status(ls: *mut dlm_ls) -> u32 {
    spin_lock_bh(&mut (*ls).ls_recover_lock);
    let status = (*ls).ls_recover_status;
    spin_unlock_bh(&mut (*ls).ls_recover_lock);
    status
}

unsafe fn _set_recover_status(ls: *mut dlm_ls, status: u32) { (*ls).ls_recover_status |= status; }

unsafe fn dlm_set_recover_status(ls: *mut dlm_ls, status: u32) {
    spin_lock_bh(&mut (*ls).ls_recover_lock);
    _set_recover_status(ls, status);
    spin_unlock_bh(&mut (*ls).ls_recover_lock);
}

unsafe fn wait_status_all(ls: *mut dlm_ls, wait_status: u32, save_slots: i32, seq: u64) -> i32 {
    let rc = (*ls).ls_recover_buf;
    let mut error = 0;
    for memb in list_for_each_entry::<dlm_member>(&(*ls).ls_nodes) {
        let mut delay = 0;
        loop {
            if dlm_recovery_stopped(ls) != 0 { error = -EINTR; return error; }
            error = dlm_rcom_status(ls, memb.nodeid, 0, seq);
            if error != 0 { return error; }
            if save_slots != 0 { dlm_slot_save(ls, rc, memb); }
            if le32_to_cpu((*rc).rc_result) & wait_status != 0 { break; }
            if delay < 1000 { delay += 20; }
            msleep(delay);
        }
    }
    error
}

unsafe fn wait_status_low(ls: *mut dlm_ls, wait_status: u32, status_flags: u32, seq: u64) -> i32 {
    let rc = (*ls).ls_recover_buf;
    let mut delay = 0;
    let nodeid = (*ls).ls_low_nodeid;
    loop {
        if dlm_recovery_stopped(ls) != 0 { return -EINTR; }
        let error = dlm_rcom_status(ls, nodeid, status_flags, seq);
        if error != 0 { return error; }
        if le32_to_cpu((*rc).rc_result) & wait_status != 0 { break; }
        if delay < 1000 { delay += 20; }
        msleep(delay);
    }
    0
}

unsafe fn wait_status(ls: *mut dlm_ls, status: u32, seq: u64) -> i32 {
    let status_all = status << 1;
    if (*ls).ls_low_nodeid == dlm_our_nodeid() {
        let error = wait_status_all(ls, status, 0, seq);
        if error == 0 { dlm_set_recover_status(ls, status_all); }
        error
    } else { wait_status_low(ls, status_all, 0, seq) }
}

unsafe fn dlm_recover_members_wait(ls: *mut dlm_ls, seq: u64) -> i32 {
    for memb in list_for_each_entry::<dlm_member>(&(*ls).ls_nodes) { memb.slot = -1; memb.generation = 0; }
    if (*ls).ls_low_nodeid == dlm_our_nodeid() {
        let error = wait_status_all(ls, DLM_RS_NODES, 1, seq);
        if error != 0 { return error; }
        let mut num_slots = 0; let mut slots_size = 0; let mut slots = core::ptr::null_mut(); let mut gen = 0;
        let rv = dlm_slots_assign(ls, &mut num_slots, &mut slots_size, &mut slots, &mut gen);
        if rv == 0 {
            spin_lock_bh(&mut (*ls).ls_recover_lock);
            _set_recover_status(ls, DLM_RS_NODES_ALL);
            (*ls).ls_num_slots = num_slots; (*ls).ls_slots_size = slots_size; (*ls).ls_slots = slots; (*ls).ls_generation = gen;
            spin_unlock_bh(&mut (*ls).ls_recover_lock);
        } else { dlm_set_recover_status(ls, DLM_RS_NODES_ALL); }
        error
    } else {
        let error = wait_status_low(ls, DLM_RS_NODES_ALL, DLM_RSF_NEED_SLOTS, seq);
        if error == 0 { dlm_slots_copy_in(ls); }
        error
    }
}

unsafe fn dlm_recover_directory_wait(ls: *mut dlm_ls, seq: u64) -> i32 { wait_status(ls, DLM_RS_DIR, seq) }
unsafe fn dlm_recover_locks_wait(ls: *mut dlm_ls, seq: u64) -> i32 { wait_status(ls, DLM_RS_LOCKS, seq) }
unsafe fn dlm_recover_done_wait(ls: *mut dlm_ls, seq: u64) -> i32 { wait_status(ls, DLM_RS_DONE, seq) }

unsafe fn recover_list_empty(ls: *mut dlm_ls) -> i32 {
    spin_lock_bh(&mut (*ls).ls_recover_list_lock); let empty = list_empty(&(*ls).ls_recover_list); spin_unlock_bh(&mut (*ls).ls_recover_list_lock); empty
}
unsafe fn recover_list_add(r: *mut dlm_rsb) { let ls = (*r).res_ls; spin_lock_bh(&mut (*ls).ls_recover_list_lock); if list_empty(&(*r).res_recover_list) { list_add_tail(&mut (*r).res_recover_list, &mut (*ls).ls_recover_list); (*ls).ls_recover_list_count += 1; dlm_hold_rsb(r); } spin_unlock_bh(&mut (*ls).ls_recover_list_lock); }
unsafe fn recover_list_del(r: *mut dlm_rsb) { let ls = (*r).res_ls; spin_lock_bh(&mut (*ls).ls_recover_list_lock); list_del_init(&mut (*r).res_recover_list); (*ls).ls_recover_list_count -= 1; spin_unlock_bh(&mut (*ls).ls_recover_list_lock); dlm_put_rsb(r); }
unsafe fn recover_list_clear(ls: *mut dlm_ls) { spin_lock_bh(&mut (*ls).ls_recover_list_lock); for r in list_for_each_entry_safe::<dlm_rsb>(&(*ls).ls_recover_list) { list_del_init(&mut r.res_recover_list); r.res_recover_locks_count = 0; dlm_put_rsb(r); (*ls).ls_recover_list_count -= 1; } if (*ls).ls_recover_list_count != 0 { log_error(ls, "warning: recover_list_count %d", (*ls).ls_recover_list_count); (*ls).ls_recover_list_count = 0; } spin_unlock_bh(&mut (*ls).ls_recover_list_lock); }

unsafe fn recover_xa_empty(ls: *mut dlm_ls) -> i32 { spin_lock_bh(&mut (*ls).ls_recover_xa_lock); let e = if (*ls).ls_recover_list_count != 0 { 0 } else { 1 }; spin_unlock_bh(&mut (*ls).ls_recover_xa_lock); e }
unsafe fn recover_xa_add(r: *mut dlm_rsb) -> i32 { let ls = (*r).res_ls; let limit = xa_limit { min: 1, max: UINT_MAX }; let mut id = 0u32; spin_lock_bh(&mut (*ls).ls_recover_xa_lock); if (*r).res_id != 0 { spin_unlock_bh(&mut (*ls).ls_recover_xa_lock); return -1; } let rv = xa_alloc(&mut (*ls).ls_recover_xa, &mut id, r, limit, GFP_ATOMIC); if rv < 0 { spin_unlock_bh(&mut (*ls).ls_recover_xa_lock); return rv; } (*r).res_id = id; (*ls).ls_recover_list_count += 1; dlm_hold_rsb(r); spin_unlock_bh(&mut (*ls).ls_recover_xa_lock); 0 }
unsafe fn recover_xa_del(r: *mut dlm_rsb) { let ls = (*r).res_ls; spin_lock_bh(&mut (*ls).ls_recover_xa_lock); xa_erase_bh(&mut (*ls).ls_recover_xa, (*r).res_id); (*r).res_id = 0; (*ls).ls_recover_list_count -= 1; spin_unlock_bh(&mut (*ls).ls_recover_xa_lock); dlm_put_rsb(r); }
unsafe fn recover_xa_find(ls: *mut dlm_ls, id: u64) -> *mut dlm_rsb { spin_lock_bh(&mut (*ls).ls_recover_xa_lock); let r = xa_load(&(*ls).ls_recover_xa, id as i32); spin_unlock_bh(&mut (*ls).ls_recover_xa_lock); r }
unsafe fn recover_xa_clear(ls: *mut dlm_ls) { spin_lock_bh(&mut (*ls).ls_recover_xa_lock); for (id, r) in xa_for_each(&mut (*ls).ls_recover_xa) { xa_erase_bh(&mut (*ls).ls_recover_xa, id); r.res_id = 0; r.res_recover_locks_count = 0; (*ls).ls_recover_list_count -= 1; dlm_put_rsb(r); } if (*ls).ls_recover_list_count != 0 { log_error(ls, "warning: recover_list_count %d", (*ls).ls_recover_list_count); (*ls).ls_recover_list_count = 0; } spin_unlock_bh(&mut (*ls).ls_recover_xa_lock); }

unsafe fn set_lock_master(queue: *mut list_head, nodeid: i32) { for lkb in list_for_each_entry::<dlm_lkb>(queue) { if test_bit(DLM_IFL_MSTCPY_BIT, &lkb.lkb_iflags) == 0 { lkb.lkb_nodeid = nodeid; lkb.lkb_remid = 0; } } }
unsafe fn set_master_lkbs(r: *mut dlm_rsb) { set_lock_master(&mut (*r).res_grantqueue, (*r).res_nodeid); set_lock_master(&mut (*r).res_convertqueue, (*r).res_nodeid); set_lock_master(&mut (*r).res_waitqueue, (*r).res_nodeid); }
unsafe fn set_new_master(r: *mut dlm_rsb) { set_master_lkbs(r); rsb_set_flag(r, RSB_NEW_MASTER); rsb_set_flag(r, RSB_NEW_MASTER2); }

unsafe fn recover_master(r: *mut dlm_rsb, count: *mut u32, seq: u64) -> i32 {
    let ls = (*r).res_ls; if (*r).res_nodeid != -1 && is_master(r) != 0 { return 0; }
    let removed = if (*r).res_nodeid != -1 { dlm_is_removed(ls, (*r).res_nodeid) } else { 0 };
    if removed == 0 && rsb_flag(r, RSB_NEW_MASTER) == 0 { return 0; }
    let our = dlm_our_nodeid(); let dir = dlm_dir_nodeid(r); let error;
    if dir == our { if removed != 0 { (*r).res_master_nodeid = our; (*r).res_nodeid = 0; } set_new_master(r); error = 0; } else { recover_xa_add(r); error = dlm_send_rcom_lookup(r, dir, seq); }
    *count += 1; error
}

unsafe fn recover_master_static(r: *mut dlm_rsb, count: *mut u32) -> i32 { let dir = dlm_dir_nodeid(r); let new_master = if dir == dlm_our_nodeid() { 0 } else { dir }; dlm_purge_mstcpy_locks(r); (*r).res_master_nodeid = dir; (*r).res_nodeid = new_master; set_new_master(r); *count += 1; 0 }

unsafe fn dlm_recover_masters(ls: *mut dlm_ls, seq: u64, root_list: *const list_head) -> i32 {
    let mut total = 0; let mut count = 0; let nodir = dlm_no_directory(ls); log_rinfo(ls, "dlm_recover_masters");
    for r in list_for_each_entry::<dlm_rsb>(root_list) { if dlm_recovery_stopped(ls) != 0 { recover_xa_clear(ls); return -EINTR; } lock_rsb(r); let error = if nodir != 0 { recover_master_static(r, &mut count) } else { recover_master(r, &mut count, seq) }; unlock_rsb(r); cond_resched(); total += 1; if error != 0 { recover_xa_clear(ls); return error; } }
    log_rinfo(ls, "dlm_recover_masters %u of %u", count, total); let error = dlm_wait_function(ls, recover_xa_empty); if error != 0 { recover_xa_clear(ls); } error
}

unsafe fn dlm_recover_master_reply(ls: *mut dlm_ls, rc: *const dlm_rcom) -> i32 { let r = recover_xa_find(ls, le64_to_cpu((*rc).rc_id)); if r.is_null() { log_error(ls, "dlm_recover_master_reply no id %llx", le64_to_cpu((*rc).rc_id)); return 0; } let ret = le32_to_cpu((*rc).rc_result); let new_master = if ret == dlm_our_nodeid() { 0 } else { ret as i32 }; lock_rsb(r); (*r).res_master_nodeid = ret as i32; (*r).res_nodeid = new_master; set_new_master(r); unlock_rsb(r); recover_xa_del(r); if recover_xa_empty(ls) != 0 { wake_up(&mut (*ls).ls_wait_general); } 0 }

unsafe fn recover_locks_queue(r: *mut dlm_rsb, head: *mut list_head, seq: u64) -> i32 { let mut error = 0; for lkb in list_for_each_entry::<dlm_lkb>(head) { error = dlm_send_rcom_lock(r, lkb, seq); if error != 0 { break; } (*r).res_recover_locks_count += 1; } error }
unsafe fn recover_locks(r: *mut dlm_rsb, seq: u64) -> i32 { lock_rsb(r); DLM_ASSERT((*r).res_recover_locks_count == 0, dlm_dump_rsb(r)); let mut e = recover_locks_queue(r, &mut (*r).res_grantqueue, seq); if e == 0 { e = recover_locks_queue(r, &mut (*r).res_convertqueue, seq); } if e == 0 { e = recover_locks_queue(r, &mut (*r).res_waitqueue, seq); } if e == 0 && (*r).res_recover_locks_count != 0 { recover_list_add(r); } else if e == 0 { rsb_clear_flag(r, RSB_NEW_MASTER); } unlock_rsb(r); e }
unsafe fn dlm_recover_locks(ls: *mut dlm_ls, seq: u64, root_list: *const list_head) -> i32 { let mut count = 0; for r in list_for_each_entry::<dlm_rsb>(root_list) { if (*r).res_nodeid != -1 && is_master(r) != 0 { rsb_clear_flag(r, RSB_NEW_MASTER); continue; } if rsb_flag(r, RSB_NEW_MASTER) == 0 { continue; } if dlm_recovery_stopped(ls) != 0 { recover_list_clear(ls); return -EINTR; } let e = recover_locks(r, seq); if e != 0 { recover_list_clear(ls); return e; } count += (*r).res_recover_locks_count; } log_rinfo(ls, "dlm_recover_locks %d out", count); let e = dlm_wait_function(ls, recover_list_empty); if e != 0 { recover_list_clear(ls); } e }
unsafe fn dlm_recovered_lock(r: *mut dlm_rsb) { DLM_ASSERT(rsb_flag(r, RSB_NEW_MASTER) != 0, dlm_dump_rsb(r)); (*r).res_recover_locks_count -= 1; if (*r).res_recover_locks_count == 0 { rsb_clear_flag(r, RSB_NEW_MASTER); recover_list_del(r); } if recover_list_empty((*r).res_ls) != 0 { wake_up(&mut (*(*r).res_ls).ls_wait_general); } }

unsafe fn recover_lvb(r: *mut dlm_rsb) {
    let mut big_lkb = core::ptr::null_mut(); let mut high_lkb = core::ptr::null_mut(); let mut high_seq = 0u32;
    let mut exists = 0; let lvblen = (*(*r).res_ls).ls_lvblen;
    if rsb_flag(r, RSB_NEW_MASTER2) == 0 && rsb_flag(r, RSB_RECOVER_LVB_INVAL) != 0 { rsb_set_flag(r, RSB_VALNOTVALID); return; }
    if rsb_flag(r, RSB_NEW_MASTER2) == 0 { return; }
    for queue in [&mut (*r).res_grantqueue, &mut (*r).res_convertqueue] {
        for iter in list_for_each_entry::<dlm_lkb>(queue) {
            if iter.lkb_exflags & DLM_LKF_VALBLK == 0 { continue; }
            exists = 1;
            if iter.lkb_grmode > DLM_LOCK_CR { big_lkb = iter; break; }
            if (iter.lkb_lvbseq as i32).wrapping_sub(high_seq as i32) >= 0 { high_lkb = iter; high_seq = iter.lkb_lvbseq; }
        }
        if !big_lkb.is_null() { break; }
    }
    if exists == 0 { return; }
    if big_lkb.is_null() { rsb_set_flag(r, RSB_VALNOTVALID); }
    if (*r).res_lvbptr.is_null() { (*r).res_lvbptr = dlm_allocate_lvb((*r).res_ls); if (*r).res_lvbptr.is_null() { return; } }
    if !big_lkb.is_null() { (*r).res_lvbseq = (*big_lkb).lkb_lvbseq; memcpy((*r).res_lvbptr, (*big_lkb).lkb_lvbptr, lvblen); }
    else if !high_lkb.is_null() { (*r).res_lvbseq = (*high_lkb).lkb_lvbseq; memcpy((*r).res_lvbptr, (*high_lkb).lkb_lvbptr, lvblen); }
    else { (*r).res_lvbseq = 0; memset((*r).res_lvbptr, 0, lvblen); }
}

unsafe fn recover_conversion(r: *mut dlm_rsb) {
    let ls = (*r).res_ls; let mut other_lkid = 0; let mut other_grmode = -1;
    for lkb in list_for_each_entry::<dlm_lkb>(&(*r).res_grantqueue) { if lkb.lkb_grmode == DLM_LOCK_PR || lkb.lkb_grmode == DLM_LOCK_CW { other_grmode = lkb.lkb_grmode; other_lkid = lkb.lkb_id; break; } }
    if other_grmode == -1 { return; }
    for lkb in list_for_each_entry::<dlm_lkb>(&(*r).res_convertqueue) { if (lkb.lkb_grmode == DLM_LOCK_PR && other_grmode == DLM_LOCK_CW) || (lkb.lkb_grmode == DLM_LOCK_CW && other_grmode == DLM_LOCK_PR) { log_rinfo(ls, "%s %x gr %d rq %d, remote %d %x, other_lkid %u, other gr %d, set gr=NL", __func__, lkb.lkb_id, lkb.lkb_grmode, lkb.lkb_rqmode, lkb.lkb_nodeid, lkb.lkb_remid, other_lkid, other_grmode); lkb.lkb_grmode = DLM_LOCK_NL; } }
}
unsafe fn recover_grant(r: *mut dlm_rsb) { if !list_empty(&(*r).res_waitqueue) || !list_empty(&(*r).res_convertqueue) { rsb_set_flag(r, RSB_RECOVER_GRANT); } }

unsafe fn dlm_recover_rsbs(ls: *mut dlm_ls, root_list: *const list_head) { let mut count = 0; for r in list_for_each_entry::<dlm_rsb>(root_list) { lock_rsb(r); if (*r).res_nodeid != -1 && is_master(r) != 0 { if rsb_flag(r, RSB_RECOVER_CONVERT) != 0 { recover_conversion(r); } recover_lvb(r); if rsb_flag(r, RSB_NEW_MASTER2) != 0 { recover_grant(r); } count += 1; } else { rsb_clear_flag(r, RSB_VALNOTVALID); } rsb_clear_flag(r, RSB_RECOVER_CONVERT); rsb_clear_flag(r, RSB_RECOVER_LVB_INVAL); rsb_clear_flag(r, RSB_NEW_MASTER2); unlock_rsb(r); } if count != 0 { log_rinfo(ls, "dlm_recover_rsbs %d done", count); } }
unsafe fn dlm_clear_inactive(ls: *mut dlm_ls) { let mut count = 0; write_lock_bh(&mut (*ls).ls_rsbtbl_lock); for r in list_for_each_entry_safe::<dlm_rsb>(&(*ls).ls_slow_inactive) { list_del(&mut r.res_slow_list); rhashtable_remove_fast(&mut (*ls).ls_rsbtbl, &mut r.res_node, dlm_rhash_rsb_params); if !list_empty(&r.res_scan_list) { list_del_init(&mut r.res_scan_list); } free_inactive_rsb(r); count += 1; } write_unlock_bh(&mut (*ls).ls_rsbtbl_lock); if count != 0 { log_rinfo(ls, "dlm_clear_inactive %u done", count); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
