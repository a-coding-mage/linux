// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

#[repr(C)]
pub struct io_cancel_data {
    pub ctx: *mut io_ring_ctx,
    pub data: u64,
    pub file: *mut file,
    pub opcode: u8,
    pub flags: u32,
    pub seq: i32,
}

unsafe extern "C" {
    pub fn io_async_cancel_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_async_cancel(req: *mut io_kiocb, issue_flags: u32) -> i32;

    pub fn io_try_cancel(
        tctx: *mut io_uring_task,
        cd: *mut io_cancel_data,
        issue_flags: u32,
    ) -> i32;

    pub fn io_sync_cancel(ctx: *mut io_ring_ctx, arg: *mut c_void) -> i32;
    pub fn io_cancel_req_match(req: *mut io_kiocb, cd: *mut io_cancel_data) -> bool;
    pub fn io_match_task_safe(
        head: *mut io_kiocb,
        tctx: *mut io_uring_task,
        cancel_all: bool,
    ) -> bool;

    pub fn io_cancel_remove_all(
        ctx: *mut io_ring_ctx,
        tctx: *mut io_uring_task,
        list: *mut hlist_head,
        cancel_all: bool,
        cancel: Option<unsafe extern "C" fn(*mut io_kiocb) -> bool>,
    ) -> bool;
    pub fn io_cancel_remove(
        ctx: *mut io_ring_ctx,
        cd: *mut io_cancel_data,
        issue_flags: u32,
        list: *mut hlist_head,
        cancel: Option<unsafe extern "C" fn(*mut io_kiocb) -> bool>,
    ) -> i32;
    pub fn io_uring_try_cancel_requests(
        ctx: *mut io_ring_ctx,
        tctx: *mut io_uring_task,
        cancel_all: bool,
        is_sqpoll_thread: bool,
    ) -> bool;
    pub fn io_uring_cancel_generic(cancel_all: bool, sqd: *mut io_sq_data);
    pub fn io_cancel_ctx_cb(work: *mut io_wq_work, data: *mut c_void) -> bool;
}

#[inline]
pub unsafe fn io_cancel_match_sequence(req: *mut io_kiocb, sequence: i32) -> bool {
    if (*req).cancel_seq_set && sequence == (*req).work.cancel_seq {
        return true;
    }

    (*req).cancel_seq_set = true;
    (*req).work.cancel_seq = sequence;
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
