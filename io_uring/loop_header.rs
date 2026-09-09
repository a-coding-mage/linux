// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by the Linux io_uring types headers:
// #include <linux/io_uring_types.h>

#[repr(C)]
pub struct iou_loop_params {
    /*
     * The CQE index to wait for. Only serves as a hint and can still be
     * woken up earlier.
     */
    pub cq_wait_idx: __u32,
}

pub const IOU_LOOP_CONTINUE: i32 = 0;
pub const IOU_LOOP_STOP: i32 = 1;

#[inline]
pub unsafe fn io_has_loop_ops(ctx: *mut io_ring_ctx) -> bool {
    // C's data_race() annotation does not alter the value returned here.
    (*ctx).loop_step
}

unsafe extern "C" {
    pub fn io_run_loop(ctx: *mut io_ring_ctx) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn io_loop_mangle_ctx(ctx: *mut io_ring_ctx) -> *mut iou_ctx {
    ctx as *mut iou_ctx
}

#[inline]
pub unsafe fn io_loop_demangle_ctx(ctx: *mut iou_ctx) -> *mut io_ring_ctx {
    ctx as *mut io_ring_ctx
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
