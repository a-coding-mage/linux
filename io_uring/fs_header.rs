// SPDX-License-Identifier: GPL-2.0

// Opaque types supplied by other translation units.
#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    _private: [u8; 0],
}

extern "C" {
    pub fn io_renameat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;
    pub fn io_renameat(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_renameat_cleanup(req: *mut io_kiocb);

    pub fn io_unlinkat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;
    pub fn io_unlinkat(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_unlinkat_cleanup(req: *mut io_kiocb);

    pub fn io_mkdirat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;
    pub fn io_mkdirat(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_mkdirat_cleanup(req: *mut io_kiocb);

    pub fn io_symlinkat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;
    pub fn io_symlinkat(req: *mut io_kiocb, issue_flags: u32) -> i32;

    pub fn io_linkat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;
    pub fn io_linkat(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_link_cleanup(req: *mut io_kiocb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
