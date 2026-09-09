// SPDX-License-Identifier: GPL-2.0

// Conditional compilation in the original header: defined(CONFIG_EPOLL).
unsafe extern "C" {
    pub fn io_epoll_ctl_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::core::ffi::c_int;
    pub fn io_epoll_ctl(
        req: *mut io_kiocb,
        issue_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn io_epoll_wait_prep(
        req: *mut io_kiocb,
        sqe: *const io_uring_sqe,
    ) -> ::core::ffi::c_int;
    pub fn io_epoll_wait(
        req: *mut io_kiocb,
        issue_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
