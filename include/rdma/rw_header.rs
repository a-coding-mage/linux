/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 HGST, a Western Digital Company.
 */

// Dependencies corresponding to the original Linux/RDMA includes are supplied
// by other translation units.

#[repr(C)]
pub struct rdma_rw_ctx {
    /* number of RDMA READ/WRITE WRs (not counting MR WRs) */
    pub nr_ops: u32,

    /* tag for the union below: */
    pub type_: u8,

    pub data: rdma_rw_ctx_data,
}

#[repr(C)]
pub union rdma_rw_ctx_data {
    /* for mapping a single SGE: */
    pub single: rdma_rw_ctx_single,

    /* for mapping of multiple SGEs: */
    pub map: rdma_rw_ctx_map,

    /* for IOVA-based mapping of bvecs into contiguous DMA range: */
    pub iova: rdma_rw_ctx_iova,

    /* for registering multiple WRs: */
    pub reg: *mut rdma_rw_reg_ctx,
}

#[repr(C)]
pub struct rdma_rw_ctx_single {
    pub sge: ib_sge,
    pub wr: ib_rdma_wr,
}

#[repr(C)]
pub struct rdma_rw_ctx_map {
    pub sges: *mut ib_sge,
    pub wrs: *mut ib_rdma_wr,
}

#[repr(C)]
pub struct rdma_rw_ctx_iova {
    pub state: dma_iova_state,
    pub sge: ib_sge,
    pub wr: ib_rdma_wr,
    pub mapped_len: usize,
}

#[repr(C)]
pub struct rdma_rw_reg_ctx {
    pub sge: ib_sge,
    pub wr: ib_rdma_wr,
    pub reg_wr: ib_reg_wr,
    pub inv_wr: ib_send_wr,
    pub mr: *mut ib_mr,
    pub sgt: sg_table,
}

pub struct bio_vec;

extern "C" {
    pub fn rdma_rw_ctx_init(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        sg: *mut scatterlist,
        sg_cnt: u32,
        sg_offset: u32,
        remote_addr: u64,
        rkey: u32,
        dir: dma_data_direction,
    ) -> i32;

    pub fn rdma_rw_ctx_destroy(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        sg: *mut scatterlist,
        sg_cnt: u32,
        dir: dma_data_direction,
    );

    pub fn rdma_rw_ctx_init_bvec(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        bvecs: *const bio_vec,
        nr_bvec: u32,
        iter: bvec_iter,
        remote_addr: u64,
        rkey: u32,
        dir: dma_data_direction,
    ) -> i32;

    pub fn rdma_rw_ctx_destroy_bvec(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        bvecs: *const bio_vec,
        nr_bvec: u32,
        dir: dma_data_direction,
    );

    pub fn rdma_rw_ctx_signature_init(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        sg: *mut scatterlist,
        sg_cnt: u32,
        prot_sg: *mut scatterlist,
        prot_sg_cnt: u32,
        sig_attrs: *mut ib_sig_attrs,
        remote_addr: u64,
        rkey: u32,
        dir: dma_data_direction,
    ) -> i32;

    pub fn rdma_rw_ctx_destroy_signature(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        sg: *mut scatterlist,
        sg_cnt: u32,
        prot_sg: *mut scatterlist,
        prot_sg_cnt: u32,
        dir: dma_data_direction,
    );

    pub fn rdma_rw_ctx_wrs(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        cqe: *mut ib_cqe,
        chain_wr: *mut ib_send_wr,
    ) -> *mut ib_send_wr;

    pub fn rdma_rw_ctx_post(
        ctx: *mut rdma_rw_ctx,
        qp: *mut ib_qp,
        port_num: u32,
        cqe: *mut ib_cqe,
        chain_wr: *mut ib_send_wr,
    ) -> i32;

    pub fn rdma_rw_mr_factor(
        device: *mut ib_device,
        port_num: u32,
        maxpages: u32,
    ) -> u32;

    pub fn rdma_rw_max_send_wr(
        dev: *mut ib_device,
        port_num: u32,
        max_rdma_ctxs: u32,
        create_flags: u32,
    ) -> u32;

    pub fn rdma_rw_init_qp(dev: *mut ib_device, attr: *mut ib_qp_init_attr);
    pub fn rdma_rw_init_mrs(qp: *mut ib_qp, attr: *mut ib_qp_init_attr) -> i32;
    pub fn rdma_rw_cleanup_mrs(qp: *mut ib_qp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
