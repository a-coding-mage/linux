// SPDX-License-Identifier: GPL-2.0

// External kernel types supplied by the corresponding C headers:
// linux/io_uring_types.h, linux/pagemap.h, and linux/uio.h.

#[repr(C)]
pub struct io_meta_state {
    pub seed: u32,
    pub iter_meta: iov_iter_state,
}

#[repr(C)]
pub union io_async_rw__clear__wait {
    pub wpq: std::mem::ManuallyDrop<wait_page_queue>,
    pub meta: std::mem::ManuallyDrop<io_async_rw__clear__meta>,
}

#[repr(C)]
pub struct io_async_rw__clear__meta {
    pub meta: uio_meta,
    pub meta_state: io_meta_state,
}

#[repr(C)]
pub struct io_async_rw {
    pub vec: iou_vec,
    pub bytes_done: usize,
    pub iter: iov_iter,
    pub iter_state: iov_iter_state,
    pub fast_iov: iovec,
    pub buf_group: u32,
    pub wait: io_async_rw__clear__wait,
}

extern "C" {
    pub fn io_prep_read_fixed(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_prep_write_fixed(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_prep_readv_fixed(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_prep_writev_fixed(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_prep_readv(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_prep_writev(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_prep_read(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_prep_write(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_read(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_write(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_read_fixed(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_write_fixed(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_readv_writev_cleanup(req: *mut io_kiocb);
    pub fn io_rw_fail(req: *mut io_kiocb);
    pub fn io_req_rw_complete(tw_req: io_tw_req, tw: io_tw_token_t);
    pub fn io_read_mshot_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_read_mshot(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_rw_cache_free(entry: *const std::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
