/* SPDX-License-Identifier: GPL-2.0-or-later */

// Opaque declaration supplied by another translation unit.
#[repr(C)]
pub struct io_uring_cmd {
    _private: [u8; 0],
}

#[cfg(CONFIG_IO_URING)]
extern "C" {
    pub fn io_uring_cmd_sock(
        cmd: *mut io_uring_cmd,
        issue_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_IO_URING))]
#[inline]
pub unsafe fn io_uring_cmd_sock(
    _cmd: *mut io_uring_cmd,
    _issue_flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
