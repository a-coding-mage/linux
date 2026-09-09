// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn io_xattr_cleanup(req: *mut io_kiocb);

    pub fn io_fsetxattr_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::std::os::raw::c_int;
    pub fn io_fsetxattr(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;

    pub fn io_setxattr_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::std::os::raw::c_int;
    pub fn io_setxattr(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;

    pub fn io_fgetxattr_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::std::os::raw::c_int;
    pub fn io_fgetxattr(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;

    pub fn io_getxattr_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::std::os::raw::c_int;
    pub fn io_getxattr(req: *mut io_kiocb, issue_flags: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
