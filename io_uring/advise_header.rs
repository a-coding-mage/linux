// SPDX-License-Identifier: GPL-2.0

// Opaque types supplied by the surrounding dependency set.
#[repr(C)]
pub struct io_kiocb {}

#[repr(C)]
pub struct io_uring_sqe {}

extern "C" {
    pub fn io_madvise_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_madvise(req: *mut io_kiocb, issue_flags: u32) -> i32;

    pub fn io_fadvise_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> i32;
    pub fn io_fadvise(req: *mut io_kiocb, issue_flags: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
