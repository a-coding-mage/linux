// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_int, c_uint};

// Opaque types supplied by dependent translation units.
#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    _private: [u8; 0],
}

extern "C" {
    pub fn io_sfr_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
    pub fn io_sync_file_range(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;

    pub fn io_fsync_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
    pub fn io_fsync(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;

    pub fn io_fallocate(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
    pub fn io_fallocate_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
