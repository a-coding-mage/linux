// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by the surrounding kernel translation: ../kernel/exit.h

#[repr(C)]
pub struct io_waitid_async {
    pub req: *mut io_kiocb,
    pub wo: wait_opts,
}

unsafe extern "C" {
    pub fn io_waitid_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::core::ffi::c_int;
    pub fn io_waitid(req: *mut io_kiocb, issue_flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn io_waitid_cancel(
        ctx: *mut io_ring_ctx,
        cd: *mut io_cancel_data,
        issue_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn io_waitid_remove_all(
        ctx: *mut io_ring_ctx,
        tctx: *mut io_uring_task,
        cancel_all: bool,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
