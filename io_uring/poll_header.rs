// SPDX-License-Identifier: GPL-2.0

// Dependency equivalent of: #include <linux/io_uring_types.h>

pub const IO_POLL_ALLOC_CACHE_MAX: i32 = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IoApoll {
    IO_APOLL_OK = 0,
    IO_APOLL_ABORTED,
    IO_APOLL_READY,
}

#[repr(C)]
pub struct io_poll {
    pub file: *mut file,
    pub head: *mut wait_queue_head,
    pub events: __poll_t,
    pub retries: ::core::ffi::c_int,
    pub wait: wait_queue_entry,
}

#[repr(C)]
pub struct async_poll {
    pub poll: io_poll,
    pub double_poll: *mut io_poll,
}

/*
 * Must only be called inside issue_flags & IO_URING_F_MULTISHOT, or
 * potentially other cases where we already "own" this poll request.
 */
#[inline]
pub unsafe fn io_poll_multishot_retry(req: *mut io_kiocb) {
    atomic_inc(&mut (*req).poll_refs);
}

unsafe extern "C" {
    pub fn io_poll_add_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::core::ffi::c_int;
    pub fn io_poll_add(req: *mut io_kiocb, issue_flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;

    pub fn io_poll_remove_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::core::ffi::c_int;
    pub fn io_poll_remove(
        req: *mut io_kiocb,
        issue_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn io_poll_cancel(
        ctx: *mut io_ring_ctx,
        cd: *mut io_cancel_data,
        issue_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn io_arm_apoll(
        req: *mut io_kiocb,
        issue_flags: ::core::ffi::c_uint,
        mask: __poll_t,
    ) -> ::core::ffi::c_int;
    pub fn io_arm_poll_handler(
        req: *mut io_kiocb,
        issue_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn io_poll_remove_all(
        ctx: *mut io_ring_ctx,
        tctx: *mut io_uring_task,
        cancel_all: bool,
    ) -> bool;

    pub fn io_poll_task_func(tw_req: io_tw_req, tw: io_tw_token_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
