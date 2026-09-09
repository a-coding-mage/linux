// SPDX-License-Identifier: GPL-2.0

// Dependencies corresponding to the original Linux io_uring headers are
// supplied externally.

use std::ffi::c_void;

#[repr(C)]
pub struct io_async_cmd {
    pub vec: iou_vec,
    pub sqes: [io_uring_sqe; 2],
}

unsafe extern "C" {
    pub fn io_uring_cmd(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_uring_cmd_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;
    pub fn io_uring_cmd_sqe_copy(req: *mut io_kiocb);
    pub fn io_uring_cmd_cleanup(req: *mut io_kiocb);

    pub fn io_uring_try_cancel_uring_cmd(
        ctx: *mut io_ring_ctx,
        tctx: *mut io_uring_task,
        cancel_all: bool,
    ) -> bool;

    pub fn io_uring_cmd_post_mshot_cqe32(
        cmd: *mut io_uring_cmd,
        issue_flags: u32,
        cqe: *mut io_uring_cqe,
    ) -> bool;

    pub fn io_cmd_cache_free(entry: *const c_void);

    pub fn io_cmd_poll_multishot(
        cmd: *mut io_uring_cmd,
        issue_flags: u32,
        mask: __poll_t,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
