/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB */
/*
 * Copyright (c) 2006 - 2021 Intel Corporation.  All rights reserved.
 * Copyright (c) 2005 Topspin Communications.  All rights reserved.
 * Copyright (c) 2005 Cisco Systems.  All rights reserved.
 * Copyright (c) 2005 Open Grid Computing, Inc. All rights reserved.
 */

// irdma must support legacy GEN_1 i40iw kernel
// and user-space whose last ABI ver is 5
pub const IRDMA_ABI_VER: u32 = 5;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum irdma_memreg_type {
    IRDMA_MEMREG_TYPE_MEM = 0,
    IRDMA_MEMREG_TYPE_QP = 1,
    IRDMA_MEMREG_TYPE_CQ = 2,
    IRDMA_MEMREG_TYPE_SRQ = 3,
}

pub const IRDMA_ALLOC_UCTX_USE_RAW_ATTR: u32 = 1 << 0;
pub const IRDMA_ALLOC_UCTX_MIN_HW_WQ_SIZE: u32 = 1 << 1;
pub const IRDMA_ALLOC_UCTX_MAX_HW_SRQ_QUANTA: u32 = 1 << 2;
pub const IRDMA_SUPPORT_WQE_FORMAT_V2: u32 = 1 << 3;

#[repr(C)]
pub struct irdma_alloc_ucontext_req {
    pub rsvd32: u32,
    pub userspace_ver: u8,
    pub rsvd8: [u8; 3],
    pub comp_mask: u64,
}

#[repr(C)]
pub struct irdma_alloc_ucontext_resp {
    pub max_pds: u32,
    pub max_qps: u32,
    pub wq_size: u32, /* size of the WQs (SQ+RQ) in the mmaped area */
    pub kernel_ver: u8,
    pub rsvd: [u8; 3],
    pub feature_flags: u64,
    pub db_mmap_key: u64,
    pub max_hw_wq_frags: u32,
    pub max_hw_read_sges: u32,
    pub max_hw_inline: u32,
    pub max_hw_rq_quanta: u32,
    pub max_hw_wq_quanta: u32,
    pub min_hw_cq_size: u32,
    pub max_hw_cq_size: u32,
    pub max_hw_sq_chunk: u16,
    pub hw_rev: u8,
    pub rsvd2: u8,
    pub comp_mask: u64,
    pub min_hw_wq_size: u16,
    pub revd3: [u8; 2],
    pub max_hw_srq_quanta: u32,
}

#[repr(C)]
pub struct irdma_alloc_pd_resp {
    pub pd_id: u32,
    pub rsvd: [u8; 4],
}

#[repr(C)]
pub struct irdma_resize_cq_req {
    pub user_cq_buffer: u64,
}

#[repr(C)]
pub struct irdma_create_cq_req {
    pub user_cq_buf: u64,
    pub user_shadow_area: u64,
}

#[repr(C)]
pub struct irdma_create_srq_req {
    pub user_srq_buf: u64,
    pub user_shadow_area: u64,
}

#[repr(C)]
pub struct irdma_create_srq_resp {
    pub srq_id: u32,
    pub srq_size: u32,
}

#[repr(C)]
pub struct irdma_create_qp_req {
    pub user_wqe_bufs: u64,
    pub user_compl_ctx: u64,
    pub legacy_dontuse: [u64; 2],
}

#[repr(C)]
pub struct irdma_mem_reg_req {
    pub reg_type: u16, /* enum irdma_memreg_type */
    pub cq_pages: u16,
    pub rq_pages: u16,
    pub sq_pages: u16,
}

#[repr(C)]
pub struct irdma_modify_qp_req {
    pub sq_flush: u8,
    pub rq_flush: u8,
    pub rsvd: [u8; 6],
}

#[repr(C)]
pub struct irdma_create_cq_resp {
    pub cq_id: u32,
    pub cq_size: u32,
}

#[repr(C)]
pub struct irdma_create_qp_resp {
    pub qp_id: u32,
    pub actual_sq_size: u32,
    pub actual_rq_size: u32,
    pub irdma_drv_opt: u32,
    pub push_idx: u16,
    pub lsmm: u8,
    pub rsvd: u8,
    pub qp_caps: u32,
}

#[repr(C)]
pub struct irdma_modify_qp_resp {
    pub push_wqe_mmap_key: u64,
    pub push_db_mmap_key: u64,
    pub push_offset: u16,
    pub push_valid: u8,
    pub rsvd: [u8; 5],
}

#[repr(C)]
pub struct irdma_create_ah_resp {
    pub ah_id: u32,
    pub rsvd: [u8; 4],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
