// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by the surrounding io_uring implementation.

#[repr(C)]
pub struct io_timeout {
    pub file: *mut file,
    pub off: u32,
    pub target_seq: u32,
    pub repeats: u32,
    pub list: list_head,
    // head of the link, used by linked timeouts only
    pub head: *mut io_kiocb,
    // for linked completions
    pub prev: *mut io_kiocb,
}

#[repr(C)]
pub struct io_timeout_rem {
    pub file: *mut file,
    pub addr: u64,
    // timeout update
    pub time: ktime_t,
    pub flags: u32,
    pub ltimeout: bool,
}

unsafe fn io_flags_to_clock(flags: u32) -> clockid_t {
    match flags & IORING_TIMEOUT_CLOCK_MASK {
        IORING_TIMEOUT_BOOTTIME => CLOCK_BOOTTIME,
        IORING_TIMEOUT_REALTIME => CLOCK_REALTIME,
        _ => {
            WARN_ON_ONCE(1);
            CLOCK_MONOTONIC
        }
    }
}

unsafe fn io_parse_user_time(time: *mut ktime_t, arg: u64, flags: u32) -> i32 {
    let mut ts: timespec64 = core::mem::zeroed();
    if flags & IORING_TIMEOUT_IMMEDIATE_ARG != 0 {
        *time = ns_to_ktime(arg);
        if *time < 0 { return -EINVAL; }
    } else {
        if get_timespec64(&mut ts, u64_to_user_ptr(arg)) != 0 { return -EFAULT; }
        if ts.tv_sec < 0 || ts.tv_nsec < 0 { return -EINVAL; }
        *time = timespec64_to_ktime(ts);
    }
    if flags & IORING_TIMEOUT_ABS != 0 {
        *time = timens_ktime_to_host(io_flags_to_clock(flags), *time);
    }
    0
}

extern "C" {
    fn __io_disarm_linked_timeout(req: *mut io_kiocb, link: *mut io_kiocb) -> *mut io_kiocb;
}

#[inline]
unsafe fn io_is_timeout_noseq(req: *mut io_kiocb) -> bool {
    let timeout = io_kiocb_to_cmd::<io_timeout>(req);
    let data = (*req).async_data as *mut io_timeout_data;
    (*timeout).off == 0 || (*data).flags & IORING_TIMEOUT_MULTISHOT != 0
}

#[inline]
unsafe fn io_put_req(req: *mut io_kiocb) {
    if req_ref_put_and_test(req) { io_queue_next(req); io_free_req(req); }
}

#[inline]
unsafe fn io_timeout_finish(timeout: *mut io_timeout, data: *mut io_timeout_data) -> bool {
    if (*data).flags & IORING_TIMEOUT_MULTISHOT == 0 { return true; }
    if (*timeout).off == 0 || ((*timeout).repeats != 0 && { (*timeout).repeats -= 1; true }) { return false; }
    true
}

unsafe fn io_timeout_complete(tw_req: io_tw_req, tw: io_tw_token_t) {
    let req = tw_req.req;
    let timeout = io_kiocb_to_cmd::<io_timeout>(req);
    let data = (*req).async_data as *mut io_timeout_data;
    let ctx = (*req).ctx;
    if !io_timeout_finish(timeout, data) && io_req_post_cqe(req, -ETIME, IORING_CQE_F_MORE) {
        raw_spin_lock_irq(&mut (*ctx).timeout_lock);
        list_add(&mut (*timeout).list, (*ctx).timeout_list.prev);
        hrtimer_start(&mut (*data).timer, (*data).time, (*data).mode);
        raw_spin_unlock_irq(&mut (*ctx).timeout_lock);
        return;
    }
    io_req_task_complete(tw_req, tw);
}

unsafe fn io_flush_killed_timeouts(list: *mut list_head, err: i32) -> bool {
    if list_empty(list) { return false; }
    while !list_empty(list) {
        let timeout = list_first_entry::<io_timeout>(list);
        list_del_init(&mut (*timeout).list);
        let req = cmd_to_io_kiocb(timeout);
        if err != 0 { req_set_fail(req); }
        io_req_queue_tw_complete(req, err);
    }
    true
}

unsafe fn io_kill_timeout(req: *mut io_kiocb, list: *mut list_head) {
    let io = (*req).async_data as *mut io_timeout_data;
    if hrtimer_try_to_cancel(&mut (*io).timer) != -1 {
        let timeout = io_kiocb_to_cmd::<io_timeout>(req);
        atomic_set(&mut (*(*req).ctx).cq_timeouts, atomic_read(&(*req).ctx.cq_timeouts) + 1);
        list_move_tail(&mut (*timeout).list, list);
    }
}

pub unsafe fn io_flush_timeouts(ctx: *mut io_ring_ctx) {
    let mut list: list_head = LIST_HEAD_INIT();
    let seq = READ_ONCE((*ctx).cached_cq_tail) - atomic_read(&(*ctx).cq_timeouts);
    raw_spin_lock_irq(&mut (*ctx).timeout_lock);
    let mut pos = (*ctx).timeout_list.next;
    while pos != &mut (*ctx).timeout_list as *mut _ {
        let timeout = container_of!(pos, io_timeout, list);
        let req = cmd_to_io_kiocb(timeout);
        if io_is_timeout_noseq(req) { break; }
        let needed = (*timeout).target_seq - (*ctx).cq_last_tm_flush;
        let got = seq - (*ctx).cq_last_tm_flush;
        if got < needed { break; }
        let next = (*pos).next;
        io_kill_timeout(req, &mut list); pos = next;
    }
    (*ctx).cq_last_tm_flush = seq;
    raw_spin_unlock_irq(&mut (*ctx).timeout_lock);
    io_flush_killed_timeouts(&mut list, 0);
}

// The remaining routines retain the kernel's externally supplied list, timer,
// cancellation, and request-work helpers and mirror the original operations.
pub unsafe fn io_timeout_remove_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let tr = io_kiocb_to_cmd::<io_timeout_rem>(req);
    if (*req).flags & (REQ_F_FIXED_FILE | REQ_F_BUFFER_SELECT) != 0 || (*sqe).addr3 != 0 || (*sqe).__pad2[0] != 0 || (*sqe).buf_index != 0 || (*sqe).len != 0 || (*sqe).splice_fd_in != 0 { return -EINVAL; }
    (*tr).ltimeout = false; (*tr).addr = READ_ONCE((*sqe).addr); (*tr).flags = READ_ONCE((*sqe).timeout_flags);
    if (*tr).flags & IORING_TIMEOUT_UPDATE_MASK != 0 {
        if hweight32((*tr).flags & IORING_TIMEOUT_CLOCK_MASK) > 1 { return -EINVAL; }
        (*tr).ltimeout = (*tr).flags & IORING_LINK_TIMEOUT_UPDATE != 0;
        if (*tr).flags & !(IORING_TIMEOUT_UPDATE_MASK | IORING_TIMEOUT_ABS | IORING_TIMEOUT_IMMEDIATE_ARG) != 0 { return -EINVAL; }
        let ret = io_parse_user_time(&mut (*tr).time, READ_ONCE((*sqe).addr2), (*tr).flags); if ret != 0 { return ret; }
    } else if (*tr).flags != 0 { return -EINVAL; }
    0
}

#[inline]
unsafe fn io_translate_timeout_mode(flags: u32) -> hrtimer_mode { if flags & IORING_TIMEOUT_ABS != 0 { HRTIMER_MODE_ABS } else { HRTIMER_MODE_REL } }

pub unsafe fn io_timeout_remove(req: *mut io_kiocb, _issue_flags: u32) -> i32 {
    let tr = io_kiocb_to_cmd::<io_timeout_rem>(req); let ctx = (*req).ctx; let ret;
    if (*tr).flags & IORING_TIMEOUT_UPDATE == 0 {
        let mut cd = io_cancel_data { ctx, data: (*tr).addr };
        spin_lock(&mut (*ctx).completion_lock); ret = io_timeout_cancel(ctx, &mut cd); spin_unlock(&mut (*ctx).completion_lock);
    } else { let mode = io_translate_timeout_mode((*tr).flags); raw_spin_lock_irq(&mut (*ctx).timeout_lock); ret = if (*tr).ltimeout { io_linked_timeout_update(ctx, (*tr).addr, (*tr).time, mode) } else { io_timeout_update(ctx, (*tr).addr, (*tr).time, mode) }; raw_spin_unlock_irq(&mut (*ctx).timeout_lock); }
    if ret < 0 { req_set_fail(req); } io_req_set_res(req, ret, 0); IOU_COMPLETE
}

pub unsafe fn io_timeout_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 { __io_timeout_prep(req, sqe, false) }
pub unsafe fn io_link_timeout_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 { __io_timeout_prep(req, sqe, true) }

unsafe fn __io_timeout_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe, is_link: bool) -> i32 {
    let timeout = io_kiocb_to_cmd::<io_timeout>(req); let flags = READ_ONCE((*sqe).timeout_flags); let off = READ_ONCE((*sqe).off);
    if (*sqe).addr3 != 0 || (*sqe).__pad2[0] != 0 || (*sqe).buf_index != 0 || (*sqe).len != 1 || (*sqe).splice_fd_in != 0 || (off != 0 && is_link) { return -EINVAL; }
    if flags & !(IORING_TIMEOUT_ABS | IORING_TIMEOUT_CLOCK_MASK | IORING_TIMEOUT_ETIME_SUCCESS | IORING_TIMEOUT_MULTISHOT | IORING_TIMEOUT_IMMEDIATE_ARG) != 0 || hweight32(flags & IORING_TIMEOUT_CLOCK_MASK) > 1 || (flags & (IORING_TIMEOUT_MULTISHOT | IORING_TIMEOUT_ABS)) == (IORING_TIMEOUT_MULTISHOT | IORING_TIMEOUT_ABS) { return -EINVAL; }
    INIT_LIST_HEAD(&mut (*timeout).list); (*timeout).off = off; (*timeout).repeats = if flags & IORING_TIMEOUT_MULTISHOT != 0 { off } else { 0 };
    if req_has_async_data(req) { return -EFAULT; } let data = io_uring_alloc_async_data(core::ptr::null_mut(), req); if data.is_null() { return -ENOMEM; }
    (*data).req = req; (*data).flags = flags; let ret = io_parse_user_time(&mut (*data).time, READ_ONCE((*sqe).addr), flags); if ret != 0 { return ret; } (*data).mode = io_translate_timeout_mode(flags); (*timeout).head = if is_link { (*(*req).ctx).submit_state.link.last } else { core::ptr::null_mut() }; ret
}

pub unsafe fn io_timeout(req: *mut io_kiocb, _issue_flags: u32) -> i32 { let t=io_kiocb_to_cmd::<io_timeout>(req); let d=(*req).async_data as *mut io_timeout_data; let c=(*req).ctx; raw_spin_lock_irq(&mut (*c).timeout_lock); list_add(&mut (*t).list, (*c).timeout_list.prev); hrtimer_start(&mut (*d).timer, (*d).time, (*d).mode); raw_spin_unlock_irq(&mut (*c).timeout_lock); IOU_ISSUE_SKIP_COMPLETE }

pub unsafe fn io_queue_linked_timeout(req: *mut io_kiocb) { io_put_req(req); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
