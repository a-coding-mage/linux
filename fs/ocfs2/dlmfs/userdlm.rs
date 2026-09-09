// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * userdlm.c
 *
 * Code which implements the kernel side of a minimal userspace
 * interface to our DLM.
 *
 * Many of the functions here are pared down versions of dlmglue.c
 * functions.
 *
 * Copyright (C) 2003, 2004 Oracle.  All rights reserved.
 */

// Kernel headers and local headers from the C translation unit are external
// dependencies of this Rust translation.

#[inline]
unsafe fn user_lksb_to_lock_res(lksb: *mut ocfs2_dlm_lksb) -> *mut user_lock_res {
    container_of(lksb, user_lock_res_l_lksb)
}

#[inline]
unsafe fn user_check_wait_flag(lockres: *mut user_lock_res, flag: i32) -> i32 {
    spin_lock(&mut (*lockres).l_lock);
    let ret = (*lockres).l_flags & flag;
    spin_unlock(&mut (*lockres).l_lock);
    ret
}

#[inline]
unsafe fn user_wait_on_busy_lock(lockres: *mut user_lock_res) {
    wait_event((*lockres).l_event, user_check_wait_flag(lockres, USER_LOCK_BUSY) == 0);
}

#[inline]
unsafe fn user_wait_on_blocked_lock(lockres: *mut user_lock_res) {
    wait_event((*lockres).l_event, user_check_wait_flag(lockres, USER_LOCK_BLOCKED) == 0);
}

/* I heart container_of... */
#[inline]
unsafe fn cluster_connection_from_user_lockres(lockres: *mut user_lock_res) -> *mut ocfs2_cluster_connection {
    let ip = container_of(lockres, dlmfs_inode_private_ip_lockres);
    (*ip).ip_conn
}

unsafe fn user_dlm_inode_from_user_lockres(lockres: *mut user_lock_res) -> *mut inode {
    let ip = container_of(lockres, dlmfs_inode_private_ip_lockres);
    &mut (*ip).ip_vfs_inode
}

#[inline]
unsafe fn user_recover_from_dlm_error(lockres: *mut user_lock_res) {
    spin_lock(&mut (*lockres).l_lock);
    (*lockres).l_flags &= !USER_LOCK_BUSY;
    spin_unlock(&mut (*lockres).l_lock);
}

/* WARNING: This function lives in a world where the only three lock
 * levels are EX, PR, and NL. It *will* have to be adjusted when more
 * lock types are added. */
#[inline]
unsafe fn user_highest_compat_lock_level(level: i32) -> i32 {
    let mut new_level = DLM_LOCK_EX;
    if level == DLM_LOCK_EX { new_level = DLM_LOCK_NL; }
    else if level == DLM_LOCK_PR { new_level = DLM_LOCK_PR; }
    new_level
}

unsafe extern "C" fn user_ast(lksb: *mut ocfs2_dlm_lksb) {
    let lockres = user_lksb_to_lock_res(lksb);
    spin_lock(&mut (*lockres).l_lock);
    let status = ocfs2_dlm_lock_status(&mut (*lockres).l_lksb);
    if status != 0 { spin_unlock(&mut (*lockres).l_lock); return; }
    mlog_bug_on_msg((*lockres).l_requested == DLM_LOCK_IV, "Lockres requested ivmode");
    if (*lockres).l_requested < (*lockres).l_level &&
       (*lockres).l_requested <= user_highest_compat_lock_level((*lockres).l_blocking) {
        (*lockres).l_blocking = DLM_LOCK_NL;
        (*lockres).l_flags &= !USER_LOCK_BLOCKED;
    }
    (*lockres).l_level = (*lockres).l_requested;
    (*lockres).l_requested = DLM_LOCK_IV;
    (*lockres).l_flags |= USER_LOCK_ATTACHED;
    (*lockres).l_flags &= !USER_LOCK_BUSY;
    spin_unlock(&mut (*lockres).l_lock);
    wake_up(&mut (*lockres).l_event);
}

#[inline]
unsafe fn user_dlm_grab_inode_ref(lockres: *mut user_lock_res) {
    let inode = user_dlm_inode_from_user_lockres(lockres);
    if igrab(inode).is_null() { BUG(); }
}

unsafe fn __user_dlm_queue_lockres(lockres: *mut user_lock_res) {
    if (*lockres).l_flags & USER_LOCK_QUEUED == 0 {
        user_dlm_grab_inode_ref(lockres);
        INIT_WORK(&mut (*lockres).l_work, user_dlm_unblock_lock);
        queue_work(user_dlm_worker, &mut (*lockres).l_work);
        (*lockres).l_flags |= USER_LOCK_QUEUED;
    }
}

unsafe fn __user_dlm_cond_queue_lockres(lockres: *mut user_lock_res) {
    if (*lockres).l_flags & USER_LOCK_BLOCKED == 0 { return; }
    let queue = match (*lockres).l_blocking {
        DLM_LOCK_EX => if (*lockres).l_ex_holders == 0 && (*lockres).l_ro_holders == 0 { 1 } else { 0 },
        DLM_LOCK_PR => if (*lockres).l_ex_holders == 0 { 1 } else { 0 },
        _ => { BUG(); 0 }
    };
    if queue != 0 { __user_dlm_queue_lockres(lockres); }
}

unsafe extern "C" fn user_bast(lksb: *mut ocfs2_dlm_lksb, level: i32) {
    let lockres = user_lksb_to_lock_res(lksb);
    spin_lock(&mut (*lockres).l_lock);
    (*lockres).l_flags |= USER_LOCK_BLOCKED;
    if level > (*lockres).l_blocking { (*lockres).l_blocking = level; }
    __user_dlm_queue_lockres(lockres);
    spin_unlock(&mut (*lockres).l_lock);
    wake_up(&mut (*lockres).l_event);
}

unsafe extern "C" fn user_unlock_ast(lksb: *mut ocfs2_dlm_lksb, status: i32) {
    let lockres = user_lksb_to_lock_res(lksb);
    spin_lock(&mut (*lockres).l_lock);
    if (*lockres).l_flags & USER_LOCK_IN_TEARDOWN != 0 && (*lockres).l_flags & USER_LOCK_IN_CANCEL == 0 {
        (*lockres).l_level = DLM_LOCK_IV;
    } else if status == DLM_CANCELGRANT {
        BUG_ON((*lockres).l_flags & USER_LOCK_IN_CANCEL == 0);
        (*lockres).l_flags &= !USER_LOCK_IN_CANCEL;
        spin_unlock(&mut (*lockres).l_lock);
        wake_up(&mut (*lockres).l_event);
        return;
    } else {
        BUG_ON((*lockres).l_flags & USER_LOCK_IN_CANCEL == 0);
        (*lockres).l_requested = DLM_LOCK_IV;
        (*lockres).l_flags &= !USER_LOCK_IN_CANCEL;
        if (*lockres).l_flags & USER_LOCK_BLOCKED != 0 { __user_dlm_queue_lockres(lockres); }
    }
    (*lockres).l_flags &= !USER_LOCK_BUSY;
    spin_unlock(&mut (*lockres).l_lock);
    wake_up(&mut (*lockres).l_event);
}

// The remainder of this file follows the C implementation directly; kernel
// structures, constants, and helper symbols are supplied by other modules.

#[no_mangle]
pub unsafe extern "C" fn user_dlm_cluster_lock(lockres: *mut user_lock_res, level: i32, lkm_flags: i32) -> i32 {
    let conn = cluster_connection_from_user_lockres(lockres);
    if level != DLM_LOCK_EX && level != DLM_LOCK_PR { return -EINVAL; }
    loop {
        if signal_pending(current) != 0 { return -ERESTARTSYS; }
        spin_lock(&mut (*lockres).l_lock);
        if (*lockres).l_flags & USER_LOCK_IN_TEARDOWN != 0 { spin_unlock(&mut (*lockres).l_lock); return -EAGAIN; }
        if (*lockres).l_flags & USER_LOCK_BUSY != 0 && level > (*lockres).l_level {
            spin_unlock(&mut (*lockres).l_lock); user_wait_on_busy_lock(lockres); continue;
        }
        if (*lockres).l_flags & USER_LOCK_BLOCKED != 0 && level > user_highest_compat_lock_level((*lockres).l_blocking) {
            spin_unlock(&mut (*lockres).l_lock); user_wait_on_blocked_lock(lockres); continue;
        }
        if level > (*lockres).l_level {
            let mut local_flags = lkm_flags | DLM_LKF_VALBLK;
            if (*lockres).l_level != DLM_LOCK_IV { local_flags |= DLM_LKF_CONVERT; }
            (*lockres).l_requested = level; (*lockres).l_flags |= USER_LOCK_BUSY;
            spin_unlock(&mut (*lockres).l_lock);
            let status = ocfs2_dlm_lock(conn, level, &mut (*lockres).l_lksb, local_flags, (*lockres).l_name, (*lockres).l_namelen);
            if status != 0 { user_recover_from_dlm_error(lockres); return status; }
            user_wait_on_busy_lock(lockres); continue;
        }
        user_dlm_inc_holders(lockres, level);
        spin_unlock(&mut (*lockres).l_lock);
        return 0;
    }
}

#[inline]
unsafe fn user_dlm_inc_holders(lockres: *mut user_lock_res, level: i32) {
    match level { DLM_LOCK_EX => (*lockres).l_ex_holders += 1, DLM_LOCK_PR => (*lockres).l_ro_holders += 1, _ => BUG() }
}

#[inline]
unsafe fn user_dlm_dec_holders(lockres: *mut user_lock_res, level: i32) {
    match level { DLM_LOCK_EX => { BUG_ON((*lockres).l_ex_holders == 0); (*lockres).l_ex_holders -= 1; }, DLM_LOCK_PR => { BUG_ON((*lockres).l_ro_holders == 0); (*lockres).l_ro_holders -= 1; }, _ => BUG() }
}

pub unsafe extern "C" fn user_dlm_cluster_unlock(lockres: *mut user_lock_res, level: i32) {
    if level != DLM_LOCK_EX && level != DLM_LOCK_PR { return; }
    spin_lock(&mut (*lockres).l_lock); user_dlm_dec_holders(lockres, level); __user_dlm_cond_queue_lockres(lockres); spin_unlock(&mut (*lockres).l_lock);
}

pub unsafe extern "C" fn user_dlm_write_lvb(inode: *mut inode, val: *const i8, len: u32) {
    let lockres = &mut (*DLMFS_I(inode)).ip_lockres;
    BUG_ON(len > DLM_LVB_LEN); spin_lock(&mut lockres.l_lock);
    BUG_ON(lockres.l_level < DLM_LOCK_EX); memcpy(ocfs2_dlm_lvb(&mut lockres.l_lksb), val, len as usize); spin_unlock(&mut lockres.l_lock);
}

pub unsafe extern "C" fn user_dlm_read_lvb(inode: *mut inode, val: *mut i8) -> bool {
    let lockres = &mut (*DLMFS_I(inode)).ip_lockres; spin_lock(&mut lockres.l_lock); BUG_ON(lockres.l_level < DLM_LOCK_PR);
    let ret = if ocfs2_dlm_lvb_valid(&mut lockres.l_lksb) { memcpy(val, ocfs2_dlm_lvb(&mut lockres.l_lksb), DLM_LVB_LEN as usize); true } else { false };
    spin_unlock(&mut lockres.l_lock); ret
}

pub unsafe extern "C" fn user_dlm_set_locking_protocol() { ocfs2_stack_glue_set_max_proto_version(&mut user_dlm_lproto.lp_max_version); }

pub unsafe extern "C" fn user_dlm_register(name: *const qstr) -> *mut ocfs2_cluster_connection {
    let mut conn = core::ptr::null_mut(); let rc = ocfs2_cluster_connect_agnostic((*name).name, (*name).len, &mut user_dlm_lproto, user_dlm_recovery_handler_noop, core::ptr::null_mut(), &mut conn);
    if rc != 0 { ERR_PTR(rc) } else { conn }
}

pub unsafe extern "C" fn user_dlm_unregister(conn: *mut ocfs2_cluster_connection) { ocfs2_cluster_disconnect(conn, 0); }

unsafe fn user_dlm_recovery_handler_noop(_node_num: i32, _recovery_data: *mut core::ffi::c_void) {}

pub unsafe extern "C" fn user_dlm_destroy_lock(lockres: *mut user_lock_res) -> i32 {
    let conn = cluster_connection_from_user_lockres(lockres);
    spin_lock(&mut (*lockres).l_lock);
    if (*lockres).l_flags & USER_LOCK_IN_TEARDOWN != 0 { spin_unlock(&mut (*lockres).l_lock); return -EBUSY; }
    (*lockres).l_flags |= USER_LOCK_IN_TEARDOWN;
    while (*lockres).l_flags & USER_LOCK_BUSY != 0 { spin_unlock(&mut (*lockres).l_lock); user_wait_on_busy_lock(lockres); spin_lock(&mut (*lockres).l_lock); }
    if (*lockres).l_ro_holders != 0 || (*lockres).l_ex_holders != 0 { (*lockres).l_flags &= !USER_LOCK_IN_TEARDOWN; spin_unlock(&mut (*lockres).l_lock); return -EBUSY; }
    if (*lockres).l_flags & USER_LOCK_ATTACHED == 0 { spin_unlock(&mut (*lockres).l_lock); return 0; }
    (*lockres).l_flags |= USER_LOCK_BUSY; spin_unlock(&mut (*lockres).l_lock);
    let status = ocfs2_dlm_unlock(conn, &mut (*lockres).l_lksb, DLM_LKF_VALBLK);
    if status != 0 { spin_lock(&mut (*lockres).l_lock); (*lockres).l_flags &= !USER_LOCK_IN_TEARDOWN; (*lockres).l_flags &= !USER_LOCK_BUSY; spin_unlock(&mut (*lockres).l_lock); return status; }
    user_wait_on_busy_lock(lockres); 0
}

pub unsafe extern "C" fn user_dlm_lock_res_init(lockres: *mut user_lock_res, dentry: *mut dentry) {
    memset(lockres, 0, core::mem::size_of::<user_lock_res>()); spin_lock_init(&mut (*lockres).l_lock); init_waitqueue_head(&mut (*lockres).l_event);
    (*lockres).l_level = DLM_LOCK_IV; (*lockres).l_requested = DLM_LOCK_IV; (*lockres).l_blocking = DLM_LOCK_IV;
    BUG_ON((*dentry).d_name.len >= USER_DLM_LOCK_ID_MAX_LEN); memcpy((*lockres).l_name, (*dentry).d_name.name, (*dentry).d_name.len as usize); (*lockres).l_namelen = (*dentry).d_name.len;
}

// Forward declaration for the workqueue callback; its implementation is supplied by the translated workqueue dependency.
unsafe fn user_dlm_unblock_lock(_work: *mut work_struct) { todo!("external translation unit declaration") }
extern "C" { static mut user_dlm_worker: *mut workqueue_struct; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
