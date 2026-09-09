// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dlmconvert.c
 *
 * underlying calls for lock conversion
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

// Kernel and DLM declarations are supplied by the surrounding translation unit.

/* NOTE: __dlmconvert_master is the only function in here that
 * needs a spinlock held on entry (res->spinlock) and it is the
 * only one that holds a lock on exit (res->spinlock).
 * All other functions in here need no locks and drop all of
 * the locks that they acquire. */
pub unsafe fn dlmconvert_master(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource,
                                lock: *mut dlm_lock, flags: i32, type_: i32) -> dlm_status {
    let mut call_ast = 0;
    let mut kick_thread = 0;
    let status;

    spin_lock(&mut (*res).spinlock);
    __dlm_wait_on_lockres(res);
    __dlm_lockres_reserve_ast(res);
    (*res).state |= DLM_LOCK_RES_IN_PROGRESS;
    status = __dlmconvert_master(dlm, res, lock, flags, type_, &mut call_ast, &mut kick_thread);
    (*res).state &= !DLM_LOCK_RES_IN_PROGRESS;
    spin_unlock(&mut (*res).spinlock);
    wake_up(&mut (*res).wq);
    if status != DLM_NORMAL && status != DLM_NOTQUEUED { dlm_error(status); }
    if call_ast != 0 { dlm_queue_ast(dlm, lock); } else { dlm_lockres_release_ast(dlm, res); }
    if kick_thread != 0 { dlm_kick_thread(dlm, res); }
    status
}

unsafe fn __dlmconvert_master(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource,
                              lock: *mut dlm_lock, mut flags: i32, type_: i32,
                              call_ast: *mut i32, kick_thread: *mut i32) -> dlm_status {
    let mut status = DLM_NORMAL;
    let mut tmplock: *mut dlm_lock = core::ptr::null_mut();
    assert_spin_locked(&(*res).spinlock);
    mlog(0, "type=%d, convert_type=%d, new convert_type=%d\n", (*lock).ml.type_, (*lock).ml.convert_type, type_);
    spin_lock(&mut (*lock).spinlock);
    if (*lock).ml.convert_type != LKM_IVMODE { mlog(ML_ERROR, "attempted to convert a lock with a lock conversion pending\n"); status = DLM_DENIED; goto_unlock!(status, lock, res, kick_thread); }
    if !dlm_lock_on_list(&(*res).granted, lock) { mlog(ML_ERROR, "attempted to convert a lock not on grant queue\n"); status = DLM_DENIED; goto_unlock!(status, lock, res, kick_thread); }
    if flags & LKM_VALBLK != 0 {
        match (*lock).ml.type_ {
            LKM_EXMODE => { mlog(0, "will set lvb: converting %s->%s\n", dlm_lock_mode_name((*lock).ml.type_), dlm_lock_mode_name(type_)); (*(*lock).lksb).flags |= DLM_LKSB_PUT_LVB; }
            LKM_PRMODE | LKM_NLMODE => if type_ > LKM_NLMODE { (*(*lock).lksb).flags |= DLM_LKSB_GET_LVB; } else { flags &= !LKM_VALBLK; }
            _ => {}
        }
    }
    if type_ <= (*lock).ml.type_ { goto_grant!(status, lock, res, type_, call_ast, kick_thread); }
    for_each_entry!(tmplock, &(*res).granted, list) {
        if tmplock == lock { continue; }
        if !dlm_lock_compatible((*tmplock).ml.type_, type_) { goto_switch!(status, lock, res, type_, flags, kick_thread); }
    }
    for_each_entry!(tmplock, &(*res).converting, list) {
        if !dlm_lock_compatible((*tmplock).ml.type_, type_) || !dlm_lock_compatible((*tmplock).ml.convert_type, type_) { goto_switch!(status, lock, res, type_, flags, kick_thread); }
    }
    goto_grant!(status, lock, res, type_, call_ast, kick_thread);
}

pub unsafe fn dlm_revert_pending_convert(res: *mut dlm_lock_resource, lock: *mut dlm_lock) {
    list_move_tail(&mut (*lock).list, &mut (*res).granted);
    (*lock).ml.convert_type = LKM_IVMODE;
    (*(*lock).lksb).flags &= !(DLM_LKSB_GET_LVB | DLM_LKSB_PUT_LVB);
}

pub unsafe fn dlmconvert_remote(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource,
                                lock: *mut dlm_lock, mut flags: i32, type_: i32) -> dlm_status {
    let mut status;
    spin_lock(&mut (*res).spinlock);
    if (*res).state & DLM_LOCK_RES_RECOVERING != 0 { status = DLM_RECOVERING; spin_unlock(&mut (*res).spinlock); return status; }
    __dlm_wait_on_lockres(res);
    if (*lock).ml.convert_type != LKM_IVMODE { status = DLM_DENIED; spin_unlock(&mut (*res).spinlock); return status; }
    if (*lock).ml.type_ == type_ && (*lock).ml.convert_type == LKM_IVMODE { status = DLM_NORMAL; spin_unlock(&mut (*res).spinlock); return status; }
    (*res).state |= DLM_LOCK_RES_IN_PROGRESS;
    list_move_tail(&mut (*lock).list, &mut (*res).converting);
    (*lock).convert_pending = 1;
    (*lock).ml.convert_type = type_;
    if flags & LKM_VALBLK != 0 { if (*lock).ml.type_ == LKM_EXMODE { flags |= LKM_PUT_LVB; (*(*lock).lksb).flags |= DLM_LKSB_PUT_LVB; } else if type_ == LKM_NLMODE { flags &= !LKM_VALBLK; } else { flags |= LKM_GET_LVB; (*(*lock).lksb).flags |= DLM_LKSB_GET_LVB; } }
    spin_unlock(&mut (*res).spinlock);
    status = dlm_send_remote_convert_request(dlm, res, lock, flags, type_);
    spin_lock(&mut (*res).spinlock);
    (*res).state &= !DLM_LOCK_RES_IN_PROGRESS;
    if status != DLM_NORMAL { if status != DLM_NOTQUEUED { dlm_error(status); } dlm_revert_pending_convert(res, lock); } else if (*lock).convert_pending == 0 { status = DLM_RECOVERING; }
    (*lock).convert_pending = 0;
    spin_unlock(&mut (*res).spinlock);
    wake_up(&mut (*res).wq);
    status
}

unsafe fn dlm_send_remote_convert_request(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource,
                                          lock: *mut dlm_lock, flags: i32, type_: i32) -> dlm_status {
    let mut convert: dlm_convert_lock = core::mem::zeroed();
    let mut status: i32 = 0;
    convert.node_idx = (*dlm).node_num;
    convert.requested_type = type_;
    convert.cookie = (*lock).ml.cookie;
    convert.namelen = (*res).lockname.len;
    convert.flags = cpu_to_be32(flags);
    core::ptr::copy_nonoverlapping((*res).lockname.name, convert.name.as_mut_ptr(), convert.namelen as usize);
    let mut vec = [kvec { iov_len: core::mem::size_of::<dlm_convert_lock>(), iov_base: (&mut convert as *mut _).cast() }, kvec { iov_len: 0, iov_base: core::ptr::null_mut() }];
    let mut veclen = 1usize;
    if flags & LKM_PUT_LVB != 0 { vec[1].iov_len = DLM_LVB_LEN as usize; vec[1].iov_base = (*(*lock).lksb).lvb.as_mut_ptr().cast(); veclen += 1; }
    let tmpret = o2net_send_message_vec(DLM_CONVERT_LOCK_MSG, (*dlm).key, vec.as_ptr(), veclen, (*res).owner, &mut status);
    if tmpret >= 0 { status as dlm_status } else if dlm_is_host_down(tmpret) { dlm_wait_for_node_death(dlm, (*res).owner, DLM_NODE_DEATH_WAIT_MAX); DLM_RECOVERING } else { dlm_err_to_dlm_status(tmpret) }
}

pub unsafe fn dlm_convert_lock_handler(msg: *mut o2net_msg, _len: u32, data: *mut core::ffi::c_void,
                                       _ret_data: *mut *mut core::ffi::c_void) -> i32 {
    let dlm = data.cast::<dlm_ctxt>();
    let cnv = (*msg).buf.cast::<dlm_convert_lock>();
    let mut res: *mut dlm_lock_resource = core::ptr::null_mut();
    let mut lock: *mut dlm_lock = core::ptr::null_mut();
    let mut status = DLM_NORMAL;
    let mut call_ast = 0;
    let mut kick_thread = 0;
    let mut ast_reserved = 0;
    let mut wake = 0;
    if !dlm_grab(dlm) { dlm_error(DLM_REJECTED); return DLM_REJECTED as i32; }
    if (*cnv).namelen > DLM_LOCKID_NAME_MAX { status = DLM_IVBUFLEN; dlm_error(status); goto_leave!(status, dlm, res, lock, call_ast, ast_reserved, kick_thread); }
    let flags = be32_to_cpu((*cnv).flags);
    if flags & (LKM_PUT_LVB | LKM_GET_LVB) == (LKM_PUT_LVB | LKM_GET_LVB) { status = DLM_BADARGS; goto_leave!(status, dlm, res, lock, call_ast, ast_reserved, kick_thread); }
    res = dlm_lookup_lockres(dlm, (*cnv).name.as_ptr(), (*cnv).namelen);
    if res.is_null() { status = DLM_IVLOCKID; goto_leave!(status, dlm, res, lock, call_ast, ast_reserved, kick_thread); }
    spin_lock(&mut (*res).spinlock);
    status = __dlm_lockres_state_to_status(res);
    if status == DLM_NORMAL { for_each_entry!(lock, &(*res).granted, list) { if (*lock).ml.cookie == (*cnv).cookie && (*lock).ml.node == (*cnv).node_idx { dlm_lock_get(lock); break; } } }
    spin_unlock(&mut (*res).spinlock);
    if lock.is_null() { status = DLM_IVLOCKID; goto_leave!(status, dlm, res, lock, call_ast, ast_reserved, kick_thread); }
    if flags & LKM_PUT_LVB != 0 { (*(*lock).lksb).flags |= DLM_LKSB_PUT_LVB; core::ptr::copy_nonoverlapping((*cnv).lvb.as_ptr(), (*(*lock).lksb).lvb.as_mut_ptr(), DLM_LVB_LEN as usize); } else if flags & LKM_GET_LVB != 0 { (*(*lock).lksb).flags |= DLM_LKSB_GET_LVB; }
    spin_lock(&mut (*res).spinlock);
    status = __dlm_lockres_state_to_status(res);
    if status == DLM_NORMAL { __dlm_lockres_reserve_ast(res); ast_reserved = 1; (*res).state |= DLM_LOCK_RES_IN_PROGRESS; status = __dlmconvert_master(dlm, res, lock, flags as i32, (*cnv).requested_type, &mut call_ast, &mut kick_thread); (*res).state &= !DLM_LOCK_RES_IN_PROGRESS; wake = 1; }
    spin_unlock(&mut (*res).spinlock);
    if wake != 0 { wake_up(&mut (*res).wq); }
    if status != DLM_NORMAL { (*(*lock).lksb).flags &= !(DLM_LKSB_GET_LVB | DLM_LKSB_PUT_LVB); }
    if !lock.is_null() { dlm_lock_put(lock); }
    if call_ast != 0 { dlm_queue_ast(dlm, lock); } else if ast_reserved != 0 { dlm_lockres_release_ast(dlm, res); }
    if kick_thread != 0 { dlm_kick_thread(dlm, res); }
    if !res.is_null() { dlm_lockres_put(res); }
    dlm_put(dlm);
    status as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
