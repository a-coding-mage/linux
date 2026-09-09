/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause) */
/*
 * Copyright 2018-2026 Amazon.com, Inc. or its affiliates. All rights reserved.
 */

// #include <linux/types.h>
// #include <rdma/ib_user_ioctl_cmds.h>

/*
 * Increment this value if any changes that break userspace ABI
 * compatibility are made.
 */
pub const EFA_UVERBS_ABI_VERSION: u32 = 1;

/*
 * Keep structs aligned to 8 bytes.
 * Keep reserved fields as arrays of __u8 named reserved_XXX where XXX is the
 * hex bit offset of the field.
 */

pub const EFA_ALLOC_UCONTEXT_CMD_SUPP_CAPS_TX_BATCH: u32 = 1 << 0;
pub const EFA_ALLOC_UCONTEXT_CMD_SUPP_CAPS_MIN_SQ_WR: u32 = 1 << 1;

#[repr(C)]
pub struct efa_ibv_alloc_ucontext_cmd {
    pub supported_caps: u32,
    pub reserved_20: [u8; 4],
}

pub const EFA_USER_CMDS_SUPP_UDATA_QUERY_DEVICE: u32 = 1 << 0;
pub const EFA_USER_CMDS_SUPP_UDATA_CREATE_AH: u32 = 1 << 1;

#[repr(C)]
pub struct efa_ibv_alloc_ucontext_resp {
    pub comp_mask: u32,
    pub cmds_supp_udata_mask: u32,
    pub sub_cqs_per_cq: u16,
    pub inline_buf_size: u16,
    pub max_llq_size: u32, /* bytes */
    pub max_tx_batch: u16, /* units of 64 bytes */
    pub min_sq_wr: u16,
    pub inline_buf_size_ex: u16,
    pub reserved_b0: [u8; 2],
}

#[repr(C)]
pub struct efa_ibv_alloc_pd_resp {
    pub comp_mask: u32,
    pub pdn: u16,
    pub reserved_30: [u8; 2],
}

pub const EFA_CREATE_CQ_WITH_COMPLETION_CHANNEL: u32 = 1 << 0;
pub const EFA_CREATE_CQ_WITH_SGID: u32 = 1 << 1;
pub const EFA_CREATE_CQ_WITH_SQ_COMP_64_BIT_REQ_ID: u32 = 1 << 2;

#[repr(C)]
pub struct efa_ibv_create_cq {
    pub comp_mask: u32,
    pub cq_entry_size: u32,
    pub num_sub_cqs: u16,
    pub flags: u8,
    pub reserved_58: [u8; 5],
}

pub const EFA_CREATE_CQ_RESP_DB_OFF: u32 = 1 << 0;

#[repr(C)]
pub struct efa_ibv_create_cq_resp {
    pub comp_mask: u32,
    pub reserved_20: [u8; 4],
    pub q_mmap_key: u64,
    pub q_mmap_size: u64,
    pub cq_idx: u16,
    pub reserved_d0: [u8; 2],
    pub db_off: u32,
    pub db_mmap_key: u64,
}

pub const EFA_QP_DRIVER_TYPE_SRD: u32 = 0;

pub const EFA_CREATE_QP_WITH_UNSOLICITED_WRITE_RECV: u32 = 1 << 0;
pub const EFA_CREATE_QP_WITH_SQ_64_BIT_REQ_ID: u32 = 1 << 1;

#[repr(C)]
pub struct efa_ibv_create_qp {
    pub comp_mask: u32,
    pub rq_ring_size: u32, /* bytes */
    pub sq_ring_size: u32, /* bytes */
    pub driver_qp_type: u32,
    pub flags: u16,
    pub sl: u8,
    pub reserved_98: [u8; 5],
}

#[repr(C)]
pub struct efa_ibv_create_qp_resp {
    pub comp_mask: u32,
    /* the offset inside the page of the rq db */
    pub rq_db_offset: u32,
    /* the offset inside the page of the sq db */
    pub sq_db_offset: u32,
    /* the offset inside the page of descriptors buffer */
    pub llq_desc_offset: u32,
    pub rq_mmap_key: u64,
    pub rq_mmap_size: u64,
    pub rq_db_mmap_key: u64,
    pub sq_db_mmap_key: u64,
    pub llq_desc_mmap_key: u64,
    pub send_sub_cq_idx: u16,
    pub recv_sub_cq_idx: u16,
    pub reserved_1e0: [u8; 4],
}

#[repr(C)]
pub struct efa_ibv_create_ah_resp {
    pub comp_mask: u32,
    pub efa_address_handle: u16,
    pub reserved_30: [u8; 2],
}

pub const EFA_QUERY_DEVICE_CAPS_RDMA_READ: u32 = 1 << 0;
pub const EFA_QUERY_DEVICE_CAPS_RNR_RETRY: u32 = 1 << 1;
pub const EFA_QUERY_DEVICE_CAPS_CQ_NOTIFICATIONS: u32 = 1 << 2;
pub const EFA_QUERY_DEVICE_CAPS_CQ_WITH_SGID: u32 = 1 << 3;
pub const EFA_QUERY_DEVICE_CAPS_DATA_POLLING_128: u32 = 1 << 4;
pub const EFA_QUERY_DEVICE_CAPS_RDMA_WRITE: u32 = 1 << 5;
pub const EFA_QUERY_DEVICE_CAPS_UNSOLICITED_WRITE_RECV: u32 = 1 << 6;
pub const EFA_QUERY_DEVICE_CAPS_CQ_WITH_EXT_MEM: u32 = 1 << 7;
pub const EFA_QUERY_DEVICE_CAPS_COMP_CNTR: u32 = 1 << 8;
pub const EFA_QUERY_DEVICE_CAPS_SQ_64_BIT_REQ_ID: u32 = 1 << 9;

#[repr(C)]
pub struct efa_ibv_ex_query_device_resp {
    pub comp_mask: u32,
    pub max_sq_wr: u32,
    pub max_rq_wr: u32,
    pub max_sq_sge: u16,
    pub max_rq_sge: u16,
    pub max_rdma_size: u32,
    pub device_caps: u32,
}

pub const EFA_QUERY_MR_VALIDITY_RECV_IC_ID: u32 = 1 << 0;
pub const EFA_QUERY_MR_VALIDITY_RDMA_READ_IC_ID: u32 = 1 << 1;
pub const EFA_QUERY_MR_VALIDITY_RDMA_RECV_IC_ID: u32 = 1 << 2;

// The value of UVERBS_ID_NS_SHIFT is supplied by rdma/ib_user_ioctl_cmds.h.
pub const EFA_IB_ATTR_QUERY_MR_HANDLE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const EFA_IB_ATTR_QUERY_MR_RESP_IC_ID_VALIDITY: u32 = EFA_IB_ATTR_QUERY_MR_HANDLE + 1;
pub const EFA_IB_ATTR_QUERY_MR_RESP_RECV_IC_ID: u32 = EFA_IB_ATTR_QUERY_MR_HANDLE + 2;
pub const EFA_IB_ATTR_QUERY_MR_RESP_RDMA_READ_IC_ID: u32 = EFA_IB_ATTR_QUERY_MR_HANDLE + 3;
pub const EFA_IB_ATTR_QUERY_MR_RESP_RDMA_RECV_IC_ID: u32 = EFA_IB_ATTR_QUERY_MR_HANDLE + 4;

pub const EFA_IB_METHOD_MR_QUERY: u32 = 1u32 << UVERBS_ID_NS_SHIFT;

pub const EFA_IB_ATTR_CREATE_COMP_CNTR_COMP_BUFFER: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const EFA_IB_ATTR_CREATE_COMP_CNTR_ERR_BUFFER: u32 = EFA_IB_ATTR_CREATE_COMP_CNTR_COMP_BUFFER + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
