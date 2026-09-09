// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/io_uring translation.

#[repr(C)]
pub struct io_cancel {
    pub file: *mut file,
    pub addr: u64,
    pub flags: u32,
    pub fd: i32,
    pub opcode: u8,
}

pub const CANCEL_FLAGS: u32 = IORING_ASYNC_CANCEL_ALL | IORING_ASYNC_CANCEL_FD |
    IORING_ASYNC_CANCEL_ANY | IORING_ASYNC_CANCEL_FD_FIXED |
    IORING_ASYNC_CANCEL_USERDATA | IORING_ASYNC_CANCEL_OP;

pub unsafe fn io_cancel_req_match(req: *mut io_kiocb, cd: *mut io_cancel_data) -> bool {
    let mut match_user_data = (*cd).flags & IORING_ASYNC_CANCEL_USERDATA != 0;
    if (*req).ctx != (*cd).ctx { return false; }
    if (*cd).flags & (IORING_ASYNC_CANCEL_FD | IORING_ASYNC_CANCEL_OP) == 0 { match_user_data = true; }
    if (*cd).flags & IORING_ASYNC_CANCEL_ANY != 0 {
        if io_cancel_match_sequence(req, (*cd).seq) { return false; }
        return true;
    }
    if (*cd).flags & IORING_ASYNC_CANCEL_FD != 0 && (*req).file != (*cd).file { return false; }
    if (*cd).flags & IORING_ASYNC_CANCEL_OP != 0 && (*req).opcode != (*cd).opcode { return false; }
    if match_user_data && (*req).cqe.user_data != (*cd).data { return false; }
    if (*cd).flags & IORING_ASYNC_CANCEL_ALL != 0 && io_cancel_match_sequence(req, (*cd).seq) { return false; }
    true
}

unsafe fn io_cancel_cb(work: *mut io_wq_work, data: *mut core::ffi::c_void) -> bool {
    let req = container_of!(work, io_kiocb, work);
    io_cancel_req_match(req, data as *mut io_cancel_data)
}

unsafe fn io_async_cancel_one(tctx: *mut io_uring_task, cd: *mut io_cancel_data) -> i32 {
    if tctx.is_null() || (*tctx).io_wq.is_null() { return -ENOENT; }
    let all = (*cd).flags & (IORING_ASYNC_CANCEL_ALL | IORING_ASYNC_CANCEL_ANY) != 0;
    match io_wq_cancel_cb((*tctx).io_wq, io_cancel_cb, cd as *mut core::ffi::c_void, all) {
        IO_WQ_CANCEL_OK => 0,
        IO_WQ_CANCEL_RUNNING => -EALREADY,
        IO_WQ_CANCEL_NOTFOUND => -ENOENT,
        _ => 0,
    }
}

pub unsafe fn io_try_cancel(tctx: *mut io_uring_task, cd: *mut io_cancel_data, issue_flags: u32) -> i32 {
    let ctx = (*cd).ctx;
    let mut ret = io_async_cancel_one(tctx, cd);
    if ret == 0 { return 0; }
    ret = io_poll_cancel(ctx, cd, issue_flags); if ret != -ENOENT { return ret; }
    ret = io_waitid_cancel(ctx, cd, issue_flags); if ret != -ENOENT { return ret; }
    ret = io_futex_cancel(ctx, cd, issue_flags); if ret != -ENOENT { return ret; }
    spin_lock(&mut (*ctx).completion_lock);
    if (*cd).flags & IORING_ASYNC_CANCEL_FD == 0 { ret = io_timeout_cancel(ctx, cd); }
    spin_unlock(&mut (*ctx).completion_lock); ret
}

pub unsafe fn io_async_cancel_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let cancel = io_kiocb_to_cmd!(req, io_cancel);
    if (*req).flags & REQ_F_BUFFER_SELECT != 0 || (*sqe).off != 0 || (*sqe).splice_fd_in != 0 { return -EINVAL; }
    (*cancel).addr = read_once!((*sqe).addr); (*cancel).flags = read_once!((*sqe).cancel_flags);
    if (*cancel).flags & !CANCEL_FLAGS != 0 { return -EINVAL; }
    if (*cancel).flags & IORING_ASYNC_CANCEL_FD != 0 { if (*cancel).flags & IORING_ASYNC_CANCEL_ANY != 0 { return -EINVAL; } (*cancel).fd = read_once!((*sqe).fd); }
    if (*cancel).flags & IORING_ASYNC_CANCEL_OP != 0 { if (*cancel).flags & IORING_ASYNC_CANCEL_ANY != 0 { return -EINVAL; } let op = read_once!((*sqe).len); if op >= IORING_OP_LAST { return -EINVAL; } (*cancel).opcode = op as u8; }
    0
}

unsafe fn __io_async_cancel(cd: *mut io_cancel_data, tctx: *mut io_uring_task, issue_flags: u32) -> i32 {
    let all = (*cd).flags & (IORING_ASYNC_CANCEL_ALL | IORING_ASYNC_CANCEL_ANY) != 0;
    let ctx = (*cd).ctx; let mut nr = 0;
    loop { let ret = io_try_cancel(tctx, cd, issue_flags); if ret == -ENOENT { break; } if !all { return ret; } nr += 1; }
    set_current_state!(TASK_RUNNING); io_ring_submit_lock(ctx, issue_flags); mutex_lock(&mut (*ctx).tctx_lock); let mut ret = -ENOENT;
    list_for_each_entry!(node in (*ctx).tctx_list.ctx_node, io_tctx_node) { ret = io_async_cancel_one((*node).task.io_uring, cd); if ret != -ENOENT { if !all { break; } nr += 1; } }
    mutex_unlock(&mut (*ctx).tctx_lock); io_ring_submit_unlock(ctx, issue_flags); if all { nr } else { ret }
}

pub unsafe fn io_async_cancel(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let cancel = io_kiocb_to_cmd!(req, io_cancel);
    let mut cd = io_cancel_data { ctx: (*req).ctx, data: (*cancel).addr, flags: (*cancel).flags, opcode: (*cancel).opcode, seq: atomic_inc_return!(&mut (*(*req).ctx).cancel_seq), file: core::ptr::null_mut() };
    if cd.flags & IORING_ASYNC_CANCEL_FD != 0 { (*req).file = if (*req).flags & REQ_F_FIXED_FILE != 0 || cd.flags & IORING_ASYNC_CANCEL_FD_FIXED != 0 { (*req).flags |= REQ_F_FIXED_FILE; io_file_get_fixed(req, (*cancel).fd, issue_flags) } else { io_file_get_normal(req, (*cancel).fd) }; if (*req).file.is_null() { io_req_set_res!(req, -EBADF, 0); return IOU_COMPLETE; } cd.file = (*req).file; }
    let ret = __io_async_cancel(&mut cd, (*req).tctx, issue_flags); if ret < 0 { req_set_fail(req); } io_req_set_res!(req, ret, 0); IOU_COMPLETE
}

// The remaining cancellation helpers are declared in the surrounding translation units.
pub unsafe fn __io_uring_cancel(cancel_all: bool) { io_uring_unreg_ringfd(); io_uring_cancel_generic(cancel_all, core::ptr::null_mut()); }

#[repr(C)]
pub struct io_task_cancel { pub tctx: *mut io_uring_task, pub all: bool }

unsafe fn __io_sync_cancel(tctx: *mut io_uring_task, cd: *mut io_cancel_data, fd: i32) -> i32 {
    let ctx = (*cd).ctx;
    if (*cd).flags & IORING_ASYNC_CANCEL_FD != 0 && (*cd).flags & IORING_ASYNC_CANCEL_FD_FIXED != 0 {
        let node = io_rsrc_node_lookup(&mut (*ctx).file_table.data, fd); if node.is_null() { return -EBADF; }
        (*cd).file = io_slot_file(node); if (*cd).file.is_null() { return -EBADF; }
    }
    __io_async_cancel(cd, tctx, 0)
}

pub unsafe fn io_sync_cancel(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void) -> i32 {
    let mut cd = io_cancel_data { ctx, seq: atomic_inc_return!(&mut (*ctx).cancel_seq), data: 0, flags: 0, opcode: 0, file: core::ptr::null_mut() };
    let mut sc: io_uring_sync_cancel_reg = core::mem::zeroed(); if copy_from_user!(&mut sc, arg, core::mem::size_of::<io_uring_sync_cancel_reg>()) != 0 { return -EFAULT; }
    if sc.flags & !CANCEL_FLAGS != 0 { return -EINVAL; }
    for v in sc.pad.iter() { if *v != 0 { return -EINVAL; } } for v in sc.pad2.iter() { if *v != 0 { return -EINVAL; } }
    cd.data = sc.addr; cd.flags = sc.flags; cd.opcode = sc.opcode;
    let mut file: *mut file = core::ptr::null_mut();
    if cd.flags & IORING_ASYNC_CANCEL_FD != 0 && cd.flags & IORING_ASYNC_CANCEL_FD_FIXED == 0 { file = fget(sc.fd); if file.is_null() { return -EBADF; } cd.file = file; }
    let mut ret = __io_sync_cancel((*ctx).current.io_uring, &mut cd, sc.fd); if ret != -EALREADY { if !file.is_null() { fput(file); } return ret; }
    let mut timeout = KTIME_MAX; if sc.timeout.tv_sec != -1isize as u64 || sc.timeout.tv_nsec != -1isize as u64 { timeout = ktime_add_ns(timespec64_to_ktime(sc.timeout), ktime_get_ns()); }
    let mut wait = define_wait!();
    loop { cd.seq = atomic_inc_return!(&mut (*ctx).cancel_seq); prepare_to_wait!(&mut (*ctx).cq_wait, &mut wait, TASK_INTERRUPTIBLE); ret = __io_sync_cancel((*ctx).current.io_uring, &mut cd, sc.fd); mutex_unlock(&mut (*ctx).uring_lock); if ret != -EALREADY { break; } ret = io_run_task_work_sig(ctx); if ret < 0 { break; } ret = schedule_hrtimeout(&timeout, HRTIMER_MODE_ABS); if ret == 0 { ret = -ETIME; break; } mutex_lock(&mut (*ctx).uring_lock); }
    finish_wait!(&mut (*ctx).cq_wait, &mut wait); mutex_lock(&mut (*ctx).uring_lock); if ret == -ENOENT || ret > 0 { ret = 0; } if !file.is_null() { fput(file); } ret
}

pub unsafe fn io_cancel_remove_all(ctx: *mut io_ring_ctx, tctx: *mut io_uring_task, list: *mut hlist_head, cancel_all: bool, cancel: unsafe fn(*mut io_kiocb) -> bool) -> bool {
    lockdep_assert_held!(&mut (*ctx).uring_lock); let mut found = false; hlist_for_each_entry_safe!(req, tmp, list, hash_node, io_kiocb) { if !io_match_task_safe(req, tctx, cancel_all) { continue; } hlist_del_init!(&mut (*req).hash_node); if cancel(req) { found = true; } } found
}

pub unsafe fn io_cancel_remove(ctx: *mut io_ring_ctx, cd: *mut io_cancel_data, issue_flags: u32, list: *mut hlist_head, cancel: unsafe fn(*mut io_kiocb) -> bool) -> i32 {
    let mut nr = 0; io_ring_submit_lock(ctx, issue_flags); hlist_for_each_entry_safe!(req, tmp, list, hash_node, io_kiocb) { if !io_cancel_req_match(req, cd) { continue; } if cancel(req) { nr += 1; } if (*cd).flags & IORING_ASYNC_CANCEL_ALL == 0 { break; } } io_ring_submit_unlock(ctx, issue_flags); if nr != 0 { nr } else { -ENOENT }
}

unsafe fn io_match_linked(head: *mut io_kiocb) -> bool { io_for_each_link!(req, head) { if (*req).flags & REQ_F_INFLIGHT != 0 { return true; } } false }
pub unsafe fn io_match_task_safe(head: *mut io_kiocb, tctx: *mut io_uring_task, cancel_all: bool) -> bool {
    if !tctx.is_null() && (*head).tctx != tctx { return false; } if cancel_all { return true; }
    if (*head).flags & REQ_F_LINK_TIMEOUT != 0 { let ctx = (*head).ctx; raw_spin_lock_irq(&mut (*ctx).timeout_lock); let m = io_match_linked(head); raw_spin_unlock_irq(&mut (*ctx).timeout_lock); m } else { io_match_linked(head) }
}

unsafe fn io_cancel_task_cb(work: *mut io_wq_work, data: *mut core::ffi::c_void) -> bool { let req = container_of!(work, io_kiocb, work); let c = data as *mut io_task_cancel; io_match_task_safe(req, (*c).tctx, (*c).all) }

unsafe fn io_cancel_defer_files(ctx: *mut io_ring_ctx, tctx: *mut io_uring_task, cancel_all: bool) -> bool {
    let mut list = list_head!(); let mut found = false;
    list_for_each_entry_reverse!(de, (*ctx).defer_list, list, io_defer_entry) { if io_match_task_safe((*de).req, tctx, cancel_all) { list_cut_position!(&mut list, &mut (*ctx).defer_list, &mut (*de).list); found = true; break; } }
    if !found || list_empty!(&list) { return false; }
    while !list_empty!(&list) { let de = list_first_entry!(&mut list, io_defer_entry, list); list_del_init!(&mut (*de).list); (*ctx).nr_drained -= io_linked_nr((*de).req); io_req_task_queue_fail((*de).req, -ECANCELED); kfree(de); }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
