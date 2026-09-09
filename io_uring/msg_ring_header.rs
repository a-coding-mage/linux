// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_int, c_uint};

pub enum io_uring_sqe {}
pub enum io_kiocb {}

unsafe extern "C" {
    pub fn io_uring_sync_msg_ring(sqe: *mut io_uring_sqe) -> c_int;
    pub fn io_msg_ring_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
    pub fn io_msg_ring(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
    pub fn io_msg_ring_cleanup(req: *mut io_kiocb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
