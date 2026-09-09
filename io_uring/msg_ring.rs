// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding io_uring implementation.

const IORING_MSG_RING_MASK: u32 = IORING_MSG_RING_CQE_SKIP | IORING_MSG_RING_FLAGS_PASS;

#[repr(C)]
pub union io_msg_dst {
    pub dst_fd: u32,
    pub cqe_flags: u32,
}

#[repr(C)]
pub struct io_msg {
    pub file: *mut file,
    pub src_file: *mut file,
    pub tw: callback_head,
    pub user_data: u64,
    pub len: u32,
    pub cmd: u32,
    pub src_fd: u32,
    pub dst: io_msg_dst,
    pub flags: u32,
}

#[inline]
unsafe fn io_double_unlock_ctx(octx: *mut io_ring_ctx) {
    mutex_unlock(core::ptr::addr_of_mut!((*octx).uring_lock));
}

unsafe fn io_lock_external_ctx(octx: *mut io_ring_ctx, issue_flags: u32) -> i32 {
    /*
     * To ensure proper ordering between the two ctxs, we can only
     * attempt a trylock on the target. If that fails and we already have
     * the source ctx lock, punt to io-wq.
     */
    if issue_flags & IO_URING_F_UNLOCKED == 0 {
        if !mutex_trylock(core::ptr::addr_of_mut!((*octx).uring_lock)) {
            return -EAGAIN;
        }
        return 0;
    }
    mutex_lock(core::ptr::addr_of_mut!((*octx).uring_lock));
    0
}

pub unsafe fn io_msg_ring_cleanup(req: *mut io_kiocb) {
    let msg = io_kiocb_to_cmd::<io_msg>(req);

    if WARN_ON_ONCE((*msg).src_file.is_null()) {
        return;
    }
    fput((*msg).src_file);
    (*msg).src_file = core::ptr::null_mut();
}

#[inline]
unsafe fn io_msg_need_remote(target_ctx: *mut io_ring_ctx) -> bool {
    (*target_ctx).int_flags & IO_RING_F_TASK_COMPLETE != 0
}

unsafe fn io_msg_tw_complete(tw_req: io_tw_req, _tw: io_tw_token_t) {
    let req = tw_req.req;
    let ctx = (*req).ctx;
    io_add_aux_cqe(ctx, (*req).cqe.user_data, (*req).cqe.res, (*req).cqe.flags);
    kfree_rcu(req, rcu_head);
    percpu_ref_put(&mut (*ctx).refs);
}

unsafe fn io_msg_remote_post(ctx: *mut io_ring_ctx, req: *mut io_kiocb, res: i32, cflags: u32, user_data: u64) {
    (*req).opcode = IORING_OP_NOP;
    (*req).cqe.user_data = user_data;
    io_req_set_res(req, res, cflags);
    percpu_ref_get(&mut (*ctx).refs);
    (*req).ctx = ctx;
    (*req).tctx = core::ptr::null_mut();
    (*req).io_task_work.func = Some(io_msg_tw_complete);
    io_req_task_work_add_remote(req, IOU_F_TWQ_LAZY_WAKE);
}

unsafe fn io_msg_ring_cqe_flags(target_ctx: *mut io_ring_ctx, msg: *const io_msg, flags: *mut u32) -> i32 {
    *flags = 0;
    if (*msg).flags & IORING_MSG_RING_FLAGS_PASS == 0 {
        return 0;
    }
    *flags = (*msg).dst.cqe_flags;
    if *flags & IORING_CQE_F_32 != 0
        && (*target_ctx).flags & (IORING_SETUP_CQE32 | IORING_SETUP_CQE_MIXED) == 0
    {
        return -EINVAL;
    }
    0
}

unsafe fn io_msg_data_remote(target_ctx: *mut io_ring_ctx, msg: *mut io_msg) -> i32 {
    let mut flags = 0u32;
    let ret = io_msg_ring_cqe_flags(target_ctx, msg, &mut flags);
    if ret != 0 { return ret; }
    let target = kmem_cache_alloc(req_cachep, GFP_KERNEL | __GFP_NOWARN | __GFP_ZERO);
    if target.is_null() { return -ENOMEM; }
    io_msg_remote_post(target_ctx, target, (*msg).len as i32, flags, (*msg).user_data);
    0
}

unsafe fn __io_msg_ring_data(target_ctx: *mut io_ring_ctx, msg: *mut io_msg, issue_flags: u32) -> i32 {
    let mut flags = 0u32;
    if (*msg).src_fd != 0 || (*msg).flags & !IORING_MSG_RING_FLAGS_PASS != 0 { return -EINVAL; }
    if (*msg).flags & IORING_MSG_RING_FLAGS_PASS == 0 && (*msg).dst.dst_fd != 0 { return -EINVAL; }
    if smp_load_acquire(&(*target_ctx).flags) & IORING_SETUP_R_DISABLED != 0 { return -EBADFD; }
    if io_msg_need_remote(target_ctx) { return io_msg_data_remote(target_ctx, msg); }
    let ret = io_msg_ring_cqe_flags(target_ctx, msg, &mut flags);
    if ret != 0 { return ret; }
    let mut ret = -EOVERFLOW;
    if (*target_ctx).flags & IORING_SETUP_IOPOLL != 0 {
        if io_lock_external_ctx(target_ctx, issue_flags) != 0 { return -EAGAIN; }
    }
    if io_post_aux_cqe(target_ctx, (*msg).user_data, (*msg).len as i32, flags) { ret = 0; }
    if (*target_ctx).flags & IORING_SETUP_IOPOLL != 0 { io_double_unlock_ctx(target_ctx); }
    ret
}

unsafe fn io_msg_ring_data(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let target_ctx = (*(*req).file).private_data as *mut io_ring_ctx;
    __io_msg_ring_data(target_ctx, io_kiocb_to_cmd::<io_msg>(req), issue_flags)
}

unsafe fn io_msg_grab_file(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let msg = io_kiocb_to_cmd::<io_msg>(req);
    let ctx = (*req).ctx;
    let mut ret = -EBADF;
    io_ring_submit_lock(ctx, issue_flags);
    let node = io_rsrc_node_lookup(&mut (*ctx).file_table.data, (*msg).src_fd);
    if !node.is_null() {
        (*msg).src_file = io_slot_file(node);
        if !(*msg).src_file.is_null() { get_file((*msg).src_file); }
        (*req).flags |= REQ_F_NEED_CLEANUP;
        ret = 0;
    }
    io_ring_submit_unlock(ctx, issue_flags);
    ret
}

unsafe fn io_msg_install_complete(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let target_ctx = (*(*req).file).private_data as *mut io_ring_ctx;
    let msg = io_kiocb_to_cmd::<io_msg>(req);
    if io_lock_external_ctx(target_ctx, issue_flags) != 0 { return -EAGAIN; }
    let mut ret = __io_fixed_fd_install(target_ctx, (*msg).src_file, (*msg).dst.dst_fd);
    if ret >= 0 {
        (*msg).src_file = core::ptr::null_mut();
        (*req).flags &= !REQ_F_NEED_CLEANUP;
        if (*msg).flags & IORING_MSG_RING_CQE_SKIP == 0 && !io_post_aux_cqe(target_ctx, (*msg).user_data, ret, 0) { ret = -EOVERFLOW; }
    }
    io_double_unlock_ctx(target_ctx);
    ret
}

unsafe fn io_msg_tw_fd_complete(head: *mut callback_head) {
    let msg = container_of::<io_msg>(head, 0);
    let req = cmd_to_io_kiocb(msg);
    let mut ret = -EOWNERDEAD;
    if (*current).flags & PF_EXITING == 0 { ret = io_msg_install_complete(req, IO_URING_F_UNLOCKED); }
    if ret < 0 { req_set_fail(req); }
    io_req_queue_tw_complete(req, ret);
}

unsafe fn io_msg_fd_remote(req: *mut io_kiocb) -> i32 {
    let ctx = (*(*req).file).private_data as *mut io_ring_ctx;
    let msg = io_kiocb_to_cmd::<io_msg>(req);
    init_task_work(&mut (*msg).tw, Some(io_msg_tw_fd_complete));
    if task_work_add((*ctx).submitter_task, &mut (*msg).tw, TWA_SIGNAL) != 0 { return -EOWNERDEAD; }
    IOU_ISSUE_SKIP_COMPLETE
}

unsafe fn io_msg_send_fd(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let target_ctx = (*(*req).file).private_data as *mut io_ring_ctx;
    let msg = io_kiocb_to_cmd::<io_msg>(req);
    let ctx = (*req).ctx;
    if (*msg).len != 0 || target_ctx == ctx { return -EINVAL; }
    if smp_load_acquire(&(*target_ctx).flags) & IORING_SETUP_R_DISABLED != 0 { return -EBADFD; }
    if (*msg).src_file.is_null() { let ret = io_msg_grab_file(req, issue_flags); if ret != 0 { return ret; } }
    if io_msg_need_remote(target_ctx) { return io_msg_fd_remote(req); }
    io_msg_install_complete(req, issue_flags)
}

unsafe fn __io_msg_ring_prep(msg: *mut io_msg, sqe: *const io_uring_sqe) -> i32 {
    if (*sqe).buf_index != 0 || (*sqe).personality != 0 { return -EINVAL; }
    (*msg).src_file = core::ptr::null_mut();
    (*msg).user_data = READ_ONCE((*sqe).off);
    (*msg).len = READ_ONCE((*sqe).len);
    (*msg).cmd = READ_ONCE((*sqe).addr);
    (*msg).src_fd = READ_ONCE((*sqe).addr3);
    (*msg).dst.dst_fd = READ_ONCE((*sqe).file_index);
    (*msg).flags = READ_ONCE((*sqe).msg_ring_flags);
    if (*msg).flags & !IORING_MSG_RING_MASK != 0 { return -EINVAL; }
    0
}

pub unsafe fn io_msg_ring_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 { __io_msg_ring_prep(io_kiocb_to_cmd::<io_msg>(req), sqe) }

pub unsafe fn io_msg_ring(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let msg = io_kiocb_to_cmd::<io_msg>(req);
    let mut ret = -EBADFD;
    if !io_is_uring_fops((*req).file) { return io_msg_finish(req, ret); }
    match (*msg).cmd {
        IORING_MSG_DATA => ret = io_msg_ring_data(req, issue_flags),
        IORING_MSG_SEND_FD => ret = io_msg_send_fd(req, issue_flags),
        _ => ret = -EINVAL,
    }
    io_msg_finish(req, ret)
}

unsafe fn io_msg_finish(req: *mut io_kiocb, ret: i32) -> i32 {
    if ret < 0 { if ret == -EAGAIN || ret == IOU_ISSUE_SKIP_COMPLETE { return ret; } req_set_fail(req); }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_uring_sync_msg_ring(sqe: *mut io_uring_sqe) -> i32 {
    let mut io_msg: io_msg = core::mem::zeroed();
    let mut ret = __io_msg_ring_prep(&mut io_msg, sqe);
    if ret != 0 { return ret; }
    if io_msg.cmd != IORING_MSG_DATA { return -EINVAL; }
    let f = fd_open((*sqe).fd);
    if f.is_null() { return -EBADF; }
    if !io_is_uring_fops(fd_file(f)) { return -EBADFD; }
    ret = __io_msg_ring_data((*fd_file(f)).private_data as *mut io_ring_ctx, &mut io_msg, IO_URING_F_UNLOCKED);
    fd_close(f);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
