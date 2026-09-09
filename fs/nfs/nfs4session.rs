// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/nfs/nfs4session.c
 *
 * Copyright (c) 2012 Trond Myklebust <Trond.Myklebust@netapp.com>
 */
// Kernel and NFS dependencies are supplied by the surrounding translation unit.

const NFSDBG_FACILITY: u32 = NFSDBG_STATE;

unsafe fn nfs4_init_slot_table(tbl: *mut nfs4_slot_table, queue: *const i8) {
    (*tbl).highest_used_slotid = NFS4_NO_SLOT;
    spin_lock_init(&mut (*tbl).slot_tbl_lock);
    rpc_init_priority_wait_queue(&mut (*tbl).slot_tbl_waitq, queue);
    init_waitqueue_head(&mut (*tbl).slot_waitq);
    init_completion(&mut (*tbl).complete);
}

unsafe fn nfs4_shrink_slot_table(tbl: *mut nfs4_slot_table, mut newsize: u32) {
    if newsize >= (*tbl).max_slots { return; }
    let mut p = &mut (*tbl).slots as *mut *mut nfs4_slot;
    while newsize != 0 { p = &mut (**p).next; newsize -= 1; }
    while !(*p).is_null() {
        let slot = *p;
        *p = (*slot).next;
        kfree(slot as *mut core::ffi::c_void);
        (*tbl).max_slots -= 1;
    }
}

pub unsafe fn nfs4_slot_tbl_drain_complete(tbl: *mut nfs4_slot_table) {
    if nfs4_slot_tbl_draining(tbl) { complete(&mut (*tbl).complete); }
}

pub unsafe fn nfs4_free_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) {
    let slotid = (*slot).slot_nr;
    __clear_bit(slotid, (*tbl).used_slots.as_mut_ptr());
    if slotid == (*tbl).highest_used_slotid {
        let new_max = find_last_bit((*tbl).used_slots.as_ptr(), slotid);
        if new_max < slotid { (*tbl).highest_used_slotid = new_max; }
        else { (*tbl).highest_used_slotid = NFS4_NO_SLOT; nfs4_slot_tbl_drain_complete(tbl); }
    }
    dprintk!("%s: slotid %u highest_used_slotid %u\n", "nfs4_free_slot", slotid, (*tbl).highest_used_slotid);
}

unsafe fn nfs4_new_slot(tbl: *mut nfs4_slot_table, slotid: u32, seq_init: u32, gfp_mask: gfp_t) -> *mut nfs4_slot {
    let slot = kzalloc_obj::<nfs4_slot>(gfp_mask);
    if !slot.is_null() {
        (*slot).table = tbl; (*slot).slot_nr = slotid; (*slot).seq_nr = seq_init;
        (*slot).seq_nr_highest_sent = seq_init; (*slot).seq_nr_last_acked = seq_init.wrapping_sub(1);
    }
    slot
}

unsafe fn nfs4_find_or_create_slot(tbl: *mut nfs4_slot_table, slotid: u32, seq_init: u32, gfp_mask: gfp_t) -> *mut nfs4_slot {
    let mut p = &mut (*tbl).slots as *mut *mut nfs4_slot;
    loop {
        if (*p).is_null() {
            *p = nfs4_new_slot(tbl, (*tbl).max_slots, seq_init, gfp_mask);
            if (*p).is_null() { break; }
            (*tbl).max_slots += 1;
        }
        let slot = *p;
        if (*slot).slot_nr == slotid { return slot; }
        p = &mut (*slot).next;
    }
    ERR_PTR(-ENOMEM)
}

unsafe fn nfs4_lock_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) {
    let slotid = (*slot).slot_nr;
    __set_bit(slotid, (*tbl).used_slots.as_mut_ptr());
    if slotid > (*tbl).highest_used_slotid || (*tbl).highest_used_slotid == NFS4_NO_SLOT { (*tbl).highest_used_slotid = slotid; }
    (*slot).generation = (*tbl).generation;
}

pub unsafe fn nfs4_try_to_lock_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) -> bool {
    if nfs4_test_locked_slot(tbl, (*slot).slot_nr) { return false; }
    nfs4_lock_slot(tbl, slot); true
}

pub unsafe fn nfs4_lookup_slot(tbl: *mut nfs4_slot_table, slotid: u32) -> *mut nfs4_slot {
    if slotid <= (*tbl).max_slotid { nfs4_find_or_create_slot(tbl, slotid, 0, GFP_NOWAIT) } else { ERR_PTR(-E2BIG) }
}

unsafe fn nfs4_slot_get_seqid(tbl: *mut nfs4_slot_table, slotid: u32, seq_nr: *mut u32) -> i32 {
    let slot = nfs4_lookup_slot(tbl, slotid); let ret = PTR_ERR_OR_ZERO(slot);
    if ret == 0 { *seq_nr = (*slot).seq_nr; } ret
}

unsafe fn nfs4_slot_seqid_in_use(tbl: *mut nfs4_slot_table, slotid: u32, seq_nr: u32) -> bool {
    let mut cur_seq = 0; let mut ret = false;
    spin_lock(&mut (*tbl).slot_tbl_lock);
    if nfs4_slot_get_seqid(tbl, slotid, &mut cur_seq) == 0 && cur_seq == seq_nr && test_bit(slotid, (*tbl).used_slots.as_ptr()) { ret = true; }
    spin_unlock(&mut (*tbl).slot_tbl_lock); ret
}

pub unsafe fn nfs4_slot_wait_on_seqid(tbl: *mut nfs4_slot_table, slotid: u32, seq_nr: u32, timeout: c_ulong) -> i32 {
    if wait_event_timeout(&mut (*tbl).slot_waitq, !nfs4_slot_seqid_in_use(tbl, slotid, seq_nr), timeout) == 0 { -ETIMEDOUT } else { 0 }
}

pub unsafe fn nfs4_alloc_slot(tbl: *mut nfs4_slot_table) -> *mut nfs4_slot {
    let mut ret = ERR_PTR(-EBUSY); let slotid = find_first_zero_bit((*tbl).used_slots.as_ptr(), (*tbl).max_slotid + 1);
    if slotid <= (*tbl).max_slotid { ret = nfs4_find_or_create_slot(tbl, slotid, 1, GFP_NOWAIT); if !IS_ERR(ret) { nfs4_lock_slot(tbl, ret); } } ret
}

unsafe fn nfs4_grow_slot_table(tbl: *mut nfs4_slot_table, max_reqs: u32, ivalue: u32) -> i32 {
    if max_reqs <= (*tbl).max_slots { return 0; }
    if !IS_ERR(nfs4_find_or_create_slot(tbl, max_reqs - 1, ivalue, GFP_NOFS)) { 0 } else { -ENOMEM }
}

unsafe fn nfs4_reset_slot_table(tbl: *mut nfs4_slot_table, server_highest_slotid: u32, ivalue: u32) {
    nfs4_shrink_slot_table(tbl, server_highest_slotid + 1); let mut p = (*tbl).slots;
    while !p.is_null() { (*p).seq_nr = ivalue; (*p).seq_nr_highest_sent = ivalue; (*p).seq_nr_last_acked = ivalue.wrapping_sub(1); p = (*p).next; }
    (*tbl).highest_used_slotid = NFS4_NO_SLOT; (*tbl).target_highest_slotid = server_highest_slotid; (*tbl).server_highest_slotid = server_highest_slotid;
    (*tbl).d_target_highest_slotid = 0; (*tbl).d2_target_highest_slotid = 0; (*tbl).max_slotid = server_highest_slotid;
}

unsafe fn nfs4_realloc_slot_table(tbl: *mut nfs4_slot_table, mut max_reqs: u32, ivalue: u32) -> i32 {
    if max_reqs > NFS4_MAX_SLOT_TABLE { max_reqs = NFS4_MAX_SLOT_TABLE; }
    let ret = nfs4_grow_slot_table(tbl, max_reqs, ivalue); if ret != 0 { return ret; }
    spin_lock(&mut (*tbl).slot_tbl_lock); nfs4_reset_slot_table(tbl, max_reqs - 1, ivalue); spin_unlock(&mut (*tbl).slot_tbl_lock); 0
}

unsafe fn nfs4_release_slot_table(tbl: *mut nfs4_slot_table) { nfs4_shrink_slot_table(tbl, 0); }
pub unsafe fn nfs4_shutdown_slot_table(tbl: *mut nfs4_slot_table) { nfs4_release_slot_table(tbl); rpc_destroy_wait_queue(&mut (*tbl).slot_tbl_waitq); }
pub unsafe fn nfs4_setup_slot_table(tbl: *mut nfs4_slot_table, max_reqs: u32, queue: *const i8) -> i32 { nfs4_init_slot_table(tbl, queue); nfs4_realloc_slot_table(tbl, max_reqs, 0) }

unsafe fn nfs41_assign_slot(task: *mut rpc_task, pslot: *mut core::ffi::c_void) -> bool {
    let args = (*task).tk_msg.rpc_argp as *mut nfs4_sequence_args; let res = (*task).tk_msg.rpc_resp as *mut nfs4_sequence_res;
    let slot = pslot as *mut nfs4_slot; let tbl = (*slot).table;
    if nfs4_slot_tbl_draining(tbl) && !(*args).sa_privileged { return false; }
    (*slot).generation = (*tbl).generation; (*args).sa_slot = slot; (*res).sr_timestamp = jiffies; (*res).sr_slot = slot; (*res).sr_status_flags = 0; (*res).sr_status = 1; true
}
unsafe fn __nfs41_wake_and_assign_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) -> bool { rpc_wake_up_first(&mut (*tbl).slot_tbl_waitq, nfs41_assign_slot, slot as *mut _) }
pub unsafe fn nfs41_wake_and_assign_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) -> bool { if (*slot).slot_nr > (*tbl).max_slotid { false } else { __nfs41_wake_and_assign_slot(tbl, slot) } }
unsafe fn nfs41_try_wake_next_slot_table_entry(tbl: *mut nfs4_slot_table) -> bool { let slot = nfs4_alloc_slot(tbl); if !IS_ERR(slot) { if __nfs41_wake_and_assign_slot(tbl, slot) { return true; } nfs4_free_slot(tbl, slot); } false }
pub unsafe fn nfs41_wake_slot_table(tbl: *mut nfs4_slot_table) { while nfs41_try_wake_next_slot_table_entry(tbl) {} }

unsafe fn nfs41_set_max_slotid_locked(tbl: *mut nfs4_slot_table, target: u32) { let mut max = core::cmp::min(NFS4_MAX_SLOT_TABLE - 1, target); if max > (*tbl).server_highest_slotid { max = (*tbl).server_highest_slotid; } if max > (*tbl).target_highest_slotid { max = (*tbl).target_highest_slotid; } (*tbl).max_slotid = max; nfs41_wake_slot_table(tbl); }
unsafe fn nfs41_set_target_slotid_locked(tbl: *mut nfs4_slot_table, target: u32) { if (*tbl).target_highest_slotid != target { (*tbl).target_highest_slotid = target; (*tbl).generation += 1; } }
pub unsafe fn nfs41_set_target_slotid(tbl: *mut nfs4_slot_table, target: u32) { spin_lock(&mut (*tbl).slot_tbl_lock); nfs41_set_target_slotid_locked(tbl, target); (*tbl).d_target_highest_slotid = 0; (*tbl).d2_target_highest_slotid = 0; nfs41_set_max_slotid_locked(tbl, target); spin_unlock(&mut (*tbl).slot_tbl_lock); }
unsafe fn nfs41_set_server_slotid_locked(tbl: *mut nfs4_slot_table, highest: u32) { if (*tbl).server_highest_slotid == highest || (*tbl).highest_used_slotid > highest { return; } nfs4_shrink_slot_table(tbl, highest + 1); (*tbl).server_highest_slotid = highest; }
unsafe fn nfs41_derivative_target_slotid(mut s1: i32, s2: i32) -> i32 { s1 -= s2; if s1 == 0 { 0 } else if s1 < 0 { (s1 - 1) >> 1 } else { (s1 + 1) >> 1 } }
unsafe fn nfs41_sign_s32(s: i32) -> i32 { if s > 0 { 1 } else if s < 0 { -1 } else { 0 } }
unsafe fn nfs41_same_sign_or_zero_s32(s1: i32, s2: i32) -> bool { s1 == 0 || s2 == 0 || nfs41_sign_s32(s1) == nfs41_sign_s32(s2) }
unsafe fn nfs41_is_outlier_target_slotid(tbl: *mut nfs4_slot_table, new_target: u32) -> bool { let d = nfs41_derivative_target_slotid(new_target as i32, (*tbl).target_highest_slotid as i32); let d2 = nfs41_derivative_target_slotid(d, (*tbl).d_target_highest_slotid as i32); let mut ret = true; if nfs41_same_sign_or_zero_s32(d, (*tbl).d_target_highest_slotid as i32) { ret = false; } if nfs41_same_sign_or_zero_s32(d2, (*tbl).d2_target_highest_slotid as i32) { ret = false; } (*tbl).d_target_highest_slotid = d as u32; (*tbl).d2_target_highest_slotid = d2 as u32; ret }

pub unsafe fn nfs41_update_target_slotid(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot, res: *mut nfs4_sequence_res) { let target = core::cmp::min((*res).sr_target_highest_slotid, NFS4_MAX_SLOTID); let highest = core::cmp::min((*res).sr_highest_slotid, NFS4_MAX_SLOTID); spin_lock(&mut (*tbl).slot_tbl_lock); if !nfs41_is_outlier_target_slotid(tbl, target) { nfs41_set_target_slotid_locked(tbl, target); } if (*tbl).generation == (*slot).generation { nfs41_set_server_slotid_locked(tbl, highest); } nfs41_set_max_slotid_locked(tbl, target); spin_unlock(&mut (*tbl).slot_tbl_lock); }

unsafe fn nfs4_release_session_slot_tables(session: *mut nfs4_session) { nfs4_release_slot_table(&mut (*session).fc_slot_table); nfs4_release_slot_table(&mut (*session).bc_slot_table); }
pub unsafe fn nfs4_setup_session_slot_tables(ses: *mut nfs4_session) -> i32 { let mut tbl = &mut (*ses).fc_slot_table; (*tbl).session = ses; let status = nfs4_realloc_slot_table(tbl, (*ses).fc_attrs.max_reqs, 1); if status != 0 || (*ses).flags & SESSION4_BACK_CHAN == 0 { return status; } tbl = &mut (*ses).bc_slot_table; (*tbl).session = ses; let status = nfs4_realloc_slot_table(tbl, (*ses).bc_attrs.max_reqs, 0); if status != 0 && (*tbl).slots.is_null() { nfs4_release_session_slot_tables(ses); } status }
pub unsafe fn nfs4_alloc_session(clp: *mut nfs_client) -> *mut nfs4_session { let session = kzalloc_obj::<nfs4_session>(GFP_NOFS); if session.is_null() { return core::ptr::null_mut(); } nfs4_init_slot_table(&mut (*session).fc_slot_table, b"ForeChannel Slot table\0".as_ptr() as *const i8); nfs4_init_slot_table(&mut (*session).bc_slot_table, b"BackChannel Slot table\0".as_ptr() as *const i8); (*session).session_state = 1 << NFS4_SESSION_INITING; (*session).clp = clp; session }
unsafe fn nfs4_destroy_session_slot_tables(session: *mut nfs4_session) { nfs4_shutdown_slot_table(&mut (*session).fc_slot_table); nfs4_shutdown_slot_table(&mut (*session).bc_slot_table); }
pub unsafe fn nfs4_destroy_session(session: *mut nfs4_session) { let cred = nfs4_get_clid_cred((*session).clp); nfs4_proc_destroy_session(session, cred); put_cred(cred); rcu_read_lock(); let xprt = rcu_dereference((*(*session).clp).cl_rpcclient).cl_xprt; rcu_read_unlock(); xprt_destroy_backchannel(xprt, NFS41_BC_MIN_CALLBACKS); nfs4_destroy_session_slot_tables(session); kfree(session as *mut _); }
unsafe fn nfs41_check_session_ready(clp: *mut nfs_client) -> i32 { if (*clp).cl_cons_state == NFS_CS_SESSION_INITING { let ret = nfs4_client_recover_expired_lease(clp); if ret != 0 { return ret; } } if (*clp).cl_cons_state < NFS_CS_READY { return -EPROTONOSUPPORT; } smp_rmb(); 0 }
pub unsafe fn nfs4_init_session(clp: *mut nfs_client) -> i32 { if !nfs4_has_session(clp) { return 0; } clear_bit(NFS4_SESSION_INITING, &mut (*(*clp).cl_session).session_state); nfs41_check_session_ready(clp) }
pub unsafe fn nfs4_init_ds_session(clp: *mut nfs_client, lease_time: c_ulong, tightly_coupled: bool) -> i32 { let session = (*clp).cl_session; spin_lock(&mut (*clp).cl_lock); if session.is_null() || test_and_clear_bit(NFS4_SESSION_INITING, &mut (*session).session_state) { (*clp).cl_lease_time = lease_time; (*clp).cl_last_renewal = jiffies; } spin_unlock(&mut (*clp).cl_lock); if session.is_null() { return 0; } let ret = nfs41_check_session_ready(clp); if ret != 0 { return ret; } if tightly_coupled && !is_ds_client(clp) { return -ENODEV; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
