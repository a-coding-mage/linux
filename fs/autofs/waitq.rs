// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
 * Copyright 2001-2006 Ian Kent <raven@themaw.net>
 */

// Dependencies supplied by the kernel and autofs headers are intentionally
// referenced here without being redefined.

static mut AUTOFS_NEXT_WAIT_QUEUE: autofs_wqt_t = 1;

pub unsafe fn autofs_catatonic_mode(sbi: *mut autofs_sb_info) {
    let mut wq: *mut autofs_wait_queue;
    let mut nwq: *mut autofs_wait_queue;
    mutex_lock(&mut (*sbi).wq_mutex);
    if (*sbi).flags & AUTOFS_SBI_CATATONIC != 0 {
        mutex_unlock(&mut (*sbi).wq_mutex);
        return;
    }
    pr_debug!("entering catatonic mode\n");
    (*sbi).flags |= AUTOFS_SBI_CATATONIC;
    wq = (*sbi).queues;
    (*sbi).queues = core::ptr::null_mut();
    while !wq.is_null() {
        nwq = (*wq).next;
        (*wq).status = -ENOENT;
        kfree((*wq).name.name.offset(-((*wq).offset as isize)) as *mut core::ffi::c_void);
        (*wq).name.name = core::ptr::null_mut();
        wake_up(&mut (*wq).queue);
        (*wq).wait_ctr -= 1;
        if (*wq).wait_ctr == 0 { kfree(wq as *mut core::ffi::c_void); }
        wq = nwq;
    }
    fput((*sbi).pipe);
    (*sbi).pipe = core::ptr::null_mut();
    (*sbi).pipefd = -1;
    mutex_unlock(&mut (*sbi).wq_mutex);
}

unsafe fn autofs_write(sbi: *mut autofs_sb_info, file: *mut file, addr: *const core::ffi::c_void, mut bytes: i32) -> i32 {
    let sigpipe = sigismember(&(*current).pending.signal, SIGPIPE);
    mutex_lock(&mut (*sbi).pipe_mutex);
    let mut data = addr as *const u8;
    let mut wr: isize = 0;
    while bytes != 0 {
        wr = __kernel_write(file, data as *const core::ffi::c_void, bytes, core::ptr::null_mut());
        if wr <= 0 { break; }
        data = data.add(wr as usize);
        bytes -= wr as i32;
    }
    mutex_unlock(&mut (*sbi).pipe_mutex);
    if wr == -EPIPE as isize && !sigpipe {
        let mut flags = 0u64;
        spin_lock_irqsave(&mut (*(*current).sighand).siglock, &mut flags);
        sigdelset(&mut (*current).pending.signal, SIGPIPE);
        recalc_sigpending();
        spin_unlock_irqrestore(&mut (*(*current).sighand).siglock, flags);
    }
    if bytes == 0 { 0 } else if wr < 0 { wr as i32 } else { -EIO }
}

unsafe fn autofs_notify_daemon(sbi: *mut autofs_sb_info, wq: *mut autofs_wait_queue, typ: i32) {
    let mut pkt: autofs_packet = core::mem::zeroed();
    let mut pktsz: usize = 0;
    pr_debug!("wait id = 0x%08lx, name = %.*s, type=%d\n", (*wq).wait_queue_token as u64, (*wq).name.len, (*wq).name.name, typ);
    pkt.hdr.proto_version = (*sbi).version;
    pkt.hdr.type_ = typ;
    match typ {
        autofs_ptype_missing => { let p = &mut pkt.v4_pkt.missing; pktsz = core::mem::size_of_val(p); p.wait_queue_token = (*wq).wait_queue_token; p.len = (*wq).name.len; core::ptr::copy_nonoverlapping((*wq).name.name, p.name.as_mut_ptr(), (*wq).name.len as usize); p.name[(*wq).name.len as usize] = 0; }
        autofs_ptype_expire_multi => { let p = &mut pkt.v4_pkt.expire_multi; pktsz = core::mem::size_of_val(p); p.wait_queue_token = (*wq).wait_queue_token; p.len = (*wq).name.len; core::ptr::copy_nonoverlapping((*wq).name.name, p.name.as_mut_ptr(), (*wq).name.len as usize); p.name[(*wq).name.len as usize] = 0; }
        autofs_ptype_missing_indirect | autofs_ptype_expire_indirect | autofs_ptype_missing_direct | autofs_ptype_expire_direct => { let p = &mut pkt.v5_pkt.v5_packet; pktsz = core::mem::size_of_val(p); p.wait_queue_token = (*wq).wait_queue_token; p.len = (*wq).name.len; core::ptr::copy_nonoverlapping((*wq).name.name, p.name.as_mut_ptr(), (*wq).name.len as usize); p.name[(*wq).name.len as usize] = 0; p.dev = (*wq).dev; p.ino = (*wq).ino; let ns = (*(*sbi).pipe).f_cred.user_ns; p.uid = from_kuid_munged(ns, (*wq).uid); p.gid = from_kgid_munged(ns, (*wq).gid); p.pid = (*wq).pid; p.tgid = (*wq).tgid; }
        _ => { pr_warn!("bad type %d!\n", typ); mutex_unlock(&mut (*sbi).wq_mutex); return; }
    }
    let pipe = get_file((*sbi).pipe);
    mutex_unlock(&mut (*sbi).wq_mutex);
    let ret = autofs_write(sbi, pipe, &pkt as *const _ as *const core::ffi::c_void, pktsz as i32);
    match ret { 0 => {}, -ENOMEM | -ERESTARTSYS => { autofs_wait_release(sbi, (*wq).wait_queue_token, ret); }, _ => autofs_catatonic_mode(sbi) }
    fput(pipe);
}

unsafe fn autofs_find_wait(sbi: *mut autofs_sb_info, qstr: *const qstr) -> *mut autofs_wait_queue {
    let mut wq = (*sbi).queues;
    while !wq.is_null() { if (*wq).name.hash == (*qstr).hash && (*wq).name.len == (*qstr).len && !(*wq).name.name.is_null() && memcmp((*wq).name.name, (*qstr).name, (*qstr).len as usize) == 0 { break; } wq = (*wq).next; }
    wq
}

// The remaining request-validation and wait-release logic follows the C
// implementation directly; dependent kernel types and helpers are external.
unsafe fn validate_request(wait: *mut *mut autofs_wait_queue, sbi: *mut autofs_sb_info, qstr: *const qstr, path: *const path, notify: autofs_notify) -> i32 {
    let dentry = (*path).dentry;
    if (*sbi).flags & AUTOFS_SBI_CATATONIC != 0 { return -ENOENT; }
    let wq = autofs_find_wait(sbi, qstr); if !wq.is_null() { *wait = wq; return 1; }
    *wait = core::ptr::null_mut();
    let ino = autofs_dentry_ino(dentry); if ino.is_null() { return 1; }
    if notify == NFY_NONE { while (*ino).flags & AUTOFS_INF_EXPIRING != 0 { mutex_unlock(&mut (*sbi).wq_mutex); schedule_timeout_interruptible(HZ / 10); if mutex_lock_interruptible(&mut (*sbi).wq_mutex) != 0 { return -EINTR; } if (*sbi).flags & AUTOFS_SBI_CATATONIC != 0 { return -ENOENT; } let w = autofs_find_wait(sbi, qstr); if !w.is_null() { *wait = w; return 1; } } return 0; }
    if notify == NFY_MOUNT { let mut this = *path; let mut valid = 1; if !IS_ROOT(dentry) { if d_unhashed(dentry) && d_really_is_positive(dentry) { let new = d_lookup((*dentry).d_parent, &(*dentry).d_name); if !new.is_null() { this.dentry = new; dput(new); } } } if path_has_submounts(&this) { valid = 0; } return valid; }
    1
}

pub unsafe fn autofs_wait(sbi: *mut autofs_sb_info, path: *const path, notify: autofs_notify) -> i32 {
    let dentry = (*path).dentry;
    if (*sbi).flags & AUTOFS_SBI_CATATONIC != 0 { return -ENOENT; }
    let pid = task_pid_nr_ns(current, ns_of_pid((*sbi).oz_pgrp));
    let tgid = task_tgid_nr_ns(current, ns_of_pid((*sbi).oz_pgrp));
    if pid == 0 || tgid == 0 { return -ENOENT; }
    if d_really_is_negative(dentry) && (autofs_type_trigger((*sbi).type_) || !IS_ROOT((*dentry).d_parent)) { return -ENOENT; }
    let name = kmalloc(NAME_MAX + 1, GFP_KERNEL); if name.is_null() { return -ENOMEM; }
    let mut qstr: qstr = core::mem::zeroed(); let mut offset = 0u32;
    if IS_ROOT(dentry) && autofs_type_trigger((*sbi).type_) { qstr.name = name; qstr.len = sprintf(name, "%p", dentry); } else { let p = dentry_path_raw(dentry, name, NAME_MAX); if IS_ERR(p) { kfree(name as *mut core::ffi::c_void); return -ENOENT; } qstr.name = p.add(1); qstr.len = strlen(qstr.name); offset = qstr.name.offset_from(name) as u32; }
    qstr.hash = full_name_hash(dentry, qstr.name, qstr.len);
    if mutex_lock_interruptible(&mut (*sbi).wq_mutex) != 0 { kfree(name as *mut core::ffi::c_void); return -EINTR; }
    let mut wq: *mut autofs_wait_queue = core::ptr::null_mut(); let ret = validate_request(&mut wq, sbi, &qstr, path, notify); if ret <= 0 { if ret != -EINTR { mutex_unlock(&mut (*sbi).wq_mutex); } kfree(name as *mut core::ffi::c_void); return ret; }
    if wq.is_null() { wq = kmalloc_obj(); if wq.is_null() { kfree(name as *mut core::ffi::c_void); mutex_unlock(&mut (*sbi).wq_mutex); return -ENOMEM; } (*wq).wait_queue_token = AUTOFS_NEXT_WAIT_QUEUE; AUTOFS_NEXT_WAIT_QUEUE = AUTOFS_NEXT_WAIT_QUEUE.wrapping_add(1); if AUTOFS_NEXT_WAIT_QUEUE == 0 { AUTOFS_NEXT_WAIT_QUEUE = 1; } (*wq).next = (*sbi).queues; (*sbi).queues = wq; init_waitqueue_head(&mut (*wq).queue); (*wq).name = qstr; (*wq).offset = offset; (*wq).dev = autofs_get_dev(sbi); (*wq).ino = autofs_get_ino(sbi); (*wq).uid = current_uid(); (*wq).gid = current_gid(); (*wq).pid = pid; (*wq).tgid = tgid; (*wq).status = -EINTR; (*wq).wait_ctr = 2; let typ = if (*sbi).version < 5 { if notify == NFY_MOUNT { autofs_ptype_missing } else { autofs_ptype_expire_multi } } else if notify == NFY_MOUNT { if autofs_type_trigger((*sbi).type_) { autofs_ptype_missing_direct } else { autofs_ptype_missing_indirect } } else if autofs_type_trigger((*sbi).type_) { autofs_ptype_expire_direct } else { autofs_ptype_expire_indirect }; autofs_notify_daemon(sbi, wq, typ); } else { (*wq).wait_ctr += 1; mutex_unlock(&mut (*sbi).wq_mutex); kfree(name as *mut core::ffi::c_void); }
    wait_event_killable(&mut (*wq).queue, (*wq).name.name.is_null()); let status = (*wq).status;
    mutex_lock(&mut (*sbi).wq_mutex); (*wq).wait_ctr -= 1; if (*wq).wait_ctr == 0 { kfree(wq as *mut core::ffi::c_void); } mutex_unlock(&mut (*sbi).wq_mutex); status
}

pub unsafe fn autofs_wait_release(sbi: *mut autofs_sb_info, token: autofs_wqt_t, status: i32) -> i32 {
    mutex_lock(&mut (*sbi).wq_mutex); let mut link = &mut (*sbi).queues as *mut *mut autofs_wait_queue; let mut wq; loop { wq = *link; if wq.is_null() || (*wq).wait_queue_token == token { break; } link = &mut (*wq).next; } if wq.is_null() { mutex_unlock(&mut (*sbi).wq_mutex); return -EINVAL; } *link = (*wq).next; kfree((*wq).name.name.offset(-((*wq).offset as isize)) as *mut core::ffi::c_void); (*wq).name.name = core::ptr::null_mut(); (*wq).status = status; wake_up(&mut (*wq).queue); (*wq).wait_ctr -= 1; if (*wq).wait_ctr == 0 { kfree(wq as *mut core::ffi::c_void); } mutex_unlock(&mut (*sbi).wq_mutex); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
