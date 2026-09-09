/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

#[repr(C)]
pub struct io_ring_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn io_eventfd_register(
        ctx: *mut io_ring_ctx,
        arg: *mut c_void, // void __user *arg
        eventfd_async: u32,
    ) -> i32;

    pub fn io_eventfd_unregister(ctx: *mut io_ring_ctx) -> i32;

    pub fn io_eventfd_signal(ctx: *mut io_ring_ctx, cqe_event: bool, defer: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
