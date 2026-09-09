// SPDX-License-Identifier: GPL-2.0

// Opaque types supplied by the surrounding translation unit.
#[repr(C)]
pub struct io_kiocb;

#[repr(C)]
pub struct io_uring_sqe;

unsafe extern "C" {
    pub fn io_ftruncate_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::std::os::raw::c_int;

    pub fn io_ftruncate(req: *mut io_kiocb, issue_flags: u32) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
