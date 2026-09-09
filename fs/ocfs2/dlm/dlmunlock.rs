// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dlmunlock.c
 *
 * underlying calls for unlocking locks
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

// Linux/OCFS2 declarations and constants are supplied by the surrounding
// translation unit.

const DLM_UNLOCK_FREE_LOCK: i32 = 0x00000001;
const DLM_UNLOCK_CALL_AST: i32 = 0x00000002;
const DLM_UNLOCK_REMOVE_LOCK: i32 = 0x00000004;
const DLM_UNLOCK_REGRANT_LOCK: i32 = 0x00000008;
const DLM_UNLOCK_CLEAR_CONVERT_TYPE: i32 = 0x00000010;

unsafe fn dlmunlock_common(
    dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource, lock: *mut dlm_lock,
    lksb: *mut dlm_lockstatus, mut flags: i32, call_ast: *mut i32,
    master_node: i32,
) -> dlm_status {
    let mut actions: i32 = 0;
    let mut in_use: i32;
    let mut owner: u8 = 0;
    let mut recovery_wait: i32 = 0;

    mlog(0, "master_node = %d, valblk = %d\n", master_node, flags & LKM_VALBLK);
    if master_node != 0 { BUG_ON((*res).owner != (*dlm).node_num); }
    else { BUG_ON((*res).owner == (*dlm).node_num); }

    spin_lock(&mut (*dlm).ast_lock);
    in_use = (!list_empty(&(*lock).ast_list)) as i32;
    spin_unlock(&mut (*dlm).ast_lock);
    if in_use != 0 && (flags & LKM_CANCEL) == 0 {
        mlog(ML_ERROR, "lockres %.*s: Someone is calling dlmunlock while waiting for an ast!\n",
             (*res).lockname.len, (*res).lockname.name);
        return DLM_BADPARAM;
    }
    spin_lock(&mut (*res).spinlock);
    if (*res).state & DLM_LOCK_RES_IN_PROGRESS != 0 {
        if master_node != 0 && (flags & LKM_CANCEL) == 0 {
            mlog(ML_ERROR, "lockres in progress!\n"); spin_unlock(&mut (*res).spinlock); return DLM_FORWARD;
        }
        __dlm_wait_on_lockres(res); (*res).state |= DLM_LOCK_RES_IN_PROGRESS;
    }
    spin_lock(&mut (*lock).spinlock);
    let status: dlm_status;
    if (*res).state & DLM_LOCK_RES_RECOVERING != 0 { status = DLM_RECOVERING; goto_leave!(res, lock); }
    if (*res).state & DLM_LOCK_RES_MIGRATING != 0 { status = DLM_MIGRATING; goto_leave!(res, lock); }
    status = if flags & LKM_CANCEL != 0 {
        dlm_get_cancel_actions(dlm, res, lock, lksb, &mut actions)
    } else { dlm_get_unlock_actions(dlm, res, lock, lksb, &mut actions) };
    if status != DLM_NORMAL && (status != DLM_CANCELGRANT || master_node == 0) { goto_leave!(res, lock); }
    if flags & LKM_VALBLK != 0 {
        if master_node != 0 { memcpy((*res).lvb, (*lksb).lvb, DLM_LVB_LEN); }
        else { flags |= LKM_PUT_LVB; }
    }
    if master_node == 0 {
        owner = (*res).owner;
        if flags & LKM_CANCEL != 0 { (*lock).cancel_pending = 1; } else { (*lock).unlock_pending = 1; }
        spin_unlock(&mut (*lock).spinlock); spin_unlock(&mut (*res).spinlock);
        let remote_status = dlm_send_remote_unlock_request(dlm, res, lock, lksb, flags, owner);
        spin_lock(&mut (*res).spinlock); spin_lock(&mut (*lock).spinlock);
        // The C implementation adjusts actions for CANCELGRANT and retry statuses here.
        if remote_status == DLM_CANCELGRANT { actions &= !(DLM_UNLOCK_REMOVE_LOCK | DLM_UNLOCK_REGRANT_LOCK | DLM_UNLOCK_CLEAR_CONVERT_TYPE); }
        else if remote_status == DLM_RECOVERING || remote_status == DLM_MIGRATING || remote_status == DLM_FORWARD || remote_status == DLM_NOLOCKMGR { actions = 0; }
        if flags & LKM_CANCEL != 0 { (*lock).cancel_pending = 0; }
        else if (*lock).unlock_pending == 0 { recovery_wait = 1; } else { (*lock).unlock_pending = 0; }
    }
    dlm_lock_get(lock);
    if actions & DLM_UNLOCK_REMOVE_LOCK != 0 { list_del_init(&mut (*lock).list); dlm_lock_put(lock); }
    if actions & DLM_UNLOCK_REGRANT_LOCK != 0 { dlm_lock_get(lock); list_add_tail(&mut (*lock).list, &mut (*res).granted); }
    if actions & DLM_UNLOCK_CLEAR_CONVERT_TYPE != 0 { (*lock).ml.convert_type = LKM_IVMODE; }
    dlm_lock_put(lock);
    goto_leave!(res, lock);
}

// The following helpers retain the original call structure and are supplied
// by the OCFS2 lock manager interface.
unsafe fn dlm_send_remote_unlock_request(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource, lock: *mut dlm_lock, lksb: *mut dlm_lockstatus, flags: i32, owner: u8) -> dlm_status {
    if owner == (*dlm).node_num { return DLM_FORWARD; }
    let mut unlock: dlm_unlock_lock = core::mem::zeroed();
    unlock.node_idx = (*dlm).node_num; unlock.flags = cpu_to_be32(flags); unlock.cookie = (*lock).ml.cookie;
    unlock.namelen = (*res).lockname.len; memcpy(unlock.name, (*res).lockname.name, unlock.namelen);
    let mut vec: [kvec; 2] = core::mem::zeroed(); let mut veclen: usize = 1;
    vec[0].iov_len = core::mem::size_of::<dlm_unlock_lock>(); vec[0].iov_base = &mut unlock as *mut _ as *mut _;
    if flags & LKM_PUT_LVB != 0 { vec[1].iov_len = DLM_LVB_LEN; vec[1].iov_base = (*lock).lksb.lvb.as_mut_ptr() as *mut _; veclen += 1; }
    let mut status = 0; let tmpret = o2net_send_message_vec(DLM_UNLOCK_LOCK_MSG, (*dlm).key, vec.as_mut_ptr(), veclen, owner, &mut status);
    if tmpret >= 0 { status } else if dlm_is_host_down(tmpret) { if dlm_is_node_dead(dlm, owner) { DLM_NORMAL } else { DLM_NOLOCKMGR } } else { dlm_err_to_dlm_status(tmpret) }
}

unsafe fn dlmunlock_master(d: *mut dlm_ctxt, r: *mut dlm_lock_resource, l: *mut dlm_lock, s: *mut dlm_lockstatus, f: i32, a: *mut i32) -> dlm_status { dlmunlock_common(d,r,l,s,f,a,1) }
unsafe fn dlmunlock_remote(d: *mut dlm_ctxt, r: *mut dlm_lock_resource, l: *mut dlm_lock, s: *mut dlm_lockstatus, f: i32, a: *mut i32) -> dlm_status { dlmunlock_common(d,r,l,s,f,a,0) }

unsafe fn dlm_get_cancel_actions(_: *mut dlm_ctxt, res: *mut dlm_lock_resource, lock: *mut dlm_lock, _: *mut dlm_lockstatus, actions: *mut i32) -> dlm_status {
    if dlm_lock_on_list(&(*res).blocked, lock) { *actions = DLM_UNLOCK_CALL_AST | DLM_UNLOCK_REMOVE_LOCK; DLM_NORMAL }
    else if dlm_lock_on_list(&(*res).converting, lock) { *actions = DLM_UNLOCK_CALL_AST | DLM_UNLOCK_REMOVE_LOCK | DLM_UNLOCK_REGRANT_LOCK | DLM_UNLOCK_CLEAR_CONVERT_TYPE; DLM_NORMAL }
    else if dlm_lock_on_list(&(*res).granted, lock) { *actions = DLM_UNLOCK_CALL_AST; DLM_CANCELGRANT }
    else { *actions = 0; DLM_IVLOCKID }
}

unsafe fn dlm_get_unlock_actions(_: *mut dlm_ctxt, res: *mut dlm_lock_resource, lock: *mut dlm_lock, _: *mut dlm_lockstatus, actions: *mut i32) -> dlm_status {
    if !dlm_lock_on_list(&(*res).granted, lock) { *actions = 0; DLM_DENIED }
    else { *actions = DLM_UNLOCK_FREE_LOCK | DLM_UNLOCK_CALL_AST | DLM_UNLOCK_REMOVE_LOCK; DLM_NORMAL }
}

pub unsafe fn dlmunlock(dlm: *mut dlm_ctxt, lksb: *mut dlm_lockstatus, mut flags: i32, unlockast: dlm_astunlockfunc_t, data: *mut core::ffi::c_void) -> dlm_status {
    if lksb.is_null() { return DLM_BADARGS; }
    if flags & !(LKM_CANCEL | LKM_VALBLK | LKM_INVVALBLK) != 0 { return DLM_BADPARAM; }
    if flags & (LKM_VALBLK | LKM_CANCEL) == (LKM_VALBLK | LKM_CANCEL) { flags &= !LKM_VALBLK; }
    if (*lksb).lockid.is_null() || (*(*lksb).lockid).lockres.is_null() { return DLM_BADPARAM; }
    let lock = (*lksb).lockid; let res = (*lock).lockres; dlm_lock_get(lock); dlm_lockres_get(res);
    loop {
        let is_master = ((*res).owner == (*dlm).node_num) as i32;
        if flags & LKM_VALBLK != 0 && (*lock).ml.type != LKM_EXMODE { flags &= !LKM_VALBLK; }
        let mut call_ast = 0; let status = if is_master != 0 { dlmunlock_master(dlm,res,lock,lksb,flags,&mut call_ast) } else { dlmunlock_remote(dlm,res,lock,lksb,flags,&mut call_ast) };
        if status == DLM_RECOVERING || status == DLM_MIGRATING || status == DLM_FORWARD || status == DLM_NOLOCKMGR { msleep(50); continue; }
        if call_ast != 0 { unlockast(data, status); }
        let final_status = if status == DLM_CANCELGRANT { DLM_NORMAL } else { status };
        dlm_lockres_calc_usage(dlm,res); dlm_lockres_put(res); dlm_lock_put(lock); return final_status;
    }
}

pub unsafe fn dlm_commit_pending_unlock(_: *mut dlm_lock_resource, lock: *mut dlm_lock) { list_del_init(&mut (*lock).list); }
pub unsafe fn dlm_commit_pending_cancel(res: *mut dlm_lock_resource, lock: *mut dlm_lock) { list_move_tail(&mut (*lock).list, &mut (*res).granted); (*lock).ml.convert_type = LKM_IVMODE; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
