/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/linux/bsg.h>.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct bsg_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct request_queue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_cmd {
    _private: [u8; 0],
}

// Supplied by <uapi/linux/bsg.h>.
#[repr(C)]
pub struct sg_io_v4 {
    _private: [u8; 0],
}

pub type bsg_sg_io_fn = unsafe extern "C" fn(
    *mut request_queue,
    *mut sg_io_v4,
    bool,
    u32,
) -> c_int;

pub type bsg_uring_cmd_fn = unsafe extern "C" fn(
    *mut request_queue,
    *mut io_uring_cmd,
    u32,
    bool,
) -> c_int;

unsafe extern "C" {
    pub fn bsg_register_queue(
        q: *mut request_queue,
        parent: *mut device,
        name: *const c_char,
        sg_io_fn: Option<bsg_sg_io_fn>,
        uring_cmd_fn: Option<bsg_uring_cmd_fn>,
    ) -> *mut bsg_device;

    pub fn bsg_unregister_queue(bcd: *mut bsg_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
