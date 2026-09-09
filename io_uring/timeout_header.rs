// SPDX-License-Identifier: GPL-2.0

// External types supplied by other translation units:
// struct io_kiocb, struct hrtimer, ktime_t, enum hrtimer_mode,
// struct io_ring_ctx, struct io_cancel_data, struct io_uring_task,
// and struct io_uring_sqe.

#[repr(C)]
pub struct io_timeout_data {
    pub req: *mut io_kiocb,
    pub timer: hrtimer,
    pub time: ktime_t,
    pub mode: hrtimer_mode,
    pub flags: u32,
}

unsafe extern "C" {
    pub fn io_flush_timeouts(ctx: *mut io_ring_ctx);
    pub fn io_timeout_cancel(
        ctx: *mut io_ring_ctx,
        cd: *mut io_cancel_data,
    ) -> i32;
    pub fn io_kill_timeouts(
        ctx: *mut io_ring_ctx,
        tctx: *mut io_uring_task,
        cancel_all: bool,
    ) -> bool;
    pub fn io_queue_linked_timeout(req: *mut io_kiocb);
    pub fn io_disarm_next(req: *mut io_kiocb);

    pub fn io_timeout_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_link_timeout_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_timeout(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_timeout_remove_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_timeout_remove(req: *mut io_kiocb, issue_flags: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
