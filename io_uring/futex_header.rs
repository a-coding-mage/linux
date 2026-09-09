// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by the corresponding Rust translation of `cancel.h`.

extern "C" {
    pub fn io_futex_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> ::std::os::raw::c_int;
    pub fn io_futex_wait_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::std::os::raw::c_int;
    pub fn io_futexv_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::std::os::raw::c_int;
    pub fn io_futex_wait(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;
    pub fn io_futexv_wait(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;
    pub fn io_futex_wake(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;
}

// `CONFIG_FUTEX` is a build-time condition supplied by the surrounding project.
#[cfg(CONFIG_FUTEX)]
extern "C" {
    pub fn io_futex_cancel(
        ctx: *mut io_ring_ctx,
        cd: *mut io_cancel_data,
        issue_flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn io_futex_remove_all(
        ctx: *mut io_ring_ctx,
        tctx: *mut io_uring_task,
        cancel_all: bool,
    ) -> bool;
    pub fn io_futex_cache_init(ctx: *mut io_ring_ctx) -> bool;
    pub fn io_futex_cache_free(ctx: *mut io_ring_ctx);
}

#[cfg(not(CONFIG_FUTEX))]
#[inline]
pub unsafe fn io_futex_cancel(
    _ctx: *mut io_ring_ctx,
    _cd: *mut io_cancel_data,
    _issue_flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(CONFIG_FUTEX))]
#[inline]
pub unsafe fn io_futex_remove_all(
    _ctx: *mut io_ring_ctx,
    _tctx: *mut io_uring_task,
    _cancel_all: bool,
) -> bool {
    false
}

#[cfg(not(CONFIG_FUTEX))]
#[inline]
pub unsafe fn io_futex_cache_init(_ctx: *mut io_ring_ctx) -> bool {
    false
}

#[cfg(not(CONFIG_FUTEX))]
#[inline]
pub unsafe fn io_futex_cache_free(_ctx: *mut io_ring_ctx) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
