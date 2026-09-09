// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by the translated bpf_filter header.

#[repr(C)]
pub struct io_ring_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_bpf_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn __io_close_fixed(
        ctx: *mut io_ring_ctx,
        issue_flags: u32,
        offset: u32,
    ) -> i32;

    pub fn io_openat_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_openat(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_open_cleanup(req: *mut io_kiocb);
    pub fn io_openat_bpf_populate(
        bctx: *mut io_uring_bpf_ctx,
        req: *mut io_kiocb,
    );

    pub fn io_openat2_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_openat2(req: *mut io_kiocb, issue_flags: u32) -> i32;

    pub fn io_close_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_close(req: *mut io_kiocb, issue_flags: u32) -> i32;

    pub fn io_pipe_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_pipe(req: *mut io_kiocb, issue_flags: u32) -> i32;

    pub fn io_install_fixed_fd_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_install_fixed_fd(req: *mut io_kiocb, issue_flags: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
