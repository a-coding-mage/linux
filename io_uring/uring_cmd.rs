// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/io_uring implementation.

pub unsafe extern "C" fn io_cmd_cache_free(entry: *const core::ffi::c_void) {
    let ac = entry as *mut io_async_cmd;
    io_vec_free(&mut (*ac).vec);
    kfree(ac as *mut core::ffi::c_void);
}

unsafe fn io_req_uring_cleanup(req: *mut io_kiocb, issue_flags: u32) {
    let ioucmd = io_kiocb_to_cmd::<io_uring_cmd>(req);
    let ac = (*req).async_data;

    if issue_flags & IO_URING_F_UNLOCKED != 0 { return; }
    io_alloc_cache_vec_kasan(&mut (*ac).vec);
    if (*ac).vec.nr > IO_VEC_CACHE_SOFT_CAP { io_vec_free(&mut (*ac).vec); }
    if io_alloc_cache_put(&mut (*(*req).ctx).cmd_cache, ac) {
        (*ioucmd).sqe = core::ptr::null();
        io_req_async_data_clear(req, REQ_F_NEED_CLEANUP);
    } else { io_vec_free(&mut (*ac).vec); }
}

pub unsafe extern "C" fn io_uring_cmd_cleanup(req: *mut io_kiocb) { io_req_uring_cleanup(req, 0); }

pub unsafe extern "C" fn io_uring_try_cancel_uring_cmd(ctx: *mut io_ring_ctx, tctx: *mut io_uring_task, cancel_all: bool) -> bool {
    let mut ret = false;
    lockdep_assert_held(&(*ctx).uring_lock);
    // hlist_for_each_entry_safe(req, tmp, &ctx->cancelable_uring_cmd, hash_node)
    let mut pos = (*ctx).cancelable_uring_cmd.first;
    while !pos.is_null() {
        let req = hlist_entry::<io_kiocb>(pos, hash_node_offset());
        let cmd = io_kiocb_to_cmd::<io_uring_cmd>(req);
        let next = (*pos).next;
        if !cancel_all && (*req).tctx != tctx { pos = next; continue; }
        if (*cmd).flags & IORING_URING_CMD_CANCELABLE != 0 {
            ((*(*req).file).f_op.uring_cmd)(cmd, IO_URING_F_CANCEL | IO_URING_F_COMPLETE_DEFER);
            ret = true;
        }
        pos = next;
    }
    io_submit_flush_completions(ctx); ret
}

unsafe fn io_uring_cmd_del_cancelable(cmd: *mut io_uring_cmd, issue_flags: u32) {
    let req = cmd_to_io_kiocb(cmd); let ctx = (*req).ctx;
    if (*cmd).flags & IORING_URING_CMD_CANCELABLE == 0 { return; }
    (*cmd).flags &= !IORING_URING_CMD_CANCELABLE;
    io_ring_submit_lock(ctx, issue_flags); hlist_del(&mut (*req).hash_node); io_ring_submit_unlock(ctx, issue_flags);
}

pub unsafe extern "C" fn io_uring_cmd_mark_cancelable(cmd: *mut io_uring_cmd, issue_flags: u32) {
    let req = cmd_to_io_kiocb(cmd); let ctx = (*req).ctx;
    if (*req).flags & REQ_F_IOPOLL != 0 { return; }
    if (*cmd).flags & IORING_URING_CMD_CANCELABLE == 0 {
        (*cmd).flags |= IORING_URING_CMD_CANCELABLE;
        io_ring_submit_lock(ctx, issue_flags); hlist_add_head(&mut (*req).hash_node, &mut (*ctx).cancelable_uring_cmd); io_ring_submit_unlock(ctx, issue_flags);
    }
}

pub unsafe extern "C" fn __io_uring_cmd_do_in_task(ioucmd: *mut io_uring_cmd, task_work_cb: io_req_tw_func_t, flags: u32) {
    let req = cmd_to_io_kiocb(ioucmd);
    if WARN_ON_ONCE((*req).flags & REQ_F_APOLL_MULTISHOT != 0) { return; }
    (*req).io_task_work.func = task_work_cb; __io_req_task_work_add(req, flags);
}

#[inline] unsafe fn io_req_set_cqe32_extra(req: *mut io_kiocb, extra1: u64, extra2: u64) { (*req).big_cqe.extra1 = extra1; (*req).big_cqe.extra2 = extra2; }

pub unsafe extern "C" fn __io_uring_cmd_done(ioucmd: *mut io_uring_cmd, ret: i32, res2: u64, issue_flags: u32, is_cqe32: bool) {
    let req = cmd_to_io_kiocb(ioucmd);
    if WARN_ON_ONCE((*req).flags & REQ_F_APOLL_MULTISHOT != 0) { return; }
    io_uring_cmd_del_cancelable(ioucmd, issue_flags); if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
    if is_cqe32 { if (*(*req).ctx).flags & IORING_SETUP_CQE_MIXED != 0 { (*req).cqe.flags |= IORING_CQE_F_32; } io_req_set_cqe32_extra(req, res2, 0); }
    io_req_uring_cleanup(req, issue_flags);
    if (*req).flags & REQ_F_IOPOLL != 0 { smp_store_release(&mut (*req).iopoll_completed, 1); }
    else if issue_flags & IO_URING_F_COMPLETE_DEFER != 0 { if WARN_ON_ONCE(issue_flags & IO_URING_F_UNLOCKED != 0) { return; } io_req_complete_defer(req); }
    else { (*req).io_task_work.func = io_req_task_complete; io_req_task_work_add(req); }
}

pub unsafe extern "C" fn io_uring_cmd_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let cmd = io_kiocb_to_cmd::<io_uring_cmd>(req); let ac;
    if (*sqe).__pad1 != 0 { return -EINVAL; }
    (*cmd).flags = READ_ONCE((*sqe).uring_cmd_flags); if (*cmd).flags & !IORING_URING_CMD_MASK != 0 { return -EINVAL; }
    if (*cmd).flags & IORING_URING_CMD_FIXED != 0 { if (*cmd).flags & IORING_URING_CMD_MULTISHOT != 0 { return -EINVAL; } (*req).buf_index = READ_ONCE((*sqe).buf_index); }
    if ((*cmd).flags & IORING_URING_CMD_MULTISHOT != 0) != ((*req).flags & REQ_F_BUFFER_SELECT != 0) { return -EINVAL; }
    (*cmd).cmd_op = READ_ONCE((*sqe).cmd_op);
    ac = io_uring_alloc_async_data(&mut (*(*req).ctx).cmd_cache, req); if ac.is_null() { return -ENOMEM; }
    if !(*ac).vec.iovec.is_null() { (*req).flags |= REQ_F_NEED_CLEANUP; } (*cmd).sqe = sqe; 0
}

#[inline] unsafe fn uring_sqe_size(req: *mut io_kiocb) -> usize { if (*(*req).ctx).flags & IORING_SETUP_SQE128 != 0 || (*req).opcode == IORING_OP_URING_CMD128 { 2 * core::mem::size_of::<io_uring_sqe>() } else { core::mem::size_of::<io_uring_sqe>() } }

pub unsafe extern "C" fn io_uring_cmd_sqe_copy(req: *mut io_kiocb) { let cmd = io_kiocb_to_cmd::<io_uring_cmd>(req); let ac = (*req).async_data; if (*cmd).sqe == (*ac).sqes { if WARN_ON_ONCE(true) { return; } } core::ptr::copy_nonoverlapping((*cmd).sqe as *const u8, (*ac).sqes as *mut u8, uring_sqe_size(req)); (*cmd).sqe = (*ac).sqes; }

// Remaining declarations use kernel-provided types and helpers.
pub unsafe extern "C" fn io_uring_cmd(req: *mut io_kiocb, issue_flags: u32) -> i32 { let cmd=io_kiocb_to_cmd::<io_uring_cmd>(req); let ctx=(*req).ctx; let file=(*req).file; if (*file).f_op.uring_cmd.is_none(){return -EOPNOTSUPP;} let mut f=issue_flags; let mut ret=security_uring_cmd(cmd); if ret!=0{return ret;} if (*ctx).flags & IORING_SETUP_SQE128!=0 || (*req).opcode==IORING_OP_URING_CMD128{f|=IO_URING_F_SQE128;} if (*ctx).flags & (IORING_SETUP_CQE32|IORING_SETUP_CQE_MIXED)!=0{f|=IO_URING_F_CQE32;} if io_is_compat(ctx){f|=IO_URING_F_COMPAT;} ret=((*file).f_op.uring_cmd)(cmd,f); if ret == -EAGAIN {(*cmd).flags|=IORING_URING_CMD_REISSUE;return ret;} if ret == -EIOCBQUEUED{return ret;} if ret<0{req_set_fail(req);} io_req_uring_cleanup(req,f);io_req_set_res(req,ret,0);IOU_COMPLETE }

pub unsafe extern "C" fn io_uring_cmd_import_fixed(ubuf: u64, len: usize, rw: i32, iter: *mut iov_iter, ioucmd: *mut io_uring_cmd, issue_flags: u32) -> i32 {
    let req=cmd_to_io_kiocb(ioucmd); if WARN_ON_ONCE((*ioucmd).flags & IORING_URING_CMD_FIXED==0){return -EINVAL;} io_import_reg_buf(req,iter,ubuf,len,rw,issue_flags)
}
pub unsafe extern "C" fn io_uring_cmd_import_fixed_vec(ioucmd:*mut io_uring_cmd,uvec:*const iovec,uvec_segs:usize,ddir:i32,iter:*mut iov_iter,issue_flags:u32)->i32{
    let req=cmd_to_io_kiocb(ioucmd);let ac=(*req).async_data;if WARN_ON_ONCE((*ioucmd).flags&IORING_URING_CMD_FIXED==0){return -EINVAL;}let ret=io_prep_reg_iovec(req,&mut (*ac).vec,uvec,uvec_segs);if ret!=0{return ret;}io_import_reg_vec(ddir,iter,req,&mut (*ac).vec,uvec_segs,issue_flags)
}
pub unsafe extern "C" fn io_uring_cmd_issue_blocking(ioucmd:*mut io_uring_cmd){io_queue_iowq(cmd_to_io_kiocb(ioucmd));}
pub unsafe extern "C" fn io_cmd_poll_multishot(cmd:*mut io_uring_cmd,issue_flags:u32,mut mask:__poll_t)->i32{let req=cmd_to_io_kiocb(cmd);if (*req).flags&REQ_F_APOLL_MULTISHOT!=0{return 0;}(*req).flags|=REQ_F_APOLL_MULTISHOT;mask&=!EPOLLONESHOT;let ret=io_arm_apoll(req,issue_flags,mask);if ret==IO_APOLL_OK{-EIOCBQUEUED}else{-ECANCELED}}
pub unsafe extern "C" fn io_uring_cmd_post_mshot_cqe32(cmd:*mut io_uring_cmd,issue_flags:u32,cqe:*mut io_uring_cqe)->bool{let req=cmd_to_io_kiocb(cmd);if WARN_ON_ONCE(issue_flags&IO_URING_F_MULTISHOT==0){return false;}io_req_post_cqe32(req,cqe)}
pub unsafe extern "C" fn io_uring_cmd_buffer_select(ioucmd:*mut io_uring_cmd,buf_group:u32,len:*mut usize,issue_flags:u32)->io_br_sel{let req=cmd_to_io_kiocb(ioucmd);if (*ioucmd).flags&IORING_URING_CMD_MULTISHOT==0{return io_br_sel{val:-EINVAL,..core::mem::zeroed()};}if WARN_ON_ONCE(!io_do_buffer_select(req)){return io_br_sel{val:-EINVAL,..core::mem::zeroed()};}io_buffer_select(req,len,buf_group,issue_flags)}
pub unsafe extern "C" fn io_uring_mshot_cmd_post_cqe(ioucmd:*mut io_uring_cmd,sel:*mut io_br_sel,issue_flags:u32)->bool{let req=cmd_to_io_kiocb(ioucmd);let mut cflags=0;if (*ioucmd).flags&IORING_URING_CMD_MULTISHOT==0{return true;}if (*sel).val>0{cflags=io_put_kbuf(req,(*sel).val,(*sel).buf_list);if io_req_post_cqe(req,(*sel).val,cflags|IORING_CQE_F_MORE){return false;}}io_kbuf_recycle(req,(*sel).buf_list,issue_flags);if (*sel).val<0{req_set_fail(req);}io_req_set_res(req,(*sel).val,cflags);true}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
