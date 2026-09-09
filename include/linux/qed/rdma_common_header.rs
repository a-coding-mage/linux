/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

/* RDMA FW CONSTANTS */

pub const RDMA_RESERVED_LKEY: u32 = 0;
pub const RDMA_RING_PAGE_SIZE: u32 = 0x1000;

pub const RDMA_MAX_SGE_PER_SQ_WQE: u32 = 4;
pub const RDMA_MAX_SGE_PER_RQ_WQE: u32 = 4;

pub const RDMA_MAX_DATA_SIZE_IN_WQE: u32 = 0x80000000;

pub const RDMA_REQ_RD_ATOMIC_ELM_SIZE: u32 = 0x50;
pub const RDMA_RESP_RD_ATOMIC_ELM_SIZE: u32 = 0x20;

pub const RDMA_MAX_CQS: u32 = 64 * 1024;
pub const RDMA_MAX_TIDS: u32 = 128 * 1024 - 1;
pub const RDMA_MAX_PDS: u32 = 64 * 1024;
pub const RDMA_MAX_XRC_SRQS: u32 = 1024;
pub const RDMA_MAX_SRQS: u32 = 32 * 1024;
pub const RDMA_MAX_IRQ_ELEMS_IN_PAGE: u32 = 128;

pub const RDMA_NUM_STATISTIC_COUNTERS: u32 = MAX_NUM_VPORTS;
pub const RDMA_NUM_STATISTIC_COUNTERS_K2: u32 = MAX_NUM_VPORTS_K2;
pub const RDMA_NUM_STATISTIC_COUNTERS_BB: u32 = MAX_NUM_VPORTS_BB;

pub const RDMA_TASK_TYPE: u32 = PROTOCOLID_ROCE;

#[repr(C)]
pub struct rdma_srq_id {
    pub srq_idx: __le16,
    pub opaque_fid: __le16,
}

#[repr(C)]
pub struct rdma_srq_producers {
    pub sge_prod: __le32,
    pub wqe_prod: __le32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
