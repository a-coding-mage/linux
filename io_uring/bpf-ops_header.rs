// SPDX-License-Identifier: GPL-2.0

// Dependency: linux/io_uring_types.h

pub const IOU_REGION_MEM: i32 = 0;
pub const IOU_REGION_CQ: i32 = 1;
pub const IOU_REGION_SQ: i32 = 2;

#[repr(C)]
pub struct io_uring_bpf_ops {
    pub loop_step:
        Option<unsafe extern "C" fn(*mut iou_ctx, *mut iou_loop_params) -> ::core::ffi::c_int>,
    pub ring_fd: u32,
    pub priv_: *mut ::core::ffi::c_void,
}

// Build-time condition from CONFIG_IO_URING_BPF_OPS is represented as a Cargo feature.
#[cfg(feature = "CONFIG_IO_URING_BPF_OPS")]
extern "C" {
    pub fn io_unregister_bpf_ops(ctx: *mut io_ring_ctx);
}

#[cfg(not(feature = "CONFIG_IO_URING_BPF_OPS"))]
#[inline]
pub unsafe fn io_unregister_bpf_ops(_ctx: *mut io_ring_ctx) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
