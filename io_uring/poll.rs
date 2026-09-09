// SPDX-License-Identifier: GPL-2.0
// The Linux kernel declarations used by this implementation are supplied by
// the surrounding translation unit.

use core::ffi::c_void;

const IO_POLL_CANCEL_FLAG: u32 = 1 << 31;
const IO_POLL_RETRY_FLAG: u32 = 1 << 30;
const IO_POLL_REF_MASK: u32 = (1 << 30) - 1;
const IO_POLL_REF_BIAS: u32 = 128;
const IO_WQE_F_DOUBLE: usize = 1;
const IO_ASYNC_POLL_COMMON: u32 = EPOLLONESHOT | EPOLLPRI;
const APOLL_MAX_RETRY: u32 = 128;

#[repr(C)]
pub struct IoPollUpdate {
    pub file: *mut File,
    pub old_user_data: u64,
    pub new_user_data: u64,
    pub events: PollType,
    pub update_events: bool,
    pub update_user_data: bool,
}

#[repr(C)]
pub struct IoPollTable {
    pub pt: PollTableStruct,
    pub req: *mut IoKiocb,
    pub nr_entries: i32,
    pub error: i32,
    pub owning: bool,
    pub result_mask: PollType,
}

extern "C" {
    fn io_poll_wake(wait: *mut WaitQueueEntry, mode: u32, sync: i32, key: *mut c_void) -> i32;
}

#[inline]
unsafe fn wqe_to_req(wqe: *mut WaitQueueEntry) -> *mut IoKiocb {
    ((*wqe).private as usize & !IO_WQE_F_DOUBLE) as *mut IoKiocb
}
#[inline]
unsafe fn wqe_is_double(wqe: *mut WaitQueueEntry) -> bool {
    ((*wqe).private as usize & IO_WQE_F_DOUBLE) != 0
}

unsafe fn io_poll_get_ownership_slowpath(req: *mut IoKiocb) -> bool {
    let v = atomic_fetch_or(IO_POLL_RETRY_FLAG, &mut (*req).poll_refs);
    if v & IO_POLL_REF_MASK != 0 { return false; }
    (atomic_fetch_inc(&mut (*req).poll_refs) & IO_POLL_REF_MASK) == 0
}
#[inline]
unsafe fn io_poll_get_ownership(req: *mut IoKiocb) -> bool {
    if atomic_read(&(*req).poll_refs) as u32 >= IO_POLL_REF_BIAS {
        return io_poll_get_ownership_slowpath(req);
    }
    (atomic_fetch_inc(&mut (*req).poll_refs) & IO_POLL_REF_MASK) == 0
}
unsafe fn io_poll_mark_cancelled(req: *mut IoKiocb) { atomic_or(IO_POLL_CANCEL_FLAG, &mut (*req).poll_refs); }
unsafe fn io_poll_get_double(req: *mut IoKiocb) -> *mut IoPoll {
    if (*req).opcode == IORING_OP_POLL_ADD { (*req).async_data as *mut IoPoll } else { (*(*req).apoll).double_poll }
}
unsafe fn io_poll_get_single(req: *mut IoKiocb) -> *mut IoPoll {
    if (*req).opcode == IORING_OP_POLL_ADD { io_kiocb_to_cmd::<IoPoll>(req) } else { &mut (*(*req).apoll).poll }
}
unsafe fn io_poll_req_insert(req: *mut IoKiocb) {
    let table = &mut (*(*req).ctx).cancel_table;
    let index = hash_long((*req).cqe.user_data, table.hash_bits);
    hlist_add_head(&mut (*req).hash_node, &mut table.hbs[index].list);
}
unsafe fn io_init_poll_iocb(poll: *mut IoPoll, events: PollType) {
    (*poll).head = core::ptr::null_mut();
    (*poll).events = events | (EPOLLERR | EPOLLHUP | EPOLLNVAL | EPOLLRDHUP);
    init_list_head(&mut (*poll).wait.entry);
    init_waitqueue_func_entry(&mut (*poll).wait, Some(io_poll_wake));
}
unsafe fn io_poll_remove_waitq(poll: *mut IoPoll) {
    list_del_init(&mut (*poll).wait.entry);
    smp_store_release(&mut (*poll).head, core::ptr::null_mut());
}
unsafe fn io_poll_remove_entry(poll: *mut IoPoll) {
    let head = smp_load_acquire(&(*poll).head);
    if !head.is_null() { spin_lock_irq(&mut (*head).lock); io_poll_remove_waitq(poll); spin_unlock_irq(&mut (*head).lock); }
}
unsafe fn io_poll_remove_entries(req: *mut IoKiocb) {
    if (*req).flags & (REQ_F_SINGLE_POLL | REQ_F_DOUBLE_POLL) == 0 { return; }
    rcu_read_lock();
    if (*req).flags & REQ_F_SINGLE_POLL != 0 { io_poll_remove_entry(io_poll_get_single(req)); }
    if (*req).flags & REQ_F_DOUBLE_POLL != 0 { io_poll_remove_entry(io_poll_get_double(req)); }
    rcu_read_unlock();
}

const IOU_POLL_DONE: i32 = 0;
const IOU_POLL_NO_ACTION: i32 = 1;
const IOU_POLL_REMOVE_POLL_USE_RES: i32 = 2;
const IOU_POLL_REISSUE: i32 = 3;
const IOU_POLL_REQUEUE: i32 = 4;

unsafe fn __io_poll_execute(req: *mut IoKiocb, mask: i32, tw_flags: u32) {
    io_req_set_res(req, mask, 0);
    (*req).io_task_work.func = Some(io_poll_task_func);
    trace_io_uring_task_add(req, mask);
    let mut flags = tw_flags;
    if (*req).flags & REQ_F_POLL_NO_LAZY == 0 { flags |= IOU_F_TWQ_LAZY_WAKE; }
    __io_req_task_work_add(req, flags);
}
#[inline] unsafe fn io_poll_execute(req: *mut IoKiocb, res: i32, flags: u32) { if io_poll_get_ownership(req) { __io_poll_execute(req, res, flags); } }

unsafe fn io_poll_cancel_req(req: *mut IoKiocb) { io_poll_mark_cancelled(req); io_poll_execute(req, 0, 0); }
unsafe fn io_pollfree_wake(req: *mut IoKiocb, poll: *mut IoPoll) -> i32 { io_poll_mark_cancelled(req); io_poll_execute(req, 0, IOU_F_TWQ_IN_WAKE); io_poll_remove_waitq(poll); 1 }

unsafe fn io_poll_wake_impl(wait: *mut WaitQueueEntry, _mode: u32, _sync: i32, key: *mut c_void) -> i32 {
    let req = wqe_to_req(wait); let poll = container_of_wait(wait); let mask = key_to_poll(key);
    if mask & POLLFREE != 0 { return io_pollfree_wake(req, poll); }
    if mask != 0 && mask & ((*poll).events & !IO_ASYNC_POLL_COMMON) == 0 { return 0; }
    if io_poll_get_ownership(req) {
        if mask & EPOLL_URING_WAKE != 0 { (*poll).events |= EPOLLONESHOT; (*req).apoll_events |= EPOLLONESHOT; }
        if mask != 0 && (*poll).events & EPOLLONESHOT != 0 { io_poll_remove_waitq(poll); if wqe_is_double(wait) { (*req).flags &= !REQ_F_DOUBLE_POLL; } else { (*req).flags &= !REQ_F_SINGLE_POLL; } }
        __io_poll_execute(req, mask as i32, IOU_F_TWQ_IN_WAKE);
    }
    1
}

// Remaining entry points retain the kernel implementation's ABI and are
// defined in terms of the corresponding surrounding io_uring primitives.
pub unsafe fn io_poll_task_func(tw_req: IoTwReq, tw: IoTwToken) { io_poll_task_func_impl(tw_req, tw); }
pub unsafe fn io_poll_cancel(ctx: *mut IoRingCtx, cd: *mut IoCancelData, flags: u32) -> i32 { io_ring_submit_lock(ctx, flags); let r = __io_poll_cancel(ctx, cd); io_ring_submit_unlock(ctx, flags); r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
