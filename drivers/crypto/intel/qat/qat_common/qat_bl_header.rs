/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2014 - 2022 Intel Corporation */

// Linux kernel dependencies supplied by the surrounding translation unit.

pub const QAT_MAX_BUFF_DESC: usize = 4;

#[repr(C, packed)]
pub struct qat_alg_buf {
    pub len: u32,
    pub resrvd: u32,
    pub addr: u64,
}

#[repr(C, packed)]
pub struct qat_alg_buf_list_hdr {
    pub resrvd: u64,
    pub num_bufs: u32,
    pub num_mapped_bufs: u32,
}

#[repr(C, packed)]
pub struct qat_alg_buf_list {
    // New members must be added within the __struct_group() macro below.
    pub hdr: qat_alg_buf_list_hdr,
    pub buffers: [qat_alg_buf; 0],
}

// Equivalent to the C static assertion that buffers starts immediately after
// qat_alg_buf_list_hdr; the flexible array member has zero size in Rust.

#[repr(C, packed, align(64))]
pub struct qat_alg_fixed_buf_list {
    pub sgl_hdr: qat_alg_buf_list_hdr,
    pub descriptors: [qat_alg_buf; QAT_MAX_BUFF_DESC],
}

#[repr(C)]
pub struct qat_request_buffs {
    pub bl: *mut qat_alg_buf_list,
    pub blp: dma_addr_t,
    pub blout: *mut qat_alg_buf_list,
    pub bloutp: dma_addr_t,
    pub sz: usize,
    pub sz_out: usize,
    pub sgl_src_valid: bool,
    pub sgl_dst_valid: bool,
    pub sgl_src: qat_alg_fixed_buf_list,
    pub sgl_dst: qat_alg_fixed_buf_list,
}

#[repr(C)]
pub struct qat_sgl_to_bufl_params {
    pub extra_dst_buff: dma_addr_t,
    pub sz_extra_dst_buff: usize,
    pub sskip: c_uint,
    pub dskip: c_uint,
}

extern "C" {
    pub fn qat_bl_free_bufl(
        accel_dev: *mut adf_accel_dev,
        buf: *mut qat_request_buffs,
    );
    pub fn qat_bl_sgl_to_bufl(
        accel_dev: *mut adf_accel_dev,
        sgl: *mut scatterlist,
        sglout: *mut scatterlist,
        buf: *mut qat_request_buffs,
        params: *mut qat_sgl_to_bufl_params,
        flags: gfp_t,
    ) -> c_int;
}

#[inline]
pub unsafe fn qat_algs_alloc_flags(req: *mut crypto_async_request) -> gfp_t {
    (*req).flags & if CRYPTO_TFM_REQ_MAY_SLEEP != 0 {
        GFP_KERNEL
    } else {
        GFP_ATOMIC
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
