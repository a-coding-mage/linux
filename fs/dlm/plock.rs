// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005-2008 Red Hat, Inc.  All rights reserved.
 */

// Kernel headers and local DLM headers supplying the following symbols are
// intentionally external dependencies of this translation.

static mut OPS_LOCK: SpinLock = DEFINE_SPINLOCK!();
static mut SEND_LIST: ListHead = LIST_HEAD!();
static mut RECV_LIST: ListHead = LIST_HEAD!();
static mut SEND_WQ: WaitQueueHead = DECLARE_WAIT_QUEUE_HEAD!();
static mut RECV_WQ: WaitQueueHead = DECLARE_WAIT_QUEUE_HEAD!();

#[repr(C)]
struct PlockAsyncData {
    fl: *mut core::ffi::c_void,
    file: *mut File,
    flc: FileLock,
    callback: Option<unsafe extern "C" fn(*mut FileLock, i32) -> i32>,
}

#[repr(C)]
struct PlockOp {
    list: ListHead,
    done: i32,
    info: DlmPlockInfo,
    // if set indicates async handling
    data: *mut PlockAsyncData,
}

#[inline]
unsafe fn set_version(info: *mut DlmPlockInfo) {
    (*info).version[0] = DLM_PLOCK_VERSION_MAJOR;
    (*info).version[1] = DLM_PLOCK_VERSION_MINOR;
    (*info).version[2] = DLM_PLOCK_VERSION_PATCH;
}

unsafe fn plock_lookup_waiter(info: *const DlmPlockInfo) -> *mut PlockOp {
    let mut op: *mut PlockOp = core::ptr::null_mut();
    let mut iter: *mut PlockOp;

    list_for_each_entry!(iter, &raw mut RECV_LIST, list) {
        if (*iter).info.fsid == (*info).fsid
            && (*iter).info.number == (*info).number
            && (*iter).info.owner == (*info).owner
            && (*iter).info.pid == (*info).pid
            && (*iter).info.start == (*info).start
            && (*iter).info.end == (*info).end
            && (*iter).info.ex == (*info).ex
            && (*iter).info.wait != 0
        {
            op = iter;
            break;
        }
    }
    op
}

unsafe fn check_version(info: *mut DlmPlockInfo) -> i32 {
    if DLM_PLOCK_VERSION_MAJOR != (*info).version[0]
        || DLM_PLOCK_VERSION_MINOR < (*info).version[1]
    {
        log_print!("plock device version mismatch: kernel (%u.%u.%u), user (%u.%u.%u)",
            DLM_PLOCK_VERSION_MAJOR, DLM_PLOCK_VERSION_MINOR, DLM_PLOCK_VERSION_PATCH,
            (*info).version[0], (*info).version[1], (*info).version[2]);
        return -EINVAL;
    }
    0
}

unsafe fn dlm_release_plock_op(op: *mut PlockOp) {
    kfree((*op).data as *mut core::ffi::c_void);
    kfree(op as *mut core::ffi::c_void);
}

unsafe fn send_op(op: *mut PlockOp) {
    set_version(&raw mut (*op).info);
    spin_lock(&raw mut OPS_LOCK);
    list_add_tail(&raw mut (*op).list, &raw mut SEND_LIST);
    spin_unlock(&raw mut OPS_LOCK);
    wake_up(&raw mut SEND_WQ);
}

unsafe fn do_lock_cancel(orig_info: *const DlmPlockInfo) -> i32 {
    let op = kzalloc_obj::<PlockOp>(GFP_NOFS);
    if op.is_null() { return -ENOMEM; }
    (*op).info = *orig_info;
    (*op).info.optype = DLM_PLOCK_OP_CANCEL;
    (*op).info.wait = 0;
    send_op(op);
    wait_event!(&raw mut RECV_WQ, (*op).done != 0);
    let rv = (*op).info.rv;
    dlm_release_plock_op(op);
    rv
}

pub unsafe extern "C" fn dlm_posix_lock(lockspace: *mut DlmLockspace, number: u64,
    file: *mut File, _cmd: i32, fl: *mut FileLock) -> i32 {
    let ls = dlm_find_lockspace_local(lockspace);
    if ls.is_null() { return -EINVAL; }
    let op = kzalloc_obj::<PlockOp>(GFP_NOFS);
    if op.is_null() { dlm_put_lockspace(ls); return -ENOMEM; }
    (*op).info.optype = DLM_PLOCK_OP_LOCK;
    (*op).info.pid = (*fl).c.flc_pid;
    (*op).info.ex = lock_is_write(fl);
    (*op).info.wait = ((*fl).c.flc_flags & FL_SLEEP) as i32;
    (*op).info.fsid = (*ls).ls_global_id;
    (*op).info.number = number;
    (*op).info.start = (*fl).fl_start;
    (*op).info.end = (*fl).fl_end;
    (*op).info.owner = (*fl).c.flc_owner as i64 as u64;
    if !(*fl).fl_lmops.is_null() && (*(*fl).fl_lmops).lm_grant.is_some() {
        let data = kzalloc_obj::<PlockAsyncData>(GFP_NOFS);
        if data.is_null() { dlm_release_plock_op(op); dlm_put_lockspace(ls); return -ENOMEM; }
        (*data).callback = (*(*fl).fl_lmops).lm_grant;
        locks_init_lock(&raw mut (*data).flc);
        locks_copy_lock(&raw mut (*data).flc, fl);
        (*data).fl = fl as *mut core::ffi::c_void;
        (*data).file = file;
        (*op).data = data;
        send_op(op);
        dlm_put_lockspace(ls);
        return FILE_LOCK_DEFERRED;
    }
    send_op(op);
    if (*op).info.wait != 0 {
        let mut rv = wait_event_interruptible!(&raw mut RECV_WQ, (*op).done != 0);
        if rv == -ERESTARTSYS {
            spin_lock(&raw mut OPS_LOCK);
            let done = (*op).done != 0;
            spin_unlock(&raw mut OPS_LOCK);
            if !done {
                rv = do_lock_cancel(&raw const (*op).info);
                if rv == 0 { spin_lock(&raw mut OPS_LOCK); list_del(&raw mut (*op).list); spin_unlock(&raw mut OPS_LOCK); rv = -EINTR; }
                else if rv == -ENOENT { wait_event!(&raw mut RECV_WQ, (*op).done != 0); }
                else { wait_event!(&raw mut RECV_WQ, (*op).done != 0); }
            }
        }
    } else { wait_event!(&raw mut RECV_WQ, (*op).done != 0); }
    WARN_ON!(!list_empty(&raw const (*op).list));
    let rv = (*op).info.rv;
    if rv == 0 && locks_lock_file_wait(file, fl) < 0 { log_error!(ls, "dlm_posix_lock: vfs lock error %llx", number); }
    dlm_release_plock_op(op);
    dlm_put_lockspace(ls);
    rv
}

pub unsafe extern "C" fn dlm_posix_unlock(lockspace: *mut DlmLockspace, number: u64, file: *mut File, fl: *mut FileLock) -> i32 {
    let ls = dlm_find_lockspace_local(lockspace); if ls.is_null() { return -EINVAL; }
    (*fl).c.flc_flags |= FL_EXISTS;
    let rv = locks_lock_file_wait(file, fl);
    if rv == -ENOENT { dlm_put_lockspace(ls); return 0; }
    let op = kzalloc_obj::<PlockOp>(GFP_NOFS); if op.is_null() { dlm_put_lockspace(ls); return -ENOMEM; }
    (*op).info.optype = DLM_PLOCK_OP_UNLOCK; (*op).info.pid = (*fl).c.flc_pid;
    (*op).info.fsid = (*ls).ls_global_id; (*op).info.number = number;
    (*op).info.start = (*fl).fl_start; (*op).info.end = (*fl).fl_end;
    (*op).info.owner = (*fl).c.flc_owner as i64 as u64;
    if (*fl).c.flc_flags & FL_CLOSE != 0 { (*op).info.flags |= DLM_PLOCK_FL_CLOSE; send_op(op); dlm_put_lockspace(ls); return 0; }
    send_op(op); wait_event!(&raw mut RECV_WQ, (*op).done != 0);
    let mut result = (*op).info.rv; if result == -ENOENT { result = 0; }
    dlm_release_plock_op(op); dlm_put_lockspace(ls); result
}

pub unsafe extern "C" fn dlm_posix_cancel(lockspace: *mut DlmLockspace, number: u64, file: *mut File, fl: *mut FileLock) -> i32 {
    if (*fl).fl_lmops.is_null() || (*(*fl).fl_lmops).lm_grant.is_none() { return -EOPNOTSUPP; }
    let ls = dlm_find_lockspace_local(lockspace); if ls.is_null() { return -EINVAL; }
    let mut info: DlmPlockInfo = core::mem::zeroed(); info.pid = (*fl).c.flc_pid; info.ex = lock_is_write(fl); info.fsid = (*ls).ls_global_id; info.number = number; info.start = (*fl).fl_start; info.end = (*fl).fl_end; info.owner = (*fl).c.flc_owner as i64 as u64; dlm_put_lockspace(ls);
    let rv = do_lock_cancel(&raw const info); if rv == -ENOENT { dlm_posix_unlock(lockspace, number, file, fl) } else { rv }
}

pub unsafe extern "C" fn dlm_posix_get(lockspace: *mut DlmLockspace, number: u64, _file: *mut File, fl: *mut FileLock) -> i32 {
    let ls = dlm_find_lockspace_local(lockspace); if ls.is_null() { return -EINVAL; }
    let op = kzalloc_obj::<PlockOp>(GFP_NOFS); if op.is_null() { dlm_put_lockspace(ls); return -ENOMEM; }
    (*op).info.optype = DLM_PLOCK_OP_GET; (*op).info.pid = (*fl).c.flc_pid; (*op).info.ex = lock_is_write(fl); (*op).info.fsid = (*ls).ls_global_id; (*op).info.number = number; (*op).info.start = (*fl).fl_start; (*op).info.end = (*fl).fl_end; (*op).info.owner = (*fl).c.flc_owner as i64 as u64;
    send_op(op); wait_event!(&raw mut RECV_WQ, (*op).done != 0); let mut rv = (*op).info.rv; (*fl).c.flc_type = F_UNLCK;
    if rv == -ENOENT { rv = 0; } else if rv > 0 { locks_init_lock(fl); (*fl).c.flc_type = if (*op).info.ex != 0 { F_WRLCK } else { F_RDLCK }; (*fl).c.flc_flags = FL_POSIX; (*fl).c.flc_pid = (*op).info.pid; (*fl).fl_start = (*op).info.start; (*fl).fl_end = (*op).info.end; rv = 0; }
    dlm_release_plock_op(op); dlm_put_lockspace(ls); rv
}

pub unsafe extern "C" fn dlm_plock_init() -> i32 { misc_register(&raw mut PLOCK_DEV_MISC) }
pub unsafe extern "C" fn dlm_plock_exit() { misc_deregister(&raw mut PLOCK_DEV_MISC); WARN_ON!(!list_empty(&raw const SEND_LIST)); WARN_ON!(!list_empty(&raw const RECV_LIST)); }

static mut PLOCK_DEV_MISC: MiscDevice = MiscDevice::zeroed();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
