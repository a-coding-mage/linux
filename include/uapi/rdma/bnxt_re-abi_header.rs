/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause) */
/* Broadcom NetXtreme-E RoCE driver; Uverbs ABI header file. */

pub const BNXT_RE_ABI_VERSION: u32 = 1;

pub const BNXT_RE_CHIP_ID0_CHIP_NUM_SFT: u32 = 0x00;
pub const BNXT_RE_CHIP_ID0_CHIP_REV_SFT: u32 = 0x10;
pub const BNXT_RE_CHIP_ID0_CHIP_MET_SFT: u32 = 0x18;

pub const BNXT_RE_UCNTX_CMASK_HAVE_CCTX: u64 = 0x1;
pub const BNXT_RE_UCNTX_CMASK_HAVE_MODE: u64 = 0x02;
pub const BNXT_RE_UCNTX_CMASK_WC_DPI_ENABLED: u64 = 0x04;
pub const BNXT_RE_UCNTX_CMASK_DBR_PACING_ENABLED: u64 = 0x08;
pub const BNXT_RE_UCNTX_CMASK_POW2_DISABLED: u64 = 0x10;
pub const BNXT_RE_UCNTX_CMASK_MSN_TABLE_ENABLED: u64 = 0x40;
pub const BNXT_RE_UCNTX_CMASK_QP_RATE_LIMIT_ENABLED: u64 = 0x80;
pub const BNXT_RE_UCNTX_CMASK_TOGGLE_MEM_UOBJ_SUPPORT: u64 = 0x400000;

pub const BNXT_QPLIB_WQE_MODE_STATIC: u32 = 0x00;
pub const BNXT_QPLIB_WQE_MODE_VARIABLE: u32 = 0x01;
pub const BNXT_QPLIB_WQE_MODE_INVALID: u32 = 0x02;

pub const BNXT_RE_COMP_MASK_REQ_UCNTX_POW2_SUPPORT: u32 = 0x01;
pub const BNXT_RE_COMP_MASK_REQ_UCNTX_VAR_WQE_SUPPORT: u32 = 0x02;

#[repr(C)]
pub struct bnxt_re_uctx_req {
    pub comp_mask: u64,
}

#[repr(C)]
pub struct bnxt_re_uctx_resp {
    pub dev_id: u32,
    pub max_qp: u32,
    pub pg_size: u32,
    pub cqe_sz: u32,
    pub max_cqd: u32,
    pub rsvd: u32,
    pub comp_mask: u64,
    pub chip_id0: u32,
    pub chip_id1: u32,
    pub mode: u32,
    pub rsvd1: u32,
}

/* Packed because this follows ib_uverbs_alloc_pd_resp, which is not 8-byte aligned. */
#[repr(C, packed(4))]
pub struct bnxt_re_pd_resp {
    pub pdid: u32,
    pub dpi: u32,
    pub dbr: u64,
}

#[repr(C)]
pub struct bnxt_re_cq_req {
    pub cq_va: u64,
    pub cq_handle: u64,
    pub comp_mask: u64,
}

pub const BNXT_RE_CQ_TOGGLE_PAGE_SUPPORT: u32 = 0x1;
pub const BNXT_RE_CQ_FIXED_NUM_CQE_ENABLE: u32 = 0x1;

#[repr(C)]
pub struct bnxt_re_cq_resp {
    pub cqid: u32,
    pub tail: u32,
    pub phase: u32,
    pub rsvd: u32,
    pub comp_mask: u64,
}

#[repr(C)]
pub struct bnxt_re_resize_cq_req {
    pub cq_va: u64,
}

pub const BNXT_RE_QP_REQ_MASK_FIXED_QUE_ATTR: u32 = 0x1;

#[repr(C)]
pub struct bnxt_re_qp_req {
    pub qpsva: u64,
    pub qprva: u64,
    pub qp_handle: u64,
    pub comp_mask: u64,
    pub sq_slots: u32,
    pub sq_npsn: u32,
}

pub const BNXT_RE_CREATE_QP_ATTR_DBR_HANDLE: u32 = UVERBS_ID_DRIVER_NS_WITH_UHW;

#[repr(C)]
pub struct bnxt_re_qp_resp {
    pub qpid: u32,
    pub rsvd: u32,
}

#[repr(C)]
pub struct bnxt_re_srq_req {
    pub srqva: u64,
    pub srq_handle: u64,
}

pub const BNXT_RE_SRQ_TOGGLE_PAGE_SUPPORT: u32 = 0x1;

#[repr(C)]
pub struct bnxt_re_srq_resp {
    pub srqid: u32,
    pub rsvd: u32,
    pub comp_mask: u64,
}

pub const BNXT_RE_BEG_RESV_OFFT: u32 = 0x00;
pub const BNXT_RE_AVID_OFFT: u32 = 0x10;
pub const BNXT_RE_AVID_SIZE: u32 = 0x04;
pub const BNXT_RE_END_RESV_OFFT: u32 = 0xFF0;

pub const BNXT_RE_OBJECT_ALLOC_PAGE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_OBJECT_NOTIFY_DRV: u32 = BNXT_RE_OBJECT_ALLOC_PAGE + 1;
pub const BNXT_RE_OBJECT_GET_TOGGLE_MEM: u32 = BNXT_RE_OBJECT_NOTIFY_DRV + 1;
pub const BNXT_RE_OBJECT_DBR: u32 = BNXT_RE_OBJECT_GET_TOGGLE_MEM + 1;
pub const BNXT_RE_OBJECT_DEFAULT_DBR: u32 = BNXT_RE_OBJECT_DBR + 1;

pub const BNXT_RE_ALLOC_WC_PAGE: u32 = 0;
pub const BNXT_RE_ALLOC_DBR_BAR_PAGE: u32 = 1;
pub const BNXT_RE_ALLOC_DBR_PAGE: u32 = 2;

pub const BNXT_RE_ALLOC_PAGE_HANDLE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_ALLOC_PAGE_TYPE: u32 = BNXT_RE_ALLOC_PAGE_HANDLE + 1;
pub const BNXT_RE_ALLOC_PAGE_DPI: u32 = BNXT_RE_ALLOC_PAGE_TYPE + 1;
pub const BNXT_RE_ALLOC_PAGE_MMAP_OFFSET: u32 = BNXT_RE_ALLOC_PAGE_DPI + 1;
pub const BNXT_RE_ALLOC_PAGE_MMAP_LENGTH: u32 = BNXT_RE_ALLOC_PAGE_MMAP_OFFSET + 1;

pub const BNXT_RE_DESTROY_PAGE_HANDLE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_METHOD_ALLOC_PAGE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_METHOD_DESTROY_PAGE: u32 = BNXT_RE_METHOD_ALLOC_PAGE + 1;
pub const BNXT_RE_METHOD_NOTIFY_DRV: u32 = 1u32 << UVERBS_ID_NS_SHIFT;

pub const BNXT_RE_CQ_TOGGLE_MEM: u32 = 0;
pub const BNXT_RE_SRQ_TOGGLE_MEM: u32 = 1;
pub const BNXT_RE_TOGGLE_MEM_HANDLE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_TOGGLE_MEM_TYPE: u32 = BNXT_RE_TOGGLE_MEM_HANDLE + 1;
pub const BNXT_RE_TOGGLE_MEM_RES_ID: u32 = BNXT_RE_TOGGLE_MEM_TYPE + 1;
pub const BNXT_RE_TOGGLE_MEM_MMAP_PAGE: u32 = BNXT_RE_TOGGLE_MEM_RES_ID + 1;
pub const BNXT_RE_TOGGLE_MEM_MMAP_OFFSET: u32 = BNXT_RE_TOGGLE_MEM_MMAP_PAGE + 1;
pub const BNXT_RE_TOGGLE_MEM_MMAP_LENGTH: u32 = BNXT_RE_TOGGLE_MEM_MMAP_OFFSET + 1;
pub const BNXT_RE_TOGGLE_MEM_CQ_HANDLE: u32 = BNXT_RE_TOGGLE_MEM_MMAP_LENGTH + 1;
pub const BNXT_RE_TOGGLE_MEM_SRQ_HANDLE: u32 = BNXT_RE_TOGGLE_MEM_CQ_HANDLE + 1;
pub const BNXT_RE_RELEASE_TOGGLE_MEM_HANDLE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_METHOD_GET_TOGGLE_MEM: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_METHOD_RELEASE_TOGGLE_MEM: u32 = BNXT_RE_METHOD_GET_TOGGLE_MEM + 1;

#[repr(C)]
pub struct bnxt_re_packet_pacing_caps {
    pub qp_rate_limit_min: u32,
    pub qp_rate_limit_max: u32,
    pub supported_qpts: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct bnxt_re_query_device_ex_resp {
    pub packet_pacing_caps: bnxt_re_packet_pacing_caps,
}

#[repr(C)]
pub struct bnxt_re_db_region {
    pub dpi: u32,
    pub reserved: u32,
    pub umdbr: u64,
}

pub const BNXT_RE_ALLOC_DBR_HANDLE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_ALLOC_DBR_ATTR: u32 = BNXT_RE_ALLOC_DBR_HANDLE + 1;
pub const BNXT_RE_ALLOC_DBR_OFFSET: u32 = BNXT_RE_ALLOC_DBR_ATTR + 1;
pub const BNXT_RE_FREE_DBR_HANDLE: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_DEFAULT_DBR_ATTR: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_METHOD_DBR_ALLOC: u32 = 1u32 << UVERBS_ID_NS_SHIFT;
pub const BNXT_RE_METHOD_DBR_FREE: u32 = BNXT_RE_METHOD_DBR_ALLOC + 1;
pub const BNXT_RE_METHOD_GET_DEFAULT_DBR: u32 = BNXT_RE_METHOD_DBR_FREE + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
