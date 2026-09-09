// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn io_tee_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
    pub fn io_tee(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;

    pub fn io_splice_cleanup(req: *mut io_kiocb);
    pub fn io_splice_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
    pub fn io_splice(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
