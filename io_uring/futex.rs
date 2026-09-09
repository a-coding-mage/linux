// SPDX-License-Identifier: GPL-2.0
// Translated from futex.c. Kernel and io_uring dependencies are supplied externally.

const IO_FUTEX_ALLOC_CACHE_MAX: usize = 32;

#[repr(C)]
pub struct io_futex {
    pub file: *mut file,
    pub uaddr: *mut core::ffi::c_void,
    pub futex_val: libc::c_ulong,
    pub futex_mask: libc::c_ulong,
    pub futex_flags: u32,
    pub futex_nr: u32,
    pub futexv_unqueued: bool,
}

#[repr(C)]
pub struct io_futex_data {
    pub q: futex_q,
    pub req: *mut io_kiocb,
}

#[repr(C)]
pub struct io_futexv_data {
    pub owned: libc::c_ulong,
    pub futexv: [futex_vector; 0],
}

pub unsafe fn io_futex_cache_init(ctx: *mut io_ring_ctx) -> bool {
    io_alloc_cache_init(
        &mut (*ctx).futex_cache,
        IO_FUTEX_ALLOC_CACHE_MAX,
        core::mem::size_of::<io_futex_data>(),
        0,
    )
}

pub unsafe fn io_futex_cache_free(ctx: *mut io_ring_ctx) {
    io_alloc_cache_free(&mut (*ctx).futex_cache, kfree);
}

unsafe fn __io_futex_complete(tw_req: io_tw_req, tw: io_tw_token_t) {
    hlist_del_init(&mut (*tw_req.req).hash_node);
    io_req_task_complete(tw_req, tw);
}

unsafe fn io_futex_complete(tw_req: io_tw_req, tw: io_tw_token_t) {
    let req = tw_req.req;
    let ctx = (*req).ctx;

    io_tw_lock(ctx, tw);
    io_cache_free(&mut (*ctx).futex_cache, (*req).async_data);
    io_req_async_data_clear(req, 0);
    __io_futex_complete(tw_req, tw);
}

unsafe fn io_futexv_complete(tw_req: io_tw_req, tw: io_tw_token_t) {
    let req = tw_req.req;
    let iof = io_kiocb_to_cmd::<io_futex>(req);
    let ifd = (*req).async_data as *mut io_futexv_data;

    io_tw_lock((*req).ctx, tw);

    if !(*iof).futexv_unqueued {
        let res = futex_unqueue_multiple((*ifd).futexv.as_mut_ptr(), (*iof).futex_nr);
        if res != -1 {
            io_req_set_res(req, res, 0);
        }
    }

    io_req_async_data_free(req);
    __io_futex_complete(tw_req, tw);
}

unsafe fn io_futexv_claim(ifd: *mut io_futexv_data) -> bool {
    if test_bit(0, &(*ifd).owned) || test_and_set_bit_lock(0, &mut (*ifd).owned) {
        return false;
    }
    true
}

unsafe fn __io_futex_cancel(req: *mut io_kiocb) -> bool {
    if (*req).opcode == IORING_OP_FUTEX_WAIT {
        let ifd = (*req).async_data as *mut io_futex_data;
        if !futex_unqueue(&mut (*ifd).q) {
            return false;
        }
        (*req).io_task_work.func = Some(io_futex_complete);
    } else {
        let ifd = (*req).async_data as *mut io_futexv_data;
        if !io_futexv_claim(ifd) {
            return false;
        }
        (*req).io_task_work.func = Some(io_futexv_complete);
    }

    hlist_del_init(&mut (*req).hash_node);
    io_req_set_res(req, -ECANCELED, 0);
    io_req_task_work_add(req);
    true
}

pub unsafe fn io_futex_cancel(
    ctx: *mut io_ring_ctx,
    cd: *mut io_cancel_data,
    issue_flags: u32,
) -> i32 {
    io_cancel_remove(ctx, cd, issue_flags, &mut (*ctx).futex_list, __io_futex_cancel)
}

pub unsafe fn io_futex_remove_all(
    ctx: *mut io_ring_ctx,
    tctx: *mut io_uring_task,
    cancel_all: bool,
) -> bool {
    io_cancel_remove_all(ctx, tctx, &mut (*ctx).futex_list, cancel_all, __io_futex_cancel)
}

pub unsafe fn io_futex_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let iof = io_kiocb_to_cmd::<io_futex>(req);
    let flags: u32;

    if (*sqe).len != 0 || (*sqe).futex_flags != 0 || (*sqe).buf_index != 0 || (*sqe).file_index != 0 {
        return -EINVAL;
    }

    (*iof).uaddr = u64_to_user_ptr(READ_ONCE((*sqe).addr));
    (*iof).futex_val = READ_ONCE((*sqe).addr2);
    (*iof).futex_mask = READ_ONCE((*sqe).addr3);
    flags = READ_ONCE((*sqe).fd);

    if flags & !FUTEX2_VALID_MASK != 0 || !futex_flags_valid(futex2_to_flags(flags)) {
        return -EINVAL;
    }
    (*iof).futex_flags = futex2_to_flags(flags);
    if !futex_validate_input((*iof).futex_flags, (*iof).futex_val)
        || !futex_validate_input((*iof).futex_flags, (*iof).futex_mask) {
        return -EINVAL;
    }
    0
}

pub unsafe fn io_futex_wait_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let iof = io_kiocb_to_cmd::<io_futex>(req);
    let ret = io_futex_prep(req, sqe);
    if ret != 0 { return ret; }
    if (*iof).futex_flags & FLAGS_SHARED == 0 { io_req_track_inflight(req); }
    0
}

unsafe fn io_futex_wakev_fn(wake_q: *mut wake_q_head, q: *mut futex_q) {
    let req = (*q).wake_data as *mut io_kiocb;
    let ifd = (*req).async_data as *mut io_futexv_data;
    if !io_futexv_claim(ifd) { __futex_wake_mark(q); return; }
    if !__futex_wake_mark(q) { return; }
    io_req_set_res(req, 0, 0);
    (*req).io_task_work.func = Some(io_futexv_complete);
    __io_req_task_work_add(req, IOU_F_TWQ_IN_WAKE);
}

pub unsafe fn io_futexv_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let iof = io_kiocb_to_cmd::<io_futex>(req);
    if (*sqe).fd != 0 || (*sqe).buf_index != 0 || (*sqe).file_index != 0 || (*sqe).addr2 != 0 || (*sqe).futex_flags != 0 || (*sqe).addr3 != 0 { return -EINVAL; }
    (*iof).uaddr = u64_to_user_ptr(READ_ONCE((*sqe).addr));
    (*iof).futex_nr = READ_ONCE((*sqe).len);
    if (*iof).futex_nr == 0 || (*iof).futex_nr > FUTEX_WAITV_MAX { return -EINVAL; }
    let size = core::mem::size_of::<io_futexv_data>() + (*iof).futex_nr as usize * core::mem::size_of::<futex_vector>();
    let ifd = kzalloc(size, GFP_KERNEL_ACCOUNT) as *mut io_futexv_data;
    if ifd.is_null() { return -ENOMEM; }
    let ret = futex_parse_waitv((*ifd).futexv.as_mut_ptr(), (*iof).uaddr, (*iof).futex_nr, io_futex_wakev_fn, req);
    if ret != 0 { kfree(ifd as *mut core::ffi::c_void); return ret; }
    for i in 0..(*iof).futex_nr as usize { if (*(*ifd).futexv.as_ptr().add(i)).w.flags & FLAGS_SHARED == 0 { io_req_track_inflight(req); break; } }
    (*iof).futexv_unqueued = false;
    (*req).flags |= REQ_F_ASYNC_DATA;
    (*req).async_data = ifd as *mut core::ffi::c_void;
    0
}

unsafe fn io_futex_wake_fn(_wake_q: *mut wake_q_head, q: *mut futex_q) {
    let ifd = container_of::<futex_q, io_futex_data>(q, |x| &mut (*x).q);
    let req = (*ifd).req;
    if !__futex_wake_mark(q) { return; }
    io_req_set_res(req, 0, 0);
    (*req).io_task_work.func = Some(io_futex_complete);
    __io_req_task_work_add(req, IOU_F_TWQ_IN_WAKE);
}

pub unsafe fn io_futexv_wait(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let iof = io_kiocb_to_cmd::<io_futex>(req);
    let ifd = (*req).async_data as *mut io_futexv_data;
    let ctx = (*req).ctx;
    let mut woken: i32 = -1;

    io_ring_submit_lock(ctx, issue_flags);
    let ret = futex_wait_multiple_setup((*ifd).futexv.as_mut_ptr(), (*iof).futex_nr, &mut woken);
    if ret < 0 {
        io_ring_submit_unlock(ctx, issue_flags);
        req_set_fail(req);
        io_req_set_res(req, ret, 0);
        io_req_async_data_free(req);
        return IOU_COMPLETE;
    }
    if ret == 0 {
        __set_current_state(TASK_RUNNING);
        hlist_add_head(&mut (*req).hash_node, &mut (*ctx).futex_list);
    } else {
        (*iof).futexv_unqueued = true;
        if woken != -1 { io_req_set_res(req, woken, 0); }
    }
    io_ring_submit_unlock(ctx, issue_flags);
    IOU_ISSUE_SKIP_COMPLETE
}

pub unsafe fn io_futex_wait(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let iof = io_kiocb_to_cmd::<io_futex>(req);
    let ctx = (*req).ctx;
    let mut ifd: *mut io_futex_data = core::ptr::null_mut();
    let ret: i32;

    if (*iof).futex_mask == 0 { ret = -EINVAL; } else {
        io_ring_submit_lock(ctx, issue_flags);
        ifd = io_cache_alloc(&mut (*ctx).futex_cache, GFP_NOWAIT) as *mut io_futex_data;
        if ifd.is_null() { ret = -ENOMEM; } else {
            (*req).flags |= REQ_F_ASYNC_DATA;
            (*req).async_data = ifd as *mut core::ffi::c_void;
            (*ifd).q = futex_q_init;
            (*ifd).q.bitset = (*iof).futex_mask;
            (*ifd).q.wake = Some(io_futex_wake_fn);
            (*ifd).req = req;
            ret = futex_wait_setup((*iof).uaddr, (*iof).futex_val, (*iof).futex_flags, &mut (*ifd).q, core::ptr::null_mut(), core::ptr::null_mut());
            if ret == 0 {
                hlist_add_head(&mut (*req).hash_node, &mut (*ctx).futex_list);
                io_ring_submit_unlock(ctx, issue_flags);
                return IOU_ISSUE_SKIP_COMPLETE;
            }
        }
        io_ring_submit_unlock(ctx, issue_flags);
    }
    if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
    io_req_async_data_free(req);
    IOU_COMPLETE
}

pub unsafe fn io_futex_wake(req: *mut io_kiocb, _issue_flags: u32) -> i32 {
    let iof = io_kiocb_to_cmd::<io_futex>(req);
    let ret = futex_wake((*iof).uaddr, FLAGS_STRICT | (*iof).futex_flags, core::ptr::null_mut(), (*iof).futex_val, (*iof).futex_mask);
    if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
