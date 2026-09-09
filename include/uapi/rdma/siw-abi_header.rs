/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause */

/* Authors: Bernard Metzler <bmt@zurich.ibm.com> */
/* Copyright (c) 2008-2019, IBM Corporation */

/* Dependency: Linux integer types are represented by their Rust equivalents. */

pub const SIW_NODE_DESC_COMMON: &str = "Software iWARP stack";
pub const SIW_ABI_VERSION: i32 = 1;
pub const SIW_MAX_SGE: usize = 6;
pub const SIW_UOBJ_MAX_KEY: u32 = 0x08FFFF;
pub const SIW_INVAL_UOBJ_KEY: u32 = SIW_UOBJ_MAX_KEY + 1;

#[repr(C)]
pub struct siw_uresp_create_cq {
    pub cq_id: u32,
    pub num_cqe: u32,
    pub cq_key: u64,
}

#[repr(C)]
pub struct siw_uresp_create_qp {
    pub qp_id: u32,
    pub num_sqe: u32,
    pub num_rqe: u32,
    pub pad: u32,
    pub sq_key: u64,
    pub rq_key: u64,
}

#[repr(C)]
pub struct siw_ureq_reg_mr {
    pub stag_key: u8,
    pub reserved: [u8; 3],
    pub pad: u32,
}

#[repr(C)]
pub struct siw_uresp_reg_mr {
    pub stag: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct siw_uresp_create_srq {
    pub num_rqe: u32,
    pub pad: u32,
    pub srq_key: u64,
}

#[repr(C)]
pub struct siw_uresp_alloc_ctx {
    pub dev_id: u32,
    pub pad: u32,
}

#[repr(i32)]
pub enum siw_opcode {
    SIW_OP_WRITE,
    SIW_OP_READ,
    SIW_OP_READ_LOCAL_INV,
    SIW_OP_SEND,
    SIW_OP_SEND_WITH_IMM,
    SIW_OP_SEND_REMOTE_INV,
    /* Unsupported */
    SIW_OP_FETCH_AND_ADD,
    SIW_OP_COMP_AND_SWAP,
    SIW_OP_RECEIVE,
    /* provider internal SQE */
    SIW_OP_READ_RESPONSE,
    /* below opcodes valid for in-kernel clients only */
    SIW_OP_INVAL_STAG,
    SIW_OP_REG_MR,
    SIW_NUM_OPCODES,
}

/* Keep it same as ibv_sge to allow for memcpy */
#[repr(C)]
pub struct siw_sge {
    pub laddr: u64,
    pub length: u32,
    pub lkey: u32,
}

/* Inline data are kept within the work request itself occupying the
 * space of sge[1] .. sge[n]. Therefore, inline data cannot be
 * supported if SIW_MAX_SGE is below 2 elements.
 */
pub const SIW_MAX_INLINE: usize = core::mem::size_of::<siw_sge>() * (SIW_MAX_SGE - 1);

#[repr(i32)]
pub enum siw_wqe_flags {
    SIW_WQE_VALID = 1,
    SIW_WQE_INLINE = 1 << 1,
    SIW_WQE_SIGNALLED = 1 << 2,
    SIW_WQE_SOLICITED = 1 << 3,
    SIW_WQE_READ_FENCE = 1 << 4,
    SIW_WQE_REM_INVAL = 1 << 5,
    SIW_WQE_COMPLETED = 1 << 6,
}

#[repr(C)]
pub union siw_sqe_addr {
    pub raddr: u64,
    pub base_mr: u64,
}

#[repr(C)]
pub union siw_sqe_data {
    pub sge: [siw_sge; SIW_MAX_SGE],
    pub access: u64,
}

/* Send Queue Element */
#[repr(C)]
pub struct siw_sqe {
    pub id: u64,
    pub flags: u16,
    pub num_sge: u8,
    /* Contains enum siw_opcode values */
    pub opcode: u8,
    pub rkey: u32,
    pub addr: siw_sqe_addr,
    pub data: siw_sqe_data,
}

/* Receive Queue Element */
#[repr(C)]
pub struct siw_rqe {
    pub id: u64,
    pub flags: u16,
    pub num_sge: u8,
    /* only used by kernel driver, ignored if set by user */
    pub opcode: u8,
    pub unused: u32,
    pub sge: [siw_sge; SIW_MAX_SGE],
}

#[repr(i32)]
pub enum siw_notify_flags {
    SIW_NOTIFY_NOT = 0,
    SIW_NOTIFY_SOLICITED = 1 << 0,
    SIW_NOTIFY_NEXT_COMPLETION = 1 << 1,
    SIW_NOTIFY_MISSED_EVENTS = 1 << 2,
    SIW_NOTIFY_ALL = (1 << 0) | (1 << 1) | (1 << 2),
}

#[repr(i32)]
pub enum siw_wc_status {
    SIW_WC_SUCCESS,
    SIW_WC_LOC_LEN_ERR,
    SIW_WC_LOC_PROT_ERR,
    SIW_WC_LOC_QP_OP_ERR,
    SIW_WC_WR_FLUSH_ERR,
    SIW_WC_BAD_RESP_ERR,
    SIW_WC_LOC_ACCESS_ERR,
    SIW_WC_REM_ACCESS_ERR,
    SIW_WC_REM_INV_REQ_ERR,
    SIW_WC_GENERAL_ERR,
    SIW_NUM_WC_STATUS,
}

#[repr(C)]
pub union siw_cqe_imm {
    pub imm_data: u64,
    pub inval_stag: u32,
}

#[repr(C)]
pub union siw_cqe_qp {
    /* QP number or QP pointer */
    pub base_qp: *mut ib_qp,
    pub qp_id: u64,
}

#[repr(C)]
pub struct siw_cqe {
    pub id: u64,
    pub flags: u8,
    pub opcode: u8,
    pub status: u16,
    pub bytes: u32,
    pub imm: siw_cqe_imm,
    pub qp: siw_cqe_qp,
}

/* Shared structure between user and kernel to control CQ arming. */
#[repr(C)]
pub struct siw_cq_ctrl {
    pub flags: u32,
    pub pad: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
