// SPDX-License-Identifier: GPL-2.0
// Original header guard: IORING_REGISTER_H

#[repr(C)]
pub struct io_ring_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn io_eventfd_unregister(ctx: *mut io_ring_ctx) -> ::std::ffi::c_int;
    pub fn io_unregister_personality(
        ctx: *mut io_ring_ctx,
        id: ::std::ffi::c_uint,
    ) -> ::std::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
