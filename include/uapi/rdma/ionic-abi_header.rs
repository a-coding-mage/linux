/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (C) 2018-2025, Advanced Micro Devices, Inc */

// Translated from ionic-abi.h. The original Linux __u* types map to the
// corresponding fixed-width Rust integer types.

pub const IONIC_ABI_VERSION: u32 = 1;

pub const IONIC_EXPDB_64: u32 = 1;
pub const IONIC_EXPDB_128: u32 = 2;
pub const IONIC_EXPDB_256: u32 = 4;
pub const IONIC_EXPDB_512: u32 = 8;

pub const IONIC_EXPDB_SQ: u32 = 1;
pub const IONIC_EXPDB_RQ: u32 = 2;

pub const IONIC_CMB_ENABLE: u32 = 1;
pub const IONIC_CMB_REQUIRE: u32 = 2;
pub const IONIC_CMB_EXPDB: u32 = 4;
pub const IONIC_CMB_WC: u32 = 8;
pub const IONIC_CMB_UC: u32 = 16;

#[repr(C)]
pub struct ionic_ctx_req {
    pub rsvd: [u32; 2],
}

#[repr(C)]
pub struct ionic_ctx_resp {
    pub rsvd: u32,
    pub page_shift: u32,
    pub dbell_offset: u64,
    pub version: u16,
    pub qp_opcodes: u8,
    pub admin_opcodes: u8,
    pub sq_qtype: u8,
    pub rq_qtype: u8,
    pub cq_qtype: u8,
    pub admin_qtype: u8,
    pub max_stride: u8,
    pub max_spec: u8,
    pub udma_count: u8,
    pub expdb_mask: u8,
    pub expdb_qtypes: u8,
    pub rsvd2: [u8; 3],
    pub phc_offset: u64,
}

#[repr(C)]
pub struct ionic_qdesc {
    pub addr: u64,
    pub size: u32,
    pub mask: u16,
    pub depth_log2: u8,
    pub stride_log2: u8,
}

#[repr(C)]
pub struct ionic_ah_resp {
    pub ahid: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct ionic_cq_req {
    pub cq: [ionic_qdesc; 2],
    pub udma_mask: u8,
    pub rsvd: [u8; 7],
}

#[repr(C)]
pub struct ionic_cq_resp {
    pub cqid: [u32; 2],
    pub udma_mask: u8,
    pub rsvd: [u8; 7],
}

#[repr(C)]
pub struct ionic_qp_req {
    pub sq: ionic_qdesc,
    pub rq: ionic_qdesc,
    pub sq_spec: u8,
    pub rq_spec: u8,
    pub sq_cmb: u8,
    pub rq_cmb: u8,
    pub udma_mask: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
pub struct ionic_qp_resp {
    pub qpid: u32,
    pub sq_cmb: u8,
    pub rq_cmb: u8,
    pub udma_idx: u8,
    pub rsvd: [u8; 1],
    pub sq_cmb_offset: u64,
    pub rq_cmb_offset: u64,
}

#[repr(C)]
pub struct ionic_srq_req {
    pub rq: ionic_qdesc,
    pub rq_spec: u8,
    pub rq_cmb: u8,
    pub udma_mask: u8,
    pub rsvd: [u8; 5],
}

#[repr(C)]
pub struct ionic_srq_resp {
    pub qpid: u32,
    pub rq_cmb: u8,
    pub udma_idx: u8,
    pub rsvd: [u8; 2],
    pub rq_cmb_offset: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
