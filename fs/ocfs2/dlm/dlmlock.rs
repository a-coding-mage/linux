// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dlmlock.c
 *
 * underlying calls for lock creation
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

// Linux kernel and OCFS2 dependencies are supplied by the surrounding crate.

static mut dlm_lock_cache: *mut kmem_cache = core::ptr::null_mut();
static mut dlm_cookie_lock: spinlock_t = spinlock_t::new();
static mut dlm_next_cookie: u64 = 1;

unsafe fn dlm_can_grant_new_lock(res: *mut dlm_lock, lock: *mut dlm_lock) -> i32 {
    let mut tmplock: *mut dlm_lock;
    list_for_each_entry!(tmplock, unsafe { &mut (*res).granted }, list) {
        if !dlm_lock_compatible((*tmplock).ml.type_, (*lock).ml.type_) { return 0; }
    }
    list_for_each_entry!(tmplock, unsafe { &mut (*res).converting }, list) {
        if !dlm_lock_compatible((*tmplock).ml.type_, (*lock).ml.type_) { return 0; }
        if !dlm_lock_compatible((*tmplock).ml.convert_type, (*lock).ml.type_) { return 0; }
    }
    1
}

unsafe fn dlmlock_master(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource,
                         lock: *mut dlm_lock, flags: i32) -> dlm_status {
    let mut call_ast = 0;
    let mut kick_thread = 0;
    let mut status = DLM_NORMAL;
    mlog!(0, "type=%d\n", (*lock).ml.type_);
    spin_lock(&mut (*res).spinlock);
    status = __dlm_lockres_state_to_status(res);
    if status != DLM_NORMAL && (*lock).ml.node != (*dlm).node_num {
        spin_unlock(&mut (*res).spinlock); dlm_error(status); return status;
    }
    __dlm_wait_on_lockres(res); __dlm_lockres_reserve_ast(res);
    if dlm_can_grant_new_lock(res, lock) != 0 {
        mlog!(0, "I can grant this lock right away\n");
        (*(*lock).lksb).status = DLM_NORMAL; status = DLM_NORMAL;
        dlm_lock_get(lock); list_add_tail!(&mut (*lock).list, &mut (*res).granted);
        if !dlm_is_recovery_lock((*res).lockname.name, (*res).lockname.len) {
            kick_thread = 1; call_ast = 1;
        } else { mlog!(0, "%s: returning DLM_NORMAL to node %u for reco lock\n", (*dlm).name, (*lock).ml.node); }
    } else if (flags & LKM_NOQUEUE) != 0 {
        status = DLM_NOTQUEUED;
        if dlm_is_recovery_lock((*res).lockname.name, (*res).lockname.len) {
            mlog!(0, "%s: returning NOTQUEUED to node %u for reco lock\n", (*dlm).name, (*lock).ml.node);
        }
    } else {
        status = DLM_NORMAL; dlm_lock_get(lock); list_add_tail!(&mut (*lock).list, &mut (*res).blocked); kick_thread = 1;
    }
    spin_unlock(&mut (*res).spinlock); wake_up(&mut (*res).wq);
    if call_ast != 0 { dlm_queue_ast(dlm, lock); } else { dlm_lockres_release_ast(dlm, res); }
    dlm_lockres_calc_usage(dlm, res); if kick_thread != 0 { dlm_kick_thread(dlm, res); }
    status
}

pub unsafe fn dlm_revert_pending_lock(res: *mut dlm_lock_resource, lock: *mut dlm_lock) {
    list_del_init!(&mut (*lock).list); (*(*lock).lksb).flags &= !DLM_LKSB_GET_LVB;
}

unsafe fn dlmlock_remote(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource,
                         lock: *mut dlm_lock, flags: i32) -> dlm_status {
    let mut status = DLM_DENIED; let mut lockres_changed = 1;
    mlog!(0, "type=%d, lockres %.*s, flags = 0x%x\n", (*lock).ml.type_, (*res).lockname.len, (*res).lockname.name, flags);
    spin_lock(&mut (*res).spinlock); __dlm_wait_on_lockres(res);
    if (*res).owner == (*dlm).node_num { spin_unlock(&mut (*res).spinlock); return DLM_RECOVERING; }
    (*res).state |= DLM_LOCK_RES_IN_PROGRESS; dlm_lock_get(lock);
    list_add_tail!(&mut (*lock).list, &mut (*res).blocked); (*lock).lock_pending = 1;
    spin_unlock(&mut (*res).spinlock);
    status = dlm_send_remote_lock_request(dlm, res, lock, flags);
    spin_lock(&mut (*res).spinlock); (*res).state &= !DLM_LOCK_RES_IN_PROGRESS; (*lock).lock_pending = 0;
    if status != DLM_NORMAL {
        if status == DLM_RECOVERING && dlm_is_recovery_lock((*res).lockname.name, (*res).lockname.len) {
            mlog!(0, "%s: recovery lock was owned by dead node %u, remaster it now.\n", (*dlm).name, (*res).owner);
        } else if status != DLM_NOTQUEUED { lockres_changed = 0; dlm_error(status); }
        dlm_revert_pending_lock(res, lock); dlm_lock_put(lock);
    } else if dlm_is_recovery_lock((*res).lockname.name, (*res).lockname.len) {
        mlog!(0, "%s: $RECOVERY lock for this node (%u) is mastered by %u; got lock, manually granting (no ast)\n", (*dlm).name, (*dlm).node_num, (*res).owner);
        list_move_tail!(&mut (*lock).list, &mut (*res).granted);
    }
    spin_unlock(&mut (*res).spinlock); if lockres_changed != 0 { dlm_lockres_calc_usage(dlm, res); }
    wake_up(&mut (*res).wq); status
}

unsafe fn dlm_send_remote_lock_request(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource,
                                       lock: *mut dlm_lock, flags: i32) -> dlm_status {
    let mut create: dlm_create_lock = core::mem::zeroed(); let mut status = 0; let tmpret;
    create.node_idx = (*dlm).node_num; create.requested_type = (*lock).ml.type_;
    create.cookie = (*lock).ml.cookie; create.namelen = (*res).lockname.len;
    create.flags = cpu_to_be32(flags as u32); core::ptr::copy_nonoverlapping((*res).lockname.name, create.name.as_mut_ptr(), create.namelen as usize);
    tmpret = o2net_send_message(DLM_CREATE_LOCK_MSG, (*dlm).key, &mut create, core::mem::size_of::<dlm_create_lock>(), (*res).owner, &mut status);
    if tmpret >= 0 { let ret = status as dlm_status; if ret == DLM_REJECTED { mlog!(ML_ERROR, "%s: stale lockres no longer owned by node %u\n", (*dlm).name, (*res).owner); dlm_print_one_lock_resource(res); BUG!(); } ret }
    else if dlm_is_host_down(tmpret) { DLM_RECOVERING } else { dlm_err_to_dlm_status(tmpret) }
}

pub unsafe fn dlm_lock_get(lock: *mut dlm_lock) { kref_get(&mut (*lock).lock_refs); }
pub unsafe fn dlm_lock_put(lock: *mut dlm_lock) { kref_put(&mut (*lock).lock_refs, dlm_lock_release); }

unsafe fn dlm_lock_release(kref: *mut kref) {
    let lock = container_of!(kref, dlm_lock, lock_refs);
    BUG_ON!(!list_empty(&(*lock).list)); BUG_ON!(!list_empty(&(*lock).ast_list)); BUG_ON!(!list_empty(&(*lock).bast_list));
    BUG_ON!((*lock).ast_pending != 0); BUG_ON!((*lock).bast_pending != 0); dlm_lock_detach_lockres(lock);
    if (*lock).lksb_kernel_allocated != 0 { mlog!(0, "freeing kernel-allocated lksb\n"); kfree((*lock).lksb); }
    kmem_cache_free(dlm_lock_cache, lock);
}

pub unsafe fn dlm_lock_attach_lockres(lock: *mut dlm_lock, res: *mut dlm_lock_resource) { dlm_lockres_get(res); (*lock).lockres = res; }
unsafe fn dlm_lock_detach_lockres(lock: *mut dlm_lock) { let res = (*lock).lockres; if !res.is_null() { (*lock).lockres = core::ptr::null_mut(); mlog!(0, "removing lock's lockres reference\n"); dlm_lockres_put(res); } }

unsafe fn dlm_init_lock(newlock: *mut dlm_lock, type_: i32, node: u8, cookie: u64) {
    INIT_LIST_HEAD!(&mut (*newlock).list); INIT_LIST_HEAD!(&mut (*newlock).ast_list); INIT_LIST_HEAD!(&mut (*newlock).bast_list); spin_lock_init(&mut (*newlock).spinlock);
    (*newlock).ml.type_ = type_; (*newlock).ml.convert_type = LKM_IVMODE; (*newlock).ml.highest_blocked = LKM_IVMODE; (*newlock).ml.node = node; (*newlock).ml.pad1 = 0; (*newlock).ml.list = 0; (*newlock).ml.flags = 0;
    (*newlock).ast = None; (*newlock).bast = None; (*newlock).astdata = core::ptr::null_mut(); (*newlock).ml.cookie = cpu_to_be64(cookie);
    (*newlock).ast_pending = 0; (*newlock).bast_pending = 0; (*newlock).convert_pending = 0; (*newlock).lock_pending = 0; (*newlock).unlock_pending = 0; (*newlock).cancel_pending = 0; (*newlock).lksb_kernel_allocated = 0; kref_init(&mut (*newlock).lock_refs);
}

pub unsafe fn dlm_new_lock(type_: i32, node: u8, cookie: u64, mut lksb: *mut dlm_lockstatus) -> *mut dlm_lock {
    let lock = kmem_cache_zalloc(dlm_lock_cache, GFP_NOFS); if lock.is_null() { return core::ptr::null_mut(); }
    let mut kernel_allocated = 0; if lksb.is_null() { lksb = kzalloc_obj!(dlm_lockstatus, GFP_NOFS); if lksb.is_null() { kmem_cache_free(dlm_lock_cache, lock); return core::ptr::null_mut(); } kernel_allocated = 1; }
    dlm_init_lock(lock, type_, node, cookie); if kernel_allocated != 0 { (*lock).lksb_kernel_allocated = 1; } (*lock).lksb = lksb; (*lksb).lockid = lock; lock
}

pub unsafe fn dlm_init_lock_cache() -> i32 { dlm_lock_cache = kmem_cache_create!("o2dlm_lock", core::mem::size_of::<dlm_lock>(), 0, SLAB_HWCACHE_ALIGN, None); if dlm_lock_cache.is_null() { -ENOMEM } else { 0 } }
pub unsafe fn dlm_destroy_lock_cache() { kmem_cache_destroy(dlm_lock_cache); }

unsafe fn dlm_get_next_cookie(node_num: u8, cookie: *mut u64) {
    let tmpnode = (node_num as u64) << 56; spin_lock(&mut dlm_cookie_lock); *cookie = dlm_next_cookie | tmpnode; dlm_next_cookie = dlm_next_cookie.wrapping_add(1);
    if (dlm_next_cookie & 0xff00000000000000u64) != 0 { mlog!(0, "This node's cookie will now wrap!\n"); dlm_next_cookie = 1; } spin_unlock(&mut dlm_cookie_lock);
}

pub unsafe fn dlm_create_lock_handler(msg: *mut o2net_msg, _len: u32, data: *mut core::ffi::c_void, _ret_data: *mut *mut core::ffi::c_void) -> i32 {
    let dlm = data as *mut dlm_ctxt; BUG_ON!(dlm.is_null()); if !dlm_grab(dlm) { return DLM_REJECTED; }
    let create = (*msg).buf as *mut dlm_create_lock; let mut res = core::ptr::null_mut(); let mut newlock = core::ptr::null_mut(); let mut status = DLM_NORMAL;
    let name = (*create).name.as_mut_ptr(); let namelen = (*create).namelen; status = DLM_REJECTED;
    if !dlm_domain_fully_joined(dlm) { mlog!(ML_ERROR, "Domain %s not fully joined\n", (*dlm).name); dlm_error(status); }
    else if namelen > DLM_LOCKID_NAME_MAX { status = DLM_IVBUFLEN; dlm_error(status); }
    else { status = DLM_SYSERR; newlock = dlm_new_lock((*create).requested_type, (*create).node_idx, be64_to_cpu((*create).cookie), core::ptr::null_mut()); if newlock.is_null() { dlm_error(status); } else { let lksb = (*newlock).lksb; if (be32_to_cpu((*create).flags) & LKM_GET_LVB) != 0 { (*lksb).flags |= DLM_LKSB_GET_LVB; } status = DLM_IVLOCKID; res = dlm_lookup_lockres(dlm, name, namelen); if res.is_null() { dlm_error(status); } else { spin_lock(&mut (*res).spinlock); status = __dlm_lockres_state_to_status(res); spin_unlock(&mut (*res).spinlock); if status == DLM_NORMAL { dlm_lock_attach_lockres(newlock, res); status = dlmlock_master(dlm, res, newlock, be32_to_cpu((*create).flags) as i32); } } } }
    if status != DLM_NORMAL && !newlock.is_null() { dlm_lock_put(newlock); } if !res.is_null() { dlm_lockres_put(res); } dlm_put(dlm); status
}

pub unsafe fn dlmlock(dlm: *mut dlm_ctxt, mode: i32, lksb: *mut dlm_lockstatus, flags: i32, name: *const i8, namelen: i32, ast: dlm_astlockfunc_t, data: *mut core::ffi::c_void, bast: dlm_bastlockfunc_t) -> dlm_status {
    let mut status; let mut res = core::ptr::null_mut(); let mut lock = core::ptr::null_mut(); let convert = flags & LKM_CONVERT; let recovery = flags & LKM_RECOVERY;
    if lksb.is_null() { dlm_error(DLM_BADARGS); return DLM_BADARGS; } status = DLM_BADPARAM;
    if mode != LKM_EXMODE && mode != LKM_PRMODE && mode != LKM_NLMODE { dlm_error(status); return status; } if flags & !LKM_VALID_FLAGS != 0 { dlm_error(status); return status; }
    if recovery != 0 && (!dlm_is_recovery_lock(name, namelen) || convert != 0) { dlm_error(status); return status; } if convert != 0 && flags & LKM_LOCAL != 0 { mlog!(ML_ERROR, "strange LOCAL convert request!\n"); return status; }
    if convert != 0 { lock = (*lksb).lockid; if lock.is_null() { return status; } res = (*lock).lockres; if res.is_null() { return status; } dlm_lockres_get(res); if (*lock).lksb != lksb || (*lock).ast != ast || (*lock).bast != bast || (*lock).astdata != data { status = DLM_BADARGS; return status; } loop { dlm_wait_for_recovery(dlm); status = if (*res).owner == (*dlm).node_num { dlmconvert_master(dlm,res,lock,flags,mode) } else { dlmconvert_remote(dlm,res,lock,flags,mode) }; if status != DLM_RECOVERING && status != DLM_MIGRATING && status != DLM_FORWARD { break; } msleep(100); } }
    else { if name.is_null() { dlm_error(status); return status; } status = DLM_IVBUFLEN; if namelen > DLM_LOCKID_NAME_MAX || namelen < 1 { dlm_error(status); return status; } let mut cookie=0; dlm_get_next_cookie((*dlm).node_num,&mut cookie); lock=dlm_new_lock(mode,(*dlm).node_num,cookie,lksb); if lock.is_null() { dlm_error(status); return status; } if recovery==0 { dlm_wait_for_recovery(dlm); } res=dlm_get_lock_resource(dlm,name,namelen,flags); if res.is_null() { dlm_error(DLM_IVLOCKID); return DLM_IVLOCKID; } dlm_lock_attach_lockres(lock,res); (*lock).ast=ast; (*lock).bast=bast; (*lock).astdata=data; status=if (*res).owner==(*dlm).node_num { dlmlock_master(dlm,res,lock,flags) } else { dlmlock_remote(dlm,res,lock,flags) }; }
    if status != DLM_NORMAL { if !lock.is_null() && convert==0 { dlm_lock_put(lock); } (*lksb).status=status; } if !res.is_null() { dlm_lockres_put(res); } status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
