// SPDX-License-Identifier: GPL-2.0

// Opaque types supplied by the surrounding io_uring implementation.
pub enum io_kiocb {}
pub enum io_uring_sqe {}

unsafe extern "C" {
    pub fn io_statx_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_statx(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn io_statx_cleanup(req: *mut io_kiocb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
